# 🌱🦀 NALA-Understory-RS (`v0.9.0-nala.6.6.6`) — English Documentation (EN)

**100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, `0` external crates, `~899 KB` single binary, `36` static prompt-cache-safe MCP tools, `38/38` unit tests, `39` benchmarked subsystems)**

## 🎯 What's New in Wave-8 (`v0.9.0-nala.6.6.6` · `RSI-UNDERSTORY-029..032`)
- **`RSI-029` (`--migrate-export`)**: Clean-room migrator for **Notion Markdown/CSV**, **Confluence HTML**, and **Jira/Linear JSON** → `OKF v0.1` in **`0.71 µs/doc`** with XSS/SSRF stripping & Notion UUID cleanup.
- **`RSI-030` (`--detect-conflicts`)**: Zero-allocation semantic contradiction & stale-claim detector in **`1.10 µs`** with **0 LLM calls** and Evidence-Lock `[path#Lline]` citations.
- **`RSI-031` (`--patch-bundle`)**: Cryptographic **RFC 2104 HMAC-SHA256** signed `.okf.patch` multi-node airgap bundles in **`4.67 µs`** with constant-time verification, monotonic anti-replay nonce & zone allowlists.
- **`RSI-032` (`--cache-telemetry`)**: Deterministic token-budget context diff & **KV-cache prefix pinning telemetry** in **`1,199.5 ns`** (`>= 98%` prefix hit rate, zone-bound prefix hash).

## ⚡ Verified Multi-Hardware Benchmarks (`v0.9.0-nala.6.6.6`)
- **Apple Silicon M4 Max**: `RSI-029`: **`0.71 µs`** · `RSI-030`: **`1.10 µs`** · `RSI-031`: **`4.67 µs`** · `RSI-032`: **`1,199.5 ns`**
- **Linux x86_64 Server**: `RSI-029`: **`2.37 µs`** · `RSI-030`: **`3.30 µs`** · `RSI-031`: **`12.20 µs`** · `RSI-032`: **`1,758.1 ns`**
- **Proxmox VE 9.2 Airgapped (`unshare -n env -i`)**: `RSI-029`: **`2.49 µs`** · `RSI-030`: **`3.54 µs`** · `RSI-031`: **`13.81 µs`** · `RSI-032`: **`1,874.7 ns`**

Download all 5 verified binaries & `SHA256SUMS.txt` from [GitHub Releases](https://github.com/Master-MD/NALA-Understory-RS/releases/tag/v0.9.0-nala.6.6.6). License: Apache-2.0.
