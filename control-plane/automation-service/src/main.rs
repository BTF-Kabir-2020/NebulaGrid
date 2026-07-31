mod config;
mod models;
mod nats;
mod service;

use axum::{routing::get, Json, Router};
use serde_json::json;
use std::sync::Arc;
use tracing::info;

use crate::config::Config;
use crate::service::JobService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    info!(
        "{} starting on port {}",
        env!("CARGO_PKG_NAME"),
        cfg.http_port
    );
    info!("connecting to NATS at {}", cfg.nats_url);

    let nc = nats::connect(&cfg.nats_url)
        .await
        .map_err(|e| anyhow::anyhow!("NATS connection failed: {e}"))?;
    let svc = Arc::new(JobService::new());

    nats::subscribe_all(nc, svc.clone())
        .await
        .map_err(|e| anyhow::anyhow!("NATS subscribe failed: {e}"))?;

    let app = Router::new().route("/health", get(health));

    let addr = format!("0.0.0.0:{}", cfg.http_port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| anyhow::anyhow!("bind failed: {e}"))?;
    info!("HTTP server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| anyhow::anyhow!("server error: {e}"))?;

    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "service": env!("CARGO_PKG_NAME") }))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("shutdown signal received, starting graceful shutdown");
}
