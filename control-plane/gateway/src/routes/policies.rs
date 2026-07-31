use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{AppState, StoredPolicy};

#[derive(Serialize)]
pub struct PolicyResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub policy_type: String,
    pub scope: String,
    pub enforcement: String,
    pub priority: i32,
    pub enabled: bool,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreatePolicyRequest {
    pub name: String,
    pub description: Option<String>,
    pub policy_type: String,
    pub scope: String,
    pub enforcement: Option<String>,
    pub priority: Option<i32>,
}

fn to_response(p: &StoredPolicy) -> PolicyResponse {
    PolicyResponse {
        id: p.id,
        name: p.name.clone(),
        description: p.description.clone(),
        policy_type: p.policy_type.clone(),
        scope: p.scope.clone(),
        enforcement: p.enforcement.clone(),
        priority: p.priority,
        enabled: p.enabled,
        created_at: p.created_at.clone(),
    }
}

pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PolicyResponse>>, StatusCode> {
    let policies = state.policies.lock().await;
    Ok(Json(policies.iter().map(to_response).collect()))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreatePolicyRequest>,
) -> Result<(StatusCode, Json<PolicyResponse>), StatusCode> {
    let policy = StoredPolicy {
        id: Uuid::new_v4(),
        name: body.name,
        description: body.description,
        policy_type: body.policy_type,
        scope: body.scope,
        enforcement: body.enforcement.unwrap_or_else(|| "enforce".into()),
        priority: body.priority.unwrap_or(100),
        enabled: true,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    let resp = to_response(&policy);
    state.policies.lock().await.push(policy);
    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<PolicyResponse>, StatusCode> {
    let policies = state.policies.lock().await;
    policies
        .iter()
        .find(|p| p.id == id)
        .map(to_response)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
pub struct UpdatePolicyRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub policy_type: Option<String>,
    pub scope: Option<String>,
    pub enforcement: Option<String>,
    pub priority: Option<i32>,
    pub enabled: Option<bool>,
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePolicyRequest>,
) -> Result<Json<PolicyResponse>, StatusCode> {
    let mut policies = state.policies.lock().await;
    let policy = policies.iter_mut().find(|p| p.id == id).ok_or(StatusCode::NOT_FOUND)?;
    if let Some(name) = body.name {
        policy.name = name;
    }
    if let Some(description) = body.description {
        policy.description = Some(description);
    }
    if let Some(policy_type) = body.policy_type {
        policy.policy_type = policy_type;
    }
    if let Some(scope) = body.scope {
        policy.scope = scope;
    }
    if let Some(enforcement) = body.enforcement {
        policy.enforcement = enforcement;
    }
    if let Some(priority) = body.priority {
        policy.priority = priority;
    }
    if let Some(enabled) = body.enabled {
        policy.enabled = enabled;
    }
    Ok(Json(to_response(policy)))
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut policies = state.policies.lock().await;
    let before = policies.len();
    policies.retain(|p| p.id != id);
    if policies.len() == before {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}
