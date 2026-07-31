use std::sync::Arc;

use axum::Router;
use reqwest::Client;
use tokio::net::TcpListener;

use crate::state::AppState;

fn test_state() -> Arc<AppState> {
    Arc::new(AppState::new("test-secret".into()))
}

fn test_router() -> Router {
    let state = test_state();
    super::create_router(state)
}

async fn spawn_app() -> String {
    let app = test_router();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    format!("http://{}", addr)
}

#[tokio::test]
async fn test_health_returns_200() {
    let addr = spawn_app().await;
    let client = Client::new();
    let resp = client
        .get(format!("{}/api/health", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "ok");
    assert!(body["uptime_seconds"].as_u64().is_some());
}

#[tokio::test]
async fn test_health_ready_returns_200() {
    let addr = spawn_app().await;
    let client = Client::new();
    let resp = client
        .get(format!("{}/api/health/ready", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "ready");
}

#[tokio::test]
async fn test_login_valid_credentials_returns_200() {
    let addr = spawn_app().await;
    let client = Client::new();
    let resp = client
        .post(format!("{}/api/auth/login", addr))
        .json(&serde_json::json!({
            "username": "admin",
            "password": "admin123"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(!body["token"].as_str().unwrap().is_empty());
    assert!(!body["refresh_token"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn test_login_invalid_credentials_returns_401() {
    let addr = spawn_app().await;
    let client = Client::new();
    let resp = client
        .post(format!("{}/api/auth/login", addr))
        .json(&serde_json::json!({
            "username": "admin",
            "password": "wrongpassword"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401);
}

#[tokio::test]
async fn test_nodes_list_requires_auth() {
    let addr = spawn_app().await;
    let client = Client::new();
    let resp = client
        .get(format!("{}/api/nodes", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401);
}

#[tokio::test]
async fn test_nodes_list_with_auth_returns_200() {
    let addr = spawn_app().await;
    let client = Client::new();

    let login_resp = client
        .post(format!("{}/api/auth/login", addr))
        .json(&serde_json::json!({
            "username": "admin",
            "password": "admin123"
        }))
        .send()
        .await
        .unwrap();
    let login_body: serde_json::Value = login_resp.json().await.unwrap();
    let token = login_body["token"].as_str().unwrap();

    let resp = client
        .get(format!("{}/api/nodes", addr))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let nodes: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert!(nodes.len() >= 2);
}

#[tokio::test]
async fn test_nonexistent_route_returns_401_unauthorized() {
    let addr = spawn_app().await;
    let client = Client::new();
    let resp = client
        .get(format!("{}/api/nonexistent", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401);
}
