//! The fleet data layer (leaf `070-fleet-view/020`). `MultiRepoView` wraps
//! **N** [`RepoView`]s — one per repo root resolved by the discovery layer
//! (`fleet::resolve`, leaf `010`) — scanned concurrently and exposed as the
//! **grouped** (repo → groves) structure the two-level nav renders (070 Q2/Q4).
//!
//! Single-repo is the N=1 case: a one-element `MultiRepoView` reproduces
//! today's single-repo data exactly, so the nav collapses to one render path
//! (070 Q4 "subsume").
//!
//! This module sits **below** the presentation boundary (ADR-0013): no
//! `ratatui`. Following the `fleet` module's pattern, the resolution logic is
//! a pure function returning breadcrumbs as data ([`MultiRepoView::scan_with_warnings`]);
//! the side-effecting [`MultiRepoView::scan`] wrapper prints them to stderr.

use std::path::{Path, PathBuf};

use crate::repo_view::{GroveDetail, GroveSummary, RepoView};

/// A fleet snapshot: the per-repo [`RepoView`]s in fleet order (the order of
/// the `repo_roots` passed to [`scan`](Self::scan), which the discovery layer
/// already sorts current-repo-first / explicit / scanned-alphabetical). A
/// *collection*, mirroring the grouped UI — accessors return per-repo groups,
/// never a flattened list, so repo attribution stays structural (each
/// `RepoView` carries its own `repo_root`).
#[derive(Debug, Clone)]
pub struct MultiRepoView {
    repos: Vec<RepoView>,
}

impl MultiRepoView {
    /// Scan every repo in `repo_roots` concurrently — one [`RepoView::scan`]
    /// per repo on its own thread (scans are independent I/O; a fleet may be a
    /// dozen+ repos). A repo whose scan fails is **dropped** (070 Q3
    /// silent-skip) and reported as a breadcrumb warning, never failing the
    /// whole fleet. Successes are returned in the **input order** of
    /// `repo_roots`, regardless of which scan finishes first.
    pub fn scan_with_warnings(repo_roots: &[PathBuf]) -> (Self, Vec<String>) {
        // Spawn one scoped thread per repo; the closures borrow `&Path` from
        // `repo_roots` directly (scoped threads cannot outlive this call).
        // Joining the handles in spawn order, then zipping back onto
        // `repo_roots`, recovers the input order independent of completion
        // order.
        let results: Vec<anyhow::Result<RepoView>> = std::thread::scope(|s| {
            let handles: Vec<_> = repo_roots
                .iter()
                .map(|root| s.spawn(move || RepoView::scan(root)))
                .collect();
            handles
                .into_iter()
                .map(|h| {
                    h.join()
                        .unwrap_or_else(|_| Err(anyhow::anyhow!("scan thread panicked")))
                })
                .collect()
        });

        let mut repos = Vec::with_capacity(repo_roots.len());
        let mut warnings = Vec::new();
        for (root, res) in repo_roots.iter().zip(results) {
            match res {
                Ok(view) => repos.push(view),
                Err(e) => warnings.push(format!(
                    "grove fleet: skipping repo (scan failed): {}: {e:#}",
                    root.display()
                )),
            }
        }
        (MultiRepoView { repos }, warnings)
    }

    /// Scan the fleet, printing any dropped-repo breadcrumbs to stderr.
    pub fn scan(repo_roots: &[PathBuf]) -> Self {
        let (view, warnings) = Self::scan_with_warnings(repo_roots);
        for w in warnings {
            eprintln!("{w}");
        }
        view
    }

    /// Wrap already-scanned [`RepoView`]s into a fleet **without re-scanning**,
    /// preserving their order. The single-repo `App` path (the N=1 "fleet of
    /// one" that 070 Q4's subsume rests on) builds a fleet from the one
    /// `RepoView` it already holds via this, rather than paying a second scan.
    pub fn from_repos(repos: Vec<RepoView>) -> Self {
        MultiRepoView { repos }
    }

    /// The [`RepoView`] whose root is `repo_root`, or `None` when no repo with
    /// that root is in the fleet. The nav (leaf `040`) uses this to read a
    /// single repo's `cli`/`repo` version data when rendering that repo's grove
    /// rows, since version drift is a per-repo property.
    pub fn repo(&self, repo_root: &Path) -> Option<&RepoView> {
        self.repos.iter().find(|r| r.repo_root == repo_root)
    }

    /// The underlying per-repo views, in fleet order. The primary read surface
    /// for the nav (leaf `040`), which needs each `RepoView`'s full data (its
    /// `repo_root`, version layers, and grove summaries) to render a repo
    /// section header plus its groves.
    pub fn repos(&self) -> &[RepoView] {
        &self.repos
    }

    /// The grouped view the two-level nav renders: `(repo root, its grove
    /// summaries)` per repo, in fleet order. A thin projection of
    /// [`repos`](Self::repos) for callers that only need the
    /// header-plus-rows shape.
    pub fn groups(&self) -> impl Iterator<Item = (&Path, &[GroveSummary])> {
        self.repos
            .iter()
            .map(|r| (r.repo_root.as_path(), r.groves()))
    }

    /// Reach a specific [`GroveDetail`] by `(repo root, grove name)` — the
    /// cross-repo lookup the harness-open path (leaf `050`) uses to resolve a
    /// selected grove back to its owning repo. `None` when no repo with that
    /// root is in the fleet, or that repo has no grove by that name.
    pub fn grove(&self, repo_root: &Path, name: &str) -> Option<&GroveDetail> {
        self.repos
            .iter()
            .find(|r| r.repo_root == repo_root)
            .and_then(|r| r.grove(name))
    }

    /// Targeted re-scan (consumed by the fleet fs-watch, leaf `030`): refresh a
    /// single repo's [`RepoView`] in place, matched by repo root, leaving every
    /// other repo's view untouched. The match is by exact path equality against
    /// the root the repo was scanned with — callers pass a root obtained from
    /// [`repos`](Self::repos) / [`groups`](Self::groups) (production: the
    /// canonical roots from `fleet::resolve`).
    ///
    /// Returns `Ok(true)` when a repo with that root was found and refreshed,
    /// `Ok(false)` when no such repo is in the fleet. A re-scan failure returns
    /// `Err` and leaves the existing view **unchanged** — a transient I/O error
    /// must not drop a repo out of the fleet mid-session or reorder the rest.
    pub fn rescan_repo(&mut self, repo_root: &Path) -> anyhow::Result<bool> {
        let Some(slot) = self.repos.iter_mut().find(|r| r.repo_root == repo_root) else {
            return Ok(false);
        };
        let fresh = RepoView::scan(repo_root)?;
        *slot = fresh;
        Ok(true)
    }

    /// Targeted re-scan driven by a fleet fs-watch event (leaf `070-fleet-view/030`):
    /// resolve which repo owns `event_path` (prefix-match against the scanned
    /// roots, via [`crate::fleet::owning_repo`]) and re-scan **only that** repo's
    /// [`RepoView`] in place — every other repo's view object is untouched (070 Q6).
    /// `Ok(false)` when the path is under no repo in the fleet (a stray event, or
    /// one from a since-removed repo) — a no-op, not an error. A scan failure
    /// propagates as `Err` and, per [`rescan_repo`](Self::rescan_repo), leaves the
    /// existing view unchanged.
    pub fn rescan_for_event_path(&mut self, event_path: &Path) -> anyhow::Result<bool> {
        let roots: Vec<PathBuf> = self.repos.iter().map(|r| r.repo_root.clone()).collect();
        match crate::fleet::owning_repo(event_path, &roots) {
            Some(i) => self.rescan_repo(&roots[i]),
            None => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Create `parent/<name>/` as a repo with one live worktree per grove in
    /// `groves` (`.grove-worktrees/<grove>/.grove/010-x.md`). Returns the repo
    /// path.
    fn make_repo(parent: &Path, name: &str, groves: &[&str]) -> PathBuf {
        let repo = parent.join(name);
        for g in groves {
            add_grove(&repo, g);
        }
        // Ensure the repo dir exists even with no groves.
        fs::create_dir_all(repo.join(".grove-worktrees")).unwrap();
        repo
    }

    /// Add one live grove worktree (`.grove-worktrees/<grove>/.grove/010-x.md`)
    /// under an existing or new repo root.
    fn add_grove(repo: &Path, grove: &str) {
        let task_root = repo.join(".grove-worktrees").join(grove).join(".grove");
        fs::create_dir_all(&task_root).unwrap();
        fs::write(task_root.join("010-x.md"), "# 010-x\n").unwrap();
    }

    #[test]
    fn from_repos_wraps_without_rescanning_and_looks_up_by_root() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["beta"]);
        // Scan each repo independently, then wrap the already-scanned views —
        // this is the single-repo `App` path (N=1 fleet of one) and the
        // nav's per-repo version lookup, neither of which should re-scan.
        let v1 = RepoView::scan(&r1).unwrap();
        let v2 = RepoView::scan(&r2).unwrap();
        let fleet = MultiRepoView::from_repos(vec![v1, v2]);

        assert_eq!(fleet.repos().len(), 2);
        // Order preserved: the wrap is identity over the input vec.
        let groups: Vec<_> = fleet.groups().collect();
        assert_eq!(groups[0].0, r1.as_path());
        assert_eq!(groups[1].0, r2.as_path());
        // `repo` resolves a RepoView by its root for per-repo version data.
        assert_eq!(fleet.repo(&r2).unwrap().repo_root, r2);
        assert!(fleet.repo(&tmp.path().join("nope")).is_none());
    }

    #[test]
    fn preserves_input_order_and_groups_per_repo() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["beta", "gamma"]);

        // Pass [r2, r1]: the concurrent scan must still emit them in this order.
        let (view, warnings) = MultiRepoView::scan_with_warnings(&[r2.clone(), r1.clone()]);
        assert!(warnings.is_empty());

        let groups: Vec<_> = view.groups().collect();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].0, r2.as_path());
        assert_eq!(groups[1].0, r1.as_path());

        // Groves within a repo keep RepoView's name order.
        let r2_groves: Vec<&str> = groups[0].1.iter().map(|g| g.name.as_str()).collect();
        assert_eq!(r2_groves, vec!["beta", "gamma"]);
        let r1_groves: Vec<&str> = groups[1].1.iter().map(|g| g.name.as_str()).collect();
        assert_eq!(r1_groves, vec!["alpha"]);

        // Cross-repo detail lookup keys on (repo, name).
        assert!(view.grove(&r2, "beta").is_some());
        assert!(view.grove(&r1, "alpha").is_some());
        assert!(view.grove(&r1, "beta").is_none()); // beta lives in r2, not r1
    }

    #[test]
    fn one_element_reproduces_single_repo_scan() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta"]);

        let direct = RepoView::scan(&r).unwrap();
        let (fleet, warnings) = MultiRepoView::scan_with_warnings(&[r.clone()]);
        assert!(warnings.is_empty());
        assert_eq!(fleet.repos().len(), 1);

        let wrapped = &fleet.repos()[0];
        assert_eq!(wrapped.repo_root, direct.repo_root);
        assert_eq!(wrapped.cli_version(), direct.cli_version());
        let direct_names: Vec<&str> = direct.groves().iter().map(|g| g.name.as_str()).collect();
        let wrapped_names: Vec<&str> = wrapped.groves().iter().map(|g| g.name.as_str()).collect();
        assert_eq!(direct_names, wrapped_names);
        assert_eq!(direct_names, vec!["alpha", "beta"]);
    }

    #[test]
    fn rescan_refreshes_one_repo_in_place() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["beta"]);
        let (mut view, _) = MultiRepoView::scan_with_warnings(&[r1.clone(), r2.clone()]);

        // A new grove lands in r1 on disk; re-scan only r1.
        add_grove(&r1, "delta");
        assert!(view.rescan_repo(&r1).unwrap());

        let groups: Vec<_> = view.groups().collect();
        // Order preserved, r1 refreshed (alpha + delta), r2 untouched.
        assert_eq!(groups[0].0, r1.as_path());
        let r1_names: Vec<&str> = groups[0].1.iter().map(|g| g.name.as_str()).collect();
        assert_eq!(r1_names, vec!["alpha", "delta"]);
        let r2_names: Vec<&str> = groups[1].1.iter().map(|g| g.name.as_str()).collect();
        assert_eq!(r2_names, vec!["beta"]);

        // An unknown root is a no-op, reported as not-found.
        assert!(!view.rescan_repo(&tmp.path().join("nope")).unwrap());
    }

    #[test]
    fn rescan_for_event_path_targets_only_the_owning_repo() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["beta"]);
        let (mut view, _) = MultiRepoView::scan_with_warnings(&[r1.clone(), r2.clone()]);

        // Capture r2's view object identity by value before the event.
        let r2_before: Vec<String> = view.groups().collect::<Vec<_>>()[1]
            .1
            .iter()
            .map(|g| g.name.clone())
            .collect();

        // A new grove lands in r1 on disk; an event *under r1* fires.
        add_grove(&r1, "delta");
        let event = r1
            .join(".grove-worktrees")
            .join("delta")
            .join(".grove")
            .join("010-x.md");
        assert!(view.rescan_for_event_path(&event).unwrap());

        let groups: Vec<_> = view.groups().collect();
        // r1 refreshed (alpha + delta)…
        let r1_names: Vec<&str> = groups[0].1.iter().map(|g| g.name.as_str()).collect();
        assert_eq!(r1_names, vec!["alpha", "delta"]);
        // …r2 untouched — same groves as before, no re-scan side effect.
        let r2_after: Vec<String> = groups[1].1.iter().map(|g| g.name.clone()).collect();
        assert_eq!(r2_before, r2_after);
        assert_eq!(r2_after, vec!["beta".to_string()]);
    }

    #[test]
    fn rescan_for_event_path_under_no_repo_is_a_noop() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let (mut view, _) = MultiRepoView::scan_with_warnings(&[r1.clone()]);
        // A stray path under no fleet repo: reported not-found, nothing re-scanned.
        assert!(!view
            .rescan_for_event_path(&tmp.path().join("elsewhere/x"))
            .unwrap());
    }

    /// A repo whose scan errors is dropped from the fleet with a breadcrumb,
    /// while a healthy repo alongside it survives. The only portable trigger
    /// for a `RepoView::scan` *error* (vs. an empty result) is an unreadable
    /// `.grove` directory, so this is unix-only and self-skips under root,
    /// which bypasses the permission check.
    #[cfg(unix)]
    #[test]
    fn failing_repo_is_skipped() {
        use std::os::unix::fs::PermissionsExt;

        let tmp = TempDir::new().unwrap();
        let good = make_repo(tmp.path(), "good", &["alpha"]);

        // A repo whose live worktree has an unreadable `.grove/`.
        let bad = tmp.path().join("bad");
        let bad_grove = bad.join(".grove-worktrees").join("beta").join(".grove");
        fs::create_dir_all(&bad_grove).unwrap();
        fs::write(bad_grove.join("010-x.md"), "# 010-x\n").unwrap();
        fs::set_permissions(&bad_grove, fs::Permissions::from_mode(0o000)).unwrap();
        // Running as root bypasses the perm bits — the scan would succeed and
        // there would be nothing to skip. Detect and self-skip.
        if fs::read_dir(&bad_grove).is_ok() {
            fs::set_permissions(&bad_grove, fs::Permissions::from_mode(0o755)).unwrap();
            return;
        }

        let (view, warnings) = MultiRepoView::scan_with_warnings(&[good.clone(), bad.clone()]);
        // Restore perms so TempDir can clean up.
        fs::set_permissions(&bad_grove, fs::Permissions::from_mode(0o755)).unwrap();

        assert_eq!(view.repos().len(), 1);
        assert_eq!(view.repos()[0].repo_root, good);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("bad"));
    }
}
