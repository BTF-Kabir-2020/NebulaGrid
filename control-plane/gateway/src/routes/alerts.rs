use axum::{Json, extract::{State, Path, Query}, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;

use crate::state::AppState;

#[derive(Serialize)]
pub struct AlertResponse {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub severity: String,
    pub source: String,
    pub acknowledged: bool,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct AlertsQuery {
    pub severity: Option<String>,
    pub acknowledged: Option<bool>,
}

#[derive(Serialize)]
pub struct ActionResponse {
    pub success: bool,
    pub message: String,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AlertsQuery>,
) -> Result<Json<Vec<AlertResponse>>, StatusCode> {
    let alerts = state.alerts.lock().await;
    let filtered: Vec<_> = alerts.iter().filter(|a| {
        if let Some(ref severity) = query.severity {
            if a.severity != *severity { return false; }
        }
        if let Some(acknowledged) = query.acknowledged {
            if a.acknowledged != acknowledged { return false; }
        }
        true
    }).map(|a| AlertResponse {
        id: a.id,
        title: a.title.clone(),
        message: a.message.clone(),
        severity: a.severity.clone(),
        source: a.source.clone(),
        acknowledged: a.acknowledged,
        created_at: a.created_at.clone(),
    }).collect();
    Ok(Json(filtered))
}

pub async fn acknowledge(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ActionResponse>, StatusCode> {
    let mut alerts = state.alerts.lock().await;
    let alert = alerts.iter_mut().find(|a| a.id == id).ok_or(StatusCode::NOT_FOUND)?;
    alert.acknowledged = true;
    Ok(Json(ActionResponse {
        success: true,
        message: "Alert acknowledged".into(),
    }))
}
