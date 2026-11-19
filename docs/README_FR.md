# 🇫🇷 NALA-Understory-RS (`v0.8.0-nala.6.6.6`) — Documentation Française (FR)

> **Moteur 100% Rust Natif (`Edition 2024`, Zéro Node.js, Zéro Python, `0` Crate Externe, 100% Autonome & Hybride) pour [`thecodacus/understory`](https://github.com/thecodacus/understory) (`OKF v0.1`)**

## Nouveautés Wave-7 `v0.8.0-nala.6.6.6` (`RSI-001..028` · `32` Outils MCP · `34/34` Tests)
- **`RSI-025` (`--obsidian-sync` / `memory_obsidian_sync`)**: Pont bidirectionnel Obsidian `[[Wikilink]]` / Dataview (`1.16 µs`) avec filtre anti-fuite `.obsidian/workspace`.
- **`RSI-026` (`--fuzzy-suggest` / `memory_fuzzy_suggest`)**: Auto-complétion préfixe & Levenshtein-1 sans allocation (`.okf.fst`, **`25.4 ns/op`**), isolée par zone.
- **`RSI-027` (`--diagram-sync` / `memory_diagram_sync`)**: Synchronisation bidirectionnelle Mermaid / PlantUML / Graphviz ↔ OKF (**`3.30 µs/op`**) avec assainisseur XSS.
- **`RSI-028` (`--salience-rank` & `--entropy-guard`)**: PageRank incrémental (**`14.62 µs`**) et garde d'entropie de Shannon (`> 4.35 bits/car`, **`800.7 ns/op`**).
