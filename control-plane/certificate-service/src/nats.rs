use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use serde_json::Value;
use tracing::{error, info};
use uuid::Uuid;

use crate::models::*;
use crate::service::CertificateService;

pub async fn start_subscribers(nc: Client, service: Arc<CertificateService>) {
    info!("Starting NATS subscribers for certificate-service");
    subscribe_create(&nc, service.clone()).await;
    subscribe_list(&nc, service.clone()).await;
    subscribe_authorities(&nc, service.clone()).await;
    subscribe_renew(&nc, service.clone()).await;
    subscribe_revoke(&nc, service).await;
}

async fn reply(nc: &Client, msg: &async_nats::Message, payload: Vec<u8>) {
    if let Some(reply) = &msg.reply {
        let _ = nc.publish(reply.clone(), payload.into()).await;
    }
}

async fn subscribe_create(nc: &Client, service: Arc<CertificateService>) {
    let mut sub = nc.subscribe("certificate.create").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<CreateCertificateRequest>(&msg.payload) {
                Ok(req) => {
                    let cert = service.create_certificate(req);
                    if let Ok(payload) = serde_json::to_vec(&cert) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                Err(e) => error!("certificate.create parse error: {e}"),
            }
        }
    });
}

async fn subscribe_list(nc: &Client, service: Arc<CertificateService>) {
    let mut sub = nc.subscribe("certificate.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let (page, per_page) = parse_page(&msg.payload);
            let result = service.list_certificates(page, per_page);
            if let Ok(payload) = serde_json::to_vec(&result) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

async fn subscribe_authorities(nc: &Client, service: Arc<CertificateService>) {
    let mut sub = nc.subscribe("certificate.ca.list").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            let cas = service.list_authorities();
            if let Ok(payload) = serde_json::to_vec(&cas) {
                reply(&nc, &msg, payload).await;
            }
        }
    });
}

async fn subscribe_renew(nc: &Client, service: Arc<CertificateService>) {
    let mut sub = nc.subscribe("certificate.renew").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match serde_json::from_slice::<RenewCertificateRequest>(&msg.payload) {
                Ok(req) => match service.renew(req) {
                    Ok(cert) => {
                        if let Ok(payload) = serde_json::to_vec(&cert) {
                            reply(&nc, &msg, payload).await;
                        }
                    }
                    Err(e) => {
                        let payload = serde_json::json!({ "error": e }).to_string().into_bytes();
                        reply(&nc, &msg, payload).await;
                    }
                },
                Err(e) => error!("certificate.renew parse error: {e}"),
            }
        }
    });
}

async fn subscribe_revoke(nc: &Client, service: Arc<CertificateService>) {
    let mut sub = nc.subscribe("certificate.revoke").await.unwrap();
    let nc = nc.clone();
    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            match parse_id(&msg.payload, "cert_id") {
                Some(id) => {
                    let cert = service.revoke(id);
                    if let Ok(payload) = serde_json::to_vec(&cert) {
                        reply(&nc, &msg, payload).await;
                    }
                }
                None => error!("certificate.revoke missing cert_id"),
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
