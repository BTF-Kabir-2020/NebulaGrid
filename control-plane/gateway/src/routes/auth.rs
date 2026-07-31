use axum::{Json, extract::{State, Extension, Path}, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use uuid::Uuid;
use argon2::{
    PasswordHash, PasswordVerifier,
    PasswordHasher,
    Argon2,
};
use rand::Rng;

use crate::state::{AppState, Claims, StoredApiToken};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}

fn sign_jwt(state: &AppState, user_id: &str, username: &str, roles: &[String]) -> Result<String, StatusCode> {
    let now = Utc::now();
    let exp = (now + Duration::hours(24)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        roles: roles.to_vec(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let users = state.users.lock().await;
    let user = users.iter().find(|u| u.username == body.username).ok_or(StatusCode::UNAUTHORIZED)?;

    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = sign_jwt(&state, &user.id.to_string(), &user.username, &user.roles)?;
    let refresh_token = Uuid::new_v4().to_string();
    state.refresh_tokens.lock().await.insert(refresh_token.clone(), user.id.to_string());

    Ok(Json(LoginResponse {
        token,
        refresh_token,
        expires_in: 86400,
    }))
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, StatusCode> {
    let user_id = state.refresh_tokens.lock().await.remove(&body.refresh_token).ok_or(StatusCode::UNAUTHORIZED)?;
    let users = state.users.lock().await;
    let user = users.iter().find(|u| u.id.to_string() == user_id).ok_or(StatusCode::UNAUTHORIZED)?;

    let token = sign_jwt(&state, &user.id.to_string(), &user.username, &user.roles)?;
    let new_refresh_token = Uuid::new_v4().to_string();
    state.refresh_tokens.lock().await.insert(new_refresh_token.clone(), user_id);

    Ok(Json(RefreshResponse {
        token,
        refresh_token: new_refresh_token,
        expires_in: 86400,
    }))
}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
) -> Json<LogoutResponse> {
    state.refresh_tokens.lock().await.clear();
    Json(LogoutResponse {
        message: "Logged out successfully".into(),
    })
}

pub async fn me(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserInfo>, StatusCode> {
    let users = state.users.lock().await;
    let user = users.iter().find(|u| u.id.to_string() == claims.sub).ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(UserInfo {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        roles: user.roles.clone(),
    }))
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct ChangePasswordResponse {
    pub message: String,
}

pub async fn change_password(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<ChangePasswordRequest>,
) -> Result<Json<ChangePasswordResponse>, StatusCode> {
    let mut users = state.users.lock().await;
    let user = users.iter_mut().find(|u| u.id.to_string() == claims.sub).ok_or(StatusCode::NOT_FOUND)?;

    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Argon2::default()
        .verify_password(body.current_password.as_bytes(), &parsed_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let new_hash = Argon2::default()
        .hash_password(body.new_password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();
    user.password_hash = new_hash;

    Ok(Json(ChangePasswordResponse {
        message: "Password changed successfully".into(),
    }))
}

#[derive(Serialize)]
pub struct ApiTokenResponse {
    pub id: Uuid,
    pub name: String,
    pub token: String,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

pub async fn list_tokens(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ApiTokenResponse>>, StatusCode> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let tokens = state.api_tokens.lock().await;
    let user_tokens = tokens.get(&user_id).cloned().unwrap_or_default();
    Ok(Json(user_tokens.into_iter().map(|t| ApiTokenResponse {
        id: t.id,
        name: t.name,
        token: t.token,
        created_at: t.created_at,
        last_used_at: t.last_used_at,
    }).collect()))
}

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub id: Uuid,
    pub name: String,
    pub token: String,
    pub created_at: String,
}

pub async fn create_api_token(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateTokenRequest>,
) -> Result<Json<CreateTokenResponse>, StatusCode> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let token_id = Uuid::new_v4();
    let raw_token: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(48)
        .map(char::from)
        .collect();
    let api_token = format!("ng_{}", raw_token);
    let name = body.name.clone();

    let stored = StoredApiToken {
        id: token_id,
        name: body.name,
        token: api_token.clone(),
        created_at: Utc::now().to_rfc3339(),
        last_used_at: None,
    };

    let mut tokens = state.api_tokens.lock().await;
    tokens.entry(user_id).or_default().push(stored);

    Ok(Json(CreateTokenResponse {
        id: token_id,
        name,
        token: api_token,
        created_at: Utc::now().to_rfc3339(),
    }))
}

#[derive(Serialize)]
pub struct DeleteTokenResponse {
    pub message: String,
}

pub async fn delete_token(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Path(token_id): Path<Uuid>,
) -> Result<Json<DeleteTokenResponse>, StatusCode> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let mut tokens = state.api_tokens.lock().await;
    if let Some(user_tokens) = tokens.get_mut(&user_id) {
        user_tokens.retain(|t| t.id != token_id);
        Ok(Json(DeleteTokenResponse {
            message: "Token revoked successfully".into(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
