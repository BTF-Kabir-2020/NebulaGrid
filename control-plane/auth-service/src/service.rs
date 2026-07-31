use std::collections::HashMap;
use std::sync::Mutex;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::models::*;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("User not found")]
    UserNotFound,
    #[error("Internal error: {0}")]
    Internal(String),
}

pub struct AuthService {
    users: Vec<User>,
    jwt_secret: String,
    refresh_tokens: Mutex<HashMap<String, String>>,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(b"admin123", &salt)
            .unwrap()
            .to_string();

        let users = vec![User {
            id: Uuid::new_v4(),
            username: "admin".to_string(),
            email: "admin@nebula.local".to_string(),
            password_hash: hash,
            roles: vec!["admin".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }];

        Self {
            users,
            jwt_secret,
            refresh_tokens: Mutex::new(HashMap::new()),
        }
    }

    pub fn login(&self, username: &str, password: &str) -> Result<LoginResponse, AuthError> {
        let user = self
            .users
            .iter()
            .find(|u| u.username == username)
            .ok_or(AuthError::InvalidCredentials)?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| AuthError::Internal(e.to_string()))?;

        let argon2 = Argon2::default();
        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AuthError::InvalidCredentials)?;

        let now = Utc::now();
        let exp = (now + Duration::hours(24)).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            roles: user.roles.clone(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AuthError::Internal(e.to_string()))?;

        let refresh_token = Uuid::new_v4().to_string();
        self.refresh_tokens
            .lock()
            .unwrap()
            .insert(refresh_token.clone(), user.id.to_string());

        Ok(LoginResponse {
            token,
            refresh_token,
            expires_in: 86400,
        })
    }

    pub fn validate(&self, token: &str) -> Result<ValidateResponse, AuthError> {
        use jsonwebtoken::errors::ErrorKind;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        })?;

        let claims = token_data.claims;

        Ok(ValidateResponse {
            valid: true,
            user_id: Some(claims.sub),
            username: Some(claims.username),
            roles: Some(claims.roles),
        })
    }

    pub fn refresh(&self, refresh_token: &str) -> Result<LoginResponse, AuthError> {
        let user_id = self
            .refresh_tokens
            .lock()
            .unwrap()
            .remove(refresh_token)
            .ok_or(AuthError::InvalidToken)?;

        let user = self
            .users
            .iter()
            .find(|u| u.id.to_string() == user_id)
            .ok_or(AuthError::UserNotFound)?;

        let now = Utc::now();
        let exp = (now + Duration::hours(24)).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            roles: user.roles.clone(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AuthError::Internal(e.to_string()))?;

        let new_refresh_token = Uuid::new_v4().to_string();
        self.refresh_tokens
            .lock()
            .unwrap()
            .insert(new_refresh_token.clone(), user.id.to_string());

        Ok(LoginResponse {
            token,
            refresh_token: new_refresh_token,
            expires_in: 86400,
        })
    }

    pub fn get_user(&self, user_id: &str) -> Result<UserResponse, AuthError> {
        let user = self
            .users
            .iter()
            .find(|u| u.id.to_string() == user_id)
            .ok_or(AuthError::UserNotFound)?;

        Ok(UserResponse {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            roles: user.roles.clone(),
        })
    }
}
