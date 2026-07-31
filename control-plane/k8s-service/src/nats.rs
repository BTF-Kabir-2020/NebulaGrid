use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use serde_json::Value;

use crate::models::K8sQuery;
use crate::service::K8sService;

pub async fn start_subscribers(nc: Client, service: Arc<K8sService>) {
    subscribe_pods(&nc, service.clone()).await;
    subscribe_deployments(&nc, service.clone()).await;
    subscribe_services(&nc, service.clone()).await;
    subscribe_nodes(&nc, service.clone()).await;
}

async fn subscribe_pods(nc: &Client, service: Arc<K8sService>) {
    let mut sub = nc.subscribe("k8s.pods").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let query: K8sQuery = match serde_json::from_slice(&msg.payload) {
                Ok(q) => q,
                Err(_) => K8sQuery { namespace: None },
            };
            let pods = service.list_pods(query.namespace.as_deref());
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&pods) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_deployments(nc: &Client, service: Arc<K8sService>) {
    let mut sub = nc.subscribe("k8s.deployments").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let query: K8sQuery = match serde_json::from_slice(&msg.payload) {
                Ok(q) => q,
                Err(_) => K8sQuery { namespace: None },
            };
            let deployments = service.list_deployments(query.namespace.as_deref());
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&deployments) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_services(nc: &Client, service: Arc<K8sService>) {
    let mut sub = nc.subscribe("k8s.services").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let query: K8sQuery = match serde_json::from_slice(&msg.payload) {
                Ok(q) => q,
                Err(_) => K8sQuery { namespace: None },
            };
            let services = service.list_services(query.namespace.as_deref());
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&services) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_nodes(nc: &Client, service: Arc<K8sService>) {
    let mut sub = nc.subscribe("k8s.nodes").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let _query: Option<Value> = serde_json::from_slice(&msg.payload).ok();
            let nodes = service.list_nodes();
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&nodes) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}
