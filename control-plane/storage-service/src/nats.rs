use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use serde_json::Value;
use tracing::{error, info};
use uuid::Uuid;

use crate::models::*;
use crate::service::StorageService;

pub async fn start_subscribers(nc: Client, service: Arc<StorageService>) {
    info!("Starting NATS subscribers for storage-service");
    subscribe_pool_create(&nc, service.clone()).await;
    subscribe_pool_list(&nc, service.clone()).await;
    subscribe_volume_create(&nc, service.clone()).await;
    subscribe_volume_list(&nc, service.clone()).await;
    subscribe_snapshot_create(&nc, service).await;
}

async fn reply(nc: &Client, msg: &async_nats::Message, payload: Vec<u8>) {
    if let Some(reply) = &msg.reply {
        let _ = nc.publish(reply.clone(), payload.into()).await;
    }
}

async fn subscribe_pool_create(nc: &Client, service: Arc<StorageService>) {
    let mut sub = nc.subscribe("storage.pool.create").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<CreatePoolRequest>(&msg.payload) {
                Ok(req) => {
                    let pool = service.create_pool(req);
                    if let Ok(payload) = serde_json::to_vec(&pool) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                Err(e) => error!("storage.pool.create parse error: {e}"),
            }
        }
    });
}

async fn subscribe_pool_list(nc: &Client, service: Arc<StorageService>) {
    let mut sub = nc.subscribe("storage.pool.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let (page, per_page) = parse_page(&msg.payload);
            let result = service.list_pools(page, per_page);
            if let Ok(payload) = serde_json::to_vec(&result) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

async fn subscribe_volume_create(nc: &Client, service: Arc<StorageService>) {
    let mut sub = nc.subscribe("storage.volume.create").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<CreateVolumeRequest>(&msg.payload) {
                Ok(req) => match service.create_volume(req) {
                    Ok(volume) => {
                        if let Ok(payload) = serde_json::to_vec(&volume) {
                            reply(&nc, &msg, payload).await;
                        }
                    }
                    Err(e) => {
                        let payload = serde_json::json!({ "error": e }).to_string().into_bytes();
                        reply(&nc, &msg, payload).await;
                    }
                },
                Err(e) => error!("storage.volume.create parse error: {e}"),
            }
        }
    });
}

async fn subscribe_volume_list(nc: &Client, service: Arc<StorageService>) {
    let mut sub = nc.subscribe("storage.volume.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let (page, per_page) = parse_page(&msg.payload);
            let result = service.list_volumes(page, per_page);
            if let Ok(payload) = serde_json::to_vec(&result) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

async fn subscribe_snapshot_create(nc: &Client, service: Arc<StorageService>) {
    let mut sub = nc.subscribe("storage.snapshot.create").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let parsed = serde_json::from_slice::<Value>(&msg.payload).ok();
            let volume_id = parsed
                .as_ref()
                .and_then(|v| v.get("volume_id"))
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<Uuid>().ok());
            let name = parsed
                .as_ref()
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("snapshot")
                .to_string();
            match volume_id {
                Some(id) => match service.create_snapshot(id, name) {
                    Ok(snap) => {
                        if let Ok(payload) = serde_json::to_vec(&snap) {
                            reply(&nc, &msg, payload).await;
                        }
                    }
                    Err(e) => {
                        let payload = serde_json::json!({ "error": e }).to_string().into_bytes();
                        reply(&nc, &msg, payload).await;
                    }
                },
                None => error!("storage.snapshot.create missing volume_id"),
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
