# NebulaGrid Development Setup Script (Windows)

Write-Host "=== NebulaGrid Development Setup ===" -ForegroundColor Cyan

# 1. Start infrastructure
Write-Host "[1/5] Starting infrastructure services..." -ForegroundColor Yellow
docker compose -f labs/docker-compose.lab.yml up postgres redis nats -d

# 2. Install npm dependencies
Write-Host "[2/5] Installing Dashboard dependencies..." -ForegroundColor Yellow
Set-Location dashboard
npm install
Set-Location ..

# 3. Create .env file if not exists
if (-not (Test-Path .env)) {
    Write-Host "[3/5] Creating .env from .env.example..." -ForegroundColor Yellow
    Copy-Item .env.example .env
}

# 4. Build Rust workspace
Write-Host "[4/5] Building Rust workspace..." -ForegroundColor Yellow
Set-Location control-plane
cargo build
Set-Location ..

# 5. Show status
Write-Host "[5/5] Setup complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Start Gateway:   cd control-plane && cargo run --bin nebula-gateway"
Write-Host "  2. Start Dashboard: cd dashboard && npm run dev"
Write-Host "  3. Start Agent:     cd agent && cargo run -- --server http://localhost:8080 --token dev-agent-token"
Write-Host "  4. Open browser:    http://localhost:5173"
