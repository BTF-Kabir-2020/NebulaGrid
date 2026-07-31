# Agent Installation Guide

The NebulaGrid agent runs on each server you want to manage. It collects system metrics and communicates with the control plane via HTTP and gRPC.

## Supported Platforms

| Platform | Support | Status |
|----------|---------|--------|
| Ubuntu 20.04+ | Full | ✅ |
| Debian 11+ | Full | ✅ |
| CentOS/RHEL 8+ | Full | ✅ |
| Fedora 36+ | Full | ✅ |
| Windows Server 2019+ | Limited | 🔄 In Progress |
| macOS 13+ | Limited | 🔄 In Progress |
| ARM64 (Raspberry Pi) | Full | ✅ |

## Quick Install (Linux)

### Build from source

```bash
# Build the agent binary
cd agent
cargo build --release
# Copy to target server
scp target/release/nebula-agent user@server:/usr/local/bin/
```

### Manual install

```bash
# 1. Build the binary
cd agent
cargo build --release
sudo cp target/release/nebula-agent /usr/local/bin/

# 2. Create config directory
sudo mkdir -p /etc/nebula-agent

# 3. Create config file
sudo tee /etc/nebula-agent/.env << EOF
AGENT_SERVER_URL=http://your-gateway:8080
AGENT_TOKEN=YOUR_AGENT_TOKEN
EOF

# 4. Create systemd service
sudo tee /etc/systemd/system/nebula-agent.service << UNIT
[Unit]
Description=NebulaGrid Agent
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=/usr/local/bin/nebula-agent --server http://your-gateway:8080 --token YOUR_AGENT_TOKEN
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
UNIT

# 5. Start the service
sudo systemctl daemon-reload
sudo systemctl enable nebula-agent
sudo systemctl start nebula-agent
```

## Configuration

### CLI Arguments

```
nebula-agent 0.1.0

USAGE:
    nebula-agent [OPTIONS] --server <URL> --token <TOKEN>

OPTIONS:
    -s, --server <URL>          Control Plane URL [env: AGENT_SERVER_URL]
    -t, --token <TOKEN>         Agent authentication token [env: AGENT_TOKEN]
    -n, --name <HOSTNAME>       Node hostname (default: OS hostname) [env: AGENT_HOSTNAME]
    -i, --interval <SECONDS>    Metrics reporting interval [default: 10] [env: METRICS_INTERVAL_SECONDS]
    -p, --grpc-port <PORT>      Local gRPC port [default: 9000]
        --no-docker             Disable Docker capability
    -v, --verbose               Enable verbose logging
    -h, --help                  Print help
```

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `AGENT_SERVER_URL` | Yes | - | Control Plane HTTP URL |
| `AGENT_TOKEN` | Yes | - | Authentication token |
| `AGENT_HOSTNAME` | No | OS hostname | Display name for this node |
| `METRICS_INTERVAL_SECONDS` | No | 10 | How often to report metrics |

## Agent Capabilities

The agent auto-detects available capabilities on startup:

| Capability | Detection | Description |
|-----------|-----------|-------------|
| `docker` | `/var/run/docker.sock` exists | Manage Docker containers |
| `systemd` | `systemctl` command exists | Systemd service management |

Pass `--no-docker` to explicitly disable Docker capability.

## Running as Non-Root

The agent does **not** require root for basic metrics collection. For Docker management, add the user to the `docker` group:

```bash
sudo usermod -aG docker nebula
```

## Monitoring

Verify the agent is running:

```bash
# Check service status
systemctl status nebula-agent

# View logs
journalctl -u nebula-agent -f
```

Expected healthy output:
```
2024-06-15T10:30:00Z INFO  NebulaGrid Agent starting — server: http://localhost:8080, interval: 10s, grpc_port: 9000
2024-06-15T10:30:01Z INFO  Registered as node: d290f1ee-6c54-4b01-90e6-d701748f0851
2024-06-15T10:30:11Z INFO  Agent — CPU: 45.2% | RAM: 62.1%
```

## Troubleshooting

| Issue | Cause | Solution |
|-------|-------|----------|
| Agent won't start | Wrong token | Check `AGENT_TOKEN` |
| Connection refused | Gateway not running | Check gateway status |
| Docker not detected | Socket permission | Add user to docker group |
| High CPU usage | Too frequent metrics | Increase `--interval` value |
| Agent offline in dashboard | Network issue | Check `AGENT_SERVER_URL` |
