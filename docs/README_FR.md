# 🇫🇷 NALA-Understory-RS (`v0.7.0-nala.6.6.6`) — Documentation Française (FR)

> **Moteur 100% Rust Natif (`Edition 2024`, Zéro Node.js, Zéro Python, `0` Crate Externe, 100% Autonome & Hybride) pour [`thecodacus/understory`](https://github.com/thecodacus/understory) (`OKF v0.1`)**

## Nouveautés `v0.7.0-nala.6.6.6` (`RSI-001..024` · `27` Outils MCP · `30` Tests Unitaires)
- **`RSI-021` (`--rewrite-links` / `memory_rewrite_links`)**: Fusion atomique des doublons et réécriture automatique des liens entrants (`#56` & `#40`, `0` lien cassé).
- **`RSI-022` (`--patch-section` / `memory_patch_section`)**: Coercition automatique du frontmatter YAML/JSON pour modèles locaux (`#33`) et patch chirurgical des sections `H1`–`H6` avec garde anti-doublon `H1` (`#34`, `#52`, `#53`).
- **`RSI-023` (`--bm25-query` / `memory_query` / `memory_seed`)**: Recherche hybride BM25/IDF + boost de liens à 1 saut (`#51`, `#54`, `#60`) et schémas MCP 100% statiques (`listChanged: false`) préservant le cache de prompt (`#58`).
- **`RSI-024` (`--snapshot` / `--restore-snapshot` / `PUT /api/concept`)**: Snapshots atomiques SHA-256 dans `.okf_snapshots/` (`#59`) et éditeur Markdown en ligne dans le Web Studio (`#23`).
