use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use serde_json::Value;
use tracing::error;
use uuid::Uuid;

use crate::models::*;
use crate::service::VmService;

pub async fn start_subscribers(nc: Client, service: Arc<VmService>) {
    subscribe_vm_list(&nc, service.clone()).await;
    subscribe_vm_get(&nc, service.clone()).await;
    subscribe_vm_create(&nc, service.clone()).await;
    subscribe_vm_start(&nc, service.clone()).await;
    subscribe_vm_stop(&nc, service.clone()).await;
    subscribe_vm_restart(&nc, service.clone()).await;
    subscribe_vm_snapshot(&nc, service.clone()).await;
    subscribe_vm_snapshots(&nc, service.clone()).await;
}

async fn subscribe_vm_list(nc: &Client, service: Arc<VmService>) {
    let mut sub = nc.subscribe("vm.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let filter: Option<String> =
                serde_json::from_slice::<Value>(&msg.payload)
                    .ok()
                    .and_then(|v| v.get("status").and_then(|s| s.as_str().map(String::from)));
            let vms = service.list_vms(filter);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&vms) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_vm_get(nc: &Client, service: Arc<VmService>) {
    let mut sub = nc.subscribe("vm.get").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let id: Uuid = match serde_json::from_slice::<Value>(&msg.payload) {
                Ok(v) => match v.get("id").and_then(|v| v.as_str()) {
                    Some(s) => match s.parse() {
                        Ok(id) => id,
                        Err(_) => {
                            error!("Failed to parse id for vm.get");
                            continue;
                        }
                    },
                    None => {
                        error!("Missing id in vm.get");
                        continue;
                    }
                },
                Err(e) => {
                    error!("Failed to parse vm.get request: {e}");
                    continue;
                }
            };
            let vm = service.get_vm(id);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&vm) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_vm_create(nc: &Client, service: Arc<VmService>) {
    let mut sub = nc.subscribe("vm.create").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let req: CreateVmRequest = match serde_json::from_slice(&msg.payload) {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse create request: {e}");
                    continue;
                }
            };
            let vm = service.create_vm(req);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&vm) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_vm_start(nc: &Client, service: Arc<VmService>) {
    let mut sub = nc.subscribe("vm.start").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let action: VmAction = match serde_json::from_slice(&msg.payload) {
                Ok(a) => a,
                Err(e) => {
                    error!("Failed to parse vm.start request: {e}");
                    continue;
                }
            };
            let result = service.start_vm(action.id);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&result) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_vm_stop(nc: &Client, service: Arc<VmService>) {
    let mut sub = nc.subscribe("vm.stop").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let action: VmAction = match serde_json::from_slice(&msg.payload) {
                Ok(a) => a,
                Err(e) => {
                    error!("Failed to parse vm.stop request: {e}");
                    continue;
                }
            };
            let result = service.stop_vm(action.id);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&result) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_vm_restart(nc: &Client, service: Arc<VmService>) {
    let mut sub = nc.subscribe("vm.restart").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let action: VmAction = match serde_json::from_slice(&msg.payload) {
                Ok(a) => a,
                Err(e) => {
                    error!("Failed to parse vm.restart request: {e}");
                    continue;
                }
            };
            let result = service.restart_vm(action.id);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&result) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_vm_snapshot(nc: &Client, service: Arc<VmService>) {
    let mut sub = nc.subscribe("vm.snapshot").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let (vm_id, name): (Uuid, String) = match serde_json::from_slice::<Value>(&msg.payload)
            {
                Ok(v) => {
                    let id = v
                        .get("id")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse().ok());
                    let name = v
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(String::from)
                        .unwrap_or_default();
                    match id {
                        Some(id) => (id, name),
                        None => {
                            error!("Missing id in vm.snapshot");
                            continue;
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to parse vm.snapshot request: {e}");
                    continue;
                }
            };
            let result = service.create_snapshot(vm_id, name);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&result) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}

async fn subscribe_vm_snapshots(nc: &Client, service: Arc<VmService>) {
    let mut sub = nc.subscribe("vm.snapshots").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let vm_id: Uuid = match serde_json::from_slice::<Value>(&msg.payload) {
                Ok(v) => match v.get("id").and_then(|v| v.as_str()) {
                    Some(s) => match s.parse() {
                        Ok(id) => id,
                        Err(_) => {
                            error!("Failed to parse id for vm.snapshots");
                            continue;
                        }
                    },
                    None => {
                        error!("Missing id in vm.snapshots");
                        continue;
                    }
                },
                Err(e) => {
                    error!("Failed to parse vm.snapshots request: {e}");
                    continue;
                }
            };
            let snapshots = service.list_snapshots(vm_id);
            if let Some(reply) = msg.reply {
                if let Ok(payload) = serde_json::to_vec(&snapshots) {
                    let _ = nc.publish(reply, payload.into()).await;
                }
            }
        }
    });
}
