# 🇮🇹 NALA-Understory-RS (`v1.3.0-nala.6.6.6`) — Documentazione Italiana (Ondata 12: Q4 HNSW-Lite 4-Bit, XMSS-Lite PQ Merkle, Lakehouse & Prove ZK)

> **100% Rust-Turbo Sicuro Nativo (`Edition 2024`, Zero Node.js, Zero Python, `0` crate esterni, `0` `unsafe`, `0` warning, binario singolo `~1.06 MB`)**  
> **52 strumenti MCP statici · 54/54 test unitari (`0.04 s`) · 55 sottosistemi misurati · Verificato su Apple Silicon M4 Max, Linux x86_64/arm64, Proxmox VE 9.2 Airgap & Windows 11**

## ⚛️ Novità Ondata 12 (`RSI-UNDERSTORY-045..048`)
1. **`RSI-045` (`--vec-q4`) — Indice Vettoriale 4-Bit Q4 & HNSW-Lite (`.okf.q4`)**: **`0.67 µs/1k`** (`2.69x` oltre lo SLA `< 1.8 µs`), partizionato per zona con HMAC-SHA256.
2. **`RSI-046` (`--pq-merkle-tree`) — Albero Merkle Gerarchico XMSS-Lite su Chiavi Post-Quantistiche WOTS+**: **`188.29 µs/op`** (`1.17x` SLA `< 220 µs`), albero binario SHA-256 a 16 foglie con contatore monotonico anti-riutilizzo.
3. **`RSI-047` (`--export-lakehouse`) — Esportatore Lakehouse NDJSON/DDL per DuckDB & SQLite**: **`42.83 µs/op`** (`46.7x` SLA `< 2.0 ms`), zero Python, rimozione automatica delle sezioni `# [VAULT:*]`.
4. **`RSI-048` (`--zk-disclose`) — Prova di Divulgazione Selettiva Zero-Knowledge con Hash-Commitment**: **`1.29 µs/op`** (`27.1x` SLA `< 35 µs`), prove Merkle salate che proteggono le foglie adiacenti.

## 🔐 Digest FIPS 180-4 `SHA-256` (`v1.3.0-nala.6.6.6`)
- `macos-arm64`: `07ebfce3d2c78f9221e9670f2c861212df5bafd253b71c585d90bebe3eb2ca50`
- `macos-x86_64`: `6d7fc2a8b33b361a3c508d23e5cb199f7fa99b0d6da3a7b5e12f9015f837428b`
- `linux-x86_64`: `4168c1524d1731c36b237a63e7365a261232c21feb9131536cbfb87b93a8f39b`
- `linux-aarch64`: `d2656bb2b762cd39d00e2366ce6ab41c5d1c4bcd9373b60bda543a0a5a5f3b4a`
- `windows-x86_64.exe`: `ee149913acac4ef42a5369809eeda98e66145052dfdea700aa51b7477658130d`
