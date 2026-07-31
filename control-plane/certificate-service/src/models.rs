use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Certificate {
    pub id: Uuid,
    pub name: String,
    pub common_name: String,
    pub san: Vec<String>,
    pub issuer: String,
    pub serial_number: String,
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
    pub status: String,
    pub fingerprint: String,
    pub ca_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CertificateAuthority {
    pub id: Uuid,
    pub name: String,
    pub common_name: String,
    pub is_root: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCertificateRequest {
    pub name: String,
    pub common_name: String,
    pub san: Vec<String>,
    pub ca_id: Option<Uuid>,
    pub validity_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RenewCertificateRequest {
    pub cert_id: Uuid,
    pub validity_days: Option<i32>,
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
