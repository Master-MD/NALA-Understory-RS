# 🇫🇷 NALA-Understory-RS (`v0.5.0-nala.6.6.6`) — Documentation Française (FR)

> **Portage 100% Rust Natif (`Edition 2024`, Zéro Node.js, Zéro Python, Zéro Télémétrie, 100% Autonome Universel) et Évolution Sécurisée Vagues 1 à 4 (`RSI-UNDERSTORY-001..016`) de [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Qu'est-ce que `NALA-Understory-RS` ?
**NALA-Understory-RS (`v0.5.0-nala.6.6.6`)** transforme un dossier de fichiers Markdown bruts (**Open Knowledge Format / `OKF v0.1`**) en une **mémoire d'agent auto-maillée**, fonctionnant à **100% en mode autonome universel** sur macOS, Linux et Windows sans aucune dépendance externe :
- **`RSI-001..004` (Vague 1)** : Vérificateur Evidence-Lock `[Dok-ID, S. X]`, SimHash-256 LSH `O(N)`, Dream Gardener et chaîne Merkle-256 `log.md` + DLP 5-Zones (`vault_primary`, `vault_enterprise`, `vault_partner`, `shared`, `public_pearl`).
- **`RSI-005..008` (Vague 2)** : Synchronisation CRDT Multi-Nœuds (`node-alpha <-> node-beta <-> node-gamma`), compilateur GraphRAG à 2 sauts, Delta-Arène `< 18 µs` et coffres chiffrés `# [VAULT:zone]`.
- **`RSI-009..012` (Vague 3)** : Cache N-Gram spéculatif (`--ngram-draft`), moissonneur statique Git-Diff (`--git-harvest`), voyage temporel Merkle (`--time-travel`) et capsule DemoPod zéro-PII (`--demopod-export`).
- **`RSI-013..016` (Vague 4)** : Cache binaire `.okf.idx` aligné 64 octets avec MAC SHA-256 (`--binary-index`, `< 4.5 µs`), baux optimistes par section H1 (`--section-lease`, `< 900 ns`), maillage GraphRAG fédéré multi-bundles (`--federated-rag`, `< 185 µs`) et attestateur de dérive des citations (`--attest-citations`, `< 14 µs`).

## 2. Démarrage Rapide (`v0.5.0-nala.6.6.6`)
```bash
nala-understory-rs --standalone-check
nala-understory-rs --self-hash
nala-understory-rs --bench
nala-understory-rs --binary-index
nala-understory-rs --section-lease
nala-understory-rs --federated-rag "billing"
nala-understory-rs --attest-citations
nala-understory-rs --serve 127.0.0.1:3800
```
