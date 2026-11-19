# 🇮🇹 NALA-Understory-RS (`v0.8.0-nala.6.6.6`) — Documentazione Italiana (IT)

> **Motore 100% Rust Nativo (`Edition 2024`, Zero Node.js, Zero Python, `0` Dipendenze Esterne, 100% Standalone & Ibrido) per [`thecodacus/understory`](https://github.com/thecodacus/understory) (`OKF v0.1`)**

## Novità Wave-7 `v0.8.0-nala.6.6.6` (`RSI-001..028` · `32` Strumenti MCP · `34/34` Test)
- **`RSI-025` (`--obsidian-sync` / `memory_obsidian_sync`)**: Ponte bidirezionale Obsidian `[[Wikilink]]` / Dataview (`1.16 µs`) con filtro segreti `.obsidian/workspace`.
- **`RSI-026` (`--fuzzy-suggest` / `memory_fuzzy_suggest`)**: Autocompletamento zero-allocazione & Levenshtein-1 (`.okf.fst`, **`25.4 ns/op`**), partizionato per zona.
- **`RSI-027` (`--diagram-sync` / `memory_diagram_sync`)**: Sincronizzazione bidirezionale Mermaid / PlantUML / Graphviz ↔ OKF (**`3.30 µs/op`**) con sanitizzatore XSS.
- **`RSI-028` (`--salience-rank` & `--entropy-guard`)**: PageRank incrementale (**`14.62 µs`**) e guardia di entropia Shannon (`> 4.35 bit/car`, **`800.7 ns/op`**).
