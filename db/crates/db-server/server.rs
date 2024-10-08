use db_proto::{prelude::*, Result};
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, Semaphore};
use tokio::time::{self, Duration};
use tracing::{debug, error, info, instrument};

pub use db_proto::DEFAULT_PORT;
pub const MAX_CONNECTIONS: usize = 250;

#[derive(Debug)]
struct Listener {
    db_holder: DbDropGuard,
    listener: TcpListener,
    limit_connections: Arc<Semaphore>,
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
    db_path: Option<PathBuf>,
}

#[derive(Debug)]
struct Handler {
    db: Db,
    connection: Connection,
    shutdown: Shutdown,
    _shutdown_complete: mpsc::Sender<()>,
}

pub async fn run(listener: TcpListener, shutdown: impl Future, db_path: Option<PathBuf>) {
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);
    let db_holder = DbDropGuard::new();

    if let Some(path) = &db_path {
        if path.exists() {
            info!("Loading database from {:?}", path);
            db_holder.db().load_from(path).await.expect("Failed to load database");
        }
    }

    let mut server = Listener {
        db_path,
        listener,
        db_holder,
        limit_connections: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
        notify_shutdown,
        shutdown_complete_tx,
    };

    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                error!(cause = %err, "failed to accept");
            }
        }
        _ = shutdown => {
            info!("shutting down");
            if let Some(path) = &server.db_path {
                info!("Saving database to {:?}", path);
                server.db_holder.db().dump_to(path).await.expect("Failed to save database");
            }
        }
    }

    let Listener {
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = server;

    drop(notify_shutdown);
    drop(shutdown_complete_tx);

    let _ = shutdown_complete_rx.recv().await;
}

impl Listener {
    async fn run(&mut self) -> Result<()> {
        info!("accepting inbound connections");

        loop {
            let permit = self.limit_connections.clone().acquire_owned().await.unwrap();
            let socket = self.accept().await?;

            let mut handler = Handler {
                db: self.db_holder.db(),
                connection: Connection::new(socket),
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
                _shutdown_complete: self.shutdown_complete_tx.clone(),
            };

            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    error!(cause = ?err, "connection error");
                }
                drop(permit);
            });
        }
    }

    async fn accept(&mut self) -> crate::Result<TcpStream> {
        let mut backoff = 1;

        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    if backoff > 64 {
                        return Err(err.into());
                    }
                }
            }

            time::sleep(Duration::from_secs(backoff)).await;
            backoff *= 2;
        }
    }
}

impl Handler {
    #[instrument(skip(self))]
    async fn run(&mut self) -> Result<()> {
        while !self.shutdown.is_shutdown() {
            let maybe_frame = tokio::select! {
                res = self.connection.read_frame() => res?,
                _ = self.shutdown.recv() => {
                    return Ok(());
                }
            };

            let frame = match maybe_frame {
                Some(frame) => frame,
                None => return Ok(()),
            };

            let cmd = Command::from_frame(frame)?;
            debug!(?cmd);

            cmd.apply(&self.db, &mut self.connection, &mut self.shutdown).await?;
        }

        Ok(())
    }
}
