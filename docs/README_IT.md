# 🇮🇹 NALA-Understory-RS (`v0.5.0-nala.6.6.6`) — Documentazione Italiana (IT)

> **Porting 100% Rust Nativo (`Edition 2024`, Zero Node.js, Zero Python, Zero Telemetria, 100% Standalone Universale) ed Evoluzione Blindata Ondate 1–4 (`RSI-UNDERSTORY-001..016`) di [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Cos'è `NALA-Understory-RS`?
**NALA-Understory-RS (`v0.5.0-nala.6.6.6`)** trasforma una cartella di file Markdown standard (**Open Knowledge Format / `OKF v0.1`**) in una **memoria per agenti IA auto-collegata**, completamente autonoma su macOS, Linux e Windows senza alcuna dipendenza esterna:
- **`RSI-001..004`**: Evidence-Lock, SimHash-256 LSH `O(N)`, Dream Gardener, catena Merkle-256 `log.md` e DLP a 5 zone (`vault_primary`, `vault_enterprise`, `vault_partner`, `shared`, `public_pearl`).
- **`RSI-005..008`**: Sincronizzazione CRDT Multi-Nodo (`node-alpha <-> node-beta <-> node-gamma`), GraphRAG a 2 salti, Delta-Arena `< 18 µs` e sezioni cifrate `# [VAULT:zone]`.
- **`RSI-009..012`**: Tabella speculativa N-Gram a 64 bit (`--ngram-draft`), harvester statico Git-Diff (`--git-harvest`), Time-Travel Merkle (`--time-travel`) ed esportatore DemoPod Zero-PII (`--demopod-export`).
- **`RSI-013..016`**: Snapshot binario `.okf.idx` con SHA-256 MAC (`--binary-index`, `< 4.5 µs`), lease multi-agente per sezioni H1 (`--section-lease`, `< 900 ns`), GraphRAG federato cross-bundle (`--federated-rag`, `< 185 µs`) e attestatore di deriva delle citazioni (`--attest-citations`, `< 14 µs`).

## 2. Avvio Rapido (`v0.5.0-nala.6.6.6`)
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
