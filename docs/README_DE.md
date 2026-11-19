# 🌱🦀 NALA-Understory-RS (`v0.9.0-nala.6.6.6`) — Deutsche Dokumentation (DE)

**100% Native Rust-Turbo (`Edition 2024`, Zero-Node.js, Zero-Python, `0` externe Crates, `~899 KB` Single Binary, `36` statische MCP-Tools, `38/38` Tests, `39` Benchmark-Subsysteme)**

## 🎯 Neu in Wave-8 (`v0.9.0-nala.6.6.6` · `RSI-UNDERSTORY-029..032`)
- **`RSI-029` (`--migrate-export`)**: Clean-Room Migrator für **Notion Markdown/CSV**, **Confluence HTML** und **Jira/Linear JSON** → `OKF v0.1` in **`0.71 µs/Dok`** (inkl. HTML/XSS/SSRF-Sanitizer & Notion-UUID-Strip).
- **`RSI-030` (`--detect-conflicts`)**: Zero-Allocation Widerspruchs- & Stale-Claim-Detektor in **`1.10 µs`** mit **0 LLM-Aufrufen** und Evidence-Lock `[pfad#Lzeile]`-Zitaten.
- **`RSI-031` (`--patch-bundle`)**: Kryptografische **RFC 2104 HMAC-SHA256** signierte `.okf.patch` Multi-Node Airgap-Sync-Archive in **`4.67 µs`** mit Constant-Time-MAC, Anti-Replay-Nonce & Zonen-Allowlist.
- **`RSI-032` (`--cache-telemetry`)**: Deterministische Token-Budget Kontext-Diff & **Prompt-Cache Prefix-Pinning Telemetrie** in **`1'199.5 ns`** (`>= 98%` KV-Cache-Hit-Rate, Zonen-gebundener Prefix-Hash).

## ⚡ Verifizierte Multi-Plattform-Benchmarks (`v0.9.0-nala.6.6.6`)
- **M4 Max (`arm64`)**: `RSI-029`: **`0.71 µs`** · `RSI-030`: **`1.10 µs`** · `RSI-031`: **`4.67 µs`** · `RSI-032`: **`1'199.5 ns`**
- **Linux x86_64 Server**: `RSI-029`: **`2.37 µs`** · `RSI-030`: **`3.30 µs`** · `RSI-031`: **`12.20 µs`** · `RSI-032`: **`1'758.1 ns`**
- **Proxmox VE 9.2 Airgap (`unshare -n env -i`)**: `RSI-029`: **`2.49 µs`** · `RSI-030`: **`3.54 µs`** · `RSI-031`: **`13.81 µs`** · `RSI-032`: **`1'874.7 ns`**

Alle 5 Plattform-Binaries inkl. `SHA256SUMS.txt` stehen unter [Releases](https://github.com/Master-MD/NALA-Understory-RS/releases/tag/v0.9.0-nala.6.6.6) bereit. Lizenz: Apache-2.0.
