#!/usr/bin/env bash
set -euo pipefail

echo "=== NebulaGrid Test Agent Launcher ==="
echo "This script launches test agent instances for local development."
echo ""

COUNT=${1:-2}
SERVER_URL=${SERVER_URL:-"http://localhost:3001"}
TOKEN=${TOKEN:-"dev-token"}

for i in $(seq 1 $COUNT); do
  AGENT_NAME="test-agent-$i"
  AGENT_PORT=$((9000 + i))

  echo "Starting $AGENT_NAME on port $AGENT_PORT..."

  cd agent
  cargo run --release -- \
    --server "$SERVER_URL" \
    --token "$TOKEN" \
    --name "$AGENT_NAME" \
    --grpc-port "$AGENT_PORT" \
    --interval 5 &

  cd ..
done

echo ""
echo "$COUNT agent(s) started. Press Ctrl+C to stop all."
wait
