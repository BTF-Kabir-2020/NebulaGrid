#!/usr/bin/env bash
set -euo pipefail

API_URL=${API_URL:-"http://localhost:3001/api"}
TOKEN=${TOKEN:-"dev-token"}

echo "Seeding NebulaGrid with sample data..."
echo ""

curl -s -X POST "$API_URL/nodes" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"hostname":"web-01","ip_address":"10.0.1.10","os_name":"Ubuntu 24.04","cpu_cores":4,"ram_total_bytes":8589934592,"disk_total_bytes":107374182400}' \
  > /dev/null && echo "Node web-01 created"

curl -s -X POST "$API_URL/nodes" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"hostname":"db-01","ip_address":"10.0.1.20","os_name":"Ubuntu 24.04","cpu_cores":8,"ram_total_bytes":17179869184,"disk_total_bytes":536870912000}' \
  > /dev/null && echo "Node db-01 created"

curl -s -X POST "$API_URL/containers" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"container_id":"abc123","name":"nginx-proxy","image":"nginx:1.27","status":"running","node_id":"web-01","ports":[{"private_port":80,"public_port":8080,"ip":"0.0.0.0"}]}' \
  > /dev/null && echo "Container nginx-proxy created"

curl -s -X POST "$API_URL/containers" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"container_id":"def456","name":"api-server","image":"node:22-alpine","status":"running","node_id":"web-01","ports":[{"private_port":3000,"public_port":3000,"ip":"0.0.0.0"}]}' \
  > /dev/null && echo "Container api-server created"

curl -s -X POST "$API_URL/vms" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name":"dev-vm-1","status":"running","os_type":"Linux","cpu_cores":2,"ram_mb":4096,"disk_gb":50,"node_id":"web-01","ip_address":"10.0.2.10"}' \
  > /dev/null && echo "VM dev-vm-1 created"

echo ""
echo "Seed data created successfully!"
