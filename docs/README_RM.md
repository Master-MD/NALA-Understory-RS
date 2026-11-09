# 🏔️ NALA-Understory-RS (`v0.4.0-nala.6.6.6`) — Documentaziun Rumantscha (RM)

> **Port 100% Rust Nativ (`Edition 2024`, Zero Node.js, Zero Python, Zero Telemetria) ed Evoluziun Undas 1–3 (`RSI-UNDERSTORY-001..012`) da [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Tge è `NALA-Understory-RS`?
**NALA-Understory-RS (`v0.4.0-nala.6.6.6`)** transfurma in ordinatur da datotecas Markdown (**Open Knowledge Format / `OKF v0.1`**) en ina **memoria d'agent auto-colliada e protegida cunter deriva**:
- **Moduls `RSI-UNDERSTORY-001..012` Integrà**:
  - `RSI-001..004`: Verificatur Evidence-Lock, SimHash-256 LSH (`O(N)`), Dream Gardener, Chadaina Merkle a 256 bits e DLP da 5 zonas.
  - `RSI-005..008`: Sincronisaziun CRDT Multi-Noda, Compilatur GraphRAG da 2 sigls, Delta-Arena `< 18 µs` e secziuns da Vault criptadas `# [VAULT:zone]`.
  - `RSI-009`: Cache da Prefix N-Gram Speculativ a 64 bits (`--ngram-draft`, `~5.9 µs/op`).
  - `RSI-010`: Harvester Static Git-Diff $\rightarrow$ Concepts OKF (`--git-harvest`, `~1.05 µs/op`).
  - `RSI-011`: Time-Travel Point-in-Time Merkle (`--time-travel`).
  - `RSI-012`: Exportatur da Capsula DemoPod Zero-PII (`--demopod-export`, `PASS_ZERO_PII`).

## 2. Cumenzament Spert
```bash
nala-understory-rs --bench
nala-understory-rs --ngram-draft
nala-understory-rs --git-harvest
nala-understory-rs --time-travel 2026-07-11
nala-understory-rs --demopod-export /tmp/demopod_capsule
nala-understory-rs --serve 127.0.0.1:3800
```
