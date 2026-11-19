# 🌱🦀 NALA-Understory-RS (`v0.9.0-nala.6.6.6`) — Drop-In 100% Native Rust Engine for `thecodacus/understory`, Obsidian, Enterprise Exports & OKF v0.1 Agent Memory

> **Drop-in 100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, `0` External Crates, `~899 KB` Single Binary, 100% Universal Standalone) Rewrite & Wave-8 Evolution (`RSI-UNDERSTORY-001..032`) of [`thecodacus/understory`](https://github.com/thecodacus/understory)**  
> **Solves Upstream Issues `#23`, `#33`, `#34`, `#40`, `#51`–`#60` + Wave-7 (Obsidian `1.15 µs`, `25.7 ns` Fuzzy FST, Diagram Sync, PageRank, `688.7 ns` Entropy Guard) + Wave-8 (Notion/Confluence/Jira/Linear Migrator `0.71 µs`, Zero-LLM Conflict Detector `1.10 µs`, HMAC-SHA256 `.okf.patch` Airgap Sync `4.67 µs`, `1.20 µs` KV-Cache Prefix Telemetry)!**  
> **Multi-Language Documentation Hub:** 🇨🇭 [Deutsch (`docs/README_DE.md`)](docs/README_DE.md) · 🇫🇷 [Français (`docs/README_FR.md`)](docs/README_FR.md) · 🇮🇹 [Italiano (`docs/README_IT.md`)](docs/README_IT.md) · 🏔️ [Rumantsch Grischun (`docs/README_RM.md`)](docs/README_RM.md) · 🇬🇧 [English (`docs/README_EN.md`)](docs/README_EN.md)  
> **Beginner & Caveman Guides:** 🪨 [`CAVEMAN_GUIDE.md`](CAVEMAN_GUIDE.md) · ❓ [`FAQ.md`](FAQ.md) · ⚖️ [`LICENSE`](LICENSE) (`Apache-2.0`) & [`LICENSE_SUMMARY_5LANG.md`](LICENSE_SUMMARY_5LANG.md) · 📜 [`README_UPSTREAM_UNDERSTORY.md`](README_UPSTREAM_UNDERSTORY.md)

---

## 🎯 Wave-8 Highlights (`v0.9.0-nala.6.6.6` · Hybrid Community + Enterprise Airgap Edition)

| Module | Capability (`100%` Native Safe Rust · `0` External Crates) | Measured Latency (M4 Max) | Security / Invariant |
| :--- | :--- | ---: | :--- |
| **`RSI-029` (`--migrate-export`)** | **Notion / Confluence / Jira / Linear → OKF v0.1 Clean-Room Migrator:** Strips Notion 32-hex UUIDs, converts Confluence HTML & Notion CSV tables, extracts Jira/Linear JSON issues. | **`0.71 µs/doc`** (`4,507x` SLA) | Strips `<script>`, `<iframe>`, `javascript:`, `file://`, IMDS `169.254.169.254`, symlinks & `..`. |
| **`RSI-030` (`--detect-conflicts`)** | **Zero-Allocation Semantic Contradiction & Stale-Claim Detector:** Detects numeric/SLA/version drift, polarity negations & expired ISO dates across concepts with **0 LLM calls**. | **`1.10 µs/op`** (`163.6x` SLA) | Evidence-Lock `[path#Lline]` citations for every contradiction & stale claim. |
| **`RSI-031` (`--patch-bundle`)** | **Cryptographic RFC 2104 HMAC-SHA256 Signed `.okf.patch` Airgap Sync:** Packs & verifies binary multi-node delta patches without Git remotes. | **`4.67 µs/op`** (`19.3x` SLA) | Constant-time 32B MAC verify, monotonic anti-replay nonce & strict `TenantZone` allowlist. |
| **`RSI-032` (`--cache-telemetry`)** | **Deterministic Token-Budget Context Diff & KV-Cache Prefix Pinning Telemetry:** 8-byte word-accelerated LCP scan measuring `>= 98%` KV-cache prefix hit rate across MCP turns. | **`1,199.5 ns/op`** (`10.0x` SLA) | Zone-bound FNV-1a prefix salt prevents cross-tenant KV-cache collision/poisoning. |

---

## 🌍 5-Language Executive Overview (`v0.9.0-nala.6.6.6` · `36` MCP Tools · `38/38` Unit Tests)

| Language | Summary |
| :--- | :--- |
| 🇨🇭 **Deutsch (DE)** | **NALA-Understory-RS (`v0.9.0-nala.6.6.6`)** ist die 100% native Rust-Turbo Engine (`Edition 2024`, `0` externe Crates, Zero-Node.js, Zero-Python) für `thecodacus/understory` (`OKF v0.1`), Obsidian, Notion/Confluence/Jira/Linear-Migration und Multi-Agenten-MCP (`36` statische Prompt-Cache-sichere Tools, `38/38` Tests, `39` Subsystem-Benchmarks). Läuft 100% eigenständig auf macOS, Linux/Proxmox (Airgap) & Windows. |
| 🇫🇷 **Français (FR)** | **NALA-Understory-RS (`v0.9.0-nala.6.6.6`)** est le moteur 100% Rust natif (`Edition 2024`, `0` crate externe) pour `thecodacus/understory`, Obsidian, Notion/Confluence/Jira/Linear et MCP (`36` outils statiques, `38/38` tests, `RSI-001..032`) avec garantie `Zero-PII`. |
| 🇮🇹 **Italiano (IT)** | **NALA-Understory-RS (`v0.9.0-nala.6.6.6`)** è il motore 100% Rust nativo (`Edition 2024`, `0` dipendenze) per `thecodacus/understory`, Obsidian, Notion/Confluence/Jira/Linear e MCP (`36` strumenti statici, `38/38` test, `RSI-001..032`). |
| 🏔️ **Rumantsch (RM)** | **NALA-Understory-RS (`v0.9.0-nala.6.6.6`)** è il motor 100% Rust nativ (`Edition 2024`) per `thecodacus/understory`, Obsidian, Notion/Confluence/Jira/Linear e MCP (`36` utensils statics, `38/38` tests, `RSI-001..032`). |
| 🇬🇧 **English (EN)** | **NALA-Understory-RS (`v0.9.0-nala.6.6.6`)** is a 100% native Rust-Turbo (`Edition 2024`, zero Node.js, zero Python, zero external crates) drop-in engine for `thecodacus/understory`, Obsidian, Notion/Confluence/Jira/Linear & `OKF v0.1` delivering `RSI-001..032`, `36` prompt-cache-safe MCP tools, and `38/38` unit tests. |

---

## ⚡ Self-Tested 39-Subsystem Benchmark Across 3 Hardware Classes (incl. Airgapped Proxmox VE 9.2)

Tested on the official OKF v0.1 `sample-bundle` (`10,000` iterations via `nala-understory-rs --bench`):

| Subsystem / Operation | Upstream (`Node.js 22`) | Apple Silicon M4 Max (`arm64`) | Dual Xeon Server (`Linux x86_64`) | Proxmox VE 9.2 Airgapped (`unshare -n env -i`) | Peak Speedup / SLA |
| :--- | ---: | ---: | ---: | ---: | :--- |
| **1. YAML Frontmatter Parse (`okf::frontmatter`)** | `48,200.0 ns` | **`581.1 ns`** | `2,177.2 ns` | `2,042.2 ns` | **82.9x faster** |
| **2–6. Arena Scan, Search, Jaccard, Seed, Indexer** | `1.4–3.8 ms` | **`3.86 µs` – `155.8 µs`** | `23.2 µs` – `108.7 µs` | `26.0 µs` – `104.2 µs` | **Up to 159x faster** |
| **7–30. `RSI-001..024` (Waves 1–6 + Fixes `#23`–`#60`)** | *N/A / Broken* | **`21.6 ns` – `336.9 µs`** | `58.0 ns` – `138.1 µs` | `61.3 ns` – `149.2 µs` | **All 24 SLAs beaten** |
| **31–35. `RSI-025..028` (Wave-7 Obsidian/FST/Diag/PR/DLP)** | *N/A* | **`25.7 ns` – `15.08 µs`** | `82.1 ns` – `70.94 µs` | `105.9 ns` – `89.42 µs` | **Up to 2,173x SLA** |
| **36. `RSI-029` Notion/Confluence/Jira/Linear Migrator** | *N/A* | **`0.71 µs`** | `2.37 µs` | `2.49 µs` | **4,507x beat `<3.2 ms` SLA** |
| **37. `RSI-030` Zero-LLM Conflict & Stale Detector** | *N/A* | **`1.10 µs`** | `3.30 µs` | `3.54 µs` | **163.6x beat `<180 µs` SLA** |
| **38. `RSI-031` HMAC-SHA256 `.okf.patch` Airgap Sync** | *N/A* | **`4.67 µs`** | `12.20 µs` | `13.81 µs` | **19.3x beat `<90 µs` SLA** |
| **39. `RSI-032` Prompt-Cache Prefix Pinning Telemetry** | *N/A* | **`1,199.5 ns`** | `1,758.1 ns` | `1,874.7 ns` | **10.0x beat `<12 µs` SLA** |

---

## 🔐 Cryptographic FIPS 180-4 `SHA-256` Binary Manifest (`v0.9.0-nala.6.6.6`)

| Platform / Architecture | Binary Asset | HAL Profile | FIPS 180-4 `SHA-256` Digest |
| :--- | :--- | :--- | :--- |
| **macOS Apple Silicon (`arm64`)** | `nala-understory-rs-macos-arm64` | `APPLE_SILICON_ARM64_NEON` | `b4c3cfff6f3289c0a588a603f82098130f4cf8ab81d788716f02d9748b5f544f` |
| **macOS Intel (`x86_64`)** | `nala-understory-rs-macos-x86_64` | `MACOS_INTEL_X86_64_SSE42_AVX` | `291cb96f1147079f37cc82fb271e7ca4d32c4bf69253dadeb8515d6c35c38f4a` |
| **Linux Static PIE (`x86_64`, Musl)** | `nala-understory-rs-linux-x86_64` | `LINUX_X86_64_UNIVERSAL` | `998c1ffb93ead2d74fb8e0f38fb1180ab53687d9efca3050bb67fab8e692872c` |
| **Linux Static (`aarch64`, Musl)** | `nala-understory-rs-linux-aarch64` | `LINUX_ARM64_NEON_UNIVERSAL` | `edddcee2ff568a630ef500d64f07b6775b66d33ffeddc113b16a538e789702a7` |
| **Windows 10/11 (`x86_64` PE32+)** | `nala-understory-rs-windows-x86_64.exe` | `WINDOWS_X86_64_UNIVERSAL` | `a9b82f6c3760378074c83a1331c8408f0eab9644f08036693de7651c0aa7ff38` |

---

## 🚀 Quickstart (Drop-In MCP Server & CLI for macOS, Linux, Proxmox & Windows)

### 1. One-Liner Install with Automatic `SHA-256` Verification
```bash
# macOS & Linux
curl -fsSL https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.sh | sh

# Windows PowerShell
irm https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.ps1 | iex
```

### 2. CLI Commands (`v0.9.0-nala.6.6.6` — 36 MCP Tools & 39 Benchmarked Subsystems)
```bash
nala-understory-rs --standalone-check
nala-understory-rs --bench
nala-understory-rs --migrate-export /path/to/notion-or-confluence-export
nala-understory-rs --detect-conflicts 2026-11-19
nala-understory-rs --patch-bundle /tmp/sync.okf.patch
nala-understory-rs --cache-telemetry
nala-understory-rs --mcp-stdio --bundle /path/to/okf-bundle
```
