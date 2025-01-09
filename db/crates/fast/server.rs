mod verbose;

use clap::Parser;
use fast_server::DEFAULT_PORT;
use std::{env::consts, path::PathBuf, process};
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber::fmt::format::FmtSpan;
use verbose::{InfoLevel, Verbosity};

#[derive(Parser, Debug)]
#[command(name = "fast-server", version)]
struct Cli {
    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,

    #[arg(id = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,

    #[arg(long, help = "Path to save/load database state")]
    state: Option<PathBuf>,
}

#[tokio::main]
pub async fn main() -> fast_proto::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_max_level(cli.verbose.log_level_filter())
        .init();

    let addr = format!("{}:{}", cli.host, cli.port);
    let listener = TcpListener::bind(&addr).await?;

    println!(
        "\n  FAST v{} {}-{}\n\n  Starting in standalone mode\n  Port: {}\n  PID: {}\n\n\n    https://themackabu.dev\n",
        env!("CARGO_PKG_VERSION"),
        consts::OS,
        consts::ARCH,
        cli.port,
        process::id()
    );

    Ok(fast_server::run(listener, signal::ctrl_c(), cli.state).await)
}
