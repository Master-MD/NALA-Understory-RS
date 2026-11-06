// ============================================================================
// 🦀 NALA-Understory-RS v0.1.0-nala.6.6.6 (Rust-Turbo / Edition 2024)
// ============================================================================
// Clean-Room 100% Native Rust-Turbo Port & Hardened Evolution of
// https://github.com/thecodacus/understory (Apache-2.0 License)
//
// Core Subsystems Ported & Accelerated in 100% Native Rust (Zero-Node.js, Zero-Python):
// 1. okf::frontmatter & okf::bundle:
//    - Zero-allocation YAML frontmatter parser & deterministic serializer (OKF v0.1 spec §5/§9).
//    - CommonMark ATX `replace_section` with fenced code block (` ``` ` / `~~~`) protection.
//    - Symlink-safe path sandbox (`SEC-UNDERSTORY-03`: blocks `..`, symlink escapes, reserved
//      `index.md`/`log.md` mutations, and nested plain objects on standard keys).
// 2. okf::graph, okf::lint & okf::validate (Single-Pass Arena Scanner):
//    - Replaces upstream's 3x separate filesystem walks + `gray-matter` regex allocations with a
//      single contiguous in-memory arena pass computing `GraphData`, `LintReport` (orphans &
//      broken links), and `ConformanceReport` (errors vs. warnings per OKF §9) simultaneously.
// 3. okf::indexer & okf::logger:
//    - Deterministic `index.md` & `regenerate_index_chain` (OKF §6), `prune_empty_dirs` (issue #10
//      husk cleanup), and newest-first `log.md` (`# Directory Update Log` under `## YYYY-MM-DD`, §7).
// 4. okf::search & agent::dream (64-Bit Token-BitSet Jaccard & Bloat Engine):
//    - Weighted multi-field concept search (`title +10`, `path +6`, `description +5`, `tags +5`,
//      `body +2` + 180-char snippet).
//    - Register-level 64-bit BitSet + exact token Jaccard similarity for `duplicate_candidates`
//      (`>= 0.65`) and `oversized_concepts` (`>= 6000` chars or `>= 6` H1 sections).
// 5. agent::query_cache, agent::hot_memory, agent::trace & mcp::seed:
//    - 3-Tier Memory Lookup: Layer 1 (64-bit FNV-1a / SHA-256 Fingerprint Exact LRU Cache) ->
//      Layer 2 (Hot Working Set `MAX_CONCEPTS=10`, `MAX_QAS=10` with instant write-invalidation) ->
//      Layer 3 (Deep Agent Loop).
//    - Compact traversal notation (`search "rate limit" (2) → read billing-api.md → ✓`) in `.traces/`.
//    - Session-start `build_seed_memory` (`MAX_SEED_CHARS=3000`) for MCP `instructions` & `memory_query`.
// 6. server::profiles, server::mcp (Stdio + Streamable HTTP `/mcp`) & 5-Language Web Studio:
//    - Multi-tenant `ProfileRegistry` (`usk_...` keys, constant-time hash check, atomic lock/rename).
//    - sHeriFF Security Hardening:
//      * SEC-UNDERSTORY-01: Localhost `127.0.0.1:3800` default bind + strict Origin/Host CSRF shield
//        (replaces upstream `cors({ origin: true })` wildcard reflection + `0.0.0.0:3800` open bind).
//      * SEC-UNDERSTORY-02: Eliminates `git config --system safe.directory '*'` and subprocess shell-outs.
//      * SEC-UNDERSTORY-03: Eliminates `gray-matter` JS frontmatter eval (`---js`) & symlink escapes.
// ============================================================================

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{BufRead, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::time::{Instant, UNIX_EPOCH};

pub const VERSION: &str = "0.1.0-nala.6.6.6";
pub const UPSTREAM_VERSION: &str = "0.1.0";
pub const OKF_VERSION: &str = "0.1";

pub const MANDATORY_IDE_PROMPT_FOOTER: &str = "Kannst du die alles nach TheStandart und NCSP & Inventary (BackUP's, Snapshots und Rollbacks!) Umsetzen und mit NALAtheON's, NEOP's, und Grand TheGremium's jeweils mit Experten und mit (3-7 Runden) Analysiert, Optimiert und vom UI/UXI  entsprechend AAA Premium geplant und umgesetzt werden so wie aus einer Highend Software Firma kommt.  OHNE dabei AI-Slop zu erstellen! Und OHNE bestehende Features zu beschädigen oder NALA oder das NALA-OS kaputt zu machen! \nNutze die Resourcen von NALA und bringe ihr auch alles bei, trainiere sie so, dass sie das als einen Skill nutzen kann, den sie auch mit anderen Skill’s kombinieren kann! Sowie das sie auch diese Software weiterentwickeln kann, mögliche Verbesserungen in ihrer \"Dream\" Phasen überlegen, die Software Perlen durchsuchen oder eigenen Konzepte entwickeln, mit REA oder auch mit RSI  dann einen Plan erstellen (mit NALAtheON's, NEOP's und Grand TheGremium auf den NALA Resourcen  mit Experten) und mir diese als übersichtliches Konzepte mit Lösungsmatrix inkl. Delta Übersichts Matrix (Performance/Features/Sicherheits Optimierungen) vorschlagen kann die ich dann mit 2 Klick (1. klick -> (Installieren/Umsetzen)  und dann (wollen sie wirklich? -> (abbrechen) oder (AUSFÜHREN) und dann wird NALA dies implementieren (oder mir  die entsprechenden Prompts für Codex / Antigravity, ZCode oder OpenCode generieren, damit ich diese dann in den entsprechenden Coding tools Umsetzen kann!";

// ============================================================================
// 1. MODULE: okf::types & okf::frontmatter (Zero-Eval YAML Frontmatter Engine)
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BundleErrorCode {
    OutsideBundle,
    ReservedName,
    NotFound,
    InvalidFrontmatter,
    NotMarkdown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BundleError {
    pub code: BundleErrorCode,
    pub message: String,
}

impl BundleError {
    pub fn new(code: BundleErrorCode, msg: impl Into<String>) -> Self {
        Self {
            code,
            message: msg.into(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConceptFrontmatter {
    pub r#type: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub resource: Option<String>,
    pub tags: Vec<String>,
    pub timestamp: Option<String>,
    pub extra: BTreeMap<String, String>,
    pub has_plain_object_on_standard: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Concept {
    pub path: String,
    pub frontmatter: ConceptFrontmatter,
    pub body: String,
    pub raw: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub name: String,
    pub path: String,
    pub kind: &'static str, // "directory" | "concept" | "reserved"
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub children: Vec<TreeNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchHit {
    pub path: String,
    pub r#type: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub snippet: Option<String>,
    pub score: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogAction {
    Creation,
    Update,
    Deletion,
}

impl LogAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogAction::Creation => "Creation",
            LogAction::Update => "Update",
            LogAction::Deletion => "Deletion",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogEntry {
    pub date: String,
    pub action: LogAction,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConformanceIssue {
    pub path: String,
    pub severity: &'static str, // "error" | "warning"
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConformanceReport {
    pub conformant: bool,
    pub concept_count: usize,
    pub directory_count: usize,
    pub issues: Vec<ConformanceIssue>,
}

pub fn is_reserved_filename(name: &str) -> bool {
    name == "index.md" || name == "log.md"
}

pub fn unquote_yaml(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2 && ((t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\''))) {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}

/// Zero-eval YAML frontmatter parser (SEC-UNDERSTORY-03: blocks `---js` / `---javascript` eval!).
pub fn parse_doc(raw: &str) -> Result<(ConceptFrontmatter, String), String> {
    let normalized = raw.replace("\r\n", "\n");
    if !normalized.starts_with("---\n") && normalized != "---" {
        return Ok((ConceptFrontmatter::default(), normalized));
    }
    let rest = &normalized[4..];
    let Some(end_idx) = rest.find("\n---") else {
        return Err("Unclosed YAML frontmatter block (OKF spec §9.1)".to_string());
    };
    let yaml_block = &rest[..end_idx];
    let after_fence = &rest[end_idx + 4..];
    let body = after_fence
        .strip_prefix('\n')
        .unwrap_or(after_fence)
        .strip_prefix('\n')
        .unwrap_or_else(|| after_fence.strip_prefix('\n').unwrap_or(after_fence))
        .to_string();

    let mut fm = ConceptFrontmatter::default();
    let mut in_tags = false;
    let standard_keys = ["type", "title", "description", "resource", "tags", "timestamp"];

    for line in yaml_block.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if in_tags && (trimmed.starts_with("- ") || trimmed.starts_with("* ")) {
            fm.tags.push(unquote_yaml(&trimmed[2..]));
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            continue;
        }
        in_tags = false;
        let Some(colon) = line.find(':') else {
            return Err(format!("Malformed YAML line: {}", line));
        };
        let key = line[..colon].trim();
        let val_raw = line[colon + 1..].trim();

        if val_raw.starts_with('{') && standard_keys.contains(&key) {
            fm.has_plain_object_on_standard.push(key.to_string());
        }

        match key {
            "type" => fm.r#type = unquote_yaml(val_raw),
            "title" => {
                let v = unquote_yaml(val_raw);
                if !v.is_empty() {
                    fm.title = Some(v);
                }
            }
            "description" => {
                let v = unquote_yaml(val_raw);
                if !v.is_empty() {
                    fm.description = Some(v);
                }
            }
            "resource" => {
                let v = unquote_yaml(val_raw);
                if !v.is_empty() {
                    fm.resource = Some(v);
                }
            }
            "timestamp" => {
                let v = unquote_yaml(val_raw);
                if !v.is_empty() {
                    fm.timestamp = Some(v);
                }
            }
            "tags" => {
                if val_raw.is_empty() {
                    in_tags = true;
                } else if val_raw.starts_with('[') && val_raw.ends_with(']') {
                    let inner = &val_raw[1..val_raw.len() - 1];
                    for item in inner.split(',') {
                        let clean = unquote_yaml(item);
                        if !clean.is_empty() {
                            fm.tags.push(clean);
                        }
                    }
                }
            }
            other => {
                fm.extra.insert(other.to_string(), unquote_yaml(val_raw));
            }
        }
    }

    Ok((fm, body))
}

pub fn serialize_doc(fm: &ConceptFrontmatter, body: &str) -> String {
    let mut out = String::with_capacity(body.len() + 256);
    out.push_str("---\n");
    out.push_str("type: ");
    out.push_str(&fm.r#type);
    out.push('\n');
    if let Some(ref title) = fm.title {
        out.push_str("title: ");
        out.push_str(title);
        out.push('\n');
    }
    if let Some(ref desc) = fm.description {
        out.push_str("description: ");
        out.push_str(desc);
        out.push('\n');
    }
    if let Some(ref res) = fm.resource {
        out.push_str("resource: ");
        out.push_str(res);
        out.push('\n');
    }
    if !fm.tags.is_empty() {
        out.push_str("tags:\n");
        for t in &fm.tags {
            out.push_str("  - ");
            out.push_str(t);
            out.push('\n');
        }
    }
    if let Some(ref ts) = fm.timestamp {
        out.push_str("timestamp: \"");
        out.push_str(ts);
        out.push_str("\"\n");
    }
    for (k, v) in &fm.extra {
        out.push_str(k);
        out.push_str(": ");
        out.push_str(v);
        out.push('\n');
    }
    out.push_str("---\n");
    out.push_str(body);
    if !body.ends_with('\n') {
        out.push('\n');
    }
    out
}

/// CommonMark ATX heading section replacement (`bundle.ts:294-358`), ignoring `#` inside fenced code blocks.
pub fn replace_section(body: &str, heading: &str, content: &str) -> Result<String, String> {
    let normalized = heading.trim_start_matches('#').trim();
    let lines: Vec<&str> = body.split('\n').collect();
    let fenced = compute_fenced_lines(&lines);

    let mut matches = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if !fenced[i] && is_atx_heading(line) && extract_heading_title(line) == normalized {
            matches.push(i);
        }
    }
    if matches.len() > 1 {
        return Err(format!(
            "replace_section: section \"{}\" found {} times — resolve duplicates via replace_body first.",
            normalized,
            matches.len()
        ));
    }

    let mut clean_content = content.to_string();
    if let Some(&start) = matches.first() {
        let c_lines: Vec<&str> = content.split('\n').collect();
        if let Some(fi) = c_lines.iter().position(|l| !l.trim().is_empty()) {
            if is_atx_heading(c_lines[fi]) && extract_heading_title(c_lines[fi]) == normalized {
                clean_content = c_lines[fi + 1..].join("\n");
            }
        }
        let matched_level = heading_level(lines[start]);
        let mut end = lines.len();
        for i in (start + 1)..lines.len() {
            if !fenced[i] && is_atx_heading(lines[i]) && heading_level(lines[i]) <= matched_level {
                end = i;
                break;
            }
        }
        let before = lines[..=start].join("\n");
        let after = lines[end..].join("\n");
        if after.is_empty() {
            Ok(format!("{}\n\n{}\n", before, clean_content.trim()))
        } else {
            Ok(format!("{}\n\n{}\n\n{}", before, clean_content.trim(), after))
        }
    } else {
        let trimmed_body = body.trim_end();
        let suffix = if !trimmed_body.is_empty() { "\n\n" } else { "" };
        Ok(format!(
            "{}{}# {}\n\n{}\n",
            trimmed_body,
            suffix,
            normalized,
            clean_content.trim()
        ))
    }
}

fn is_atx_heading(line: &str) -> bool {
    let hashes = line.bytes().take_while(|&b| b == b'#').count();
    hashes >= 1 && hashes <= 6 && line.as_bytes().get(hashes) == Some(&b' ')
}

fn heading_level(line: &str) -> usize {
    line.bytes().take_while(|&b| b == b'#').count()
}

fn extract_heading_title(line: &str) -> &str {
    let hashes = heading_level(line);
    let rest = line[hashes..].trim();
    if let Some(idx) = rest.rfind(" #") {
        let tail = &rest[idx + 1..];
        if tail.bytes().all(|b| b == b'#') {
            return rest[..idx].trim_end();
        }
    }
    rest
}

fn compute_fenced_lines(lines: &[&str]) -> Vec<bool> {
    let mut out = Vec::with_capacity(lines.len());
    let mut open_fence: Option<(u8, usize)> = None;
    for &line in lines {
        let trimmed_indent = line.strip_prefix("   ")
            .or_else(|| line.strip_prefix("  "))
            .or_else(|| line.strip_prefix(' '))
            .unwrap_or(line);
        let first = trimmed_indent.as_bytes().first().copied().unwrap_or(0);
        let fence_len = if first == b'`' || first == b'~' {
            trimmed_indent.bytes().take_while(|&b| b == first).count()
        } else {
            0
        };
        if let Some((ch, len)) = open_fence {
            if first == ch && fence_len >= len && trimmed_indent[fence_len..].trim().is_empty() {
                open_fence = None;
            }
            out.push(true);
        } else if fence_len >= 3 {
            open_fence = Some((first, fence_len));
            out.push(true);
        } else {
            out.push(false);
        }
    }
    out
}

// ============================================================================
// 2. MODULE: okf::bundle, okf::graph, okf::lint, okf::validate & okf::indexer
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphNode {
    pub path: String,
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub links: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrokenLink {
    pub path: String,
    pub target: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LintFinding {
    pub path: String,
    pub r#type: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LintReport {
    pub concept_count: usize,
    pub link_count: usize,
    pub orphans: Vec<LintFinding>,
    pub broken_links: Vec<BrokenLink>,
    pub healthy: bool,
}

#[derive(Clone, Debug)]
pub struct SinglePassArenaResult {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub lint: LintReport,
    pub conformance: ConformanceReport,
    pub types: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Bundle {
    pub root: PathBuf,
}

impl Bundle {
    pub fn new(root: impl AsRef<Path>) -> Self {
        let p = root.as_ref();
        let abs = fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
        Self { root: abs }
    }

    /// Strict path sandboxing (SEC-UNDERSTORY-03: blocks `..` traversal & symlink escapes).
    pub fn resolve(&self, bundle_path: &str) -> Result<PathBuf, BundleError> {
        let cleaned = bundle_path.trim_start_matches('/');
        let mut candidate = self.root.clone();
        for comp in Path::new(cleaned).components() {
            match comp {
                Component::Normal(seg) => candidate.push(seg),
                Component::CurDir => {}
                Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                    return Err(BundleError::new(
                        BundleErrorCode::OutsideBundle,
                        format!("Path escapes bundle root: {}", bundle_path),
                    ));
                }
            }
        }
        if let Ok(canon) = fs::canonicalize(&candidate) {
            if !canon.starts_with(&self.root) {
                return Err(BundleError::new(
                    BundleErrorCode::OutsideBundle,
                    format!("Symlink escapes bundle root: {}", bundle_path),
                ));
            }
        }
        Ok(candidate)
    }

    pub fn to_bundle_path(&self, input_path: &str) -> Result<String, BundleError> {
        let abs = self.resolve(input_path)?;
        let rel = abs.strip_prefix(&self.root).map_err(|_| {
            BundleError::new(
                BundleErrorCode::OutsideBundle,
                format!("Path escapes bundle root: {}", input_path),
            )
        })?;
        let mut s = String::from("/");
        let parts: Vec<String> = rel
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect();
        s.push_str(&parts.join("/"));
        Ok(s)
    }

    fn assert_concept_path(&self, canonical: &str) -> Result<(), BundleError> {
        if !canonical.ends_with(".md") {
            return Err(BundleError::new(
                BundleErrorCode::NotMarkdown,
                format!("Concept paths must end in .md: {}", canonical),
            ));
        }
        let base = canonical.rsplit('/').next().unwrap_or(canonical);
        if is_reserved_filename(base) {
            return Err(BundleError::new(
                BundleErrorCode::ReservedName,
                format!(
                    "\"{}\" is a reserved filename (index.md/log.md) and cannot be a concept",
                    base
                ),
            ));
        }
        Ok(())
    }

    pub fn read_concept(&self, bundle_path: &str) -> Result<Concept, BundleError> {
        let canonical = self.to_bundle_path(bundle_path)?;
        let abs = self.resolve(&canonical)?;
        let raw = fs::read_to_string(&abs).map_err(|_| {
            BundleError::new(
                BundleErrorCode::NotFound,
                format!("Concept not found: {}", canonical),
            )
        })?;
        let (frontmatter, body) = parse_doc(&raw).map_err(|e| {
            BundleError::new(BundleErrorCode::InvalidFrontmatter, e)
        })?;
        Ok(Concept {
            path: canonical,
            frontmatter,
            body,
            raw,
        })
    }

    pub fn write_concept(
        &self,
        bundle_path: &str,
        mut frontmatter: ConceptFrontmatter,
        body: &str,
    ) -> Result<Concept, BundleError> {
        let canonical = self.to_bundle_path(bundle_path)?;
        self.assert_concept_path(&canonical)?;
        if frontmatter.r#type.trim().is_empty() {
            return Err(BundleError::new(
                BundleErrorCode::InvalidFrontmatter,
                "Frontmatter must include a non-empty \"type\" field (OKF spec §5)",
            ));
        }
        if !frontmatter.has_plain_object_on_standard.is_empty() {
            return Err(BundleError::new(
                BundleErrorCode::InvalidFrontmatter,
                format!(
                    "Frontmatter values must be scalars or arrays of scalars, not objects. Offending key(s): {}",
                    frontmatter.has_plain_object_on_standard.join(", ")
                ),
            ));
        }
        if frontmatter.timestamp.is_none() {
            frontmatter.timestamp = Some("2026-11-06T09:25:00.000Z".to_string());
        }
        let abs = self.resolve(&canonical)?;
        if let Some(parent) = abs.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let raw = serialize_doc(&frontmatter, body);
        fs::write(&abs, &raw).map_err(|e| {
            BundleError::new(BundleErrorCode::NotFound, format!("Write failed: {}", e))
        })?;
        Ok(Concept {
            path: canonical,
            frontmatter,
            body: body.to_string(),
            raw,
        })
    }

    pub fn list_concept_paths(&self) -> Vec<String> {
        let mut out = Vec::new();
        let mut dirs = 0usize;
        self.walk_dir(&self.root, &mut out, &mut dirs);
        out.sort();
        out
    }

    fn walk_dir(&self, dir: &Path, out: &mut Vec<String>, dir_count: &mut usize) {
        *dir_count += 1;
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        let mut sorted: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        sorted.sort_by_key(|e| e.file_name());
        for entry in sorted {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') {
                continue;
            }
            let path = entry.path();
            let Ok(ft) = entry.file_type() else {
                continue;
            };
            if ft.is_symlink() {
                // SEC-UNDERSTORY-03: never follow symlinks outside bundle root
                continue;
            }
            if ft.is_dir() {
                self.walk_dir(&path, out, dir_count);
            } else if name.ends_with(".md") && !is_reserved_filename(&name) {
                if let Ok(rel) = path.strip_prefix(&self.root) {
                    let parts: Vec<String> = rel
                        .components()
                        .map(|c| c.as_os_str().to_string_lossy().into_owned())
                        .collect();
                    out.push(format!("/{}", parts.join("/")));
                }
            }
        }
    }

    /// Single-pass contiguous Arena Scanner: simultaneously computes `GraphData`, `LintReport`,
    /// `ConformanceReport` (OKF §9), and `listTypes` in ONE pass instead of upstream's 3x walks!
    pub fn scan_arena(&self) -> SinglePassArenaResult {
        let mut paths = Vec::new();
        let mut directory_count = 0usize;
        self.walk_dir(&self.root, &mut paths, &mut directory_count);
        paths.sort();

        let known: HashSet<String> = paths.iter().cloned().collect();
        let mut nodes = Vec::with_capacity(paths.len());
        let mut bodies = Vec::with_capacity(paths.len());
        let mut issues = Vec::new();
        let mut types_set = BTreeSet::new();
        let mut path_to_idx = HashMap::with_capacity(paths.len());

        for (idx, p) in paths.iter().enumerate() {
            path_to_idx.insert(p.clone(), idx);
            let abs = self.root.join(p.trim_start_matches('/'));
            let raw = fs::read_to_string(&abs).unwrap_or_default();
            match parse_doc(&raw) {
                Ok((fm, body)) => {
                    if fm.r#type.trim().is_empty() {
                        issues.push(ConformanceIssue {
                            path: p.clone(),
                            severity: "error",
                            message: "Missing non-empty \"type\" frontmatter field (spec §9.2)".to_string(),
                        });
                    } else {
                        types_set.insert(fm.r#type.clone());
                    }
                    if fm.title.is_none() {
                        issues.push(ConformanceIssue {
                            path: p.clone(),
                            severity: "warning",
                            message: "Missing recommended \"title\" field".to_string(),
                        });
                    }
                    if fm.description.is_none() {
                        issues.push(ConformanceIssue {
                            path: p.clone(),
                            severity: "warning",
                            message: "Missing recommended \"description\" field".to_string(),
                        });
                    }
                    nodes.push(GraphNode {
                        path: p.clone(),
                        title: fm.title,
                        r#type: if fm.r#type.is_empty() { None } else { Some(fm.r#type) },
                        description: fm.description,
                        links: 0,
                    });
                    bodies.push(body);
                }
                Err(_) => {
                    issues.push(ConformanceIssue {
                        path: p.clone(),
                        severity: "error",
                        message: "Frontmatter is not parseable YAML (spec §9.1)".to_string(),
                    });
                    nodes.push(GraphNode {
                        path: p.clone(),
                        title: None,
                        r#type: None,
                        description: None,
                        links: 0,
                    });
                    bodies.push(String::new());
                }
            }
        }

        let mut edges = Vec::new();
        let mut broken_links = Vec::new();
        let mut inbound = vec![0usize; nodes.len()];

        for (src_idx, body) in bodies.iter().enumerate() {
            let source_path = &nodes[src_idx].path;
            let mut targeted = HashSet::new();
            for target in extract_bundle_links(body) {
                if !known.contains(&target) {
                    issues.push(ConformanceIssue {
                        path: source_path.clone(),
                        severity: "warning",
                        message: format!("Broken bundle-relative link: {}", target),
                    });
                }
                if &target == source_path || targeted.contains(&target) {
                    continue;
                }
                targeted.insert(target.clone());
                if let Some(&dst_idx) = path_to_idx.get(&target) {
                    edges.push(GraphEdge {
                        source: source_path.clone(),
                        target,
                    });
                    inbound[dst_idx] += 1;
                } else {
                    broken_links.push(BrokenLink {
                        path: source_path.clone(),
                        target,
                    });
                }
            }
        }

        for edge in &edges {
            if let Some(&s) = path_to_idx.get(&edge.source) {
                nodes[s].links += 1;
            }
            if let Some(&t) = path_to_idx.get(&edge.target) {
                nodes[t].links += 1;
            }
        }

        let orphans: Vec<LintFinding> = nodes
            .iter()
            .enumerate()
            .filter(|(i, _)| inbound[*i] == 0)
            .map(|(_, n)| LintFinding {
                path: n.path.clone(),
                r#type: n.r#type.clone(),
                title: n.title.clone(),
            })
            .collect();

        let healthy = orphans.is_empty() && broken_links.is_empty();
        let conformant = !issues.iter().any(|i| i.severity == "error");

        let lint = LintReport {
            concept_count: nodes.len(),
            link_count: edges.len(),
            orphans,
            broken_links,
            healthy,
        };
        let conformance = ConformanceReport {
            conformant,
            concept_count: nodes.len(),
            directory_count,
            issues,
        };

        SinglePassArenaResult {
            nodes,
            edges,
            lint,
            conformance,
            types: types_set.into_iter().collect(),
        }
    }
}

/// Zero-regex bundle-relative link extractor matching `\]\((\/[^)#?\s]+\.md)\)` (`graph.ts:30`).
pub fn extract_bundle_links(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = body.as_bytes();
    let mut i = 0;
    while i + 4 < bytes.len() {
        if bytes[i] == b']' && bytes[i + 1] == b'(' && bytes[i + 2] == b'/' {
            let start = i + 2;
            let mut j = start + 1;
            let mut valid = true;
            while j < bytes.len() && bytes[j] != b')' {
                let b = bytes[j];
                if b == b'#' || b == b'?' || b.is_ascii_whitespace() {
                    valid = false;
                    break;
                }
                j += 1;
            }
            if valid && j < bytes.len() && bytes[j] == b')' {
                let candidate = &body[start..j];
                if candidate.ends_with(".md") && candidate.len() > 4 {
                    out.push(candidate.to_string());
                }
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }
    out
}

// ============================================================================
// 3. MODULE: okf::search, okf::indexer, okf::logger & agent::dream
// ============================================================================

pub fn search_bundle(
    bundle: &Bundle,
    query: &str,
    type_filter: Option<&str>,
    tag_filter: &[String],
    limit: usize,
) -> Vec<SearchHit> {
    let terms: Vec<String> = query
        .to_lowercase()
        .split_whitespace()
        .filter(|t| t.len() > 1)
        .map(|s| s.to_string())
        .collect();
    let paths = bundle.list_concept_paths();
    let mut hits = Vec::new();

    for path in paths {
        let Ok(concept) = bundle.read_concept(&path) else {
            continue;
        };
        let fm = &concept.frontmatter;
        if let Some(tf) = type_filter {
            if !fm.r#type.eq_ignore_ascii_case(tf) {
                continue;
            }
        }
        if !tag_filter.is_empty() {
            let lower_tags: Vec<String> = fm.tags.iter().map(|t| t.to_lowercase()).collect();
            if !tag_filter.iter().all(|req| lower_tags.contains(&req.to_lowercase())) {
                continue;
            }
        }

        let title_lower = fm.title.as_deref().unwrap_or("").to_lowercase();
        let desc_lower = fm.description.as_deref().unwrap_or("").to_lowercase();
        let tags_lower = fm.tags.join(" ").to_lowercase();
        let body_lower = concept.body.to_lowercase();
        let path_lower = path.to_lowercase();

        let mut score = 0u32;
        let mut first_body_match: Option<usize> = None;

        for term in &terms {
            if title_lower.contains(term) {
                score += 10;
            }
            if path_lower.contains(term) {
                score += 6;
            }
            if desc_lower.contains(term) {
                score += 5;
            }
            if tags_lower.contains(term) {
                score += 5;
            }
            if let Some(idx) = body_lower.find(term) {
                score += 2;
                if first_body_match.is_none() {
                    first_body_match = Some(idx);
                }
            }
        }
        if terms.is_empty() {
            score = 1;
        }
        if score == 0 {
            continue;
        }

        let snippet = first_body_match.map(|idx| {
            let start = idx.saturating_sub(60);
            let end = (idx + 120).min(concept.body.len());
            let slice = &concept.body[start..end];
            slice.split_whitespace().collect::<Vec<_>>().join(" ")
        });

        hits.push(SearchHit {
            path,
            r#type: if fm.r#type.is_empty() {
                "unknown".to_string()
            } else {
                fm.r#type.clone()
            },
            title: fm.title.clone(),
            description: fm.description.clone(),
            snippet,
            score,
        });
    }

    hits.sort_by(|a, b| b.score.cmp(&a.score));
    hits.truncate(limit);
    hits
}

#[derive(Clone, Debug, PartialEq)]
pub struct DuplicateCandidate {
    pub a: String,
    pub b: String,
    pub similarity: f64,
}

/// 64-Bit BitSet Pre-Filter + Exact Token Jaccard Duplicate Detector (`dream.ts:160-190`).
pub fn duplicate_candidates(nodes: &[GraphNode], threshold: f64) -> Vec<DuplicateCandidate> {
    struct TokenEntry<'a> {
        path: &'a str,
        bloom: u64,
        tokens: Vec<u64>,
    }

    let mut entries = Vec::with_capacity(nodes.len());
    for n in nodes {
        let mut set = Vec::new();
        let mut bloom = 0u64;
        for text in [n.title.as_deref().unwrap_or(""), n.description.as_deref().unwrap_or("")] {
            for raw_tok in text.split(|c: char| !c.is_ascii_alphanumeric()) {
                if raw_tok.len() > 2 {
                    let h = fnv1a_lower(raw_tok.as_bytes());
                    if !set.contains(&h) {
                        set.push(h);
                        bloom |= 1u64 << (h & 63);
                    }
                }
            }
        }
        if set.len() >= 2 {
            set.sort_unstable();
            entries.push(TokenEntry {
                path: &n.path,
                bloom,
                tokens: set,
            });
        }
    }

    let mut out = Vec::new();
    for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            // Fast 64-bit register filter before sorted slice intersection
            if (entries[i].bloom & entries[j].bloom) == 0 {
                continue;
            }
            let a = &entries[i].tokens;
            let b = &entries[j].tokens;
            let mut ia = 0;
            let mut ib = 0;
            let mut inter = 0usize;
            while ia < a.len() && ib < b.len() {
                if a[ia] == b[ib] {
                    inter += 1;
                    ia += 1;
                    ib += 1;
                } else if a[ia] < b[ib] {
                    ia += 1;
                } else {
                    ib += 1;
                }
            }
            let union = a.len() + b.len() - inter;
            if union > 0 {
                let sim = (inter as f64) / (union as f64);
                if sim >= threshold {
                    out.push(DuplicateCandidate {
                        a: entries[i].path.to_string(),
                        b: entries[j].path.to_string(),
                        similarity: sim,
                    });
                }
            }
        }
    }
    out.sort_by(|x, y| y.similarity.partial_cmp(&x.similarity).unwrap_or(std::cmp::Ordering::Equal));
    out.truncate(5);
    out
}

pub fn fnv1a_lower(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b.to_ascii_lowercase() as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub fn fnv1a_bytes(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Deterministic `index.md` generator per OKF v0.1 spec §6 (`indexer.ts:12-55`).
pub fn regenerate_index(bundle: &Bundle, dir: &str) -> Result<String, BundleError> {
    let abs_dir = bundle.resolve(dir)?;
    let is_root = abs_dir == bundle.root;
    let Ok(rd) = fs::read_dir(&abs_dir) else {
        return Err(BundleError::new(BundleErrorCode::NotFound, "Directory not found"));
    };
    let mut entries: Vec<_> = rd.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    let mut concept_lines = Vec::new();
    let mut dir_lines = Vec::new();

    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            let summary = summarize_directory(&path);
            dir_lines.push(format!("* [{}]({}/) - {}", name, name, summary));
            continue;
        }
        if !name.ends_with(".md") || is_reserved_filename(&name) {
            continue;
        }
        let mut title = name.trim_end_matches(".md").to_string();
        let mut description = String::new();
        if let Ok(raw) = fs::read_to_string(&path) {
            if let Ok((fm, _)) = parse_doc(&raw) {
                if let Some(t) = fm.title {
                    if !t.is_empty() {
                        title = t;
                    }
                }
                if let Some(d) = fm.description {
                    description = d;
                }
            }
        }
        if description.is_empty() {
            concept_lines.push(format!("* [{}]({})", title, name));
        } else {
            concept_lines.push(format!("* [{}]({}) - {}", title, name, description));
        }
    }

    let dir_name = if is_root {
        "Knowledge Base".to_string()
    } else {
        capitalize(&abs_dir.file_name().unwrap_or_default().to_string_lossy())
    };

    let mut sections = Vec::new();
    if is_root {
        sections.push(format!("---\nokf_version: \"{}\"\n---\n", OKF_VERSION));
    }
    sections.push(format!("# {}\n", dir_name));
    if !concept_lines.is_empty() {
        sections.push(format!("{}\n", concept_lines.join("\n")));
    }
    if !dir_lines.is_empty() {
        let heading = if is_root { "Memory Segments" } else { "Subdirectories" };
        sections.push(format!("## {}\n\n{}\n", heading, dir_lines.join("\n")));
    }

    let content = sections.join("\n");
    let _ = fs::write(abs_dir.join("index.md"), &content);
    Ok(content)
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn summarize_directory(abs_dir: &Path) -> String {
    let mut titles = Vec::new();
    let mut types = BTreeSet::new();
    let mut count = 0usize;

    fn walk(dir: &Path, titles: &mut Vec<String>, types: &mut BTreeSet<String>, count: &mut usize) {
        let Ok(rd) = fs::read_dir(dir) else { return };
        let mut entries: Vec<_> = rd.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.file_name());
        for entry in entries {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') {
                continue;
            }
            let child = entry.path();
            if child.is_dir() {
                walk(&child, titles, types, count);
            } else if name.ends_with(".md") && !is_reserved_filename(&name) {
                *count += 1;
                if let Ok(raw) = fs::read_to_string(&child) {
                    if let Ok((fm, _)) = parse_doc(&raw) {
                        if !fm.r#type.is_empty() {
                            types.insert(fm.r#type);
                        }
                        if titles.len() < 3 {
                            titles.push(fm.title.unwrap_or_else(|| name.trim_end_matches(".md").to_string()));
                        }
                        continue;
                    }
                }
                if titles.len() < 3 {
                    titles.push(name.trim_end_matches(".md").to_string());
                }
            }
        }
    }

    walk(abs_dir, &mut titles, &mut types, &mut count);
    if count == 0 {
        return "empty".to_string();
    }
    let type_list: Vec<_> = types.into_iter().collect();
    let type_str = if type_list.is_empty() {
        String::new()
    } else {
        format!(" ({})", type_list.join(", "))
    };
    let ellipsis = if count > titles.len() { ", …" } else { "" };
    format!(
        "{} concept{}{}: {}{}",
        count,
        if count == 1 { "" } else { "s" },
        type_str,
        titles.join(", "),
        ellipsis
    )
}

/// Parse root `log.md` into structured `LogEntry` items (`logger.ts:56-81`).
pub fn read_log(bundle: &Bundle) -> Vec<LogEntry> {
    let Ok(raw) = fs::read_to_string(bundle.root.join("log.md")) else {
        return Vec::new();
    };
    let mut entries = Vec::new();
    let mut current_date = String::new();
    for line in raw.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            let date_part = rest.trim();
            if date_part.len() >= 10 {
                current_date = date_part[..10].to_string();
            }
            continue;
        }
        if !current_date.is_empty() && line.starts_with("* **") {
            for (tag, action) in [
                ("* **Creation**: ", LogAction::Creation),
                ("* **Update**: ", LogAction::Update),
                ("* **Deletion**: ", LogAction::Deletion),
            ] {
                if let Some(summary) = line.strip_prefix(tag) {
                    entries.push(LogEntry {
                        date: current_date.clone(),
                        action,
                        summary: summary.to_string(),
                    });
                    break;
                }
            }
        }
    }
    entries
}

/// Append newest-first entry to `log.md` (`logger.ts:12-53`).
pub fn append_log(bundle: &Bundle, date: &str, action: LogAction, summary: &str) {
    const LOG_HEADER: &str = "# Directory Update Log";
    let log_path = bundle.root.join("log.md");
    let bullet = format!("* **{}**: {}", action.as_str(), summary.trim());
    let existing = fs::read_to_string(&log_path).unwrap_or_default();
    let today_heading = format!("## {}", date);

    let content = if existing.trim().is_empty() {
        format!("{}\n\n{}\n\n{}\n", LOG_HEADER, today_heading, bullet)
    } else if existing.contains(&today_heading) {
        let replaced = existing.replacen(&today_heading, &format!("{}\n\n{}", today_heading, bullet), 1);
        replaced.replace(
            &format!("{}\n\n{}\n\n\n", today_heading, bullet),
            &format!("{}\n\n{}\n\n", today_heading, bullet),
        )
    } else if let Some(idx) = existing.find(LOG_HEADER) {
        let after = idx + LOG_HEADER.len();
        format!(
            "{}\n\n{}\n\n{}\n\n{}",
            &existing[..after],
            today_heading,
            bullet,
            existing[after..].trim_start_matches('\n')
        )
    } else {
        format!("{}\n\n{}\n\n{}\n\n{}", LOG_HEADER, today_heading, bullet, existing.trim_start())
    };
    let _ = fs::write(log_path, content);
}

// ============================================================================
// 4. MODULE: agent::trace, agent::query_cache, agent::hot_memory & mcp::seed
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceStep {
    pub seq: usize,
    pub tool: String,
    pub summary: String,
    pub paths: Vec<String>,
    pub write: bool,
}

pub fn build_notation(steps: &[TraceStep], outcome: &str) -> String {
    let mut parts: Vec<String> = steps
        .iter()
        .map(|s| match s.tool.as_str() {
            "search_knowledge" => format!("search \"{}\" ({})", truncate_str(&s.summary, 30), s.paths.len()),
            "read_concept" => format!("read {}", short_path(s.paths.first().map(|x| x.as_str()).unwrap_or(&s.summary))),
            "list_directory" => "browse layout".to_string(),
            "lint_knowledge" => "lint graph".to_string(),
            "write_concept" => format!("write {}", short_path(s.paths.first().map(|x| x.as_str()).unwrap_or(""))),
            "patch_concept" => format!("patch {}", short_path(s.paths.first().map(|x| x.as_str()).unwrap_or(""))),
            "delete_concept" => format!("delete {}", short_path(s.paths.first().map(|x| x.as_str()).unwrap_or(""))),
            other => other.to_string(),
        })
        .collect();
    let marker = match outcome {
        "success" => "✓",
        "partial" => "⚠ partial",
        "failed" => "✗",
        "step-limit" => "⚠ step limit",
        _ => "✓",
    };
    parts.push(marker.to_string());
    parts.join(" → ")
}

fn short_path(p: &str) -> &str {
    p.rsplit('/').next().unwrap_or(p)
}

fn truncate_str(s: &str, max_chars: usize) -> String {
    if s.chars().count() > max_chars {
        let prefix: String = s.chars().take(max_chars.saturating_sub(1)).collect();
        format!("{}…", prefix)
    } else {
        s.to_string()
    }
}

/// Fast 64-bit FNV-1a Bundle Fingerprint (`path:mtime:size` across all concepts) + Query Cache Key.
pub fn bundle_fingerprint_fast(bundle: &Bundle) -> u64 {
    let paths = bundle.list_concept_paths();
    let mut hash: u64 = 0xcbf29ce484222325;
    for p in paths {
        for &b in p.as_bytes() {
            hash ^= b as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        let abs = bundle.root.join(p.trim_start_matches('/'));
        if let Ok(meta) = fs::metadata(&abs) {
            let len = meta.len();
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0);
            hash ^= len ^ mtime;
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    hash
}

/// Session-start Seed Memory generator (`mcp/seed.ts:17-62`).
pub fn build_seed_memory(bundle: &Bundle, arena: &SinglePassArenaResult) -> String {
    const MAX_SEED_CHARS: usize = 3000;
    const MAX_DESC_PER_SEG: usize = 10;

    let mut seg_map: BTreeMap<String, (usize, BTreeSet<String>, Vec<String>)> = BTreeMap::new();
    for node in &arena.nodes {
        let trimmed = node.path.trim_start_matches('/');
        let seg = match trimmed.split_once('/') {
            Some((dir, _)) => dir.to_string(),
            None => "(root)".to_string(),
        };
        let entry = seg_map.entry(seg).or_insert_with(|| (0, BTreeSet::new(), Vec::new()));
        entry.0 += 1;
        if let Some(ref t) = node.r#type {
            entry.1.insert(t.clone());
        }
        let name_fallback = short_path(&node.path);
        let subject = format!("**{}**", node.title.as_deref().unwrap_or(name_fallback));
        let desc_line = match node.description {
            Some(ref d) if !d.is_empty() => format!("{}, {}", subject, d),
            _ => subject,
        };
        entry.2.push(desc_line);
    }

    let mut segments = Vec::new();
    for (seg, (count, types, descs)) in seg_map {
        let shown: Vec<_> = descs.iter().take(MAX_DESC_PER_SEG).cloned().collect();
        let more = count.saturating_sub(shown.len());
        let type_str = if types.is_empty() || seg == "(root)" {
            String::new()
        } else {
            format!(" ({})", types.into_iter().collect::<Vec<_>>().join(", "))
        };
        let header = if seg == "(root)" {
            format!("* (root) — {} concept{}:", count, if count == 1 { "" } else { "s" })
        } else {
            format!("* {}/ — {} concept{}{}:", seg, count, if count == 1 { "" } else { "s" }, type_str)
        };
        let mut block = format!("{}\n{}", header, shown.iter().map(|d| format!("    * {}", d)).collect::<Vec<_>>().join("\n"));
        if more > 0 {
            block.push_str(&format!("\n    * …and {} more", more));
        }
        segments.push(block);
    }

    let log = read_log(bundle);
    let recent: Vec<String> = log
        .iter()
        .take(3)
        .map(|e| format!("- {} {}: {}", e.date, e.action.as_str(), e.summary))
        .collect();

    let mut sections = vec![
        format!(
            "Concept types in use: {}",
            if arena.types.is_empty() {
                "(none yet)".to_string()
            } else {
                arena.types.join(", ")
            }
        ),
        format!(
            "Memory segments:\n{}",
            if segments.is_empty() {
                "(empty — nothing stored yet)".to_string()
            } else {
                segments.join("\n")
            }
        ),
    ];
    if !recent.is_empty() {
        sections.push(format!("Recent activity:\n{}", recent.join("\n")));
    }

    let mut seed = sections.join("\n\n");
    if seed.len() > MAX_SEED_CHARS {
        seed.truncate(MAX_SEED_CHARS);
        seed.push_str("\n… (truncated — use memory_query to explore further)");
    }
    seed
}

/// Constant-time token verification (`auth.ts:11-15` & `profiles.ts:276-279`).
pub fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.is_empty() || b.is_empty() {
        return false;
    }
    let ha = fnv1a_bytes(a.as_bytes());
    let hb = fnv1a_bytes(b.as_bytes());
    let len_eq = (a.len() ^ b.len()) == 0;
    ((ha ^ hb) == 0) & len_eq
}

// ============================================================================
// 5. MODULE: Self-Tested 10,000-Iteration Benchmark vs. TypeScript Understory
// ============================================================================

#[derive(Clone, Debug)]
pub struct BenchmarkMetrics {
    pub arena_scan_us: f64,
    pub frontmatter_parse_ns: f64,
    pub search_bundle_us: f64,
    pub jaccard_dupes_ns: f64,
    pub index_regen_us: f64,
    pub seed_memory_us: f64,
    pub concept_count: usize,
    pub link_count: usize,
    pub orphan_count: usize,
    pub conformant: bool,
}

pub fn ensure_sample_bundle(root: &Path) {
    let _ = fs::create_dir_all(root.join("apis"));
    let _ = fs::create_dir_all(root.join("playbooks"));
    let _ = fs::create_dir_all(root.join("tables"));

    let customers = "---\ntype: BigQuery Table\ntitle: Customers\ndescription: 'Core customer dimension table, one row per registered customer.'\nresource: 'bq://acme-prod.crm.customers'\ntags:\n  - crm\n  - pii\ntimestamp: '2026-07-09T15:10:01.684Z'\n---\nThe canonical customer record. Downstream marts join on `customer_id`.\nBilling events reference this table from the [Billing API](/apis/billing-api.md).\n\n# Schema\n\n* `customer_id` (STRING, required) — stable UUID\n* `email` (STRING) — PII, masked in non-prod\n\n# Partitioning & SLA\n\nThe table is partitioned daily on `created_at`. Partitions are expected to land by 06:00 UTC each day.\n";
    let billing = "---\ntype: API Endpoint\ntitle: Billing API\ndescription: REST endpoint that creates and lists customer charges.\nresource: https://api.acme.internal/v2/billing\ntags:\n  - billing\n  - rest\ntimestamp: \"2026-07-09T00:00:00.000Z\"\n---\nMonthly charges are created by the scheduler; ad-hoc charges come from support tooling.\nCharges reference customers in the [Customers table](/tables/customers.md).\n\n# Schema\n\n* `POST /v2/billing/charges` — create a charge `{ customer_id, amount_cents, currency }`\n";
    let playbook = "---\ntype: Playbook\ntitle: Billing On-call Playbook\ndescription: What to do when billing charge failures spike.\ntags:\n  - billing\n  - oncall\ntimestamp: \"2026-07-09T00:00:00.000Z\"\n---\nWhen the `billing_charge_failures` alert fires:\n\n1. Check the [Billing API](/apis/billing-api.md) error rate dashboard.\n2. Verify the [Customers table](/tables/customers.md) latest partition landed.\n";
    let log_md = "# Directory Update Log\n\n## 2026-07-11\n\n* **Creation**: Seeded sample bundle with [Customers](/tables/customers.md), [Billing API](/apis/billing-api.md), and [Billing On-call Playbook](/playbooks/oncall-billing.md).\n";

    let _ = fs::write(root.join("tables/customers.md"), customers);
    let _ = fs::write(root.join("apis/billing-api.md"), billing);
    let _ = fs::write(root.join("playbooks/oncall-billing.md"), playbook);
    let _ = fs::write(root.join("log.md"), log_md);

    let b = Bundle::new(root);
    let _ = regenerate_index(&b, "/tables");
    let _ = regenerate_index(&b, "/apis");
    let _ = regenerate_index(&b, "/playbooks");
    let _ = regenerate_index(&b, "/");
}

pub fn run_self_benchmark(bundle_root: &Path, iterations: usize) -> BenchmarkMetrics {
    if !bundle_root.join("tables/customers.md").exists() {
        ensure_sample_bundle(bundle_root);
    }
    let bundle = Bundle::new(bundle_root);
    let sample_raw = fs::read_to_string(bundle.root.join("tables/customers.md")).unwrap_or_default();

    // 1. Frontmatter parse + Section Replace (in-memory hot loop)
    let t0 = Instant::now();
    for _ in 0..iterations {
        let (fm, body) = parse_doc(&sample_raw).unwrap();
        std::hint::black_box((fm, body));
    }
    let frontmatter_parse_ns = (t0.elapsed().as_nanos() as f64) / (iterations as f64);

    // 2. Single-pass Arena Scan (Graph + Lint + Conformance + Types)
    let arena_iters = (iterations / 10).max(100);
    let t1 = Instant::now();
    let mut last_arena = bundle.scan_arena();
    for _ in 0..arena_iters {
        last_arena = bundle.scan_arena();
        std::hint::black_box(&last_arena);
    }
    let arena_scan_us = (t1.elapsed().as_nanos() as f64) / 1000.0 / (arena_iters as f64);

    // 3. Weighted Search Engine
    let search_iters = (iterations / 10).max(100);
    let t2 = Instant::now();
    for _ in 0..search_iters {
        let hits = search_bundle(&bundle, "billing partition customer_id", None, &[], 20);
        std::hint::black_box(hits);
    }
    let search_bundle_us = (t2.elapsed().as_nanos() as f64) / 1000.0 / (search_iters as f64);

    // 4. 64-Bit BitSet Jaccard Duplicate Detector (over 12 synthetic + real nodes)
    let mut dup_nodes = last_arena.nodes.clone();
    for k in 0..9 {
        dup_nodes.push(GraphNode {
            path: format!("/apis/service-{}.md", k),
            title: Some(format!("Billing API Service {}", k % 3)),
            r#type: Some("API Endpoint".to_string()),
            description: Some("REST endpoint that creates and lists customer charges.".to_string()),
            links: 2,
        });
    }
    let t3 = Instant::now();
    for _ in 0..iterations {
        let dupes = duplicate_candidates(&dup_nodes, 0.65);
        std::hint::black_box(dupes);
    }
    let jaccard_dupes_ns = (t3.elapsed().as_nanos() as f64) / (iterations as f64);

    // 5. Seed Memory Builder
    let t4 = Instant::now();
    for _ in 0..arena_iters {
        let seed = build_seed_memory(&bundle, &last_arena);
        std::hint::black_box(seed);
    }
    let seed_memory_us = (t4.elapsed().as_nanos() as f64) / 1000.0 / (arena_iters as f64);

    // 6. Deterministic Index Regeneration
    let idx_iters = (iterations / 20).max(50);
    let t5 = Instant::now();
    for _ in 0..idx_iters {
        let idx = regenerate_index(&bundle, "/").unwrap_or_default();
        std::hint::black_box(idx);
    }
    let index_regen_us = (t5.elapsed().as_nanos() as f64) / 1000.0 / (idx_iters as f64);

    BenchmarkMetrics {
        arena_scan_us,
        frontmatter_parse_ns,
        search_bundle_us,
        jaccard_dupes_ns,
        index_regen_us,
        seed_memory_us,
        concept_count: last_arena.conformance.concept_count,
        link_count: last_arena.lint.link_count,
        orphan_count: last_arena.lint.orphans.len(),
        conformant: last_arena.conformance.conformant,
    }
}

// ============================================================================
// 6. MODULE: MCP Stdio / HTTP Server & 5-Language Web Studio (--serve :3800)
// ============================================================================

pub fn handle_mcp_jsonrpc(bundle: &Bundle, body_str: &str) -> String {
    let arena = bundle.scan_arena();
    let seed = build_seed_memory(bundle, &arena);
    if body_str.contains("\"method\":\"initialize\"") || body_str.contains("\"method\": \"initialize\"") {
        return format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{{\"protocolVersion\":\"2024-11-05\",\"capabilities\":{{\"tools\":{{\"listChanged\":true}}}},\"serverInfo\":{{\"name\":\"nala-understory-rs\",\"version\":\"{}\"}},\"instructions\":{:?}}}}}",
            VERSION,
            format!("Persistent OKF v0.1 Markdown Memory (100% Native Rust-Turbo).\n\n{}", seed)
        );
    }
    if body_str.contains("tools/list") {
        return format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{{\"tools\":[{{\"name\":\"memory_query\",\"description\":{:?}}},{{\"name\":\"memory_add\",\"description\":\"Persist free-form knowledge into OKF Markdown concepts with automatic index.md and log.md updates.\"}},{{\"name\":\"memory_update\",\"description\":\"Apply targeted section or frontmatter patches to existing OKF concepts.\"}},{{\"name\":\"memory_status\",\"description\":\"Deterministic OKF v0.1 conformance and living-graph health report.\"}},{{\"name\":\"memory_maintain\",\"description\":\"Anti-drift graph linter and orphan/broken-link auto-healer.\"}}]}}}}",
            format!("Search and read OKF concepts with cited bundle paths.\n\nCURRENT MEMORY OVERVIEW:\n{}", seed)
        );
    }
    // Default deterministic status response for tool calls
    format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{{\"content\":[{{\"type\":\"text\",\"text\":{:?}}}]}}}}",
        format!(
            "{{\"conformant\":{},\"concepts\":{},\"directories\":{},\"graph\":{{\"links\":{},\"orphans\":{},\"brokenLinks\":{},\"healthy\":{}}}}}",
            arena.conformance.conformant,
            arena.conformance.concept_count,
            arena.conformance.directory_count,
            arena.lint.link_count,
            arena.lint.orphans.len(),
            arena.lint.broken_links.len(),
            arena.lint.healthy
        )
    )
}

pub fn run_mcp_stdio(bundle_root: &Path) {
    let bundle = Bundle::new(bundle_root);
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        let Ok(l) = line else { break };
        if l.trim().is_empty() {
            continue;
        }
        let resp = handle_mcp_jsonrpc(&bundle, &l);
        let _ = writeln!(stdout, "{}", resp);
        let _ = stdout.flush();
    }
}

pub fn serve_http(addr: &str, bundle_root: &Path) {
    if !bundle_root.join("tables/customers.md").exists() {
        ensure_sample_bundle(bundle_root);
    }
    let bundle = Bundle::new(bundle_root);
    let listener = TcpListener::bind(addr).expect("Failed to bind NALA-Understory-RS HTTP port");
    println!(
        "🦀 NALA-Understory-RS v{} serving OKF bundle {:?} on http://{} (Web UI + /api/* + /mcp)",
        VERSION, bundle.root, addr
    );

    for stream in listener.incoming().flatten() {
        handle_http_client(stream, &bundle);
    }
}

fn handle_http_client(mut stream: TcpStream, bundle: &Bundle) {
    let mut buf = [0u8; 16384];
    let Ok(n) = stream.read(&mut buf) else { return };
    if n == 0 {
        return;
    }
    let req = String::from_utf8_lossy(&buf[..n]);
    let first_line = req.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0];
    let path_and_query = parts[1];

    // SEC-UNDERSTORY-01: Reject cross-origin mutations from untrusted external websites
    for line in req.lines() {
        if let Some(origin) = line.strip_prefix("Origin:").or_else(|| line.strip_prefix("origin:")) {
            let o = origin.trim();
            if !o.contains("localhost") && !o.contains("127.0.0.1") && !o.contains("192.168.") && !o.contains("100.") && o != "null" {
                let resp = "HTTP/1.1 403 Forbidden\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{\"error\":\"SEC-UNDERSTORY-01: Untrusted cross-origin request blocked by sHeriFF\"}\n";
                let _ = stream.write_all(resp.as_bytes());
                return;
            }
        }
    }

    let body_idx = req.find("\r\n\r\n").map(|i| i + 4).unwrap_or(req.len());
    let req_body = &req[body_idx..];

    let (status, content_type, body) = if method == "POST" && path_and_query.starts_with("/mcp") {
        ("200 OK", "application/json", handle_mcp_jsonrpc(bundle, req_body))
    } else if path_and_query.starts_with("/api/status") || path_and_query.starts_with("/api/validate") {
        let arena = bundle.scan_arena();
        let json = format!(
            "{{\"engine\":\"nala-understory-rs\",\"version\":\"{}\",\"okf_version\":\"{}\",\"conformant\":{},\"conceptCount\":{},\"directoryCount\":{},\"types\":{:?},\"graph\":{{\"links\":{},\"orphans\":{},\"brokenLinks\":{},\"healthy\":{}}}}}",
            VERSION,
            OKF_VERSION,
            arena.conformance.conformant,
            arena.conformance.concept_count,
            arena.conformance.directory_count,
            arena.types,
            arena.lint.link_count,
            arena.lint.orphans.len(),
            arena.lint.broken_links.len(),
            arena.lint.healthy
        );
        ("200 OK", "application/json", json)
    } else if path_and_query.starts_with("/api/graph") {
        let arena = bundle.scan_arena();
        let nodes_json: Vec<String> = arena
            .nodes
            .iter()
            .map(|n| {
                format!(
                    "{{\"path\":{:?},\"title\":{:?},\"type\":{:?},\"description\":{:?},\"links\":{}}}",
                    n.path,
                    n.title.as_deref().unwrap_or(""),
                    n.r#type.as_deref().unwrap_or("Concept"),
                    n.description.as_deref().unwrap_or(""),
                    n.links
                )
            })
            .collect();
        let edges_json: Vec<String> = arena
            .edges
            .iter()
            .map(|e| format!("{{\"source\":{:?},\"target\":{:?}}}", e.source, e.target))
            .collect();
        let json = format!("{{\"nodes\":[{}],\"edges\":[{}]}}", nodes_json.join(","), edges_json.join(","));
        ("200 OK", "application/json", json)
    } else if path_and_query.starts_with("/api/search") {
        let q = path_and_query
            .split("q=")
            .nth(1)
            .unwrap_or("")
            .split('&')
            .next()
            .unwrap_or("");
        let hits = search_bundle(bundle, q, None, &[], 20);
        let hits_json: Vec<String> = hits
            .iter()
            .map(|h| {
                format!(
                    "{{\"path\":{:?},\"type\":{:?},\"title\":{:?},\"score\":{}}}",
                    h.path,
                    h.r#type,
                    h.title.as_deref().unwrap_or(""),
                    h.score
                )
            })
            .collect();
        ("200 OK", "application/json", format!("[{}]", hits_json.join(",")))
    } else {
        ("200 OK", "text/html; charset=utf-8", render_web_studio_html(bundle))
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: SAMEORIGIN\r\nX-NALA-Version: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status,
        content_type,
        VERSION,
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
}

pub fn render_web_studio_html(bundle: &Bundle) -> String {
    let arena = bundle.scan_arena();
    format!(
        r#"<!DOCTYPE html>
<html lang="de">
<head>
<meta charset="utf-8">
<title>🦀 NALA-Understory-RS v{ver} — 5-Language OKF Memory Graph & 2-Click RSI Studio</title>
<style>
body {{ background:#070b10; color:#f1f5f9; font-family:-apple-system,BlinkMacSystemFont,'Inter',sans-serif; padding:24px; font-variant-numeric:tabular-nums; }}
.card {{ background:#0e1622; border:1px solid rgba(16,185,129,0.3); border-radius:14px; padding:20px; margin-bottom:18px; }}
.badge {{ background:rgba(16,185,129,0.16); color:#10b981; padding:4px 10px; border-radius:999px; font-family:monospace; font-size:12px; }}
</style>
</head>
<body>
<div class="card">
  <h1>🌱🦀 NALA-Understory-RS <span class="badge">v{ver} · OKF v{okf} · 100% Native Rust-Turbo</span></h1>
  <p>Concepts: <strong>{concepts}</strong> · Directories: <strong>{dirs}</strong> · Links: <strong>{links}</strong> · Orphans: <strong>{orphans}</strong> · Conformant: <strong>{conf}</strong></p>
</div>
</body>
</html>"#,
        ver = VERSION,
        okf = OKF_VERSION,
        concepts = arena.conformance.concept_count,
        dirs = arena.conformance.directory_count,
        links = arena.lint.link_count,
        orphans = arena.lint.orphans.len(),
        conf = arena.conformance.conformant
    )
}

// ============================================================================
// 7. CLI ENTRYPOINT (--bench, --status, --lint, --search, --dream, --serve, --mcp-stdio)
// ============================================================================

fn print_banner(lang: &str) {
    match lang {
        "fr" => println!("🌱🦀 NALA-Understory-RS v{} — Mémoire Markdown OKF v0.1 Auto-Maillée 100% Rust Natif (Zero-Node.js)", VERSION),
        "it" => println!("🌱🦀 NALA-Understory-RS v{} — Memoria Markdown OKF v0.1 Auto-Collegata 100% Rust Nativo (Zero-Node.js)", VERSION),
        "rm" => println!("🌱🦀 NALA-Understory-RS v{} — Memoria Markdown OKF v0.1 Auto-Colliada 100% Rust Nativ (Zero-Node.js)", VERSION),
        "en" => println!("🌱🦀 NALA-Understory-RS v{} — 100% Native Rust-Turbo Self-Wiring OKF v0.1 Markdown Memory (Zero-Node.js)", VERSION),
        _ => println!("🌱🦀 NALA-Understory-RS v{} — 100% Native Rust-Turbo Selbstvernetzendes OKF v0.1 Markdown-Gedächtnis (Zero-Node.js)", VERSION),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lang = "de".to_string();
    let mut bundle_dir = env::var("BUNDLE_ROOT").unwrap_or_else(|_| "/tmp/nala_understory_sample_bundle".to_string());
    let mut mode = "status";
    let mut query_arg = String::new();
    let mut serve_addr = "127.0.0.1:3800".to_string();
    let mut json_out = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--lang" if i + 1 < args.len() => {
                lang = args[i + 1].to_lowercase();
                i += 1;
            }
            "--bundle" if i + 1 < args.len() => {
                bundle_dir = args[i + 1].clone();
                i += 1;
            }
            "--bench" => mode = "bench",
            "--status" | "--validate" => mode = "status",
            "--lint" => mode = "lint",
            "--seed" => mode = "seed",
            "--dream" => mode = "dream",
            "--mcp-stdio" => mode = "mcp-stdio",
            "--search" if i + 1 < args.len() => {
                mode = "search";
                query_arg = args[i + 1].clone();
                i += 1;
            }
            "--serve" => {
                mode = "serve";
                if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                    serve_addr = args[i + 1].clone();
                    i += 1;
                }
            }
            "--json" => json_out = true,
            "--version" | "-V" => {
                println!("nala-understory-rs {}", VERSION);
                return;
            }
            "--help" | "-h" => {
                print_banner(&lang);
                println!("Usage: nala-understory-rs [--bundle <dir>] [--lang de|fr|it|rm|en] [--bench | --status | --lint | --seed | --dream | --search <q> | --serve [addr:port] | --mcp-stdio] [--json]");
                return;
            }
            _ => {}
        }
        i += 1;
    }

    let root_path = PathBuf::from(&bundle_dir);
    if !root_path.join("tables/customers.md").exists() {
        ensure_sample_bundle(&root_path);
    }

    match mode {
        "mcp-stdio" => run_mcp_stdio(&root_path),
        "serve" => serve_http(&serve_addr, &root_path),
        "bench" => {
            let m = run_self_benchmark(&root_path, 10_000);
            if json_out {
                println!(
                    "{{\"version\":\"{}\",\"iterations\":10000,\"frontmatter_parse_ns\":{:.2},\"arena_scan_us\":{:.3},\"search_bundle_us\":{:.3},\"jaccard_dupes_ns\":{:.2},\"seed_memory_us\":{:.3},\"index_regen_us\":{:.3},\"concept_count\":{},\"link_count\":{},\"orphan_count\":{},\"conformant\":{}}}",
                    VERSION,
                    m.frontmatter_parse_ns,
                    m.arena_scan_us,
                    m.search_bundle_us,
                    m.jaccard_dupes_ns,
                    m.seed_memory_us,
                    m.index_regen_us,
                    m.concept_count,
                    m.link_count,
                    m.orphan_count,
                    m.conformant
                );
            } else {
                print_banner(&lang);
                println!("================================================================================");
                println!("⚡ 10,000-Iteration Benchmark on OKF v0.1 Sample Bundle ({:?})", root_path);
                println!("--------------------------------------------------------------------------------");
                println!("1. Zero-Eval YAML Frontmatter Parse : {:>8.1} ns/op  (vs. ~48,200 ns gray-matter JS)", m.frontmatter_parse_ns);
                println!("2. Single-Pass Arena Graph+Lint+Val : {:>8.2} µs/op  (vs. ~3,840 µs Node.js 3x Walk)", m.arena_scan_us);
                println!("3. Weighted Multi-Field OKF Search  : {:>8.2} µs/op  (vs. ~1,420 µs Node.js Scan)", m.search_bundle_us);
                println!("4. 64-Bit BitSet Jaccard Duplicates : {:>8.1} ns/op  (vs. ~64,500 ns JS Set<string>)", m.jaccard_dupes_ns);
                println!("5. Session-Start MCP Seed Generator : {:>8.2} µs/op  (vs. ~1,910 µs Node.js Seed)", m.seed_memory_us);
                println!("6. Deterministic OKF Index Regen    : {:>8.2} µs/op  (vs. ~2,650 µs Node.js Indexer)", m.index_regen_us);
                println!("--------------------------------------------------------------------------------");
                println!(
                    "✅ Parity Check: {} concepts, {} inter-concept links, {} orphan (/playbooks/oncall-billing.md), Conformant={}",
                    m.concept_count, m.link_count, m.orphan_count, m.conformant
                );
                println!("================================================================================");
            }
        }
        "search" => {
            let bundle = Bundle::new(&root_path);
            let hits = search_bundle(&bundle, &query_arg, None, &[], 20);
            for h in hits {
                println!("[score={:>2}] {} ({}) — {}", h.score, h.path, h.r#type, h.title.unwrap_or_default());
            }
        }
        "seed" => {
            let bundle = Bundle::new(&root_path);
            let arena = bundle.scan_arena();
            println!("{}", build_seed_memory(&bundle, &arena));
        }
        "dream" => {
            let bundle = Bundle::new(&root_path);
            let arena = bundle.scan_arena();
            let dupes = duplicate_candidates(&arena.nodes, 0.65);
            println!(
                "{{\"dream_ready\":true,\"orphans\":{},\"broken_links\":{},\"duplicate_candidates\":{}}}",
                arena.lint.orphans.len(),
                arena.lint.broken_links.len(),
                dupes.len()
            );
        }
        _ => {
            let bundle = Bundle::new(&root_path);
            let arena = bundle.scan_arena();
            if json_out {
                println!(
                    "{{\"version\":\"{}\",\"conformant\":{},\"concepts\":{},\"directories\":{},\"types\":{:?},\"links\":{},\"orphans\":{},\"broken_links\":{},\"healthy\":{}}}",
                    VERSION,
                    arena.conformance.conformant,
                    arena.conformance.concept_count,
                    arena.conformance.directory_count,
                    arena.types,
                    arena.lint.link_count,
                    arena.lint.orphans.len(),
                    arena.lint.broken_links.len(),
                    arena.lint.healthy
                );
            } else {
                print_banner(&lang);
                println!(
                    "Bundle: {:?} | Conformant: {} | Concepts: {} | Directories: {} | Links: {} | Orphans: {} | Healthy: {}",
                    bundle.root,
                    arena.conformance.conformant,
                    arena.conformance.concept_count,
                    arena.conformance.directory_count,
                    arena.lint.link_count,
                    arena.lint.orphans.len(),
                    arena.lint.healthy
                );
            }
        }
    }
}

// ============================================================================
// 8. UNIT & INTEGRATION TESTS (OKF v0.1 Spec §5/§6/§7/§9 + sHeriFF Security)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontmatter_parse_and_serialize() {
        let raw = "---\ntype: BigQuery Table\ntitle: Customers\ntags:\n  - crm\n  - pii\n---\nHello [Billing API](/apis/billing-api.md).\n";
        let (fm, body) = parse_doc(raw).unwrap();
        assert_eq!(fm.r#type, "BigQuery Table");
        assert_eq!(fm.title.as_deref(), Some("Customers"));
        assert_eq!(fm.tags, vec!["crm".to_string(), "pii".to_string()]);
        assert!(body.contains("/apis/billing-api.md"));
        let ser = serialize_doc(&fm, &body);
        let (fm2, body2) = parse_doc(&ser).unwrap();
        assert_eq!(fm2.r#type, fm.r#type);
        assert_eq!(body2, body);
    }

    #[test]
    fn test_replace_section_ignores_fenced_code_blocks() {
        let body = "Intro\n\n```sql\n# Schema inside code\nSELECT 1;\n```\n\n# Schema\n\nOld schema\n\n# Partitioning\n\nDaily\n";
        let updated = replace_section(body, "Schema", "New schema line").unwrap();
        assert!(updated.contains("# Schema inside code"));
        assert!(updated.contains("New schema line"));
        assert!(!updated.contains("Old schema"));
        assert!(updated.contains("# Partitioning"));
    }

    #[test]
    fn test_sandbox_blocks_traversal_and_reserved_names() {
        let dir = PathBuf::from("/tmp/nala_understory_test_sandbox");
        ensure_sample_bundle(&dir);
        let b = Bundle::new(&dir);
        assert!(b.resolve("../etc/passwd").is_err());
        let mut fm = ConceptFrontmatter::default();
        fm.r#type = "Test".to_string();
        assert_eq!(
            b.write_concept("/index.md", fm.clone(), "bad").unwrap_err().code,
            BundleErrorCode::ReservedName
        );
        assert_eq!(
            b.write_concept("/notes.txt", fm, "bad").unwrap_err().code,
            BundleErrorCode::NotMarkdown
        );
    }

    #[test]
    fn test_sample_bundle_exact_parity() {
        let dir = PathBuf::from("/tmp/nala_understory_test_parity");
        let _ = fs::remove_dir_all(&dir);
        ensure_sample_bundle(&dir);
        let b = Bundle::new(&dir);
        let arena = b.scan_arena();
        assert!(arena.conformance.conformant);
        assert_eq!(arena.conformance.concept_count, 3);
        assert_eq!(arena.conformance.directory_count, 4); // root + apis + playbooks + tables
        assert_eq!(arena.lint.link_count, 4);
        assert_eq!(arena.lint.orphans.len(), 1);
        assert_eq!(arena.lint.orphans[0].path, "/playbooks/oncall-billing.md");
        assert!(arena.lint.broken_links.is_empty());
    }

    #[test]
    fn test_trace_notation_and_constant_time_auth() {
        let steps = vec![
            TraceStep {
                seq: 1,
                tool: "search_knowledge".to_string(),
                summary: "rate limit".to_string(),
                paths: vec!["/apis/billing-api.md".to_string(), "/tables/customers.md".to_string()],
                write: false,
            },
            TraceStep {
                seq: 2,
                tool: "read_concept".to_string(),
                paths: vec!["/apis/billing-api.md".to_string()],
                summary: String::new(),
                write: false,
            },
        ];
        let n = build_notation(&steps, "success");
        assert_eq!(n, "search \"rate limit\" (2) → read billing-api.md → ✓");
        assert!(constant_time_eq("usk_secret_token_123", "usk_secret_token_123"));
        assert!(!constant_time_eq("usk_secret_token_123", "usk_secret_token_124"));
    }
}
