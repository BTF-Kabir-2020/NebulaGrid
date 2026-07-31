use std::sync::Arc;

use bollard::container::{ListContainersOptions, LogsOptions, RemoveContainerOptions};
use bollard::Docker;
use chrono::DateTime;
use futures::stream::StreamExt;

use crate::models::*;

#[derive(Clone)]
pub struct DockerService {
    docker: Option<Arc<Docker>>,
}

impl DockerService {
    pub fn new(docker: Option<Arc<Docker>>) -> Self {
        Self { docker }
    }

    pub async fn list_containers(&self, all: bool) -> Vec<ContainerSummary> {
        let docker = match &self.docker {
            Some(d) => d,
            None => return Vec::new(),
        };

        let options = ListContainersOptions::<String> {
            all,
            ..Default::default()
        };

        match docker.list_containers(Some(options)).await {
            Ok(containers) => containers
                .into_iter()
                .map(|c| {
                    let ports = c
                        .ports
                        .unwrap_or_default()
                        .iter()
                        .map(|p| {
                            format!(
                                "{}:{}->{}/{}",
                                p.ip.as_deref().unwrap_or("0.0.0.0"),
                                p.public_port
                                    .map(|v| v.to_string())
                                    .unwrap_or_else(|| "?".to_string()),
                                p.private_port,
                                p.typ.as_ref().map(AsRef::as_ref).unwrap_or("tcp"),
                            )
                        })
                        .collect();

                    ContainerSummary {
                        id: c.id.unwrap_or_default(),
                        name: c
                            .names
                            .unwrap_or_default()
                            .first()
                            .cloned()
                            .unwrap_or_default()
                            .trim_start_matches('/')
                            .to_string(),
                        image: c.image.unwrap_or_default(),
                        status: c.status.unwrap_or_default(),
                        state: c.state.unwrap_or_default(),
                        created: c
                            .created
                            .and_then(|ts| DateTime::from_timestamp(ts, 0))
                            .unwrap_or_default(),
                        ports,
                    }
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    pub async fn get_container(&self, id: &str) -> Option<ContainerSummary> {
        let docker = match &self.docker {
            Some(d) => d,
            None => return None,
        };

        let options = ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        };

        let containers = docker.list_containers(Some(options)).await.ok()?;
        let container = containers.into_iter().find(|c| {
            let cid = c.id.as_deref().unwrap_or("");
            let name = c
                .names
                .as_ref()
                .and_then(|n| n.first())
                .map(|n| n.trim_start_matches('/'))
                .unwrap_or("");
            cid == id || cid.starts_with(id) || name == id
        })?;

        let ports = container
            .ports
            .unwrap_or_default()
            .iter()
            .map(|p| {
                format!(
                    "{}:{}->{}/{}",
                    p.ip.as_deref().unwrap_or("0.0.0.0"),
                    p.public_port
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "?".to_string()),
                    p.private_port,
                    p.typ.as_ref().map(AsRef::as_ref).unwrap_or("tcp"),
                )
            })
            .collect();

        Some(ContainerSummary {
            id: container.id.unwrap_or_default(),
            name: container
                .names
                .unwrap_or_default()
                .first()
                .cloned()
                .unwrap_or_default()
                .trim_start_matches('/')
                .to_string(),
            image: container.image.unwrap_or_default(),
            status: container.status.unwrap_or_default(),
            state: container.state.unwrap_or_default(),
            created: container
                .created
                .and_then(|ts| DateTime::from_timestamp(ts, 0))
                .unwrap_or_default(),
            ports,
        })
    }

    pub async fn start_container(&self, id: &str) -> ActionResult {
        let docker = match &self.docker {
            Some(d) => d,
            None => {
                return ActionResult {
                    success: false,
                    message: "Docker is not available".to_string(),
                    container_id: id.to_string(),
                }
            }
        };

        match docker
            .start_container::<String>(
                id,
                None::<bollard::container::StartContainerOptions<String>>,
            )
            .await
        {
            Ok(_) => ActionResult {
                success: true,
                message: "Container started successfully".to_string(),
                container_id: id.to_string(),
            },
            Err(e) => ActionResult {
                success: false,
                message: format!("Failed to start container: {e}"),
                container_id: id.to_string(),
            },
        }
    }

    pub async fn stop_container(&self, id: &str) -> ActionResult {
        let docker = match &self.docker {
            Some(d) => d,
            None => {
                return ActionResult {
                    success: false,
                    message: "Docker is not available".to_string(),
                    container_id: id.to_string(),
                }
            }
        };

        match docker
            .stop_container(id, None::<bollard::container::StopContainerOptions>)
            .await
        {
            Ok(_) => ActionResult {
                success: true,
                message: "Container stopped successfully".to_string(),
                container_id: id.to_string(),
            },
            Err(e) => ActionResult {
                success: false,
                message: format!("Failed to stop container: {e}"),
                container_id: id.to_string(),
            },
        }
    }

    pub async fn restart_container(&self, id: &str) -> ActionResult {
        let docker = match &self.docker {
            Some(d) => d,
            None => {
                return ActionResult {
                    success: false,
                    message: "Docker is not available".to_string(),
                    container_id: id.to_string(),
                }
            }
        };

        match docker
            .restart_container(id, None::<bollard::container::RestartContainerOptions>)
            .await
        {
            Ok(_) => ActionResult {
                success: true,
                message: "Container restarted successfully".to_string(),
                container_id: id.to_string(),
            },
            Err(e) => ActionResult {
                success: false,
                message: format!("Failed to restart container: {e}"),
                container_id: id.to_string(),
            },
        }
    }

    pub async fn get_container_logs(&self, id: &str, tail: usize) -> ContainerLogs {
        let docker = match &self.docker {
            Some(d) => d,
            None => {
                return ContainerLogs {
                    id: id.to_string(),
                    logs: String::new(),
                }
            }
        };

        let options = LogsOptions::<String> {
            stdout: true,
            stderr: true,
            tail: tail.to_string(),
            ..Default::default()
        };

        let mut logs = String::new();

        let mut stream = docker.logs::<String>(id, Some(options));
        while let Some(Ok(output)) = stream.next().await {
            use std::fmt::Write;
            let _ = write!(logs, "{output}");
        }

        ContainerLogs {
            id: id.to_string(),
            logs,
        }
    }

    pub async fn delete_container(&self, id: &str) -> ActionResult {
        let docker = match &self.docker {
            Some(d) => d,
            None => {
                return ActionResult {
                    success: false,
                    message: "Docker is not available".to_string(),
                    container_id: id.to_string(),
                }
            }
        };

        let options = RemoveContainerOptions {
            force: true,
            v: true,
            link: false,
        };

        match docker.remove_container(id, Some(options)).await {
            Ok(_) => ActionResult {
                success: true,
                message: "Container removed successfully".to_string(),
                container_id: id.to_string(),
            },
            Err(e) => ActionResult {
                success: false,
                message: format!("Failed to remove container: {e}"),
                container_id: id.to_string(),
            },
        }
    }
}
