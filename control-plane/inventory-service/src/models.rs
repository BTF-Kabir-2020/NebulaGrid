use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryItem {
    pub id: Uuid,
    pub asset_tag: String,
    pub name: String,
    pub item_type: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub location: Option<String>,
    pub rack_id: Option<String>,
    pub rack_position: Option<i32>,
    pub status: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventorySummary {
    pub total_items: i64,
    pub by_type: serde_json::Value,
    pub by_status: serde_json::Value,
    pub by_location: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct RegisterItemRequest {
    pub asset_tag: Option<String>,
    pub name: String,
    pub item_type: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub location: Option<String>,
    pub rack_id: Option<String>,
    pub rack_position: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ListInventoryQuery {
    pub item_type: Option<String>,
    pub status: Option<String>,
    pub location: Option<String>,
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
