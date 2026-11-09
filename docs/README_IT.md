# 🇮🇹 NALA-Understory-RS (`v0.4.0-nala.6.6.6`) — Documentazione Italiana (IT)

> **Porting 100% Rust Nativo (`Edition 2024`, Zero Node.js, Zero Python, Zero Telemetria) ed Evoluzione Blindata Ondate 1–3 (`RSI-UNDERSTORY-001..012`) di [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Cos'è `NALA-Understory-RS`?
**NALA-Understory-RS (`v0.4.0-nala.6.6.6`)** trasforma una cartella di file Markdown standard secondo la specifica **Open Knowledge Format (`OKF v0.1`)** in una **memoria per agenti IA auto-collegata e protetta dalla deriva**:
- **Moduli `RSI-UNDERSTORY-001..012` Attivi**:
  - `RSI-001..004`: Evidence-Lock, SimHash-256 LSH `O(N)`, Dream Gardener (`USER_PROJECT_SHIELD`), Catena Merkle-256 `log.md` + DLP a 5 zone.
  - `RSI-005..008`: Sincronizzazione CRDT Multi-Nodo (`M4 Max <-> nala-brain <-> nala-worker`), GraphRAG a 2 salti, Delta-Arena `< 18 µs` e sezioni cifrate `# [VAULT:zone]`.
  - `RSI-009`: Tabella Speculativa N-Gram a 64 bit (`--ngram-draft`, `~5.9 µs/op`) isolata per zona.
  - `RSI-010`: Harvester Statico Git-Diff $\rightarrow$ Concetti OKF (`--git-harvest`, `~1.05 µs/op`).
  - `RSI-011`: Time-Travel Point-in-Time Merkle (`--time-travel`).
  - `RSI-012`: Esportatore Capsula DemoPod Zero-PII (`--demopod-export`, `PASS_ZERO_PII`).

## 2. Avvio Rapido
```bash
nala-understory-rs --bench
nala-understory-rs --ngram-draft
nala-understory-rs --git-harvest
nala-understory-rs --time-travel 2026-07-11
nala-understory-rs --demopod-export /tmp/demopod_capsule
nala-understory-rs --serve 127.0.0.1:3800
```
