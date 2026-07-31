use axum::{Json, extract::{State, Path}, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};

use crate::state::{AppState, StoredUser};

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn to_response(user: &StoredUser) -> UserResponse {
    UserResponse {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
    }
}

pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let users = state.users.lock().await;
    Ok(Json(users.iter().map(to_response).collect()))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let mut users = state.users.lock().await;
    if users.iter().any(|u| u.username == body.username) {
        return Err(StatusCode::CONFLICT);
    }

    let user = StoredUser {
        id: Uuid::new_v4(),
        username: body.username,
        email: body.email,
        password_hash: hash_password(&body.password),
        roles: vec!["viewer".into()],
    };

    let response = to_response(&user);
    users.push(user);
    Ok(Json(response))
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, StatusCode> {
    let users = state.users.lock().await;
    users.iter().find(|u| u.id == id).map(to_response).map(Json).ok_or(StatusCode::NOT_FOUND)
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let mut users = state.users.lock().await;
    let user = users.iter_mut().find(|u| u.id == id).ok_or(StatusCode::NOT_FOUND)?;
    if let Some(email) = body.email {
        user.email = email;
    }
    Ok(Json(to_response(user)))
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut users = state.users.lock().await;
    let idx = users.iter().position(|u| u.id == id);
    match idx {
        Some(i) => {
            users.remove(i);
            StatusCode::NO_CONTENT
        }
        None => StatusCode::NOT_FOUND,
    }
}
