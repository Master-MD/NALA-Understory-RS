# 🇨🇭 NALA-Understory-RS (`v0.4.0-nala.6.6.6`) — Deutsche Dokumentation (DE)

> **100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-Telemetry) Portierung, Wave-1 bis Wave-3 Evolution (`RSI-UNDERSTORY-001..012`) & Security-Härtung von [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Was ist `NALA-Understory-RS`?
KI-Agenten vergessen zwischen Sitzungen alles oder halluzinieren auf veralteten Notizen. **NALA-Understory-RS (`v0.4.0-nala.6.6.6`)** verwandelt ein gewöhnliches Verzeichnis aus **reinen Markdown-Dateien (`Open Knowledge Format / OKF v0.1`)** in ein **selbstvernetzendes, drift-geschütztes Agenten-Gedächtnis**:
- **Kein Lock-in, keine Vektor-Blackbox**: Jedes Konzept ist eine lesbare, Git-versionierbare `.md`-Datei mit YAML-Frontmatter (`type` Pflichtfeld, plus `description`, `owners`, `tags`, `sources`, `related`, `updated`).
- **Single-Pass Living Graph & Anti-Drift Linter**: Liest das gesamte Bundle in einem einzigen Arena-Durchlauf (`~58 µs` auf Linux `x86_64`, `~90 µs` auf Apple M4 Max).
- **Wave-1 (`RSI-001..004`), Wave-2 (`RSI-005..008`) & Wave-3 (`RSI-009..012`) Evolution LIVE**:
  - **`RSI-001` Evidence-Lock Sync Bridge**: Bidirektionale Zitat-Prüfung (`[Dok-ID, S. X]`, `Unbelegt-Rate=0`) + Hybrid-RRF-Boost für `NALA-RAG (:18700)`.
  - **`RSI-002` 256-Bit SimHash LSH (`O(N)`)**: Findet Duplikat-Cluster über `10'000` Konzepte in `~9.3 ms`.
  - **`RSI-003` Autonomer Dream Gardener**: Nachtlauf mit `USER_PROJECT_SHIELD` und NCSP Pre-Merge Snapshot über `NALA-Parallax-RS (:4000)`.
  - **`RSI-004` 256-Bit Merkle `log.md` Chain & `sHeriFF` 5-Zonen DLP**: Manipulationssichere Hash-Kette und Mandantentrennung (`mike_privat`, `dc_gmbh`, `andrea_privat`, `shared`, `public_pearl`).
  - **`RSI-005` Lock-Free CRDT Multi-Node Sync**: Konfliktfreier Sektions-Merge + 256-Bit Bundle-Merkle-Root über `M4 Max <-> nala-brain <-> nala-worker`.
  - **`RSI-006` Sub-Millisecond 2-Hop GraphRAG Context Compiler**: Kompiliert exakte 2-Hop Nachbarschafts-Subgraphen unter striktem Zeichen-/Token-Budget.
  - **`RSI-007` Incremental Delta-Arena (`< 18 µs` SLA)**: Erkennt und re-indexiert Einzeldatei-Änderungen via `(mtime_ns, size_bytes, fnv1a_64)` in `~2.5 µs/op`.
  - **`RSI-008` Encrypted-at-Rest Vault-Sektionen (`# [VAULT:zone]`)**: Authentifizierte ARX-256 + MAC Verschlüsselung sensitiver Sektionen in OKF-Markdown-Dateien (`ENC-OKF-V1:...`).
  - **`RSI-009` Speculative N-Gram Prefix Cache Bridge (`--ngram-draft`)**: Zonen-isolierte 64-Bit Bigram $\rightarrow$ 4-Token Draft-Tabelle (`~5.9 µs/op`) für spekulatives Decoding lokaler LLMs.
  - **`RSI-010` Autonomous Git-Diff $\rightarrow$ OKF Concept Harvester (`--git-harvest`)**: Statischer Clean-Room Diff- & Symbol-Extraktor (`~1.05 µs/op`, `Zero-Eval`) für Rust, Swift, TypeScript und Python.
  - **`RSI-011` Temporal Point-in-Time OKF Time-Travel (`--time-travel`)**: Kryptografische Stichtags-Rekonstruktion des Wissensgraphen zu jedem historischen Merkle-Hash oder Datum.
  - **`RSI-012` Zero-Knowledge Customer DemoPod Capsule Exporter (`--demopod-export`)**: Physisches Stripping aller `# [VAULT:*]` Sektionen (`~0.44 µs/op`) inkl. signiertem `DEMOPOD_ZERO_PII_CERTIFICATE.json` (`PASS_ZERO_PII`).

## 2. Schnellstart (`v0.4.0-nala.6.6.6`)
```bash
# Selbst-Benchmark über alle 18 Subsysteme (10'000 Iterationen)
nala-understory-rs --bench

# Wave-3 Spezial-Modi
nala-understory-rs --ngram-draft
nala-understory-rs --git-harvest
nala-understory-rs --time-travel 2026-07-11
nala-understory-rs --demopod-export /tmp/demopod_capsule

# 5-Sprachen Web-Studio + REST API + HTTP MCP Server starten (14 MCP Tools)
nala-understory-rs --serve 127.0.0.1:3800
```
