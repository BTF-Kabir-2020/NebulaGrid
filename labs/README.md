# NebulaGrid Lab Environment

Lab/testing environment using Docker Compose with profiles.

## Profiles

| Profile   | Services                                       | Ports (host)                    |
|-----------|------------------------------------------------|---------------------------------|
| `core`    | Gateway + Dashboard                            | 8080, **5173**                  |
| `infra`   | PostgreSQL + Redis + NATS                      | 5432, 6379, **14222**, 8222     |
| `agents`  | 3× Python Agent Simulators (lab-agent-1..3)    | —                               |
| `full`    | Core + Infra + Agents (everything)             | All of the above                |

### Quick Start — Core Only (in-memory, no infra)

```bash
docker compose -f labs/docker-compose.lab.yml --profile core up -d
```

Gateway starts with seed data (3 nodes, 3 containers, 3 VMs). No database required.

### Full Stack (with agent simulators)

```bash
docker compose -f labs/docker-compose.lab.yml --profile full up --build -d
```

This starts: Gateway, Dashboard, Postgres, Redis, NATS, and 3 Python agent simulators that automatically register as nodes and send CPU/RAM/Disk/Network metrics every 15 seconds.

### Agents Only (to test against a running gateway)

```bash
docker compose -f labs/docker-compose.lab.yml --profile agents up -d
```

Each agent:
1. Logs in via `POST /api/auth/login` with `admin/admin123`
2. Registers via `POST /api/nodes/register` as a new node
3. Sends metrics every 15s via `POST /api/nodes/{id}/metrics`

## Health Checks

Each service has built-in healthcheck with automatic recovery.

```bash
# Manual health check
curl http://localhost:8080/api/health

# Run the healthcheck script
.\scripts\healthcheck.ps1
```

## Service Ports

| Service       | Port (host) | Notes                                      |
|---------------|-------------|--------------------------------------------|
| Gateway       | 8080        | REST API                                   |
| Dashboard     | **5173**    | Vite (container listens on 3000)           |
| Postgres      | 5432        | Primary database                           |
| Redis         | 6379        | Cache + session                            |
| NATS          | **14222**   | Message bus (container 4222; Hyper-V safe) |
| NATS HTTP     | 8222        | NATS monitoring                            |
| Agent Sims    | —           | No external ports (outbound)               |

Open http://localhost:5173 — login `admin` / `admin123`.

## API Testing

```bash
# Health
curl http://localhost:8080/api/health

# Login
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}'

# Check nodes (after agents have registered)
curl http://localhost:8080/api/nodes \
  -H "Authorization: Bearer $(TOKEN)"
```

## Cleanup

```bash
docker compose -f labs/docker-compose.lab.yml down -v

# Or just agents
docker compose -f labs/docker-compose.lab.yml --profile agents down
```
