mod config;
mod error;
mod crypto;
mod auth;
mod api;
mod storage;

use crate::config::AppConfig;
use crate::storage::{StorageBackend, memory::InMemoryStorage, redis::RedisStorage};
use api::build_router;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = AppConfig::from_env()?;

    let storage: Box<dyn StorageBackend> = if cfg.use_redis {
        Box::new(RedisStorage::new(&cfg.redis_url).await?)
    } else {
        Box::new(InMemoryStorage::new())
    };

    let crypto = crypto::Crypto::new(cfg.data_encryption_key.as_bytes())?;

    let app = build_router(storage, crypto, cfg.clone());

    let addr: SocketAddr = cfg
        .bind_addr
        .parse()
        .expect("Invalid BIND_ADDR (use host:port)");

    tracing::info!("Starting server on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app,
    )
    .await?;

    Ok(())
}