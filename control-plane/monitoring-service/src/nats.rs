use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use tracing::{error, info};

use crate::models::*;
use crate::service::MonitoringService;

pub async fn start_subscribers(nc: Client, service: Arc<MonitoringService>) {
    subscribe_overview(&nc, service.clone()).await;
    subscribe_list_alerts(&nc, service.clone()).await;
    subscribe_acknowledge_alert(&nc, service.clone()).await;
    subscribe_prometheus_query(&nc, service.clone()).await;
}

async fn subscribe_overview(nc: &Client, service: Arc<MonitoringService>) {
    let mut sub = nc.subscribe("monitoring.overview").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let overview = service.get_overview();
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&overview) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
    info!("Subscribed to monitoring.overview");
}

async fn subscribe_list_alerts(nc: &Client, service: Arc<MonitoringService>) {
    let mut sub = nc.subscribe("monitoring.alerts").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let query: ListAlertsQuery =
                serde_json::from_slice(&msg.payload).unwrap_or(ListAlertsQuery {
                    severity: None,
                    acknowledged: None,
                });

            let alerts = service.list_alerts(query);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&alerts) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
    info!("Subscribed to monitoring.alerts");
}

async fn subscribe_acknowledge_alert(nc: &Client, service: Arc<MonitoringService>) {
    let mut sub = nc.subscribe("monitoring.alert.ack").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let req: AcknowledgeAlert = match serde_json::from_slice(&msg.payload) {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse acknowledge request: {e}");
                    continue;
                }
            };

            match service.acknowledge_alert(req.id) {
                Ok(alert) => {
                    if let Some(reply) = msg.reply {
                        if let Ok(payload) = serde_json::to_vec(&alert) {
                            let _ = nc.publish(reply, payload.into()).await;
                        }
                    }
                }
                Err(e) => {
                    if let Some(reply) = msg.reply {
                        let err_payload = serde_json::json!({"error": e});
                        if let Ok(payload) = serde_json::to_vec(&err_payload) {
                            let _ = nc.publish(reply, payload.into()).await;
                        }
                    }
                }
            }
        }
    });
    info!("Subscribed to monitoring.alert.ack");
}

async fn subscribe_prometheus_query(nc: &Client, service: Arc<MonitoringService>) {
    let mut sub = nc.subscribe("monitoring.prometheus.query").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let req: PrometheusQuery = match serde_json::from_slice(&msg.payload) {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse prometheus query request: {e}");
                    continue;
                }
            };

            match service.prometheus_query(&req.query).await {
                Ok(result) => {
                    if let Some(reply) = msg.reply {
                        if let Ok(payload) = serde_json::to_vec(&result) {
                            let _ = nc.publish(reply, payload.into()).await;
                        }
                    }
                }
                Err(e) => {
                    if let Some(reply) = msg.reply {
                        let err_payload = serde_json::json!({"error": e});
                        if let Ok(payload) = serde_json::to_vec(&err_payload) {
                            let _ = nc.publish(reply, payload.into()).await;
                        }
                    }
                }
            }
        }
    });
    info!("Subscribed to monitoring.prometheus.query");
}
