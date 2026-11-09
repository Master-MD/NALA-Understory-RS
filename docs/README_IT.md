# 🇮🇹 NALA-Understory-RS (`v0.6.0-nala.6.6.6`) — Documentazione Italiana (IT)

> **Porting 100% Rust Nativo (`Edition 2024`, Zero Node.js, Zero Python, Zero Telemetria, 100% Standalone Universale) ed Evoluzione Blindata Ondate 1–5 (`RSI-UNDERSTORY-001..020`) di [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Panoramica (`v0.6.0-nala.6.6.6`)
**NALA-Understory-RS (`v0.6.0-nala.6.6.6`)** trasforma una cartella Markdown (**OKF v0.1**) in una memoria auto-collegata completamente autonoma su macOS, Linux e Windows:
- **`RSI-001..016` (Ondate 1–4)**: Evidence-Lock, SimHash-256 LSH, Dream Gardener, catena Merkle-256 `log.md` + DLP a 5 zone (`vault_primary..public_pearl`), CRDT Multi-Nodo (`node-alpha..node-gamma`), GraphRAG a 2 salti, Delta-Arena, `# [VAULT:zone]`, N-Gram speculativo, Git-Harvest, Time-Travel, DemoPod, snapshot `.okf.idx`, lease multi-agente, GraphRAG federato e attestatore citazioni.
- **`RSI-017..020` (Ondata 5)**: Ricerca booleana su bitmap invertite a 64 bit `.okf.bm` (`--bitmap-search`, `< 950 ns`), importatore statico SQL DDL / OpenAPI / Protobuf (`--import-schema`, `< 1.8 ms`), ottimizzatore Knapsack del contesto (`--budget-pack`, `< 35 µs`) e sigillo FIPS 180-4 SHA-256 `OKF_BUNDLE_SEAL.json` (`--seal-bundle`, `< 2.4 ms`).

## 2. Avvio Rapido (`22` Strumenti MCP & `26` Benchmark)
```bash
nala-understory-rs --standalone-check
nala-understory-rs --self-hash
nala-understory-rs --bench
nala-understory-rs --bitmap-search customer
nala-understory-rs --import-schema
nala-understory-rs --budget-pack "billing"
nala-understory-rs --seal-bundle
nala-understory-rs --serve 127.0.0.1:3800
```
