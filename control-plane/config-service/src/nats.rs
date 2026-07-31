use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use serde_json::Value;
use tracing::{error, info};

use crate::models::*;
use crate::service::ConfigService;

pub async fn start_subscribers(nc: Client, service: Arc<ConfigService>) {
    info!("Starting NATS subscribers for config-service");
    subscribe_set(&nc, service.clone()).await;
    subscribe_get(&nc, service.clone()).await;
    subscribe_list(&nc, service.clone()).await;
    subscribe_groups(&nc, service.clone()).await;
    subscribe_delete(&nc, service).await;
}

async fn reply(nc: &Client, msg: &async_nats::Message, payload: Vec<u8>) {
    if let Some(reply) = &msg.reply {
        let _ = nc.publish(reply.clone(), payload.into()).await;
    }
}

async fn subscribe_set(nc: &Client, service: Arc<ConfigService>) {
    let mut sub = nc.subscribe("config.set").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<SetConfigRequest>(&msg.payload) {
                Ok(req) => {
                    let entry = service.set_config(req);
                    if let Ok(payload) = serde_json::to_vec(&entry) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                Err(e) => error!("config.set parse error: {e}"),
            }
        }
    });
}

async fn subscribe_get(nc: &Client, service: Arc<ConfigService>) {
    let mut sub = nc.subscribe("config.get").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let key = serde_json::from_slice::<Value>(&msg.payload)
                .ok()
                .and_then(|v| v.get("key")?.as_str().map(|s| s.to_string()));
            match key {
                Some(key) => {
                    let entry = service.get_config(&key);
                    if let Ok(payload) = serde_json::to_vec(&entry) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                None => error!("config.get missing key"),
            }
        }
    });
}

async fn subscribe_list(nc: &Client, service: Arc<ConfigService>) {
    let mut sub = nc.subscribe("config.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let query = serde_json::from_slice::<ListConfigQuery>(&msg.payload).unwrap_or(
                ListConfigQuery {
                    group: None,
                    search: None,
                    page: None,
                    per_page: None,
                },
            );
            let result = service.list_config(query);
            if let Ok(payload) = serde_json::to_vec(&result) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

async fn subscribe_groups(nc: &Client, service: Arc<ConfigService>) {
    let mut sub = nc.subscribe("config.groups").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let groups = service.list_groups();
            if let Ok(payload) = serde_json::to_vec(&groups) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

async fn subscribe_delete(nc: &Client, service: Arc<ConfigService>) {
    let mut sub = nc.subscribe("config.delete").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let key = serde_json::from_slice::<Value>(&msg.payload)
                .ok()
                .and_then(|v| v.get("key")?.as_str().map(|s| s.to_string()));
            match key {
                Some(key) => {
                    let ok = service.delete_config(&key);
                    let payload = serde_json::json!({ "ok": ok }).to_string().into_bytes();
                    reply(&nc, &msg, payload).await;
                }
                None => error!("config.delete missing key"),
            }
        }
    });
}
