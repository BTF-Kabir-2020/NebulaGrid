use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoragePool {
    pub id: Uuid,
    pub name: String,
    pub pool_type: String,
    pub total_bytes: i64,
    pub used_bytes: i64,
    pub free_bytes: i64,
    pub mount_path: Option<String>,
    pub status: String,
    pub labels: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Volume {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub name: String,
    pub size_bytes: i64,
    pub volume_type: String,
    pub format: String,
    pub mount_path: Option<String>,
    pub attached_to: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snapshot {
    pub id: Uuid,
    pub volume_id: Uuid,
    pub name: String,
    pub size_bytes: i64,
    pub snapshot_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePoolRequest {
    pub name: String,
    pub pool_type: String,
    pub total_bytes: i64,
    pub mount_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVolumeRequest {
    pub pool_id: Uuid,
    pub name: String,
    pub size_bytes: i64,
    pub volume_type: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    pub has_next: bool,
    pub has_prev: bool,
}
