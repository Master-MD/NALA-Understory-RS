# 🌱🦀 NALA-Understory-RS (`v0.6.0-nala.6.6.6`) — 100% Native Rust-Turbo Universal Standalone OKF v0.1 Markdown Memory, 511ns Bitmap Search, Schema Auto-Importer, Knapsack Packer & Airgap FIPS SHA-256 Engine

> **Clean-Room 100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-External-Crates, Zero-Telemetry, 100% Universal Standalone) Port, Wave-1..5 Evolution (`RSI-UNDERSTORY-001..020`) & Security-Hardened Evolution of [`thecodacus/understory`](https://github.com/thecodacus/understory)**  
> **Multi-Language Documentation Hub:** 🇨🇭 [Deutsch (`docs/README_DE.md`)](docs/README_DE.md) · 🇫🇷 [Français (`docs/README_FR.md`)](docs/README_FR.md) · 🇮🇹 [Italiano (`docs/README_IT.md`)](docs/README_IT.md) · 🏔️ [Rumantsch Grischun (`docs/README_RM.md`)](docs/README_RM.md) · 🇬🇧 [English (`docs/README_EN.md`)](docs/README_EN.md)  
> **Beginner & Caveman Guides:** 🪨 [`CAVEMAN_GUIDE.md`](CAVEMAN_GUIDE.md) · ❓ [`FAQ.md`](FAQ.md) · ⚖️ [`LICENSE`](LICENSE) (`Apache-2.0`) & [`LICENSE_SUMMARY_5LANG.md`](LICENSE_SUMMARY_5LANG.md) · 📜 [`README_UPSTREAM_UNDERSTORY.md`](README_UPSTREAM_UNDERSTORY.md)

---

## 🌍 5-Language Executive Overview (`v0.6.0-nala.6.6.6` · 100% Standalone & Universal)

| Language | Summary |
| :--- | :--- |
| 🇨🇭 **Deutsch (DE)** | **NALA-Understory-RS (`v0.6.0-nala.6.6.6`)** ist der 100% native Rust-Turbo Port (`Edition 2024`, `0` externe Crates, `~685 KB` Binary) von `thecodacus/understory`. Läuft **zu 100% eigenständig und universell (auch komplett offline im Airgap ohne jegliche NALA-OS Anbindung)** auf macOS (`arm64`/`x86_64`), Linux (`x86_64`/`aarch64`, inkl. Proxmox VE & LXC) und Windows (`x86_64`). Vereint alle 20 Evolutions-Module `RSI-001..020` (**`510.9 ns` 64-Bit Inverted Bitmap Search über 100k Konzepte**, **`4.08 µs` SQL DDL / OpenAPI / Protobuf Auto-Importer**, **`4.58 µs` Fence-Safe Knapsack Context Packer**, **`136.7 µs` FIPS 180-4 SHA-256 Airgap Bundle Seal**, `.okf.idx` Binary Snapshot, `21.3 ns` Multi-Agent Section Lease, Federated 4-Bundle GraphRAG, Citation Drift Attestor, Speculative N-Gram, Git-Harvest, Merkle Time-Travel & Zero-PII DemoPod Capsule) sowie `22` MCP-Tools. |
| 🇫🇷 **Français (FR)** | **NALA-Understory-RS (`v0.6.0-nala.6.6.6`)** est le portage 100% Rust natif (`Edition 2024`, zéro Node.js/Python, `0` crate externe) de `thecodacus/understory`. Fonctionne **100% en mode autonome universel (avec ou sans NALA-OS, même hors-ligne)** sur macOS, Linux/Proxmox et Windows avec les 20 modules `RSI-001..020` (recherche bitmap `510.9 ns`, importateur SQL/OpenAPI/Protobuf `4.08 µs`, optimiseur Knapsack `4.58 µs`, scellement FIPS SHA-256 `136.7 µs`) et `22` outils MCP. |
| 🇮🇹 **Italiano (IT)** | **NALA-Understory-RS (`v0.6.0-nala.6.6.6`)** è il porting 100% Rust nativo (`Edition 2024`, zero Node.js/Python, `0` dipendenze esterne) di `thecodacus/understory`. Opera **al 100% in modalità standalone universale (anche airgapped senza NALA-OS)** su macOS, Linux/Proxmox e Windows con tutti i 20 moduli `RSI-001..020` e `22` strumenti MCP. |
| 🏔️ **Rumantsch (RM)** | **NALA-Understory-RS (`v0.6.0-nala.6.6.6`)** è il port 100% Rust nativ (`Edition 2024`, nagin Node.js/Python) da `thecodacus/understory`. Funcziuna a **100% en modus standalone universal (era offline senza NALA-OS)** sin macOS, Linux/Proxmox e Windows cun tut ils 20 moduls `RSI-001..020` e `22` utensils MCP. |
| 🇬🇧 **English (EN)** | **NALA-Understory-RS (`v0.6.0-nala.6.6.6`)** is a 100% native Rust-Turbo (`Edition 2024`, zero Node.js, zero Python, zero external crates, single `~685 KB` binary) rewrite and hardened evolution of `thecodacus/understory`. It runs **100% standalone & universally (works completely airgapped with zero NALA-OS or external daemon dependencies)** across macOS, Linux/Proxmox VE, and Windows, delivering `RSI-001..020`, `22` MCP tools, `25/25` unit tests, and built-in FIPS 180-4 `SHA-256` verification. |

---

## ⚡ Self-Tested 26-Subsystem Benchmark Across 3 Hardware Classes (incl. Airgapped Zero-NALA-OS Proxmox VE 9.2)

Tested on the official OKF v0.1 `sample-bundle` (`3` concepts, `4` directories, `4` inter-concept links, `1` orphan `/playbooks/oncall-billing.md`, `10,000` iterations via `nala-understory-rs --bench`):

| Subsystem / Operation | Upstream (`Node.js 22`) | Apple Silicon M4 Max (`arm64`) | Dual Xeon Server (`Linux x86_64`) | Proxmox VE 9.2 Airgapped (`unshare -n env -i`, Xeon `E5-2680`) | Peak Speedup / SLA |
| :--- | ---: | ---: | ---: | ---: | :--- |
| **1. YAML Frontmatter Parse (`okf::frontmatter`)** | `48,200.0 ns` | **`628.8 ns`** | `1,890.2 ns` | `1,292.2 ns` | **76.7x faster** |
| **2. Full Bundle Graph + Lint + Conformance** | `3,840.0 µs` | `113.87 µs` | `60.99 µs` | **`55.51 µs`** | **69.2x faster** |
| **3. Weighted Multi-Field Concept Search** | `1,420.0 µs` | `198.02 µs` | `95.73 µs` | **`82.84 µs`** | **17.1x faster** |
| **4. Duplicate Concept Jaccard Scan (`dream`)** | `64,500.0 ns` | **`7,799.8 ns`** | `15,064.6 ns` | `10,535.9 ns` | **8.3x faster** |
| **5. Session-Start MCP Seed (`build_seed_memory`)** | `1,910.0 µs` | `73.96 µs` | `12.50 µs` | **`11.24 µs`** | **169.9x faster** |
| **6. Deterministic OKF Index Regen (`indexer`)** | `2,650.0 µs` | `367.23 µs` | `87.48 µs` | **`83.28 µs`** | **31.8x faster** |
| **7. `RSI-001` Evidence-Lock Verifier (`[Dok-ID, S. X]`)** | *N/A* | **`288.5 ns`** | `701.1 ns` | `507.6 ns` | **3.47M ops/s** |
| **8. `RSI-002` 256-Bit SimHash LSH (`10,000` Nodes)** | `~1,850.0 ms` | **`22.60 ms`** | `88.97 ms` | `55.84 ms` | **81.9x faster** |
| **9. `RSI-003` Autonomous Dream Gardener Plan** | *N/A* | `174.33 µs` | **`106.77 µs`** | `115.02 µs` | **100% Standalone** |
| **10. `RSI-004` 256-Bit Merkle `log.md` + 5-Zone DLP** | *N/A* | **`1,574.5 ns`** | `9,860.9 ns` | `6,584.6 ns` | **635k ops/s** |
| **11. `RSI-005` CRDT Multi-Node Section Merge** | *N/A* | **`1,028.4 ns`** | `1,484.7 ns` | `1,516.8 ns` | **972k ops/s** |
| **12. `RSI-006` 2-Hop GraphRAG Context Compiler** | *N/A* | `355.27 µs` | **`130.74 µs`** | `139.57 µs` | **Sub-0.14 ms** |
| **13. `RSI-007` Incremental Delta-Arena (`<18 µs` SLA)** | `3,840.0 µs` | **`2.58 µs`** | `4.43 µs` | `4.22 µs` | **1,488x faster** |
| **14. `RSI-008` Vault ARX-256 + MAC (`# [VAULT:zone]`)** | *N/A* | **`3,186.2 ns`** | `8,213.5 ns` | `6,663.1 ns` | **314k ops/s** |
| **15. `RSI-009` Speculative N-Gram Cache (`<9.5 µs` SLA)** | *N/A* | **`1.90 µs`** | `5.36 µs` | `5.82 µs` | **5.0x beat SLA** |
| **16. `RSI-010` Static Git-Diff Harvester (`<1.1 ms` SLA)** | *N/A* | **`1.25 µs`** | `2.15 µs` | `2.30 µs` | **880x beat SLA** |
| **17. `RSI-011` Point-in-Time Time-Travel (`<28 µs` SLA)** | *N/A* | **`1.12 µs`** | `1.46 µs` | `1.80 µs` | **25.0x beat SLA** |
| **18. `RSI-012` DemoPod Zero-PII Stripper (`<2.2 ms` SLA)** | *N/A* | **`0.65 µs`** | `1.01 µs` | `0.98 µs` | **1.54M ops/s** |
| **19. `RSI-013` Binary `.okf.idx` Snapshot (`<4.5 µs` SLA)** | `3,840.0 µs` | **`1.96 µs`** | `3.00 µs` | `3.70 µs` | **1,959x vs. Node** |
| **20. `RSI-014` Multi-Agent Section Lease (`<900 ns` SLA)** | *N/A* | **`27.0 ns`** | `51.4 ns` | `61.9 ns` | **37.0M ops/s (33x SLA)** |
| **21. `RSI-015` Federated 4-Bundle Mesh (`<185 µs` SLA)** | *N/A* | **`6.08 µs`** | `9.04 µs` | `8.88 µs` | **30.4x beat SLA** |
| **22. `RSI-016` Citation Drift Attestor (`<14 µs` SLA)** | *N/A* | **`1.70 µs`** | `4.46 µs` | `4.25 µs` | **8.2x beat SLA** |
| **23. `RSI-017` Inverted Bitmap `100k` (`<950 ns` SLA)** | `~1,420 µs` | **`510.9 ns`** | `2,144.0 ns` | `2,196.4 ns` | **1.96M/s (1.86x SLA)** |
| **24. `RSI-018` SQL/OpenAPI/Proto Import (`<1.8 ms` SLA)** | *N/A* | **`4.08 µs`** | `7.54 µs` | `7.34 µs` | **441x beat SLA** |
| **25. `RSI-019` Knapsack Budget Packer (`<35 µs` SLA)** | *N/A* | **`4.58 µs`** | `7.91 µs` | `7.78 µs` | **7.6x beat SLA (`0` broken)** |
| **26. `RSI-020` FIPS SHA-256 Bundle Seal (`<2.4 ms` SLA)** | *N/A* | `275.30 µs` | **`136.70 µs`** | `144.28 µs` | **17.6x beat SLA** |

---

## 🔐 Cryptographic FIPS 180-4 `SHA-256` Binary Manifest (`v0.6.0-nala.6.6.6`)

| Platform / Architecture | Binary Asset | HAL Profile | FIPS 180-4 `SHA-256` Digest |
| :--- | :--- | :--- | :--- |
| **macOS Apple Silicon (`arm64`)** | `nala-understory-rs-macos-arm64` | `APPLE_SILICON_ARM64_NEON` | `56bbd50fd0a5e0eede967d1a4406928a308f0128541bee5acdb717eb5ddffd4e` |
| **macOS Intel (`x86_64`)** | `nala-understory-rs-macos-x86_64` | `MACOS_INTEL_X86_64_SSE42_AVX` | `7006fe385a62085496806fce9b0374d7def6f9b0f20c372d0bd8c2e5e6c03d15` |
| **Linux Universal (`x86_64`, Sandy Bridge+)** | `nala-understory-rs-linux-x86_64` | `LINUX_X86_64_UNIVERSAL` | `965429764158c0f0571eb96ce50890d516ab6d9f1f90c2c09f0ca91d5b4927f6` |
| **Linux Static Musl (`aarch64`)** | `nala-understory-rs-linux-aarch64` | `LINUX_ARM64_NEON_UNIVERSAL` | `52c20f89538c05a6b3942ac0f5a1b89ab7baec2bc22a819dd1e0baf181e71b64` |
| **Windows 10/11 (`x86_64` PE32+)** | `nala-understory-rs-windows-x86_64.exe` | `WINDOWS_X86_64_UNIVERSAL` | `f8be1b676b25dcf88e39b54b8d5bdfad69f5f7bbb3442e2d14bff22dbf9cc0f4` |

---

## 🚀 Quickstart (macOS, Linux, Proxmox, Windows)

### 1-Liner Install with Automatic `SHA-256` Verification (macOS & Linux)
```bash
curl -fsSL https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.sh | sh
```

### 1-Liner Install with Automatic `SHA-256` Verification (Windows PowerShell)
```powershell
irm https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.ps1 | iex
```

### CLI Commands (`v0.6.0-nala.6.6.6` — 22 MCP Tools & 26 Benchmarked Subsystems)
```bash
nala-understory-rs --standalone-check
nala-understory-rs --self-hash
nala-understory-rs --bench
nala-understory-rs --bitmap-search customer --json
nala-understory-rs --import-schema --json
nala-understory-rs --budget-pack "billing charges" --json
nala-understory-rs --seal-bundle --json
nala-understory-rs --serve 127.0.0.1:3800
nala-understory-rs --mcp-stdio --bundle /path/to/bundle
```
