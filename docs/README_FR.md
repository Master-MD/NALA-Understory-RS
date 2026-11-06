# 🇫🇷 NALA-Understory-RS (`v0.1.0-nala.6.6.6`) — Documentation Française (FR)

> **Portage 100% Rust Natif (`Edition 2024`, Zéro Node.js, Zéro Python, Zéro Télémétrie) et Évolution Sécurisée de [`thecodacus/understory`](https://github.com/thecodacus/understory)**

## 1. Qu'est-ce que `NALA-Understory-RS` ?
**NALA-Understory-RS** transforme un simple dossier de fichiers Markdown bruts au format **Open Knowledge Format (`OKF v0.1`)** en une **mémoire d'agent auto-maillée et protégée contre la dérive documentaire** :
- **Aucune boîte noire** : Chaque concept est un fichier `.md` lisible et versionnable dans Git avec un en-tête YAML (`type` requis, plus `description`, `owners`, `tags`, `sources`, `related`, `updated`).
- **Scan Arène en Une Passe & Linter Anti-Dérive** : Remplace les 3 parcours disque distincts de Node.js par un scan mémoire unique (`57.7 µs` sous Linux `x86_64`, `105.3 µs` sur Apple M4 Max — jusqu'à **66.5x plus rapide**).
- **Cache de Requêtes à 3 Niveaux** :
  1. **Couche 1 (Empreinte Exacte 64-bit)** : Réponse instantanée sans appel LLM.
  2. **Couche 2 (Ensemble de Travail Chaud)** : 10 concepts les plus actifs + 10 dernières Q&R avec invalidation immédiate à l'écriture.
  3. **Couche 3 (Boucle Agent Profonde)** : Ollama local (`:11434/:11435`), OpenAI/LiteLLM (`:4000`) ou Anthropic.
- **Traces `.traces/` & Isolation Multi-Profils (`usk_...`)** : Rejeu complet du chemin de recherche (`search "rate limit" (2) → read billing-api.md → ✓`) et clés API isolées par profil.

## 2. Durcissement Sécurité (`sHeriFF + NVIDIA SkillSpector`)
- **`SEC-UNDERSTORY-01`** : Écoute par défaut sur `127.0.0.1:3800` et pare-feu CSRF strict sur l'en-tête `Origin:` (contre le `cors({ origin: true })` permissif d'origine).
- **`SEC-UNDERSTORY-02`** : Opérations fichiers atomiques 100% Rust sans sous-processus Git ni `safe.directory '*'`.
- **`SEC-UNDERSTORY-03`** : Parseur YAML sans exécution JavaScript (`---js`) et vérification canonique des liens symboliques.

## 3. Démarrage Rapide
```bash
nala-understory-rs --bench
nala-understory-rs --status --lang fr
nala-understory-rs --dream --lang fr
nala-understory-rs --serve 127.0.0.1:3800
nala-understory-rs --mcp-stdio --bundle /chemin/vers/bundle
```
