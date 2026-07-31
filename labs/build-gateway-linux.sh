#!/bin/bash
# Optional local helper: compile nebula-gateway (linux/amd64).
# Prefer: docker compose -f labs/docker-compose.lab.yml --profile full build gateway
# Override mirror: DEBIAN_MIRROR=http://deb.debian.org bash /build.sh
set -euo pipefail
MIRROR="${DEBIAN_MIRROR:-http://mirrors.aliyun.com}"
echo "== apt mirrors -> ${MIRROR} =="
if [ -f /etc/apt/sources.list.d/debian.sources ]; then
  sed -i "s|https\?://deb.debian.org/debian-security|${MIRROR}/debian-security|g" /etc/apt/sources.list.d/debian.sources
  sed -i "s|https\?://deb.debian.org/debian|${MIRROR}/debian|g" /etc/apt/sources.list.d/debian.sources
fi
apt-get update
apt-get install -y --no-install-recommends pkg-config libssl-dev protobuf-compiler ca-certificates
echo "== cargo build =="
cargo build --release --manifest-path control-plane/Cargo.toml --bin nebula-gateway
ls -lh control-plane/target/release/nebula-gateway
echo "== DONE =="
