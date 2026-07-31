use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use tracing::{error, info};

use crate::models::*;
use crate::service::AuthService;

pub async fn start_subscribers(nc: Client, service: Arc<AuthService>) {
    subscribe_auth_login(&nc, service.clone()).await;
    subscribe_auth_validate(&nc, service.clone()).await;
    subscribe_auth_refresh(&nc, service.clone()).await;
    subscribe_auth_me(&nc, service.clone()).await;
}

async fn subscribe_auth_login(nc: &Client, service: Arc<AuthService>) {
    let mut sub = nc.subscribe("auth.login").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let req: LoginRequest = match serde_json::from_slice(&msg.payload) {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse login request: {e}");
                    continue;
                }
            };

            let result = service.login(&req.username, &req.password);
            match result {
                Ok(resp) => {
                    info!("Login successful for user: {}", req.username);
                    if let Some(reply) = msg.reply {
                        if let Ok(payload) = serde_json::to_vec(&resp) {
                            let _ = nc.publish(reply, payload.into()).await;
                        }
                    }
                }
                Err(e) => {
                    error!("Login failed for user {}: {e}", req.username);
                    if let Some(reply) = msg.reply {
                        let err_payload = serde_json::json!({"error": e.to_string()});
                        if let Ok(payload) = serde_json::to_vec(&err_payload) {
                            let _ = nc.publish(reply, payload.into()).await;
                        }
                    }
                }
            }
        }
    });
}

async fn subscribe_auth_validate(nc: &Client, service: Arc<AuthService>) {
    let mut sub = nc.subscribe("auth.validate").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let req: ValidateRequest = match serde_json::from_slice(&msg.payload) {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse validate request: {e}");
                    continue;
                }
            };

            let result = service.validate(&req.token);
            match result {
                Ok(resp) => {
                    if let Some(reply) = msg.reply {
                        if let Ok(payload) = serde_json::to_vec(&resp) {
                            let _ = nc.publish(reply, payload.into()).await;
                        }
                    }
                }
                Err(e) => {
                    if let Some(reply) = msg.reply {
                        let err_payload = serde_json::json!({"error": e.to_string()});
                        if let Ok(payload) = serde_json::to_vec(&err_payload) {
                            let _ = nc.publish(reply, payload.into()).await;
                        }
                    }
                }
            }
        }
    });
}

async fn subscribe_auth_refresh(nc: &Client, service: Arc<AuthService>) {
    let mut sub = nc.subscribe("auth.refresh").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let req: RefreshRequest = match serde_json::from_slice(&msg.payload) {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse refresh request: {e}");
                    continue;
                }
            };

            let result = service.refresh(&req.refresh_token);
            match result {
                Ok(resp) => {
                    if let Some(reply) = msg.reply {
                        if let Ok(payload) = serde_json::to_vec(&resp) {
                            let _ = nc.publish(reply, payload.into()).await;
                        }
                    }
                }
                Err(e) => {
                    if let Some(reply) = msg.reply {
                        let err_payload = serde_json::json!({"error": e.to_string()});
                        if let Ok(payload) = serde_json::to_vec(&err_payload) {
                            let _ = nc.publish(reply, payload.into()).await;
                        }
                    }
                }
            }
        }
    });
}

async fn subscribe_auth_me(nc: &Client, service: Arc<AuthService>) {
    let mut sub = nc.subscribe("auth.me").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let req: ValidateRequest = match serde_json::from_slice(&msg.payload) {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse auth.me request: {e}");
                    continue;
                }
            };

            let validate_result = service.validate(&req.token);
            match validate_result {
                Ok(v) => {
                    if let Some(user_id) = v.user_id {
                        match service.get_user(&user_id) {
                            Ok(user) => {
                                if let Some(reply) = msg.reply {
                                    if let Ok(payload) = serde_json::to_vec(&user) {
                                        let _ = nc.publish(reply, payload.into()).await;
                                    }
                                }
                            }
                            Err(e) => {
                                if let Some(reply) = msg.reply {
                                    let err_payload = serde_json::json!({"error": e.to_string()});
                                    if let Ok(payload) = serde_json::to_vec(&err_payload) {
                                        let _ = nc.publish(reply, payload.into()).await;
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    if let Some(reply) = msg.reply {
                        let err_payload = serde_json::json!({"error": e.to_string()});
                        if let Ok(payload) = serde_json::to_vec(&err_payload) {
                            let _ = nc.publish(reply, payload.into()).await;
                        }
                    }
                }
            }
        }
    });
}
