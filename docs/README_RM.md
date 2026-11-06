# 🏔️ NALA-Understory-RS (`v0.1.0-nala.6.6.6`) — Documentaziun Rumantscha (RM)

> **Port 100% Rust Nativ (`Edition 2024`, Zero Node.js, Zero Python, Zero Telemetria) ed Evoluziun da Segirezza da [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Tge è `NALA-Understory-RS`?
**NALA-Understory-RS** transfurma in ordinatur da datotecas Markdown (**Open Knowledge Format / `OKF v0.1`**) en ina **memoria d'agent auto-colliada e protegida cunter deriva**:
- **Nagina chista naira**: Mintga concept è ina datoteca `.md` legibla cun frontmatter YAML (`type` obligatoric, plinavant `description`, `owners`, `tags`, `sources`, `related`, `updated`).
- **Scan d'Arena en In Pass & Linter Cunter Deriva**: Remplazza las 3 scansiuns separadas da Node.js tras in sulet pass en memoria (`57.7 µs` sin Linux `x86_64`, `105.3 µs` sin Apple M4 Max — fin a **66.5x pli spert**).
- **Cache da 3 Stgalims & `.traces/` Replay**:
  1. **Stgalim 1**: Cache d'impronta exacta a 64 bits (`0 cloms LLM`).
  2. **Stgalim 2**: Hot Working Set (10 concepts + 10 dumondas/rispostas) cun invalidaziun immediata tar mintga midada.
  3. **Stgalim 3**: Loop d'agent profund via GPUs localas da NALA (`:11434/:11435`), `NALA-Parallax-RS` (`:4000`) u Anthropic.

## 2. Segirezza `sHeriFF + NVIDIA SkillSpector`
- **`SEC-UNDERSTORY-01`**: Ligia da standard sin `127.0.0.1:3800` e protecziun CSRF severa sin l'header `Origin:`.
- **`SEC-UNDERSTORY-02`**: Operaziuns da datotecas atomaras en Rust senza sutprocess da shell.
- **`SEC-UNDERSTORY-03`**: Parser YAML senza execuziun da JavaScript e controlla canonica da symlinks.

## 3. Cumenzament Spert
```bash
nala-understory-rs --bench
nala-understory-rs --status --lang rm
nala-understory-rs --dream --lang rm
nala-understory-rs --serve 127.0.0.1:3800
nala-understory-rs --mcp-stdio --bundle /via/al/bundle
```
