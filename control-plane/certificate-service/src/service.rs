use std::sync::Mutex;

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::models::*;

pub struct AppState {
    pub certificates: Vec<Certificate>,
    pub authorities: Vec<CertificateAuthority>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            certificates: Vec::new(),
            authorities: Vec::new(),
        }
    }
}

pub struct CertificateService {
    state: Mutex<AppState>,
}

impl CertificateService {
    pub fn new() -> Self {
        let service = Self {
            state: Mutex::new(AppState::new()),
        };
        let now = Utc::now();
        {
            let mut state = service.state.lock().unwrap();
            state.authorities.push(CertificateAuthority {
                id: Uuid::new_v4(),
                name: "Nebula Root CA".into(),
                common_name: "NebulaGrid Root CA".into(),
                is_root: true,
                created_at: now,
                expires_at: now + Duration::days(3650),
            });
        }
        service
    }

    pub fn create_certificate(&self, req: CreateCertificateRequest) -> Certificate {
        let now = Utc::now();
        let days = req.validity_days.unwrap_or(365) as i64;
        let ca_name = {
            let state = self.state.lock().unwrap();
            req.ca_id
                .and_then(|id| state.authorities.iter().find(|c| c.id == id))
                .map(|c| c.common_name.clone())
                .unwrap_or_else(|| "NebulaGrid Root CA".into())
        };

        let cert = Certificate {
            id: Uuid::new_v4(),
            name: req.name,
            common_name: req.common_name.clone(),
            san: req.san,
            issuer: ca_name,
            serial_number: format!("{:x}", Uuid::new_v4().as_u128()),
            not_before: now,
            not_after: now + Duration::days(days),
            status: "valid".into(),
            fingerprint: format!("sha256:{}", Uuid::new_v4().to_string().replace('-', "")),
            ca_id: req.ca_id,
            created_at: now,
            updated_at: now,
        };
        self.state.lock().unwrap().certificates.push(cert.clone());
        cert
    }

    pub fn list_certificates(&self, page: i64, per_page: i64) -> PaginatedResponse<Certificate> {
        let state = self.state.lock().unwrap();
        paginate(&state.certificates, page, per_page)
    }

    pub fn list_authorities(&self) -> Vec<CertificateAuthority> {
        self.state.lock().unwrap().authorities.clone()
    }

    pub fn renew(&self, req: RenewCertificateRequest) -> Result<Certificate, String> {
        let mut state = self.state.lock().unwrap();
        let cert = state
            .certificates
            .iter_mut()
            .find(|c| c.id == req.cert_id)
            .ok_or_else(|| "certificate not found".to_string())?;
        let days = req.validity_days.unwrap_or(365) as i64;
        let now = Utc::now();
        cert.not_before = now;
        cert.not_after = now + Duration::days(days);
        cert.status = "valid".into();
        cert.updated_at = now;
        cert.serial_number = format!("{:x}", Uuid::new_v4().as_u128());
        Ok(cert.clone())
    }

    pub fn revoke(&self, id: Uuid) -> Option<Certificate> {
        let mut state = self.state.lock().unwrap();
        let cert = state.certificates.iter_mut().find(|c| c.id == id)?;
        cert.status = "revoked".into();
        cert.updated_at = Utc::now();
        Some(cert.clone())
    }
}

fn paginate<T: Clone + serde::Serialize>(
    items: &[T],
    page: i64,
    per_page: i64,
) -> PaginatedResponse<T> {
    let page = page.max(1);
    let per_page = per_page.clamp(1, 100);
    let total = items.len() as i64;
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i64;
    let offset = ((page - 1) * per_page) as usize;
    let data = items
        .iter()
        .skip(offset)
        .take(per_page as usize)
        .cloned()
        .collect();
    PaginatedResponse {
        data,
        page,
        per_page,
        total,
        total_pages,
        has_next: page < total_pages,
        has_prev: page > 1,
    }
}
