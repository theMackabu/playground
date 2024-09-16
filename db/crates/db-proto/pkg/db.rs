use tokio::sync::{broadcast, Notify};
use tokio::time::{self, Duration, Instant};

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing::debug;

#[derive(Debug)]
pub struct DbDropGuard {
    db: Db,
}

#[derive(Debug, Clone)]
pub struct Db {
    shared: Arc<Shared>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableState {
    entries: HashMap<String, SerializableEntry>,
    expirations: Vec<(u64, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableEntry {
    data: Vec<u8>,
    expires_at: Option<u64>,
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
    background_task: Notify,
}

#[derive(Debug)]
struct State {
    entries: HashMap<String, Entry>,
    pub_sub: HashMap<String, broadcast::Sender<Bytes>>,
    expirations: BTreeSet<(Instant, String)>,
    shutdown: bool,
}

#[derive(Debug)]
struct Entry {
    data: Bytes,
    expires_at: Option<Instant>,
}

impl DbDropGuard {
    pub fn new() -> DbDropGuard { DbDropGuard { db: Db::new() } }
    pub fn db(&self) -> Db { self.db.clone() }
}

impl Drop for DbDropGuard {
    fn drop(&mut self) { self.db.shutdown_purge_task(); }
}

impl Db {
    pub fn new() -> Db {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                entries: HashMap::new(),
                pub_sub: HashMap::new(),
                expirations: BTreeSet::new(),
                shutdown: false,
            }),
            background_task: Notify::new(),
        });

        tokio::spawn(purge_expired_tasks(shared.clone()));

        Db { shared }
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().unwrap();
        state.entries.get(key).map(|entry| entry.data.clone())
    }

    pub fn set(&self, key: String, value: Bytes, expire: Option<Duration>) {
        let mut state = self.shared.state.lock().unwrap();
        let mut notify = false;

        let expires_at = expire.map(|duration| {
            let when = Instant::now() + duration;
            notify = state.next_expiration().map(|expiration| expiration > when).unwrap_or(true);

            when
        });

        let prev = state.entries.insert(key.clone(), Entry { data: value, expires_at });

        if let Some(prev) = prev {
            if let Some(when) = prev.expires_at {
                state.expirations.remove(&(when, key.clone()));
            }
        }

        if let Some(when) = expires_at {
            state.expirations.insert((when, key));
        }

        drop(state);

        if notify {
            self.shared.background_task.notify_one();
        }
    }

    pub fn subscribe(&self, key: String) -> broadcast::Receiver<Bytes> {
        use std::collections::hash_map::Entry;
        let mut state = self.shared.state.lock().unwrap();

        match state.pub_sub.entry(key) {
            Entry::Occupied(e) => e.get().subscribe(),
            Entry::Vacant(e) => {
                let (tx, rx) = broadcast::channel(1024);
                e.insert(tx);
                rx
            }
        }
    }

    pub fn publish(&self, key: &str, value: Bytes) -> usize {
        let state = self.shared.state.lock().unwrap();
        state.pub_sub.get(key).map(|tx| tx.send(value).unwrap_or(0)).unwrap_or(0)
    }

    pub fn dump(&self) -> SerializableState {
        let state = self.shared.state.lock().unwrap();
        let now = Instant::now();

        SerializableState {
            entries: state
                .entries
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SerializableEntry {
                            data: v.data.to_vec(),
                            expires_at: v.expires_at.map(|instant| instant.duration_since(now).as_secs()),
                        },
                    )
                })
                .collect(),
            expirations: state.expirations.iter().map(|(instant, key)| (instant.duration_since(now).as_secs(), key.clone())).collect(),
        }
    }

    pub fn load(&self, serializable_state: SerializableState) {
        let mut state = self.shared.state.lock().unwrap();
        let now = Instant::now();

        state.entries = serializable_state
            .entries
            .into_iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    Entry {
                        data: Bytes::from(v.data),
                        expires_at: v.expires_at.map(|secs| now + Duration::from_secs(secs)),
                    },
                )
            })
            .collect();

        state.expirations = serializable_state.expirations.into_iter().map(|(secs, key)| (now + Duration::from_secs(secs), key)).collect();
    }

    pub async fn dump_to(&self, path: &PathBuf) -> crate::Result<()> {
        let serializable_state = self.dump();
        let serialized = bincode::serialize(&serializable_state)?;
        tokio::fs::write(path, serialized).await?;
        Ok(())
    }

    pub async fn load_from(&self, path: &PathBuf) -> crate::Result<()> {
        let serialized = tokio::fs::read(path).await?;
        let serializable_state: SerializableState = bincode::deserialize(&serialized)?;
        self.load(serializable_state);
        Ok(())
    }

    fn shutdown_purge_task(&self) {
        let mut state = self.shared.state.lock().unwrap();
        state.shutdown = true;

        drop(state);
        self.shared.background_task.notify_one();
    }
}

impl Shared {
    fn purge_expired_keys(&self) -> Option<Instant> {
        let mut state = self.state.lock().unwrap();

        if state.shutdown {
            return None;
        }

        let state = &mut *state;
        let now = Instant::now();

        while let Some(&(when, ref key)) = state.expirations.iter().next() {
            if when > now {
                return Some(when);
            }

            state.entries.remove(key);
            state.expirations.remove(&(when, key.clone()));
        }

        None
    }

    fn is_shutdown(&self) -> bool { self.state.lock().unwrap().shutdown }
}

impl State {
    fn next_expiration(&self) -> Option<Instant> { self.expirations.iter().next().map(|expiration| expiration.0) }
}

async fn purge_expired_tasks(shared: Arc<Shared>) {
    while !shared.is_shutdown() {
        if let Some(when) = shared.purge_expired_keys() {
            tokio::select! {
                _ = time::sleep_until(when) => {}
                _ = shared.background_task.notified() => {}
            }
        } else {
            shared.background_task.notified().await;
        }
    }

    debug!("Purge background task shut down")
}
