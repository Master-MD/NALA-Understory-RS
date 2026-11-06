# 🇫🇷 NALA-Understory-RS (`v0.3.0-nala.6.6.6`) — Documentation Française (FR)

> **Portage 100% Rust Natif (`Edition 2024`, Zéro Node.js, Zéro Python, Zéro Télémétrie) et Évolution Sécurisée Wave-1 & Wave-2 (`RSI-UNDERSTORY-001..008`) de [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Qu'est-ce que `NALA-Understory-RS` ?
**NALA-Understory-RS (`v0.3.0-nala.6.6.6`)** transforme un simple dossier de fichiers Markdown bruts au format **Open Knowledge Format (`OKF v0.1`)** en une **mémoire d'agent auto-maillée et protégée contre la dérive documentaire** :
- **Scan Arène en Une Passe & Linter Anti-Dérive** : Remplace les 3 parcours disque distincts de Node.js par un scan mémoire unique (`79.9 µs` sur Apple M4 Max — jusqu'à **66.5x plus rapide**).
- **Modules d'Évolution `RSI-UNDERSTORY-001..008` Actifs** :
  - **`RSI-001` Evidence-Lock (`199.6 ns/op`)** : Vérification déterministe des citations `[Dok-ID, S. X]` et pont `NALA-RAG (:18700)`.
  - **`RSI-002` SimHash-256 LSH (`O(N)`)** : Déduplication de `10 000` concepts en `9.31 ms`.
  - **`RSI-003` Dream Gardener (`134.7 µs/op`)** : Maintenance autonome avec `USER_PROJECT_SHIELD`.
  - **`RSI-004` Chaîne Merkle-256 `log.md` & DLP 5-Zones (`1.29 µs/op`)** : Protection anti-injection et isolation multi-locataires (`sHeriFF`).
  - **`RSI-005` Synchronisation CRDT Multi-Nœuds (`740.5 ns/op`)** : Fusion sans conflit par section H1 entre `M4 Max <-> nala-brain <-> Beelink`.
  - **`RSI-006` Compilateur GraphRAG 2-Sauts (`162.3 µs/op`)** : Extraction de sous-graphe filtré par zone sous budget strict de caractères.
  - **`RSI-007` Delta-Arène Incrémentale (`2.01 µs/op`)** : Mise à jour d'index en `< 18 µs` par fichier modifié.
  - **`RSI-008` Sections Coffre-Fort Chiffrées (`# [VAULT:zone]`, `2.36 µs/op`)** : Chiffrement authentifié ARX-256 + MAC dans les fichiers Markdown OKF.

## 2. Démarrage Rapide
```bash
nala-understory-rs --bench
nala-understory-rs --crdt-sync --json
nala-understory-rs --graph-rag "billing charges" --json
nala-understory-rs --delta-scan --json
nala-understory-rs --vault-verify --json
nala-understory-rs --serve 127.0.0.1:3800
```
