# 🌱🦀 NALA-Understory-RS (`v0.8.0-nala.6.6.6`) — Drop-In 100% Native Rust Engine for `thecodacus/understory`, Obsidian Vaults & OKF v0.1 Agent Memory

> **Drop-in 100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, `0` External Crates, `~820 KB` Single Binary, 100% Universal Standalone) Rewrite & Wave-7 Evolution (`RSI-UNDERSTORY-001..028`) of [`thecodacus/understory`](https://github.com/thecodacus/understory)**  
> **Solves Upstream Issues `#23`, `#33`, `#34`, `#40`, `#51`–`#60` + Adds Bidirectional Obsidian Vault Sync (`1.16 µs`), `25.4 ns` Levenshtein-1 Fuzzy FST Autocomplete, Mermaid/PlantUML/Graphviz Sync (`3.30 µs`), PageRank Salience (`14.62 µs`), and `800.7 ns` Shannon-Entropy Secret Guard!**  
> **Multi-Language Documentation Hub:** 🇨🇭 [Deutsch (`docs/README_DE.md`)](docs/README_DE.md) · 🇫🇷 [Français (`docs/README_FR.md`)](docs/README_FR.md) · 🇮🇹 [Italiano (`docs/README_IT.md`)](docs/README_IT.md) · 🏔️ [Rumantsch Grischun (`docs/README_RM.md`)](docs/README_RM.md) · 🇬🇧 [English (`docs/README_EN.md`)](docs/README_EN.md)  
> **Beginner & Caveman Guides:** 🪨 [`CAVEMAN_GUIDE.md`](CAVEMAN_GUIDE.md) · ❓ [`FAQ.md`](FAQ.md) · ⚖️ [`LICENSE`](LICENSE) (`Apache-2.0`) & [`LICENSE_SUMMARY_5LANG.md`](LICENSE_SUMMARY_5LANG.md) · 📜 [`README_UPSTREAM_UNDERSTORY.md`](README_UPSTREAM_UNDERSTORY.md)

---

## 🎯 Wave-7 Highlights (`v0.8.0-nala.6.6.6` · Hybrid Community + NALA-OS Edition)

| Module | Capability (`100%` Native Safe Rust · `0` External Crates) | Measured Latency (M4 Max) | Security / Invariant |
| :--- | :--- | ---: | :--- |
| **`RSI-025` (`--obsidian-sync`)** | **Obsidian Vault ↔ OKF v0.1 Bidirectional Bridge:** Translates `[[Target\|Alias]]`, `key:: value` Dataview fields, and `#inline-tags` outside code fences. | **`1.16 µs/note`** (`2,155x` SLA) | Blocks `.obsidian/workspace.json`, plugin `data.json`, `.trash`, `.env`, and symlinks. |
| **`RSI-026` (`--fuzzy-suggest`)** | **Zero-Allocation Succinct Prefix-Trie & Levenshtein-1 Autocomplete (`.okf.fst`):** Instant typo-tolerant suggestions (`"biling"` → `"billing-api"`). | **`25.4 ns/op`** (`25.6x` SLA) | Strictly `TenantZone`-partitioned + keyed SHA-256 MAC on `.okf.fst`. |
| **`RSI-027` (`--diagram-sync`)** | **Clean-Room Mermaid / PlantUML / Graphviz ↔ OKF Diagram Sync:** Extracts graph edges from diagrams and synthesizes deterministic Mermaid `flowchart LR`. | **`3.30 µs/op`** (`28.8x` SLA) | `sHeriFF` XSS & `<script>` breakout sanitizer on every node/edge label. |
| **`RSI-028A` (`--salience-rank`)** | **Incremental Power-Iteration PageRank Salience Scorer:** 8-iteration eigenvector centrality (`d=0.85`) prioritizing core architecture hubs. | **`14.62 µs/1k edges`** (`3.1x` SLA) | Self-loop & duplicate-edge cap prevents artificial link-farm boosting. |
| **`RSI-028B` (`--entropy-guard`)** | **Deterministic Shannon-Entropy (`>4.35 bits/char`) & Credential Canary Guard:** Scans plaintext outside `# [VAULT:*]` for API keys, JWTs & PEM blocks. | **`800.7 ns/op`** (`2.25x` SLA) | Zero-allocation ASCII byte-mask prefilter + fail-closed write guard. |

---

## 🌍 5-Language Executive Overview (`v0.8.0-nala.6.6.6` · `32` MCP Tools · `34/34` Unit Tests)

| Language | Summary |
| :--- | :--- |
| 🇨🇭 **Deutsch (DE)** | **NALA-Understory-RS (`v0.8.0-nala.6.6.6`)** ist die 100% native Rust-Turbo Engine (`Edition 2024`, `0` externe Crates, Zero-Node.js, Zero-Python) für `thecodacus/understory` (`OKF v0.1`), Obsidian-Vaults und Multi-Agenten-MCP (`32` statische Prompt-Cache-sichere Tools, `34/34` Tests, `35` Subsystem-Benchmarks). Läuft 100% eigenständig auf macOS, Linux/Proxmox (Airgap) & Windows und dient gleichzeitig als Zero-PII Upstream-Quelle für souveräne KI-Cluster (`NALA-OS`, `Antigravity`, `Codex`, `ZCode`). |
| 🇫🇷 **Français (FR)** | **NALA-Understory-RS (`v0.8.0-nala.6.6.6`)** est le moteur 100% Rust natif (`Edition 2024`, `0` crate externe) pour `thecodacus/understory`, Obsidian et MCP (`32` outils statiques, `34/34` tests, `RSI-001..028`) avec garantie `Zero-PII`. |
| 🇮🇹 **Italiano (IT)** | **NALA-Understory-RS (`v0.8.0-nala.6.6.6`)** è il motore 100% Rust nativo (`Edition 2024`, `0` dipendenze) per `thecodacus/understory`, Obsidian e MCP (`32` strumenti statici, `34/34` test, `RSI-001..028`). |
| 🏔️ **Rumantsch (RM)** | **NALA-Understory-RS (`v0.8.0-nala.6.6.6`)** è il motor 100% Rust nativ (`Edition 2024`) per `thecodacus/understory`, Obsidian e MCP (`32` utensils statics, `34/34` tests, `RSI-001..028`). |
| 🇬🇧 **English (EN)** | **NALA-Understory-RS (`v0.8.0-nala.6.6.6`)** is a 100% native Rust-Turbo (`Edition 2024`, zero Node.js, zero Python, zero external crates) drop-in engine for `thecodacus/understory`, Obsidian vaults, and `OKF v0.1` delivering `RSI-001..028`, `32` prompt-cache-safe MCP tools, and `34/34` unit tests. |

---

## ⚡ Self-Tested 35-Subsystem Benchmark Across 3 Hardware Classes (incl. Airgapped Proxmox VE 9.2)

Tested on the official OKF v0.1 `sample-bundle` (`10,000` iterations via `nala-understory-rs --bench`):

| Subsystem / Operation | Upstream (`Node.js 22`) | Apple Silicon M4 Max (`arm64`) | Dual Xeon Server (`Linux x86_64`) | Proxmox VE 9.2 Airgapped (`unshare -n env -i`) | Peak Speedup / SLA |
| :--- | ---: | ---: | ---: | ---: | :--- |
| **1. YAML Frontmatter Parse (`okf::frontmatter`)** | `48,200.0 ns` | **`552.4 ns`** | `2,177.2 ns` | `2,042.2 ns` | **87.3x faster** |
| **2. Full Bundle Graph + Lint + Conformance** | `3,840.0 µs` | `92.14 µs` | **`74.33 µs`** | `84.40 µs` | **51.7x faster** |
| **3. Weighted Multi-Field Concept Search** | `1,420.0 µs` | `120.14 µs` | **`106.72 µs`** | `107.91 µs` | **13.3x faster** |
| **4. Duplicate Concept Jaccard Scan (`dream`)** | `64,500.0 ns` | **`3,996.6 ns`** | `23,256.8 ns` | `25,999.9 ns` | **16.1x faster** |
| **5. Session-Start MCP Seed (`build_seed_memory`)** | `1,910.0 µs` | **`12.24 µs`** | `26.68 µs` | `29.93 µs` | **156.0x faster** |
| **6. Deterministic OKF Index Regen (`indexer`)** | `2,650.0 µs` | `139.70 µs` | `108.69 µs` | **`104.23 µs`** | **25.4x faster** |
| **7–26. `RSI-001..020` (Waves 1–5 Core Engine)** | *N/A* | **`23.4 ns` – `316.0 µs`** | `58.0 ns` – `189.1 µs` | `61.3 ns` – `236.6 µs` | **All 20 SLAs beaten** |
| **27–30. `RSI-021..024` (Wave-6 Upstream Fixes `#23`–`#60`)** | *Broken/360k tok* | **`523.3 ns` – `117.7 µs`** | `1.74 µs` – `115.0 µs` | `2.07 µs` – `143.0 µs` | **`0` broken + Cache-Safe** |
| **31. `RSI-025` Obsidian `[[Wiki]]` / Dataview Sync** | *N/A* | **`1.16 µs`** | `4.41 µs` | `4.76 µs` | **2,155x beat `<2.5 ms` SLA** |
| **32. `RSI-026` Zero-Alloc Fuzzy FST (`.okf.fst`)** | *N/A* | **`25.4 ns`** | `82.1 ns` | `105.9 ns` | **25.6x beat `<650 ns` SLA** |
| **33. `RSI-027` Mermaid / PlantUML / DOT Sync** | *N/A* | **`3.30 µs`** | `11.12 µs` | `12.36 µs` | **28.8x beat `<95 µs` SLA** |
| **34. `RSI-028A` PageRank Salience (`1k` Edges)** | *N/A* | **`14.62 µs`** | `70.94 µs` | `89.42 µs` | **3.1x beat `<45 µs` SLA** |
| **35. `RSI-028B` Shannon-Entropy Canary Guard** | *N/A* | **`800.7 ns`** | `1,378.8 ns` | `1,993.0 ns` | **2.25x beat `<1.8 µs` SLA** |

---

## 🔐 Cryptographic FIPS 180-4 `SHA-256` Binary Manifest (`v0.8.0-nala.6.6.6`)

| Platform / Architecture | Binary Asset | HAL Profile | FIPS 180-4 `SHA-256` Digest |
| :--- | :--- | :--- | :--- |
| **macOS Apple Silicon (`arm64`)** | `nala-understory-rs-macos-arm64` | `APPLE_SILICON_ARM64_NEON` | `2d148bb4dac2ba0f52748ab7f6ffd16a827e0d020e8a38004d31e350b4343e7f` |
| **macOS Intel (`x86_64`)** | `nala-understory-rs-macos-x86_64` | `MACOS_INTEL_X86_64_SSE42_AVX` | `197eeb7ea195dabf3ca1f416776e0803466e8e76f310a6deb56422d2df63a144` |
| **Linux Static PIE (`x86_64`, Musl)** | `nala-understory-rs-linux-x86_64` | `LINUX_X86_64_UNIVERSAL` | `815e7745c8d30d21421c4bbea8848bc067b220d0429172689bd50954989db44f` |
| **Linux Static (`aarch64`, Musl)** | `nala-understory-rs-linux-aarch64` | `LINUX_ARM64_NEON_UNIVERSAL` | `c8ea77da4c236772e0056fd970b782d69ec164028282f39dc0ef6bdbb9a87715` |
| **Windows 10/11 (`x86_64` PE32+)** | `nala-understory-rs-windows-x86_64.exe` | `WINDOWS_X86_64_UNIVERSAL` | `041c4b5d1700ae13bafd2888c03eb758ae161a841fa1c6b4ab99c10d2654b824` |

---

## 🚀 Quickstart (Drop-In MCP Server & CLI for macOS, Linux, Proxmox & Windows)

### 1. One-Liner Install with Automatic `SHA-256` Verification
```bash
# macOS & Linux
curl -fsSL https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.sh | sh

# Windows PowerShell
irm https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.ps1 | iex
```

### 2. CLI Commands (`v0.8.0-nala.6.6.6` — 32 MCP Tools & 35 Benchmarked Subsystems)
```bash
nala-understory-rs --standalone-check
nala-understory-rs --bench
nala-understory-rs --obsidian-sync /path/to/obsidian-vault
nala-understory-rs --fuzzy-suggest biling
nala-understory-rs --diagram-sync
nala-understory-rs --salience-rank
nala-understory-rs --entropy-guard
nala-understory-rs --mcp-stdio --bundle /path/to/okf-bundle
```
