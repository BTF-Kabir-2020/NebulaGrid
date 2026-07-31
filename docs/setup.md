# Development Setup Guide

## Prerequisites

- **Rust**: 1.75+ (`rustup install stable`)
- **Node.js**: 20+ (`nvm install 20`)
- **Docker**: 24+ with Compose plugin (optional)
- **Git**: Latest

## Docker Compose Setup (Recommended)

The fastest way to see everything working — gateway, dashboard, and 3 Python agent simulators:

```bash
# Full stack with agent simulators
docker compose -f labs/docker-compose.lab.yml --profile full up --build -d
```

Profiles:
| Command | What starts |
|---|---|
| `--profile core` | Gateway + Dashboard (in-memory, no infra) |
| `--profile agents` | 3 Python agent simulators only |
| `--profile full` | Core + Infra + Agents (everything) |

Open `http://localhost:5173` (Compose maps host 5173 → container 3000; Windows often reserves 3000) — default login: `admin` / `admin123`.

The agent simulators automatically register as new nodes and send CPU/RAM/Disk/Network metrics every 15 seconds.

### Troubleshooting

If the dashboard shows a white screen, restart the container:

```bash
docker compose -f labs/docker-compose.lab.yml restart dashboard
```

## Manual Setup

### 1. Infrastructure (Optional)

NebulaGrid runs with in-memory storage by default. To use PostgreSQL/Redis/NATS:

```bash
docker compose -f labs/docker-compose.lab.yml --profile infra up -d
```

### 2. Environment Configuration

Create `.env` in the project root:

```bash
cp .env.example .env
```

Edit `.env` with your settings:
```bash
# At minimum, change these:
JWT_SECRET=your-random-secret-here
AGENT_TOKEN=dev-token
```

### 3. Start the Gateway

```bash
cd control-plane
cargo run
```

The API will be available at `http://localhost:8080`.

Verify:
```bash
curl http://localhost:8080/api/health
# {"status":"ok","version":"0.1.0","uptime_seconds":42}
```

The gateway starts with in-memory seed data — 3 nodes, 3 containers, 3 VMs, K8s resources, and sample alerts. Labs expose 50+ REST routes immediately.

### 4. Start the Dashboard

In a new terminal:

```bash
cd dashboard
npm install
npm run dev
```

Open `http://localhost:5173` when using Compose (or `http://localhost:3000` for native `npm run dev`).

### 5. Start a Test Agent

**Option A — Rust Agent (requires Rust toolchain):**

```bash
cd agent
cargo run -- --server http://localhost:8080 --token dev-token
```

**Option B — Python Agent Simulator (no Rust needed):**

```bash
cd labs/agent-simulator
pip install requests
python3 simulator.py
```

The agent will register with the gateway and begin sending metrics every 15 seconds.

### Default Credentials

| Username   | Password      | Role     |
|------------|---------------|----------|
| admin      | admin123      | admin    |
| operator   | operator123   | operator |

## Running Tests

### Rust Tests

```bash
cd control-plane

# All tests
cargo test --all

# Specific package
cargo test -p nebula-gateway

# Specific test
cargo test test_login_success
```

### Frontend Tests

```bash
cd dashboard

# Run once
npm run test -- --run

# Watch mode
npm run test
```

### Linting

```bash
# Rust
cd control-plane
cargo clippy --all -- -D warnings
cargo fmt --all --check

# Frontend
cd dashboard
npm run lint
npm run typecheck
```

## IDE Setup

### VS Code Extensions
- **rust-analyzer** — Rust language support
- **Tailwind CSS IntelliSense** — Tailwind class completion
- **ESLint** — JavaScript/TypeScript linting
- **Prettier** — Code formatting
- **Docker** — Docker Compose support

### Settings (`.vscode/settings.json`)
```json
{
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": "explicit"
  }
}
```
