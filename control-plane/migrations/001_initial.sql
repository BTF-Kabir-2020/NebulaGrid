CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(100) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    roles TEXT[] NOT NULL DEFAULT '{viewer}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE nodes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    hostname VARCHAR(255) NOT NULL,
    ip_address VARCHAR(45) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'offline',
    os_name VARCHAR(100),
    os_version VARCHAR(100),
    cpu_cores INT DEFAULT 0,
    cpu_model VARCHAR(255),
    ram_total_bytes BIGINT DEFAULT 0,
    disk_total_bytes BIGINT DEFAULT 0,
    last_seen_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE node_metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    node_id UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    cpu_percent DOUBLE PRECISION NOT NULL DEFAULT 0,
    ram_percent DOUBLE PRECISION NOT NULL DEFAULT 0,
    ram_used_bytes BIGINT NOT NULL DEFAULT 0,
    ram_total_bytes BIGINT NOT NULL DEFAULT 0,
    disk_percent DOUBLE PRECISION NOT NULL DEFAULT 0,
    disk_used_bytes BIGINT NOT NULL DEFAULT 0,
    disk_total_bytes BIGINT NOT NULL DEFAULT 0,
    net_rx_bytes BIGINT NOT NULL DEFAULT 0,
    net_tx_bytes BIGINT NOT NULL DEFAULT 0,
    process_count INT NOT NULL DEFAULT 0,
    collected_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE containers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    container_id VARCHAR(64) NOT NULL,
    name VARCHAR(255) NOT NULL,
    image VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    node_id UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    ports JSONB DEFAULT '[]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE vms (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'stopped',
    os_type VARCHAR(100),
    cpu_cores INT NOT NULL DEFAULT 1,
    ram_mb INT NOT NULL DEFAULT 1024,
    disk_gb INT NOT NULL DEFAULT 10,
    node_id UUID REFERENCES nodes(id) ON DELETE SET NULL,
    ip_address VARCHAR(45)
);

CREATE TABLE k8s_resources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    resource_type VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    namespace VARCHAR(100) NOT NULL DEFAULT 'default',
    status VARCHAR(50),
    details JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE alerts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    node_id UUID REFERENCES nodes(id) ON DELETE CASCADE,
    alert_type VARCHAR(100) NOT NULL,
    severity VARCHAR(20) NOT NULL DEFAULT 'warning',
    message TEXT NOT NULL,
    acknowledged BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    acknowledged_at TIMESTAMPTZ
);

CREATE TABLE jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    job_type VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    config JSONB DEFAULT '{}',
    result JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_node_metrics_node_id ON node_metrics(node_id);
CREATE INDEX idx_node_metrics_collected_at ON node_metrics(collected_at);
CREATE INDEX idx_containers_node_id ON containers(node_id);
CREATE INDEX idx_alerts_node_id ON alerts(node_id);
CREATE INDEX idx_alerts_severity ON alerts(severity);
