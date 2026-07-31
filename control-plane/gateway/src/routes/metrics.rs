use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use std::sync::Arc;

use crate::state::AppState;

#[derive(Serialize)]
pub struct OverviewResponse {
    pub total_nodes: u32,
    pub online_nodes: u32,
    pub total_containers: u32,
    pub total_vms: u32,
    pub active_alerts: u32,
    pub avg_cpu_percent: f64,
    pub avg_ram_percent: f64,
}

#[derive(Serialize)]
pub struct PrometheusQueryResponse {
    pub status: String,
    pub data: serde_json::Value,
}

pub async fn overview(
    State(state): State<Arc<AppState>>,
) -> Result<Json<OverviewResponse>, StatusCode> {
    let nodes = state.nodes.lock().await;
    let total_nodes = nodes.len() as u32;
    let online_nodes = nodes.iter().filter(|n| n.status == "online").count() as u32;
    let avg_cpu = nodes.iter().map(|n| n.cpu_percent).sum::<f64>() / total_nodes as f64;
    let avg_ram = nodes.iter().map(|n| n.ram_percent).sum::<f64>() / total_nodes as f64;
    drop(nodes);

    let containers = state.containers.lock().await;
    let total_containers = containers.len() as u32;
    drop(containers);

    let vms = state.vms.lock().await;
    let total_vms = vms.len() as u32;
    drop(vms);

    let alerts = state.alerts.lock().await;
    let active_alerts = alerts.iter().filter(|a| !a.acknowledged).count() as u32;
    drop(alerts);

    Ok(Json(OverviewResponse {
        total_nodes,
        online_nodes,
        total_containers,
        total_vms,
        active_alerts,
        avg_cpu_percent: (avg_cpu * 100.0).round() / 100.0,
        avg_ram_percent: (avg_ram * 100.0).round() / 100.0,
    }))
}

pub async fn prometheus_query(
    State(state): State<Arc<AppState>>,
) -> Result<Json<PrometheusQueryResponse>, StatusCode> {
    let nodes = state.nodes.lock().await;
    let mut series = Vec::new();
    for node in nodes.iter() {
        series.push(serde_json::json!({
            "metric": {
                "__name__": "node_cpu_percent",
                "hostname": node.hostname,
                "instance": node.ip_address,
            },
            "value": [chrono::Utc::now().timestamp(), format!("{:.1}", node.cpu_percent)],
        }));
        series.push(serde_json::json!({
            "metric": {
                "__name__": "node_memory_percent",
                "hostname": node.hostname,
                "instance": node.ip_address,
            },
            "value": [chrono::Utc::now().timestamp(), format!("{:.1}", node.ram_percent)],
        }));
    }

    Ok(Json(PrometheusQueryResponse {
        status: "success".into(),
        data: serde_json::json!({
            "resultType": "vector",
            "result": series,
        }),
    }))
}
