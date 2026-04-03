use anyhow::Context;

#[derive(Clone)]
pub struct AppConfig {
    pub bind_addr: String,
    pub redis_url: String,
    pub use_redis: bool,
    pub data_encryption_key: String,
    pub tokenize_api_key: String,
    pub detokenize_api_key: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
        let use_redis = std::env::var("USE_REDIS").unwrap_or_else(|_| "false".to_string()) == "true";

        let data_encryption_key = std::env::var("DATA_ENCRYPTION_KEY")
            .context("DATA_ENCRYPTION_KEY must be set")?;

        let tokenize_api_key = std::env::var("TOKENIZE_API_KEY")
            .context("TOKENIZE_API_KEY must be set")?;
        let detokenize_api_key = std::env::var("DETOKENIZE_API_KEY")
            .context("DETOKENIZE_API_KEY must be set")?;

        Ok(Self {
            bind_addr,
            redis_url,
            use_redis,
            data_encryption_key,
            tokenize_api_key,
            detokenize_api_key,
        })
    }
}