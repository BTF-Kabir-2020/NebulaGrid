use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "nebula-auth-service")]
pub struct Config {
    #[arg(long, env = "NATS_URL", default_value = "nats://localhost:4222")]
    pub nats_url: String,

    #[arg(long, env = "HTTP_PORT", default_value_t = 8081)]
    pub http_port: u16,

    #[arg(long, env = "JWT_SECRET")]
    pub jwt_secret: String,

    #[arg(long, env = "DATABASE_URL")]
    pub db_url: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        Config::parse()
    }
}
