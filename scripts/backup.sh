#!/usr/bin/env bash
set -euo pipefail

# NebulaGrid Backup Script
# Creates a timestamped tarball excluding build artifacts

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BACKUP_DIR="${BACKUP_DIR:-${PROJECT_DIR}/../backups}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="${BACKUP_DIR}/nebulagrid-${TIMESTAMP}.tar.gz"

mkdir -p "$BACKUP_DIR"

echo "📦 Creating backup: ${BACKUP_FILE}"
echo "   (excluding target/, node_modules/, .cargo-target/, dist/)"

tar -czf "$BACKUP_FILE" \
  --exclude='target' \
  --exclude='node_modules' \
  --exclude='.cargo-target' \
  --exclude='dist' \
  --exclude='*.tar.gz' \
  -C "$PROJECT_DIR" .

echo "✅ Backup created: $(du -h "$BACKUP_FILE" | cut -f1)"
echo "   Location: ${BACKUP_FILE}"
