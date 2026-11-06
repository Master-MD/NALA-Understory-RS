# 🏔️ NALA-Understory-RS (`v0.3.0-nala.6.6.6`) — Documentaziun Rumantscha (RM)

> **Port 100% Rust Nativ (`Edition 2024`, Zero Node.js, Zero Python, Zero Telemetria) ed Evoluziun Wave-1 & Wave-2 (`RSI-UNDERSTORY-001..008`) da [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Tge è `NALA-Understory-RS`?
**NALA-Understory-RS (`v0.3.0-nala.6.6.6`)** transfurma in ordinatur da datotecas Markdown (**Open Knowledge Format / `OKF v0.1`**) en ina **memoria d'agent auto-colliada e protegida cunter deriva**:
- **Scan d'Arena en In Pass (`79.9 µs`)** + **Delta-Arena Incrementala (`2.01 µs/op`)**.
- **Moduls `RSI-UNDERSTORY-001..008` Integrà**:
  - `RSI-001`: Verificatur Evidence-Lock (`199.6 ns/op`) cun punt `NALA-RAG (:18700)`.
  - `RSI-002`: SimHash-256 LSH (`O(N)`) per `10'000` concepts en `9.31 ms`.
  - `RSI-003`: Dream Gardener autonom cun `USER_PROJECT_SHIELD` (`134.7 µs/op`).
  - `RSI-004`: Chadaina Merkle a 256 bits per `log.md` e protecziun DLP da 5 zonas (`1.29 µs/op`).
  - `RSI-005`: Sincronisaziun CRDT Multi-Noda (`740.5 ns/op`).
  - `RSI-006`: Compilatur GraphRAG da 2 sigls (`162.3 µs/op`).
  - `RSI-007`: Delta-Arena incrementala (`2.01 µs/op`, `< 18 µs`).
  - `RSI-008`: Secziuns da Vault criptadas `# [VAULT:zone]` (`2.36 µs/op`).

## 2. Cumenzament Spert
```bash
nala-understory-rs --bench
nala-understory-rs --crdt-sync --json
nala-understory-rs --graph-rag "billing charges" --json
nala-understory-rs --delta-scan --json
nala-understory-rs --vault-verify --json
nala-understory-rs --serve 127.0.0.1:3800
```
