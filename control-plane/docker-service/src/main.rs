use std::sync::Arc;

use axum::{routing::get, Json, Router};
use bollard::Docker;
use serde::Serialize;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

mod config;
mod models;
mod nats;
mod service;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}

#[tokio::main]
async fn main() {
    let cfg = config::DockerServiceConfig::load();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| cfg.rust_log.clone().into()),
        )
        .init();

    tracing::info!(
        "Starting NebulaGrid Docker Service v{}",
        env!("CARGO_PKG_VERSION")
    );

    let nc = async_nats::connect(&cfg.nats_url)
        .await
        .expect("Failed to connect to NATS");
    tracing::info!("Connected to NATS at {}", cfg.nats_url);

    let docker = connect_docker(&cfg.docker_socket).await;
    let svc = Arc::new(service::DockerService::new(docker));

    nats::start_nats_handlers(nc.clone(), svc.clone()).await;

    let app = Router::new().route("/health", get(health));

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.http_port));
    tracing::info!("Docker Service listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server exited with error");
}

async fn connect_docker(socket_path: &str) -> Option<Arc<Docker>> {
    if cfg!(target_os = "windows") {
        // On Windows, try named pipe first, then TCP
        match Docker::connect_with_local(
            "npipe:////./pipe/docker_engine",
            120,
            bollard::API_DEFAULT_VERSION,
        ) {
            Ok(d) => {
                tracing::info!("Connected to Docker via named pipe");
                return Some(Arc::new(d));
            }
            Err(e) => {
                tracing::warn!("Failed to connect to Docker via named pipe: {e}");
            }
        }

        match Docker::connect_with_http("http://localhost:2375", 120, bollard::API_DEFAULT_VERSION)
        {
            Ok(d) => {
                tracing::info!("Connected to Docker via TCP");
                return Some(Arc::new(d));
            }
            Err(e) => {
                tracing::warn!("Failed to connect to Docker via TCP: {e}");
            }
        }
    } else {
        match Docker::connect_with_local(socket_path, 120, bollard::API_DEFAULT_VERSION) {
            Ok(d) => {
                tracing::info!("Connected to Docker via socket: {socket_path}");
                return Some(Arc::new(d));
            }
            Err(e) => {
                tracing::warn!("Failed to connect to Docker via socket: {e}");
            }
        }
    }

    tracing::error!(
        "Docker is not available. Service will start but container operations will fail."
    );
    None
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    tracing::info!("Shutdown signal received, starting graceful shutdown...");
}
