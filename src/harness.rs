use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Whether a verb wants one harness or all detected ones.
///
/// - `Multi` operates on every detected harness (e.g. a repo with both
///   `.claude/` and `.codex/`). Retained for callers that fan out across all
///   harnesses; the live launch path uses `Single`.
/// - `Single` is for session-launching verbs (`start`, `continue`, ...): one
///   grove session runs in one harness, so an ambiguous repo without
///   `--harness` is an error rather than a silent dual-launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectMode {
    Multi,
    Single,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Harness {
    pub name: &'static str,
    pub project_dir: &'static str,
    /// Binary name to exec (resolved via PATH at runtime).
    pub exec_bin: &'static str,
    /// CLI arg used by the harness to set the session name.
    /// For claude: ["-n", "<name>"]. For codex: see codex docs; default
    /// matches claude's pattern until verified.
    pub name_args: &'static [&'static str],
}

pub const HARNESSES: &[Harness] = &[
    Harness {
        name: "claude",
        project_dir: ".claude",
        exec_bin: "claude",
        name_args: &["-n"],
    },
    Harness {
        name: "codex",
        project_dir: ".codex",
        exec_bin: "codex",
        // Verified during implementation against `codex --help`; if codex
        // doesn't support a session-name flag, leave empty and skip pre-naming.
        name_args: &["--name"],
    },
];

pub fn by_name(name: &str) -> Option<&'static Harness> {
    HARNESSES.iter().find(|h| h.name == name)
}

impl Harness {
    pub fn project_dir_path(&self, repo: &Path) -> PathBuf {
        repo.join(self.project_dir)
    }
}

/// Return the harnesses that have a project directory in `repo`.
pub fn detect_in_repo(repo: &Path) -> Vec<&'static Harness> {
    HARNESSES
        .iter()
        .filter(|h| h.project_dir_path(repo).is_dir())
        .collect()
}

/// Resolve the harnesses for a verb:
/// - If `explicit` is non-empty, look each up by name (deduplicating repeats).
/// - Else, fall back to `detect_in_repo`.
/// - If neither yields anything: error.
/// - If `mode` is `Single` and more than one harness is detected (with no
///   `explicit` override), error and ask the user to disambiguate.
pub fn select(repo: &Path, explicit: &[String], mode: SelectMode) -> Result<Vec<&'static Harness>> {
    if !explicit.is_empty() {
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        for name in explicit {
            if !seen.insert(name.as_str()) {
                continue;
            }
            let h = by_name(name)
                .ok_or_else(|| anyhow!("unknown harness: {name}. Known: claude, codex"))?;
            out.push(h);
        }
        return Ok(out);
    }

    let detected = detect_in_repo(repo);
    match (mode, detected.len()) {
        (_, 0) => anyhow::bail!(
            "no harness session detected in {}; run the harness at least once in this repo or pass --harness explicitly",
            repo.display()
        ),
        (SelectMode::Single, n) if n > 1 => anyhow::bail!(
            "multiple harnesses detected in {} — pass --harness explicitly",
            repo.display()
        ),
        _ => Ok(detected),
    }
}
