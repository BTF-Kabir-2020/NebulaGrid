# NebulaGrid Backup Script (Windows)
# Creates a timestamped archive excluding build artifacts

$ProjectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$BackupDir = Join-Path (Split-Path -Parent $ProjectRoot) "backups"
$Timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$BackupFile = Join-Path $BackupDir "nebulagrid-${Timestamp}.7z"

if (-not (Test-Path $BackupDir)) {
    New-Item -ItemType Directory -Path $BackupDir -Force | Out-Null
}

Write-Host "Creating backup: $BackupFile"
Write-Host "  (excluding target/, node_modules/, .cargo-target/, dist/)"

if (Get-Command 7z -ErrorAction SilentlyContinue) {
    7z a -t7z -mx=5 -bsp1 $BackupFile `
        "$ProjectRoot\*" `
        -xr!'target\' `
        -xr!'node_modules\' `
        -xr!'.cargo-target\' `
        -xr!'dist\'
} elseif (Get-Command tar -ErrorAction SilentlyContinue) {
    tar -czf "$BackupDir\nebulagrid-${Timestamp}.tar.gz" `
        --exclude='target' `
        --exclude='node_modules' `
        --exclude='.cargo-target' `
        --exclude='dist' `
        -C $ProjectRoot .
} else {
    Write-Error "No archiver found. Install 7-Zip or use WSL tar."
    exit 1
}

Write-Host "Backup created: $BackupFile"
