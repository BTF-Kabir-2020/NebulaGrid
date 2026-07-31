use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Node {
    pub id: Uuid,
    pub hostname: String,
    pub ip_address: String,
    pub status: String,
    pub os_name: String,
    pub os_version: String,
    pub cpu_cores: i32,
    pub ram_total_bytes: i64,
    pub disk_total_bytes: i64,
    pub labels: serde_json::Value,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeMetrics {
    pub node_id: Uuid,
    pub cpu_percent: f64,
    pub ram_percent: f64,
    pub ram_used_bytes: i64,
    pub ram_total_bytes: i64,
    pub disk_percent: f64,
    pub disk_used_bytes: i64,
    pub disk_total_bytes: i64,
    pub net_rx_bytes: i64,
    pub net_tx_bytes: i64,
    pub collected_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterNodeRequest {
    pub hostname: String,
    pub ip_address: String,
    pub os_name: String,
    pub os_version: String,
    pub cpu_cores: i32,
    pub ram_total_bytes: i64,
    pub disk_total_bytes: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListNodesQuery {
    pub status: Option<String>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
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
