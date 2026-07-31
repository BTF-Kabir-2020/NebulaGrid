# NebulaGrid Health Check
# Run this to verify all services are running correctly.

Write-Host "╔══════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║        NebulaGrid Health Check              ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$passed = 0
$failed = 0

function Check-Service {
    param($Name, $Url, $Expected = 200)
    try {
        $resp = Invoke-WebRequest -Uri $Url -Method Get -TimeoutSec 5 -UseBasicParsing
        if ($resp.StatusCode -eq $Expected) {
            Write-Host "  [✓] $Name" -ForegroundColor Green
            return $true
        } else {
            Write-Host "  [✗] $Name (HTTP $($resp.StatusCode))" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "  [✗] $Name (unreachable)" -ForegroundColor Red
        return $false
    }
}

Write-Host "─── Services ───────────────────────────────" -ForegroundColor Yellow

if (Check-Service "Gateway API" "http://localhost:8080/api/health") { $passed++ } else { $failed++ }
if (Check-Service "Gateway Ready" "http://localhost:8080/api/health/ready") { $passed++ } else { $failed++ }
if (Check-Service "Dashboard" "http://localhost:3000") { $passed++ } else { $failed++ }

Write-Host ""
Write-Host "─── Docker ──────────────────────────────────" -ForegroundColor Yellow

try {
    $ps = docker ps --format "{{.Names}} {{.Status}}"
    foreach ($line in $ps) {
        Write-Host "  $line"
    }
    $passed++
} catch {
    Write-Host "  [✗] Docker not available" -ForegroundColor Red
    $failed++
}

Write-Host ""
Write-Host "─── API Test ────────────────────────────────" -ForegroundColor Yellow

try {
    $login = Invoke-WebRequest -Uri "http://localhost:8080/api/auth/login" -Method Post `
        -ContentType "application/json" `
        -Body '{"username":"admin","password":"admin123"}' `
        -TimeoutSec 5 -UseBasicParsing
    if ($login.StatusCode -eq 200) {
        Write-Host "  [✓] Login works (admin/admin123)" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "  [✗] Login failed" -ForegroundColor Red
        $failed++
    }
} catch {
    Write-Host "  [✗] Login (unreachable)" -ForegroundColor Red
    $failed++
}

Write-Host ""
Write-Host "══════════════════════════════════════════════" -ForegroundColor Cyan
if ($failed -eq 0) {
    Write-Host "  ALL CHECKS PASSED ($passed/$($passed+$failed))" -ForegroundColor Green
} else {
    Write-Host "  $passed passed, $failed FAILED" -ForegroundColor Red
}
Write-Host "══════════════════════════════════════════════" -ForegroundColor Cyan
