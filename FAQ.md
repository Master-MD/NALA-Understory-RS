# ❓ FAQ — `NALA-Understory-RS` (`DE / FR / IT / RM / EN`)

## 🇨🇭 Deutsch (DE)
**F1: Ersetzt `NALA-Understory-RS` unser `NALA-RAG` (`:18700`)?**  
**A:** Nein — beide ergänzen sich perfekt! `NALA-RAG` durchsucht 50'000+ PDFs/Dokumente via Qdrant HNSW + SQLite FTS5 mit Evidence-Lock. `NALA-Understory-RS` ist das **selbstvernetzende, Git-versionierbare Arbeits- und Architektur-Gedächtnis (`OKF v0.1`)** für KI-Agenten (Konzepte, APIs, Playbooks, Runbooks, Entscheidungen).

**F2: Ist `NALA-Understory-RS` zu 100% kompatibel mit Original-Understory Bundles und MCP-Clients?**  
**A:** Ja! Alle 5 Standard-MCP-Tools (`memory_query`, `memory_add`, `memory_update`, `memory_status`, `memory_maintain`) sowie die 9 NALA-Erweiterungs-Tools (`memory_rag_sync`, `memory_merkle_verify`, `memory_crdt_sync`, `memory_graph_rag`, `memory_vault_inspect`, `memory_ngram_draft`, `memory_git_harvest`, `memory_time_travel`, `memory_demopod_export` — insgesamt `14` MCP-Tools), `index.md`/`log.md`-Formate, `.traces/`-Notation und `profiles.json` (`usk_...` Keys) sind 1:1 kompatibel.

**F3: Welche Sicherheitslücken aus dem Original wurden behoben?**  
**A:** `SEC-UNDERSTORY-01` (`cors({ origin: true })` + `0.0.0.0:3800` ohne Token blockiert jetzt fremde Browser-Origins via striktem CSRF-Guard auf `127.0.0.1:3800`), `SEC-UNDERSTORY-02` (kein `git safe.directory '*'` Subprozess mehr) und `SEC-UNDERSTORY-03` (kein `gray-matter` JavaScript-Eval mehr + kanonischer Symlink-Schutz).

---

## 🇫🇷 Français (FR)
**Q1 : `NALA-Understory-RS` est-il 100% compatible avec les bundles `OKF v0.1` et MCP ?**  
**R :** Oui. Les 5 outils MCP (`memory_query`, `memory_add`, `memory_update`, `memory_status`, `memory_maintain`), les fichiers `index.md`/`log.md` et `.traces/` sont 100% compatibles.

**Q2 : Ai-je besoin de Node.js ou Python ?**  
**R :** Non. C'est un binaire Rust natif autonome (`~430 Ko`) sans aucune dépendance externe.

---

## 🇮🇹 Italiano (IT)
**D1: `NALA-Understory-RS` è compatibile al 100% con i bundle `OKF v0.1` e i client MCP?**  
**R:** Sì, supporta tutti i 5 strumenti MCP (`memory_query`, `memory_add`, `memory_update`, `memory_status`, `memory_maintain`) sia via `stdio` che via HTTP `/mcp`.

---

## 🏔️ Rumantsch (RM)
**D1: Dovra `NALA-Understory-RS` Node.js u Python?**  
**R:** Na, el è in sulet binari Rust nativ (`~430 KB`) senza dependenzas externas.

---

## 🇬🇧 English (EN)
**Q1: Does `NALA-Understory-RS` require Node.js, npm, or Python?**  
**A:** Zero. It is a single native Rust binary (`~430 KB`) compiled with `Edition 2024` and zero external crate dependencies.

**Q2: How do I connect it to Claude Code, Cursor, Codex, Antigravity, ZCode, or OpenCode?**  
**A:** Either via stdio (`nala-understory-rs --mcp-stdio --bundle /path/to/bundle`) or via Streamable HTTP (`http://127.0.0.1:3800/mcp` when running `nala-understory-rs --serve`).
