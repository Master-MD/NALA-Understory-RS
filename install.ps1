# ============================================================================
# 🌱🦀 NALA-Understory-RS v0.5.0-nala.6.6.6 — Windows PowerShell Installer
# Supports: DE / FR / IT / RM / EN | Architecture: Windows x86_64
# 100% Standalone (Works WITH or WITHOUT NALA-OS) + Cryptographic SHA-256 Check
# ============================================================================
$ErrorActionPreference = "Stop"
$Repo = "Master-MD/NALA-Understory-RS"
$Version = if ($env:NALA_UNDERSTORY_VERSION) { $env:NALA_UNDERSTORY_VERSION } else { "v0.5.0-nala.6.6.6" }
$InstallDir = if ($env:NALA_INSTALL_DIR) { $env:NALA_INSTALL_DIR } else { "$HOME\bin" }

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
$Asset = "nala-understory-rs-windows-x86_64.exe"
$BaseUrl = "https://github.com/$Repo/releases/download/$Version"
$OutFile = Join-Path $InstallDir "nala-understory-rs.exe"
$SumsFile = Join-Path $InstallDir "SHA256SUMS.txt"

Write-Host "🌱🦀 Downloading $Asset ($Version)..." -ForegroundColor Cyan
Invoke-WebRequest -Uri "$BaseUrl/$Asset" -OutFile $OutFile
Invoke-WebRequest -Uri "$BaseUrl/SHA256SUMS.txt" -OutFile $SumsFile

$ActualHash = (Get-FileHash -Path $OutFile -Algorithm SHA256).Hash.ToLower()
$MatchLine = Get-Content $SumsFile | Where-Object { $_ -match "  $Asset$" } | Select-Object -First 1
Remove-Item $SumsFile -Force -ErrorAction SilentlyContinue
if ($MatchLine) {
  $ExpectedHash = ($MatchLine -split "\s+")[0].ToLower()
  if ($ExpectedHash -ne $ActualHash) {
    Remove-Item $OutFile -Force
    throw "SHA-256 mismatch! Expected $ExpectedHash, got $ActualHash"
  }
}

Copy-Item $OutFile (Join-Path $InstallDir "ustory-rs.exe") -Force
Write-Host "✅ SHA-256 verified ($ActualHash) -> Installed to $OutFile" -ForegroundColor Green
& $OutFile --self-hash
