# 🇫🇷 NALA-Understory-RS (`v1.3.0-nala.6.6.6`) — Documentation Française (Vague 12 : Q4 HNSW-Lite 4-Bit, XMSS-Lite PQ Merkle, Lakehouse & Preuves ZK)

> **100% Rust-Turbo Sûr Natif (`Edition 2024`, Zéro Node.js, Zéro Python, `0` crate externe, `0` `unsafe`, `0` avertissement, binaire unique `~1.06 MB`)**  
> **52 outils MCP statiques · 54/54 tests unitaires (`0.04 s`) · 55 sous-systèmes mesurés · Vérifié sur Apple Silicon M4 Max, Linux x86_64/arm64, Proxmox VE 9.2 Airgap & Windows 11**

## ⚛️ Nouveautés Vague 12 (`RSI-UNDERSTORY-045..048`)
1. **`RSI-045` (`--vec-q4`) — Index Vectoriel Quantifié 4-Bit Q4 & HNSW-Lite (`.okf.q4`)** : **`0.67 µs/1k`** (`2.69x` plus rapide que le SLA `< 1.8 µs`), partitionné par zone + HMAC-SHA256.
2. **`RSI-046` (`--pq-merkle-tree`) — Arbre Merkle Hiérarchique XMSS-Lite sur Clés Post-Quantiques WOTS+** : **`188.29 µs/op`** (`1.17x` SLA `< 220 µs`), arbre binaire SHA-256 à 16 feuilles avec garde monotone anti-réutilisation.
3. **`RSI-047` (`--export-lakehouse`) — Exportateur Lakehouse NDJSON/DDL DuckDB & SQLite** : **`42.83 µs/op`** (`46.7x` SLA `< 2.0 ms`), zéro Python, purge automatique des sections `# [VAULT:*]`.
4. **`RSI-048` (`--zk-disclose`) — Preuve de Divulgation Sélective Zero-Knowledge par Engagement SHA-256** : **`1.29 µs/op`** (`27.1x` SLA `< 35 µs`), preuves d'inclusion salées protégeant les feuilles voisines.

## 🔐 Empreintes FIPS 180-4 `SHA-256` (`v1.3.0-nala.6.6.6`)
- `macos-arm64`: `07ebfce3d2c78f9221e9670f2c861212df5bafd253b71c585d90bebe3eb2ca50`
- `macos-x86_64`: `6d7fc2a8b33b361a3c508d23e5cb199f7fa99b0d6da3a7b5e12f9015f837428b`
- `linux-x86_64`: `4168c1524d1731c36b237a63e7365a261232c21feb9131536cbfb87b93a8f39b`
- `linux-aarch64`: `d2656bb2b762cd39d00e2366ce6ab41c5d1c4bcd9373b60bda543a0a5a5f3b4a`
- `windows-x86_64.exe`: `ee149913acac4ef42a5369809eeda98e66145052dfdea700aa51b7477658130d`
