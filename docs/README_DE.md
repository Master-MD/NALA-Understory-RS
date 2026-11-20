# 🌱🦀 NALA-Understory-RS (`v1.2.0-nala.6.6.6`) — Deutsche Dokumentation (DE)

**100% Native Safe Rust-Turbo (`Edition 2024`, `0` Crates, `0` `unsafe`, `0` Warnings, `~1.03 MB` Single Binary, `48` statische MCP-Tools, `50/50` Tests, `51` Benchmark-Subsysteme)**

## ⚛️ Neu in Wave-11 (`v1.2.0-nala.6.6.6` · `RSI-UNDERSTORY-041..044`)
- **`RSI-041` (`--hybrid-rrf`)**: **Tri-Modal Reciprocal Rank Fusion (`k=60`)** über BM25 + 2-Hop Living-Graph + 1-Bit/Int8 Vektoren in **`313.8 ns/op`** (`79.7x` SLA) mit strikter Zonen-Schnittmenge.
- **`RSI-042` (`--compact-heal`)**: Zero-Pause **Arena Compactor & Dead-Link Auto-Healer** (Levenshtein-1 Fuzzy FST) in **`0.52 µs/op`** inkl. `.okf_snapshots` Pre-Heal-Backup.
- **`RSI-043` (`--pq-seal`)**: **Post-Quantum Hash-Based WOTS+ (`w=16`, 67 Ketten, Zero-Alloc FIPS 180-4 SHA-256)** in **`177.32 µs/op`** (`0` externe Crates, NIST SP 800-208 Prinzip).
- **`RSI-044` (`--self-audit --strict`)**: Kontinuierlicher **Runtime-Invarianten-Self-Audit & Chaos-Verifier** in **`7.93 µs/op`** (7 Security/Determinismus-Invarianten, Fail-Closed CI Exit-Code).

Alle 5 verifizierten Plattform-Binaries, `SBOM_SLSA_v1.2.0-nala.6.6.6.json` & `SHA256SUMS.txt`: [GitHub Releases](https://github.com/Master-MD/NALA-Understory-RS/releases/tag/v1.2.0-nala.6.6.6). Lizenz: Apache-2.0.
