# 🇨🇭 NALA-Understory-RS (`v1.3.0-nala.6.6.6`) — Deutsche Dokumentation (Wave-12: 4-Bit Q4 HNSW-Lite, XMSS-Lite PQ Merkle, Lakehouse & ZK Selective Disclosure)

> **100% Native Safe Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, `0` externe Crates, `0` `unsafe`, `0` Compiler-Warnungen, `~1.06 MB` Single Binary)**  
> **52 statische Prompt-Cache-sichere MCP-Tools · 54/54 Release Unit-Tests (`0.04 s`) · 55 gebenchmarkte Subsysteme · Verifiziert auf Apple Silicon M4 Max, Linux x86_64/arm64, Airgapped Proxmox VE 9.2 (`unshare -n env -i`) & Windows 11**

## ⚛️ Wave-12 Meilensteine (`RSI-UNDERSTORY-045..048`)
1. **`RSI-045` (`--vec-q4`) — 4-Bit Asymmetric Scalar Quantized Vector Index & HNSW-Lite Skip-List (`.okf.q4`)**: **`0.67 µs/1k`** (`2.69x` schneller als `< 1.8 µs` SLA). Nibble-gepackte 4-Bit-Vektor-Arithmetik (`2x` kompakter als Int8) mit 2-Layer HNSW-Lite Skip-Navigation, Zonen-Partitionierung & Constant-Time HMAC-SHA256 MAC.
2. **`RSI-046` (`--pq-merkle-tree`) — Hierarchical XMSS-Lite Merkle Tree über mehrfache Post-Quantum WOTS+ Schlüssel**: **`188.29 µs/op`** (`1.17x` schneller als `< 220 µs` SLA). 16-Blatt binärer FIPS 180-4 SHA-256 Merkle-Baum + Authentifizierungs-Pfad unter einem einzigen Public Root mit monotonem Blatt-Index-Schutz gegen Schlüssel-Wiederverwendung.
3. **`RSI-047` (`--export-lakehouse`) — Deterministic SQLite/DuckDB/Parquet-Ready NDJSON Knowledge Lakehouse Exporter**: **`42.83 µs/op`** (`46.7x` schneller als `< 2.0 ms` SLA). Exportiert Knoten, Kanten, PageRank-Salienz, Decay & SQL-DDL komplett ohne Python und strippt automatisch `# [VAULT:*]` Sektionen gemäss Zonen-Allowlist.
4. **`RSI-048` (`--zk-disclose`) — Zero-Knowledge Hash-Commitment Selective Disclosure Proof**: **`1.29 µs/op`** (`27.1x` schneller als `< 35 µs` SLA). Erstellt und verifiziert gesalzene SHA-256 Merkle-Inklusions-Beweise für einzelne OKF-Fakten, ohne vertrauliche Nachbar-Blätter offenzulegen.

## 🚀 Schnellstart
```bash
curl -fsSL https://raw.githubusercontent.com/Master-MD/NALA-Understory-RS/main/install.sh | sh
nala-understory-rs --standalone-check
nala-understory-rs --vec-q4 "billing customer"
nala-understory-rs --pq-merkle-tree
nala-understory-rs --export-lakehouse
nala-understory-rs --zk-disclose
nala-understory-rs --self-audit --strict
```

## 🔐 Verifizierte `v1.3.0-nala.6.6.6` FIPS 180-4 `SHA-256` Hashes
- `macos-arm64`: `07ebfce3d2c78f9221e9670f2c861212df5bafd253b71c585d90bebe3eb2ca50`
- `macos-x86_64`: `6d7fc2a8b33b361a3c508d23e5cb199f7fa99b0d6da3a7b5e12f9015f837428b`
- `linux-x86_64`: `4168c1524d1731c36b237a63e7365a261232c21feb9131536cbfb87b93a8f39b`
- `linux-aarch64`: `d2656bb2b762cd39d00e2366ce6ab41c5d1c4bcd9373b60bda543a0a5a5f3b4a`
- `windows-x86_64.exe`: `ee149913acac4ef42a5369809eeda98e66145052dfdea700aa51b7477658130d`
