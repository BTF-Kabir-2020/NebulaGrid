use axum::{Router, routing::get, Extension, Json};
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use std::sync::LazyLock;
use std::time::Instant;

mod config;
mod routes;
mod middleware;
mod ws;
mod grpc;
mod state;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    uptime_seconds: u64,
}

static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        uptime_seconds: START_TIME.elapsed().as_secs(),
    })
}

async fn ready_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ready".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        uptime_seconds: START_TIME.elapsed().as_secs(),
    })
}

pub fn create_router(state: Arc<state::AppState>) -> Router {
    let hub = ws::hub::WsHub::new();

    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/health/ready", get(ready_check))
        .route("/ws", get(ws::handler::ws_handler))
        .merge(routes::create_routes(state))
        .layer(Extension(hub))
        .layer(axum::middleware::from_fn(middleware::logging::request_logging))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=debug".into()))
        .init();

    let config = config::GatewayConfig::load();
    let jwt_secret = if config.jwt_secret.is_empty() {
        "nebula-grid-default-secret-2025".into()
    } else {
        config.jwt_secret.clone()
    };

    tracing::info!("Starting NebulaGrid Gateway v{}", env!("CARGO_PKG_VERSION"));

    let grpc_addr = SocketAddr::from(([0, 0, 0, 0], config.grpc_port));
    tokio::spawn(async move {
        grpc::server::start_grpc_server(grpc_addr).await;
    });

    let state = Arc::new(state::AppState::new(jwt_secret));
    let app = create_router(state)
        .layer(config.cors_layer())
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.http_port));
    tracing::info!("Gateway listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests;

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    tracing::info!("Shutdown signal received, starting graceful shutdown...");
}
