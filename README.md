# NebulaGrid

**Infrastructure Control Plane** — Rust + React

Manage servers, containers, VMs, Kubernetes, storage, backups, policies, certificates, configuration, and plugins from a single API and dashboard.

[![Rust CI](https://github.com/BTF-Kabir-2020/NebulaGrid/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/BTF-Kabir-2020/NebulaGrid/actions/workflows/rust-ci.yml)
[![Frontend CI](https://github.com/BTF-Kabir-2020/NebulaGrid/actions/workflows/frontend-ci.yml/badge.svg)](https://github.com/BTF-Kabir-2020/NebulaGrid/actions/workflows/frontend-ci.yml)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

**Wiki:** [Architecture & guides](https://github.com/BTF-Kabir-2020/NebulaGrid/wiki) · **Docs:** [setup](docs/setup.md) · [API](docs/api.md)

> **Status: early development.** The gateway currently serves an **in-memory** control plane for demos and development. See [Status & roadmap](#status--roadmap) for what is implemented, experimental, and planned.

---

## Quick start

```bash
docker compose -f labs/docker-compose.lab.yml --profile full up -d --build
# Dashboard: http://localhost:5173  (login admin / admin123)
# API:       http://localhost:8080
```

Manual:

```bash
# Gateway
cd control-plane && cargo run -p nebula-gateway

# Dashboard
cd dashboard && npm install && npm run dev
```

---

## Architecture

```
                         ┌─────────────────────────┐
                         │   React Dashboard        │
                         │   Vite · TypeScript      │
                         │   :5173 (Compose)        │
                         └───────────┬─────────────┘
                                     │ HTTP / JWT
                                     ▼
┌────────────────────────────────────────────────────────────┐
│                 Gateway (Rust · Axum)                       │
│  Auth · RBAC · REST /api/* · WebSocket hub · seed state     │
│  :8080                                                      │
└───────┬──────────────┬──────────────┬───────────────────────┘
        │              │              │
        ▼              ▼              ▼
   Agents / sims   Compose infra   Service crates
   (metrics POST)  Postgres/Redis  (node, docker, vm,
                   NATS (optional)  k8s, storage, …)
```

### Repository layout

| Path | Role |
|------|------|
| `control-plane/gateway` | Main API gateway (Axum) |
| `control-plane/*-service` | Domain service crates (NATS-oriented scaffolds) |
| `dashboard` | React control UI |
| `agent` | Rust host agent (sysinfo → gateway) |
| `labs` | Docker Compose stack + Python agent simulators |
| `deployment` | Dockerfiles, Compose, Helm, Terraform |
| `proto` | gRPC / Protobuf definitions |
| `docs` | Setup, API, architecture |

### Managers (UI ↔ API)

| Area | UI | API |
|------|----|-----|
| Nodes | `/servers` | `/api/nodes` |
| Containers | `/containers` | `/api/containers` |
| VMs | `/vms` | `/api/vms` |
| Kubernetes | `/kubernetes` | `/api/k8s/*` |
| Networks | `/networks` | `/api/networks` |
| Storage | `/storage` | `/api/storage/*` |
| Backups | `/backups` | `/api/backups` |
| Monitoring / Logs | `/monitoring`, `/logs` | `/api/monitoring/*`, container logs |
| Inventory / Policies | `/inventory`, `/policies` | `/api/inventory`, `/api/policies` |
| Certificates / Config | `/certificates`, `/config` | `/api/certificates`, `/api/config` |
| Plugins / Jobs / Alerts | `/plugins`, `/jobs`, `/alerts` | matching `/api/*` |

Default credentials for local stacks: `admin` / `admin123`.

> Persistence (Postgres/Redis/NATS) and edge TLS are **planned** — see [Status & roadmap](#status--roadmap).

---

## Status & roadmap

Honest maturity labels for this repository. **Implemented** = works in the current code path. **Experimental** = API/UI exist and are exercised against the in-memory control plane, but the backing provider is not production-wired yet. **Planned** = designed, not yet built.

| Area | Status |
|------|--------|
| Axum gateway — REST `/api/*`, JWT auth, RBAC, WebSocket hub | Implemented |
| React dashboard (Vite · TypeScript) with routing to all managers | Implemented |
| Docker Compose local stack + host agent metrics | Implemented |
| CI for Rust and frontend | Implemented |
| Domain service crates (node, docker, vm, k8s, storage, …) | Experimental (scaffolds) |
| Managers surfaced in UI/API (K8s, VMs, storage, backups, policies, certificates, plugins) | Experimental (in-memory backing) |
| Postgres / Redis / NATS persistence | Planned |
| Edge TLS and long-lived deployment hardening | Planned |

Contributions toward the **Planned** items are especially welcome.

---

## Documentation

| Resource | Link |
|----------|------|
| Wiki Home | [Wiki](https://github.com/BTF-Kabir-2020/NebulaGrid/wiki) |
| Setup | [docs/setup.md](docs/setup.md) |
| API reference | [docs/api.md](docs/api.md) |
| Architecture | [docs/architecture.md](docs/architecture.md) |
| Compose profiles | [labs/README.md](labs/README.md) |
| Contributing | [CONTRIBUTING.md](CONTRIBUTING.md) |

## License

MIT
