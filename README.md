# NebulaGrid

**Infrastructure Control Plane (labs MVP)** — Rust + React

Unified management UI/API for servers, containers, VMs, Kubernetes, storage, backups, policies, certificates, config, and plugins.

[![Status](https://img.shields.io/badge/status-labs%20MVP-blue)](README.md)
[![Gateway Tests](https://img.shields.io/badge/tests-7/7-green)](control-plane/gateway/tests/)
[![Build](https://img.shields.io/badge/build-passing-green)](dashboard/)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

---

## Quick Start (Docker Compose — Full Stack)

```bash
docker compose -f labs/docker-compose.lab.yml --profile full up -d --build

# Dashboard (Windows often reserves :3000 → mapped to :5173)
open http://localhost:5173

# Default login: admin / admin123
```

### Managers available in labs

| Manager | UI | API |
|---------|----|-----|
| Node | `/servers` | `/api/nodes` |
| Container | `/containers` | `/api/containers` |
| VM (+ snapshot/backup actions) | `/vms` | `/api/vms` |
| Storage | `/storage` | `/api/storage/*` |
| Backup | `/backups` | `/api/backups` |
| Metrics | `/monitoring` | `/api/monitoring/*`, `/api/nodes/:id/metrics` |
| Inventory | `/inventory` | `/api/inventory` |
| Network | `/networks` | `/api/networks` |
| Policy | `/policies` | `/api/policies` |
| Certificate | `/certificates` | `/api/certificates` |
| Configuration | `/config` | `/api/config` |
| Plugin | `/plugins` | `/api/plugins` |

### Or run manually

```bash
# Terminal 1 — Gateway
cd control-plane
$env:CARGO_TARGET_DIR = "..\.cargo-target"   # optional on Windows
cargo run -p nebula-gateway

# Terminal 2 — Dashboard
cd dashboard && npm install && npm run dev
```

---

## Architecture (labs)

```
┌──────────────┐      ┌──────────────────────┐
│  Dashboard   │      │  Lab Agent Simulators │
│  (React+TS)  │      │  (Python × 3)         │
│  Port 5173*  │      │  —                   │
└──────┬───────┘      └──────────┬───────────┘
       │ HTTP                     │ metrics POST
       ▼                          ▼
┌─────────────────────────────────────────────┐
│              Gateway (Rust Axum)             │
│  Port 8080  ·  REST + JWT + in-memory seed  │
└─────────────────────────────────────────────┘
* Host 3000/4222 are often Hyper-V reserved on Windows → lab maps 5173 and 14222.
```

Labs use **in-memory seed data** (restart clears state). Optional Postgres/Redis/NATS run under `--profile infra|full` for later production wiring — the gateway does not persist to them yet.

> This repository is a **labs / MVP control-plane demo**, not a production infrastructure product.

## Docs

| Doc | Purpose |
|-----|---------|
| [docs/setup.md](docs/setup.md) | Dev setup |
| [docs/api.md](docs/api.md) | API reference |
| [labs/README.md](labs/README.md) | Compose profiles |
| [docs/architecture.md](docs/architecture.md) | Target architecture |

## License

MIT
