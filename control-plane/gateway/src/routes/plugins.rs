use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{AppState, StoredPlugin};

#[derive(Serialize)]
pub struct PluginResponse {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: String,
    pub enabled: bool,
    pub config: serde_json::Value,
    pub installed_at: String,
}

#[derive(Deserialize)]
pub struct CreatePluginRequest {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub config: Option<serde_json::Value>,
    pub enabled: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdatePluginRequest {
    pub enabled: Option<bool>,
    pub description: Option<String>,
    pub config: Option<serde_json::Value>,
    pub version: Option<String>,
}

fn to_response(p: &StoredPlugin) -> PluginResponse {
    PluginResponse {
        id: p.id,
        name: p.name.clone(),
        version: p.version.clone(),
        description: p.description.clone(),
        category: p.category.clone(),
        enabled: p.enabled,
        config: p.config.clone(),
        installed_at: p.installed_at.clone(),
    }
}

pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PluginResponse>>, StatusCode> {
    let plugins = state.plugins.lock().await;
    Ok(Json(plugins.iter().map(to_response).collect()))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreatePluginRequest>,
) -> Result<(StatusCode, Json<PluginResponse>), StatusCode> {
    let plugin = StoredPlugin {
        id: Uuid::new_v4(),
        name: body.name,
        version: body.version.unwrap_or_else(|| "1.0.0".into()),
        description: body.description.unwrap_or_default(),
        category: body.category.unwrap_or_else(|| "general".into()),
        enabled: body.enabled.unwrap_or(true),
        config: body.config.unwrap_or_else(|| serde_json::json!({})),
        installed_at: chrono::Utc::now().to_rfc3339(),
    };
    let resp = to_response(&plugin);
    state.plugins.lock().await.push(plugin);
    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<PluginResponse>, StatusCode> {
    let plugins = state.plugins.lock().await;
    plugins
        .iter()
        .find(|p| p.id == id)
        .map(to_response)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePluginRequest>,
) -> Result<Json<PluginResponse>, StatusCode> {
    let mut plugins = state.plugins.lock().await;
    let plugin = plugins
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;
    if let Some(enabled) = body.enabled {
        plugin.enabled = enabled;
    }
    if let Some(description) = body.description {
        plugin.description = description;
    }
    if let Some(config) = body.config {
        plugin.config = config;
    }
    if let Some(version) = body.version {
        plugin.version = version;
    }
    Ok(Json(to_response(plugin)))
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut plugins = state.plugins.lock().await;
    let before = plugins.len();
    plugins.retain(|p| p.id != id);
    if plugins.len() == before {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}
