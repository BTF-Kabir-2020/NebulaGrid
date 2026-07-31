use axum::{Json, extract::{State, Path}, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;

use crate::state::{AppState, StoredVm, StoredVmSnapshot};

#[derive(Serialize)]
pub struct VmResponse {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub os_type: String,
    pub cpu_cores: u32,
    pub ram_mb: u32,
    pub disk_gb: u32,
    pub node_id: Uuid,
    pub ip_address: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateVmRequest {
    pub name: String,
    pub os_type: String,
    pub cpu_cores: u32,
    pub ram_mb: u32,
    pub disk_gb: u32,
    pub node_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateVmRequest {
    pub name: Option<String>,
    pub cpu_cores: Option<u32>,
    pub ram_mb: Option<u32>,
    pub disk_gb: Option<u32>,
}

#[derive(Serialize)]
pub struct SnapshotResponse {
    pub name: String,
    pub created_at: String,
    pub size_bytes: u64,
}

#[derive(Serialize)]
pub struct ActionResponse {
    pub success: bool,
    pub message: String,
}

fn vm_to_response(vm: &StoredVm) -> VmResponse {
    VmResponse {
        id: vm.id,
        name: vm.name.clone(),
        status: vm.status.clone(),
        os_type: vm.os_type.clone(),
        cpu_cores: vm.cpu_cores,
        ram_mb: vm.ram_mb,
        disk_gb: vm.disk_gb,
        node_id: vm.node_id,
        ip_address: vm.ip_address.clone(),
    }
}

pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<VmResponse>>, StatusCode> {
    let vms = state.vms.lock().await;
    Ok(Json(vms.iter().map(vm_to_response).collect()))
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<VmResponse>, StatusCode> {
    let vms = state.vms.lock().await;
    vms.iter().find(|v| v.id == id).map(vm_to_response).map(Json).ok_or(StatusCode::NOT_FOUND)
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateVmRequest>,
) -> Result<Json<VmResponse>, StatusCode> {
    let mut vms = state.vms.lock().await;
    let vm = StoredVm {
        id: Uuid::new_v4(),
        name: body.name,
        os_type: body.os_type,
        cpu_cores: body.cpu_cores,
        ram_mb: body.ram_mb,
        disk_gb: body.disk_gb,
        status: "stopped".into(),
        node_id: body.node_id,
        ip_address: None,
    };
    let response = vm_to_response(&vm);
    vms.push(vm);
    Ok(Json(response))
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateVmRequest>,
) -> Result<Json<VmResponse>, StatusCode> {
    let mut vms = state.vms.lock().await;
    let vm = vms.iter_mut().find(|v| v.id == id).ok_or(StatusCode::NOT_FOUND)?;
    if let Some(name) = body.name { vm.name = name; }
    if let Some(cores) = body.cpu_cores { vm.cpu_cores = cores; }
    if let Some(ram) = body.ram_mb { vm.ram_mb = ram; }
    if let Some(disk) = body.disk_gb { vm.disk_gb = disk; }
    Ok(Json(vm_to_response(vm)))
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut vms = state.vms.lock().await;
    let idx = vms.iter().position(|v| v.id == id);
    match idx {
        Some(i) => {
            vms.remove(i);
            StatusCode::NO_CONTENT
        }
        None => StatusCode::NOT_FOUND,
    }
}

pub async fn start(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ActionResponse>, StatusCode> {
    let mut vms = state.vms.lock().await;
    let vm = vms.iter_mut().find(|v| v.id == id).ok_or(StatusCode::NOT_FOUND)?;
    if vm.status == "running" {
        return Ok(Json(ActionResponse { success: false, message: "VM already running".into() }));
    }
    vm.status = "running".into();
    Ok(Json(ActionResponse { success: true, message: format!("VM {} started", vm.name) }))
}

pub async fn stop(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ActionResponse>, StatusCode> {
    let mut vms = state.vms.lock().await;
    let vm = vms.iter_mut().find(|v| v.id == id).ok_or(StatusCode::NOT_FOUND)?;
    if vm.status == "stopped" {
        return Ok(Json(ActionResponse { success: false, message: "VM already stopped".into() }));
    }
    vm.status = "stopped".into();
    Ok(Json(ActionResponse { success: true, message: format!("VM {} stopped", vm.name) }))
}

pub async fn restart(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ActionResponse>, StatusCode> {
    let mut vms = state.vms.lock().await;
    let vm = vms.iter_mut().find(|v| v.id == id).ok_or(StatusCode::NOT_FOUND)?;
    let name = vm.name.clone();
    vm.status = "running".into();
    Ok(Json(ActionResponse { success: true, message: format!("VM {} restarted", name) }))
}

pub async fn create_snapshot(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<SnapshotResponse>, StatusCode> {
    let vms = state.vms.lock().await;
    if vms.iter().all(|v| v.id != id) {
        return Err(StatusCode::NOT_FOUND);
    }
    drop(vms);

    let mut snapshots = state.vm_snapshots.lock().await;
    let snapshot = StoredVmSnapshot {
        name: format!("snapshot-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")),
        created_at: chrono::Utc::now().to_rfc3339(),
        size_bytes: 1073741824,
        vm_id: id,
    };
    let response = SnapshotResponse {
        name: snapshot.name.clone(),
        created_at: snapshot.created_at.clone(),
        size_bytes: snapshot.size_bytes,
    };
    snapshots.push(snapshot);
    Ok(Json(response))
}

pub async fn list_snapshots(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<SnapshotResponse>>, StatusCode> {
    let snapshots = state.vm_snapshots.lock().await;
    let filtered: Vec<_> = snapshots.iter().filter(|s| s.vm_id == id).map(|s| SnapshotResponse {
        name: s.name.clone(),
        created_at: s.created_at.clone(),
        size_bytes: s.size_bytes,
    }).collect();
    Ok(Json(filtered))
}

pub async fn backup(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ActionResponse>, StatusCode> {
    let vms = state.vms.lock().await;
    let vm = vms.iter().find(|v| v.id == id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(ActionResponse {
        success: true,
        message: format!("Backup initiated for VM {}. Estimated completion: 5 minutes", vm.name),
    }))
}
