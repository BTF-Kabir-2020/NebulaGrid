# NebulaGrid Architecture

## Overview

NebulaGrid is an infrastructure control-plane project (Rust + React) aimed at managing servers, containers, VMs, Kubernetes, networking, storage, and automation from one dashboard.

> **Current runtime:** the gateway serves REST/JWT over **in-memory seed data**. Postgres/Redis/NATS may run in Compose but are not yet the system of record. Diagrams marked **Target** are the longer-term shape; **Current** is what runs today.

## Target Architecture (Complete)

```
┌───────────────────────────┐
│        Web Dashboard      │
│ React + TS (+ Tauri opt.) │
└─────────────┬─────────────┘
              │ HTTPS / gRPC / WebSocket
              ▼
┌───────────────────────────┐
│        API Gateway        │
│     Rust (Axum + Tower)   │
└─────────────┬─────────────┘
              │
════════════════════════════════════════════════════════
        Authentication / Authorization Layer
════════════════════════════════════════════════════════
 JWT · OAuth2 · OIDC · LDAP · RBAC · API Keys · Audit Logs

════════════════════════════════════════════════════════
              Internal Event Bus (NATS / JetStream)
════════════════════════════════════════════════════════
      │            │            │            │
      ▼            ▼            ▼            ▼
  Cluster      Scheduler     Automation    Alert Engine
  (K8s)                                    + Notifiers

════════════════════════════════════════════════════════
                 Infrastructure Services
════════════════════════════════════════════════════════
 Node Manager · Container Manager · VM Manager
 Storage Manager · Backup Manager · Snapshot Manager
 Metrics Collector · Inventory · Network Manager
 Policy Engine · Certificate Manager · Configuration Manager
 Plugin Manager · Secrets Manager · DNS / LB Manager

════════════════════════════════════════════════════════
 Observability          Multi-tenancy         Control-plane HA
 Prometheus/Grafana     Org / Project         Leader election
 Loki / OTel traces     Quotas                Agent auto-update
                        Audit export

════════════════════════════════════════════════════════
           PostgreSQL · Redis · Object Storage
════════════════════════════════════════════════════════
               Rust Agents (Linux · Windows · macOS)
                           │
              Server 1 · Server 2 · … · Server N
```

### What we build vs integrate

| Build | Integrate |
|-------|-----------|
| Control plane, Gateway, Agents, Dashboard | Docker, Proxmox/KVM, Kubernetes |
| Adapters, RBAC, event bus wiring | PostgreSQL, Redis, NATS, MinIO |
| Policy/automation orchestration | Prometheus, Keycloak/OIDC IdP |

## Runtime Architecture (Current Gateway)

```
┌─────────────────────────────────────────────────────────────┐
│               DASHBOARD (host :5173 in Compose)              │
│               React 18 + TypeScript + Tailwind                │
└──────────────────────────┬──────────────────────────────────┘
                           │ HTTP (Vite proxy → :8080)
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   GATEWAY (Port 8080)                        │
│               Rust + Axum + Tokio + Tower                    │
│  Auth (JWT) · REST /api/* · in-memory seed state             │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
                    Lab agent simulators /
                    optional Rust agent
```

### Gateway components (current)

- HTTP API entrypoint with JWT auth (admin / operator / viewer roles)
- In-memory store with seed data (Postgres/Redis/NATS env vars accepted but not wired as persistence yet)
- Live metrics fan-out via WebSocket hub (where enabled)
- Microservice crates exist in the workspace; traffic today goes through the gateway in-memory APIs

### Agents

Rust agent collects CPU/RAM/Disk/Network and can register/report to the gateway. Compose also ships Python agent simulators for local stacks.

### Dashboard

Pages for servers, containers, VMs, Kubernetes, networks, storage, jobs, inventory, policies, certificates, config, monitoring, logs, backups, plugins, alerts, settings, and API docs.

## Data Flow

```
Agent / simulator (interval)
  → POST /api/nodes/:id/metrics → Gateway
       → store metrics in memory + update node gauges
       → optional WsHub publish
       → Dashboard charts refresh
```

## Security

- Passwords: Argon2
- Session: JWT + refresh tokens
- Route-level RBAC
- CORS is open for local demos — tighten before public deploy

## Technology Stack

| Layer | Technology |
|-------|------------|
| HTTP | Axum 0.7 + Tower |
| Async | Tokio |
| State (current) | In-memory seed (Postgres/Redis/NATS planned) |
| Auth | JWT + Argon2 |
| Frontend | React 18 + TypeScript + Vite + Tailwind |
| Packaging | Docker Compose + deployment Dockerfiles / Helm / Terraform |
