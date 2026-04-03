use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, header},
};
use crate::{config::AppConfig, error::VaultError};

#[derive(Clone)]
pub enum Operation {
    Tokenize,
    Detokenize,
}

#[warn(dead_code)]
pub struct ApiKeyAuth {
    pub cfg: AppConfig,
    pub op: Operation,
}

#[warn(dead_code)]
pub struct ApiKeyExtractor(pub Operation);

#[async_trait]
impl<S> FromRequestParts<S> for ApiKeyExtractor
where
    S: Send + Sync,
{
    type Rejection = VaultError;

    // Added underscores to unused variables to silence warnings
    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // This extractor is currently a placeholder as logic is handled in the handlers
        Ok(ApiKeyExtractor(Operation::Tokenize))
    }
}

/// Simple helper to validate API key inside handlers.
pub fn check_api_key(
    headers: &header::HeaderMap,
    cfg: &AppConfig,
    op: Operation,
) -> Result<(), VaultError> {
    let provided = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .ok_or(VaultError::Unauthorized)?;

    let expected = match op {
        Operation::Tokenize => &cfg.tokenize_api_key,
        Operation::Detokenize => &cfg.detokenize_api_key,
    };

    if subtle_equals(provided.as_bytes(), expected.as_bytes()) {
        Ok(())
    } else {
        Err(VaultError::Forbidden)
    }
}

/// Constant-time compare to avoid timing leaks.
fn subtle_equals(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}