use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{AppState, StoredNetwork};

#[derive(Serialize)]
pub struct NetworkResponse {
    pub id: Uuid,
    pub name: String,
    pub subnet: String,
    pub gateway: Option<String>,
    pub vlan_id: Option<i32>,
    pub network_type: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreateNetworkRequest {
    pub name: String,
    pub subnet: String,
    pub gateway: Option<String>,
    pub vlan_id: Option<i32>,
    pub network_type: Option<String>,
}

fn to_response(n: &StoredNetwork) -> NetworkResponse {
    NetworkResponse {
        id: n.id,
        name: n.name.clone(),
        subnet: n.subnet.clone(),
        gateway: n.gateway.clone(),
        vlan_id: n.vlan_id,
        network_type: n.network_type.clone(),
        status: n.status.clone(),
        created_at: n.created_at.clone(),
    }
}

pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<NetworkResponse>>, StatusCode> {
    let networks = state.networks.lock().await;
    Ok(Json(networks.iter().map(to_response).collect()))
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<NetworkResponse>, StatusCode> {
    let networks = state.networks.lock().await;
    networks
        .iter()
        .find(|n| n.id == id)
        .map(to_response)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateNetworkRequest>,
) -> Result<(StatusCode, Json<NetworkResponse>), StatusCode> {
    let network = StoredNetwork {
        id: Uuid::new_v4(),
        name: body.name,
        subnet: body.subnet,
        gateway: body.gateway,
        vlan_id: body.vlan_id,
        network_type: body.network_type.unwrap_or_else(|| "bridge".into()),
        status: "active".into(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    let resp = to_response(&network);
    state.networks.lock().await.push(network);
    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut networks = state.networks.lock().await;
    let before = networks.len();
    networks.retain(|n| n.id != id);
    if networks.len() == before {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}
