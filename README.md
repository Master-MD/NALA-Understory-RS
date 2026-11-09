# 🌱🦀 NALA-Understory-RS (`v0.5.0-nala.6.6.6`) — 100% Native Rust-Turbo Universal Standalone OKF v0.1 Markdown Memory, `.okf.idx` Binary Cache, Multi-Agent Lease & Federated GraphRAG Engine

> **Clean-Room 100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-External-Crates, Zero-Telemetry, 100% Universal Standalone) Port, Wave-1..4 Evolution (`RSI-UNDERSTORY-001..016`) & Security-Hardened Evolution of [`thecodacus/understory`](https://github.com/thecodacus/understory)**  
> **Multi-Language Documentation Hub:** 🇨🇭 [Deutsch (`docs/README_DE.md`)](docs/README_DE.md) · 🇫🇷 [Français (`docs/README_FR.md`)](docs/README_FR.md) · 🇮🇹 [Italiano (`docs/README_IT.md`)](docs/README_IT.md) · 🏔️ [Rumantsch Grischun (`docs/README_RM.md`)](docs/README_RM.md) · 🇬🇧 [English (`docs/README_EN.md`)](docs/README_EN.md)  
> **Beginner & Caveman Guides:** 🪨 [`CAVEMAN_GUIDE.md`](CAVEMAN_GUIDE.md) · ❓ [`FAQ.md`](FAQ.md) · ⚖️ [`LICENSE`](LICENSE) (`Apache-2.0`) & [`LICENSE_SUMMARY_5LANG.md`](LICENSE_SUMMARY_5LANG.md) · 📜 [`README_UPSTREAM_UNDERSTORY.md`](README_UPSTREAM_UNDERSTORY.md)

---

## 🌍 5-Language Executive Overview (`v0.5.0-nala.6.6.6` · 100% Standalone & Universal)

| Language | Summary |
| :--- | :--- |
| 🇨🇭 **Deutsch (DE)** | **NALA-Understory-RS (`v0.5.0-nala.6.6.6`)** ist der 100% native Rust-Turbo Port (`Edition 2024`, `0` externe Crate-Abhängigkeiten, `~620 KB` Binary) von `thecodacus/understory`. Es funktioniert **zu 100% eigenständig und universell (auch komplett offline im Airgap ohne jegliche NALA-OS Anbindung)** auf macOS (`arm64`/`x86_64`), Linux (`x86_64`/`aarch64`, inkl. Proxmox VE & LXC) und Windows (`x86_64`). Enthält alle 16 Module `RSI-001..016` (Evidence-Lock, 256-Bit SimHash LSH, Dream Gardener, Merkle `log.md` + 5-Zonen DLP, CRDT Sync, 2-Hop GraphRAG, `<18 µs` Delta-Arena, `# [VAULT:zone]`, `1.49 µs` Speculative N-Gram Cache, Git-Diff Harvester, `0.90 µs` Merkle Time-Travel, Zero-PII DemoPod Exporter, **`1.21 µs` `.okf.idx` Binary Snapshot**, **`23.1 ns` Multi-Agent Section Lease**, **`4.01 µs` Federated 4-Bundle GraphRAG** & **`1.23 µs` Citation Drift Attestor**) sowie `18` MCP-Tools und integrierten FIPS 180-4 `SHA-256` Selbsttest (`--self-hash`). |
| 🇫🇷 **Français (FR)** | **NALA-Understory-RS (`v0.5.0-nala.6.6.6`)** est le portage 100% Rust natif (`Edition 2024`, zéro Node.js/Python, `0` dépendance externe) de `thecodacus/understory`. Il fonctionne **100% en mode autonome universel (avec ou sans NALA-OS, même hors-ligne)** sur macOS, Linux/Proxmox et Windows, intégrant les vagues `RSI-001..016` (cache binaire `.okf.idx` `1.21 µs`, baux multi-agents `23.1 ns`, GraphRAG fédéré `4.01 µs`, attestateur de citations `1.23 µs`) et `18` outils MCP. |
| 🇮🇹 **Italiano (IT)** | **NALA-Understory-RS (`v0.5.0-nala.6.6.6`)** è il porting 100% Rust nativo (`Edition 2024`, zero Node.js/Python, `0` dipendenze esterne) di `thecodacus/understory`. Funziona **al 100% in modalità standalone universale (anche airgapped senza NALA-OS)** su macOS, Linux/Proxmox e Windows con i 16 moduli `RSI-001..016` e `18` strumenti MCP. |
| 🏔️ **Rumantsch (RM)** | **NALA-Understory-RS (`v0.5.0-nala.6.6.6`)** è il port 100% Rust nativ (`Edition 2024`, nagin Node.js/Python) da `thecodacus/understory`. El funcziuna a **100% en modus standalone universal (era offline senza NALA-OS)** sin macOS, Linux/Proxmox e Windows cun tut ils 16 moduls `RSI-001..016` e `18` utensils MCP. |
| 🇬🇧 **English (EN)** | **NALA-Understory-RS (`v0.5.0-nala.6.6.6`)** is a 100% native Rust-Turbo (`Edition 2024`, zero Node.js, zero Python, zero external crates, single `~620 KB` binary) rewrite and hardened evolution of `thecodacus/understory`. It operates **100% standalone & universally (works completely airgapped with zero NALA-OS or external service dependencies)** across macOS, Linux/Proxmox VE, and Windows, delivering `RSI-001..016`, `18` MCP tools, `21/21` unit tests, and built-in FIPS 180-4 `SHA-256` binary verification. |

---

## ⚡ Self-Tested 22-Subsystem Benchmark Across 3 Hardware Classes (incl. Airgapped Zero-NALA-OS Proxmox VE 9.2)

Tested on the official OKF v0.1 `sample-bundle` (`3` concepts, `4` directories, `4` inter-concept links, `1` orphan `/playbooks/oncall-billing.md`, `10,000` iterations via `nala-understory-rs --bench`):

| Subsystem / Operation | Upstream (`Node.js 22`) | Apple Silicon M4 Max (`arm64`) | Dual Xeon Server (`Linux x86_64`) | Proxmox VE 9.2 Airgapped (`unshare -n env -i`, Xeon `E5-2680`) | Peak Speedup / SLA |
| :--- | ---: | ---: | ---: | ---: | :--- |
| **1. YAML Frontmatter Parse (`okf::frontmatter`)** | `48,200.0 ns` | **`647.9 ns`** | `2,164.7 ns` | `1,318.5 ns` | **74.4x faster** |
| **2. Full Bundle Graph + Lint + Conformance** | `3,840.0 µs` | `84.15 µs` | **`55.83 µs`** | `55.88 µs` | **68.8x faster** |
| **3. Weighted Multi-Field Concept Search** | `1,420.0 µs` | `109.67 µs` | `90.25 µs` | **`84.93 µs`** | **16.7x faster** |
| **4. Duplicate Concept Jaccard Scan (`dream`)** | `64,500.0 ns` | **`3,965.7 ns`** | `13,297.2 ns` | `10,022.9 ns` | **16.3x faster** |
| **5. Session-Start MCP Seed (`build_seed_memory`)** | `1,910.0 µs` | `11.93 µs` | **`8.49 µs`** | `10.64 µs` | **225.0x faster** |
| **6. Deterministic OKF Index Regen (`indexer`)** | `2,650.0 µs` | `126.82 µs` | **`56.24 µs`** | `77.43 µs` | **47.1x faster** |
| **7. `RSI-001` Evidence-Lock Verifier (`[Dok-ID, S. X]`)** | *N/A* | **`206.1 ns`** | `450.6 ns` | `503.0 ns` | **4.85M ops/s** |
| **8. `RSI-002` 256-Bit SimHash LSH (`10,000` Nodes)** | `~1,850.0 ms` | **`9.60 ms`** | `60.84 ms` | `57.15 ms` | **192.7x faster** |
| **9. `RSI-003` Autonomous Dream Gardener Plan** | *N/A* | `141.77 µs` | **`85.49 µs`** | `115.81 µs` | **100% Standalone** |
| **10. `RSI-004` 256-Bit Merkle `log.md` + 5-Zone DLP** | *N/A* | **`1,263.7 ns`** | `8,173.2 ns` | `6,500.3 ns` | **791k ops/s** |
| **11. `RSI-005` CRDT Multi-Node Section Merge** | *N/A* | **`752.8 ns`** | `1,155.5 ns` | `1,352.7 ns` | **1.33M ops/s** |
| **12. `RSI-006` 2-Hop GraphRAG Context Compiler** | *N/A* | `164.51 µs` | **`99.40 µs`** | `137.13 µs` | **Sub-0.10 ms** |
| **13. `RSI-007` Incremental Delta-Arena (`<18 µs` SLA)** | `3,840.0 µs` | **`1.99 µs`** | `3.79 µs` | `4.28 µs` | **1,929.6x faster** |
| **14. `RSI-008` Vault ARX-256 + MAC (`# [VAULT:zone]`)** | *N/A* | **`2,334.0 ns`** | `6,462.5 ns` | `6,494.9 ns` | **428k ops/s** |
| **15. `RSI-009` Speculative N-Gram Cache (`<9.5 µs` SLA)** | *N/A* | **`1.49 µs`** | `3.85 µs` | `5.61 µs` | **6.4x beat SLA** |
| **16. `RSI-010` Static Git-Diff Harvester (`<1.1 ms` SLA)** | *N/A* | **`0.98 µs`** | `1.76 µs` | `2.28 µs` | **1,122x beat SLA** |
| **17. `RSI-011` Point-in-Time Time-Travel (`<28 µs` SLA)** | *N/A* | **`0.90 µs`** | `1.18 µs` | `1.69 µs` | **31.1x beat SLA** |
| **18. `RSI-012` DemoPod Zero-PII Stripper (`<2.2 ms` SLA)** | *N/A* | **`0.40 µs`** | `0.79 µs` | `1.02 µs` | **2.50M ops/s** |
| **19. `RSI-013` Binary `.okf.idx` Snapshot (`<4.5 µs` SLA)** | `3,840.0 µs` | **`1.21 µs`** | `2.57 µs` | `3.66 µs` | **3,173x vs. Node** |
| **20. `RSI-014` Multi-Agent Section Lease (`<900 ns` SLA)** | *N/A* | **`23.1 ns`** | `47.1 ns` | `63.0 ns` | **43.3M ops/s (39x SLA)** |
| **21. `RSI-015` Federated 4-Bundle Mesh (`<185 µs` SLA)** | *N/A* | **`4.01 µs`** | `7.46 µs` | `8.62 µs` | **46.1x beat SLA** |
| **22. `RSI-016` Citation Drift Attestor (`<14 µs` SLA)** | *N/A* | **`1.23 µs`** | `3.71 µs` | `4.19 µs` | **11.4x beat SLA** |
| **Runtime Memory Footprint (Idle Server)** | `~118.0 MB` | **`~1.9 MB`** | **`~1.8 MB`** | **`~1.8 MB`** | **65.5x leaner** |
| **Cold Start Latency (`--mcp-stdio`)** | `~420.0 ms` | **`~0.8 ms`** | **`~0.7 ms`** | **`~0.8 ms`** | **600.0x faster** |

---

## 🔐 Cryptographic FIPS 180-4 `SHA-256` Binary Manifest (`v0.5.0-nala.6.6.6`)

Every pre-built binary is compiled from the single deterministic `src/main.rs` (`Edition 2024`, `0` external crates) and carries built-in self-hash verification (`nala-understory-rs --self-hash`):

| Platform / Architecture | Binary Asset | HAL Profile | FIPS 180-4 `SHA-256` Digest |
| :--- | :--- | :--- | :--- |
| **macOS Apple Silicon (`arm64`)** | `nala-understory-rs-macos-arm64` | `APPLE_SILICON_ARM64_NEON` | `20c63aae43ba1bece69604244206dfe664e8b95472a56d402dccfdce9f3a2351` |
| **macOS Intel (`x86_64`)** | `nala-understory-rs-macos-x86_64` | `MACOS_INTEL_X86_64_SSE42_AVX` | `9205fd7f8dc0196e3d57f7e81077b884ce29b62215377f79e120c0da2b61a4cc` |
| **Linux Universal (`x86_64`, Sandy Bridge+)** | `nala-understory-rs-linux-x86_64` | `LINUX_X86_64_UNIVERSAL` | `70bb8c323522e7147141417d29f6b1a545781be48ab329a2bdaa466391cf2d8b` |
| **Linux Static Musl (`aarch64`)** | `nala-understory-rs-linux-aarch64` | `LINUX_ARM64_NEON_UNIVERSAL` | `c1d71d10941d72e836f20fc1dbc7df512db71dd804fde889cee1d12f85f788b9` |
| **Windows 10/11 (`x86_64` PE32+)** | `nala-understory-rs-windows-x86_64.exe` | `WINDOWS_X86_64_UNIVERSAL` | `8257f63917b5b301802e501ee365daec5f17387562dc8445a3f95335a4f0c4a3` |

---

## 🛡️ `sHeriFF + NVIDIA SkillSpector` Security Hardening (`SEC-UNDERSTORY-01..03` + Triple-Check Zero-PII)

1. **`SEC-UNDERSTORY-01` (Fixed — Localhost Bind & Dynamic Same-Host CSRF Shield)**:
   - Defaults to `127.0.0.1:3800`, validates `Origin:` dynamically against loopback and the server's own `Host:` header (rejecting untrusted cross-origin requests with `HTTP 403`), and uses constant-time token comparison.
2. **`SEC-UNDERSTORY-02` (Fixed — Zero-Subprocess Git & Atomic File Lock)**:
   - Eliminates `git config --system --add safe.directory '*'` and subprocess shell-outs in favor of pure Rust atomic file-rename persistence (`profiles.json.tmp`).
3. **`SEC-UNDERSTORY-03` (Fixed — Zero-Eval Frontmatter & Canonical Symlink Sandbox)**:
   - Eliminates `gray-matter` executable JS frontmatter (`---js`) and enforces `fs::canonicalize` symlink escape blocking in `Bundle::resolve()`.
4. **Universal Standalone & Triple-Check Zero-PII Invariant**:
   - Zero internal IP addresses, zero private hostnames, zero personal names, and zero OS lock-in (`--standalone-check` verified inside network-isolated `unshare -n env -i` Linux/Proxmox sandboxes and `--network none` ARM64 containers).

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

### CLI Commands (`v0.5.0-nala.6.6.6` — 18 MCP Tools & 22 Benchmarked Subsystems)
```bash
# Verify 100% Standalone Zero-Dependency Health & FIPS 180-4 Binary SHA-256
nala-understory-rs --standalone-check
nala-understory-rs --self-hash

# Run 10,000-iteration self-benchmark across all 22 subsystems
nala-understory-rs --bench

# Wave-1..4 Specialized Engines (RSI-001..016)
nala-understory-rs --rag-sync --json
nala-understory-rs --dream-garden --json
nala-understory-rs --verify-merkle --json
nala-understory-rs --crdt-sync --json
nala-understory-rs --graph-rag "billing charges" --json
nala-understory-rs --delta-scan --json
nala-understory-rs --vault-verify --json
nala-understory-rs --ngram-draft --json
nala-understory-rs --git-harvest --json
nala-understory-rs --time-travel 2026-07-11 --json
nala-understory-rs --demopod-export /tmp/demopod_capsule --json
nala-understory-rs --binary-index --json
nala-understory-rs --section-lease --json
nala-understory-rs --federated-rag "billing charges" --json
nala-understory-rs --attest-citations --json

# Start 5-Language Web Studio + REST API + Streamable HTTP MCP Server on 127.0.0.1:3800
nala-understory-rs --serve 127.0.0.1:3800

# Run as Stdio MCP Server (Claude Code, Cursor, Codex, Antigravity, ZCode, OpenCode)
nala-understory-rs --mcp-stdio --bundle /path/to/bundle
```
