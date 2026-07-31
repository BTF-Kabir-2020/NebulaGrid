use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone, clap::Parser)]
#[command(
    name = "nebula-gateway",
    version,
    about = "NebulaGrid Control Plane Gateway"
)]
pub struct GatewayConfig {
    #[arg(long, env = "HTTP_PORT", default_value = "8080")]
    pub http_port: u16,

    #[arg(long, env = "GRPC_PORT", default_value = "9000")]
    pub grpc_port: u16,

    #[arg(long, env = "DATABASE_URL")]
    pub database_url: String,

    #[arg(long, env = "REDIS_URL", default_value = "redis://localhost:6379")]
    pub redis_url: String,

    #[arg(long, env = "NATS_URL", default_value = "nats://localhost:4222")]
    pub nats_url: String,

    #[arg(long, env = "JWT_SECRET")]
    pub jwt_secret: String,

    #[arg(long, env = "JWT_EXPIRY_HOURS", default_value = "24")]
    pub jwt_expiry_hours: u32,

    #[arg(long, env = "AGENT_TOKEN")]
    pub agent_token: String,

    #[arg(long, env = "RUST_LOG", default_value = "info")]
    pub rust_log: String,
}

impl GatewayConfig {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();
        clap::Parser::parse()
    }

    pub fn cors_layer(&self) -> CorsLayer {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
            .max_age(Duration::from_secs(3600))
    }
}
