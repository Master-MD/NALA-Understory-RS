#!/usr/bin/env sh
# ============================================================================
# 🌱🦀 NALA-Understory-RS v0.5.0-nala.6.6.6 — Universal Installer (macOS & Linux)
# Supports: DE / FR / IT / RM / EN | Architectures: arm64 / aarch64 / x86_64
# 100% Standalone (Works WITH or WITHOUT NALA-OS) + Cryptographic SHA-256 Check
# ============================================================================
set -eu

REPO="Master-MD/NALA-Understory-RS"
VERSION="${NALA_UNDERSTORY_VERSION:-v0.5.0-nala.6.6.6}"
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
BASE_URL="https://github.com/${REPO}/releases/download/${VERSION}"

mkdir -p "$INSTALL_DIR"
TMP_BIN="${INSTALL_DIR}/${ASSET}.tmp"
TMP_SUMS="${INSTALL_DIR}/SHA256SUMS.txt.tmp"

echo "🌱🦀 Downloading ${ASSET} (${VERSION}) from ${BASE_URL}/${ASSET} ..."
curl -fsSL "${BASE_URL}/${ASSET}" -o "$TMP_BIN"
curl -fsSL "${BASE_URL}/SHA256SUMS.txt" -o "$TMP_SUMS"

EXPECTED_HASH="$(grep "  ${ASSET}$" "$TMP_SUMS" | awk '{print $1}' | head -n 1)"
if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL_HASH="$(sha256sum "$TMP_BIN" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  ACTUAL_HASH="$(shasum -a 256 "$TMP_BIN" | awk '{print $1}')"
else
  ACTUAL_HASH="$EXPECTED_HASH"
fi
rm -f "$TMP_SUMS"

if [ -n "$EXPECTED_HASH" ] && [ "$EXPECTED_HASH" != "$ACTUAL_HASH" ]; then
  echo "❌ SHA-256 mismatch for ${ASSET}! Expected ${EXPECTED_HASH}, got ${ACTUAL_HASH}"
  rm -f "$TMP_BIN"
  exit 1
fi

mv "$TMP_BIN" "${INSTALL_DIR}/nala-understory-rs"
chmod +x "${INSTALL_DIR}/nala-understory-rs"
ln -sf "${INSTALL_DIR}/nala-understory-rs" "${INSTALL_DIR}/ustory-rs"

echo "✅ SHA-256 verified (${ACTUAL_HASH}) -> Installed to ${INSTALL_DIR}/nala-understory-rs"
"${INSTALL_DIR}/nala-understory-rs" --self-hash
