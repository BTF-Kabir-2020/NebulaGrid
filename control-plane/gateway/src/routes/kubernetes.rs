use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use std::sync::Arc;

use crate::state::AppState;

#[derive(Serialize)]
pub struct K8sNodeResponse {
    pub name: String,
    pub status: String,
    pub version: String,
    pub pod_count: u32,
}

#[derive(Serialize)]
pub struct PodResponse {
    pub name: String,
    pub namespace: String,
    pub status: String,
    pub node: String,
    pub pod_ip: String,
}

#[derive(Serialize)]
pub struct DeploymentResponse {
    pub name: String,
    pub namespace: String,
    pub replicas: u32,
    pub available_replicas: u32,
}

#[derive(Serialize)]
pub struct ServiceResponse {
    pub name: String,
    pub namespace: String,
    pub cluster_ip: String,
    pub ports: Vec<String>,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize)]
pub struct NamespaceResponse {
    pub name: String,
    pub status: String,
}

pub async fn nodes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<K8sNodeResponse>>, StatusCode> {
    let k8s_nodes = state.k8s_nodes.lock().await;
    Ok(Json(
        k8s_nodes
            .iter()
            .map(|n| K8sNodeResponse {
                name: n.name.clone(),
                status: n.status.clone(),
                version: n.version.clone(),
                pod_count: n.pod_count,
            })
            .collect(),
    ))
}

pub async fn pods(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PodResponse>>, StatusCode> {
    let pods = state.k8s_pods.lock().await;
    Ok(Json(
        pods.iter()
            .map(|p| PodResponse {
                name: p.name.clone(),
                namespace: p.namespace.clone(),
                status: p.status.clone(),
                node: p.node.clone(),
                pod_ip: p.pod_ip.clone(),
            })
            .collect(),
    ))
}

pub async fn deployments(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<DeploymentResponse>>, StatusCode> {
    let deployments = state.k8s_deployments.lock().await;
    Ok(Json(
        deployments
            .iter()
            .map(|d| DeploymentResponse {
                name: d.name.clone(),
                namespace: d.namespace.clone(),
                replicas: d.replicas,
                available_replicas: d.available_replicas,
            })
            .collect(),
    ))
}

pub async fn services(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ServiceResponse>>, StatusCode> {
    let services = state.k8s_services.lock().await;
    Ok(Json(
        services
            .iter()
            .map(|s| ServiceResponse {
                name: s.name.clone(),
                namespace: s.namespace.clone(),
                cluster_ip: s.cluster_ip.clone(),
                ports: s.ports.clone(),
                type_: s.type_.clone(),
            })
            .collect(),
    ))
}

pub async fn namespaces(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<NamespaceResponse>>, StatusCode> {
    let pods = state.k8s_pods.lock().await;
    let mut ns_set: Vec<String> = Vec::new();
    for p in pods.iter() {
        if !ns_set.contains(&p.namespace) {
            ns_set.push(p.namespace.clone());
        }
    }
    let deployments = state.k8s_deployments.lock().await;
    for d in deployments.iter() {
        if !ns_set.contains(&d.namespace) {
            ns_set.push(d.namespace.clone());
        }
    }
    let services = state.k8s_services.lock().await;
    for s in services.iter() {
        if !ns_set.contains(&s.namespace) {
            ns_set.push(s.namespace.clone());
        }
    }

    Ok(Json(
        ns_set
            .into_iter()
            .map(|name| NamespaceResponse {
                name,
                status: "Active".into(),
            })
            .collect(),
    ))
}
