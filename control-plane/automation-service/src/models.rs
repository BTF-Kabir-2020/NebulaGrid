use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub job_type: String,
    pub status: String,
    pub target_nodes: Vec<String>,
    pub params: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub name: String,
    pub job_type: String,
    pub target_nodes: Vec<String>,
    pub params: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct JobQuery {
    pub status: Option<String>,
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
}

#[derive(Debug, Serialize)]
pub struct ActionResult {
    pub success: bool,
    pub message: String,
}
