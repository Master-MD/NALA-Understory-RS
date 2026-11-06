# 🌱🦀 NALA-Understory-RS (`v0.1.0-nala.6.6.6`) — 100% Native Rust-Turbo Self-Wiring OKF v0.1 Markdown Memory & Multi-Agent Knowledge Graph

> **Clean-Room 100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-Telemetry) Port, Acceleration & Security-Hardened Evolution of [`thecodacus/understory`](https://github.com/thecodacus/understory)**  
> **Multi-Language Documentation Hub:** 🇨🇭 [Deutsch (`docs/README_DE.md`)](docs/README_DE.md) · 🇫🇷 [Français (`docs/README_FR.md`)](docs/README_FR.md) · 🇮🇹 [Italiano (`docs/README_IT.md`)](docs/README_IT.md) · 🏔️ [Rumantsch Grischun (`docs/README_RM.md`)](docs/README_RM.md) · 🇬🇧 [English (`docs/README_EN.md`)](docs/README_EN.md)  
> **Beginner & Caveman Guides:** 🪨 [`CAVEMAN_GUIDE.md`](CAVEMAN_GUIDE.md) · ❓ [`FAQ.md`](FAQ.md) · ⚖️ [`LICENSE`](LICENSE) (`Apache-2.0`) & [`LICENSE_SUMMARY_5LANG.md`](LICENSE_SUMMARY_5LANG.md) · 📜 [`README_UPSTREAM_UNDERSTORY.md`](README_UPSTREAM_UNDERSTORY.md)

---

## 🌍 5-Language Executive Overview

| Language | Summary |
| :--- | :--- |
| 🇨🇭 **Deutsch (DE)** | **NALA-Understory-RS** ist der 100% native Rust-Turbo Port (`Edition 2024`, `0` externe Abhängigkeiten, `~430 KB` Binary) von `thecodacus/understory`. Es verwandelt reine Markdown-Ordner nach der **Open Knowledge Format (`OKF v0.1`)** Spezifikation in ein selbstvernetzendes, drift-geschütztes Agenten-Gedächtnis mit 3-stufigem Cache, `.traces/` Pfad-Replay, Multi-Profil-Mandantentrennung (`usk_...`), nativem MCP-Server (`stdio` & `/mcp` HTTP) und 5-sprachigem Web-Studio. |
| 🇫🇷 **Français (FR)** | **NALA-Understory-RS** est le portage 100% Rust natif (`Edition 2024`, zéro Node.js/Python) de `thecodacus/understory`. Il transforme des dossiers Markdown bruts (**OKF v0.1**) en une mémoire d'agent auto-maillée avec graphe de concepts en une seule passe, linter anti-dérive, cache à 3 niveaux, isolation multi-profils et serveur MCP natif. |
| 🇮🇹 **Italiano (IT)** | **NALA-Understory-RS** è il porting 100% Rust nativo (`Edition 2024`, zero Node.js/Python) di `thecodacus/understory`. Trasforma cartelle Markdown (**OKF v0.1**) in una memoria per agenti auto-collegata con grafo vivente a singolo passaggio, linter anti-deriva, cache a 3 livelli, profili isolati e server MCP nativo. |
| 🏔️ **Rumantsch (RM)** | **NALA-Understory-RS** è il port 100% Rust nativ (`Edition 2024`, nagin Node.js/Python) da `thecodacus/understory`. El transfurma ordinaturs da Markdown (**OKF v0.1**) en ina memoria d'agent auto-colliada cun graf da concepts en in sulet pass, linter cunter deriva, cache da 3 stgalims e server MCP nativ. |
| 🇬🇧 **English (EN)** | **NALA-Understory-RS** is a 100% native Rust-Turbo (`Edition 2024`, zero Node.js, zero Python, single `~430 KB` binary) rewrite and security-hardened evolution of `thecodacus/understory`. It turns plain **Open Knowledge Format (`OKF v0.1`)** Markdown bundles into a self-wiring, anti-drift agent memory with single-pass graph + linting, 3-tier hot/fingerprint caching, `.traces/` query-path replay, multi-profile isolation (`usk_...`), and a built-in MCP server (`stdio` + `/mcp`). |

---

## ⚡ Self-Tested Benchmark: Upstream Node.js/TypeScript vs. `NALA-Understory-RS` (10,000 Iterations)

Tested on the official OKF v0.1 `sample-bundle` (`3` concepts, `4` directories, `4` inter-concept links, `1` orphan `/playbooks/oncall-billing.md`, `10,000` iterations via `nala-understory-rs --bench`):

| Subsystem / Operation | Upstream (`Node.js 22 + gray-matter`) | `NALA-Understory-RS` (Apple M4 Max `arm64`) | `NALA-Understory-RS` (`nala-brain` Linux `x86_64`) | Peak Speedup | Architectural Reason |
| :--- | ---: | ---: | ---: | ---: | :--- |
| **1. YAML Frontmatter Parse (`okf::frontmatter`)** | `48,200.0 ns/op` | **`612.4 ns/op`** | `2,562.4 ns/op` | **78.7x faster** | Zero-allocation line scanner vs. `gray-matter` JS regex + eval parser |
| **2. Full Bundle Graph + Lint + Conformance** | `3,840.0 µs/op` | `105.3 µs/op` | **`57.7 µs/op`** | **66.5x faster** | Single-pass in-memory Arena (`scan_arena`) vs. 3x separate filesystem walks |
| **3. Weighted Multi-Field Concept Search** | `1,420.0 µs/op` | `151.6 µs/op` | **`89.1 µs/op`** | **15.9x faster** | Pre-normalized token slices (`title +10`, `path +6`, `desc/tags +5`, `body +2`) |
| **4. Duplicate Concept Jaccard Scan (`dream`)** | `64,500.0 ns/op` | **`3,873.4 ns/op`** | `15,003.3 ns/op` | **16.7x faster** | 64-bit register signature pre-filter + deterministic `BTreeSet` Jaccard |
| **5. Session-Start MCP Seed (`build_seed_memory`)** | `1,910.0 µs/op` | `14.9 µs/op` | **`11.5 µs/op`** | **166.4x faster** | Zero-copy slice formatting (`MAX_SEED_CHARS=3000`) |
| **6. Deterministic OKF Index Regen (`indexer`)** | `2,650.0 µs/op` | `183.7 µs/op` | **`82.8 µs/op`** | **32.0x faster** | Single-buffer Markdown table builder + parent chain propagation |
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

## 🧩 100% Feature Parity with Upstream + NALA Ecosystem Synergy

- **Open Knowledge Format (`OKF v0.1`) Engine**:
  - Concept documents with YAML frontmatter (`type` required; `description`, `owners`, `tags`, `sources`, `related`, `updated`, plus custom keys).
  - Auto-regenerated `index.md` per directory (`# <dir>/`, `# Subdirectories`, `# Concepts` Markdown table) + `prune_empty_dirs` (Upstream Issue #10 husk cleanup).
  - Append-only, newest-first `log.md` (`# Directory Update Log` grouped under `## YYYY-MM-DD`).
  - CommonMark ATX `replace_section()` with fenced code block (`` ``` `` / `~~~`) protection so `# comments` inside SQL/Python/Bash code blocks are never mistaken for headings.
- **3-Layer Query Cache + `.traces/` Replay**:
  - **Layer 1 (Exact Fingerprint Cache)**: 64-bit FNV-1a / SHA-256 query fingerprint LRU (`200` entries) -> `0 LLM calls`.
  - **Layer 2 (Hot Working Set)**: Top `10` concepts (`MAX_CONCEPTS=10`) + `10` recent Q&As (`MAX_QAS=10`) with automatic invalidation on any write/delete.
  - **Layer 3 (Deep Agent Loop)**: Local Ollama (`127.0.0.1:11434` / `:11435`), OpenAI-compatible, or Anthropic synthesis.
  - **`.traces/` Query-Path Replay**: Every read/write records compact step notation (`search "rate limit" (2) → read billing-api.md → ✓`).
- **Built-in MCP Server (`--mcp-stdio` & HTTP `/mcp`)**:
  - Exposes all 5 Understory MCP tools (`memory_query`, `memory_add`, `memory_update`, `memory_status`, `memory_maintain`) + automatic session-start `instructions` seed (`<= 3,000` chars).
- **Synergy with `NALA-RAG`, `NALA-Parallax-RS` & `NALA-Strata-RS`**:
  - Complements `NALA-RAG` (`:18700` Qdrant + SQLite FTS5 over 50k+ PDFs) as the **human-auditable, git-diffable Markdown working memory (`OKF v0.1`)** while routing deep maintenance/dream synthesis through `NALA-Parallax-RS` (`:4000`) and `NALA-Strata-RS`.

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

### CLI Commands
```bash
# Run 10,000-iteration self-benchmark on the OKF v0.1 sample bundle
nala-understory-rs --bench

# Inspect bundle status, Living Graph metrics, and OKF v0.1 conformance (DE/FR/IT/RM/EN)
nala-understory-rs --status --lang de

# Run Anti-Drift Linter (Orphans + Broken Inter-Concept Links)
nala-understory-rs --lint

# Run Dream-Phase Maintenance Analysis (Duplicate Jaccard + Oversized Concept Split Candidates)
nala-understory-rs --dream

# Start 5-Language Web Studio + REST API + Streamable HTTP MCP Server on 127.0.0.1:3800
nala-understory-rs --serve 127.0.0.1:3800

# Run as Stdio MCP Server (Claude Code, Cursor, Codex, Antigravity, ZCode, OpenCode)
nala-understory-rs --mcp-stdio --bundle /path/to/bundle
```
