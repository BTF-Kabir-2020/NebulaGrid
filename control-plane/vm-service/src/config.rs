use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "nebula-vm-service")]
pub struct Config {
    #[arg(long, env = "NATS_URL", default_value = "nats://localhost:4222")]
    pub nats_url: String,

    #[arg(long, env = "HTTP_PORT", default_value_t = 5005)]
    pub http_port: u16,

    #[arg(long, env = "PROXMOX_URL", default_value = "https://localhost:8006")]
    pub proxmox_url: String,

    #[arg(long, env = "PROXMOX_TOKEN")]
    pub proxmox_token: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        Config::parse()
    }
}
