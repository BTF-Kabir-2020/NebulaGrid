use clap::Parser;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
use tracing_subscriber::EnvFilter;

mod config;
mod collector;
mod docker;
mod grpc;
mod reporter;
mod watcher;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let cfg = config::AgentConfig::parse();
    let interval = Duration::from_secs(cfg.interval_seconds);

    if cfg.verbose {
        tracing::info!("Verbose mode enabled");
    }

    tracing::info!(
        "NebulaGrid Agent starting — server: {}, interval: {}s, grpc_port: {}",
        cfg.server, cfg.interval_seconds, cfg.grpc_port
    );

    let grpc_addr = SocketAddr::from(([0, 0, 0, 0], cfg.grpc_port));
    tokio::spawn(async move {
        if let Err(e) = grpc::start_server(grpc_addr).await {
            tracing::error!("gRPC server failed: {e}");
        }
    });

    let reporter = reporter::MetricsReporter::new(cfg.server.clone(), cfg.token.clone());

    if !cfg.no_docker {
        tokio::spawn(async {
            match docker::DockerManager::connect().await {
                Ok(_) => tracing::info!("Docker connected"),
                Err(e) => tracing::warn!("Docker unavailable: {e}"),
            }
        });
    }

    {
        tokio::spawn(async {
            loop {
                match watcher::CommandWatcher::watch().await {
                    Ok(Some(cmd)) => {
                        tracing::info!("Received command: {} {:?}", cmd.command, cmd.args);
                        let _ = watcher::CommandWatcher::ack_command(&cmd.id).await;
                    }
                    Ok(None) => {}
                    Err(e) => tracing::warn!("Command watch error: {e}"),
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }

    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    let os_info = collector::os::get_os_info(&sys);
    let mem_info = collector::memory::get_memory_info(&sys);
    let disk_info = collector::disk::get_disk_info();
    let ip_address = get_local_ip();

    let node_id = register_with_gateway(&cfg, &os_info, &mem_info, &disk_info, &ip_address).await;

    tracing::info!("Agent node_id: {node_id}");

    loop {
        sys.refresh_all();

        let cpu = collector::cpu::get_cpu_usage(&sys);
        let mem = collector::memory::get_memory_info(&sys);
        let disk = collector::disk::get_disk_info();
        let net = collector::network::get_network_info();
        let procs = collector::process::get_process_count(&sys);

        let snapshot = collector::MetricsSnapshot {
            cpu_percent: cpu,
            ram_percent: mem.percent,
            ram_used_bytes: mem.used_bytes,
            ram_total_bytes: mem.total_bytes,
            disk_percent: disk.percent,
            disk_used_bytes: disk.used_bytes,
            disk_total_bytes: disk.total_bytes,
            net_rx_bytes: net.rx_bytes,
            net_tx_bytes: net.tx_bytes,
            process_count: procs,
            load_avg_1min: 0.0,
            load_avg_5min: 0.0,
            load_avg_15min: 0.0,
        };

        if cfg.verbose {
            tracing::debug!(
                "Metrics — CPU: {:.1}% RAM: {:.1}% Disk: {:.1}% Processes: {}",
                snapshot.cpu_percent,
                snapshot.ram_percent,
                snapshot.disk_percent,
                snapshot.process_count,
            );
        }

        if let Err(e) = reporter.send_metrics(&snapshot, &node_id).await {
            tracing::error!("Failed to send metrics: {e}");
        }

        tokio::time::sleep(interval).await;
    }
}

fn get_local_ip() -> String {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                return addr.ip().to_string();
            }
        }
    }
    "0.0.0.0".to_string()
}

async fn register_with_gateway(
    cfg: &config::AgentConfig,
    os_info: &collector::os::OsInfo,
    mem_info: &collector::memory::MemoryInfo,
    disk_info: &collector::disk::DiskInfo,
    ip: &str,
) -> String {
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "hostname": os_info.hostname,
        "ip_address": ip,
        "os_name": os_info.os_name,
        "os_version": os_info.os_version,
        "cpu_cores": os_info.cpu_cores,
        "ram_total_bytes": mem_info.total_bytes,
        "disk_total_bytes": disk_info.total_bytes,
    });

    match client
        .post(&format!("{}/api/nodes/register", cfg.server))
        .header("Authorization", format!("Bearer {}", cfg.token))
        .json(&body)
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            let node: serde_json::Value = resp.json().await.unwrap_or_default();
            let id = node["id"].as_str().unwrap_or("").to_string();
            if !id.is_empty() {
                tracing::info!("Registered as node {id}");
                return id;
            }
            tracing::warn!("Registration response missing node id, generating local");
            uuid::Uuid::new_v4().to_string()
        }
        Ok(resp) => {
            tracing::warn!("Registration failed: {}", resp.status());
            uuid::Uuid::new_v4().to_string()
        }
        Err(e) => {
            tracing::warn!("Registration error: {e}, using auto-generated node id");
            uuid::Uuid::new_v4().to_string()
        }
    }
}
