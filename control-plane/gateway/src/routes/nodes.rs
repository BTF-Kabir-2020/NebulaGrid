use axum::{Json, extract::{State, Path, Extension}, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;

use crate::state::{AppState, StoredNode, StoredNodeMetrics};
use crate::ws::hub::WsHub;

#[derive(Serialize)]
pub struct NodeResponse {
    pub id: Uuid,
    pub hostname: String,
    pub ip_address: String,
    pub status: String,
    pub cpu_percent: f64,
    pub ram_percent: f64,
    pub disk_percent: f64,
    pub os_name: String,
    pub last_seen_at: String,
}

#[derive(Deserialize)]
pub struct RegisterNodeRequest {
    pub hostname: String,
    pub ip_address: String,
    pub os_name: String,
}

#[derive(Serialize)]
pub struct NodeMetricsResponse {
    pub node_id: Uuid,
    pub cpu_percent: f64,
    pub ram_percent: f64,
    pub disk_percent: f64,
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
    pub collected_at: String,
}

#[derive(Deserialize)]
pub struct UpdateNodeRequest {
    pub hostname: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct CommandResponse {
    pub success: bool,
    pub message: String,
}

fn node_to_response(n: &StoredNode) -> NodeResponse {
    NodeResponse {
        id: n.id,
        hostname: n.hostname.clone(),
        ip_address: n.ip_address.clone(),
        status: n.status.clone(),
        cpu_percent: n.cpu_percent,
        ram_percent: n.ram_percent,
        disk_percent: n.disk_percent,
        os_name: n.os_name.clone(),
        last_seen_at: n.last_seen_at.clone(),
    }
}

fn metrics_to_response(m: &StoredNodeMetrics) -> NodeMetricsResponse {
    NodeMetricsResponse {
        node_id: m.node_id,
        cpu_percent: m.cpu_percent,
        ram_percent: m.ram_percent,
        disk_percent: m.disk_percent,
        net_rx_bytes: m.net_rx_bytes,
        net_tx_bytes: m.net_tx_bytes,
        collected_at: m.collected_at.clone(),
    }
}

pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<NodeResponse>>, StatusCode> {
    let nodes = state.nodes.lock().await;
    Ok(Json(nodes.iter().map(node_to_response).collect()))
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterNodeRequest>,
) -> Result<Json<NodeResponse>, StatusCode> {
    let mut nodes = state.nodes.lock().await;
    let node = StoredNode {
        id: Uuid::new_v4(),
        hostname: body.hostname,
        ip_address: body.ip_address,
        status: "online".into(),
        cpu_percent: 0.0,
        ram_percent: 0.0,
        disk_percent: 0.0,
        os_name: body.os_name,
        last_seen_at: chrono::Utc::now().to_rfc3339(),
    };
    let response = node_to_response(&node);
    nodes.push(node);
    Ok(Json(response))
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<NodeResponse>, StatusCode> {
    let nodes = state.nodes.lock().await;
    nodes.iter().find(|n| n.id == id).map(node_to_response).map(Json).ok_or(StatusCode::NOT_FOUND)
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateNodeRequest>,
) -> Result<Json<NodeResponse>, StatusCode> {
    let mut nodes = state.nodes.lock().await;
    let node = nodes.iter_mut().find(|n| n.id == id).ok_or(StatusCode::NOT_FOUND)?;
    if let Some(hostname) = body.hostname {
        node.hostname = hostname;
    }
    if let Some(status) = body.status {
        node.status = status;
    }
    Ok(Json(node_to_response(node)))
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut nodes = state.nodes.lock().await;
    let idx = nodes.iter().position(|n| n.id == id);
    match idx {
        Some(i) => {
            nodes.remove(i);
            StatusCode::NO_CONTENT
        }
        None => StatusCode::NOT_FOUND,
    }
}

pub async fn metrics_history(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<NodeMetricsResponse>>, StatusCode> {
    let metrics_map = state.node_metrics.lock().await;
    let metrics = metrics_map.get(&id).cloned().unwrap_or_default();
    Ok(Json(metrics.iter().map(metrics_to_response).collect()))
}

pub async fn metrics_latest(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<NodeMetricsResponse>, StatusCode> {
    let metrics_map = state.node_metrics.lock().await;
    metrics_map.get(&id)
        .and_then(|v| v.last())
        .map(metrics_to_response)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn execute_command(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<CommandResponse>, StatusCode> {
    let nodes = state.nodes.lock().await;
    if nodes.iter().any(|n| n.id == id) {
        Ok(Json(CommandResponse {
            success: true,
            message: "Command queued for execution on node".into(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
pub struct SubmitMetricsRequest {
    pub cpu_percent: f64,
    pub ram_percent: f64,
    pub disk_percent: f64,
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
}

pub async fn submit_metrics(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Extension(hub): Extension<WsHub>,
    Json(body): Json<SubmitMetricsRequest>,
) -> Result<Json<NodeMetricsResponse>, StatusCode> {
    let exists = state.nodes.lock().await.iter().any(|n| n.id == id);
    if !exists {
        return Err(StatusCode::NOT_FOUND);
    }
    let metrics = StoredNodeMetrics {
        node_id: id,
        cpu_percent: body.cpu_percent,
        ram_percent: body.ram_percent,
        disk_percent: body.disk_percent,
        net_rx_bytes: body.net_rx_bytes,
        net_tx_bytes: body.net_tx_bytes,
        collected_at: chrono::Utc::now().to_rfc3339(),
    };
    {
        let mut nodes = state.nodes.lock().await;
        if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
            node.cpu_percent = body.cpu_percent;
            node.ram_percent = body.ram_percent;
            node.disk_percent = body.disk_percent;
            node.status = "online".into();
            node.last_seen_at = metrics.collected_at.clone();
        }
    }
    state.node_metrics.lock().await.entry(id).or_default().push(metrics.clone());
    let response = metrics_to_response(&metrics);
    if let Ok(data) = serde_json::to_value(&response) {
        hub.publish_typed("metrics", "metrics", &data).await;
    }
    Ok(Json(response))
}
