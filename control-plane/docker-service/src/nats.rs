use std::sync::Arc;

use async_nats::Client;
use futures::stream::StreamExt;
use serde::Deserialize;
use tracing::error;

use crate::models::*;
use crate::service::DockerService;

#[derive(Debug, Deserialize)]
struct ContainerLogsRequest {
    id: String,
    #[serde(default = "default_tail")]
    tail: usize,
}

fn default_tail() -> usize {
    100
}

async fn nats_handler(
    nc: &Client,
    svc: &DockerService,
    subject: &str,
    reply: &str,
    payload: &[u8],
) {
    let respond = |data: Vec<u8>| async move {
        let _ = nc.publish(reply.to_string(), data.into()).await;
    };

    match subject {
        "docker.list" => {
            let query: ListContainersQuery =
                serde_json::from_slice(payload).unwrap_or(ListContainersQuery {
                    status: None,
                    node_id: None,
                });
            let all = query.status.as_deref() == Some("all");
            respond(serde_json::to_vec(&svc.list_containers(all).await).unwrap()).await;
        }
        "docker.get" => {
            let action: ContainerAction = match serde_json::from_slice(payload) {
                Ok(a) => a,
                Err(e) => {
                    respond(
                        serde_json::to_vec(&ActionResult {
                            success: false,
                            message: format!("Invalid payload: {e}"),
                            container_id: String::new(),
                        })
                        .unwrap(),
                    )
                    .await;
                    return;
                }
            };
            respond(serde_json::to_vec(&svc.get_container(&action.id).await).unwrap()).await;
        }
        "docker.start" => {
            let action: ContainerAction = match serde_json::from_slice(payload) {
                Ok(a) => a,
                Err(e) => {
                    respond(
                        serde_json::to_vec(&ActionResult {
                            success: false,
                            message: format!("Invalid payload: {e}"),
                            container_id: String::new(),
                        })
                        .unwrap(),
                    )
                    .await;
                    return;
                }
            };
            respond(serde_json::to_vec(&svc.start_container(&action.id).await).unwrap()).await;
        }
        "docker.stop" => {
            let action: ContainerAction = match serde_json::from_slice(payload) {
                Ok(a) => a,
                Err(e) => {
                    respond(
                        serde_json::to_vec(&ActionResult {
                            success: false,
                            message: format!("Invalid payload: {e}"),
                            container_id: String::new(),
                        })
                        .unwrap(),
                    )
                    .await;
                    return;
                }
            };
            respond(serde_json::to_vec(&svc.stop_container(&action.id).await).unwrap()).await;
        }
        "docker.restart" => {
            let action: ContainerAction = match serde_json::from_slice(payload) {
                Ok(a) => a,
                Err(e) => {
                    respond(
                        serde_json::to_vec(&ActionResult {
                            success: false,
                            message: format!("Invalid payload: {e}"),
                            container_id: String::new(),
                        })
                        .unwrap(),
                    )
                    .await;
                    return;
                }
            };
            respond(serde_json::to_vec(&svc.restart_container(&action.id).await).unwrap()).await;
        }
        "docker.logs" => {
            let req: ContainerLogsRequest = match serde_json::from_slice(payload) {
                Ok(r) => r,
                Err(e) => {
                    respond(
                        serde_json::to_vec(&ContainerLogs {
                            id: String::new(),
                            logs: format!("Invalid payload: {e}"),
                        })
                        .unwrap(),
                    )
                    .await;
                    return;
                }
            };
            respond(serde_json::to_vec(&svc.get_container_logs(&req.id, req.tail).await).unwrap())
                .await;
        }
        "docker.delete" => {
            let action: ContainerAction = match serde_json::from_slice(payload) {
                Ok(a) => a,
                Err(e) => {
                    respond(
                        serde_json::to_vec(&ActionResult {
                            success: false,
                            message: format!("Invalid payload: {e}"),
                            container_id: String::new(),
                        })
                        .unwrap(),
                    )
                    .await;
                    return;
                }
            };
            respond(serde_json::to_vec(&svc.delete_container(&action.id).await).unwrap()).await;
        }
        _ => {}
    }
}

pub async fn start_nats_handlers(nc: Client, svc: Arc<DockerService>) {
    let subjects = [
        "docker.list",
        "docker.get",
        "docker.start",
        "docker.stop",
        "docker.restart",
        "docker.logs",
        "docker.delete",
    ];

    for subject in subjects {
        let nc = nc.clone();
        let svc = svc.clone();
        tokio::spawn(async move {
            let mut sub = match nc.subscribe(subject.to_string()).await {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to subscribe to {subject}: {e}");
                    return;
                }
            };

            while let Some(msg) = sub.next().await {
                let reply = match &msg.reply {
                    Some(r) => r.clone(),
                    None => continue,
                };
                nats_handler(&nc, &svc, subject, &reply, &msg.payload).await;
            }
        });
    }
}
