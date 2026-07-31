use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Clone)]
pub struct MonitoringOverview {
    pub total_nodes: i64,
    pub online_nodes: i64,
    pub total_containers: i64,
    pub total_vms: i64,
    pub active_alerts: i64,
    pub avg_cpu_percent: f64,
    pub avg_ram_percent: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alert {
    pub id: Uuid,
    pub node_id: Option<Uuid>,
    pub alert_type: String,
    pub severity: String,
    pub message: String,
    pub acknowledged: bool,
    pub created_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ListAlertsQuery {
    pub severity: Option<String>,
    pub acknowledged: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AcknowledgeAlert {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct PrometheusQuery {
    pub query: String,
    pub time: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PrometheusResult {
    pub status: String,
    pub data: serde_json::Value,
}
