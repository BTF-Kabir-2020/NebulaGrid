use std::sync::Arc;

use async_nats::Client;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use serde_json::Value;
use tracing::{error, info};
use uuid::Uuid;

use crate::models::*;
use crate::service::NodeService;

pub async fn start_subscribers(nc: Client, service: Arc<NodeService>) {
    subscribe_node_register(&nc, service.clone()).await;
    subscribe_node_list(&nc, service.clone()).await;
    subscribe_node_get(&nc, service.clone()).await;
    subscribe_node_metrics(&nc, service.clone()).await;
    subscribe_node_metrics_latest(&nc, service.clone()).await;
    subscribe_node_metrics_history(&nc, service.clone()).await;
}

async fn subscribe_node_register(nc: &Client, service: Arc<NodeService>) {
    let mut sub = nc.subscribe("node.register").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let req: RegisterNodeRequest = match serde_json::from_slice(&msg.payload) {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse register request: {e}");
                    continue;
                }
            };
            let node = service.register_node(req);
            info!("Node registered: {} ({})", node.hostname, node.id);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&node) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_node_list(nc: &Client, service: Arc<NodeService>) {
    let mut sub = nc.subscribe("node.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let query: ListNodesQuery = match serde_json::from_slice(&msg.payload) {
                Ok(q) => q,
                Err(_) => ListNodesQuery {
                    status: None,
                    search: None,
                    page: None,
                    per_page: None,
                },
            };
            let result = service.list_nodes(query);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&result) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_node_get(nc: &Client, service: Arc<NodeService>) {
    let mut sub = nc.subscribe("node.get").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let id: Uuid = match serde_json::from_slice::<Value>(&msg.payload) {
                Ok(v) => match v.get("node_id").and_then(|v| v.as_str()) {
                    Some(s) => match s.parse() {
                        Ok(id) => id,
                        Err(_) => {
                            error!("Failed to parse node_id for node.get");
                            continue;
                        }
                    },
                    None => {
                        error!("Missing node_id in node.get");
                        continue;
                    }
                },
                Err(e) => {
                    error!("Failed to parse node.get request: {e}");
                    continue;
                }
            };
            let node = service.get_node(id);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&node) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_node_metrics(nc: &Client, service: Arc<NodeService>) {
    let mut sub = nc.subscribe("node.metrics").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let metrics: NodeMetrics = match serde_json::from_slice(&msg.payload) {
                Ok(m) => m,
                Err(e) => {
                    error!("Failed to parse node metrics: {e}");
                    continue;
                }
            };
            let node_id = metrics.node_id;
            service.store_metrics(node_id, metrics);
            info!("Metrics stored for node {node_id}");
            if let Some(reply) = msg.reply {
                let _ = nc.publish(reply, "ok".into()).await;
            }
        }
    });
}

async fn subscribe_node_metrics_latest(nc: &Client, service: Arc<NodeService>) {
    let mut sub = nc.subscribe("node.metrics.latest").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let node_id: Uuid = match serde_json::from_slice::<Value>(&msg.payload) {
                Ok(v) => match v.get("node_id").and_then(|v| v.as_str()) {
                    Some(s) => match s.parse() {
                        Ok(id) => id,
                        Err(_) => {
                            error!("Failed to parse node_id for node.metrics.latest");
                            continue;
                        }
                    },
                    None => {
                        error!("Missing node_id in node.metrics.latest");
                        continue;
                    }
                },
                Err(e) => {
                    error!("Failed to parse node.metrics.latest request: {e}");
                    continue;
                }
            };
            let metrics = service.get_latest_metrics(node_id);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&metrics) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_node_metrics_history(nc: &Client, service: Arc<NodeService>) {
    let mut sub = nc.subscribe("node.metrics.history").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let (node_id, from, to): (Uuid, Option<DateTime<Utc>>, Option<DateTime<Utc>>) =
                match serde_json::from_slice::<Value>(&msg.payload) {
                    Ok(v) => {
                        let id = v
                            .get("node_id")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok());
                        let from = v
                            .get("from")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok());
                        let to = v
                            .get("to")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok());
                        match id {
                            Some(id) => (id, from, to),
                            None => {
                                error!("Missing node_id in node.metrics.history");
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse node.metrics.history request: {e}");
                        continue;
                    }
                };
            let metrics = service.get_metrics_history(node_id, from, to);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&metrics) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}
