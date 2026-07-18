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
    /// Home-relative path of this harness's **global skills dir** — the
    /// provisioning target for the embedded methodology. A field (not derived
    /// from `project_dir`) because pi nests its skills under `agent/`.
    pub skills_dir: &'static str,
    /// Binary name to exec (resolved via PATH at runtime).
    pub exec_bin: &'static str,
    /// CLI flag template for pre-naming the session at launch. For claude:
    /// `["-n"]`. An **empty** template means the harness has no launch-time
    /// name flag (codex — it names sessions after start, via `/rename`), and the
    /// launch paths skip pre-naming rather than passing a flag it would reject.
    pub name_args: &'static [&'static str],
    /// CLI flag template for selecting the launch model (model-per-task-kind),
    /// parallel to `name_args`: *how* to pass a model is per-harness, while the
    /// *value* comes from the picked leaf's kind's env var (one of
    /// `GROVE_PLANNING_MODEL` / `GROVE_RESEARCH_MODEL` / `GROVE_PROTOTYPE_MODEL`
    /// / `GROVE_WORK_MODEL` / `GROVE_REVIEW_MODEL`). For claude: `["--model"]`.
    /// An **empty** template means "this harness opts out of model selection" —
    /// the loop driver skips passing any model flag for it.
    pub model_args: &'static [&'static str],
}

pub const HARNESSES: &[Harness] = &[
    Harness {
        name: "claude",
        project_dir: ".claude",
        skills_dir: ".claude/skills",
        exec_bin: "claude",
        name_args: &["-n"],
        model_args: &["--model"],
    },
    Harness {
        name: "codex",
        project_dir: ".codex",
        skills_dir: ".codex/skills",
        exec_bin: "codex",
        // codex has **no launch-time session-name flag** (checked against
        // codex-cli 0.144.1: `codex --help` has zero `--name` matches). Session
        // names do exist, but are assigned *after* start — `codex resume` takes a
        // "session id (UUID) or session name", and the name is set in-session via
        // `/rename`. Naming at launch is an open upstream request (openai/codex
        // #22526, #4163). Empty ⇒ the launch paths skip pre-naming.
        name_args: &[],
        // codex model-per-task-kind values name **profiles** (`--profile`), not
        // models: a profile binds model + reasoning effort together
        // (e.g. sol-xhigh / sol-high), which bare `-m` cannot express.
        model_args: &["--profile"],
    },
    Harness {
        name: "pi",
        // Opt-in detection marker; pi does not create repo-local `.pi/` dirs,
        // so explicit `--harness pi` + the stamp is the normal binding route.
        project_dir: ".pi",
        skills_dir: ".pi/agent/skills",
        exec_bin: "pi",
        // pi has no launch-time session-name flag (pi --help, checked
        // 2026-07-18); empty ⇒ the launch paths skip pre-naming.
        name_args: &[],
        // pi accepts `--model <pattern>` incl. "provider/id" ids, so it takes
        // part in model-per-task-kind on the same terms as claude.
        model_args: &["--model"],
    },
];

pub fn by_name(name: &str) -> Option<&'static Harness> {
    HARNESSES.iter().find(|h| h.name == name)
}

/// The registry's names, comma-joined — the single source for "known:" error
/// text, so adding a harness never leaves a stale hardcoded list behind.
pub fn known_names() -> String {
    HARNESSES
        .iter()
        .map(|h| h.name)
        .collect::<Vec<_>>()
        .join(", ")
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
                .ok_or_else(|| anyhow!("unknown harness: {name}. Known: {}", known_names()))?;
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
