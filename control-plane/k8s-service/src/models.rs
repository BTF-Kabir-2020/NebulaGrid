use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Clone)]
pub struct PodSummary {
    pub name: String,
    pub namespace: String,
    pub status: String,
    pub node: String,
    pub ip: String,
    pub created_at: DateTime<Utc>,
    pub containers: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DeploymentSummary {
    pub name: String,
    pub namespace: String,
    pub replicas: i32,
    pub available: i32,
    pub image: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ServiceSummary {
    pub name: String,
    pub namespace: String,
    pub cluster_ip: String,
    pub ports: Vec<String>,
    pub type_: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct K8sNodeSummary {
    pub name: String,
    pub status: String,
    pub version: String,
    pub cpu_capacity: String,
    pub memory_capacity: String,
    pub pod_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct K8sQuery {
    pub namespace: Option<String>,
}
