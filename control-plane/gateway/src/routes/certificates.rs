use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{AppState, StoredCertificate};

#[derive(Serialize)]
pub struct CertificateResponse {
    pub id: Uuid,
    pub name: String,
    pub common_name: String,
    pub issuer: String,
    pub status: String,
    pub not_after: String,
    pub fingerprint: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreateCertificateRequest {
    pub name: String,
    pub common_name: String,
    pub validity_days: Option<i32>,
}

fn to_response(c: &StoredCertificate) -> CertificateResponse {
    CertificateResponse {
        id: c.id,
        name: c.name.clone(),
        common_name: c.common_name.clone(),
        issuer: c.issuer.clone(),
        status: c.status.clone(),
        not_after: c.not_after.clone(),
        fingerprint: c.fingerprint.clone(),
        created_at: c.created_at.clone(),
    }
}

pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CertificateResponse>>, StatusCode> {
    let certs = state.certificates.lock().await;
    Ok(Json(certs.iter().map(to_response).collect()))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateCertificateRequest>,
) -> Result<(StatusCode, Json<CertificateResponse>), StatusCode> {
    let days = body.validity_days.unwrap_or(365);
    let not_after = (chrono::Utc::now() + chrono::Duration::days(days as i64)).to_rfc3339();
    let cert = StoredCertificate {
        id: Uuid::new_v4(),
        name: body.name,
        common_name: body.common_name,
        issuer: "NebulaGrid Root CA".into(),
        status: "valid".into(),
        not_after,
        fingerprint: format!("sha256:{}", &Uuid::new_v4().to_string().replace('-', "")[..16]),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    let resp = to_response(&cert);
    state.certificates.lock().await.push(cert);
    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn revoke(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<CertificateResponse>, StatusCode> {
    let mut certs = state.certificates.lock().await;
    let cert = certs.iter_mut().find(|c| c.id == id).ok_or(StatusCode::NOT_FOUND)?;
    cert.status = "revoked".into();
    Ok(Json(to_response(cert)))
}
