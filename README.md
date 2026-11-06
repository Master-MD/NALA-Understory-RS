# 🌱🦀 NALA-Understory-RS (`v0.3.0-nala.6.6.6`) — 100% Native Rust-Turbo Self-Wiring OKF v0.1 Markdown Memory, 256-Bit SimHash LSH, CRDT Sync & 2-Hop GraphRAG Engine

> **Clean-Room 100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-Telemetry) Port, Wave-1 & Wave-2 Evolution (`RSI-UNDERSTORY-001..008`) & Security-Hardened Evolution of [`thecodacus/understory`](https://github.com/thecodacus/understory)**  
> **Multi-Language Documentation Hub:** 🇨🇭 [Deutsch (`docs/README_DE.md`)](docs/README_DE.md) · 🇫🇷 [Français (`docs/README_FR.md`)](docs/README_FR.md) · 🇮🇹 [Italiano (`docs/README_IT.md`)](docs/README_IT.md) · 🏔️ [Rumantsch Grischun (`docs/README_RM.md`)](docs/README_RM.md) · 🇬🇧 [English (`docs/README_EN.md`)](docs/README_EN.md)  
> **Beginner & Caveman Guides:** 🪨 [`CAVEMAN_GUIDE.md`](CAVEMAN_GUIDE.md) · ❓ [`FAQ.md`](FAQ.md) · ⚖️ [`LICENSE`](LICENSE) (`Apache-2.0`) & [`LICENSE_SUMMARY_5LANG.md`](LICENSE_SUMMARY_5LANG.md) · 📜 [`README_UPSTREAM_UNDERSTORY.md`](README_UPSTREAM_UNDERSTORY.md)

---

## 🌍 5-Language Executive Overview (`v0.3.0-nala.6.6.6`)

| Language | Summary |
| :--- | :--- |
| 🇨🇭 **Deutsch (DE)** | **NALA-Understory-RS (`v0.3.0-nala.6.6.6`)** ist der 100% native Rust-Turbo Port (`Edition 2024`, `0` externe Abhängigkeiten, `~539 KB` Binary) von `thecodacus/understory`. Es verwandelt reine Markdown-Ordner nach der **Open Knowledge Format (`OKF v0.1`)** Spezifikation in ein selbstvernetzendes, drift-geschütztes Agenten-Gedächtnis mit 3-stufigem Cache, `.traces/` Pfad-Replay, Multi-Profil-Mandantentrennung (`usk_...`), Wave-1 & Wave-2 Modulen (`RSI-001..008`: Evidence-Lock, 256-Bit SimHash LSH, Dream Gardener, 256-Bit Merkle `log.md` + 5-Zonen DLP, CRDT Multi-Node Sync, 2-Hop GraphRAG Compiler, `<18 µs` Incremental Delta-Arena & `# [VAULT:zone]` Verschlüsselung), nativem MCP-Server (`8` Tools via `stdio` & `/mcp` HTTP) und 5-sprachigem Web-Studio. |
| 🇫🇷 **Français (FR)** | **NALA-Understory-RS (`v0.3.0-nala.6.6.6`)** est le portage 100% Rust natif (`Edition 2024`, zéro Node.js/Python) de `thecodacus/understory`. Il transforme des dossiers Markdown bruts (**OKF v0.1**) en une mémoire d'agent auto-maillée avec graphe de concepts en une seule passe, linter anti-dérive, cache à 3 niveaux, synchronisation CRDT multi-nœuds, compilateur GraphRAG à 2 sauts, Delta-Arène `<18 µs`, coffres-forts `# [VAULT:zone]` et serveur MCP natif (`8` outils). |
| 🇮🇹 **Italiano (IT)** | **NALA-Understory-RS (`v0.3.0-nala.6.6.6`)** è il porting 100% Rust nativo (`Edition 2024`, zero Node.js/Python) di `thecodacus/understory`. Trasforma cartelle Markdown (**OKF v0.1**) in una memoria per agenti auto-collegata con grafo vivente a singolo passaggio, sincronizzazione CRDT multi-nodo, compilatore GraphRAG a 2 salti, Delta-Arena `<18 µs`, sezioni cifrate `# [VAULT:zone]` e server MCP nativo (`8` strumenti). |
| 🏔️ **Rumantsch (RM)** | **NALA-Understory-RS (`v0.3.0-nala.6.6.6`)** è il port 100% Rust nativ (`Edition 2024`, nagin Node.js/Python) da `thecodacus/understory`. El transfurma ordinaturs da Markdown (**OKF v0.1**) en ina memoria d'agent auto-colliada cun graf da concepts en in sulet pass, sincronisaziun CRDT multi-noda, compilatur GraphRAG da 2 sigls, Delta-Arena `<18 µs`, secziuns criptadas `# [VAULT:zone]` e server MCP nativ. |
| 🇬🇧 **English (EN)** | **NALA-Understory-RS (`v0.3.0-nala.6.6.6`)** is a 100% native Rust-Turbo (`Edition 2024`, zero Node.js, zero Python, single `~539 KB` binary) rewrite and security-hardened evolution of `thecodacus/understory`. It turns plain **Open Knowledge Format (`OKF v0.1`)** Markdown bundles into a self-wiring, anti-drift agent memory with single-pass graph + linting, 3-tier caching, `.traces/` replay, CRDT multi-node sync, 2-hop GraphRAG context compilation, `<18 µs` incremental delta updates, `# [VAULT:zone]` encrypted Markdown sections, and a built-in MCP server (`8` tools over `stdio` + `/mcp`). |

---

## ⚡ Self-Tested 14-Subsystem Benchmark: Upstream Node.js/TypeScript vs. `NALA-Understory-RS v0.3.0-nala.6.6.6` (10,000 Iterations)

Tested on the official OKF v0.1 `sample-bundle` (`3` concepts, `4` directories, `4` inter-concept links, `1` orphan `/playbooks/oncall-billing.md`, `10,000` iterations via `nala-understory-rs --bench`):

| Subsystem / Operation | Upstream (`Node.js 22 + gray-matter`) | `NALA-Understory-RS` (Apple M4 Max `arm64`) | `NALA-Understory-RS` (`nala-brain` Linux `x86_64`) | Peak Speedup | Architectural Reason |
| :--- | ---: | ---: | ---: | ---: | :--- |
| **1. YAML Frontmatter Parse (`okf::frontmatter`)** | `48,200.0 ns/op` | **`536.7 ns/op`** | `2,170.0 ns/op` | **89.8x faster** | Zero-allocation line scanner vs. `gray-matter` JS regex + eval parser |
| **2. Full Bundle Graph + Lint + Conformance** | `3,840.0 µs/op` | `79.86 µs/op` | **`57.34 µs/op`** | **67.0x faster** | Single-pass in-memory Arena (`scan_arena`) vs. 3x separate filesystem walks |
| **3. Weighted Multi-Field Concept Search** | `1,420.0 µs/op` | `106.97 µs/op` | **`92.88 µs/op`** | **15.3x faster** | Pre-normalized token slices (`title +10`, `path +6`, `desc/tags +5`, `body +2`) |
| **4. Duplicate Concept Jaccard Scan (`dream`)** | `64,500.0 ns/op` | **`3,936.7 ns/op`** | `15,148.5 ns/op` | **16.4x faster** | 64-bit register signature pre-filter + deterministic `BTreeSet` Jaccard |
| **5. Session-Start MCP Seed (`build_seed_memory`)** | `1,910.0 µs/op` | `12.59 µs/op` | **`11.78 µs/op`** | **162.1x faster** | Zero-copy slice formatting (`MAX_SEED_CHARS=3000`) |
| **6. Deterministic OKF Index Regen (`indexer`)** | `2,650.0 µs/op` | `124.39 µs/op` | **`83.52 µs/op`** | **31.7x faster** | Single-buffer Markdown table builder + parent chain propagation |
| **7. `RSI-001` Evidence-Lock Verifier (`[Dok-ID, S. X]`)** | *N/A (Missing)* | **`199.6 ns/op`** | `687.7 ns/op` | **5.01M ops/s** | Deterministic Rust citation & `Unbelegt-Rate=0` verifier + `NALA-RAG :18700` bridge |
| **8. `RSI-002` 256-Bit SimHash LSH (`10,000` Concepts)** | `~1,850.0 ms` ($O(N^2)$) | **`9.31 ms`** | `81.14 ms` | **198.7x faster** | $O(N)$ 4-band 256-bit SimHash LSH bucketing + POPCNT Hamming distance |
| **9. `RSI-003` Autonomous Dream Gardener Plan** | *N/A (Manual)* | `134.74 µs/op` | **`99.24 µs/op`** | **100% Local** | `USER_PROJECT_SHIELD` + NCSP pre-merge snapshot + orphan auto-wiring |
| **10. `RSI-004` 256-Bit Merkle `log.md` + 5-Zone DLP** | *N/A (Unsigned)* | **`1,291.8 ns/op`** | `9,456.1 ns/op` | **774k ops/s** | Tamper-evident `log.md` hash chain + `sHeriFF` Prompt-Injection & 5-Zone DLP guard |
| **11. `RSI-005` CRDT Multi-Node Section Merge** | *N/A (Missing)* | **`740.5 ns/op`** | `1,430.9 ns/op` | **1.35M ops/s** | Conflict-free H1 section LWW merge + 256-bit bundle Merkle root (`M4 Max <-> nala-brain <-> Beelink`) |
| **12. `RSI-006` 2-Hop GraphRAG Context Compiler** | *N/A (Missing)* | `162.27 µs/op` | **`121.47 µs/op`** | **Sub-0.17 ms** | Zone-filtered 2-hop subgraph packing within strict char/token budget |
| **13. `RSI-007` Incremental Delta-Arena (`<18 µs` SLA)** | `3,840.0 µs/op` | **`2.01 µs/op`** | `4.38 µs/op` | **1,910.4x faster** | Zero-polling `(mtime_ns, size_bytes, fnv1a_64)` single-file delta & parent `index.md` update |
| **14. `RSI-008` Vault ARX-256 + MAC (`# [VAULT:zone]`)** | *N/A (Plaintext)* | **`2,357.0 ns/op`** | `7,464.6 ns/op` | **424k ops/s** | Authenticated ARX-256 stream cipher + 256-bit keyed MAC inside OKF Markdown |
| **Runtime Memory Footprint (Idle Server)** | `~118.0 MB RSS` | **`~1.9 MB RSS`** | **`~1.8 MB RSS`** | **65.5x leaner** | Native Rust `std::net::TcpListener` vs. V8 + Express + React bundle |
| **Cold Start Latency (`--mcp-stdio`)** | `~420.0 ms` | **`~0.8 ms`** | **`~0.7 ms`** | **600.0x faster** | Native Mach-O / ELF / PE32+ binary without `npx`/`tsx` bootstrap |

---

## 🛡️ `sHeriFF + NVIDIA SkillSpector` Security Audit & Fixes (`SEC-UNDERSTORY-01..03`)

During our mandatory Clean-Room REA & Security Audit of upstream `thecodacus/understory`, three security vulnerabilities were identified and permanently fixed in `NALA-Understory-RS`:

1. **`SEC-UNDERSTORY-01` (Fixed — Localhost Bind & Strict Origin/Host CSRF Shield)**:
   - *Upstream Issue*: `packages/server/src/app.ts:29` configured `cors({ origin: true })` (reflecting any external Origin header) while `packages/server/src/index.ts:108` bound `0.0.0.0:3800` with `AUTH_TOKEN` unset by default. Any website opened in a local browser could silently query or overwrite agent memory (`POST /api/chat`, `POST /api/profiles`).
   - *NALA-Understory-RS Fix*: Defaults to `127.0.0.1:3800`, enforces strict `Origin:` host validation (rejecting external origins with `HTTP 403`), and uses constant-time `constant_time_eq` token comparison.
2. **`SEC-UNDERSTORY-02` (Fixed — Zero-Subprocess Git & Atomic File Lock)**:
   - *Upstream Issue*: `Dockerfile:20` executed `git config --system --add safe.directory '*'` and invoked subprocess git operations via `simple-git`.
   - *NALA-Understory-RS Fix*: 100% native Rust filesystem operations with atomic temp-file + rename persistence (`profiles.json.tmp`) and stale-lock recovery.
3. **`SEC-UNDERSTORY-03` (Fixed — Zero-Eval Frontmatter & Canonical Symlink Sandbox)**:
   - *Upstream Issue*: `gray-matter` supports executable JavaScript frontmatter (`---js`) by default, and string-only `path.resolve` checks can be bypassed if a symlink inside a bundle points outside the bundle root.
   - *NALA-Understory-RS Fix*: Pure data-only Rust YAML frontmatter parser (zero JS/script execution) + `fs::canonicalize` symlink escape verification in `Bundle::resolve()`.

---

## 🧩 100% Feature Parity with Upstream + Wave-1 & Wave-2 NALA Ecosystem Synergy

- **Open Knowledge Format (`OKF v0.1`) Engine**:
  - Concept documents with YAML frontmatter (`type` required; `description`, `owners`, `tags`, `sources`, `related`, `updated`, plus custom keys).
  - Auto-regenerated `index.md` per directory (`# <dir>/`, `# Subdirectories`, `# Concepts` Markdown table) + `prune_empty_dirs` (Upstream Issue #10 husk cleanup).
  - Append-only, newest-first `log.md` (`# Directory Update Log` grouped under `## YYYY-MM-DD`).
  - CommonMark ATX `replace_section()` with fenced code block (`` ``` `` / `~~~`) protection so `# comments` inside SQL/Python/Bash code blocks are never mistaken for headings.
- **Wave-1 (`RSI-001..004`) & Wave-2 (`RSI-005..008`) Subsystems**:
  - **`RSI-001`**: Bidirectional `NALA-RAG (:18700)` <-> `NALA-Understory-RS` Evidence-Lock Sync Bridge (`--evidence-sync`).
  - **`RSI-002`**: Zero-Copy `O(N)` 256-Bit SimHash LSH Graph Engine for `100,000+` concepts.
  - **`RSI-003`**: Autonomous Dream Gardener via `NALA-Parallax-RS (:4000)` & `USER_PROJECT_SHIELD` (`--gardener`).
  - **`RSI-004`**: Cryptographic 256-Bit Merkle `log.md` Chain & `sHeriFF` 5-Zone Tenant DLP (`--verify-merkle`).
  - **`RSI-005`**: Lock-Free CRDT Multi-Node OKF Memory Sync (`M4 Max <-> nala-brain <-> Beelink`) (`--crdt-sync`).
  - **`RSI-006`**: Sub-Millisecond 2-Hop GraphRAG Context Compiler (`--graph-rag <query>`).
  - **`RSI-007`**: Incremental Delta-Arena & Live Auto-Index Updater (`< 18 µs` SLA) (`--delta-scan`).
  - **`RSI-008`**: Encrypted-at-Rest Vault Sections (`# [VAULT:zone]`) inside OKF Markdown (`--vault-verify`).
- **Built-in MCP Server (`--mcp-stdio` & HTTP `/mcp` — 8 Tools)**:
  - Exposes all 5 Understory MCP tools (`memory_query`, `memory_add`, `memory_update`, `memory_status`, `memory_maintain`) + 3 Wave-2 tools (`memory_crdt_sync`, `memory_graph_rag`, `memory_vault_inspect`) + automatic session-start `instructions` seed (`<= 3,000` chars).

---

## 🚀 Quickstart (macOS, Linux, Windows)

### 1-Liner Install (macOS & Linux)
```bash
curl -fsSL https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.sh | sh
```

### 1-Liner Install (Windows PowerShell)
```powershell
irm https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.ps1 | iex
```

### CLI Commands (`v0.3.0-nala.6.6.6`)
```bash
# Run 10,000-iteration self-benchmark across all 14 subsystems
nala-understory-rs --bench

# Inspect bundle status, Living Graph metrics, and OKF v0.1 conformance (DE/FR/IT/RM/EN)
nala-understory-rs --status --lang de

# Run Wave-1 & Wave-2 Subsystems
nala-understory-rs --evidence-sync --json
nala-understory-rs --gardener --json
nala-understory-rs --verify-merkle --json
nala-understory-rs --crdt-sync --json
nala-understory-rs --graph-rag "billing charges" --json
nala-understory-rs --delta-scan --json
nala-understory-rs --vault-verify --json

# Start 5-Language Web Studio + REST API + Streamable HTTP MCP Server on 127.0.0.1:3800
nala-understory-rs --serve 127.0.0.1:3800

# Run as Stdio MCP Server (Claude Code, Cursor, Codex, Antigravity, ZCode, OpenCode)
nala-understory-rs --mcp-stdio --bundle /path/to/bundle
```
