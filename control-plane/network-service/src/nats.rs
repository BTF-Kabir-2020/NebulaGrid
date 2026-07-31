use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use serde_json::Value;
use tracing::{error, info};
use uuid::Uuid;

use crate::models::*;
use crate::service::NetworkService;

pub async fn start_subscribers(nc: Client, service: Arc<NetworkService>) {
    info!("Starting NATS subscribers for network-service");
    subscribe_create(&nc, service.clone()).await;
    subscribe_list(&nc, service.clone()).await;
    subscribe_get(&nc, service.clone()).await;
    subscribe_delete(&nc, service.clone()).await;
    subscribe_fw_add(&nc, service.clone()).await;
    subscribe_fw_list(&nc, service.clone()).await;
    subscribe_attach(&nc, service).await;
}

async fn reply(nc: &Client, msg: &async_nats::Message, payload: Vec<u8>) {
    if let Some(reply) = &msg.reply {
        let _ = nc.publish(reply.clone(), payload.into()).await;
    }
}

async fn subscribe_create(nc: &Client, service: Arc<NetworkService>) {
    let mut sub = nc.subscribe("network.create").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<CreateNetworkRequest>(&msg.payload) {
                Ok(req) => {
                    let network = service.create_network(req);
                    if let Ok(payload) = serde_json::to_vec(&network) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                Err(e) => error!("network.create parse error: {e}"),
            }
        }
    });
}

async fn subscribe_list(nc: &Client, service: Arc<NetworkService>) {
    let mut sub = nc.subscribe("network.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let (page, per_page) = parse_page(&msg.payload);
            let result = service.list_networks(page, per_page);
            if let Ok(payload) = serde_json::to_vec(&result) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

async fn subscribe_get(nc: &Client, service: Arc<NetworkService>) {
    let mut sub = nc.subscribe("network.get").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match parse_id(&msg.payload, "network_id") {
                Some(id) => {
                    let network = service.get_network(id);
                    if let Ok(payload) = serde_json::to_vec(&network) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                None => error!("network.get missing network_id"),
            }
        }
    });
}

async fn subscribe_delete(nc: &Client, service: Arc<NetworkService>) {
    let mut sub = nc.subscribe("network.delete").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match parse_id(&msg.payload, "network_id") {
                Some(id) => {
                    let ok = service.delete_network(id);
                    let payload = serde_json::json!({ "ok": ok }).to_string().into_bytes();
                    reply(&nc, &msg, payload).await;
                }
                None => error!("network.delete missing network_id"),
            }
        }
    });
}

async fn subscribe_fw_add(nc: &Client, service: Arc<NetworkService>) {
    let mut sub = nc.subscribe("network.firewall.add").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<CreateFirewallRuleRequest>(&msg.payload) {
                Ok(req) => match service.add_firewall_rule(req) {
                    Ok(rule) => {
                        if let Ok(payload) = serde_json::to_vec(&rule) {
                            reply(&nc, &msg, payload).await;
                        }
                    }
                    Err(e) => {
                        let payload = serde_json::json!({ "error": e }).to_string().into_bytes();
                        reply(&nc, &msg, payload).await;
                    }
                },
                Err(e) => error!("network.firewall.add parse error: {e}"),
            }
        }
    });
}

async fn subscribe_fw_list(nc: &Client, service: Arc<NetworkService>) {
    let mut sub = nc.subscribe("network.firewall.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match parse_id(&msg.payload, "network_id") {
                Some(id) => {
                    let rules = service.list_firewall_rules(id);
                    if let Ok(payload) = serde_json::to_vec(&rules) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                None => error!("network.firewall.list missing network_id"),
            }
        }
    });
}

async fn subscribe_attach(nc: &Client, service: Arc<NetworkService>) {
    let mut sub = nc.subscribe("network.attach").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<NetworkAttachment>(&msg.payload) {
                Ok(req) => match service.attach(req) {
                    Ok(att) => {
                        if let Ok(payload) = serde_json::to_vec(&att) {
                            reply(&nc, &msg, payload).await;
                        }
                    }
                    Err(e) => {
                        let payload = serde_json::json!({ "error": e }).to_string().into_bytes();
                        reply(&nc, &msg, payload).await;
                    }
                },
                Err(e) => error!("network.attach parse error: {e}"),
            }
        }
    });
}

fn parse_page(payload: &[u8]) -> (i64, i64) {
    match serde_json::from_slice::<Value>(payload) {
        Ok(v) => (
            v.get("page").and_then(|x| x.as_i64()).unwrap_or(1),
            v.get("per_page").and_then(|x| x.as_i64()).unwrap_or(20),
        ),
        Err(_) => (1, 20),
    }
}

fn parse_id(payload: &[u8], key: &str) -> Option<Uuid> {
    serde_json::from_slice::<Value>(payload)
        .ok()?
        .get(key)?
        .as_str()?
        .parse()
        .ok()
}
