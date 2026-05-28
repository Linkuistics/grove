// Tests for the `grove meta` verbs introduced by leaf
// 030-cli-meta-remote-and-sync. See ADR-0005 for the binding semantics.

use grove::{inboxes, meta};
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn git(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .expect("git")
}

fn git_ok(repo: &Path, args: &[&str]) {
    let out = git(repo, args);
    assert!(
        out.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&out.stderr)
    );
}

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::new("git").arg("init").arg(tmp.path()).status().unwrap();
    git(tmp.path(), &["config", "user.email", "grove-test@example.com"]);
    git(tmp.path(), &["config", "user.name", "Grove Test"]);
    git(tmp.path(), &["config", "core.hooksPath", "/dev/null"]);
    fs::write(tmp.path().join("README"), b"r\n").unwrap();
    git(tmp.path(), &["add", "README"]);
    git(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

fn init_bare() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::new("git")
        .arg("init")
        .arg("--bare")
        .arg(tmp.path())
        .status()
        .unwrap();
    tmp
}

fn config_get(repo: &Path, key: &str) -> Option<String> {
    let out = git(repo, &["config", "--get", key]);
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

// ---------------------------------------------------------------------------
// init

#[test]
fn init_creates_branch_and_worktree_on_fresh_repo() {
    let repo = init_repo();

    meta::init(repo.path()).unwrap();

    // Branch + worktree both materialised.
    let wt = repo.path().join(".grove-meta");
    assert!(wt.is_dir(), "worktree dir missing");
    assert!(wt.join("inboxes").is_dir(), "inboxes/ subdir missing");

    let out = git(repo.path(), &["branch", "--list", "grove-meta"]);
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("grove-meta"), "expected grove-meta branch: {s}");
}

#[test]
fn init_is_idempotent_when_already_materialised() {
    let repo = init_repo();
    meta::init(repo.path()).unwrap();
    // Second call must succeed without producing extra commits or worktrees.
    meta::init(repo.path()).unwrap();

    let out = git(repo.path(), &["rev-list", "--count", "grove-meta"]);
    let n: usize = String::from_utf8_lossy(&out.stdout).trim().parse().unwrap();
    assert_eq!(n, 1, "expected 1 commit on grove-meta after re-init, got {n}");
}

#[test]
fn init_matches_install_end_state() {
    // Spinning up install in a test would require the full extract+fetch
    // pipeline; instead we assert the contract that install uses the same
    // code path (`inboxes::materialise`) by checking init's outputs match
    // what a direct materialise call produces.
    let init_repo_tmp = init_repo();
    meta::init(init_repo_tmp.path()).unwrap();

    let direct = init_repo();
    inboxes::materialise(direct.path()).unwrap();

    for path in [".grove-meta", ".grove-meta/inboxes"] {
        assert_eq!(
            init_repo_tmp.path().join(path).is_dir(),
            direct.path().join(path).is_dir(),
            "init vs materialise diverge at {}",
            path
        );
    }
    let init_branch = git(init_repo_tmp.path(), &["branch", "--list", "grove-meta"]);
    let direct_branch = git(direct.path(), &["branch", "--list", "grove-meta"]);
    assert_eq!(init_branch.stdout, direct_branch.stdout);
}

// ---------------------------------------------------------------------------
// remote_add

#[test]
fn remote_add_sets_upstream_tracking_when_no_origin_exists() {
    let repo = init_repo();
    let bare = init_bare();
    inboxes::materialise(repo.path()).unwrap();

    meta::remote_add(repo.path(), bare.path().to_str().unwrap()).unwrap();

    let wt = repo.path().join(".grove-meta");
    assert_eq!(
        config_get(&wt, "branch.grove-meta.remote").as_deref(),
        Some("origin")
    );
    assert_eq!(
        config_get(&wt, "branch.grove-meta.merge").as_deref(),
        Some("refs/heads/grove-meta")
    );
    // The origin remote was created.
    let url = config_get(&wt, "remote.origin.url").unwrap();
    assert_eq!(url, bare.path().to_str().unwrap());
}

#[test]
fn remote_add_reuses_existing_origin_when_url_matches() {
    let repo = init_repo();
    let bare = init_bare();
    // Pre-existing origin on the main repo pointing at the same URL —
    // the common case (meta branch lives on the same remote as the code).
    git_ok(
        repo.path(),
        &["remote", "add", "origin", bare.path().to_str().unwrap()],
    );
    inboxes::materialise(repo.path()).unwrap();

    meta::remote_add(repo.path(), bare.path().to_str().unwrap()).unwrap();

    // Tracking config is set; origin URL unchanged.
    let wt = repo.path().join(".grove-meta");
    assert_eq!(
        config_get(&wt, "branch.grove-meta.remote").as_deref(),
        Some("origin")
    );
    assert_eq!(
        config_get(&wt, "remote.origin.url").unwrap(),
        bare.path().to_str().unwrap()
    );
}

#[test]
fn remote_add_refuses_when_existing_origin_has_different_url() {
    let repo = init_repo();
    let other = init_bare();
    let bare = init_bare();
    git_ok(
        repo.path(),
        &["remote", "add", "origin", other.path().to_str().unwrap()],
    );
    inboxes::materialise(repo.path()).unwrap();

    let err = meta::remote_add(repo.path(), bare.path().to_str().unwrap()).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("already exists with URL") && msg.contains("differs from requested"),
        "got: {msg}"
    );

    // origin URL was not overwritten.
    let wt = repo.path().join(".grove-meta");
    assert_eq!(
        config_get(&wt, "remote.origin.url").unwrap(),
        other.path().to_str().unwrap()
    );
    // No tracking config was set.
    assert!(config_get(&wt, "branch.grove-meta.remote").is_none());
}

#[test]
fn remote_add_refuses_when_upstream_already_configured() {
    let repo = init_repo();
    let bare = init_bare();
    inboxes::materialise(repo.path()).unwrap();
    meta::remote_add(repo.path(), bare.path().to_str().unwrap()).unwrap();

    let err = meta::remote_add(repo.path(), bare.path().to_str().unwrap()).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("already has an upstream configured")
            && msg.contains("grove meta remote remove"),
        "got: {msg}"
    );
}

#[test]
fn remote_add_refuses_when_worktree_absent() {
    let repo = init_repo();
    let bare = init_bare();
    let err = meta::remote_add(repo.path(), bare.path().to_str().unwrap()).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("worktree not present") && msg.contains("grove install"),
        "got: {msg}"
    );
}

// ---------------------------------------------------------------------------
// remote_remove

#[test]
fn remote_remove_clears_tracking_but_leaves_origin_alone() {
    let repo = init_repo();
    let bare = init_bare();
    inboxes::materialise(repo.path()).unwrap();
    meta::remote_add(repo.path(), bare.path().to_str().unwrap()).unwrap();

    meta::remote_remove(repo.path()).unwrap();

    let wt = repo.path().join(".grove-meta");
    assert!(config_get(&wt, "branch.grove-meta.remote").is_none());
    assert!(config_get(&wt, "branch.grove-meta.merge").is_none());
    // origin remote is intentionally preserved (shared with the main repo).
    assert_eq!(
        config_get(&wt, "remote.origin.url").unwrap(),
        bare.path().to_str().unwrap()
    );
}

#[test]
fn remote_remove_is_idempotent_when_nothing_configured() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    // No remote ever added — remove should succeed silently.
    meta::remote_remove(repo.path()).unwrap();
}

// ---------------------------------------------------------------------------
// sync

#[test]
fn sync_is_noop_when_no_upstream_configured() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    // Should not error.
    meta::sync(repo.path()).unwrap();
}

#[test]
fn sync_pushes_pending_capture_then_fetch_is_clean() {
    let repo = init_repo();
    let bare = init_bare();
    inboxes::materialise(repo.path()).unwrap();
    meta::remote_add(repo.path(), bare.path().to_str().unwrap()).unwrap();

    inboxes::capture(repo.path(), "yum", "first observation", None).unwrap();
    // capture's best-effort push already published; sync should be a no-op.
    meta::sync(repo.path()).unwrap();

    // Verify bare has grove-meta with the captured commit.
    let out = git(bare.path(), &["log", "--oneline", "grove-meta"]);
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("inbox: capture to yum"), "bare log: {s}");
}

// ---------------------------------------------------------------------------
// end-to-end two-machine simulation (the BRIEF's required smoke test)

#[test]
fn two_machine_sim_a_captures_b_syncs_drain_sees() {
    let bare = init_bare();

    // --- Machine A: materialise, wire to bare, capture (which pushes).
    let a = init_repo();
    git_ok(
        a.path(),
        &["remote", "add", "origin", bare.path().to_str().unwrap()],
    );
    // Seed bare with the main branch so the clone for B has something to
    // check out beyond grove-meta.
    git_ok(a.path(), &["push", "-u", "origin", "HEAD:main"]);

    inboxes::materialise(a.path()).unwrap();
    meta::remote_add(a.path(), bare.path().to_str().unwrap()).unwrap();
    inboxes::capture(a.path(), "yum", "observation from A", None).unwrap();

    // Bare now has the grove-meta branch with A's capture commit.
    let bare_branches = String::from_utf8_lossy(
        &git(bare.path(), &["branch", "--list"]).stdout,
    )
    .into_owned();
    assert!(
        bare_branches.contains("grove-meta"),
        "bare should have grove-meta after A's capture; branches: {bare_branches}"
    );

    // --- Machine B: clone bare, set up grove-meta worktree from origin's,
    // then sync and drain.
    let b_tmp = TempDir::new().unwrap();
    let b = b_tmp.path().join("repo");
    Command::new("git")
        .arg("clone")
        .arg(bare.path())
        .arg(&b)
        .status()
        .unwrap();
    git(&b, &["config", "user.email", "b@example.com"]);
    git(&b, &["config", "user.name", "B"]);
    git(&b, &["config", "core.hooksPath", "/dev/null"]);

    // `git branch <name> <remote-ref>` sets up tracking automatically (git's
    // default `branch.autoSetupMerge=true`). This is how a freshly-cloned
    // machine attaches to an existing grove-meta branch. `grove meta init`
    // covers the orphan-create case (leaf 070); the clone-attach case here
    // is still done by hand and is a candidate for a later expansion of
    // `meta init`.
    git_ok(&b, &["branch", "grove-meta", "origin/grove-meta"]);
    git_ok(&b, &["worktree", "add", ".grove-meta", "grove-meta"]);

    // sync = fetch (no-op, just-cloned) + push (no pending).
    meta::sync(&b).unwrap();

    // drain enumerate on B sees A's observation.
    let pending = inboxes::drain_enumerate(&b, "yum").unwrap();
    assert_eq!(pending.len(), 1, "expected 1 pending on B, got: {pending:?}");
    let body = fs::read_to_string(&pending[0]).unwrap();
    assert!(
        body.contains("observation from A"),
        "B should see A's observation body; got: {body}"
    );
}
