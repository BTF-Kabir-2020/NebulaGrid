#[derive(Clone, clap::Parser)]
#[command(name = "nebula-docker-service", version, about = "NebulaGrid Docker Container Service")]
pub struct DockerServiceConfig {
    #[arg(long, env = "NATS_URL", default_value = "nats://localhost:4222")]
    pub nats_url: String,

    #[arg(long, env = "HTTP_PORT", default_value = "8092")]
    pub http_port: u16,

    #[arg(long, env = "DOCKER_SOCKET", default_value = "/var/run/docker.sock")]
    pub docker_socket: String,

    #[arg(long, env = "RUST_LOG", default_value = "info")]
    pub rust_log: String,
}

impl DockerServiceConfig {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();
        clap::Parser::parse()
    }
}
