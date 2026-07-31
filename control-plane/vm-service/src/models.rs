use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vm {
    pub id: Uuid,
    pub name: String,
    pub os_type: String,
    pub cpu_cores: i32,
    pub ram_mb: i64,
    pub disk_gb: i64,
    pub status: String,
    pub node_id: Uuid,
    pub proxmox_vmid: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVmRequest {
    pub name: String,
    pub os_type: String,
    pub cpu_cores: i32,
    pub ram_mb: i64,
    pub disk_gb: i64,
    pub node_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct VmAction {
    pub id: Uuid,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateSnapshotRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct VmSnapshot {
    pub id: Uuid,
    pub vm_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ActionResult {
    pub success: bool,
    pub message: String,
}
