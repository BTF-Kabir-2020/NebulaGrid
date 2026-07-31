use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use serde_json::Value;
use tracing::{error, info};
use uuid::Uuid;

use crate::models::*;
use crate::service::InventoryService;

pub async fn start_subscribers(nc: Client, service: Arc<InventoryService>) {
    info!("Starting NATS subscribers for inventory-service");
    subscribe_register(&nc, service.clone()).await;
    subscribe_list(&nc, service.clone()).await;
    subscribe_get(&nc, service.clone()).await;
    subscribe_delete(&nc, service.clone()).await;
    subscribe_summary(&nc, service).await;
}

async fn reply(nc: &Client, msg: &async_nats::Message, payload: Vec<u8>) {
    if let Some(reply) = &msg.reply {
        let _ = nc.publish(reply.clone(), payload.into()).await;
    }
}

async fn subscribe_register(nc: &Client, service: Arc<InventoryService>) {
    let mut sub = nc.subscribe("inventory.register").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<RegisterItemRequest>(&msg.payload) {
                Ok(req) => {
                    let item = service.register_item(req);
                    if let Ok(payload) = serde_json::to_vec(&item) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                Err(e) => error!("inventory.register parse error: {e}"),
            }
        }
    });
}

async fn subscribe_list(nc: &Client, service: Arc<InventoryService>) {
    let mut sub = nc.subscribe("inventory.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let query = serde_json::from_slice::<ListInventoryQuery>(&msg.payload).unwrap_or(
                ListInventoryQuery {
                    item_type: None,
                    status: None,
                    location: None,
                    search: None,
                    page: None,
                    per_page: None,
                },
            );
            let result = service.list_items(query);
            if let Ok(payload) = serde_json::to_vec(&result) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

async fn subscribe_get(nc: &Client, service: Arc<InventoryService>) {
    let mut sub = nc.subscribe("inventory.get").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match parse_id(&msg.payload, "item_id") {
                Some(id) => {
                    let item = service.get_item(id);
                    if let Ok(payload) = serde_json::to_vec(&item) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                None => error!("inventory.get missing item_id"),
            }
        }
    });
}

async fn subscribe_delete(nc: &Client, service: Arc<InventoryService>) {
    let mut sub = nc.subscribe("inventory.delete").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match parse_id(&msg.payload, "item_id") {
                Some(id) => {
                    let ok = service.delete_item(id);
                    let payload = serde_json::json!({ "ok": ok }).to_string().into_bytes();
                    reply(&nc, &msg, payload).await;
                }
                None => error!("inventory.delete missing item_id"),
            }
        }
    });
}

async fn subscribe_summary(nc: &Client, service: Arc<InventoryService>) {
    let mut sub = nc.subscribe("inventory.summary").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let summary = service.summary();
            if let Ok(payload) = serde_json::to_vec(&summary) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

fn parse_id(payload: &[u8], key: &str) -> Option<Uuid> {
    serde_json::from_slice::<Value>(payload)
        .ok()?
        .get(key)?
        .as_str()?
        .parse()
        .ok()
}
