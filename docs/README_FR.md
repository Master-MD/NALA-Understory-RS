# 🌱🦀 NALA-Understory-RS (`v0.9.0-nala.6.6.6`) — Documentation Française (FR)

**100% Rust-Turbo Natif (`Edition 2024`, Zéro-Node.js, Zéro-Python, `0` crate externe, binaire `~899 Ko`, `36` outils MCP statiques, `38/38` tests unitaires, `39` sous-systèmes)**

## 🎯 Nouveautés Vague-8 (`v0.9.0-nala.6.6.6` · `RSI-UNDERSTORY-029..032`)
- **`RSI-029` (`--migrate-export`)** : Migrateur Clean-Room **Notion / Confluence HTML / Jira / Linear** → `OKF v0.1` en **`0.71 µs`** avec filtrage XSS/SSRF.
- **`RSI-030` (`--detect-conflicts`)** : Détecteur sémantique de contradictions et dates expirées en **`1.10 µs`** (**0 appel LLM**, citations Evidence-Lock `[chemin#Lligne]`).
- **`RSI-031` (`--patch-bundle`)** : Archives delta `.okf.patch` signées **HMAC-SHA256 (RFC 2104)** en **`4.67 µs`** avec vérification à temps constant, anti-rejeu et filtrage par zone.
- **`RSI-032` (`--cache-telemetry`)** : Télémétrie de préfixe KV-Cache et diff déterministe en **`1 199.5 ns`** (`>= 98%` de taux de succès de préfixe).

Binaires vérifiés (`SHA256SUMS.txt`) : [GitHub Releases](https://github.com/Master-MD/NALA-Understory-RS/releases/tag/v0.9.0-nala.6.6.6). Licence : Apache-2.0.
