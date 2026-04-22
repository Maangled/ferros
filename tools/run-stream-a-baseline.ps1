# Stream A Baseline Lock
# ======================
# Runs deterministic generators and verifies generated artifacts are drift-free.
#
# Usage:
#   powershell -ExecutionPolicy Bypass -File tools\run-stream-a-baseline.ps1
#
# Exit code:
#   0 -> baseline lock passed (no drift)
#   1 -> drift detected or prerequisite failure

param(
  [string]$RepoRoot = $PSScriptRoot + "\.."
)

$ErrorActionPreference = 'Stop'

function Assert-Command {
  param([string]$CommandName)

  $cmd = Get-Command -Name $CommandName -ErrorAction SilentlyContinue
  if (-not $cmd) {
    Write-Error "Required command not found: $CommandName"
    exit 1
  }
}

function Assert-Path {
  param([string]$Path, [string]$Label)

  if (-not (Test-Path $Path)) {
    Write-Error "$Label not found: $Path"
    exit 1
  }
}

$repoRootPath = (Resolve-Path $RepoRoot).Path
Push-Location $repoRootPath
try {
  Assert-Command -CommandName "git"
  Assert-Command -CommandName "powershell"

  $harnessGenerator = Join-Path $repoRootPath "tools\generate-harness-constants.ps1"
  $coreGenerator = Join-Path $repoRootPath "tools\generate-ferros-core.ps1"
  Assert-Path -Path $harnessGenerator -Label "Harness generator"
  Assert-Path -Path $coreGenerator -Label "Core generator"

  Write-Host "[1/4] Running harness constants generator..." -ForegroundColor Cyan
  powershell -ExecutionPolicy Bypass -File $harnessGenerator -RepoRoot $repoRootPath

  Write-Host "[2/4] Running ferros-core generator..." -ForegroundColor Cyan
  powershell -ExecutionPolicy Bypass -File $coreGenerator -RepoRoot $repoRootPath

  Write-Host "[3/4] Checking drift on generated files..." -ForegroundColor Cyan
  $driftFiles = @(
    "harnesses/_constants.js",
    "docs/assets/_core/ferros-core.js"
  )

  $driftDetected = $false
  foreach ($file in $driftFiles) {
    $status = git status --porcelain -- $file
    if ($status) {
      if (-not $driftDetected) {
        Write-Host "" 
        Write-Host "Drift detected in generated artifacts:" -ForegroundColor Yellow
      }
      $driftDetected = $true
      Write-Host "  - $file" -ForegroundColor Yellow
    }
  }

  if ($driftDetected) {
    Write-Host "" 
    Write-Host "[4/4] Baseline lock failed." -ForegroundColor Red
    Write-Host "Run git diff on listed files and commit regenerated output if intentional." -ForegroundColor Yellow
    exit 1
  }

  Write-Host "[4/4] Baseline lock passed (no drift)." -ForegroundColor Green
  exit 0
}
finally {
  Pop-Location
}
