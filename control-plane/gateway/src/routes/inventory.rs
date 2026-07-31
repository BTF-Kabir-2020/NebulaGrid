use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{AppState, StoredInventoryItem};

#[derive(Serialize)]
pub struct InventoryResponse {
    pub id: Uuid,
    pub asset_tag: String,
    pub name: String,
    pub item_type: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub location: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreateInventoryRequest {
    pub asset_tag: Option<String>,
    pub name: String,
    pub item_type: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub location: Option<String>,
}

fn to_response(i: &StoredInventoryItem) -> InventoryResponse {
    InventoryResponse {
        id: i.id,
        asset_tag: i.asset_tag.clone(),
        name: i.name.clone(),
        item_type: i.item_type.clone(),
        manufacturer: i.manufacturer.clone(),
        model: i.model.clone(),
        location: i.location.clone(),
        status: i.status.clone(),
        created_at: i.created_at.clone(),
    }
}

pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<InventoryResponse>>, StatusCode> {
    let items = state.inventory.lock().await;
    Ok(Json(items.iter().map(to_response).collect()))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateInventoryRequest>,
) -> Result<(StatusCode, Json<InventoryResponse>), StatusCode> {
    let item = StoredInventoryItem {
        id: Uuid::new_v4(),
        asset_tag: body
            .asset_tag
            .unwrap_or_else(|| format!("AST-{}", &Uuid::new_v4().to_string()[..8])),
        name: body.name,
        item_type: body.item_type,
        manufacturer: body.manufacturer,
        model: body.model,
        location: body.location,
        status: "active".into(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    let resp = to_response(&item);
    state.inventory.lock().await.push(item);
    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<InventoryResponse>, StatusCode> {
    let items = state.inventory.lock().await;
    items
        .iter()
        .find(|i| i.id == id)
        .map(to_response)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
pub struct UpdateInventoryRequest {
    pub name: Option<String>,
    pub item_type: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub location: Option<String>,
    pub status: Option<String>,
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateInventoryRequest>,
) -> Result<Json<InventoryResponse>, StatusCode> {
    let mut items = state.inventory.lock().await;
    let item = items.iter_mut().find(|i| i.id == id).ok_or(StatusCode::NOT_FOUND)?;
    if let Some(name) = body.name {
        item.name = name;
    }
    if let Some(item_type) = body.item_type {
        item.item_type = item_type;
    }
    if let Some(manufacturer) = body.manufacturer {
        item.manufacturer = Some(manufacturer);
    }
    if let Some(model) = body.model {
        item.model = Some(model);
    }
    if let Some(location) = body.location {
        item.location = Some(location);
    }
    if let Some(status) = body.status {
        item.status = status;
    }
    Ok(Json(to_response(item)))
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut items = state.inventory.lock().await;
    let before = items.len();
    items.retain(|i| i.id != id);
    if items.len() == before {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}
