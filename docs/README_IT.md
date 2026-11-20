# 🌱🦀 NALA-Understory-RS (`v1.2.0-nala.6.6.6`) — Documentazione Italiana (IT)

**100% Rust-Turbo Sicuro (`Edition 2024`, `0` dipendenze, `0` `unsafe`, `0` warning, `48` strumenti MCP statici, `50/50` test, `51` sottosistemi)**

## ⚛️ Novità Ondata-11 (`v1.2.0-nala.6.6.6` · `RSI-UNDERSTORY-041..044`)
- **`RSI-041` (`--hybrid-rrf`)**: Fusione **RRF tri-modale (`k=60`)** BM25 + Grafo 2-Hop + Vettori in **`313.8 ns/op`**.
- **`RSI-042` (`--compact-heal`)**: Auto-riparazione link rotti senza pausa in **`0.52 µs/op`** con snapshot.
- **`RSI-043` (`--pq-seal`)**: Firme **Post-Quantistiche WOTS+ (`w=16`, FIPS SHA-256)** in **`177.32 µs/op`** (`0` crate).
- **`RSI-044` (`--self-audit --strict`)**: Self-audit continuo degli invarianti di sicurezza in **`7.93 µs/op`**.

Binari verificati (`SHA256SUMS.txt`): [GitHub Releases](https://github.com/Master-MD/NALA-Understory-RS/releases/tag/v1.2.0-nala.6.6.6). Licenza: Apache-2.0.
