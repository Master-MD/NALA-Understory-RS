# 🇬🇧 NALA-Understory-RS (`v0.1.0-nala.6.6.6`) — English Documentation (EN)

> **100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, Zero-Telemetry) Port, Acceleration & Security-Hardened Evolution of [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. What is `NALA-Understory-RS`?
AI agents suffer from session amnesia and documentation drift. **NALA-Understory-RS** turns a plain directory of Markdown files following the **Open Knowledge Format (`OKF v0.1`)** specification into a **self-wiring, drift-proof agent memory**:
- **Zero Lock-In, Human-Auditable Markdown**: Every concept is a standard `.md` file with YAML frontmatter (`type` required; `description`, `owners`, `tags`, `sources`, `related`, `updated`). Every directory maintains a deterministic `index.md` and an append-only `log.md`.
- **Single-Pass Arena Graph + Linter + Validator**: Replaces upstream Node.js's three separate directory traversals (`buildGraph`, `lintBundle`, `validateBundle`) with a single contiguous arena scan (`57.7 µs` on Linux `x86_64`, `105.3 µs` on Apple M4 Max — up to **66.5x faster**).
- **3-Layer Query Cache**:
  1. **Layer 1 (Exact Fingerprint Cache)**: 64-bit FNV-1a / SHA-256 fingerprint LRU (`0 LLM calls`).
  2. **Layer 2 (Hot Working Set)**: Top 10 active concepts + 10 recent Q&As in RAM with instant invalidation on any write.
  3. **Layer 3 (Deep Agent Loop)**: Local Ollama (`127.0.0.1:11434/:11435`), OpenAI-compatible (`NALA-Parallax-RS :4000`), or Anthropic.
- **`.traces/` Query-Path Replay & Multi-Profile Isolation**: Compact step notation (`search "rate limit" (2) → read billing-api.md → ✓`) and isolated bundles with `usk_...` API keys.

## 2. `sHeriFF + NVIDIA SkillSpector` Security Fixes
- **`SEC-UNDERSTORY-01`**: Localhost `127.0.0.1:3800` default bind + strict `Origin:` CSRF verification (eliminates upstream `cors({ origin: true })` wildcard reflection + `0.0.0.0:3800` open bind).
- **`SEC-UNDERSTORY-02`**: Native atomic Rust file operations without `git config --system safe.directory '*'` or subprocess shell-outs.
- **`SEC-UNDERSTORY-03`**: Data-only Rust YAML frontmatter parser (blocks `gray-matter` `---js` execution) + canonical symlink sandbox check.

## 3. Quickstart
```bash
nala-understory-rs --bench
nala-understory-rs --status --lang en
nala-understory-rs --dream --lang en
nala-understory-rs --serve 127.0.0.1:3800
nala-understory-rs --mcp-stdio --bundle /path/to/bundle
```
