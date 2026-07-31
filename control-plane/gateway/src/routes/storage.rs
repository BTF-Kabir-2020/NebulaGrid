use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{AppState, StoredStoragePool, StoredVolume};

#[derive(Serialize)]
pub struct PoolResponse {
    pub id: Uuid,
    pub name: String,
    pub pool_type: String,
    pub total_bytes: i64,
    pub used_bytes: i64,
    pub free_bytes: i64,
    pub mount_path: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct VolumeResponse {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub name: String,
    pub size_bytes: i64,
    pub volume_type: String,
    pub format: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreatePoolRequest {
    pub name: String,
    pub pool_type: String,
    pub total_bytes: i64,
    pub mount_path: Option<String>,
}

fn pool_to_response(p: &StoredStoragePool) -> PoolResponse {
    PoolResponse {
        id: p.id,
        name: p.name.clone(),
        pool_type: p.pool_type.clone(),
        total_bytes: p.total_bytes,
        used_bytes: p.used_bytes,
        free_bytes: p.free_bytes,
        mount_path: p.mount_path.clone(),
        status: p.status.clone(),
        created_at: p.created_at.clone(),
    }
}

fn volume_to_response(v: &StoredVolume) -> VolumeResponse {
    VolumeResponse {
        id: v.id,
        pool_id: v.pool_id,
        name: v.name.clone(),
        size_bytes: v.size_bytes,
        volume_type: v.volume_type.clone(),
        format: v.format.clone(),
        status: v.status.clone(),
        created_at: v.created_at.clone(),
    }
}

pub async fn list_pools(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PoolResponse>>, StatusCode> {
    let pools = state.storage_pools.lock().await;
    Ok(Json(pools.iter().map(pool_to_response).collect()))
}

pub async fn list_volumes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<VolumeResponse>>, StatusCode> {
    let volumes = state.volumes.lock().await;
    Ok(Json(volumes.iter().map(volume_to_response).collect()))
}

pub async fn create_pool(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreatePoolRequest>,
) -> Result<(StatusCode, Json<PoolResponse>), StatusCode> {
    let pool = StoredStoragePool {
        id: Uuid::new_v4(),
        name: body.name,
        pool_type: body.pool_type,
        total_bytes: body.total_bytes,
        used_bytes: 0,
        free_bytes: body.total_bytes,
        mount_path: body.mount_path,
        status: "online".into(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    let resp = pool_to_response(&pool);
    state.storage_pools.lock().await.push(pool);
    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn get_pool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<PoolResponse>, StatusCode> {
    let pools = state.storage_pools.lock().await;
    pools
        .iter()
        .find(|p| p.id == id)
        .map(pool_to_response)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
pub struct CreateVolumeRequest {
    pub pool_id: Uuid,
    pub name: String,
    pub size_bytes: i64,
    pub volume_type: Option<String>,
    pub format: Option<String>,
}

pub async fn create_volume(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateVolumeRequest>,
) -> Result<(StatusCode, Json<VolumeResponse>), StatusCode> {
    {
        let pools = state.storage_pools.lock().await;
        if !pools.iter().any(|p| p.id == body.pool_id) {
            return Err(StatusCode::BAD_REQUEST);
        }
    }
    let volume = StoredVolume {
        id: Uuid::new_v4(),
        pool_id: body.pool_id,
        name: body.name,
        size_bytes: body.size_bytes,
        volume_type: body.volume_type.unwrap_or_else(|| "raw".into()),
        format: body.format.unwrap_or_else(|| "ext4".into()),
        status: "available".into(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    {
        let mut pools = state.storage_pools.lock().await;
        if let Some(pool) = pools.iter_mut().find(|p| p.id == body.pool_id) {
            pool.used_bytes += body.size_bytes;
            pool.free_bytes = (pool.total_bytes - pool.used_bytes).max(0);
        }
    }
    let resp = volume_to_response(&volume);
    state.volumes.lock().await.push(volume);
    Ok((StatusCode::CREATED, Json(resp)))
}

#[derive(Deserialize)]
pub struct UpdatePoolRequest {
    pub name: Option<String>,
    pub pool_type: Option<String>,
    pub total_bytes: Option<i64>,
    pub mount_path: Option<Option<String>>,
    pub status: Option<String>,
}

pub async fn update_pool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePoolRequest>,
) -> Result<Json<PoolResponse>, StatusCode> {
    let mut pools = state.storage_pools.lock().await;
    let pool = pools.iter_mut().find(|p| p.id == id).ok_or(StatusCode::NOT_FOUND)?;
    if let Some(name) = body.name {
        pool.name = name;
    }
    if let Some(pool_type) = body.pool_type {
        pool.pool_type = pool_type;
    }
    if let Some(total) = body.total_bytes {
        pool.total_bytes = total;
        pool.free_bytes = (total - pool.used_bytes).max(0);
    }
    if let Some(mount_path) = body.mount_path {
        pool.mount_path = mount_path;
    }
    if let Some(status) = body.status {
        pool.status = status;
    }
    Ok(Json(pool_to_response(pool)))
}

pub async fn delete_pool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    {
        let volumes = state.volumes.lock().await;
        if volumes.iter().any(|v| v.pool_id == id) {
            return StatusCode::CONFLICT;
        }
    }
    let mut pools = state.storage_pools.lock().await;
    let before = pools.len();
    pools.retain(|p| p.id != id);
    if pools.len() == before {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::NO_CONTENT
    }
}

#[derive(Deserialize)]
pub struct UpdateVolumeRequest {
    pub name: Option<String>,
    pub size_bytes: Option<i64>,
    pub volume_type: Option<String>,
    pub format: Option<String>,
    pub status: Option<String>,
}

pub async fn update_volume(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateVolumeRequest>,
) -> Result<Json<VolumeResponse>, StatusCode> {
    let mut volumes = state.volumes.lock().await;
    let volume = volumes.iter_mut().find(|v| v.id == id).ok_or(StatusCode::NOT_FOUND)?;
    let old_size = volume.size_bytes;
    let pool_id = volume.pool_id;
    if let Some(name) = body.name {
        volume.name = name;
    }
    if let Some(size) = body.size_bytes {
        volume.size_bytes = size;
    }
    if let Some(volume_type) = body.volume_type {
        volume.volume_type = volume_type;
    }
    if let Some(format) = body.format {
        volume.format = format;
    }
    if let Some(status) = body.status {
        volume.status = status;
    }
    let new_size = volume.size_bytes;
    let resp = volume_to_response(volume);
    drop(volumes);
    if new_size != old_size {
        let mut pools = state.storage_pools.lock().await;
        if let Some(pool) = pools.iter_mut().find(|p| p.id == pool_id) {
            pool.used_bytes = (pool.used_bytes - old_size + new_size).max(0);
            pool.free_bytes = (pool.total_bytes - pool.used_bytes).max(0);
        }
    }
    Ok(Json(resp))
}

pub async fn delete_volume(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut volumes = state.volumes.lock().await;
    let Some(idx) = volumes.iter().position(|v| v.id == id) else {
        return StatusCode::NOT_FOUND;
    };
    let removed = volumes.remove(idx);
    drop(volumes);
    let mut pools = state.storage_pools.lock().await;
    if let Some(pool) = pools.iter_mut().find(|p| p.id == removed.pool_id) {
        pool.used_bytes = (pool.used_bytes - removed.size_bytes).max(0);
        pool.free_bytes = (pool.total_bytes - pool.used_bytes).max(0);
    }
    StatusCode::NO_CONTENT
}
