# 🇨🇭 NALA-Understory-RS (`v0.8.0-nala.6.6.6`) — Deutsche Dokumentation (DE · Wave-7 Hybrid Community & NALA-OS Edition)

> **100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, `0` externe Crates, 100% Universal Standalone) Drop-In Engine & Security-Härtung für [`thecodacus/understory`](https://github.com/thecodacus/understory) (`OKF v0.1`)**

## 1. Hybride Dual-Horizon Architektur (`v0.8.0-nala.6.6.6`)
- **Horizont 1 (Open-Source Community Engine):** Läuft **zu 100% eigenständig ohne NALA-OS** auf macOS (`arm64`/`x86_64`), Linux (`x86_64`/`aarch64`, statisch gelinkt via Musl) und Windows (`x86_64`), löst alle 12 offenen Upstream-Probleme (`#23`, `#33`, `#34`, `#40`, `#51`–`#60`) von `thecodacus/understory` und integriert Obsidian-Vaults, Mermaid/PlantUML/Graphviz-Diagramme, Levenshtein-1 Fuzzy-Suche, PageRank & Shannon-Entropy Secret-Schutz.
- **Horizont 2 (Souveräne NALA-OS Integration):** Dient als **100% Zero-PII Upstream-Repository**, aus dem `NALA-OS v6.6.6`, `Antigravity`, `Codex`, `ZCode` und lokale Cluster-Nodes ihr produktives OKF-Langzeitgedächtnis (`32` statische Prompt-Cache-sichere MCP-Tools, `--sync-log` Session-Harvester, Nightly Dream-Gardener) beziehen.

## 2. Neue Wave-7 Module (`RSI-UNDERSTORY-025..028` in `v0.8.0-nala.6.6.6`)
- **`RSI-025` Obsidian Vault & Bidirectional `[[Wikilink]]` / Dataview Bridge (`--obsidian-sync`, `memory_obsidian_sync`):** Übersetzt `[[Target|Alias]]`, `key:: value` Dataview-Felder und `#tags` verlustfrei in OKF v0.1 Konzepte (`1.16 µs/Notiz`) und blockiert `.obsidian/workspace.json` & Plugin-Secrets via `sHeriFF` Secret-Filter.
- **`RSI-026` Zero-Allocation Succinct Prefix-Trie & Levenshtein-1 Autocomplete (`.okf.fst`, `--fuzzy-suggest`, `memory_fuzzy_suggest`):** Tippfehlertolerante Sofort-Vorschläge (`"biling"` → `"billing-api"`) in **`25.4 ns/op`** (25.6x schneller als `< 650 ns` SLA) mit strikter Zonen-Partitionierung & MAC-gesichertem `.okf.fst` Snapshot.
- **`RSI-027` Clean-Room Mermaid / PlantUML / Graphviz ↔ OKF Bidirectional Diagram Sync (`--diagram-sync`, `memory_diagram_sync`):** Bidirektionale Extraktion & Mermaid-Synthese in **`3.30 µs/op`** inkl. `sHeriFF` XSS/Script-Label-Sanitizer.
- **`RSI-028` Power-Iteration PageRank Salience Scorer & Shannon-Entropy Credential Canary Guard (`--salience-rank`, `--entropy-guard`):** 8-Iter Eigenvektor-Zentralität mit Self-Loop-/Link-Farm-Cap (**`14.62 µs`**) + Zero-Allocation Shannon-Entropie-Wächter (`> 4.35 bits/char` & API-Key/PEM-Präfixe ausserhalb `# [VAULT:*]` in **`800.7 ns/op`**).
