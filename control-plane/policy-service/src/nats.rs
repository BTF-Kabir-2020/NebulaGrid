use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use serde_json::Value;
use tracing::{error, info};
use uuid::Uuid;

use crate::models::*;
use crate::service::PolicyService;

pub async fn start_subscribers(nc: Client, service: Arc<PolicyService>) {
    info!("Starting NATS subscribers for policy-service");
    subscribe_create(&nc, service.clone()).await;
    subscribe_list(&nc, service.clone()).await;
    subscribe_get(&nc, service.clone()).await;
    subscribe_delete(&nc, service.clone()).await;
    subscribe_evaluate(&nc, service).await;
}

async fn reply(nc: &Client, msg: &async_nats::Message, payload: Vec<u8>) {
    if let Some(reply) = &msg.reply {
        let _ = nc.publish(reply.clone(), payload.into()).await;
    }
}

async fn subscribe_create(nc: &Client, service: Arc<PolicyService>) {
    let mut sub = nc.subscribe("policy.create").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<CreatePolicyRequest>(&msg.payload) {
                Ok(req) => {
                    let policy = service.create_policy(req);
                    if let Ok(payload) = serde_json::to_vec(&policy) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                Err(e) => error!("policy.create parse error: {e}"),
            }
        }
    });
}

async fn subscribe_list(nc: &Client, service: Arc<PolicyService>) {
    let mut sub = nc.subscribe("policy.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let (page, per_page) = parse_page(&msg.payload);
            let result = service.list_policies(page, per_page);
            if let Ok(payload) = serde_json::to_vec(&result) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

async fn subscribe_get(nc: &Client, service: Arc<PolicyService>) {
    let mut sub = nc.subscribe("policy.get").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match parse_id(&msg.payload, "policy_id") {
                Some(id) => {
                    let policy = service.get_policy(id);
                    if let Ok(payload) = serde_json::to_vec(&policy) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                None => error!("policy.get missing policy_id"),
            }
        }
    });
}

async fn subscribe_delete(nc: &Client, service: Arc<PolicyService>) {
    let mut sub = nc.subscribe("policy.delete").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match parse_id(&msg.payload, "policy_id") {
                Some(id) => {
                    let ok = service.delete_policy(id);
                    let payload = serde_json::json!({ "ok": ok }).to_string().into_bytes();
                    reply(&nc, &msg, payload).await;
                }
                None => error!("policy.delete missing policy_id"),
            }
        }
    });
}

async fn subscribe_evaluate(nc: &Client, service: Arc<PolicyService>) {
    let mut sub = nc.subscribe("policy.evaluate").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<EvaluateRequest>(&msg.payload) {
                Ok(req) => {
                    let results = service.evaluate(req);
                    if let Ok(payload) = serde_json::to_vec(&results) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                Err(e) => error!("policy.evaluate parse error: {e}"),
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
