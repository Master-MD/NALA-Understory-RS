# 🌱🦀 NALA-Understory-RS (`v1.1.0-nala.6.6.6`) — Production 100% Native Rust Engine for `thecodacus/understory`, 1-Bit/Int8 Vector Search, Markov Prefetch & OKF v0.1 Agent Memory

> **Wave-10 (`v1.1.0`) Drop-in 100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, `0` External Crates, `0` Compiler Warnings, `~998 KB` Single Binary, 100% Universal Standalone) Rewrite & 10-Wave Evolution (`RSI-UNDERSTORY-001..040`) of [`thecodacus/understory`](https://github.com/thecodacus/understory)**  
> **44 Static Prompt-Cache-Safe MCP Tools · 46/46 Release Unit Tests (`0.03 s`) · 47 Benchmarked Subsystems · Solves Upstream `#23`, `#33`, `#34`, `#40`, `#51`–`#60` · Verified on Apple Silicon M4 Max, Linux x86_64/arm64, Airgapped Proxmox VE 9.2 (`unshare -n env -i`) & Windows 11!**  
> **Multi-Language Documentation Hub:** 🇨🇭 [Deutsch (`docs/README_DE.md`)](docs/README_DE.md) · 🇫🇷 [Français (`docs/README_FR.md`)](docs/README_FR.md) · 🇮🇹 [Italiano (`docs/README_IT.md`)](docs/README_IT.md) · 🏔️ [Rumantsch Grischun (`docs/README_RM.md`)](docs/README_RM.md) · 🇬🇧 [English (`docs/README_EN.md`)](docs/README_EN.md)  
> **Beginner & Caveman Guides:** 🪨 [`CAVEMAN_GUIDE.md`](CAVEMAN_GUIDE.md) · ❓ [`FAQ.md`](FAQ.md) · ⚖️ [`LICENSE`](LICENSE) (`Apache-2.0`) & [`LICENSE_SUMMARY_5LANG.md`](LICENSE_SUMMARY_5LANG.md) · 📜 [`README_UPSTREAM_UNDERSTORY.md`](README_UPSTREAM_UNDERSTORY.md)

---

## 🚀 Wave-10 Highlights (`v1.1.0-nala.6.6.6` · `RSI-UNDERSTORY-037..040`)

| Module | Capability (`100%` Native Safe Rust · `0` External Crates · `0` `unsafe` · `0` Warnings) | Measured Latency (M4 Max) | Security / Invariant |
| :--- | :--- | ---: | :--- |
| **`RSI-037` (`--vec-search`)** | **Binary 1-Bit / Int8 Quantized Semantic Vector Index (`.okf.vec`):** Hardware-`POPCNT` 256-bit Hamming shortlist + 32-dim Int8 dot-product rerank without external vector DBs. | **`2.84 µs / 1k`** (`4.23x` SLA) | Strict `TenantZone` isolation + constant-time HMAC-SHA256 MAC on `.okf.vec`. |
| **`RSI-038` (`--prefetch-markov`)** | **Predictive Markovian Concept Pre-Fetcher & Hot-Cache Warmer:** Learns transition probabilities from `.traces/` & link co-occurrences into L1-friendly CSR rows. | **`17.7 ns/op`** (`226x` SLA) | Strictly zone-bound transition matrix prevents cross-tenant inference leaks. |
| **`RSI-039` (`--precommit-guard`)** | **Deterministic 1-Binary Git Pre-Commit Diff Auditor:** Audits staged diffs for plaintext secrets, revDSG/GDPR PII, and broken internal `.md` backlinks without Node/Python/Husky. | **`0.45 µs/op`** (`55.6x` SLA) | Fail-closed exit code `1` before `git commit` completes. |
| **`RSI-040` (`--quorum-seal`)** | **Proof-Carrying Policy Certificate & Airgap `M-of-N` Quorum Multi-Sig Verifier:** Cryptographically binds Policy + CycloneDX/SLSA SBOM + Merkle Root with `2-of-3` / `M-of-N` signers. | **`6.99 µs/op`** (`8.58x` SLA) | Constant-time multi-key HMAC-SHA256 + duplicate-signer reject + anti-downgrade version lock. |

---

## 🌍 5-Language Executive Overview (`v1.1.0-nala.6.6.6` · `44` MCP Tools · `46/46` Unit Tests)

| Language | Summary |
| :--- | :--- |
| 🇨🇭 **Deutsch (DE)** | **NALA-Understory-RS (`v1.1.0-nala.6.6.6`)** ist die 100% native Rust-Turbo Engine (`Edition 2024`, `0` Crates, `0` Warnings, `998 KB` Single Binary) mit 1-Bit/Int8 Vektor-Suche (`2.84 µs/1k`), Markov-Prefetch (`17.7 ns`), 1-Binary Git Pre-Commit Guard (`0.45 µs`), M-of-N Quorum Policy-Zertifikat (`6.99 µs`) und `44` statischen MCP-Tools (`46/46` Tests, `47` Subsysteme). |
| 🇫🇷 **Français (FR)** | **NALA-Understory-RS (`v1.1.0-nala.6.6.6`)** est le moteur 100% Rust natif (`Edition 2024`, `0` crate externe, `0` avertissement) avec recherche vectorielle 1-Bit/Int8, pré-chargement Markov, audit pre-commit Git et certificat Quorum M-sur-N (`44` outils MCP, `46/46` tests, `RSI-001..040`). |
| 🇮🇹 **Italiano (IT)** | **NALA-Understory-RS (`v1.1.0-nala.6.6.6`)** è il motore 100% Rust nativo (`Edition 2024`, `0` dipendenze, `0` warning) con indice vettoriale 1-Bit/Int8, prefetch Markov, audit Git pre-commit e certificati Quorum M-di-N (`44` strumenti MCP, `46/46` test, `RSI-001..040`). |
| 🏔️ **Rumantsch (RM)** | **NALA-Understory-RS (`v1.1.0-nala.6.6.6`)** è il motor 100% Rust nativ (`Edition 2024`, `0` crates) cun tschertga vectoriala 1-Bit/Int8, prefetch Markov, guard pre-commit Git e certificat Quorum M-da-N (`44` utensils MCP, `46/46` tests, `RSI-001..040`). |
| 🇬🇧 **English (EN)** | **NALA-Understory-RS (`v1.1.0-nala.6.6.6`)** is a 100% native safe Rust-Turbo (`Edition 2024`, zero crates, zero warnings, `998 KB` binary) drop-in engine for `thecodacus/understory` & `OKF v0.1` delivering `RSI-001..040`, `44` prompt-cache-safe MCP tools, and `46/46` unit tests. |

---

## ⚡ Self-Tested 47-Subsystem Benchmark Across 3 Hardware Classes (incl. Airgapped Proxmox VE 9.2)

Tested on the official OKF v0.1 `sample-bundle` (`10,000` iterations via `nala-understory-rs --bench`):

| Subsystem / Operation | Upstream (`Node.js 22`) | Apple Silicon M4 Max (`arm64`) | Dual Xeon Server (`Linux x86_64`) | Proxmox VE 9.2 Airgapped (`unshare -n env -i`) | Peak Speedup / SLA |
| :--- | ---: | ---: | ---: | ---: | :--- |
| **1–39. Core OKF + `RSI-001..032` (Waves 1–8)** | `48 µs – 3.84 ms` | **`21.6 ns` – `336.9 µs`** | `47.3 ns` – `138.1 µs` | `61.3 ns` – `149.2 µs` | **All 39 SLAs beaten** |
| **40–43. `RSI-033..036` (Wave-9 LTS Decay/SSE/SBOM/Pol)** | *N/A* | **`295.8 ns` – `7.20 µs`** | `1.13 µs` – `25.20 µs` | `1.41 µs` – `37.53 µs` | **Up to 206x SLA** |
| **44. `RSI-037` 1-Bit/Int8 Vector Search (`1k` Vecs)** | *External DB* | **`2.84 µs / 1k`** | `7.40 µs / 1k` | `9.21 µs / 1k` | **4.23x beat `<12 µs` SLA** |
| **45. `RSI-038` Markov Concept Prefetcher** | *N/A* | **`17.7 ns`** | `64.1 ns` | `73.0 ns` | **226x beat `<4 µs` SLA** |
| **46. `RSI-039` 1-Binary Git Pre-Commit Diff Auditor** | *Node/Husky* | **`0.45 µs`** | `1.16 µs` | `1.45 µs` | **55.6x beat `<25 µs` SLA** |
| **47. `RSI-040` M-of-N Quorum Policy Certificate** | *N/A* | **`6.99 µs`** | `18.27 µs` | `21.91 µs` | **8.58x beat `<60 µs` SLA** |

---

## 🔐 Cryptographic FIPS 180-4 `SHA-256` Binary Manifest (`v1.1.0-nala.6.6.6`)

| Platform / Architecture | Binary Asset | HAL Profile | FIPS 180-4 `SHA-256` Digest |
| :--- | :--- | :--- | :--- |
| **macOS Apple Silicon (`arm64`)** | `nala-understory-rs-macos-arm64` | `APPLE_SILICON_ARM64_NEON` | `337c35d8a8bde4c39cf58ba650432fd4ab65239897b94bb7637aeb0ba3e98734` |
| **macOS Intel (`x86_64`)** | `nala-understory-rs-macos-x86_64` | `MACOS_INTEL_X86_64_SSE42_AVX` | `e52b603cc54d55b93eb9d75cba358950b57a54e4b86901c9d2625fdbebf3dc3a` |
| **Linux Static PIE (`x86_64`, Musl)** | `nala-understory-rs-linux-x86_64` | `LINUX_X86_64_UNIVERSAL` | `f76529e3e0ae7f0deaf7c7be1f4a305269a1bbd7dc3c3073cc63c8eac4c02fa4` |
| **Linux Static (`aarch64`, Musl)** | `nala-understory-rs-linux-aarch64` | `LINUX_ARM64_NEON_UNIVERSAL` | `1ba4353386d81b5a2982de7aa64cd1b911553f5b5538e591f901205111b7d662` |
| **Windows 10/11 (`x86_64` PE32+)** | `nala-understory-rs-windows-x86_64.exe` | `WINDOWS_X86_64_UNIVERSAL` | `0741179d656d7c3cfd2b90336057c9552bdad138e8b040779526c49126a864f4` |

---

## 🚀 Quickstart (Drop-In MCP Server & CLI for macOS, Linux, Proxmox & Windows)

### 1. One-Liner Install with Automatic `SHA-256` Verification
```bash
# macOS & Linux
curl -fsSL https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.sh | sh

# Windows PowerShell
irm https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.ps1 | iex
```

### 2. CLI Commands (`v1.1.0-nala.6.6.6` — 44 MCP Tools & 47 Benchmarked Subsystems)
```bash
nala-understory-rs --standalone-check
nala-understory-rs --bench
nala-understory-rs --vec-search "billing customer"
nala-understory-rs --prefetch-markov /tables/customers.md
nala-understory-rs --precommit-guard
nala-understory-rs --quorum-seal
nala-understory-rs --mcp-stdio --bundle /path/to/okf-bundle
```
