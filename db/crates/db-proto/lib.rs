use parking_lot::RwLock;

use std::collections::HashMap;
use std::io;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DB {
    hashmap: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl DB {
    pub async fn open() -> io::Result<Self> {
        Ok(Self {
            hashmap: Arc::new(RwLock::new(HashMap::<Vec<u8>, Vec<u8>>::new())),
        })
    }

    pub async fn set(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> io::Result<Option<Vec<u8>>> {
        let hashmap = self.hashmap.clone();
        let mut hashmap = hashmap.write();
        Ok(hashmap.insert(key.as_ref().to_owned(), value.as_ref().to_owned()))
    }

    #[must_use]
    pub async fn get(&self, key: impl AsRef<[u8]>) -> io::Result<Option<Vec<u8>>> {
        let key = key.as_ref().to_owned();
        let hashmap = &self.hashmap.read();
        Ok(hashmap.get(&key).cloned())
    }

    pub async fn del(&mut self, key: impl AsRef<[u8]>) -> io::Result<Option<Vec<u8>>> {
        let key = key.as_ref().to_owned();
        let hashmap = &mut self.hashmap.write();
        Ok(hashmap.remove(&key))
    }
}
