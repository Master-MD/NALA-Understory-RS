# 🪨 CAVEMAN GUIDE — `NALA-Understory-RS` (`DE / FR / IT / RM / EN`)

> **Why Agent Forget? Why Agent Lie? How Rust Make Markdown Brain Fast!**

---

## 🇨🇭 Deutsch (DE) — Höhlenmensch-Erklärung
- **Problem**: KI-Agent hat Goldfisch-Gedächtnis. Neue Sitzung -> alles vergessen. Oder alte Notizen lesen -> Quatsch erzählen (Drift).
- **Lösung (`Understory` / `OKF v0.1`)**: Wissen liegt in normalen `.md` Textdateien. Jede Datei zeigt mit Pfeilen (`related`) auf andere Dateien. Jeder Ordner hat automatisch eine Inhalts-Tabelle (`index.md`) und ein Tagebuch (`log.md`).
- **Warum Rust (`NALA-Understory-RS`)?**:
  - Alte Version brauchte `Node.js` (`118 MB` RAM) und lief dreimal langsam über die Festplatte (`3.84 ms`).
  - Neue Rust-Version ist **1 winzige Datei (`~430 KB`)**, braucht nur **`1.9 MB` RAM** und prüft den gesamten Wissens-Graphen in **`0.057 ms` (bis zu 66.5x schneller)**!
- **3 Befehle reichen**:
  1. `nala-understory-rs --bench` -> Zeigt wie schnell Rust rennt!
  2. `nala-understory-rs --status` -> Prüft ob alle Notizen sauber verlinkt sind!
  3. `nala-understory-rs --serve` -> Öffnet das 5-Sprachen Web-Cockpit auf `http://127.0.0.1:3800`!

---

## 🇫🇷 Français (FR) — Guide Homme des Cavernes
- **Problème** : L'agent IA oublie tout à chaque session ou lit de vieilles notes cassées.
- **Solution** : Des fichiers `.md` simples (`OKF v0.1`) reliés en graphe avec `index.md` et `log.md` automatiques.
- **Pourquoi Rust ?** : Un seul binaire de `~430 Ko`, zéro Node.js, **66.5x plus rapide** (`57.7 µs`), `1.9 Mo` de RAM.
- **Commandes** : `nala-understory-rs --bench` | `nala-understory-rs --status --lang fr` | `nala-understory-rs --serve`

---

## 🇮🇹 Italiano (IT) — Guida Uomo delle Caverne
- **Problema**: L'agente IA dimentica tutto tra una sessione e l'altra.
- **Soluzione**: File Markdown semplici (`OKF v0.1`) collegati in un grafo vivente con `index.md` e `log.md` automatici.
- **Perché Rust?**: Un solo eseguibile da `~430 KB`, zero Node.js, **66.5x più veloce** (`57.7 µs`), `1.9 MB` di RAM.
- **Comandi**: `nala-understory-rs --bench` | `nala-understory-rs --status --lang it` | `nala-understory-rs --serve`

---

## 🏔️ Rumantsch (RM) — Guida Uman da lasTaunas
- **Problem**: L'agent d'IA emblida tut tranter las sessiuns.
- **Soluziun**: Datotecas Markdown simplas (`OKF v0.1`) colliadas en in graf vivent cun `index.md` e `log.md` automatics.
- **Pertge Rust?**: Ina suletta datoteca da `~430 KB`, nagin Node.js, **66.5x pli spert** (`57.7 µs`), `1.9 MB` RAM.
- **Cumonds**: `nala-understory-rs --bench` | `nala-understory-rs --status --lang rm` | `nala-understory-rs --serve`

---

## 🇬🇧 English (EN) — Caveman Guide
- **Problem**: AI agent has goldfish brain. New chat = total amnesia. Old notes = broken links and hallucination.
- **Solution (`OKF v0.1`)**: Plain `.md` files with auto-wired links, auto-generated directory tables (`index.md`), and append-only changelogs (`log.md`).
- **Why Rust (`NALA-Understory-RS`)?**: Single `~430 KB` binary, zero Node.js, **up to 66.5x faster** (`57.7 µs` full graph + lint + validation), and only `1.9 MB` RAM.
- **3 Commands**: `nala-understory-rs --bench` | `nala-understory-rs --status --lang en` | `nala-understory-rs --serve`
