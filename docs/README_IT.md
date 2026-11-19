# 🌱🦀 NALA-Understory-RS (`v0.9.0-nala.6.6.6`) — Documentazione Italiana (IT)

**100% Rust-Turbo Nativo (`Edition 2024`, Zero-Node.js, Zero-Python, `0` dipendenze esterne, binario `~899 KB`, `36` strumenti MCP statici, `38/38` test, `39` sottosistemi)**

## 🎯 Novità Ondata-8 (`v0.9.0-nala.6.6.6` · `RSI-UNDERSTORY-029..032`)
- **`RSI-029` (`--migrate-export`)**: Migratore Clean-Room **Notion / Confluence HTML / Jira / Linear** → `OKF v0.1` in **`0.71 µs`** con filtro XSS/SSRF.
- **`RSI-030` (`--detect-conflicts`)**: Rilevatore deterministico di contraddizioni e date scadute in **`1.10 µs`** (**0 chiamate LLM**, citazioni Evidence-Lock `[percorso#Lriga]`).
- **`RSI-031` (`--patch-bundle`)**: Pacchetti delta `.okf.patch` firmati **HMAC-SHA256 (RFC 2104)** in **`4.67 µs`** con verifica constant-time, anti-replay e filtro di zona.
- **`RSI-032` (`--cache-telemetry`)**: Telemetria prefisso KV-Cache e diff deterministico in **`1.199,5 ns`** (`>= 98%` hit rate).

Binari verificati (`SHA256SUMS.txt`): [GitHub Releases](https://github.com/Master-MD/NALA-Understory-RS/releases/tag/v0.9.0-nala.6.6.6). Licenza: Apache-2.0.
