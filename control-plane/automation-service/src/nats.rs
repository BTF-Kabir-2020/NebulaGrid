use async_nats::Client;
use async_nats::Subject;
use futures::stream::StreamExt;
use std::sync::Arc;
use tracing::{error, info};

use crate::models::*;
use crate::service::JobService;

pub async fn connect(url: &str) -> Result<Client, Box<dyn std::error::Error>> {
    let client = async_nats::connect(url).await?;
    Ok(client)
}

async fn publish_reply(nc: &Client, reply: Option<Subject>, data: Vec<u8>) {
    if let Some(subject) = reply {
        if let Err(e) = nc.publish(subject, data.into()).await {
            error!("failed to publish NATS reply: {e}");
        }
    }
}

pub async fn subscribe_all(
    nc: Client,
    svc: Arc<JobService>,
) -> Result<(), Box<dyn std::error::Error>> {
    let sub = nc
        .queue_subscribe(
            String::from("automation.job.create"),
            String::from("automation-workers"),
        )
        .await?;
    let nc_clone = nc.clone();
    let svc_clone = svc.clone();
    tokio::spawn(async move {
        let mut incoming = sub;
        while let Some(msg) = incoming.next().await {
            let payload: CreateJobRequest = match serde_json::from_slice(&msg.payload) {
                Ok(v) => v,
                Err(e) => {
                    error!("failed to deserialize create job request: {e}");
                    continue;
                }
            };
            let job = svc_clone.create_job(payload).await;
            publish_reply(&nc_clone, msg.reply, serde_json::to_vec(&job).unwrap()).await;
        }
    });

    let sub = nc
        .queue_subscribe(
            String::from("automation.job.list"),
            String::from("automation-workers"),
        )
        .await?;
    let nc_clone = nc.clone();
    let svc_clone = svc.clone();
    tokio::spawn(async move {
        let mut incoming = sub;
        while let Some(msg) = incoming.next().await {
            let query: JobQuery = serde_json::from_slice(&msg.payload).unwrap_or(JobQuery {
                status: None,
                page: None,
                per_page: None,
            });
            let result = svc_clone.list_jobs(query).await;
            publish_reply(&nc_clone, msg.reply, serde_json::to_vec(&result).unwrap()).await;
        }
    });

    let sub = nc
        .queue_subscribe(
            String::from("automation.job.get"),
            String::from("automation-workers"),
        )
        .await?;
    let nc_clone = nc.clone();
    let svc_clone = svc.clone();
    tokio::spawn(async move {
        let mut incoming = sub;
        while let Some(msg) = incoming.next().await {
            let id: uuid::Uuid = match serde_json::from_slice(&msg.payload) {
                Ok(v) => v,
                Err(e) => {
                    error!("failed to deserialize job id: {e}");
                    continue;
                }
            };
            let job = svc_clone.get_job(id).await;
            publish_reply(&nc_clone, msg.reply, serde_json::to_vec(&job).unwrap()).await;
        }
    });

    let sub = nc
        .queue_subscribe(
            String::from("automation.job.logs"),
            String::from("automation-workers"),
        )
        .await?;
    let nc_clone = nc.clone();
    let svc_clone = svc.clone();
    tokio::spawn(async move {
        let mut incoming = sub;
        while let Some(msg) = incoming.next().await {
            let id: uuid::Uuid = match serde_json::from_slice(&msg.payload) {
                Ok(v) => v,
                Err(e) => {
                    error!("failed to deserialize job id: {e}");
                    continue;
                }
            };
            let logs = svc_clone.get_job_logs(id).await;
            publish_reply(&nc_clone, msg.reply, serde_json::to_vec(&logs).unwrap()).await;
        }
    });

    info!("NATS subscriptions registered for automation service");
    Ok(())
}
