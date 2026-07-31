use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub job_type: String,
    pub target_id: String,
    pub payload: serde_json::Value,
    pub status: String,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobSchedule {
    pub id: Uuid,
    pub name: String,
    pub job_type: String,
    pub cron_expr: String,
    pub target_id: String,
    pub payload: serde_json::Value,
    pub enabled: bool,
    pub last_run_at: Option<DateTime<Utc>>,
    pub next_run_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub name: String,
    pub job_type: String,
    pub target_id: String,
    pub payload: serde_json::Value,
    pub scheduled_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateScheduleRequest {
    pub name: String,
    pub job_type: String,
    pub cron_expr: String,
    pub target_id: String,
    pub payload: serde_json::Value,
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
