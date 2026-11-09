# 🇬🇧 NALA-Understory-RS (`v0.5.0-nala.6.6.6`) — English Documentation (EN)

> **100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-Telemetry, 100% Universal Standalone) Port, Wave-1 to Wave-4 Evolution (`RSI-UNDERSTORY-001..016`) & Security-Hardened Evolution of [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. What is `NALA-Understory-RS`?
AI agents suffer from session amnesia and documentation drift. **NALA-Understory-RS (`v0.5.0-nala.6.6.6`)** turns a plain directory of Markdown files following the **Open Knowledge Format (`OKF v0.1`)** specification into a **self-wiring, drift-proof agent memory** that runs **100% standalone (zero external services, zero OS lock-in, works completely airgapped)** across macOS (`arm64` / `x86_64`), Linux (`x86_64` / `aarch64`), and Windows (`x86_64`):
- **`RSI-001..004` (Wave-1)**: Evidence-Lock citation verifier (`[Dok-ID, S. X]`), $O(N)$ 256-Bit SimHash LSH (`100,000+` concepts), Nightly Dream Gardener (`USER_PROJECT_SHIELD`), and 256-Bit Merkle `log.md` + 5-Zone DLP (`vault_primary`, `vault_enterprise`, `vault_partner`, `shared`, `public_pearl`).
- **`RSI-005..008` (Wave-2)**: Lock-Free CRDT Multi-Node Sync (`node-alpha <-> node-beta <-> node-gamma`), 2-Hop GraphRAG Context Compiler, `< 18 µs` Incremental Delta-Arena, and Encrypted-at-Rest `# [VAULT:zone]` sections.
- **`RSI-009..012` (Wave-3)**: Zero-Copy Speculative N-Gram Prefix Cache (`--ngram-draft`), Static Git-Diff $\rightarrow$ OKF Harvester (`--git-harvest`), Point-in-Time Merkle Time-Travel (`--time-travel`), and Zero-Knowledge DemoPod Capsule Exporter (`--demopod-export`).
- **`RSI-013..016` (Wave-4)**:
  - **`RSI-013` Zero-Copy Binary Graph Snapshot Cache (`.okf.idx`, `--binary-index`)**: 64-byte-aligned flat-frame graph snapshot protected by a 256-bit SHA-256 Header MAC (`< 4.5 µs` SLA).
  - **`RSI-014` Multi-Agent Optimistic Section Lease (`--section-lease`)**: 256-slot open-addressed H1 section lease guard preventing concurrent agent overwrites (`< 900 ns` SLA, `0%` lost updates).
  - **`RSI-015` Cross-Bundle Federated 2-Hop GraphRAG Mesh (`--federated-rag`)**: Multi-bundle `bundle://<alias>/<path.md>` 2-hop compiler with strict zone-rank intersection (`< 185 µs` SLA across 4 bundles).
  - **`RSI-016` Deterministic Citation & Source-Anchor Drift Attestor (`--attest-citations`)**: Verifies `resource:`/`sources:` YAML anchors and `[DOC-ID, S. X]` citations against sealed fingerprints (`< 14 µs` SLA).

## 2. Quickstart (`v0.5.0-nala.6.6.6`)
```bash
nala-understory-rs --standalone-check
nala-understory-rs --self-hash
nala-understory-rs --bench
nala-understory-rs --binary-index
nala-understory-rs --section-lease
nala-understory-rs --federated-rag "billing charges"
nala-understory-rs --attest-citations
nala-understory-rs --serve 127.0.0.1:3800
nala-understory-rs --mcp-stdio --bundle /path/to/bundle
```
