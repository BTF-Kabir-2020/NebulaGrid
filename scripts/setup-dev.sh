#!/usr/bin/env bash
set -euo pipefail

echo "=== NebulaGrid Dev Setup ==="

command -v rustc >/dev/null 2>&1 || { echo "Install Rust: https://rustup.rs"; exit 1; }
command -v node >/dev/null 2>&1 || { echo "Install Node.js: https://nodejs.org"; exit 1; }

if [ ! -f .env ]; then
  cp .env.example .env
  echo "Created .env from .env.example"
fi

echo "Building agent..."
cd agent
rustup target add wasm32-unknown-unknown 2>/dev/null || true
cargo build --release
cd ..

echo "Building control-plane..."
cd control-plane
cargo build --release
cd ..

echo "Installing dashboard dependencies..."
cd dashboard
npm install
cd ..

echo "=== Setup Complete ==="
echo "Run './scripts/setup-dev.sh' to start development servers."
