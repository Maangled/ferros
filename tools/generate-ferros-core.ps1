# Generate FERROS Core Bundle
# ============================
# Reads docs/assets/_core/templates.json, validates against schemas/template.schema.json,
# and embeds the template array into docs/assets/_core/ferros-core.js at the placeholder line.
#
# This script is the CANONICAL build step for ferros-core.js template data.
# Run whenever templates.json changes.
#
# Usage:
#   cd <repo-root>
#   .\tools\generate-ferros-core.ps1
#
# Prerequisite: Run AFTER any template edits. Deterministic output.

param(
  [string]$RepoRoot = $PSScriptRoot + "\.."
)

$ErrorActionPreference = 'Stop'

function Normalize-JsonText {
  param([string]$text)

  if ($text.Length -gt 0 -and $text[0] -eq [char]0xFEFF) {
    $text = $text.Substring(1)
  }

  $normalized = $text -replace "`r`n", "`n"
  $normalized = $normalized -replace "`r", "`n"
  $normalized = $normalized.TrimEnd("`r", "`n")
  return $normalized -replace "`n", "`r`n"
}

$coreJs       = Join-Path $RepoRoot "docs\assets\_core\ferros-core.js"
$templatesJson = Join-Path $RepoRoot "docs\assets\_core\templates.json"
$templateSchema = Join-Path $RepoRoot "schemas\template.schema.json"

# ── Validate paths ────────────────────────────────────────────────────────────
if (-not (Test-Path $coreJs)) { Write-Error "ferros-core.js not found: $coreJs"; exit 1 }
if (-not (Test-Path $templatesJson)) { Write-Error "templates.json not found: $templatesJson"; exit 1 }
if (-not (Test-Path $templateSchema)) { Write-Error "template.schema.json not found: $templateSchema"; exit 1 }

# ── Load templates ────────────────────────────────────────────────────────────
$rawTemplates = Get-Content -Path $templatesJson -Raw -Encoding UTF8
try {
  $templates = $rawTemplates | ConvertFrom-Json
} catch {
  Write-Error "Invalid JSON in templates.json: $_"
  exit 1
}

if ($templates.Count -eq 0) {
  Write-Error "templates.json is empty -- at least one template required."
  exit 1
}

# ── Load schema for basic field validation ────────────────────────────────────
$rawSchema = Get-Content -Path $templateSchema -Raw -Encoding UTF8
try {
  $schema = $rawSchema | ConvertFrom-Json
} catch {
  Write-Error "Invalid JSON in template.schema.json: $_"
  exit 1
}

$requiredFields = $schema.required
$validClasses = $schema.properties.aliasClass.enum
$validStreams = $schema.properties.stream.enum

$errorCount = 0
for ($i = 0; $i -lt $templates.Count; $i++) {
  $t = $templates[$i]
  $label = "templates[$i] (id=$($t.id))"

  # Check required fields
  foreach ($field in $requiredFields) {
    $val = $t.$field
    if ($null -eq $val -or ($val -is [string] -and $val -eq '')) {
      Write-Warning "$label is missing required field: $field"
      $errorCount++
    }
  }

  # Validate aliasClass enum
  if ($t.aliasClass -and $validClasses -notcontains $t.aliasClass) {
    Write-Warning "$label has invalid aliasClass: $($t.aliasClass)"
    $errorCount++
  }

  # Validate stream enum
  if ($t.stream -and $validStreams -notcontains $t.stream) {
    Write-Warning "$label has invalid stream: $($t.stream)"
    $errorCount++
  }

  # Validate id pattern
  if ($t.id -and $t.id -notmatch '^[a-z0-9-]+$') {
    Write-Warning "$label has invalid id pattern: $($t.id)"
    $errorCount++
  }

  # Validate templateSchedule has blocks
  if (-not $t.templateSchedule -or -not $t.templateSchedule.blocks -or $t.templateSchedule.blocks.Count -eq 0) {
    Write-Warning "$label has no templateSchedule.blocks"
    $errorCount++
  }
}

# Check uniqueness of ids
$ids = $templates | ForEach-Object { $_.id }
$dupes = $ids | Group-Object | Where-Object { $_.Count -gt 1 }
if ($dupes) {
  foreach ($d in $dupes) {
    Write-Warning "Duplicate template id: $($d.Name)"
    $errorCount++
  }
}

if ($errorCount -gt 0) {
  Write-Error "Template validation failed with $errorCount error(s). Fix templates.json and re-run."
  exit 1
}

Write-Host "  Validated $($templates.Count) templates against schema." -ForegroundColor Cyan

# ── Generate compact JSON for embedding ───────────────────────────────────────
$templateJson = Normalize-JsonText $rawTemplates

# ── Inject into ferros-core.js ────────────────────────────────────────────────
$coreContent = Get-Content -Path $coreJs -Raw -Encoding UTF8

$placeholder = 'FerrosCore.TEMPLATE_PROFILES = [];'
$replacement = "FerrosCore.TEMPLATE_PROFILES = $templateJson;"

if ($coreContent -notmatch [regex]::Escape($placeholder)) {
  # Check if already populated (idempotent re-run)
  if ($coreContent -match 'FerrosCore\.TEMPLATE_PROFILES = \[') {
    # Replace existing populated line
    $coreContent = [regex]::Replace(
      $coreContent,
      'FerrosCore\.TEMPLATE_PROFILES = \[.*?\];',
      $replacement,
      [System.Text.RegularExpressions.RegexOptions]::Singleline
    )
  } else {
    Write-Error "Could not find TEMPLATE_PROFILES placeholder in ferros-core.js"
    exit 1
  }
} else {
  $coreContent = $coreContent.Replace($placeholder, $replacement)
}

[System.IO.File]::WriteAllText($coreJs, $coreContent, [System.Text.UTF8Encoding]::new($false))

Write-Host ""
Write-Host "FERROS Core bundle updated successfully." -ForegroundColor Green
Write-Host "  Output:    $coreJs"
Write-Host "  Templates: $($templates.Count)"
Write-Host ""
