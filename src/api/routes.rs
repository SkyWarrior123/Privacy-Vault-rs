use std::sync::Arc;

use axum::{
    routing::post,
    Router,
    extract::{State, FromRef},
    Json,
    http::HeaderMap,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use rand::{distributions::Alphanumeric, Rng};

use crate::{
    config::AppConfig,
    crypto::Crypto,
    error::VaultError,
    storage::StorageBackend,
    auth::{check_api_key, Operation},
};

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<dyn StorageBackend>,
    pub crypto: Arc<Crypto>,
    pub cfg: AppConfig,
}

impl FromRef<AppState> for AppConfig {
    fn from_ref(state: &AppState) -> Self {
        state.cfg.clone()
    }
}

pub fn build_router(
    storage: Box<dyn StorageBackend>,
    crypto: Crypto,
    cfg: AppConfig,
) -> Router {
    let state = AppState {
        storage: storage.into(),
        crypto: Arc::new(crypto),
        cfg,
    };

    Router::new()
        .route("/tokenize", post(tokenize))
        .route("/detokenize", post(detokenize))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct TokenizeRequest {
    pub id: String,
    pub data: Value, // expect object
}

#[derive(Serialize)]
pub struct TokenizeResponse {
    pub id: String,
    pub data: Value, // object with tokens
}

#[derive(Deserialize)]
pub struct DetokenizeRequest {
    pub id: String,
    pub data: Value, // Object with tokens
}

#[derive(Serialize)]
pub struct DetokenizeField {
    pub found: bool,
    pub value: String,
}

pub async fn tokenize(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<TokenizeRequest>,
) -> Result<(axum::http::StatusCode, Json<TokenizeResponse>), VaultError> {
    check_api_key(&headers, &state.cfg, Operation::Tokenize)?;

    let obj = req.data.as_object().ok_or_else(|| {
        VaultError::BadRequest("data must be a JSON object".into())
    })?;

    let mut resp_map = serde_json::Map::new();

    for (field, value) in obj {
        let plaintext = value
            .as_str()
            .ok_or_else(|| VaultError::BadRequest("all fields must be strings".into()))?;

        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        let aad = format!("{}:{}", req.id, field);
        let ciphertext = state
            .crypto
            .encrypt(plaintext.as_bytes(), aad.as_bytes())?;

        state
            .storage
            .store(&token, ciphertext)
            .await?;

        resp_map.insert(field.clone(), json!(token));
    }

    let resp = TokenizeResponse {
        id: req.id,
        data: Value::Object(resp_map),
    };

    Ok((axum::http::StatusCode::CREATED, Json(resp)))
}

pub async fn detokenize(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<DetokenizeRequest>,
) -> Result<Json<Value>, VaultError> {
    check_api_key(&headers, &state.cfg, Operation::Detokenize)?;

    let obj = req.data.as_object().ok_or_else(|| {
        VaultError::BadRequest("data must be a JSON object".into())
    })?;

    let mut resp_map = serde_json::Map::new();

    for (field, token_val) in obj {
        let token = token_val
            .as_str()
            .ok_or_else(|| VaultError::BadRequest("all tokens must be strings".into()))?;

        let maybe_ct = state
            .storage
            .fetch(token)
            .await?;

        if let Some(ct) = maybe_ct {
            let aad = format!("{}:{}", req.id, field);
            let plaintext = state
                .crypto
                .decrypt(&ct, aad.as_bytes())?;

            let s = String::from_utf8(plaintext)
                .map_err(|_| VaultError::Crypto("invalid utf-8".into()))?;

            resp_map.insert(
                field.clone(),
                json!(DetokenizeField {
                    found: true,
                    value: s,
                }),
            );
        } else {
            resp_map.insert(
                field.clone(),
                json!(DetokenizeField {
                    found: false,
                    value: "".to_string(),
                }),
            );
        }
    }

    Ok(Json(Value::Object(resp_map)))
}