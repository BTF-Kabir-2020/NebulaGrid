use axum::{Json, extract::{State, Path}, http::StatusCode};
use serde::Serialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::state::{AppState, StoredContainer};

#[derive(Serialize)]
pub struct ContainerResponse {
    pub id: Uuid,
    pub container_id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub node_id: Uuid,
    pub ports: Vec<String>,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct ContainerDetailResponse {
    pub id: Uuid,
    pub container_id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub node_id: Uuid,
    pub ports: Vec<String>,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct ActionResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct LogsResponse {
    pub logs: String,
}

fn to_container_response(c: &StoredContainer) -> ContainerResponse {
    ContainerResponse {
        id: c.id,
        container_id: c.container_id.clone(),
        name: c.name.clone(),
        image: c.image.clone(),
        status: c.status.clone(),
        node_id: c.node_id,
        ports: c.ports.clone(),
        created_at: c.created_at.clone(),
    }
}

fn to_detail_response(c: &StoredContainer) -> ContainerDetailResponse {
    ContainerDetailResponse {
        id: c.id,
        container_id: c.container_id.clone(),
        name: c.name.clone(),
        image: c.image.clone(),
        status: c.status.clone(),
        node_id: c.node_id,
        ports: c.ports.clone(),
        created_at: c.created_at.clone(),
    }
}

pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ContainerResponse>>, StatusCode> {
    let containers = state.containers.lock().await;
    Ok(Json(containers.iter().map(to_container_response).collect()))
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ContainerDetailResponse>, StatusCode> {
    let containers = state.containers.lock().await;
    containers.iter().find(|c| c.id == id).map(to_detail_response).map(Json).ok_or(StatusCode::NOT_FOUND)
}

pub async fn start(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ActionResponse>, StatusCode> {
    let mut containers = state.containers.lock().await;
    let container = containers.iter_mut().find(|c| c.id == id).ok_or(StatusCode::NOT_FOUND)?;
    if container.status == "running" {
        return Ok(Json(ActionResponse { success: false, message: "Container already running".into() }));
    }
    container.status = "running".into();
    Ok(Json(ActionResponse { success: true, message: format!("Container {} started", container.name) }))
}

pub async fn stop(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ActionResponse>, StatusCode> {
    let mut containers = state.containers.lock().await;
    let container = containers.iter_mut().find(|c| c.id == id).ok_or(StatusCode::NOT_FOUND)?;
    if container.status == "stopped" {
        return Ok(Json(ActionResponse { success: false, message: "Container already stopped".into() }));
    }
    container.status = "stopped".into();
    Ok(Json(ActionResponse { success: true, message: format!("Container {} stopped", container.name) }))
}

pub async fn restart(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ActionResponse>, StatusCode> {
    let mut containers = state.containers.lock().await;
    let container = containers.iter_mut().find(|c| c.id == id).ok_or(StatusCode::NOT_FOUND)?;
    let name = container.name.clone();
    container.status = "running".into();
    Ok(Json(ActionResponse { success: true, message: format!("Container {} restarted", name) }))
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut containers = state.containers.lock().await;
    let idx = containers.iter().position(|c| c.id == id);
    match idx {
        Some(i) => {
            containers.remove(i);
            StatusCode::NO_CONTENT
        }
        None => StatusCode::NOT_FOUND,
    }
}

pub async fn logs(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<LogsResponse>, StatusCode> {
    let containers = state.containers.lock().await;
    let container = containers.iter().find(|c| c.id == id).ok_or(StatusCode::NOT_FOUND)?;
    let ts = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S");
    let key = format!("{} {}", container.name, container.image).to_lowercase();
    let body = if key.contains("nginx") {
        format!(
            "[{ts}] nginx/{name}: worker process started\n\
[{ts}] {name} | configuring upstream backend-1:8080\n\
[{ts}] {name} | 10.0.1.10 - GET / HTTP/1.1 200\n\
[{ts}] {name} | 10.0.1.22 - GET /healthz HTTP/1.1 200\n\
[{ts}] {name} | 10.0.2.5 - POST /api/login HTTP/1.1 401\n\
[{ts}] {name} | reloading configuration (SIGHUP)\n\
[{ts}] {name} | SSL handshake completed for api.example.local",
            ts = ts,
            name = container.name
        )
    } else if key.contains("redis") {
        format!(
            "[{ts}] redis/{name}: Ready to accept connections\n\
[{ts}] {name} | DB 0: 128 keys\n\
[{ts}] {name} | Client connected 10.0.1.40:52311\n\
[{ts}] {name} | SET session:abc EX 3600\n\
[{ts}] {name} | GET cache:user:42 -> hit\n\
[{ts}] {name} | eviction:volatile-lru removed 3 keys\n\
[{ts}] {name} | background save started",
            ts = ts,
            name = container.name
        )
    } else if key.contains("postgres") || key.contains("pg") {
        format!(
            "[{ts}] postgres/{name}: database system is ready\n\
[{ts}] {name} | connection authorized: user=nebula database=nebula\n\
[{ts}] {name} | LOG:  checkpoint starting: time\n\
[{ts}] {name} | LOG:  duration: 12.4 ms  statement: SELECT 1\n\
[{ts}] {name} | LOG:  incomplete startup packet\n\
[{ts}] {name} | LOG:  autovacuum launcher started\n\
[{ts}] {name} | LOG:  checkpoint complete: wrote 42 buffers",
            ts = ts,
            name = container.name
        )
    } else {
        format!(
            "[{ts}] container/{name} image={image} started\n\
[{ts}] {name} | Listening on configured ports\n\
[{ts}] {name} | Health check passed\n\
[{ts}] {name} | Request processed OK\n\
[{ts}] {name} | WARN memory usage elevated",
            ts = ts,
            name = container.name,
            image = container.image
        )
    };
    Ok(Json(LogsResponse { logs: body }))
}
