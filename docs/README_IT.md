# 🇮🇹 NALA-Understory-RS (`v0.7.0-nala.6.6.6`) — Documentazione Italiana (IT)

> **Motore 100% Rust Nativo (`Edition 2024`, Zero Node.js, Zero Python, `0` Dipendenze Esterne, 100% Standalone & Ibrido) per [`thecodacus/understory`](https://github.com/thecodacus/understory) (`OKF v0.1`)**

## Novità `v0.7.0-nala.6.6.6` (`RSI-001..024` · `27` Strumenti MCP · `30` Test Unitari)
- **`RSI-021` (`--rewrite-links` / `memory_rewrite_links`)**: Unione atomica dei duplicati e riscrittura automatica dei backlink in entrata (`#56` & `#40`, `0` link interrotti).
- **`RSI-022` (`--patch-section` / `memory_patch_section`)**: Sanificazione automatica frontmatter YAML/JSON per modelli locali (`#33`) e modifica chirurgica delle sezioni `H1`–`H6` con blocco dei duplicati `# H1` (`#34`, `#52`, `#53`).
- **`RSI-023` (`--bm25-query` / `memory_query` / `memory_seed`)**: Ricerca ibrida BM25/IDF + boost 1-Hop (`#51`, `#54`, `#60`) e descrizioni MCP statiche (`listChanged: false`) che preservano la prompt-cache (`#58`).
- **`RSI-024` (`--snapshot` / `--restore-snapshot` / `PUT /api/concept`)**: Snapshot atomici SHA-256 in `.okf_snapshots/` (`#59`) ed editor Markdown inline nella Web UI (`#23`).
