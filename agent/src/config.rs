use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct AgentConfig {
    #[arg(short, long, env = "AGENT_SERVER_URL")]
    pub server: String,

    #[arg(short, long, env = "AGENT_TOKEN")]
    pub token: String,

    #[arg(short = 'n', long = "name", env = "AGENT_HOSTNAME")]
    pub hostname: Option<String>,

    #[arg(short, long, default_value = "10", env = "METRICS_INTERVAL_SECONDS")]
    pub interval_seconds: u64,

    #[arg(short, long, default_value = "9000")]
    pub grpc_port: u16,

    #[arg(long)]
    pub no_docker: bool,

    #[arg(short, long, default_value = "false")]
    pub verbose: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_interval() {
        let cfg = AgentConfig {
            server: "http://localhost:8080".into(),
            token: "test".into(),
            hostname: None,
            interval_seconds: 10,
            grpc_port: 9000,
            no_docker: false,
            verbose: false,
        };
        assert_eq!(cfg.interval_seconds, 10);
        assert_eq!(cfg.grpc_port, 9000);
    }

    #[test]
    fn test_default_grpc_port() {
        let cfg = AgentConfig {
            server: "http://localhost:8080".into(),
            token: "test".into(),
            hostname: None,
            interval_seconds: 10,
            grpc_port: 9000,
            no_docker: false,
            verbose: false,
        };
        assert_eq!(cfg.grpc_port, 9000);
    }
}
