# 🏔️ NALA-Understory-RS (`v0.8.0-nala.6.6.6`) — Documentaziun Rumantscha (RM)

> **Motor 100% Rust Nativ (`Edition 2024`, Nagin Node.js, Nagin Python, `0` Crates Externs, 100% Standalone & Ibrid) per [`thecodacus/understory`](https://github.com/thecodacus/understory) (`OKF v0.1`)**

## Novitads Wave-7 `v0.8.0-nala.6.6.6` (`RSI-001..028` · `32` Utensils MCP · `34/34` Tests)
- **`RSI-025` (`--obsidian-sync` / `memory_obsidian_sync`)**: Punt bidirecziunala Obsidian `[[Wikilink]]` / Dataview (`1.16 µs`) cun filter da segrets `.obsidian`.
- **`RSI-026` (`--fuzzy-suggest` / `memory_fuzzy_suggest`)**: Autocompletaziun Levenshtein-1 senza allocaziun (`.okf.fst`, **`25.4 ns/op`**).
- **`RSI-027` (`--diagram-sync` / `memory_diagram_sync`)**: Sincronisaziun Mermaid / PlantUML / Graphviz ↔ OKF (**`3.30 µs/op`**) cun sanitader XSS.
- **`RSI-028` (`--salience-rank` & `--entropy-guard`)**: PageRank incremental (**`14.62 µs`**) e guardia d'entropia Shannon (`> 4.35 bits/car`, **`800.7 ns/op`**).
