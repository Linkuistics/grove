//! Fleet repo discovery (ADR-0025) — resolve the set of repo roots a fleet
//! `grove tui` process spans, from a manifest file + scan roots + `--repo`
//! flags + the current repo. Produces a plain, canonical, deduped
//! `Vec<repo root>` consumed by the `MultiRepoView` data layer (leaf 070/020).
//!
//! This module sits **below** the presentation boundary (ADR-0013): it has no
//! `ratatui` dependency. The resolution logic is a pure function over injected
//! inputs (`Discovery`); the side-effecting [`resolve`] wrapper builds those
//! inputs from the real environment (XDG manifest + cwd git root) and prints
//! breadcrumbs to stderr.

use serde::Deserialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Parsed fleet manifest (`fleet.toml`). Both keys default to empty so a
/// manifest may list only `repos`, only `scan_roots`, or neither.
#[derive(Debug, Default, Deserialize)]
pub struct FleetManifest {
    /// Explicit repo roots, always candidates (a missing one is dropped with a
    /// stderr breadcrumb — never blocks).
    #[serde(default)]
    pub repos: Vec<PathBuf>,
    /// Directories walked one level deep to discover repos whose immediate
    /// children contain a `.grove-worktrees/`.
    #[serde(default)]
    pub scan_roots: Vec<PathBuf>,
}

impl FleetManifest {
    /// Load and parse the manifest at `path`. `Ok(None)` when the file does not
    /// exist (the zero-config case); `Err` only on a read or parse failure.
    pub fn load(path: &Path) -> anyhow::Result<Option<FleetManifest>> {
        use anyhow::Context;
        if !path.exists() {
            return Ok(None);
        }
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading fleet manifest {}", path.display()))?;
        let manifest: FleetManifest = toml::from_str(&text)
            .with_context(|| format!("parsing fleet manifest {}", path.display()))?;
        Ok(Some(manifest))
    }
}

/// XDG manifest path: `$XDG_CONFIG_HOME/grove/fleet.toml`, falling back to
/// `~/.config/grove/fleet.toml`. `None` when neither `XDG_CONFIG_HOME` nor
/// `HOME` is set.
pub fn default_manifest_path() -> Option<PathBuf> {
    manifest_path_from(
        std::env::var("XDG_CONFIG_HOME").ok(),
        std::env::var("HOME").ok(),
    )
}

/// Pure XDG-path policy, factored out for testing: prefer `$XDG_CONFIG_HOME`,
/// fall back to `$HOME/.config`, `None` when neither is set.
fn manifest_path_from(xdg: Option<String>, home: Option<String>) -> Option<PathBuf> {
    if let Some(xdg) = xdg.filter(|s| !s.is_empty()) {
        return Some(PathBuf::from(xdg).join("grove").join("fleet.toml"));
    }
    home.filter(|s| !s.is_empty())
        .map(|home| PathBuf::from(home).join(".config").join("grove").join("fleet.toml"))
}

/// Injected inputs to the resolver, kept separate from the environment so the
/// resolution logic is pure and unit-testable without touching XDG or the
/// process cwd.
pub struct Discovery {
    /// The cwd's git root, if any — always included (ADR-0025 §3).
    pub current_repo: Option<PathBuf>,
    /// `--repo <path>` flags, layered additively (ADR-0025 §2).
    pub repo_flags: Vec<PathBuf>,
    /// The parsed manifest, if one was found.
    pub manifest: Option<FleetManifest>,
}

impl Discovery {
    /// Resolve the canonical, deduped repo list, returning any breadcrumb
    /// warnings (one per missing *explicit* repo) alongside it for the caller
    /// to surface. Never fails — unresolvable repos are dropped.
    ///
    /// Order: current repo first, then manifest `repos` (file order), then
    /// `--repo` flags (CLI order), then scan-discovered repos (alphabetical).
    /// Dedup keeps the first occurrence, so explicit routes win over scan
    /// (ADR-0025 §4; the final fleet sort is leaf 070/040's concern).
    pub fn resolve_with_warnings(&self) -> (Vec<PathBuf>, Vec<String>) {
        let mut out: Vec<PathBuf> = Vec::new();
        let mut seen: HashSet<PathBuf> = HashSet::new();
        let mut warnings: Vec<String> = Vec::new();

        let mut push = |path: &Path, out: &mut Vec<PathBuf>| {
            // Only existing paths reach here, so canonicalize cannot fail; guard
            // anyway and skip on the off chance of a race.
            if let Ok(canon) = std::fs::canonicalize(path) {
                if seen.insert(canon.clone()) {
                    out.push(canon);
                }
            }
        };

        // 1. Current repo — always included, no existence breadcrumb (it is the
        //    cwd's git root, which exists by construction).
        if let Some(cur) = &self.current_repo {
            if cur.is_dir() {
                push(cur, &mut out);
            }
        }

        // 2/3. Explicit repos (manifest `repos`, then `--repo` flags). A missing
        //      one is dropped with a single stderr breadcrumb — never blocks.
        let explicit = self
            .manifest
            .as_ref()
            .map(|m| m.repos.as_slice())
            .unwrap_or(&[])
            .iter()
            .chain(self.repo_flags.iter());
        for repo in explicit {
            if repo.is_dir() {
                push(repo, &mut out);
            } else {
                warnings.push(format!(
                    "grove fleet: configured repo not found, skipping: {}",
                    repo.display()
                ));
            }
        }

        // 4. Scan roots — each walked one level; an immediate child that contains
        //    a `.grove-worktrees/` is a discovered repo. Empty/missing scan
        //    roots are expected and stay silent. Discovered repos are sorted
        //    alphabetically by path for a deterministic order.
        let scan_roots = self
            .manifest
            .as_ref()
            .map(|m| m.scan_roots.as_slice())
            .unwrap_or(&[]);
        let mut discovered = scan_roots
            .iter()
            .flat_map(|root| scan_root(root))
            .collect::<Vec<_>>();
        discovered.sort();
        for repo in discovered {
            push(&repo, &mut out);
        }

        (out, warnings)
    }

    /// Resolve the canonical, deduped repo list, printing breadcrumb warnings
    /// to stderr.
    pub fn resolve(&self) -> Vec<PathBuf> {
        let (repos, warnings) = self.resolve_with_warnings();
        for w in warnings {
            eprintln!("{w}");
        }
        repos
    }
}

/// Top-level entry: build [`Discovery`] from the real environment (the XDG
/// manifest and the cwd git root) plus the given `--repo` flags, and resolve.
/// The canonical, deduped repo list is returned; breadcrumbs go to stderr.
pub fn resolve(repo_flags: &[PathBuf]) -> Vec<PathBuf> {
    let manifest = default_manifest_path()
        .and_then(|p| match FleetManifest::load(&p) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("grove fleet: ignoring unreadable manifest: {e:#}");
                None
            }
        });
    let current_repo = crate::repo::resolve(None).ok();
    Discovery {
        current_repo,
        repo_flags: repo_flags.to_vec(),
        manifest,
    }
    .resolve()
}

/// Discover repos under one scan root: each immediate subdirectory that
/// contains a `.grove-worktrees/`. A missing or unreadable root yields nothing
/// (scan-empty is an expected, silent outcome — ADR-0025 / 070 Q3).
fn scan_root(root: &Path) -> Vec<PathBuf> {
    let entries = match std::fs::read_dir(root) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir() && p.join(".grove-worktrees").is_dir())
        .collect()
}

// ---------------------------------------------------------------------------
// Fleet fs-watch helpers (leaf 070/030)
//
// One `notify` watcher spans the whole fleet (070 Q6). These pure helpers give
// the watch its three fleet-scale behaviours: which directories to register,
// which events to ignore (`.git/` churn), and which repo owns an event so a
// settle re-scans only that repo's `RepoView`. They live here (below the
// presentation boundary, ADR-0013) so both watch paths — the legacy `--local`
// `WatchSet` and the native `spawn_grove_watch` — share one implementation.

/// True when `path` lies inside (or *is*) a `.git/` directory — any path
/// component is exactly `.git`. The fleet fs-watch drops such events before they
/// mark the snapshot dirty: pack writes, ref updates, and index churn inside
/// each worktree's `.git/` are noise the 200ms debounce only *masked*, amplified
/// N-fold at fleet scale (070 Q6; root BRIEF "fs-watch .git/ noise"). Matching is
/// by whole path component, so a sibling like `digit/` is never mistaken for it.
pub fn path_is_git_internal(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == ".git")
}

/// The index into `repo_roots` of the repo that owns `event_path` — the root
/// that is an ancestor of (or equal to) the event path. Fleet repo roots do not
/// nest, so at most one matches; `None` when the path is under no known repo (a
/// stray event, or one from a since-removed repo). `Path::starts_with` matches
/// whole components, so `/work/alpha` never falsely owns `/work/alpha-two`.
/// The watch uses this to re-scan only the owning repo's `RepoView` (070 Q6).
pub fn owning_repo(event_path: &Path, repo_roots: &[PathBuf]) -> Option<usize> {
    repo_roots.iter().position(|root| event_path.starts_with(root))
}

/// The directories the fleet fs-watch registers: each repo's two grove-state
/// roots — `.grove-worktrees/` and `.grove-meta/inboxes/` — in repo order. One
/// `notify` watcher watches them all (070 Q6: one watcher, not N per-repo). Every
/// root is listed unconditionally; the watch constructor skips any that do not
/// yet exist on disk. At N=1 this is exactly today's single-repo watch set.
pub fn fleet_watch_dirs(repo_roots: &[PathBuf]) -> Vec<PathBuf> {
    repo_roots
        .iter()
        .flat_map(|r| {
            [
                r.join(".grove-worktrees"),
                r.join(".grove-meta").join("inboxes"),
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Create `dir/<name>/` and, unless `bare`, `dir/<name>/.grove-worktrees/`.
    /// Returns the canonicalized repo path (matching what the resolver emits).
    fn make_repo(dir: &Path, name: &str, bare: bool) -> PathBuf {
        let repo = dir.join(name);
        fs::create_dir_all(&repo).unwrap();
        if !bare {
            fs::create_dir_all(repo.join(".grove-worktrees")).unwrap();
        }
        fs::canonicalize(&repo).unwrap()
    }

    #[test]
    fn explicit_repos_only() {
        let tmp = TempDir::new().unwrap();
        let a = make_repo(tmp.path(), "a", false);
        let b = make_repo(tmp.path(), "b", false);
        let d = Discovery {
            current_repo: None,
            repo_flags: vec![],
            manifest: Some(FleetManifest {
                repos: vec![a.clone(), b.clone()],
                scan_roots: vec![],
            }),
        };
        let (repos, warnings) = d.resolve_with_warnings();
        assert_eq!(repos, vec![a, b]);
        assert!(warnings.is_empty());
    }

    #[test]
    fn scan_root_discovers_children_with_worktrees_only() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("work");
        fs::create_dir_all(&root).unwrap();
        let with = make_repo(&root, "has-groves", false);
        let _without = make_repo(&root, "no-groves", true); // bare: excluded
        let d = Discovery {
            current_repo: None,
            repo_flags: vec![],
            manifest: Some(FleetManifest {
                repos: vec![],
                scan_roots: vec![root.clone()],
            }),
        };
        let (repos, warnings) = d.resolve_with_warnings();
        assert_eq!(repos, vec![with]);
        assert!(warnings.is_empty());
    }

    #[test]
    fn repo_in_both_repos_and_scan_root_deduped() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("work");
        fs::create_dir_all(&root).unwrap();
        let a = make_repo(&root, "a", false);
        let d = Discovery {
            current_repo: None,
            repo_flags: vec![],
            manifest: Some(FleetManifest {
                repos: vec![a.clone()],
                scan_roots: vec![root.clone()],
            }),
        };
        let (repos, _) = d.resolve_with_warnings();
        assert_eq!(repos, vec![a]); // once, not twice
    }

    #[test]
    fn current_repo_always_included_even_without_worktrees() {
        let tmp = TempDir::new().unwrap();
        let cur = make_repo(tmp.path(), "current", true); // no .grove-worktrees/
        let d = Discovery {
            current_repo: Some(cur.clone()),
            repo_flags: vec![],
            manifest: None,
        };
        let (repos, warnings) = d.resolve_with_warnings();
        assert_eq!(repos, vec![cur]);
        assert!(warnings.is_empty());
    }

    #[test]
    fn missing_explicit_repo_dropped_with_breadcrumb() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("gone");
        let d = Discovery {
            current_repo: None,
            repo_flags: vec![],
            manifest: Some(FleetManifest {
                repos: vec![missing.clone()],
                scan_roots: vec![],
            }),
        };
        let (repos, warnings) = d.resolve_with_warnings();
        assert!(repos.is_empty());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains(&missing.display().to_string()));
    }

    #[test]
    fn empty_everything_yields_empty() {
        let d = Discovery {
            current_repo: None,
            repo_flags: vec![],
            manifest: None,
        };
        let (repos, warnings) = d.resolve_with_warnings();
        assert!(repos.is_empty());
        assert!(warnings.is_empty());
    }

    #[test]
    fn repo_flags_layer_additively() {
        let tmp = TempDir::new().unwrap();
        let a = make_repo(tmp.path(), "a", false);
        let d = Discovery {
            current_repo: None,
            repo_flags: vec![a.clone()],
            manifest: None,
        };
        let (repos, _) = d.resolve_with_warnings();
        assert_eq!(repos, vec![a]);
    }

    #[test]
    fn current_repo_first_then_manifest_repos() {
        let tmp = TempDir::new().unwrap();
        let cur = make_repo(tmp.path(), "current", false);
        let a = make_repo(tmp.path(), "a", false);
        let d = Discovery {
            current_repo: Some(cur.clone()),
            repo_flags: vec![],
            manifest: Some(FleetManifest {
                repos: vec![a.clone()],
                scan_roots: vec![],
            }),
        };
        let (repos, _) = d.resolve_with_warnings();
        assert_eq!(repos, vec![cur, a]);
    }

    #[test]
    fn current_repo_also_in_manifest_deduped_kept_first() {
        let tmp = TempDir::new().unwrap();
        let cur = make_repo(tmp.path(), "current", false);
        let a = make_repo(tmp.path(), "a", false);
        let d = Discovery {
            current_repo: Some(cur.clone()),
            repo_flags: vec![],
            manifest: Some(FleetManifest {
                repos: vec![a.clone(), cur.clone()],
                scan_roots: vec![],
            }),
        };
        let (repos, _) = d.resolve_with_warnings();
        assert_eq!(repos, vec![cur, a]);
    }

    #[test]
    fn scanned_repos_sorted_alphabetically() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("work");
        fs::create_dir_all(&root).unwrap();
        let z = make_repo(&root, "zeta", false);
        let a = make_repo(&root, "alpha", false);
        let d = Discovery {
            current_repo: None,
            repo_flags: vec![],
            manifest: Some(FleetManifest {
                repos: vec![],
                scan_roots: vec![root],
            }),
        };
        let (repos, _) = d.resolve_with_warnings();
        assert_eq!(repos, vec![a, z]);
    }

    #[test]
    fn missing_scan_root_silent() {
        let tmp = TempDir::new().unwrap();
        let d = Discovery {
            current_repo: None,
            repo_flags: vec![],
            manifest: Some(FleetManifest {
                repos: vec![],
                scan_roots: vec![tmp.path().join("nope")],
            }),
        };
        let (repos, warnings) = d.resolve_with_warnings();
        assert!(repos.is_empty());
        assert!(warnings.is_empty()); // scan-empty is expected, never a breadcrumb
    }

    #[test]
    fn manifest_load_absent_is_none() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("fleet.toml");
        assert!(FleetManifest::load(&path).unwrap().is_none());
    }

    #[test]
    fn manifest_load_parses_repos_and_scan_roots() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("fleet.toml");
        fs::write(
            &path,
            "repos = [\"/a\", \"/b\"]\nscan_roots = [\"/work\"]\n",
        )
        .unwrap();
        let m = FleetManifest::load(&path).unwrap().unwrap();
        assert_eq!(m.repos, vec![PathBuf::from("/a"), PathBuf::from("/b")]);
        assert_eq!(m.scan_roots, vec![PathBuf::from("/work")]);
    }

    #[test]
    fn manifest_load_empty_file_defaults_to_empty() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("fleet.toml");
        fs::write(&path, "").unwrap();
        let m = FleetManifest::load(&path).unwrap().unwrap();
        assert!(m.repos.is_empty());
        assert!(m.scan_roots.is_empty());
    }

    #[test]
    fn manifest_path_prefers_xdg_over_home() {
        let p = manifest_path_from(Some("/xdg".into()), Some("/home/u".into()));
        assert_eq!(p, Some(PathBuf::from("/xdg/grove/fleet.toml")));
    }

    #[test]
    fn manifest_path_falls_back_to_home_config() {
        let p = manifest_path_from(None, Some("/home/u".into()));
        assert_eq!(p, Some(PathBuf::from("/home/u/.config/grove/fleet.toml")));
    }

    #[test]
    fn manifest_path_none_without_env() {
        assert_eq!(manifest_path_from(None, None), None);
    }

    // ---- fleet fs-watch helpers (leaf 070/030) ----

    #[test]
    fn git_internal_paths_are_recognised() {
        // A path inside any `.git/` directory is git-internal noise.
        assert!(path_is_git_internal(Path::new(
            "/r/.grove-worktrees/feat/.git/refs/heads/main"
        )));
        // The `.git` directory itself counts too (an event on the dir node).
        assert!(path_is_git_internal(Path::new("/r/.grove-worktrees/feat/.git")));
        // The nested-worktree `.git` file/dir at a repo root.
        assert!(path_is_git_internal(Path::new("/r/.git/index")));
    }

    #[test]
    fn grove_state_paths_are_not_git_internal() {
        // A real grove-state write must pass the filter.
        assert!(!path_is_git_internal(Path::new(
            "/r/.grove-worktrees/feat/.grove/030-x.md"
        )));
        // An inbox observation write must pass the filter.
        assert!(!path_is_git_internal(Path::new(
            "/r/.grove-meta/inboxes/feat/obs.md"
        )));
        // A directory that merely *contains* the substring "git" is not `.git`.
        assert!(!path_is_git_internal(Path::new("/r/.grove-worktrees/digit/.grove/x.md")));
    }

    #[test]
    fn owning_repo_prefix_matches_the_right_repo() {
        let roots = vec![
            PathBuf::from("/work/alpha"),
            PathBuf::from("/work/beta"),
        ];
        // An event under beta resolves to index 1.
        assert_eq!(
            owning_repo(Path::new("/work/beta/.grove-worktrees/x/.grove/010.md"), &roots),
            Some(1),
        );
        // An event under alpha resolves to index 0.
        assert_eq!(
            owning_repo(Path::new("/work/alpha/.grove-meta/inboxes/y/o.md"), &roots),
            Some(0),
        );
        // A path under no known repo is unowned.
        assert_eq!(owning_repo(Path::new("/elsewhere/z"), &roots), None);
    }

    #[test]
    fn owning_repo_does_not_false_match_sibling_prefix() {
        // `/work/alpha` must not own a path under `/work/alpha-two`: matching is
        // by path component, not raw string prefix.
        let roots = vec![
            PathBuf::from("/work/alpha"),
            PathBuf::from("/work/alpha-two"),
        ];
        assert_eq!(
            owning_repo(Path::new("/work/alpha-two/.grove-worktrees/x"), &roots),
            Some(1),
        );
    }

    #[test]
    fn fleet_watch_dirs_are_two_roots_per_repo_in_order() {
        let roots = vec![
            PathBuf::from("/work/alpha"),
            PathBuf::from("/work/beta"),
        ];
        let dirs = fleet_watch_dirs(&roots);
        assert_eq!(
            dirs,
            vec![
                PathBuf::from("/work/alpha/.grove-worktrees"),
                PathBuf::from("/work/alpha/.grove-meta/inboxes"),
                PathBuf::from("/work/beta/.grove-worktrees"),
                PathBuf::from("/work/beta/.grove-meta/inboxes"),
            ],
        );
    }
}
