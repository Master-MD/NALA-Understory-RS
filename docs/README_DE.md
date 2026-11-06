# 🇨🇭 NALA-Understory-RS (`v0.3.0-nala.6.6.6`) — Deutsche Dokumentation (DE)

> **100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-Telemetry) Portierung, Wave-1 & Wave-2 Evolution (`RSI-UNDERSTORY-001..008`) & Security-Härtung von [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Was ist `NALA-Understory-RS`?
KI-Agenten vergessen zwischen Sitzungen alles oder halluzinieren auf veralteten Notizen. **NALA-Understory-RS (`v0.3.0-nala.6.6.6`)** verwandelt ein gewöhnliches Verzeichnis aus **reinen Markdown-Dateien (`Open Knowledge Format / OKF v0.1`)** in ein **selbstvernetzendes, drift-geschütztes Agenten-Gedächtnis**:
- **Kein Lock-in, keine Vektor-Blackbox**: Jedes Konzept ist eine lesbare, Git-versionierbare `.md`-Datei mit YAML-Frontmatter (`type` Pflichtfeld, plus `description`, `owners`, `tags`, `sources`, `related`, `updated`).
- **Single-Pass Living Graph & Anti-Drift Linter**: Statt wie die Node.js-Originalversion dreimal über das Dateisystem zu laufen (`buildGraph`, `lintBundle`, `validateBundle`), liest `NALA-Understory-RS` das gesamte Bundle in einem einzigen Arena-Durchlauf (`57.7 µs` auf Linux `x86_64`, `79.9 µs` auf Apple M4 Max — bis zu **66.5x schneller**).
- **Wave-1 (`RSI-UNDERSTORY-001..004`) & Wave-2 (`RSI-UNDERSTORY-005..008`) Evolution LIVE**:
  - **`RSI-001` Evidence-Lock Sync Bridge**: Bidirektionale Zitat-Prüfung (`[Dok-ID, S. X]`, `Unbelegt-Rate=0`) in `199.6 ns/op` + Hybrid-RRF-Boost für `NALA-RAG (:18700)`.
  - **`RSI-002` 256-Bit SimHash LSH (`O(N)`)**: Findet Duplikat-Cluster über `10'000` Konzepte in `9.31 ms` (198.7x schneller als $O(N^2)$).
  - **`RSI-003` Autonomer Dream Gardener**: Nachtlauf mit `USER_PROJECT_SHIELD` und NCSP Pre-Merge Snapshot über `NALA-Parallax-RS (:4000)`.
  - **`RSI-004` 256-Bit Merkle `log.md` Chain & `sHeriFF` 5-Zonen DLP**: Manipulationssichere Hash-Kette (`1.29 µs/op`) und Mandantentrennung (`mike_privat`, `dc_gmbh`, `andrea_privat`, `shared`, `public_pearl`).
  - **`RSI-005` Lock-Free CRDT Multi-Node Sync**: Konfliktfreier Sektions-Merge (`740.5 ns/op`) + 256-Bit Bundle-Merkle-Root über `M4 Max <-> nala-brain <-> Beelink`.
  - **`RSI-006` Sub-Millisecond 2-Hop GraphRAG Context Compiler**: Kompiliert exakte 2-Hop Nachbarschafts-Subgraphen (`162.3 µs/op`) unter striktem Zeichen-/Token-Budget.
  - **`RSI-007` Incremental Delta-Arena (`< 18 µs` SLA)**: Erkennt und re-indexiert Einzeldatei-Änderungen via `(mtime_ns, size_bytes, fnv1a_64)` in **`2.01 µs/op`** (9x schneller als das 18 µs SLA!).
  - **`RSI-008` Encrypted-at-Rest Vault-Sektionen (`# [VAULT:zone]`)**: Authentifizierte ARX-256 + MAC Verschlüsselung (`2.36 µs/op`) sensitiver Sektionen mitten in regulären OKF-Markdown-Dateien (`ENC-OKF-V1:...`).

## 2. Sicherheits-Härtung (`sHeriFF + NVIDIA SkillSpector`)
1. **`SEC-UNDERSTORY-01` (CSRF & Open-Bind Schutz)**: Standard-Bindung auf `127.0.0.1:3800` statt `0.0.0.0:3800` sowie strikte `Origin`-Header-Prüfung (`HTTP 403`).
2. **`SEC-UNDERSTORY-02` (Zero-Subprocess Git)**: Keine Shell-Aufrufe und kein unsicheres `git config --system safe.directory '*'`.
3. **`SEC-UNDERSTORY-03` (Zero-Eval YAML & Symlink-Sandbox)**: Reiner Rust-Parser ohne `gray-matter` JavaScript-Ausführung (`---js`) und kanonische Symlink-Prüfung.

## 3. Schnellstart (`v0.3.0-nala.6.6.6`)
```bash
# Selbst-Benchmark über alle 14 Subsysteme (10'000 Iterationen)
nala-understory-rs --bench

# Wave-1 & Wave-2 Spezial-Modi
nala-understory-rs --crdt-sync --json
nala-understory-rs --graph-rag "billing charges" --json
nala-understory-rs --delta-scan --json
nala-understory-rs --vault-verify --json

# 5-Sprachen Web-Studio + REST API + HTTP MCP Server starten (8 MCP Tools)
nala-understory-rs --serve 127.0.0.1:3800
```
