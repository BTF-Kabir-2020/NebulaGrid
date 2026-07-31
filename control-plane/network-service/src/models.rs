use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    pub id: Uuid,
    pub name: String,
    pub subnet: String,
    pub gateway: Option<String>,
    pub vlan_id: Option<i32>,
    pub network_type: String,
    pub status: String,
    pub labels: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirewallRule {
    pub id: Uuid,
    pub network_id: Uuid,
    pub name: String,
    pub direction: String,
    pub protocol: String,
    pub source: String,
    pub destination: String,
    pub ports: Option<String>,
    pub action: String,
    pub priority: i32,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNetworkRequest {
    pub name: String,
    pub subnet: String,
    pub gateway: Option<String>,
    pub vlan_id: Option<i32>,
    pub network_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFirewallRuleRequest {
    pub network_id: Uuid,
    pub name: String,
    pub direction: String,
    pub protocol: String,
    pub source: String,
    pub destination: String,
    pub ports: Option<String>,
    pub action: String,
    pub priority: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkAttachment {
    pub network_id: Uuid,
    pub target_id: String,
    pub target_type: String,
    pub ip_address: Option<String>,
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
