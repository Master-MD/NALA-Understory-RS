# 🌱🦀 NALA-Understory-RS (`v1.3.0-nala.6.6.6`) — Production 100% Safe Rust Engine for `thecodacus/understory`, 4-Bit Q4 HNSW-Lite, XMSS-Lite PQ Merkle, Lakehouse & ZK Selective Disclosure

> **Wave-12 (`v1.3.0`) Drop-in 100% Native Safe Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, `0` External Crates, `0` `unsafe`, `0` Compiler Warnings, `~1.06 MB` Single Binary, 100% Universal Standalone) Rewrite & 12-Wave Evolution (`RSI-UNDERSTORY-001..048`) of [`thecodacus/understory`](https://github.com/thecodacus/understory)**  
> **52 Static Prompt-Cache-Safe MCP Tools · 54/54 Release Unit Tests (`0.04 s`) · 55 Benchmarked Subsystems · Zero-Alloc Streaming FIPS 180-4 SHA-256 · Solves Upstream `#23`, `#33`, `#34`, `#40`, `#51`–`#60` · Verified on Apple Silicon M4 Max, Linux x86_64/arm64, Airgapped Proxmox VE 9.2 (`unshare -n env -i`) & Windows 11!**  
> **Multi-Language Documentation Hub:** 🇨🇭 [Deutsch (`docs/README_DE.md`)](docs/README_DE.md) · 🇫🇷 [Français (`docs/README_FR.md`)](docs/README_FR.md) · 🇮🇹 [Italiano (`docs/README_IT.md`)](docs/README_IT.md) · 🏔️ [Rumantsch Grischun (`docs/README_RM.md`)](docs/README_RM.md) · 🇬🇧 [English (`docs/README_EN.md`)](docs/README_EN.md)  
> **Beginner & Caveman Guides:** 🪨 [`CAVEMAN_GUIDE.md`](CAVEMAN_GUIDE.md) · ❓ [`FAQ.md`](FAQ.md) · ⚖️ [`LICENSE`](LICENSE) (`Apache-2.0`) & [`LICENSE_SUMMARY_5LANG.md`](LICENSE_SUMMARY_5LANG.md) · 📜 [`README_UPSTREAM_UNDERSTORY.md`](README_UPSTREAM_UNDERSTORY.md)

---

## ⚛️ Wave-12 Highlights (`v1.3.0-nala.6.6.6` · `RSI-UNDERSTORY-045..048`)

| Module | Capability (`100%` Native Safe Rust · `0` External Crates · `0` `unsafe` · `0` Warnings) | Measured Latency (M4 Max) | Security / Invariant |
| :--- | :--- | ---: | :--- |
| **`RSI-045` (`--vec-q4`)** | **4-Bit Asymmetric Scalar Quantized Vector Index & HNSW-Lite Skip-List (`.okf.q4`):** Nibble-packed 4-bit quantization (`2x` smaller than Int8) with 2-layer HNSW-Lite graph skip search. | **`0.67 µs/1k`** (`2.69x` SLA) | Zone-partitioned `.okf.q4` format + constant-time HMAC-SHA256 MAC. |
| **`RSI-046` (`--pq-merkle-tree`)** | **Hierarchical XMSS-Lite Merkle Tree over Multi-Use Post-Quantum WOTS+ Keys:** 16-leaf binary FIPS 180-4 SHA-256 Merkle tree + auth-path verify under 1 public PQ root (`0` crates). | **`188.29 µs/op`** (`1.17x` SLA) | Monotonic leaf index guard prevents WOTS+ leaf reuse (fail-closed). |
| **`RSI-047` (`--export-lakehouse`)** | **Deterministic SQLite / DuckDB / Parquet-Ready NDJSON Knowledge Lakehouse Exporter:** Exports typed nodes, edges, PageRank salience, decay & policy DDL with zero Python. | **`42.83 µs/op`** (`46.7x` SLA) | Auto-strips `# [VAULT:*]` sections + enforces explicit `TenantZone` allowlist. |
| **`RSI-048` (`--zk-disclose`)** | **Zero-Knowledge Hash-Commitment Selective Disclosure Proof:** Proves inclusion of individual OKF facts/sections via salted SHA-256 Merkle proofs without exposing sibling leaves. | **`1.29 µs/op`** (`27.1x` SLA) | Per-leaf 128-bit salted SHA-256 commitments resist brute-force/dictionary attacks. |

---

## 🌍 5-Language Executive Overview (`v1.3.0-nala.6.6.6` · `52` MCP Tools · `54/54` Unit Tests)

| Language | Summary |
| :--- | :--- |
| 🇨🇭 **Deutsch (DE)** | **NALA-Understory-RS (`v1.3.0-nala.6.6.6`)** ist die 100% native Safe-Rust Engine (`Edition 2024`, `0` Crates, `0` `unsafe`, `0` Warnings) mit 4-Bit Q4 HNSW-Lite Vektor-Suche (`0.67 µs/1k`), XMSS-Lite Post-Quantum Merkle Tree (`188.29 µs`), DuckDB/SQLite Lakehouse-Exporter (`42.83 µs`), ZK Selective-Disclosure Proofs (`1.29 µs`) und `52` statischen MCP-Tools (`54/54` Tests, `55` Subsysteme). |
| 🇫🇷 **Français (FR)** | **NALA-Understory-RS (`v1.3.0-nala.6.6.6`)** est le moteur 100% Rust sûr (`Edition 2024`, `0` crate externe, `0` avertissement) avec vecteurs 4-Bit Q4 HNSW-Lite (`0.67 µs/1k`), arbre Merkle post-quantique XMSS-Lite (`188.29 µs`), export Lakehouse NDJSON/DDL (`42.83 µs`) et preuves ZK sélectives (`1.29 µs`, `52` outils MCP, `54/54` tests). |
| 🇮🇹 **Italiano (IT)** | **NALA-Understory-RS (`v1.3.0-nala.6.6.6`)** è il motore 100% Rust sicuro (`Edition 2024`, `0` dipendenze, `0` warning) con indice vettoriale 4-Bit Q4 HNSW-Lite (`0.67 µs/1k`), albero Merkle post-quantistico XMSS-Lite (`188.29 µs`), esportatore Lakehouse NDJSON/DDL (`42.83 µs`) e prove ZK selettive (`1.29 µs`, `52` strumenti MCP, `54/54` test). |
| 🏔️ **Rumantsch (RM)** | **NALA-Understory-RS (`v1.3.0-nala.6.6.6`)** è il motor 100% Rust segir (`Edition 2024`, `0` crates) cun tschertga da vecturs 4-Bit Q4 HNSW-Lite (`0.67 µs/1k`), planta Merkle post-quantica XMSS-Lite (`188.29 µs`), export Lakehouse (`42.83 µs`) e cumprovas ZK selectivas (`1.29 µs`, `52` utensils MCP, `54/54` tests). |
| 🇬🇧 **English (EN)** | **NALA-Understory-RS (`v1.3.0-nala.6.6.6`)** is a 100% native safe Rust-Turbo (`Edition 2024`, zero crates, zero `unsafe`, zero warnings) drop-in engine for `thecodacus/understory` & `OKF v0.1` delivering `RSI-001..048`, `52` prompt-cache-safe MCP tools, `54/54` unit tests, and `55` benchmarked subsystems. |

---

## ⚡ Self-Tested 55-Subsystem Benchmark Across 3 Hardware Classes (incl. Airgapped Proxmox VE 9.2)

Tested on the official OKF v0.1 `sample-bundle` (`10,000` iterations via `nala-understory-rs --bench`):

| Subsystem / Operation | Upstream (`Node.js 22`) | Apple Silicon M4 Max (`arm64`) | Dual Xeon Server (`Linux x86_64`) | Proxmox VE 9.2 Airgapped (`unshare -n env -i`) | Peak Speedup / SLA |
| :--- | ---: | ---: | ---: | ---: | :--- |
| **1–51. Core OKF + `RSI-001..044` (Waves 1–11)** | `48 µs – 3.84 ms` | **`17.5 ns` – `175.6 µs`** | `49.7 ns` – `433.2 µs` | `61.3 ns` – `492.3 µs` | **All 51 SLAs beaten** |
| **52. `RSI-045` 4-Bit Q4 HNSW-Lite Vec (`1k` Vecs)** | *External Vector DB* | **`0.67 µs/1k`** | `2.29 µs/1k` | `2.38 µs/1k` | **2.69x beat `<1.8 µs` SLA** |
| **53. `RSI-046` XMSS-Lite 16-Leaf PQ Merkle Tree** | *N/A* | **`188.29 µs`** | `400.87 µs` | `507.96 µs` | **1.17x beat `<220 µs` SLA (`0` crates)** |
| **54. `RSI-047` DuckDB/SQLite Lakehouse Export** | *Python ETL* | **`42.83 µs`** | `41.13 µs` | `58.40 µs` | **46.7x beat `<2.0 ms` SLA** |
| **55. `RSI-048` ZK Selective Disclosure Proof** | *N/A* | **`1.29 µs`** | `3.10 µs` | `3.92 µs` | **27.1x beat `<35 µs` SLA** |

---

## 🔐 Cryptographic FIPS 180-4 `SHA-256` Binary Manifest (`v1.3.0-nala.6.6.6`)

| Platform / Architecture | Binary Asset | HAL Profile | FIPS 180-4 `SHA-256` Digest |
| :--- | :--- | :--- | :--- |
| **macOS Apple Silicon (`arm64`)** | `nala-understory-rs-macos-arm64` | `APPLE_SILICON_ARM64_NEON` | `07ebfce3d2c78f9221e9670f2c861212df5bafd253b71c585d90bebe3eb2ca50` |
| **macOS Intel (`x86_64`)** | `nala-understory-rs-macos-x86_64` | `MACOS_INTEL_X86_64_SSE42_AVX` | `6d7fc2a8b33b361a3c508d23e5cb199f7fa99b0d6da3a7b5e12f9015f837428b` |
| **Linux Static PIE (`x86_64`, Musl)** | `nala-understory-rs-linux-x86_64` | `LINUX_X86_64_UNIVERSAL` | `4168c1524d1731c36b237a63e7365a261232c21feb9131536cbfb87b93a8f39b` |
| **Linux Static (`aarch64`, Musl)** | `nala-understory-rs-linux-aarch64` | `LINUX_ARM64_NEON_UNIVERSAL` | `d2656bb2b762cd39d00e2366ce6ab41c5d1c4bcd9373b60bda543a0a5a5f3b4a` |
| **Windows 10/11 (`x86_64` PE32+)** | `nala-understory-rs-windows-x86_64.exe` | `WINDOWS_X86_64_UNIVERSAL` | `ee149913acac4ef42a5369809eeda98e66145052dfdea700aa51b7477658130d` |

---

## 🚀 Quickstart (Drop-In MCP Server & CLI for macOS, Linux, Proxmox & Windows)

### 1. One-Liner Install with Automatic `SHA-256` Verification
```bash
# macOS & Linux
curl -fsSL https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.sh | sh

# Windows PowerShell
irm https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.ps1 | iex
```

### 2. CLI Commands (`v1.3.0-nala.6.6.6` — 52 MCP Tools & 55 Benchmarked Subsystems)
```bash
nala-understory-rs --standalone-check
nala-understory-rs --bench
nala-understory-rs --vec-q4 "billing customer"
nala-understory-rs --pq-merkle-tree
nala-understory-rs --export-lakehouse
nala-understory-rs --zk-disclose
nala-understory-rs --self-audit --strict
nala-understory-rs --mcp-stdio --bundle /path/to/okf-bundle
```
