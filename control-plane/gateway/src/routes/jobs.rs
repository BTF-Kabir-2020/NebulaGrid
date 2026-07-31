use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{AppState, StoredJob};

#[derive(Serialize)]
pub struct JobResponse {
    pub id: Uuid,
    pub name: String,
    pub job_type: String,
    pub status: String,
    pub target_nodes: Vec<String>,
    pub params: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub name: String,
    pub job_type: String,
    pub target_nodes: Option<Vec<String>>,
    pub params: Option<serde_json::Value>,
}

fn to_response(j: &StoredJob) -> JobResponse {
    JobResponse {
        id: j.id,
        name: j.name.clone(),
        job_type: j.job_type.clone(),
        status: j.status.clone(),
        target_nodes: j.target_nodes.clone(),
        params: j.params.clone(),
        result: j.result.clone(),
        error: j.error.clone(),
        created_at: j.created_at.clone(),
        started_at: j.started_at.clone(),
        completed_at: j.completed_at.clone(),
    }
}

pub async fn list(State(state): State<Arc<AppState>>) -> Result<Json<Vec<JobResponse>>, StatusCode> {
    let jobs = state.jobs.lock().await;
    Ok(Json(jobs.iter().map(to_response).collect()))
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<JobResponse>, StatusCode> {
    let jobs = state.jobs.lock().await;
    jobs.iter()
        .find(|j| j.id == id)
        .map(to_response)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<JobResponse>), StatusCode> {
    let now = chrono::Utc::now().to_rfc3339();
    let job = StoredJob {
        id: Uuid::new_v4(),
        name: body.name,
        job_type: body.job_type,
        status: "pending".into(),
        target_nodes: body.target_nodes.unwrap_or_default(),
        params: body.params.unwrap_or(serde_json::json!({})),
        result: None,
        error: None,
        created_at: now.clone(),
        started_at: None,
        completed_at: None,
    };

    let job_id = job.id;
    {
        let mut jobs = state.jobs.lock().await;
        jobs.insert(0, job.clone());
    }

    let state_clone = state.clone();
    tokio::spawn(async move {
        {
            let mut jobs = state_clone.jobs.lock().await;
            if let Some(j) = jobs.iter_mut().find(|j| j.id == job_id) {
                j.status = "running".into();
                j.started_at = Some(chrono::Utc::now().to_rfc3339());
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let mut jobs = state_clone.jobs.lock().await;
        if let Some(j) = jobs.iter_mut().find(|j| j.id == job_id) {
            j.status = "completed".into();
            j.completed_at = Some(chrono::Utc::now().to_rfc3339());
            j.result = Some(serde_json::json!({
                "message": "Job completed successfully"
            }));
        }
    });

    Ok((StatusCode::CREATED, Json(to_response(&job))))
}
