# ============================================================================
# 🌱🦀 NALA-Understory-RS v0.1.0-nala.6.6.6 — Windows PowerShell Installer
# Supports: DE / FR / IT / RM / EN | Architecture: Windows x86_64
# ============================================================================
$ErrorActionPreference = "Stop"
$Repo = "Master-MD/NALA-Understory-RS"
$Version = if ($env:NALA_UNDERSTORY_VERSION) { $env:NALA_UNDERSTORY_VERSION } else { "v0.2.0-nala.6.6.6" }
$InstallDir = if ($env:NALA_INSTALL_DIR) { $env:NALA_INSTALL_DIR } else { "$HOME\bin" }

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
$Asset = "nala-understory-rs-windows-x86_64.exe"
$Url = "https://github.com/$Repo/releases/download/$Version/$Asset"
$OutFile = Join-Path $InstallDir "nala-understory-rs.exe"

Write-Host "🌱🦀 Downloading $Asset ($Version)..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $Url -OutFile $OutFile
Copy-Item $OutFile (Join-Path $InstallDir "ustory-rs.exe") -Force
Write-Host "✅ Installed to $OutFile" -ForegroundColor Green
& $OutFile --version
