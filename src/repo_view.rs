// Read-only data layer for the TUI navigator (leaf
// `020-design-seed-convention/090-tui-server/010-data-layer.md`). A
// `RepoView::scan` walks a single repo and produces every fact the TUI's
// two screens need: per-grove summaries (name, lifecycle, leaf counts,
// inbox-pending count) plus per-grove detail (the task-tree shape and the
// pending inbox observations). Leaf and observation *bodies* are read on
// demand via the free `read_path` helper.
//
// Single-repo today; the deferred multi-repo evolution wraps this type
// rather than rewriting it (see the
// `tui-multi-repo-and-multiplexer` seed and the parent BRIEF's
// "Factoring for future" decision).

use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::harness::HARNESSES;
use crate::status;

/// What the TUI sees for one repo. Construct with `scan`; re-construct on
/// every filesystem-watch refresh (no memoisation — see the leaf's
/// "No memoisation in v1" note).
#[derive(Debug, Clone)]
pub struct RepoView {
    pub repo_root: PathBuf,
    groves: Vec<GroveSummary>,
    details: BTreeMap<String, GroveDetail>,
    /// The `cli` layer: the running `grove` binary's version. Snapshotted at
    /// scan time so rendering stays a pure function of the snapshot.
    cli_version: String,
    /// The `repo` layer: each installed harness's `VERSION.md`, keyed by
    /// harness name. Read once per scan via [`status::repo_versions`]; absent
    /// harnesses omitted, unreadable `VERSION.md` mapped to `None`. The header
    /// renders this; grove rows compare their worktree stamp against it.
    repo_versions: BTreeMap<&'static str, Option<String>>,
}

#[derive(Debug, Clone)]
pub struct GroveSummary {
    pub name: String,
    pub lifecycle: Lifecycle,
    /// Live `.md` leaves under `.grove/`, excluding `BRIEF.md` and any file
    /// inside a `done/` subtree. Always 0 for `Seed` (no worktree).
    pub live_leaves: usize,
    /// `.md` leaves under any `done/` subtree, excluding `BRIEF.md`. Always
    /// 0 for `Seed`.
    pub retired_leaves: usize,
    /// Pending observation files at `.grove-meta/inboxes/<name>/` (the
    /// `.gitkeep` placeholder is excluded; only `.md` files are counted).
    pub inbox_pending: usize,
    /// The `worktree` layer: this grove worktree's materialised `VERSION.md`
    /// per relevant harness (the union of repo-installed harnesses and any
    /// harness materialised in the worktree, so legacy and orphan worktrees
    /// stay visible). `None` = missing/unreadable, rendered `(unknown)` and
    /// never drift. Always empty for a `Seed` (no worktree). Read via the
    /// shared [`status::worktree_version`] reader.
    pub worktree_versions: BTreeMap<&'static str, Option<String>>,
}

/// Per-grove lifecycle state distinguishable from `scan`-time evidence.
/// `Finished` is intentionally absent — detecting it requires walking
/// `grove-meta` history, which the leaf's "out of v1 unless cheap" note
/// defers to the future grove.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lifecycle {
    /// `.grove-worktrees/<name>/` exists.
    Live,
    /// `.grove-meta/inboxes/<name>/` exists but no worktree does.
    Seed,
}

#[derive(Debug, Clone)]
pub struct GroveDetail {
    pub name: String,
    /// `None` when the grove has no materialised `.grove/` (Seed, or a
    /// freshly-created worktree before its first bootstrap).
    pub task_tree: Option<TaskTree>,
    /// Absolute paths of pending observation files, lexicographically
    /// sorted (chronological by filename per ADR-0004). Empty when the
    /// inbox directory does not exist or holds only the `.gitkeep`.
    pub inbox: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct TaskTree {
    /// Absolute path to `<worktree>/.grove/`.
    pub root: PathBuf,
    /// `<worktree>/.grove/BRIEF.md` if present.
    pub root_brief: Option<PathBuf>,
    /// Top-level entries under `.grove/`, sorted by the same key
    /// `pick` uses (numeric-prefix entries first in lex order, then the
    /// rest — `done/` rides along under that ordering, distinguishable by
    /// `is_retired`).
    pub entries: Vec<TaskEntry>,
}

#[derive(Debug, Clone)]
pub struct TaskEntry {
    /// File or directory name (e.g. `010-data-layer.md`, `090-tui-server`,
    /// `done`).
    pub name: String,
    pub path: PathBuf,
    pub kind: TaskKind,
    /// True if this entry is itself the `done/` archive or sits anywhere
    /// inside one. Set on the directory itself *and* propagated to every
    /// descendant so the TUI can render retired material distinctly
    /// without re-walking ancestors.
    pub is_retired: bool,
}

#[derive(Debug, Clone)]
pub enum TaskKind {
    Leaf,
    Node {
        /// `<dir>/BRIEF.md` if present.
        brief: Option<PathBuf>,
        children: Vec<TaskEntry>,
    },
}

impl RepoView {
    /// Walk `<repo_root>/.grove-worktrees/` and `<repo_root>/.grove-meta/inboxes/`
    /// to produce a single snapshot. Reads metadata only; no leaf or
    /// observation bodies are loaded (the leaf's "scan is cheap enough
    /// to re-run on filesystem-watch" constraint).
    pub fn scan(repo_root: &Path) -> Result<Self> {
        let worktrees_dir = repo_root.join(".grove-worktrees");
        let inboxes_dir = repo_root.join(".grove-meta").join("inboxes");

        let live_names = list_subdirs(&worktrees_dir)?;
        let inbox_names = list_subdirs(&inboxes_dir)?;

        // Version layers, read once per scan (rendering reads only the
        // snapshot, never the filesystem — keeps the TUI snapshot tests honest).
        let repo_versions = status::repo_versions(repo_root);

        let mut all: BTreeSet<String> = BTreeSet::new();
        all.extend(live_names.iter().cloned());
        all.extend(inbox_names.iter().cloned());

        let mut groves = Vec::with_capacity(all.len());
        let mut details = BTreeMap::new();

        for name in all {
            let is_live = live_names.contains(&name);
            let lifecycle = if is_live { Lifecycle::Live } else { Lifecycle::Seed };

            let (task_tree, live_leaves, retired_leaves) = if is_live {
                let task_root = worktrees_dir.join(&name).join(".grove");
                if task_root.is_dir() {
                    let tree = scan_task_tree(&task_root)?;
                    let (l, d) = count_leaves(&tree);
                    (Some(tree), l, d)
                } else {
                    (None, 0, 0)
                }
            } else {
                (None, 0, 0)
            };

            let inbox = if inbox_names.contains(&name) {
                list_observations(&inboxes_dir.join(&name))?
            } else {
                Vec::new()
            };
            let inbox_pending = inbox.len();

            // The `worktree` layer for this grove. Relevant harnesses are the
            // union of those installed in the repo and those materialised in
            // the worktree — the same union `grove status` uses, so legacy
            // worktrees (no stamp) still show one `(unknown)` per repo harness
            // and orphan worktrees (harness gone from the repo) stay visible.
            let mut worktree_versions: BTreeMap<&'static str, Option<String>> = BTreeMap::new();
            if is_live {
                let worktree = worktrees_dir.join(&name);
                for h in HARNESSES {
                    let in_repo = repo_versions.contains_key(h.name);
                    let in_worktree = h.install_path(&worktree).exists();
                    if in_repo || in_worktree {
                        worktree_versions
                            .insert(h.name, status::worktree_version(repo_root, &name, h));
                    }
                }
            }

            groves.push(GroveSummary {
                name: name.clone(),
                lifecycle,
                live_leaves,
                retired_leaves,
                inbox_pending,
                worktree_versions,
            });
            details.insert(
                name.clone(),
                GroveDetail { name, task_tree, inbox },
            );
        }

        Ok(RepoView {
            repo_root: repo_root.to_path_buf(),
            groves,
            details,
            cli_version: status::CLI_VERSION.to_string(),
            repo_versions,
        })
    }

    /// The `cli` layer: the running binary's version, snapshotted at scan.
    pub fn cli_version(&self) -> &str {
        &self.cli_version
    }

    /// The `repo` layer: installed harness versions captured at scan time.
    pub fn repo_versions(&self) -> &BTreeMap<&'static str, Option<String>> {
        &self.repo_versions
    }

    /// All groves seen in this scan, sorted by name. The slice is the
    /// stable read-side surface; mutation goes through a fresh `scan`.
    pub fn groves(&self) -> &[GroveSummary] {
        &self.groves
    }

    /// Detail for `name` (`None` when no grove by that name exists in
    /// this snapshot).
    pub fn grove(&self, name: &str) -> Option<&GroveDetail> {
        self.details.get(name)
    }
}

/// Read a leaf body or observation body on demand. Distinct from
/// `RepoView::scan` because the scan deliberately does not load bodies.
/// `path` must already be an absolute path obtained from a `TaskEntry`
/// or `GroveDetail::inbox` — no path-traversal validation is performed,
/// since the data layer's caller (the TUI) is the only consumer and the
/// paths it holds came from the same `RepoView` scan.
pub fn read_path(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

// ---------------------------------------------------------------------------
// internals

fn list_subdirs(dir: &Path) -> Result<BTreeSet<String>> {
    let mut out = BTreeSet::new();
    if !dir.is_dir() {
        return Ok(out);
    }
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            out.insert(entry.file_name().to_string_lossy().into_owned());
        }
    }
    Ok(out)
}

fn scan_task_tree(root: &Path) -> Result<TaskTree> {
    let root_brief = brief_in(root);
    let entries = scan_dir(root, false)?;
    Ok(TaskTree {
        root: root.to_path_buf(),
        root_brief,
        entries,
    })
}

fn scan_dir(dir: &Path, inside_done: bool) -> Result<Vec<TaskEntry>> {
    let mut entries: Vec<fs::DirEntry> = fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(|e| e.ok())
        .collect();
    entries.sort_by(|a, b| {
        let na = a.file_name().to_string_lossy().into_owned();
        let nb = b.file_name().to_string_lossy().into_owned();
        sort_key(&na).cmp(&sort_key(&nb))
    });

    let mut out = Vec::with_capacity(entries.len());
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        let ft = match entry.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };
        if ft.is_dir() {
            let child_retired = inside_done || name == "done";
            let brief = brief_in(&path);
            let children = scan_dir(&path, child_retired)?;
            out.push(TaskEntry {
                name,
                path,
                kind: TaskKind::Node { brief, children },
                is_retired: child_retired,
            });
        } else if ft.is_file() {
            if name == "BRIEF.md" {
                continue;
            }
            if !name.ends_with(".md") {
                continue;
            }
            out.push(TaskEntry {
                name,
                path,
                kind: TaskKind::Leaf,
                is_retired: inside_done,
            });
        }
    }
    Ok(out)
}

fn brief_in(dir: &Path) -> Option<PathBuf> {
    let p = dir.join("BRIEF.md");
    if p.is_file() {
        Some(p)
    } else {
        None
    }
}

/// `(live, retired)` leaf counts over `tree`. A *leaf* is any `.md` file
/// other than `BRIEF.md`. This matches `pick`'s walk; `status.rs`
/// historically counted briefs too, which is a separate bug.
fn count_leaves(tree: &TaskTree) -> (usize, usize) {
    let mut live = 0;
    let mut done = 0;
    for entry in &tree.entries {
        count_entry(entry, &mut live, &mut done);
    }
    (live, done)
}

fn count_entry(entry: &TaskEntry, live: &mut usize, done: &mut usize) {
    match &entry.kind {
        TaskKind::Leaf => {
            if entry.is_retired {
                *done += 1;
            } else {
                *live += 1;
            }
        }
        TaskKind::Node { children, .. } => {
            for c in children {
                count_entry(c, live, done);
            }
        }
    }
}

/// Lexicographic-among-prefixed-first, then anything else lex. Same shape
/// as `pick::sort_key` — duplicated rather than reused because making
/// `pick`'s helper public would expose internals the verb's contract
/// does not promise.
fn sort_key(name: &str) -> (bool, &str) {
    let has_prefix = name.chars().next().map_or(false, |c| c.is_ascii_digit());
    (!has_prefix, name)
}

fn list_observations(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut v = Vec::new();
    if !dir.is_dir() {
        return Ok(v);
    }
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if name == ".gitkeep" || !name.ends_with(".md") {
            continue;
        }
        v.push(path);
    }
    v.sort();
    Ok(v)
}

#[cfg(test)]
mod inline_tests {
    use super::*;
    use crate::harness;
    use tempfile::TempDir;

    #[test]
    fn sort_key_prefixed_before_unprefixed() {
        let mut names = vec!["readme.md", "010-a.md", "020-b.md", "alpha"];
        names.sort_by(|a, b| sort_key(a).cmp(&sort_key(b)));
        assert_eq!(names, vec!["010-a.md", "020-b.md", "alpha", "readme.md"]);
    }

    fn write_version_md(dir: &Path, version: &str) {
        fs::create_dir_all(dir).unwrap();
        fs::write(dir.join("VERSION.md"), format!("| version | `{}` |\n", version)).unwrap();
    }

    #[test]
    fn scan_captures_cli_repo_and_worktree_versions() {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path();
        let claude = harness::by_name("claude").unwrap();
        // Repo install on 4.0.0, one live grove whose worktree is on 3.0.1.
        write_version_md(&claude.install_path(repo), "4.0.0");
        let worktree = repo.join(".grove-worktrees").join("alpha");
        write_version_md(&claude.install_path(&worktree), "3.0.1");
        fs::create_dir_all(worktree.join(".grove")).unwrap();
        fs::write(worktree.join(".grove/010-task.md"), "# 010-task\n").unwrap();

        let view = RepoView::scan(repo).unwrap();

        assert_eq!(view.cli_version(), crate::status::CLI_VERSION);
        assert_eq!(view.repo_versions().get("claude"), Some(&Some("4.0.0".to_string())));
        let alpha = view.groves().iter().find(|g| g.name == "alpha").unwrap();
        assert_eq!(
            alpha.worktree_versions.get("claude"),
            Some(&Some("3.0.1".to_string()))
        );
    }

    #[test]
    fn scan_marks_legacy_worktree_version_unknown() {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path();
        let claude = harness::by_name("claude").unwrap();
        write_version_md(&claude.install_path(repo), "4.0.0");
        // Worktree has a task tree but no materialised VERSION.md.
        let worktree = repo.join(".grove-worktrees").join("legacy");
        fs::create_dir_all(worktree.join(".grove")).unwrap();
        fs::write(worktree.join(".grove/010-task.md"), "# 010-task\n").unwrap();

        let view = RepoView::scan(repo).unwrap();
        let legacy = view.groves().iter().find(|g| g.name == "legacy").unwrap();
        // Repo-installed harness is relevant even with no worktree stamp,
        // and renders as unknown (None), never as drift.
        assert_eq!(legacy.worktree_versions.get("claude"), Some(&None));
    }
}
