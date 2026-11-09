# 🇫🇷 NALA-Understory-RS (`v0.4.0-nala.6.6.6`) — Documentation Française (FR)

> **Portage 100% Rust Natif (`Edition 2024`, Zéro Node.js, Zéro Python, Zéro Télémétrie) et Évolution Sécurisée Vagues 1 à 3 (`RSI-UNDERSTORY-001..012`) de [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Qu'est-ce que `NALA-Understory-RS` ?
**NALA-Understory-RS (`v0.4.0-nala.6.6.6`)** transforme un simple dossier de fichiers Markdown bruts au format **Open Knowledge Format (`OKF v0.1`)** en une **mémoire d'agent auto-maillée et protégée contre la dérive documentaire** :
- **Modules d'Évolution `RSI-UNDERSTORY-001..012` Actifs** :
  - **`RSI-001..004` (Vague 1)** : Vérificateur Evidence-Lock `[Dok-ID, S. X]`, déduplication `O(N)` SimHash-256 LSH (`10 000` concepts), Dream Gardener (`USER_PROJECT_SHIELD`) et chaîne Merkle-256 `log.md` + DLP 5-Zones.
  - **`RSI-005..008` (Vague 2)** : Synchronisation CRDT Multi-Nœuds (`M4 Max <-> nala-brain <-> nala-worker`), compilateur GraphRAG à 2 sauts, Delta-Arène `< 18 µs` et coffres-forts chiffrés `# [VAULT:zone]`.
  - **`RSI-009` Pont de Cache Préfixe N-Gram Spéculatif (`--ngram-draft`)** : Table 64-bit isolée par zone (`~5.9 µs/op`) pour le décodage spéculatif des LLMs locaux (`NALA-Parallax-RS`).
  - **`RSI-010` Moissonneur Autonome Git-Diff $\rightarrow$ OKF (`--git-harvest`)** : Extraction statique sans exécution des signatures Rust/Swift/TS/Python modifiées (`~1.05 µs/op`).
  - **`RSI-011` Voyage Temporel Point-in-Time OKF (`--time-travel`)** : Reconstruction exacte du graphe à n'importe quelle date ou empreinte Merkle historique.
  - **`RSI-012` Exportateur de Capsule DemoPod Zéro-Connaissance (`--demopod-export`)** : Suppression physique de toutes les sections `# [VAULT:*]` avec certificat `PASS_ZERO_PII`.

## 2. Démarrage Rapide
```bash
nala-understory-rs --bench
nala-understory-rs --ngram-draft
nala-understory-rs --git-harvest
nala-understory-rs --time-travel 2026-07-11
nala-understory-rs --demopod-export /tmp/demopod_capsule
nala-understory-rs --serve 127.0.0.1:3800
```
