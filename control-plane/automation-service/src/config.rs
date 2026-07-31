use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "nebula-automation-service")]
pub struct Config {
    #[arg(long, env = "NATS_URL", default_value = "nats://localhost:4222")]
    pub nats_url: String,

    #[arg(long, env = "HTTP_PORT", default_value = "8085")]
    pub http_port: u16,
}

impl Config {
    pub fn parse() -> Self {
        dotenvy::dotenv().ok();
        Parser::parse()
    }
}
