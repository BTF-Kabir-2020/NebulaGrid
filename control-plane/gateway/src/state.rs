use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct StoredUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct StoredNode {
    pub id: Uuid,
    pub hostname: String,
    pub ip_address: String,
    pub status: String,
    pub cpu_percent: f64,
    pub ram_percent: f64,
    pub disk_percent: f64,
    pub os_name: String,
    pub last_seen_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredNodeMetrics {
    pub node_id: Uuid,
    pub cpu_percent: f64,
    pub ram_percent: f64,
    pub disk_percent: f64,
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
    pub collected_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredContainer {
    pub id: Uuid,
    pub container_id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub node_id: Uuid,
    pub ports: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredVm {
    pub id: Uuid,
    pub name: String,
    pub os_type: String,
    pub cpu_cores: u32,
    pub ram_mb: u32,
    pub disk_gb: u32,
    pub status: String,
    pub node_id: Uuid,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StoredVmSnapshot {
    pub name: String,
    pub created_at: String,
    pub size_bytes: u64,
    pub vm_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct StoredK8sPod {
    pub name: String,
    pub namespace: String,
    pub status: String,
    pub node: String,
    pub pod_ip: String,
}

#[derive(Debug, Clone)]
pub struct StoredK8sDeployment {
    pub name: String,
    pub namespace: String,
    pub replicas: u32,
    pub available_replicas: u32,
}

#[derive(Debug, Clone)]
pub struct StoredK8sService {
    pub name: String,
    pub namespace: String,
    pub cluster_ip: String,
    pub ports: Vec<String>,
    pub type_: String,
}

#[derive(Debug, Clone)]
pub struct StoredK8sNode {
    pub name: String,
    pub status: String,
    pub version: String,
    pub pod_count: u32,
}

#[derive(Debug, Clone)]
pub struct StoredAlert {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub severity: String,
    pub source: String,
    pub acknowledged: bool,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredJob {
    pub id: Uuid,
    pub name: String,
    pub job_type: String,
    pub status: String,
    pub target_nodes: Vec<String>,
    pub params: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StoredNetwork {
    pub id: Uuid,
    pub name: String,
    pub subnet: String,
    pub gateway: Option<String>,
    pub vlan_id: Option<i32>,
    pub network_type: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredStoragePool {
    pub id: Uuid,
    pub name: String,
    pub pool_type: String,
    pub total_bytes: i64,
    pub used_bytes: i64,
    pub free_bytes: i64,
    pub mount_path: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredVolume {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub name: String,
    pub size_bytes: i64,
    pub volume_type: String,
    pub format: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredInventoryItem {
    pub id: Uuid,
    pub asset_tag: String,
    pub name: String,
    pub item_type: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub location: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredPolicy {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub policy_type: String,
    pub scope: String,
    pub enforcement: String,
    pub priority: i32,
    pub enabled: bool,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredCertificate {
    pub id: Uuid,
    pub name: String,
    pub common_name: String,
    pub issuer: String,
    pub status: String,
    pub not_after: String,
    pub fingerprint: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredConfigEntry {
    pub id: Uuid,
    pub key: String,
    pub value: serde_json::Value,
    pub group: String,
    pub description: Option<String>,
    pub version: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredAuditEvent {
    pub id: Uuid,
    pub actor: String,
    pub action: String,
    pub resource: String,
    pub details: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct StoredBackup {
    pub id: Uuid,
    pub name: String,
    pub backup_type: String,
    pub status: String,
    pub size_bytes: i64,
    pub created_at: String,
    pub restored_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StoredPlugin {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: String,
    pub enabled: bool,
    pub config: serde_json::Value,
    pub installed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredApiToken {
    pub id: Uuid,
    pub name: String,
    pub token: String,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPrefs {
    pub email_alerts: bool,
    pub email_digest: bool,
    pub browser_alerts: bool,
    pub slack_webhook: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub roles: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

pub struct AppState {
    pub jwt_secret: String,
    pub users: Mutex<Vec<StoredUser>>,
    pub nodes: Mutex<Vec<StoredNode>>,
    pub node_metrics: Mutex<HashMap<Uuid, Vec<StoredNodeMetrics>>>,
    pub containers: Mutex<Vec<StoredContainer>>,
    pub vms: Mutex<Vec<StoredVm>>,
    pub vm_snapshots: Mutex<Vec<StoredVmSnapshot>>,
    pub k8s_pods: Mutex<Vec<StoredK8sPod>>,
    pub k8s_deployments: Mutex<Vec<StoredK8sDeployment>>,
    pub k8s_services: Mutex<Vec<StoredK8sService>>,
    pub k8s_nodes: Mutex<Vec<StoredK8sNode>>,
    pub alerts: Mutex<Vec<StoredAlert>>,
    pub jobs: Mutex<Vec<StoredJob>>,
    pub networks: Mutex<Vec<StoredNetwork>>,
    pub storage_pools: Mutex<Vec<StoredStoragePool>>,
    pub volumes: Mutex<Vec<StoredVolume>>,
    pub inventory: Mutex<Vec<StoredInventoryItem>>,
    pub policies: Mutex<Vec<StoredPolicy>>,
    pub certificates: Mutex<Vec<StoredCertificate>>,
    pub config_entries: Mutex<Vec<StoredConfigEntry>>,
    pub audit_events: Mutex<Vec<StoredAuditEvent>>,
    pub backups: Mutex<Vec<StoredBackup>>,
    pub plugins: Mutex<Vec<StoredPlugin>>,
    pub refresh_tokens: Mutex<HashMap<String, String>>,
    pub api_tokens: Mutex<HashMap<Uuid, Vec<StoredApiToken>>>,
    pub notification_prefs: Mutex<HashMap<Uuid, NotificationPrefs>>,
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

impl AppState {
    pub fn new(jwt_secret: String) -> Self {
        let admin_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        let node1_id = Uuid::parse_str("00000000-0000-0000-0000-000000000010").unwrap();
        let node2_id = Uuid::parse_str("00000000-0000-0000-0000-000000000011").unwrap();
        let node3_id = Uuid::parse_str("00000000-0000-0000-0000-000000000012").unwrap();

        let admin = StoredUser {
            id: admin_id,
            username: "admin".into(),
            email: "admin@nebula.local".into(),
            password_hash: hash_password("admin123"),
            roles: vec!["admin".into()],
        };

        let operator = StoredUser {
            id: Uuid::new_v4(),
            username: "operator".into(),
            email: "operator@nebula.local".into(),
            password_hash: hash_password("operator123"),
            roles: vec!["operator".into()],
        };

        let nodes = vec![
            StoredNode {
                id: node1_id,
                hostname: "compute-1.nebula.internal".into(),
                ip_address: "10.0.1.10".into(),
                status: "online".into(),
                cpu_percent: 45.2,
                ram_percent: 62.8,
                disk_percent: 71.3,
                os_name: "NebulaOS 2.1".into(),
                last_seen_at: "2025-06-15T10:30:00Z".into(),
            },
            StoredNode {
                id: node2_id,
                hostname: "compute-2.nebula.internal".into(),
                ip_address: "10.0.1.11".into(),
                status: "online".into(),
                cpu_percent: 78.1,
                ram_percent: 45.3,
                disk_percent: 55.0,
                os_name: "NebulaOS 2.1".into(),
                last_seen_at: "2025-06-15T10:30:00Z".into(),
            },
            StoredNode {
                id: node3_id,
                hostname: "compute-3.nebula.internal".into(),
                ip_address: "10.0.1.12".into(),
                status: "offline".into(),
                cpu_percent: 0.0,
                ram_percent: 0.0,
                disk_percent: 88.2,
                os_name: "NebulaOS 2.0".into(),
                last_seen_at: "2025-06-14T22:15:00Z".into(),
            },
        ];

        let mut node_metrics: HashMap<Uuid, Vec<StoredNodeMetrics>> = HashMap::new();
        for node in &nodes {
            let metrics = vec![
                StoredNodeMetrics {
                    node_id: node.id,
                    cpu_percent: node.cpu_percent,
                    ram_percent: node.ram_percent,
                    disk_percent: node.disk_percent,
                    net_rx_bytes: 104857600,
                    net_tx_bytes: 52428800,
                    collected_at: "2025-06-15T10:30:00Z".into(),
                },
                StoredNodeMetrics {
                    node_id: node.id,
                    cpu_percent: node.cpu_percent - 5.0,
                    ram_percent: node.ram_percent + 2.0,
                    disk_percent: node.disk_percent,
                    net_rx_bytes: 83886080,
                    net_tx_bytes: 41943040,
                    collected_at: "2025-06-15T10:25:00Z".into(),
                },
            ];
            node_metrics.insert(node.id, metrics);
        }

        let containers = vec![
            StoredContainer {
                id: Uuid::new_v4(),
                container_id: "abc123def456".into(),
                name: "nginx-proxy".into(),
                image: "nginx:1.25-alpine".into(),
                status: "running".into(),
                node_id: node1_id,
                ports: vec!["0.0.0.0:80->80/tcp".into(), "0.0.0.0:443->443/tcp".into()],
                created_at: "2025-06-10T08:00:00Z".into(),
            },
            StoredContainer {
                id: Uuid::new_v4(),
                container_id: "def789ghi012".into(),
                name: "redis-cache".into(),
                image: "redis:7.2-alpine".into(),
                status: "running".into(),
                node_id: node1_id,
                ports: vec!["0.0.0.0:6379->6379/tcp".into()],
                created_at: "2025-06-11T12:00:00Z".into(),
            },
            StoredContainer {
                id: Uuid::new_v4(),
                container_id: "jkl345mno678".into(),
                name: "postgres-db".into(),
                image: "postgres:16".into(),
                status: "stopped".into(),
                node_id: node2_id,
                ports: vec!["0.0.0.0:5432->5432/tcp".into()],
                created_at: "2025-06-09T16:30:00Z".into(),
            },
        ];

        let vms = vec![
            StoredVm {
                id: Uuid::new_v4(),
                name: "web-server-01".into(),
                os_type: "ubuntu-22.04".into(),
                cpu_cores: 4,
                ram_mb: 8192,
                disk_gb: 100,
                status: "running".into(),
                node_id: node1_id,
                ip_address: Some("10.0.1.10".into()),
            },
            StoredVm {
                id: Uuid::new_v4(),
                name: "db-server-01".into(),
                os_type: "debian-12".into(),
                cpu_cores: 8,
                ram_mb: 16384,
                disk_gb: 500,
                status: "running".into(),
                node_id: node2_id,
                ip_address: Some("10.0.2.20".into()),
            },
            StoredVm {
                id: Uuid::new_v4(),
                name: "dev-sandbox-01".into(),
                os_type: "ubuntu-24.04".into(),
                cpu_cores: 2,
                ram_mb: 4096,
                disk_gb: 50,
                status: "stopped".into(),
                node_id: node3_id,
                ip_address: None,
            },
        ];

        let vm_snapshots = vec![
            StoredVmSnapshot {
                name: "pre-upgrade-snap".into(),
                created_at: "2025-06-10T08:00:00Z".into(),
                size_bytes: 2147483648,
                vm_id: vms[0].id,
            },
            StoredVmSnapshot {
                name: "backup-2025-06-01".into(),
                created_at: "2025-06-01T00:00:00Z".into(),
                size_bytes: 4294967296,
                vm_id: vms[0].id,
            },
        ];

        let k8s_pods = vec![
            StoredK8sPod {
                name: "nginx-frontend-7d8f9c5b6c-xk4m2".into(),
                namespace: "default".into(),
                status: "Running".into(),
                node: "worker-1".into(),
                pod_ip: "10.42.0.10".into(),
            },
            StoredK8sPod {
                name: "api-gateway-6c8b7d9e5f-ht9p1".into(),
                namespace: "default".into(),
                status: "Running".into(),
                node: "worker-2".into(),
                pod_ip: "10.42.0.11".into(),
            },
            StoredK8sPod {
                name: "redis-cache-9f4e3d2c1b-v8n7m".into(),
                namespace: "cache".into(),
                status: "Running".into(),
                node: "worker-1".into(),
                pod_ip: "10.42.1.5".into(),
            },
        ];

        let k8s_deployments = vec![
            StoredK8sDeployment {
                name: "nginx-frontend".into(),
                namespace: "default".into(),
                replicas: 3,
                available_replicas: 3,
            },
            StoredK8sDeployment {
                name: "api-gateway".into(),
                namespace: "default".into(),
                replicas: 2,
                available_replicas: 2,
            },
            StoredK8sDeployment {
                name: "redis-cache".into(),
                namespace: "cache".into(),
                replicas: 1,
                available_replicas: 1,
            },
        ];

        let k8s_services = vec![
            StoredK8sService {
                name: "nginx-frontend".into(),
                namespace: "default".into(),
                cluster_ip: "10.96.0.10".into(),
                ports: vec!["80/TCP".into(), "443/TCP".into()],
                type_: "ClusterIP".into(),
            },
            StoredK8sService {
                name: "api-gateway".into(),
                namespace: "default".into(),
                cluster_ip: "10.96.0.20".into(),
                ports: vec!["8080/TCP".into(), "8443/TCP".into()],
                type_: "ClusterIP".into(),
            },
            StoredK8sService {
                name: "ingress-nginx".into(),
                namespace: "ingress".into(),
                cluster_ip: "10.96.0.1".into(),
                ports: vec!["80:30080/TCP".into(), "443:30443/TCP".into()],
                type_: "NodePort".into(),
            },
        ];

        let k8s_nodes = vec![
            StoredK8sNode {
                name: "worker-1".into(),
                status: "Ready".into(),
                version: "v1.30.2".into(),
                pod_count: 12,
            },
            StoredK8sNode {
                name: "worker-2".into(),
                status: "Ready".into(),
                version: "v1.30.2".into(),
                pod_count: 9,
            },
            StoredK8sNode {
                name: "control-plane-1".into(),
                status: "Ready".into(),
                version: "v1.30.2".into(),
                pod_count: 8,
            },
        ];

        let alerts = vec![
            StoredAlert {
                id: Uuid::new_v4(),
                title: "High CPU Usage".into(),
                message: "Node compute-2 CPU usage at 95%".into(),
                severity: "critical".into(),
                source: "monitoring-agent".into(),
                acknowledged: false,
                created_at: "2025-06-15T10:15:00Z".into(),
            },
            StoredAlert {
                id: Uuid::new_v4(),
                title: "Disk Space Warning".into(),
                message: "Node compute-3 disk usage at 88%".into(),
                severity: "warning".into(),
                source: "monitoring-agent".into(),
                acknowledged: false,
                created_at: "2025-06-15T09:45:00Z".into(),
            },
            StoredAlert {
                id: Uuid::new_v4(),
                title: "Node Offline".into(),
                message: "Node compute-3 has been unreachable for 12 hours".into(),
                severity: "critical".into(),
                source: "heartbeat-checker".into(),
                acknowledged: true,
                created_at: "2025-06-14T22:15:00Z".into(),
            },
        ];

        let networks = vec![
            StoredNetwork {
                id: Uuid::new_v4(),
                name: "mgmt-net".into(),
                subnet: "10.0.1.0/24".into(),
                gateway: Some("10.0.1.1".into()),
                vlan_id: Some(10),
                network_type: "bridge".into(),
                status: "active".into(),
                created_at: "2025-06-01T00:00:00Z".into(),
            },
            StoredNetwork {
                id: Uuid::new_v4(),
                name: "workload-net".into(),
                subnet: "10.10.0.0/16".into(),
                gateway: Some("10.10.0.1".into()),
                vlan_id: Some(100),
                network_type: "overlay".into(),
                status: "active".into(),
                created_at: "2025-06-01T00:00:00Z".into(),
            },
        ];

        let pool_id = Uuid::new_v4();
        let storage_pools = vec![
            StoredStoragePool {
                id: pool_id,
                name: "local-lvm".into(),
                pool_type: "lvm".into(),
                total_bytes: 2_199_023_255_552,
                used_bytes: 879_609_302_220,
                free_bytes: 1_319_413_953_332,
                mount_path: Some("/var/lib/nebula/storage".into()),
                status: "online".into(),
                created_at: "2025-06-01T00:00:00Z".into(),
            },
            StoredStoragePool {
                id: Uuid::new_v4(),
                name: "s3-backups".into(),
                pool_type: "object".into(),
                total_bytes: 10_995_116_277_760,
                used_bytes: 2_199_023_255_552,
                free_bytes: 8_796_093_022_208,
                mount_path: None,
                status: "online".into(),
                created_at: "2025-06-01T00:00:00Z".into(),
            },
        ];

        let volumes = vec![
            StoredVolume {
                id: Uuid::new_v4(),
                pool_id,
                name: "vm-web-root".into(),
                size_bytes: 107_374_182_400,
                volume_type: "block".into(),
                format: "qcow2".into(),
                status: "attached".into(),
                created_at: "2025-06-10T08:00:00Z".into(),
            },
            StoredVolume {
                id: Uuid::new_v4(),
                pool_id,
                name: "db-data".into(),
                size_bytes: 536_870_912_000,
                volume_type: "block".into(),
                format: "raw".into(),
                status: "attached".into(),
                created_at: "2025-06-09T16:30:00Z".into(),
            },
        ];

        let jobs = vec![StoredJob {
            id: Uuid::new_v4(),
            name: "nightly-backup".into(),
            job_type: "backup".into(),
            status: "completed".into(),
            target_nodes: vec!["compute-1.nebula.internal".into()],
            params: serde_json::json!({ "retention_days": 7 }),
            result: Some(serde_json::json!({ "message": "Backup finished" })),
            error: None,
            created_at: "2025-06-15T02:00:00Z".into(),
            started_at: Some("2025-06-15T02:00:01Z".into()),
            completed_at: Some("2025-06-15T02:12:00Z".into()),
        }];

        let inventory = vec![
            StoredInventoryItem {
                id: Uuid::new_v4(),
                asset_tag: "AST-1001".into(),
                name: "Dell R750".into(),
                item_type: "server".into(),
                manufacturer: Some("Dell".into()),
                model: Some("PowerEdge R750".into()),
                location: Some("DC1-Rack-A".into()),
                status: "active".into(),
                created_at: "2025-06-01T00:00:00Z".into(),
            },
            StoredInventoryItem {
                id: Uuid::new_v4(),
                asset_tag: "AST-2001".into(),
                name: "Cisco Nexus".into(),
                item_type: "switch".into(),
                manufacturer: Some("Cisco".into()),
                model: Some("N9K-C93180".into()),
                location: Some("DC1-Rack-A".into()),
                status: "active".into(),
                created_at: "2025-06-01T00:00:00Z".into(),
            },
        ];

        let policies = vec![
            StoredPolicy {
                id: Uuid::new_v4(),
                name: "require-tls".into(),
                description: Some("Deny plaintext HTTP on edge".into()),
                policy_type: "security".into(),
                scope: "cluster".into(),
                enforcement: "enforce".into(),
                priority: 10,
                enabled: true,
                created_at: "2025-06-01T00:00:00Z".into(),
            },
            StoredPolicy {
                id: Uuid::new_v4(),
                name: "max-vm-size".into(),
                description: Some("Limit VM RAM to 64GB".into()),
                policy_type: "quota".into(),
                scope: "tenant".into(),
                enforcement: "enforce".into(),
                priority: 50,
                enabled: true,
                created_at: "2025-06-01T00:00:00Z".into(),
            },
        ];

        let certificates = vec![
            StoredCertificate {
                id: Uuid::new_v4(),
                name: "api-gateway".into(),
                common_name: "api.nebula.local".into(),
                issuer: "NebulaGrid Root CA".into(),
                status: "valid".into(),
                not_after: "2026-06-01T00:00:00Z".into(),
                fingerprint: "sha256:abcd1234ef567890".into(),
                created_at: "2025-06-01T00:00:00Z".into(),
            },
            StoredCertificate {
                id: Uuid::new_v4(),
                name: "dashboard".into(),
                common_name: "dashboard.nebula.local".into(),
                issuer: "NebulaGrid Root CA".into(),
                status: "valid".into(),
                not_after: "2026-06-01T00:00:00Z".into(),
                fingerprint: "sha256:1122334455667788".into(),
                created_at: "2025-06-01T00:00:00Z".into(),
            },
        ];

        let config_entries = vec![
            StoredConfigEntry {
                id: Uuid::new_v4(),
                key: "metrics.interval_seconds".into(),
                value: serde_json::json!(10),
                group: "agent".into(),
                description: Some("Default agent scrape interval".into()),
                version: 1,
                created_at: "2025-06-01T00:00:00Z".into(),
                updated_at: "2025-06-01T00:00:00Z".into(),
            },
            StoredConfigEntry {
                id: Uuid::new_v4(),
                key: "alerts.cpu_threshold".into(),
                value: serde_json::json!(90),
                group: "monitoring".into(),
                description: Some("CPU alert threshold percent".into()),
                version: 1,
                created_at: "2025-06-01T00:00:00Z".into(),
                updated_at: "2025-06-01T00:00:00Z".into(),
            },
        ];

        let audit_events = vec![
            StoredAuditEvent {
                id: Uuid::new_v4(),
                actor: "admin".into(),
                action: "auth.login".into(),
                resource: "user/admin".into(),
                details: "Successful login".into(),
                created_at: "2025-06-15T08:00:00Z".into(),
            },
            StoredAuditEvent {
                id: Uuid::new_v4(),
                actor: "admin".into(),
                action: "vm.start".into(),
                resource: "vm/web-server-01".into(),
                details: "Started virtual machine".into(),
                created_at: "2025-06-15T09:12:00Z".into(),
            },
            StoredAuditEvent {
                id: Uuid::new_v4(),
                actor: "operator".into(),
                action: "policy.update".into(),
                resource: "policy/require-tls".into(),
                details: "Enabled enforcement".into(),
                created_at: "2025-06-15T10:01:00Z".into(),
            },
            StoredAuditEvent {
                id: Uuid::new_v4(),
                actor: "system".into(),
                action: "backup.create".into(),
                resource: "backup/nightly".into(),
                details: "Scheduled full backup completed".into(),
                created_at: "2025-06-16T01:00:00Z".into(),
            },
        ];

        let backups = vec![
            StoredBackup {
                id: Uuid::new_v4(),
                name: "nightly-2025-06-16".into(),
                backup_type: "full".into(),
                status: "completed".into(),
                size_bytes: 512 * 1024 * 1024,
                created_at: "2025-06-16T01:00:00Z".into(),
                restored_at: None,
            },
            StoredBackup {
                id: Uuid::new_v4(),
                name: "config-2025-06-10".into(),
                backup_type: "config".into(),
                status: "completed".into(),
                size_bytes: 8 * 1024 * 1024,
                created_at: "2025-06-10T12:00:00Z".into(),
                restored_at: None,
            },
        ];

        let plugins = vec![
            StoredPlugin {
                id: Uuid::new_v4(),
                name: "prometheus-exporter".into(),
                version: "1.2.0".into(),
                description: "Expose cluster metrics in Prometheus format".into(),
                category: "observability".into(),
                enabled: true,
                config: serde_json::json!({ "listen": ":9100", "path": "/metrics" }),
                installed_at: "2025-06-01T00:00:00Z".into(),
            },
            StoredPlugin {
                id: Uuid::new_v4(),
                name: "slack-notifier".into(),
                version: "1.0.3".into(),
                description: "Push critical alerts to Slack".into(),
                category: "notifications".into(),
                enabled: false,
                config: serde_json::json!({ "webhook": null, "channel": "#ops" }),
                installed_at: "2025-06-01T00:00:00Z".into(),
            },
            StoredPlugin {
                id: Uuid::new_v4(),
                name: "webhook-hooks".into(),
                version: "0.9.1".into(),
                description: "Outbound webhooks for job/vm lifecycle events".into(),
                category: "automation".into(),
                enabled: true,
                config: serde_json::json!({ "endpoints": [] }),
                installed_at: "2025-06-05T00:00:00Z".into(),
            },
            StoredPlugin {
                id: Uuid::new_v4(),
                name: "ldap-auth".into(),
                version: "2.0.0".into(),
                description: "Optional LDAP/AD authentication backend".into(),
                category: "security".into(),
                enabled: false,
                config: serde_json::json!({ "url": "", "base_dn": "" }),
                installed_at: "2025-06-08T00:00:00Z".into(),
            },
        ];

        Self {
            jwt_secret,
            users: Mutex::new(vec![admin, operator]),
            nodes: Mutex::new(nodes),
            node_metrics: Mutex::new(node_metrics),
            containers: Mutex::new(containers),
            vms: Mutex::new(vms),
            vm_snapshots: Mutex::new(vm_snapshots),
            k8s_pods: Mutex::new(k8s_pods),
            k8s_deployments: Mutex::new(k8s_deployments),
            k8s_services: Mutex::new(k8s_services),
            k8s_nodes: Mutex::new(k8s_nodes),
            alerts: Mutex::new(alerts),
            jobs: Mutex::new(jobs),
            networks: Mutex::new(networks),
            storage_pools: Mutex::new(storage_pools),
            volumes: Mutex::new(volumes),
            inventory: Mutex::new(inventory),
            policies: Mutex::new(policies),
            certificates: Mutex::new(certificates),
            config_entries: Mutex::new(config_entries),
            audit_events: Mutex::new(audit_events),
            backups: Mutex::new(backups),
            plugins: Mutex::new(plugins),
            refresh_tokens: Mutex::new(HashMap::new()),
            api_tokens: Mutex::new(HashMap::new()),
            notification_prefs: {
                let mut prefs = HashMap::new();
                prefs.insert(
                    admin_id,
                    NotificationPrefs {
                        email_alerts: true,
                        email_digest: false,
                        browser_alerts: true,
                        slack_webhook: None,
                    },
                );
                Mutex::new(prefs)
            },
        }
    }
}
