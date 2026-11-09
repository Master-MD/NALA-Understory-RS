#!/usr/bin/env sh
# ============================================================================
# 🌱🦀 NALA-Understory-RS v0.4.0-nala.6.6.6 — Universal Installer (macOS & Linux)
# Supports: DE / FR / IT / RM / EN | Architectures: arm64 / aarch64 / x86_64
# ============================================================================
set -eu

REPO="Master-MD/NALA-Understory-RS"
VERSION="${NALA_UNDERSTORY_VERSION:-v0.4.0-nala.6.6.6}"
INSTALL_DIR="${NALA_INSTALL_DIR:-$HOME/bin}"

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  darwin) PLATFORM="macos" ;;
  linux)  PLATFORM="linux" ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  arm64|aarch64)
    if [ "$PLATFORM" = "macos" ]; then
      TARGET_ARCH="arm64"
    else
      TARGET_ARCH="aarch64"
    fi
    ;;
  x86_64|amd64) TARGET_ARCH="x86_64" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

ASSET="nala-understory-rs-${PLATFORM}-${TARGET_ARCH}"
URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET}"

mkdir -p "$INSTALL_DIR"
echo "🌱🦀 Downloading ${ASSET} (${VERSION}) from ${URL} ..."
curl -fsSL "$URL" -o "${INSTALL_DIR}/nala-understory-rs"
chmod +x "${INSTALL_DIR}/nala-understory-rs"
ln -sf "${INSTALL_DIR}/nala-understory-rs" "${INSTALL_DIR}/ustory-rs"

echo "✅ Installed to ${INSTALL_DIR}/nala-understory-rs (alias: ustory-rs)"
"${INSTALL_DIR}/nala-understory-rs" --version
