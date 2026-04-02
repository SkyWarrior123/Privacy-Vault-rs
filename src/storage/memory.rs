use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use async_trait::async_trait;
use crate::error::VaultError;
use super::StorageBackend;

#[derive(Clone)]
pub struct InMemoryStorage {
    inner: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl StorageBackend for InMemoryStorage {
    async fn store(&self, token: &str, value: Vec<u8>) -> Result<(), VaultError> {
        let mut map = self.inner.write().unwrap();
        map.insert(token.to_string(), value);
        Ok(())
    }

    async fn fetch(&self, token: &str) -> Result<Option<Vec<u8>>, VaultError> {
        let map = self.inner.read().unwrap();
        Ok(map.get(token).cloned())
    }
}