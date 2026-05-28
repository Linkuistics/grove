// Fixture-driven tests for the TUI data layer (`grove::repo_view`). Each
// test stands up a temp directory shaped like a real repo — `.git/` so
// repo path resolution works for any future caller, `.grove-worktrees/`
// for live groves, `.grove-meta/inboxes/` for inbox state. We do *not*
// shell out to git: the data layer reads the filesystem only.

use std::fs;
use std::path::Path;
use tempfile::TempDir;

use grove::repo_view::{Lifecycle, RepoView, TaskKind};

fn touch(p: &Path, body: &str) {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, body).unwrap();
}

fn mkdir(p: &Path) {
    fs::create_dir_all(p).unwrap();
}

fn repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    // .git/ presence isn't actually consulted by `repo_view::scan` —
    // scan reads directories under the repo root only. Keep this minimal
    // so the test stays focused on the data layer's contract.
    mkdir(&tmp.path().join(".git"));
    tmp
}

#[test]
fn zero_groves_returns_empty_view() {
    let tmp = repo();
    let view = RepoView::scan(tmp.path()).unwrap();
    assert!(view.groves().is_empty());
    assert!(view.grove("anything").is_none());
}

#[test]
fn live_grove_with_mixed_live_and_retired_leaves() {
    let tmp = repo();
    let grove_root = tmp.path().join(".grove-worktrees").join("alpha").join(".grove");
    touch(&grove_root.join("BRIEF.md"), "# alpha\n");
    touch(&grove_root.join("010-first.md"), "# 010-first\n");
    touch(&grove_root.join("020-node/BRIEF.md"), "# 020-node\n");
    touch(&grove_root.join("020-node/010-child.md"), "# 010-child\n");
    touch(&grove_root.join("020-node/020-child.md"), "# 020-child\n");
    touch(&grove_root.join("done/000-original.md"), "# retired\n");
    touch(&grove_root.join("done/050-old-node/010-old.md"), "# old\n");

    let view = RepoView::scan(tmp.path()).unwrap();

    assert_eq!(view.groves().len(), 1);
    let summary = &view.groves()[0];
    assert_eq!(summary.name, "alpha");
    assert_eq!(summary.lifecycle, Lifecycle::Live);
    assert_eq!(summary.live_leaves, 3); // 010-first + 020-node/{010,020}
    assert_eq!(summary.retired_leaves, 2); // done/000-original + done/050-old-node/010-old
    assert_eq!(summary.inbox_pending, 0);

    let detail = view.grove("alpha").unwrap();
    let tree = detail.task_tree.as_ref().expect("live grove has task tree");
    assert!(tree.root_brief.is_some(), "root brief is loaded");

    // Top-level entries: 010-first.md, 020-node/, done/.
    let names: Vec<&str> = tree.entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, vec!["010-first.md", "020-node", "done"]);

    // The done/ entry must self-flag.
    let done = tree.entries.iter().find(|e| e.name == "done").unwrap();
    assert!(done.is_retired);
    match &done.kind {
        TaskKind::Node { children, .. } => {
            for child in children {
                assert!(child.is_retired, "descendant of done/ is retired: {}", child.name);
            }
        }
        _ => panic!("done/ should be a Node"),
    }

    // 020-node/ has its own brief and two leaves.
    let node = tree.entries.iter().find(|e| e.name == "020-node").unwrap();
    assert!(!node.is_retired);
    match &node.kind {
        TaskKind::Node { brief, children } => {
            assert!(brief.is_some());
            assert_eq!(children.len(), 2);
            assert!(children.iter().all(|c| matches!(c.kind, TaskKind::Leaf) && !c.is_retired));
        }
        _ => panic!("020-node/ should be a Node"),
    }
}

#[test]
fn seed_lifecycle_when_inbox_has_no_worktree() {
    let tmp = repo();
    let inbox = tmp.path().join(".grove-meta").join("inboxes").join("racket-bugs");
    mkdir(&inbox);
    touch(&inbox.join(".gitkeep"), "");

    let view = RepoView::scan(tmp.path()).unwrap();

    assert_eq!(view.groves().len(), 1);
    let summary = &view.groves()[0];
    assert_eq!(summary.name, "racket-bugs");
    assert_eq!(summary.lifecycle, Lifecycle::Seed);
    assert_eq!(summary.live_leaves, 0);
    assert_eq!(summary.retired_leaves, 0);
    assert_eq!(summary.inbox_pending, 0);

    let detail = view.grove("racket-bugs").unwrap();
    assert!(detail.task_tree.is_none());
    assert!(detail.inbox.is_empty());
}

#[test]
fn grove_with_pending_inbox_observations() {
    let tmp = repo();
    // Make it live too, so we can also confirm Live + non-empty inbox
    // composes (the parallel-handoff use case from the parent brief).
    mkdir(&tmp.path().join(".grove-worktrees").join("beta").join(".grove"));
    let inbox = tmp.path().join(".grove-meta").join("inboxes").join("beta");
    touch(&inbox.join(".gitkeep"), "");
    touch(
        &inbox.join("2026-05-28T12:00:00Z-first-aaaaaaaa.md"),
        "first observation\n",
    );
    touch(
        &inbox.join("2026-05-28T12:00:01Z-second-bbbbbbbb.md"),
        "second observation\n",
    );

    let view = RepoView::scan(tmp.path()).unwrap();
    let summary = view.groves().iter().find(|g| g.name == "beta").unwrap();
    assert_eq!(summary.lifecycle, Lifecycle::Live);
    assert_eq!(summary.inbox_pending, 2);

    let detail = view.grove("beta").unwrap();
    assert_eq!(detail.inbox.len(), 2);
    // Chronological order is filename-lex order under ADR-0004.
    let names: Vec<String> = detail
        .inbox
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        names,
        vec![
            "2026-05-28T12:00:00Z-first-aaaaaaaa.md".to_string(),
            "2026-05-28T12:00:01Z-second-bbbbbbbb.md".to_string(),
        ]
    );

    let body = grove::repo_view::read_path(&detail.inbox[0]).unwrap();
    assert_eq!(body, "first observation\n");
}

#[test]
fn live_and_seed_coexist_and_sort_by_name() {
    let tmp = repo();
    mkdir(&tmp.path().join(".grove-worktrees").join("zeta").join(".grove"));
    mkdir(&tmp.path().join(".grove-meta").join("inboxes").join("alpha-seed"));

    let view = RepoView::scan(tmp.path()).unwrap();
    let names: Vec<&str> = view.groves().iter().map(|g| g.name.as_str()).collect();
    assert_eq!(names, vec!["alpha-seed", "zeta"]);
    assert_eq!(view.groves()[0].lifecycle, Lifecycle::Seed);
    assert_eq!(view.groves()[1].lifecycle, Lifecycle::Live);
}
