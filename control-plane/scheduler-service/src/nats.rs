use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use serde_json::Value;
use tracing::{error, info};
use uuid::Uuid;

use crate::models::*;
use crate::service::SchedulerService;

pub async fn start_subscribers(nc: Client, service: Arc<SchedulerService>) {
    info!("Starting NATS subscribers for scheduler-service");
    subscribe_create_job(&nc, service.clone()).await;
    subscribe_list_jobs(&nc, service.clone()).await;
    subscribe_get_job(&nc, service.clone()).await;
    subscribe_create_schedule(&nc, service.clone()).await;
    subscribe_list_schedules(&nc, service.clone()).await;
    subscribe_set_enabled(&nc, service.clone()).await;
    subscribe_run_due(&nc, service).await;
}

async fn reply(nc: &Client, msg: &async_nats::Message, payload: Vec<u8>) {
    if let Some(reply) = &msg.reply {
        let _ = nc.publish(reply.clone(), payload.into()).await;
    }
}

async fn subscribe_create_job(nc: &Client, service: Arc<SchedulerService>) {
    let mut sub = nc.subscribe("scheduler.job.create").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<CreateJobRequest>(&msg.payload) {
                Ok(req) => {
                    let job = service.create_job(req);
                    if let Ok(payload) = serde_json::to_vec(&job) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                Err(e) => error!("scheduler.job.create parse error: {e}"),
            }
        }
    });
}

async fn subscribe_list_jobs(nc: &Client, service: Arc<SchedulerService>) {
    let mut sub = nc.subscribe("scheduler.job.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let (page, per_page) = parse_page(&msg.payload);
            let result = service.list_jobs(page, per_page);
            if let Ok(payload) = serde_json::to_vec(&result) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

async fn subscribe_get_job(nc: &Client, service: Arc<SchedulerService>) {
    let mut sub = nc.subscribe("scheduler.job.get").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let id = parse_id(&msg.payload, "job_id");
            match id {
                Some(id) => {
                    let job = service.get_job(id);
                    if let Ok(payload) = serde_json::to_vec(&job) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                None => error!("scheduler.job.get missing job_id"),
            }
        }
    });
}

async fn subscribe_create_schedule(nc: &Client, service: Arc<SchedulerService>) {
    let mut sub = nc.subscribe("scheduler.schedule.create").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<CreateScheduleRequest>(&msg.payload) {
                Ok(req) => {
                    let schedule = service.create_schedule(req);
                    if let Ok(payload) = serde_json::to_vec(&schedule) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                Err(e) => error!("scheduler.schedule.create parse error: {e}"),
            }
        }
    });
}

async fn subscribe_list_schedules(nc: &Client, service: Arc<SchedulerService>) {
    let mut sub = nc.subscribe("scheduler.schedule.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let (page, per_page) = parse_page(&msg.payload);
            let result = service.list_schedules(page, per_page);
            if let Ok(payload) = serde_json::to_vec(&result) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

async fn subscribe_set_enabled(nc: &Client, service: Arc<SchedulerService>) {
    let mut sub = nc
        .subscribe("scheduler.schedule.set_enabled")
        .await
        .unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let parsed = serde_json::from_slice::<Value>(&msg.payload).ok();
            let id = parsed
                .as_ref()
                .and_then(|v| v.get("schedule_id"))
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<Uuid>().ok());
            let enabled = parsed
                .as_ref()
                .and_then(|v| v.get("enabled"))
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            match id {
                Some(id) => {
                    let schedule = service.set_schedule_enabled(id, enabled);
                    if let Ok(payload) = serde_json::to_vec(&schedule) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                None => error!("scheduler.schedule.set_enabled missing schedule_id"),
            }
        }
    });
}

async fn subscribe_run_due(nc: &Client, service: Arc<SchedulerService>) {
    let mut sub = nc.subscribe("scheduler.run_due").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let jobs = service.run_due_schedules();
            if let Ok(payload) = serde_json::to_vec(&jobs) {
                reply(&nc, &msg, payload).await;
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
