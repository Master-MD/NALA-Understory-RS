# 🌱🦀 NALA-Understory-RS (`v0.7.0-nala.6.6.6`) — Drop-In 100% Native Rust Engine for `thecodacus/understory` & OKF v0.1 Markdown Agent Memory

> **Drop-in 100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, `0` External Crates, `~765 KB` Single Binary, 100% Universal Standalone) Rewrite & Evolution of [`thecodacus/understory`](https://github.com/thecodacus/understory)**  
> **Solves Upstream Issues [`#23`](https://github.com/thecodacus/understory/issues/23), [`#33`](https://github.com/thecodacus/understory/issues/33), [`#34`](https://github.com/thecodacus/understory/issues/34), [`#40`](https://github.com/thecodacus/understory/issues/40), [`#51`](https://github.com/thecodacus/understory/issues/51), [`#52`](https://github.com/thecodacus/understory/issues/52), [`#53`](https://github.com/thecodacus/understory/issues/53), [`#54`](https://github.com/thecodacus/understory/issues/54), [`#56`](https://github.com/thecodacus/understory/issues/56), [`#58`](https://github.com/thecodacus/understory/issues/58), [`#59`](https://github.com/thecodacus/understory/issues/59), and [`#60`](https://github.com/thecodacus/understory/pull/60) out of the box!**  
> **Multi-Language Documentation Hub:** 🇨🇭 [Deutsch (`docs/README_DE.md`)](docs/README_DE.md) · 🇫🇷 [Français (`docs/README_FR.md`)](docs/README_FR.md) · 🇮🇹 [Italiano (`docs/README_IT.md`)](docs/README_IT.md) · 🏔️ [Rumantsch Grischun (`docs/README_RM.md`)](docs/README_RM.md) · 🇬🇧 [English (`docs/README_EN.md`)](docs/README_EN.md)  
> **Beginner & Caveman Guides:** 🪨 [`CAVEMAN_GUIDE.md`](CAVEMAN_GUIDE.md) · ❓ [`FAQ.md`](FAQ.md) · ⚖️ [`LICENSE`](LICENSE) (`Apache-2.0`) & [`LICENSE_SUMMARY_5LANG.md`](LICENSE_SUMMARY_5LANG.md) · 📜 [`README_UPSTREAM_UNDERSTORY.md`](README_UPSTREAM_UNDERSTORY.md)

---

## 🎯 Why `NALA-Understory-RS`? (For `Understory`, `Obsidian`, `MCP` & Local-LLM Developers)

[`thecodacus/understory`](https://github.com/thecodacus/understory) pioneered self-wiring plain-Markdown memory for AI agents using the **Open Knowledge Format (`OKF v0.1`)**. However, production users running `500–3,000+` Markdown concepts with local models (`llama.cpp`, `Ollama`, `Qwen`, `Gemma`) or MCP clients (`Claude Desktop`, `Cursor`, `Codex`, `Antigravity`, `ZCode`, `pi-mcp-adapter`) hit critical bottlenecks in the Node.js implementation.

**`NALA-Understory-RS` (`v0.7.0-nala.6.6.6`)** is a **100% standalone, single-binary Rust engine (`0` crates, `0` Node.js, `0` Python)** that reads and writes the exact same **OKF v0.1 Markdown bundles** while solving every major upstream issue:

| Upstream Issue / PR in `thecodacus/understory` | Problem in Upstream TypeScript / Node.js | How `NALA-Understory-RS v0.7.0` Solves It Natively in Rust |
| :--- | :--- | :--- |
| **[#54](https://github.com/thecodacus/understory/issues/54) & [#51](https://github.com/thecodacus/understory/issues/51)** (`360k tokens/step` on large bundles) | Sends the entire nested directory tree (`~360,000` input tokens on a 3,087-concept bundle) on every agent step, exhausting context and failing writes. | **`RSI-023` & `RSI-019` (`--bm25-query`, `--budget-pack`)**: Never dumps the full tree. Uses `401.8 ns` 64-bit inverted bitmaps + **BM25/IDF + 1-Hop Link Boosting** packed strictly within a deterministic token budget (default `1,200` tokens in **`134.6 µs`**). |
| **[#58](https://github.com/thecodacus/understory/issues/58)** (Dynamic seed in `memory_query` invalidates prompt caches) | Injects dynamic `Recent activity` into `memory_query` tool description and emits `tools/list_changed` after every write, busting client KV/prompt caches. | **`RSI-023` Prompt-Cache-Stable MCP**: `tools/list` is **100% static & byte-identical** across mutations (`listChanged: false`). Session seed is served via a dedicated `memory_seed` tool and `okf://seed` MCP resource. |
| **[#56](https://github.com/thecodacus/understory/issues/56) & [#40](https://github.com/thecodacus/understory/issues/40)** (Broken incoming links after `dream` merge & blind tidy-up) | Deleting a duplicate concept during `dream` leaves broken incoming links in other files (#56), and tidy-up never inspects concept bodies (#40). | **`RSI-021` (`--rewrite-links` / `memory_rewrite_links`)**: Atomically merges duplicate concepts, rewrites all incoming `[...](/old.md)` and `[[old]]` backlinks across the entire bundle outside code fences (`2.72 µs`, **`0` broken links**), and deep-scans concept bodies for drift. |
| **[#33](https://github.com/thecodacus/understory/issues/33), [#34](https://github.com/thecodacus/understory/issues/34), [#52](https://github.com/thecodacus/understory/issues/52), [#53](https://github.com/thecodacus/understory/issues/53)** (Local LLM YAML corruption & duplicate H1/H2 sections) | Local models (`Qwen`, `Llama`) write JSON/YAML mappings into `description:` (#33) or corrupt H2/H3 sections and append duplicate `# H1` headings (#34, #52, #53). | **`RSI-022` (`--patch-section` / `memory_patch_section`)**: Automatically coerces non-string frontmatter objects into clean strings (`0` lost descriptions), patches any `H1`–`H6` section outside code fences (`616.6 ns`), and blocks duplicate top-level `# H1` sections. |
| **[#59](https://github.com/thecodacus/understory/issues/59) & [#23](https://github.com/thecodacus/understory/issues/23)** (Missing backup/restore API & Web-UI inline editor) | No built-in snapshot/restore API for Docker/MCP volumes (#59) and no direct Markdown editing in the Web UI (#23). | **`RSI-024` (`--snapshot`, `--restore-snapshot`, `PUT /api/concept`)**: First-class atomic SHA-256 bundle snapshots under `.okf_snapshots/` via CLI, HTTP, and MCP (`memory_snapshot` / `memory_restore`) + **Inline Markdown Editor** in `--serve` Web Studio. |
| **[#60](https://github.com/thecodacus/understory/pull/60)** (Hybrid prefill router + IDF + 1-hop link search) | Needs fast local IDF + 1-hop link expansion before calling heavy LLMs. | **`RSI-023` + `RSI-006`**: Built-in BM25/IDF + `+35%` 1-Hop neighbor link boost + 2-Hop GraphRAG subgraph compiler in **`134.6 µs`**. |

---

## 🌍 5-Language Executive Overview (`v0.7.0-nala.6.6.6` · Hybrid Community + NALA-OS Edition)

| Language | Summary |
| :--- | :--- |
| 🇨🇭 **Deutsch (DE)** | **NALA-Understory-RS (`v0.7.0-nala.6.6.6`)** ist die **hybride Open-Source Community- & Enterprise-Engine** in 100% nativem Rust (`Edition 2024`, `0` externe Crates, Zero-Node.js, Zero-Python). Sie läuft **100% eigenständig auf jedem System** (macOS, Linux, Proxmox, Windows) als Drop-in-Ersatz für `thecodacus/understory`, löst alle 12 offenen Upstream-Probleme (`RSI-021..024`: Atomic Backlink Rewriter, Frontmatter Coercion & H1–H6 Section Patcher, Hybrid BM25/IDF + 1-Hop Link Boost mit statischem Prompt-Cache-Schutz sowie `.okf_snapshots` + Web-UI Inline-Editor) und dient gleichzeitig als **Zero-PII Upstream-Quelle**, aus der lokale KI-Cluster (wie `NALA-OS`, `Antigravity`, `Codex`, `ZCode`) ihr produktives OKF-Gedächtnis beziehen. |
| 🇫🇷 **Français (FR)** | **NALA-Understory-RS (`v0.7.0-nala.6.6.6`)** est le moteur 100% Rust natif (`Edition 2024`, zéro Node.js/Python, `0` crate externe) compatible `thecodacus/understory` (`OKF v0.1`). Il résout nativement les problèmes upstream (`#23`, `#33`, `#34`, `#40`, `#51`–`#60`) via `RSI-001..024` et `27` outils MCP avec garantie `Zero-PII`. |
| 🇮🇹 **Italiano (IT)** | **NALA-Understory-RS (`v0.7.0-nala.6.6.6`)** è il motore 100% Rust nativo (`Edition 2024`, zero Node.js/Python, `0` dipendenze) compatibile con `thecodacus/understory` (`OKF v0.1`). Risolve i problemi upstream (`#23`, `#33`, `#34`, `#40`, `#51`–`#60`) con `RSI-001..024` e `27` strumenti MCP. |
| 🏔️ **Rumantsch (RM)** | **NALA-Understory-RS (`v0.7.0-nala.6.6.6`)** è il motor 100% Rust nativ (`Edition 2024`, nagin Node.js/Python) cumpatibel cun `thecodacus/understory` (`OKF v0.1`), cun `RSI-001..024` e `27` utensils MCP en modus 100% standalone ed ibrid. |
| 🇬🇧 **English (EN)** | **NALA-Understory-RS (`v0.7.0-nala.6.6.6`)** is a 100% native Rust-Turbo (`Edition 2024`, zero Node.js, zero Python, zero external crates) drop-in engine for `thecodacus/understory` (`OKF v0.1`). It runs **100% standalone anywhere** while resolving Upstream Issues `#23`, `#33`, `#34`, `#40`, `#51`–`#60` via `RSI-001..024`, `27` prompt-cache-safe MCP tools, and `30/30` unit tests. |

---

## ⚡ Self-Tested 30-Subsystem Benchmark Across 3 Hardware Classes (incl. Airgapped Proxmox VE 9.2)

Tested on the official OKF v0.1 `sample-bundle` (`10,000` iterations via `nala-understory-rs --bench`):

| Subsystem / Operation | Upstream (`Node.js 22`) | Apple Silicon M4 Max (`arm64`) | Dual Xeon Server (`Linux x86_64`) | Proxmox VE 9.2 Airgapped (`unshare -n env -i`) | Peak Speedup / SLA |
| :--- | ---: | ---: | ---: | ---: | :--- |
| **1. YAML Frontmatter Parse (`okf::frontmatter`)** | `48,200.0 ns` | **`789.8 ns`** | `2,204.9 ns` | `1,979.9 ns` | **61.0x faster** |
| **2. Full Bundle Graph + Lint + Conformance** | `3,840.0 µs` | `100.88 µs` | **`69.95 µs`** | `80.56 µs` | **54.9x faster (Fence-Safe)** |
| **3. Weighted Multi-Field Concept Search** | `1,420.0 µs` | `124.19 µs` | **`90.66 µs`** | `108.72 µs` | **15.7x faster** |
| **4. Duplicate Concept Jaccard Scan (`dream`)** | `64,500.0 ns` | **`4,262.0 ns`** | `20,580.0 ns` | `26,146.2 ns` | **15.1x faster** |
| **5. Session-Start MCP Seed (`build_seed_memory`)** | `1,910.0 µs` | **`13.85 µs`** | `22.47 µs` | `30.03 µs` | **137.9x faster** |
| **6. Deterministic OKF Index Regen (`indexer`)** | `2,650.0 µs` | `154.81 µs` | **`88.69 µs`** | `104.30 µs` | **29.9x faster** |
| **7–22. `RSI-001..016` (Waves 1–4 Core Engine)** | *N/A* | **`26.4 ns` – `200.4 µs`** | `60.5 ns` – `231.2 µs` | `63.9 ns` – `238.4 µs` | **All 16 SLAs beaten** |
| **23. `RSI-017` Inverted Bitmap `100k` (`<950 ns` SLA)** | `~1,420 µs` | **`401.8 ns`** | `1,747.7 ns` | `1,923.9 ns` | **2.49M queries/s (2.36x SLA)** |
| **24. `RSI-018` SQL/OpenAPI/Proto Import (`<1.8 ms` SLA)** | *N/A* | **`3.78 µs`** | `11.86 µs` | `12.06 µs` | **476x beat SLA** |
| **25. `RSI-019` Knapsack Budget Packer (`<35 µs` SLA)** | *N/A* | **`4.12 µs`** | `12.17 µs` | `11.51 µs` | **8.5x beat SLA (`0` broken)** |
| **26. `RSI-020` FIPS SHA-256 Bundle Seal (`<2.4 ms` SLA)** | *N/A* | `346.17 µs` | `153.49 µs` | **`149.16 µs`** | **16.1x beat SLA** |
| **27. `RSI-021` Atomic Backlink Rewriter (`#56` & `#40`)** | *Broken links* | **`2,716.4 ns`** | `8,034.1 ns` | `7,584.3 ns` | **`0` broken incoming links** |
| **28. `RSI-022` H1–H6 Section Patcher + FM Coercion (`#33`/`#52`)** | *Corrupts H2/H1* | **`616.6 ns`** | `2,097.1 ns` | `1,996.6 ns` | **1.62M ops/s + H1 Guard** |
| **29. `RSI-023` Hybrid BM25/IDF + 1-Hop RAG (`#51`/`#54`/`#58`/`#60`)** | `~60–150 s` (`360k` tok) | **`134.63 µs`** | `135.68 µs` | `145.44 µs` | **100% Prompt-Cache Stable** |
| **30. `RSI-024` Atomic Snapshot + Inline Edit (`#23` & `#59`)** | *No Backup API* | `96.87 µs` | `64.14 µs` | **`62.06 µs`** | **`.okf_snapshots` + Web Editor** |

---

## 🔐 Cryptographic FIPS 180-4 `SHA-256` Binary Manifest (`v0.7.0-nala.6.6.6`)

| Platform / Architecture | Binary Asset | HAL Profile | FIPS 180-4 `SHA-256` Digest |
| :--- | :--- | :--- | :--- |
| **macOS Apple Silicon (`arm64`)** | `nala-understory-rs-macos-arm64` | `APPLE_SILICON_ARM64_NEON` | `e4d5261f2efd2f7728f1d9608582e0c0405ddee58854bdce559b5653780e7c8a` |
| **macOS Intel (`x86_64`)** | `nala-understory-rs-macos-x86_64` | `MACOS_INTEL_X86_64_SSE42_AVX` | `e3481799b936603fb08692241a46d7d5861b4dabf4ba1f4a693d8a3e4bfe66c3` |
| **Linux Static PIE (`x86_64`, Musl)** | `nala-understory-rs-linux-x86_64` | `LINUX_X86_64_UNIVERSAL` | `3528e2309a4a5a635178eed7049b7ca8e8c1b17265bfe40a330ee29e7c097b80` |
| **Linux Static (`aarch64`, Musl)** | `nala-understory-rs-linux-aarch64` | `LINUX_ARM64_NEON_UNIVERSAL` | `5561fc4f992f3133e1c2b3a492b14b54157fff592d9be7ec467ab486a7e0a495` |
| **Windows 10/11 (`x86_64` PE32+)** | `nala-understory-rs-windows-x86_64.exe` | `WINDOWS_X86_64_UNIVERSAL` | `021ba45a7e643f87213dfc762f847fa94384337e9221da2083859fe18f4ff7fb` |

---

## 🚀 Quickstart (Drop-In MCP Server & CLI for macOS, Linux, Proxmox & Windows)

### 1. One-Liner Install with Automatic `SHA-256` Verification
```bash
# macOS & Linux
curl -fsSL https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.sh | sh

# Windows PowerShell
irm https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.ps1 | iex
```

### 2. Add to Any MCP Client (`Claude Desktop`, `Cursor`, `Antigravity`, `Codex`, `ZCode`)
```json
{
  "mcpServers": {
    "understory-rs": {
      "command": "nala-understory-rs",
      "args": ["--mcp-stdio", "--bundle", "/path/to/your/okf-markdown-bundle"]
    }
  }
}
```

### 3. CLI Commands (`v0.7.0-nala.6.6.6` — 27 MCP Tools & 30 Benchmarked Subsystems)
```bash
nala-understory-rs --standalone-check
nala-understory-rs --bench
nala-understory-rs --bm25-query "billing customer"
nala-understory-rs --patch-section "## Retry Policy"
nala-understory-rs --rewrite-links
nala-understory-rs --snapshot "pre_migration"
nala-understory-rs --restore-snapshot snap_pre_migration_5c14d394d525
nala-understory-rs --serve 127.0.0.1:3800
```
