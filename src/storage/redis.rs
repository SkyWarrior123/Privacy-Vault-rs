use async_trait::async_trait;
use redis::{Client, AsyncCommands};
use crate::error::VaultError;
use super::StorageBackend;

#[derive(Clone)]
pub struct RedisStorage {
    client: Client,
}

impl RedisStorage {
    pub async fn new(url: &str) -> Result<Self, VaultError> {
        let client = Client::open(url)
            .map_err(|e| VaultError::Storage(format!("redis client error: {e}")))?;
        Ok(Self { client })
    }
}

#[async_trait]
impl StorageBackend for RedisStorage {
    async fn store(&self, token: &str, value: Vec<u8>) -> Result<(), VaultError> {
        // Use multiplexed connection to avoid deprecation warnings
        let mut conn = self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| VaultError::Storage(format!("redis conn error: {e}")))?;

        let key = format!("vault:{}", token);
        
        // Explicitly type as () to tell Redis we don't need a return value
        let _: () = conn.set(key, value)
            .await
            .map_err(|e| VaultError::Storage(format!("redis set error: {e}")))?;
            
        Ok(())
    }

    async fn fetch(&self, token: &str) -> Result<Option<Vec<u8>>, VaultError> {
        let mut conn = self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| VaultError::Storage(format!("redis conn error: {e}")))?;

        let key = format!("vault:{}", token);
        
        // Explicitly type the result to help the compiler
        let res: Option<Vec<u8>> = conn
            .get(key)
            .await
            .map_err(|e| VaultError::Storage(format!("redis get error: {e}")))?;
            
        Ok(res)
    }
}