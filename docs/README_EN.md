# 🇬🇧 NALA-Understory-RS (`v0.3.0-nala.6.6.6`) — English Documentation (EN)

> **100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-Telemetry) Port, Wave-1 & Wave-2 Evolution (`RSI-UNDERSTORY-001..008`) & Security-Hardened Evolution of [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. What is `NALA-Understory-RS`?
AI agents suffer from session amnesia and documentation drift. **NALA-Understory-RS (`v0.3.0-nala.6.6.6`)** turns a plain directory of Markdown files following the **Open Knowledge Format (`OKF v0.1`)** specification into a **self-wiring, drift-proof agent memory**:
- **Single-Pass Arena Graph + Linter + Validator (`79.9 µs` on M4 Max)** and **Incremental Delta-Arena (`2.01 µs/op`, `< 18 µs` SLA)**.
- **Wave-1 (`RSI-001..004`) & Wave-2 (`RSI-005..008`) Subsystems**:
  - **`RSI-001` Evidence-Lock Sync Bridge (`199.6 ns/op`)**: Deterministic `[Dok-ID, S. X]` verifier + `NALA-RAG (:18700)` Hybrid RRF boost.
  - **`RSI-002` 256-Bit SimHash LSH (`9.31 ms / 10k`)**: $O(N)$ 4-band LSH duplicate clustering for `100,000+` concepts.
  - **`RSI-003` Autonomous Dream Gardener (`134.7 µs/op`)**: Nightly graph maintenance with `USER_PROJECT_SHIELD` and NCSP pre-merge snapshot.
  - **`RSI-004` 256-Bit Merkle `log.md` & 5-Zone DLP (`1.29 µs/op`)**: Tamper-evident hash chain and `sHeriFF` Prompt-Injection / multi-tenant isolation.
  - **`RSI-005` Lock-Free CRDT Multi-Node Sync (`740.5 ns/op`)**: Conflict-free H1 section LWW merge and 256-bit bundle Merkle root across `M4 Max <-> nala-brain <-> Beelink`.
  - **`RSI-006` 2-Hop GraphRAG Context Compiler (`162.3 µs/op`)**: Zone-filtered 2-hop subgraph packing within an exact character/token budget.
  - **`RSI-007` Incremental Delta-Arena (`2.01 µs/op`)**: Zero-polling single-file `(mtime_ns, size_bytes, fnv1a_64)` delta detection and parent `index.md` update.
  - **`RSI-008` Encrypted-at-Rest Vault Sections (`# [VAULT:zone]`, `2.36 µs/op`)**: Authenticated ARX-256 + MAC cipher blocks embedded directly inside standard OKF Markdown files.

## 2. Quickstart (`v0.3.0-nala.6.6.6`)
```bash
nala-understory-rs --bench
nala-understory-rs --crdt-sync --json
nala-understory-rs --graph-rag "billing charges" --json
nala-understory-rs --delta-scan --json
nala-understory-rs --vault-verify --json
nala-understory-rs --serve 127.0.0.1:3800
nala-understory-rs --mcp-stdio --bundle /path/to/bundle
```
