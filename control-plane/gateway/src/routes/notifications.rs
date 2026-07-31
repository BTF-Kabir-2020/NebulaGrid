use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{AppState, Claims, NotificationPrefs as StoredPrefs};

#[derive(Serialize)]
pub struct NotificationPrefsResponse {
    pub email_alerts: bool,
    pub email_digest: bool,
    pub browser_alerts: bool,
    pub slack_webhook: Option<String>,
}

pub async fn get_prefs(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<NotificationPrefsResponse>, StatusCode> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let prefs = state.notification_prefs.lock().await;
    let user_prefs = prefs.get(&user_id).cloned().unwrap_or(StoredPrefs {
        email_alerts: true,
        email_digest: false,
        browser_alerts: true,
        slack_webhook: None,
    });
    Ok(Json(NotificationPrefsResponse {
        email_alerts: user_prefs.email_alerts,
        email_digest: user_prefs.email_digest,
        browser_alerts: user_prefs.browser_alerts,
        slack_webhook: user_prefs.slack_webhook,
    }))
}

#[derive(Deserialize)]
pub struct UpdatePrefsRequest {
    pub email_alerts: Option<bool>,
    pub email_digest: Option<bool>,
    pub browser_alerts: Option<bool>,
    pub slack_webhook: Option<Option<String>>,
}

#[derive(Serialize)]
pub struct UpdatePrefsResponse {
    pub message: String,
}

pub async fn update_prefs(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<UpdatePrefsRequest>,
) -> Result<Json<UpdatePrefsResponse>, StatusCode> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let mut prefs = state.notification_prefs.lock().await;
    let entry = prefs.entry(user_id).or_insert(StoredPrefs {
        email_alerts: true,
        email_digest: false,
        browser_alerts: true,
        slack_webhook: None,
    });
    if let Some(v) = body.email_alerts {
        entry.email_alerts = v;
    }
    if let Some(v) = body.email_digest {
        entry.email_digest = v;
    }
    if let Some(v) = body.browser_alerts {
        entry.browser_alerts = v;
    }
    if let Some(v) = body.slack_webhook {
        entry.slack_webhook = v;
    }
    Ok(Json(UpdatePrefsResponse {
        message: "Notification preferences updated".into(),
    }))
}
