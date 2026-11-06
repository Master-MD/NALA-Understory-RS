# 🇮🇹 NALA-Understory-RS (`v0.3.0-nala.6.6.6`) — Documentazione Italiana (IT)

> **Porting 100% Rust Nativo (`Edition 2024`, Zero Node.js, Zero Python, Zero Telemetria) ed Evoluzione Blindata Wave-1 & Wave-2 (`RSI-UNDERSTORY-001..008`) di [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Cos'è `NALA-Understory-RS`?
**NALA-Understory-RS (`v0.3.0-nala.6.6.6`)** trasforma una cartella di file Markdown standard secondo la specifica **Open Knowledge Format (`OKF v0.1`)** in una **memoria per agenti IA auto-collegata e protetta dalla deriva**:
- **Arena Scanner a Singolo Passaggio (`79.9 µs` su M4 Max)** + **Delta-Arena Incrementale (`2.01 µs/op`, `< 18 µs` SLA)**.
- **Moduli `RSI-UNDERSTORY-001..008` Attivi**:
  - `RSI-001`: Verificatore Evidence-Lock (`199.6 ns/op`) e ponte `NALA-RAG (:18700)`.
  - `RSI-002`: Deduplicazione `O(N)` SimHash-256 LSH su `10.000` concetti in `9.31 ms`.
  - `RSI-003`: Dream Gardener autonomo con `USER_PROJECT_SHIELD` (`134.7 µs/op`).
  - `RSI-004`: Catena Merkle a 256 bit per `log.md` e guardia DLP a 5 zone (`1.29 µs/op`).
  - `RSI-005`: Sincronizzazione CRDT Multi-Nodo (`M4 Max <-> nala-brain <-> nala-worker`, `740.5 ns/op`).
  - `RSI-006`: Compilatore di contesto GraphRAG a 2 salti (`162.3 µs/op`).
  - `RSI-007`: Aggiornamento incrementale Delta-Arena (`2.01 µs/op`).
  - `RSI-008`: Sezioni Vault crittografate `# [VAULT:zone]` (`2.36 µs/op`).

## 2. Avvio Rapido
```bash
nala-understory-rs --bench
nala-understory-rs --crdt-sync --json
nala-understory-rs --graph-rag "billing charges" --json
nala-understory-rs --delta-scan --json
nala-understory-rs --vault-verify --json
nala-understory-rs --serve 127.0.0.1:3800
```
