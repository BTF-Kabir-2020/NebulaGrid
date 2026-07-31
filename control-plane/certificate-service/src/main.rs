mod config;
mod models;
mod nats;
mod service;

use std::sync::Arc;

use axum::{routing::get, Json, Router};
use serde_json::json;
use tracing::info;

use crate::config::Config;
use crate::nats::start_subscribers;
use crate::service::CertificateService;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nebula_certificate_service.".into()),
        )
        .init();

    let config = Config::load();
    info!("Starting NebulaGrid certificate-service");

    let nc = async_nats::connect(&config.nats_url).await.unwrap();
    info!("Connected to NATS at {}", config.nats_url);

    let service = Arc::new(CertificateService::new());

    start_subscribers(nc, service.clone()).await;

    let app = Router::new().route(
        "/health",
        get(|| async move {
            Json(json!({
                "status": "ok",
                "service": "nebula-certificate-service"
            }))
        }),
    );

    let addr = format!("0.0.0.0:{}", config.http_port);
    info!("HTTP server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
