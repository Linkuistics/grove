//! The pure filter/sort engine behind the nav's **filter mode**
//! (rmux-tui-polish 040): [`Criteria`] (fuzzy needle, inbox-pending toggle,
//! lifecycle cycle, sort order) and the projection from a fleet snapshot to a
//! **flat ranked list** of [`Ranked`] rows. No keys, no focus, no rendering —
//! the mode's *interaction* (`/`, layered Esc, the criteria line) is 050; this
//! module is consumable by headless tests before any key exists.
//!
//! Dimensions and cycles are fixed by 010-plan Q3/Q4: the needle matches over
//! the row label (`<repo>/<grove>` at N>1, the bare grove name at N=1 — same
//! convention as the nav's flat picker shape, so at a single-repo fleet typing
//! the repo name does not vacuously match every row); lifecycle cycles
//! all → live → seed; sort cycles name → recency (the 020 `last_commit`
//! datum) → inbox-pending count.
//!
//! Ranking is deterministic: fuzzy score descending, then the active sort
//! order as tiebreak, then the label. An empty needle with any toggle engaged
//! still ranks — the list is simply stable under the active sort.
//!
//! Fuzzy scoring is `nucleo-matcher` (Helix's matcher; chosen over the dormant
//! `fuzzy-matcher` — see Cargo.toml). A single fuzzy [`Atom`] rather than
//! `Pattern::parse`, so fzf operator syntax (`^`, `!`, `'`) is matched
//! literally instead of surprising the user.

use std::cmp::Ordering;
use std::path::PathBuf;

use nucleo_matcher::pattern::{Atom, AtomKind, CaseMatching, Normalization};
use nucleo_matcher::{Config, Matcher, Utf32Str};

use crate::multi_repo_view::MultiRepoView;
use crate::repo_view::Lifecycle;

/// The sort dimension, cycled by Ctrl-s (050). `Name` is the idle default —
/// the grouped shape already lists by name, so only a non-default sort
/// engages the flat shape on its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortOrder {
    #[default]
    Name,
    /// Most recent `last_commit` first; groves with no datum (seeds, failed
    /// lookups) sort last.
    Recency,
    /// Highest pending-observation count first.
    InboxPending,
}

impl SortOrder {
    /// The fixed cycle: name → recency → inbox-pending → name.
    pub fn cycle(self) -> Self {
        match self {
            SortOrder::Name => SortOrder::Recency,
            SortOrder::Recency => SortOrder::InboxPending,
            SortOrder::InboxPending => SortOrder::Name,
        }
    }
}

/// The lifecycle dimension, cycled by Ctrl-l (050). `All` is the idle default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LifecycleFilter {
    #[default]
    All,
    Live,
    Seed,
}

impl LifecycleFilter {
    /// The fixed cycle: all → live → seed → all.
    pub fn cycle(self) -> Self {
        match self {
            LifecycleFilter::All => LifecycleFilter::Live,
            LifecycleFilter::Live => LifecycleFilter::Seed,
            LifecycleFilter::Seed => LifecycleFilter::All,
        }
    }

    fn keeps(self, lifecycle: Lifecycle) -> bool {
        match self {
            LifecycleFilter::All => true,
            LifecycleFilter::Live => lifecycle == Lifecycle::Live,
            LifecycleFilter::Seed => lifecycle == Lifecycle::Seed,
        }
    }
}

/// The filter mode's state: every dimension at its default means **idle**
/// (the nav keeps its grouped shape); any dimension moved off its default
/// means **engaged** (the nav shows the flat ranked list this module
/// produces). Ephemeral per session (constraint 1) — nothing here persists.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Criteria {
    /// The fuzzy needle typed in filter mode. Empty = not engaged; rows are
    /// then kept (subject to the toggles) rather than matched.
    pub needle: String,
    /// Keep only groves with pending inbox observations.
    pub inbox_only: bool,
    pub lifecycle: LifecycleFilter,
    pub sort: SortOrder,
}

impl Criteria {
    /// Whether any dimension is off its default — the grouped-vs-flat shape
    /// boundary the nav switches on.
    pub fn engaged(&self) -> bool {
        *self != Criteria::default()
    }
}

/// One row of the flat ranked list: the same identity the nav keys panes on
/// (`repo_root` + `name`) plus the display label and the data the toggles,
/// sorts, and the 050 criteria line consume.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ranked {
    pub repo_root: PathBuf,
    pub name: String,
    /// `<repo>/<grove>` when the fleet spans more than one repo, the bare
    /// grove name at N=1. Also the fuzzy haystack.
    pub label: String,
    pub lifecycle: Lifecycle,
    pub inbox_pending: usize,
    pub last_commit: Option<i64>,
}

/// Project a fleet snapshot through `criteria` into the flat ranked list:
/// flatten every repo's groves (live *and* seed) into labelled rows, then
/// filter and rank them. Pure — a fresh projection per call, no cached state.
pub fn project(fleet: &MultiRepoView, criteria: &Criteria) -> Vec<Ranked> {
    let multi = fleet.repos().len() > 1;
    let mut rows = Vec::new();
    for repo in fleet.repos() {
        let repo_name = repo
            .repo_root
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| repo.repo_root.to_string_lossy().into_owned());
        for grove in repo.groves() {
            let label = if multi {
                format!("{repo_name}/{}", grove.name)
            } else {
                grove.name.clone()
            };
            rows.push(Ranked {
                repo_root: repo.repo_root.clone(),
                name: grove.name.clone(),
                label,
                lifecycle: grove.lifecycle,
                inbox_pending: grove.inbox_pending,
                last_commit: grove.last_commit,
            });
        }
    }
    rank(rows, criteria)
}

/// Filter + rank already-flattened rows. Split from [`project`] so tests can
/// exercise ranking with hand-built rows (no tempdir fleets, no git fixtures).
fn rank(rows: Vec<Ranked>, criteria: &Criteria) -> Vec<Ranked> {
    let mut kept: Vec<(Option<u16>, Ranked)> = if criteria.needle.is_empty() {
        rows.into_iter()
            .filter(|r| keeps(r, criteria))
            .map(|r| (None, r))
            .collect()
    } else {
        let mut matcher = Matcher::new(Config::DEFAULT);
        let atom = Atom::new(
            &criteria.needle,
            CaseMatching::Ignore,
            Normalization::Smart,
            AtomKind::Fuzzy,
            false,
        );
        let mut buf = Vec::new();
        rows.into_iter()
            .filter(|r| keeps(r, criteria))
            .filter_map(|r| {
                let haystack = Utf32Str::new(&r.label, &mut buf);
                atom.score(haystack, &mut matcher).map(|s| (Some(s), r))
            })
            .collect()
    };
    // Fuzzy score descending, then the active sort order, then the label —
    // a total order, so the result is deterministic regardless of input order.
    kept.sort_by(|(sa, a), (sb, b)| {
        sb.cmp(sa)
            .then_with(|| sort_cmp(a, b, criteria.sort))
            .then_with(|| a.label.cmp(&b.label))
    });
    kept.into_iter().map(|(_, r)| r).collect()
}

/// The toggle dimensions (everything but the needle).
fn keeps(row: &Ranked, criteria: &Criteria) -> bool {
    (!criteria.inbox_only || row.inbox_pending > 0) && criteria.lifecycle.keeps(row.lifecycle)
}

/// The active sort order's comparator. `Name` is `Equal` here — the final
/// label tiebreak in [`rank`] *is* the name order.
fn sort_cmp(a: &Ranked, b: &Ranked, sort: SortOrder) -> Ordering {
    match sort {
        SortOrder::Name => Ordering::Equal,
        SortOrder::Recency => match (a.last_commit, b.last_commit) {
            (Some(x), Some(y)) => y.cmp(&x),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        },
        SortOrder::InboxPending => b.inbox_pending.cmp(&a.inbox_pending),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(name: &str) -> Ranked {
        Ranked {
            repo_root: PathBuf::from("/r"),
            name: name.to_string(),
            label: name.to_string(),
            lifecycle: Lifecycle::Live,
            inbox_pending: 0,
            last_commit: None,
        }
    }

    fn names(rows: &[Ranked]) -> Vec<&str> {
        rows.iter().map(|r| r.name.as_str()).collect()
    }

    // -- engaged/idle boundary ------------------------------------------------

    #[test]
    fn default_criteria_is_idle() {
        assert!(!Criteria::default().engaged());
    }

    #[test]
    fn each_dimension_alone_engages() {
        let c = Criteria { needle: "a".into(), ..Criteria::default() };
        assert!(c.engaged());
        let c = Criteria { inbox_only: true, ..Criteria::default() };
        assert!(c.engaged());
        let c = Criteria { lifecycle: LifecycleFilter::Live, ..Criteria::default() };
        assert!(c.engaged());
        let c = Criteria { sort: SortOrder::Recency, ..Criteria::default() };
        assert!(c.engaged());
    }

    // -- cycles ---------------------------------------------------------------

    #[test]
    fn sort_cycles_name_recency_inbox() {
        let s = SortOrder::default();
        assert_eq!(s, SortOrder::Name);
        let s = s.cycle();
        assert_eq!(s, SortOrder::Recency);
        let s = s.cycle();
        assert_eq!(s, SortOrder::InboxPending);
        assert_eq!(s.cycle(), SortOrder::Name);
    }

    #[test]
    fn lifecycle_cycles_all_live_seed() {
        let l = LifecycleFilter::default();
        assert_eq!(l, LifecycleFilter::All);
        let l = l.cycle();
        assert_eq!(l, LifecycleFilter::Live);
        let l = l.cycle();
        assert_eq!(l, LifecycleFilter::Seed);
        assert_eq!(l.cycle(), LifecycleFilter::All);
    }

    // -- needle ranking -------------------------------------------------------

    #[test]
    fn needle_ranks_contiguous_match_above_scattered() {
        let rows = vec![row("a1b2c3"), row("abc-grove")];
        let c = Criteria { needle: "abc".into(), ..Criteria::default() };
        let out = rank(rows, &c);
        assert_eq!(names(&out), vec!["abc-grove", "a1b2c3"]);
    }

    #[test]
    fn needle_drops_non_matching_rows() {
        let rows = vec![row("alpha"), row("beta")];
        let c = Criteria { needle: "alp".into(), ..Criteria::default() };
        let out = rank(rows, &c);
        assert_eq!(names(&out), vec!["alpha"]);
    }

    #[test]
    fn needle_is_case_insensitive() {
        let rows = vec![row("Alpha")];
        let c = Criteria { needle: "alp".into(), ..Criteria::default() };
        assert_eq!(rank(rows, &c).len(), 1);
    }

    #[test]
    fn score_ties_break_by_active_sort_then_label() {
        // Same needle prefix → equal fuzzy score; recency must decide, even
        // against name order.
        let mut old = row("ab-a");
        old.last_commit = Some(100);
        let mut new = row("ab-x");
        new.last_commit = Some(200);
        let needle = Criteria { needle: "ab".into(), ..Criteria::default() };

        // Name sort (default): label decides the tie.
        let out = rank(vec![new.clone(), old.clone()], &needle);
        assert_eq!(names(&out), vec!["ab-a", "ab-x"]);

        // Recency sort: the newer row wins the tie despite name order.
        let c = Criteria { sort: SortOrder::Recency, ..needle };
        let out = rank(vec![old, new], &c);
        assert_eq!(names(&out), vec!["ab-x", "ab-a"]);
    }

    // -- toggles --------------------------------------------------------------

    #[test]
    fn inbox_toggle_keeps_only_pending_and_stays_name_stable() {
        let mut a = row("alpha");
        a.inbox_pending = 1;
        let b = row("beta");
        let mut c_row = row("gamma");
        c_row.inbox_pending = 3;
        // Empty needle + engaged toggle: still ranks, stable under name sort.
        let c = Criteria { inbox_only: true, ..Criteria::default() };
        let out = rank(vec![c_row, b, a], &c);
        assert_eq!(names(&out), vec!["alpha", "gamma"]);
    }

    #[test]
    fn lifecycle_filter_selects_live_or_seed() {
        let live = row("alpha");
        let mut seed = row("sprout");
        seed.lifecycle = Lifecycle::Seed;
        let rows = || vec![live.clone(), seed.clone()];

        let c = Criteria { lifecycle: LifecycleFilter::Live, ..Criteria::default() };
        assert_eq!(names(&rank(rows(), &c)), vec!["alpha"]);

        let c = Criteria { lifecycle: LifecycleFilter::Seed, ..Criteria::default() };
        assert_eq!(names(&rank(rows(), &c)), vec!["sprout"]);

        // All keeps both; engaged via another dimension is irrelevant here —
        // rank() just applies criteria.
        let c = Criteria { lifecycle: LifecycleFilter::All, ..Criteria::default() };
        assert_eq!(names(&rank(rows(), &c)), vec!["alpha", "sprout"]);
    }

    // -- sort orders ----------------------------------------------------------

    #[test]
    fn sort_by_name_orders_labels_ascending() {
        let rows = vec![row("gamma"), row("alpha"), row("beta")];
        let out = rank(rows, &Criteria::default());
        assert_eq!(names(&out), vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn sort_by_recency_puts_newest_first_and_undated_last() {
        let mut a = row("alpha");
        a.last_commit = Some(100);
        let mut b = row("beta");
        b.last_commit = Some(200);
        let mut seed = row("sprout");
        seed.lifecycle = Lifecycle::Seed; // no branch → no datum
        let c = Criteria { sort: SortOrder::Recency, ..Criteria::default() };
        let out = rank(vec![a, seed, b], &c);
        assert_eq!(names(&out), vec!["beta", "alpha", "sprout"]);
    }

    #[test]
    fn sort_by_inbox_pending_descends_with_name_tiebreak() {
        let mut a = row("alpha");
        a.inbox_pending = 1;
        let mut b = row("beta");
        b.inbox_pending = 3;
        let g = row("gamma"); // 0 pending: listed, just last
        let mut z = row("zeta");
        z.inbox_pending = 3;
        let c = Criteria { sort: SortOrder::InboxPending, ..Criteria::default() };
        let out = rank(vec![z, g, a, b], &c);
        assert_eq!(names(&out), vec!["beta", "zeta", "alpha", "gamma"]);
    }

    // -- project: flatten + labels -------------------------------------------

    use crate::repo_view::RepoView;
    use std::fs;
    use tempfile::TempDir;

    /// Same fixture shape as the nav tests: `parent/<repo>/` with one live
    /// grove worktree per name.
    fn make_repo(parent: &std::path::Path, repo: &str, groves: &[&str]) -> PathBuf {
        let root = parent.join(repo);
        for g in groves {
            let task_root = root.join(".grove-worktrees").join(g).join(".grove");
            fs::create_dir_all(&task_root).unwrap();
            fs::write(task_root.join("010-x.md"), "# 010-x\n").unwrap();
        }
        fs::create_dir_all(root.join(".grove-worktrees")).unwrap();
        root
    }

    fn add_seed(root: &std::path::Path, name: &str) {
        fs::create_dir_all(root.join(".grove-meta").join("inboxes").join(name)).unwrap();
    }

    fn fleet_of(repos: &[PathBuf]) -> MultiRepoView {
        MultiRepoView::from_repos(repos.iter().map(|r| RepoView::scan(r).unwrap()).collect())
    }

    #[test]
    fn project_qualifies_labels_at_multi_repo_and_not_at_single() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["beta"]);

        let out = project(&fleet_of(&[r1.clone(), r2]), &Criteria::default());
        let labels: Vec<&str> = out.iter().map(|r| r.label.as_str()).collect();
        assert_eq!(labels, vec!["one/alpha", "two/beta"]);
        // Identity survives beside the label: the pane map keys on these.
        assert_eq!(out[0].repo_root, r1);
        assert_eq!(out[0].name, "alpha");

        let out = project(&fleet_of(&[r1]), &Criteria::default());
        let labels: Vec<&str> = out.iter().map(|r| r.label.as_str()).collect();
        assert_eq!(labels, vec!["alpha"]);
    }

    #[test]
    fn project_includes_seeds_and_their_pending_counts() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha"]);
        add_seed(&r, "sprout");
        fs::write(
            r.join(".grove-meta/inboxes/sprout/2026-01-01T00:00:00Z-x-aaaaaaaa.md"),
            "obs\n",
        )
        .unwrap();

        // Idle criteria: both lifecycles listed.
        let out = project(&fleet_of(std::slice::from_ref(&r)), &Criteria::default());
        assert_eq!(names(&out), vec!["alpha", "sprout"]);
        let sprout = &out[1];
        assert_eq!(sprout.lifecycle, Lifecycle::Seed);
        assert_eq!(sprout.inbox_pending, 1);
        assert_eq!(sprout.last_commit, None);

        // Lifecycle filtering reaches seeds through the projection too.
        let c = Criteria { lifecycle: LifecycleFilter::Seed, ..Criteria::default() };
        let out = project(&fleet_of(&[r]), &c);
        assert_eq!(names(&out), vec!["sprout"]);
    }

    #[test]
    fn project_needle_matches_over_the_qualified_label() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["alpha"]);
        // The repo name distinguishes same-named groves: `<repo>/<grove>` is
        // the haystack (010-plan Q3).
        let c = Criteria { needle: "two/al".into(), ..Criteria::default() };
        let out = project(&fleet_of(&[r1, r2]), &c);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].label, "two/alpha");
    }
}
