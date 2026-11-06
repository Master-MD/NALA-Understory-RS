# 🇮🇹 NALA-Understory-RS (`v0.1.0-nala.6.6.6`) — Documentazione Italiana (IT)

> **Porting 100% Rust Nativo (`Edition 2024`, Zero Node.js, Zero Python, Zero Telemetria) ed Evoluzione Blindata di [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Cos'è `NALA-Understory-RS`?
**NALA-Understory-RS** trasforma una cartella di file Markdown standard secondo la specifica **Open Knowledge Format (`OKF v0.1`)** in una **memoria per agenti IA auto-collegata e protetta dalla deriva**:
- **Nessuna scatola nera**: Ogni concetto è un file `.md` leggibile dall'uomo e tracciabile con Git con frontmatter YAML (`type` obbligatorio, oltre a `description`, `owners`, `tags`, `sources`, `related`, `updated`).
- **Arena Scanner a Singolo Passaggio & Linter Anti-Deriva**: Sostituisce le 3 scansioni separate del filesystem di Node.js con un unico passaggio in memoria (`57.7 µs` su Linux `x86_64`, `105.3 µs` su Apple M4 Max — fino a **66.5x più veloce**).
- **Cache a 3 Livelli & Replay `.traces/`**:
  1. **Livello 1**: Cache LRU con impronta a 64 bit (`0 chiamate LLM`).
  2. **Livello 2**: Hot Working Set (top 10 concetti + 10 Q&A recenti) invalidato istantaneamente ad ogni scrittura.
  3. **Livello 3**: Sintesi profonda tramite GPU locali NALA (`Ollama :11434/:11435`), `NALA-Parallax-RS` (`:4000`) o Anthropic.

## 2. Sicurezza `sHeriFF + NVIDIA SkillSpector`
- **`SEC-UNDERSTORY-01`**: Bind predefinito su `127.0.0.1:3800` e protezione CSRF rigorosa sull'header `Origin:`.
- **`SEC-UNDERSTORY-02`**: Scritture atomiche native in Rust senza sottoprocessi shell o `safe.directory '*'`.
- **`SEC-UNDERSTORY-03`**: Parser YAML privo di `eval` JavaScript e sandbox contro l'evasione tramite symlink.

## 3. Avvio Rapido
```bash
nala-understory-rs --bench
nala-understory-rs --status --lang it
nala-understory-rs --dream --lang it
nala-understory-rs --serve 127.0.0.1:3800
nala-understory-rs --mcp-stdio --bundle /percorso/bundle
```
