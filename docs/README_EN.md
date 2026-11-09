# 🇬🇧 NALA-Understory-RS (`v0.4.0-nala.6.6.6`) — English Documentation (EN)

> **100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-Telemetry) Port, Wave-1 to Wave-3 Evolution (`RSI-UNDERSTORY-001..012`) & Security-Hardened Evolution of [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. What is `NALA-Understory-RS`?
AI agents suffer from session amnesia and documentation drift. **NALA-Understory-RS (`v0.4.0-nala.6.6.6`)** turns a plain directory of Markdown files following the **Open Knowledge Format (`OKF v0.1`)** specification into a **self-wiring, drift-proof agent memory**:
- **Wave-1 (`RSI-001..004`), Wave-2 (`RSI-005..008`) & Wave-3 (`RSI-009..012`) Subsystems**:
  - **`RSI-001..004`**: Evidence-Lock Sync Bridge (`[Dok-ID, S. X]`), $O(N)$ 256-Bit SimHash LSH (`100,000+` concepts), Nightly Dream Gardener (`USER_PROJECT_SHIELD`), and 256-Bit Merkle `log.md` + `sHeriFF` 5-Zone DLP.
  - **`RSI-005..008`**: Lock-Free CRDT Multi-Node Sync (`M4 Max <-> nala-brain <-> nala-worker`), 2-Hop GraphRAG Context Compiler, `< 18 µs` Incremental Delta-Arena, and Encrypted-at-Rest `# [VAULT:zone]` sections.
  - **`RSI-009` Zero-Copy Speculative N-Gram Prefix Cache Bridge (`--ngram-draft`)**: Pre-compiles hot OKF concept subgraphs into zone-isolated 64-bit bigram $\rightarrow$ 4-token draft lookup tables (`~5.9 µs/op`) for `NALA-Parallax-RS` & local LLM speculative decoding.
  - **`RSI-010` Autonomous Git-Diff $\rightarrow$ OKF Concept Harvester (`--git-harvest`)**: Clean-room static unified-diff symbol extractor (`~1.05 µs/op`, `Zero-Eval`) for Rust, Swift, TypeScript, and Python with `sHeriFF` DLP + path-traversal blocking.
  - **`RSI-011` Temporal Point-in-Time OKF Time-Travel (`--time-travel`)**: Reconstructs the exact Living Graph as of any historical Merkle hash or `YYYY-MM-DD` cutoff in `log.md`.
  - **`RSI-012` Zero-Knowledge Customer DemoPod OKF Capsule Exporter (`--demopod-export`)**: Physically strips all `# [VAULT:*]` sections (`~0.44 µs/op`) and emits a cryptographically signed `DEMOPOD_ZERO_PII_CERTIFICATE.json` (`PASS_ZERO_PII`).

## 2. Quickstart (`v0.4.0-nala.6.6.6`)
```bash
nala-understory-rs --bench
nala-understory-rs --ngram-draft
nala-understory-rs --git-harvest
nala-understory-rs --time-travel 2026-07-11
nala-understory-rs --demopod-export /tmp/demopod_capsule
nala-understory-rs --serve 127.0.0.1:3800
nala-understory-rs --mcp-stdio --bundle /path/to/bundle
```
