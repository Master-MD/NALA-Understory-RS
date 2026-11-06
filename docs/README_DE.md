# 🇨🇭 NALA-Understory-RS (`v0.1.0-nala.6.6.6`) — Deutsche Dokumentation (DE)

> **100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-Telemetry) Portierung & Security-Evolution von [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Was ist `NALA-Understory-RS`?
KI-Agenten vergessen zwischen Sitzungen alles oder halluzinieren auf veralteten Notizen. **NALA-Understory-RS** verwandelt ein gewöhnliches Verzeichnis aus **reinen Markdown-Dateien (`Open Knowledge Format / OKF v0.1`)** in ein **selbstvernetzendes, drift-geschütztes Agenten-Gedächtnis**:
- **Kein Lock-in, keine Vektor-Blackbox**: Jedes Konzept ist eine lesbare, Git-versionierbare `.md`-Datei mit YAML-Frontmatter (`type` Pflichtfeld, plus `description`, `owners`, `tags`, `sources`, `related`, `updated`).
- **Single-Pass Living Graph & Anti-Drift Linter**: Statt wie die Node.js-Originalversion dreimal über das Dateisystem zu laufen (`buildGraph`, `lintBundle`, `validateBundle`), liest `NALA-Understory-RS` das gesamte Bundle in einem einzigen Arena-Durchlauf (`57.7 µs` auf Linux `x86_64`, `105.3 µs` auf Apple M4 Max — bis zu **66.5x schneller**).
- **3-Stufiger Antwort-Cache**:
  1. **Layer 1 (Exact Fingerprint Cache)**: 64-Bit FNV-1a / SHA-256 Anfrage-Fingerprint (`0 LLM-Aufrufe`).
  2. **Layer 2 (Hot Working Set)**: Top-10 aktive Konzepte + letzte 10 Q&As im Speicher mit sofortiger Invalidierung bei Schreibzugriffen.
  3. **Layer 3 (Deep Agent Loop)**: Lokale NALA-GPUs (`Ollama :11434/:11435`), OpenAI-kompatible Router (`NALA-Parallax-RS :4000`) oder Anthropic.
- **`.traces/` Pfad-Replay**: Jede Abfrage protokolliert den exakten Traversierungs-Pfad (`search "rate limit" (2) → read billing-api.md → ✓`).
- **Multi-Profil-Mandantentrennung**: Isolierte Wissens-Bundles mit eigenen `usk_...` API-Schlüsseln und atomarem File-Locking.

## 2. Sicherheits-Härtung (`sHeriFF + NVIDIA SkillSpector`)
1. **`SEC-UNDERSTORY-01` (CSRF & Open-Bind Schutz)**: Standard-Bindung auf `127.0.0.1:3800` statt `0.0.0.0:3800` sowie strikte `Origin`-Header-Prüfung (blockiert fremde Browser-Tabs mit `HTTP 403`).
2. **`SEC-UNDERSTORY-02` (Zero-Subprocess Git)**: Keine Shell-Aufrufe und kein unsicheres `git config --system safe.directory '*'`.
3. **`SEC-UNDERSTORY-03` (Zero-Eval YAML & Symlink-Sandbox)**: Reiner Rust-Parser ohne `gray-matter` JavaScript-Ausführung (`---js`) und kanonische Symlink-Prüfung gegen Path-Traversal.

## 3. Schnellstart
```bash
# Selbst-Benchmark (10'000 Iterationen)
nala-understory-rs --bench

# Status, Graph & OKF v0.1 Konformität prüfen
nala-understory-rs --status --lang de

# Dream-Phase Analyse (Duplikat-Erkennung via 64-Bit BitSet Jaccard & Split-Vorschläge)
nala-understory-rs --dream --lang de

# 5-Sprachen Web-Studio + REST API + HTTP MCP Server starten
nala-understory-rs --serve 127.0.0.1:3800

# Stdio MCP Server für Claude Code / Cursor / Codex / Antigravity / ZCode
nala-understory-rs --mcp-stdio --bundle /pfad/zum/bundle
```
