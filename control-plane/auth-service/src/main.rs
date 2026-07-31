mod config;
mod models;
mod nats;
mod service;

use std::sync::Arc;

use axum::{routing::get, Json, Router};
use serde_json::json;
use tokio::signal;
use tracing::info;

use crate::config::Config;
use crate::nats::start_subscribers;
use crate::service::AuthService;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nebula_auth_service=info".into()),
        )
        .init();

    let config = Config::load();
    info!("Starting NebulaGrid Auth Service");

    let nc = async_nats::connect(&config.nats_url).await.unwrap();
    info!("Connected to NATS at {}", config.nats_url);

    let service = Arc::new(AuthService::new(config.jwt_secret.clone()));

    start_subscribers(nc, service.clone()).await;

    let app = Router::new().route(
        "/health",
        get(|| async move {
            Json(json!({
                "status": "ok",
                "service": "nebula-auth-service"
            }))
        }),
    );

    let addr = format!("0.0.0.0:{}", config.http_port);
    info!("HTTP server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received, shutting down gracefully");
}
