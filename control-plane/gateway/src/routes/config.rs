use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{AppState, StoredConfigEntry};

#[derive(Serialize)]
pub struct ConfigResponse {
    pub id: Uuid,
    pub key: String,
    pub value: serde_json::Value,
    pub group: String,
    pub description: Option<String>,
    pub version: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct SetConfigRequest {
    pub key: String,
    pub value: serde_json::Value,
    pub group: Option<String>,
    pub description: Option<String>,
}

fn to_response(e: &StoredConfigEntry) -> ConfigResponse {
    ConfigResponse {
        id: e.id,
        key: e.key.clone(),
        value: e.value.clone(),
        group: e.group.clone(),
        description: e.description.clone(),
        version: e.version,
        created_at: e.created_at.clone(),
        updated_at: e.updated_at.clone(),
    }
}

pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ConfigResponse>>, StatusCode> {
    let entries = state.config_entries.lock().await;
    Ok(Json(entries.iter().map(to_response).collect()))
}

pub async fn set(
    State(state): State<Arc<AppState>>,
    Json(body): Json<SetConfigRequest>,
) -> Result<Json<ConfigResponse>, StatusCode> {
    let now = chrono::Utc::now().to_rfc3339();
    let mut entries = state.config_entries.lock().await;
    if let Some(existing) = entries.iter_mut().find(|e| e.key == body.key) {
        existing.value = body.value;
        if let Some(g) = body.group {
            existing.group = g;
        }
        if let Some(d) = body.description {
            existing.description = Some(d);
        }
        existing.version += 1;
        existing.updated_at = now;
        return Ok(Json(to_response(existing)));
    }

    let entry = StoredConfigEntry {
        id: Uuid::new_v4(),
        key: body.key,
        value: body.value,
        group: body.group.unwrap_or_else(|| "default".into()),
        description: body.description,
        version: 1,
        created_at: now.clone(),
        updated_at: now,
    };
    let resp = to_response(&entry);
    entries.push(entry);
    Ok(Json(resp))
}

pub async fn get_by_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Result<Json<ConfigResponse>, StatusCode> {
    let entries = state.config_entries.lock().await;
    entries
        .iter()
        .find(|e| e.key == key)
        .map(to_response)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
