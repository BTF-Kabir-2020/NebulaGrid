use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigEntry {
    pub id: Uuid,
    pub key: String,
    pub value: serde_json::Value,
    pub group: String,
    pub description: Option<String>,
    pub version: i64,
    pub encrypted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigGroup {
    pub name: String,
    pub description: Option<String>,
    pub entry_count: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SetConfigRequest {
    pub key: String,
    pub value: serde_json::Value,
    pub group: Option<String>,
    pub description: Option<String>,
    pub encrypted: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ListConfigQuery {
    pub group: Option<String>,
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
