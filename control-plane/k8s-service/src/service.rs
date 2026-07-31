use std::sync::Mutex;

use chrono::Utc;

use crate::models::*;

pub struct K8sService {
    pods: Mutex<Vec<PodSummary>>,
    deployments: Mutex<Vec<DeploymentSummary>>,
    services: Mutex<Vec<ServiceSummary>>,
    nodes: Mutex<Vec<K8sNodeSummary>>,
}

impl K8sService {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            pods: Mutex::new(vec![
                PodSummary {
                    name: "nginx-frontend-7d8f9c5b6c-xk4m2".into(),
                    namespace: "default".into(),
                    status: "Running".into(),
                    node: "worker-1".into(),
                    ip: "10.42.0.10".into(),
                    created_at: now,
                    containers: vec!["nginx".into(), "sidecar-logger".into()],
                },
                PodSummary {
                    name: "api-gateway-6c8b7d9e5f-ht9p1".into(),
                    namespace: "default".into(),
                    status: "Running".into(),
                    node: "worker-2".into(),
                    ip: "10.42.0.11".into(),
                    created_at: now,
                    containers: vec!["gateway".into()],
                },
                PodSummary {
                    name: "redis-cache-9f4e3d2c1b-v8n7m".into(),
                    namespace: "cache".into(),
                    status: "Running".into(),
                    node: "worker-1".into(),
                    ip: "10.42.1.5".into(),
                    created_at: now,
                    containers: vec!["redis".into(), "exporter".into()],
                },
                PodSummary {
                    name: "db-migrator-job-28j3k".into(),
                    namespace: "default".into(),
                    status: "Succeeded".into(),
                    node: "worker-3".into(),
                    ip: "10.42.0.12".into(),
                    created_at: now,
                    containers: vec!["migrator".into()],
                },
                PodSummary {
                    name: "fluentd-daemon-9k2m4".into(),
                    namespace: "logging".into(),
                    status: "Running".into(),
                    node: "worker-1".into(),
                    ip: "10.42.2.1".into(),
                    created_at: now,
                    containers: vec!["fluentd".into()],
                },
            ]),
            deployments: Mutex::new(vec![
                DeploymentSummary {
                    name: "nginx-frontend".into(),
                    namespace: "default".into(),
                    replicas: 3,
                    available: 3,
                    image: "nginx:1.25".into(),
                    created_at: now,
                },
                DeploymentSummary {
                    name: "api-gateway".into(),
                    namespace: "default".into(),
                    replicas: 2,
                    available: 2,
                    image: "nebula/gateway:latest".into(),
                    created_at: now,
                },
                DeploymentSummary {
                    name: "redis-cache".into(),
                    namespace: "cache".into(),
                    replicas: 1,
                    available: 1,
                    image: "redis:7.2-alpine".into(),
                    created_at: now,
                },
            ]),
            services: Mutex::new(vec![
                ServiceSummary {
                    name: "nginx-frontend".into(),
                    namespace: "default".into(),
                    cluster_ip: "10.96.0.10".into(),
                    ports: vec!["80/TCP".into(), "443/TCP".into()],
                    type_: "ClusterIP".into(),
                },
                ServiceSummary {
                    name: "api-gateway".into(),
                    namespace: "default".into(),
                    cluster_ip: "10.96.0.20".into(),
                    ports: vec!["8080/TCP".into(), "8443/TCP".into()],
                    type_: "ClusterIP".into(),
                },
                ServiceSummary {
                    name: "redis-cache".into(),
                    namespace: "cache".into(),
                    cluster_ip: "10.96.0.30".into(),
                    ports: vec!["6379/TCP".into()],
                    type_: "ClusterIP".into(),
                },
                ServiceSummary {
                    name: "ingress-nginx".into(),
                    namespace: "ingress".into(),
                    cluster_ip: "10.96.0.1".into(),
                    ports: vec!["80:30080/TCP".into(), "443:30443/TCP".into()],
                    type_: "NodePort".into(),
                },
            ]),
            nodes: Mutex::new(vec![
                K8sNodeSummary {
                    name: "worker-1".into(),
                    status: "Ready".into(),
                    version: "v1.30.2".into(),
                    cpu_capacity: "8".into(),
                    memory_capacity: "32Gi".into(),
                    pod_count: 12,
                },
                K8sNodeSummary {
                    name: "worker-2".into(),
                    status: "Ready".into(),
                    version: "v1.30.2".into(),
                    cpu_capacity: "8".into(),
                    memory_capacity: "32Gi".into(),
                    pod_count: 9,
                },
                K8sNodeSummary {
                    name: "worker-3".into(),
                    status: "Ready".into(),
                    version: "v1.30.2".into(),
                    cpu_capacity: "4".into(),
                    memory_capacity: "16Gi".into(),
                    pod_count: 5,
                },
                K8sNodeSummary {
                    name: "control-plane-1".into(),
                    status: "Ready".into(),
                    version: "v1.30.2".into(),
                    cpu_capacity: "4".into(),
                    memory_capacity: "8Gi".into(),
                    pod_count: 8,
                },
            ]),
        }
    }

    pub fn list_pods(&self, namespace: Option<&str>) -> Vec<PodSummary> {
        let pods = self.pods.lock().unwrap();
        match namespace {
            Some(ns) => pods.iter().filter(|p| p.namespace == ns).cloned().collect(),
            None => pods.clone(),
        }
    }

    pub fn list_deployments(&self, namespace: Option<&str>) -> Vec<DeploymentSummary> {
        let deployments = self.deployments.lock().unwrap();
        match namespace {
            Some(ns) => deployments
                .iter()
                .filter(|d| d.namespace == ns)
                .cloned()
                .collect(),
            None => deployments.clone(),
        }
    }

    pub fn list_services(&self, namespace: Option<&str>) -> Vec<ServiceSummary> {
        let services = self.services.lock().unwrap();
        match namespace {
            Some(ns) => services
                .iter()
                .filter(|s| s.namespace == ns)
                .cloned()
                .collect(),
            None => services.clone(),
        }
    }

    pub fn list_nodes(&self) -> Vec<K8sNodeSummary> {
        self.nodes.lock().unwrap().clone()
    }
}
