use async_trait::async_trait;
use crate::error::VaultError;

pub mod memory;
pub mod redis;

#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn store(&self, token: &str, value: Vec<u8>) -> Result<(), VaultError>;
    async fn fetch(&self, token: &str) -> Result<Option<Vec<u8>>, VaultError>;
}