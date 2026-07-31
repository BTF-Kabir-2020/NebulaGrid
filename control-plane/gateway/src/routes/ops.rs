use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{AppState, StoredAuditEvent, StoredBackup};

#[derive(Serialize)]
pub struct AuditEventResponse {
    pub id: Uuid,
    pub actor: String,
    pub action: String,
    pub resource: String,
    pub details: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct BackupResponse {
    pub id: Uuid,
    pub name: String,
    pub backup_type: String,
    pub status: String,
    pub size_bytes: i64,
    pub created_at: String,
    pub restored_at: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateBackupRequest {
    pub name: Option<String>,
    pub backup_type: Option<String>,
}

fn audit_to_response(e: &StoredAuditEvent) -> AuditEventResponse {
    AuditEventResponse {
        id: e.id,
        actor: e.actor.clone(),
        action: e.action.clone(),
        resource: e.resource.clone(),
        details: e.details.clone(),
        created_at: e.created_at.clone(),
    }
}

fn backup_to_response(b: &StoredBackup) -> BackupResponse {
    BackupResponse {
        id: b.id,
        name: b.name.clone(),
        backup_type: b.backup_type.clone(),
        status: b.status.clone(),
        size_bytes: b.size_bytes,
        created_at: b.created_at.clone(),
        restored_at: b.restored_at.clone(),
    }
}

pub async fn list_audit(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<AuditEventResponse>>, StatusCode> {
    let events = state.audit_events.lock().await;
    Ok(Json(events.iter().map(audit_to_response).collect()))
}

pub async fn list_backups(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<BackupResponse>>, StatusCode> {
    let backups = state.backups.lock().await;
    Ok(Json(backups.iter().map(backup_to_response).collect()))
}

pub async fn create_backup(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateBackupRequest>,
) -> Result<(StatusCode, Json<BackupResponse>), StatusCode> {
    let now = chrono::Utc::now().to_rfc3339();
    let backup_type = body.backup_type.unwrap_or_else(|| "full".into());
    let name = body
        .name
        .unwrap_or_else(|| format!("backup-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")));
    let backup = StoredBackup {
        id: Uuid::new_v4(),
        name,
        backup_type: backup_type.clone(),
        status: "completed".into(),
        size_bytes: 256 * 1024 * 1024,
        created_at: now.clone(),
        restored_at: None,
    };
    let resp = backup_to_response(&backup);
    state.backups.lock().await.insert(0, backup);
    state.audit_events.lock().await.insert(
        0,
        StoredAuditEvent {
            id: Uuid::new_v4(),
            actor: "admin".into(),
            action: "backup.create".into(),
            resource: format!("backup/{}", resp.id),
            details: format!("Created {} backup", backup_type),
            created_at: now,
        },
    );
    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn restore_backup(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<BackupResponse>, StatusCode> {
    let now = chrono::Utc::now().to_rfc3339();
    let mut backups = state.backups.lock().await;
    let backup = backups
        .iter_mut()
        .find(|b| b.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;
    backup.status = "restored".into();
    backup.restored_at = Some(now.clone());
    let resp = backup_to_response(backup);
    drop(backups);
    state.audit_events.lock().await.insert(
        0,
        StoredAuditEvent {
            id: Uuid::new_v4(),
            actor: "admin".into(),
            action: "backup.restore".into(),
            resource: format!("backup/{}", id),
            details: format!("Restored backup {}", resp.name),
            created_at: now,
        },
    );
    Ok(Json(resp))
}

pub async fn delete_backup(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> StatusCode {
    let mut backups = state.backups.lock().await;
    let before = backups.len();
    backups.retain(|b| b.id != id);
    if backups.len() == before {
        return StatusCode::NOT_FOUND;
    }
    drop(backups);
    state.audit_events.lock().await.insert(
        0,
        StoredAuditEvent {
            id: Uuid::new_v4(),
            actor: "admin".into(),
            action: "backup.delete".into(),
            resource: format!("backup/{}", id),
            details: "Deleted backup".into(),
            created_at: chrono::Utc::now().to_rfc3339(),
        },
    );
    StatusCode::NO_CONTENT
}
