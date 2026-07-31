# API Reference

Base URL: `http://localhost:8080/api`

## Authentication

### Login
```bash
POST /api/auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "admin123"
}
```

Response (200):
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "uuid",
  "expires_in": 86400
}
```

Default credentials: `admin` / `admin123`, `operator` / `operator123`

### Refresh Token
```bash
POST /api/auth/refresh
Content-Type: application/json

{
  "refresh_token": "uuid"
}
```

### Logout
```bash
POST /api/auth/logout
Authorization: Bearer <token>
```

Response:
```json
{
  "message": "Logged out successfully"
}
```

### Get Current User
```bash
GET /api/auth/me
Authorization: Bearer <token>
```

Response:
```json
{
  "id": "uuid",
  "username": "admin",
  "email": "admin@nebula.local",
  "roles": ["admin"]
}
```

## Users

### List Users
```bash
GET /api/users
Authorization: Bearer <token>
```

Response:
```json
[
  {
    "id": "uuid",
    "username": "admin",
    "email": "admin@nebula.local"
  }
]
```

### Create User
```bash
POST /api/users
Authorization: Bearer <token>
Content-Type: application/json

{
  "username": "jdoe",
  "email": "jdoe@example.com",
  "password": "secure-password"
}
```

### Get User
```bash
GET /api/users/:id
Authorization: Bearer <token>
```

### Update User
```bash
PUT /api/users/:id
Authorization: Bearer <token>
Content-Type: application/json

{
  "email": "newemail@example.com"
}
```

### Delete User
```bash
DELETE /api/users/:id
Authorization: Bearer <token>
```

## Nodes

### List Nodes
```bash
GET /api/nodes
Authorization: Bearer <token>
```

Response:
```json
[
  {
    "id": "uuid",
    "hostname": "compute-1.nebula.internal",
    "ip_address": "10.0.1.10",
    "status": "online",
    "cpu_percent": 45.2,
    "ram_percent": 62.8,
    "disk_percent": 71.3,
    "os_name": "NebulaOS 2.1",
    "last_seen_at": "2025-06-15T10:30:00Z"
  }
]
```

### Register Node
```bash
POST /api/nodes/register
Authorization: Bearer <token>
Content-Type: application/json

{
  "hostname": "web-server-01",
  "ip_address": "192.168.1.10",
  "os_name": "Ubuntu",
  "os_version": "22.04",
  "cpu_cores": 8,
  "ram_total_bytes": 17179869184,
  "disk_total_bytes": 536870912000
}
```

### Get Node
```bash
GET /api/nodes/:id
Authorization: Bearer <token>
```

### Update Node
```bash
PUT /api/nodes/:id
Authorization: Bearer <token>
Content-Type: application/json

{
  "hostname": "updated-hostname",
  "status": "online"
}
```

### Delete Node
```bash
DELETE /api/nodes/:id
Authorization: Bearer <token>
```

### Get Node Metrics History
```bash
GET /api/nodes/:id/metrics
Authorization: Bearer <token>
```

Response:
```json
[
  {
    "node_id": "uuid",
    "cpu_percent": 45.2,
    "ram_percent": 62.8,
    "disk_percent": 71.3,
    "net_rx_bytes": 104857600,
    "net_tx_bytes": 52428800,
    "collected_at": "2025-06-15T10:30:00Z"
  }
]
```

### Get Latest Metrics
```bash
GET /api/nodes/:id/metrics/latest
Authorization: Bearer <token>
```

Response:
```json
{
  "node_id": "uuid",
  "cpu_percent": 45.2,
  "ram_percent": 62.8,
  "disk_percent": 71.3,
  "net_rx_bytes": 104857600,
  "net_tx_bytes": 52428800,
  "collected_at": "2025-06-15T10:30:00Z"
}
```

### Submit Metrics
```bash
POST /api/nodes/:id/metrics
Authorization: Bearer <token>
Content-Type: application/json

{
  "cpu_percent": 45.2,
  "ram_percent": 62.8,
  "ram_used_bytes": 8589934592,
  "ram_total_bytes": 17179869184,
  "disk_percent": 71.3,
  "disk_used_bytes": 382730240000,
  "disk_total_bytes": 536870912000,
  "net_rx_bytes": 1048576,
  "net_tx_bytes": 524288,
  "process_count": 234
}
```

### Execute Command on Node
```bash
POST /api/nodes/:id/command
Authorization: Bearer <token>
```

Response:
```json
{
  "success": true,
  "message": "Command queued for execution on node"
}
```

## Containers

### List Containers
```bash
GET /api/containers
Authorization: Bearer <token>
```

### Get Container
```bash
GET /api/containers/:id
Authorization: Bearer <token>
```

### Start Container
```bash
POST /api/containers/:id/start
Authorization: Bearer <token>
```

### Stop Container
```bash
POST /api/containers/:id/stop
Authorization: Bearer <token>
```

### Restart Container
```bash
POST /api/containers/:id/restart
Authorization: Bearer <token>
```

### Get Container Logs
```bash
GET /api/containers/:id/logs?tail=100
Authorization: Bearer <token>
```

### Delete Container
```bash
DELETE /api/containers/:id
Authorization: Bearer <token>
```

## Virtual Machines

### List VMs
```bash
GET /api/vms
Authorization: Bearer <token>
```

### Create VM
```bash
POST /api/vms
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "web-vm-01",
  "os_type": "ubuntu",
  "cpu_cores": 4,
  "ram_mb": 8192,
  "disk_gb": 100,
  "node_id": "uuid"
}
```

### Get VM
```bash
GET /api/vms/:id
Authorization: Bearer <token>
```

### Update VM
```bash
PUT /api/vms/:id
Authorization: Bearer <token>
```

### Delete VM
```bash
DELETE /api/vms/:id
Authorization: Bearer <token>
```

### Start/Stop/Restart VM
```bash
POST /api/vms/:id/start
POST /api/vms/:id/stop
POST /api/vms/:id/restart
Authorization: Bearer <token>
```

### Create Snapshot
```bash
POST /api/vms/:id/snapshot
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "before-update-v2"
}
```

### List Snapshots
```bash
GET /api/vms/:id/snapshots
Authorization: Bearer <token>
```

### Backup VM
```bash
POST /api/vms/:id/backup
Authorization: Bearer <token>
```

## Kubernetes

### List K8s Nodes
```bash
GET /api/k8s/nodes
Authorization: Bearer <token>
```

### List Pods
```bash
GET /api/k8s/pods
Authorization: Bearer <token>
```

### List Deployments
```bash
GET /api/k8s/deployments
Authorization: Bearer <token>
```

### List Services
```bash
GET /api/k8s/services
Authorization: Bearer <token>
```

### List Namespaces
```bash
GET /api/k8s/namespaces
Authorization: Bearer <token>
```

## Monitoring

### Overview
```bash
GET /api/monitoring/overview
Authorization: Bearer <token>
```

Response:
```json
{
  "total_nodes": 3,
  "online_nodes": 2,
  "total_containers": 3,
  "total_vms": 3,
  "active_alerts": 2,
  "avg_cpu_percent": 41.1,
  "avg_ram_percent": 36.03
}
```

### List Alerts
```bash
GET /api/monitoring/alerts
Authorization: Bearer <token>
```

### Acknowledge Alert
```bash
POST /api/monitoring/alerts/:id/ack
Authorization: Bearer <token>
```

### Prometheus Query
```bash
GET /api/monitoring/prometheus/query?query=node_cpu_seconds_total
Authorization: Bearer <token>
```

Response:
```json
{
  "status": "success",
  "data": {
    "resultType": "vector",
    "result": [
      {
        "metric": {
          "__name__": "node_cpu_percent",
          "hostname": "compute-1.nebula.internal",
          "instance": "10.0.1.10"
        },
        "value": [1718461800, "45.2"]
      }
    ]
  }
}
```

## Audit

```bash
GET /api/audit
```

Returns recent audit events (labs: in-memory).

## Backups

```bash
GET    /api/backups
POST   /api/backups
DELETE /api/backups/:id
POST   /api/backups/:id/restore
```

Labs store backup metadata in memory (not real disk images).

## Storage

```bash
GET    /api/storage/pools
POST   /api/storage/pools
GET    /api/storage/pools/:id
PUT    /api/storage/pools/:id
DELETE /api/storage/pools/:id
GET    /api/storage/volumes
POST   /api/storage/volumes
PUT    /api/storage/volumes/:id
DELETE /api/storage/volumes/:id
```

## Plugins

```bash
GET    /api/plugins
POST   /api/plugins
GET    /api/plugins/:id
PUT    /api/plugins/:id
DELETE /api/plugins/:id
```

Seed plugins include prometheus-exporter, slack-notifier, webhook-hooks, ldap-auth.

## WebSocket

Connect to real-time metrics stream:

```javascript
const ws = new WebSocket(`ws://localhost:8080/ws?token=${jwt}`);
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
};
```

## Health

```bash
GET /api/health
```

Response:
```json
{
  "status": "ok",
  "version": "0.1.0",
  "uptime_seconds": 42
}
```

```bash
GET /api/health/ready
```

## HTTP Status Codes

| Code | Meaning |
|------|---------|
| 200 | Success |
| 201 | Created |
| 204 | No Content (delete success) |
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 409 | Conflict |
| 500 | Internal Server Error |
