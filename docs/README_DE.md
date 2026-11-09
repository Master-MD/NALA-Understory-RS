# 🇨🇭 NALA-Understory-RS (`v0.5.0-nala.6.6.6`) — Deutsche Dokumentation (DE)

> **100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-Telemetry, 100% Standalone ohne externe System-Abhängigkeit) Portierung, Wave-1 bis Wave-4 Evolution (`RSI-UNDERSTORY-001..016`) & Security-Härtung von [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Was ist `NALA-Understory-RS`?
KI-Agenten vergessen zwischen Sitzungen alles oder halluzinieren auf veralteten Notizen. **NALA-Understory-RS (`v0.5.0-nala.6.6.6`)** verwandelt ein gewöhnliches Verzeichnis aus **reinen Markdown-Dateien (`Open Knowledge Format / OKF v0.1`)** in ein **selbstvernetzendes, drift-geschütztes Agenten-Gedächtnis**, das **100% universell und komplett eigenständig (auch komplett offline und ohne NALA-OS)** auf macOS (`arm64` / `x86_64`), Linux (`x86_64` / `aarch64`) und Windows (`x86_64`) läuft:
- **Wave-1 (`RSI-001..004`)**: Evidence-Lock Zitat-Prüfung (`[Dok-ID, S. X]`, `Unbelegt-Rate=0`), `O(N)` 256-Bit SimHash LSH (`100'000+` Konzepte), autonomer Dream Gardener (`USER_PROJECT_SHIELD`) & kryptografische 256-Bit Merkle `log.md` Kette mit 5-Zonen DLP (`vault_primary`, `vault_enterprise`, `vault_partner`, `shared`, `public_pearl`).
- **Wave-2 (`RSI-005..008`)**: Konfliktfreier CRDT Multi-Node Sync (`node-alpha <-> node-beta <-> node-gamma`), 2-Hop GraphRAG Context Compiler, `< 18 µs` Incremental Delta-Arena & `# [VAULT:zone]` ARX-256 + MAC Verschlüsselung.
- **Wave-3 (`RSI-009..012`)**: Zero-Copy Speculative N-Gram Prefix Cache (`--ngram-draft`), statischer Clean-Room Git-Diff Harvester (`--git-harvest`), Point-in-Time Merkle Time-Travel (`--time-travel`) & Zero-PII DemoPod Capsule Exporter (`--demopod-export`).
- **Wave-4 (`RSI-013..016`)**:
  - **`RSI-013` Zero-Copy Binary Graph Snapshot Cache (`.okf.idx`, `--binary-index`)**: 64-Byte-aligned Flat-Frame Binär-Snapshot mit 256-Bit SHA-256 Header-MAC (`< 4.5 µs` SLA).
  - **`RSI-014` Multi-Agent Optimistic Section Lease (`--section-lease`)**: Atomares H1-Sektions-Claim-Protokoll für parallele Agenten-Schwärme (`< 900 ns` SLA, `0%` Lost Updates).
  - **`RSI-015` Cross-Bundle Federated 2-Hop GraphRAG Mesh (`--federated-rag`)**: Föderierte 2-Hop Abfragen über mehrere OKF-Bundles (`bundle://<alias>/<path.md>`) mit strikter Zonen-Intersection (`< 185 µs` SLA).
  - **`RSI-016` Deterministic Citation & Source-Anchor Drift Attestor (`--attest-citations`)**: Prüft `resource:`/`sources:` und `[DOC-ID, S. X]` gegen versiegelte Fingerprints (`< 14 µs` SLA).

## 2. Schnellstart (`v0.5.0-nala.6.6.6`)
```bash
# Eigenständige Universal- & SHA-256 Selbstprüfung (ohne Netzwerk / ohne NALA-OS)
nala-understory-rs --standalone-check
nala-understory-rs --self-hash

# Selbst-Benchmark über alle 22 Subsysteme (10'000 Iterationen)
nala-understory-rs --bench

# Wave-4 Spezial-Modi
nala-understory-rs --binary-index
nala-understory-rs --section-lease
nala-understory-rs --federated-rag "billing charges"
nala-understory-rs --attest-citations

# 5-Sprachen Web-Studio + REST API + HTTP MCP Server starten (18 MCP Tools)
nala-understory-rs --serve 127.0.0.1:3800
```
