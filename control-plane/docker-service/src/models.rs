use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct ContainerSummary {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub state: String,
    pub created: DateTime<Utc>,
    pub ports: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListContainersQuery {
    pub status: Option<String>,
    pub node_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContainerAction {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct ContainerLogs {
    pub id: String,
    pub logs: String,
}

#[derive(Debug, Serialize)]
pub struct ActionResult {
    pub success: bool,
    pub message: String,
    pub container_id: String,
}
