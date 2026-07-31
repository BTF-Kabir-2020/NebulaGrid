#!/usr/bin/env bash
set -euo pipefail

# NebulaGrid Linux Install Script
# Installs dependencies, builds binaries, and sets up the stack

# Colors
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

# --- Detect OS ---
OS=""
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
fi

# --- Install system deps ---
install_deps() {
    info "Installing system dependencies..."
    case "$OS" in
        ubuntu|debian)
            sudo apt-get update -qq
            sudo apt-get install -y -qq curl wget git build-essential pkg-config libssl-dev postgresql-client redis-tools
            ;;
        centos|rhel|fedora|rocky|almalinux)
            sudo dnf install -y curl wget git gcc gcc-c++ pkg-config openssl-devel postgresql redis
            ;;
        *)
            warn "Unsupported OS: $OS. Please install manually: curl, git, build tools, libssl-dev"
            ;;
    esac
}

# --- Install Rust ---
install_rust() {
    if ! command -v rustc &>/dev/null; then
        info "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        info "Rust already installed: $(rustc --version)"
    fi
}

# --- Install Docker ---
install_docker() {
    if ! command -v docker &>/dev/null; then
        info "Installing Docker..."
        curl -fsSL https://get.docker.com | sh
        sudo usermod -aG docker "$USER"
        warn "Log out and back in for Docker group to take effect, or run: newgrp docker"
    else
        info "Docker already installed: $(docker --version)"
    fi
}

# --- Install Node.js ---
install_node() {
    if ! command -v node &>/dev/null; then
        info "Installing Node.js 20 LTS..."
        curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
        sudo apt-get install -y nodejs
    else
        info "Node.js already installed: $(node --version)"
    fi
}

# --- Build Rust binaries ---
build_binaries() {
    info "Building NebulaGrid binaries (this may take a while)..."
    cd "$PROJECT_DIR/control-plane"
    cargo build --release --bin nebula-gateway
    cd "$PROJECT_DIR/agent"
    cargo build --release --bin nebula-agent
    info "Binaries built:"
    ls -lh "$PROJECT_DIR/control-plane/target/release/nebula-gateway"
    ls -lh "$PROJECT_DIR/agent/target/release/nebula-agent"
}

# --- Setup environment ---
setup_env() {
    if [ ! -f "$PROJECT_DIR/.env" ]; then
        info "Creating .env from .env.example..."
        cp "$PROJECT_DIR/.env.example" "$PROJECT_DIR/.env"
        warn "Edit .env with your configuration before running."
    else
        info ".env already exists"
    fi
}

# --- Main ---
main() {
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

    echo "============================================"
    echo "  NebulaGrid - Linux Installation"
    echo "============================================"

    install_deps
    install_rust
    install_docker
    install_node
    build_binaries
    setup_env

    echo ""
    echo "============================================"
    info "Installation complete!"
    echo ""
    echo "Next steps:"
    echo "  1. Start infrastructure:"
    echo "     docker compose -f labs/docker-compose.lab.yml up -d postgres redis nats"
    echo ""
    echo "  2. Run the gateway:"
    echo "     ./control-plane/target/release/nebula-gateway"
    echo ""
    echo "  3. Run the dashboard:"
    echo "     cd dashboard && npm install && npm run dev"
    echo ""
    echo "  4. Or start everything with Docker:"
    echo "     docker compose -f labs/docker-compose.lab.yml up --build"
    echo "============================================"
}

main "$@"
