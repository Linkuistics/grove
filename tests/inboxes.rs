use grove::inboxes;
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

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::new("git").arg("init").arg(tmp.path()).status().unwrap();
    git(tmp.path(), &["config", "user.email", "grove-test@example.com"]);
    git(tmp.path(), &["config", "user.name", "Grove Test"]);
    git(tmp.path(), &["config", "core.hooksPath", "/dev/null"]);
    // Initial commit so the repo has a HEAD; not strictly required by the
    // inbox branch (it starts from an empty tree of its own), but makes the
    // test setup match real-world repos.
    fs::write(tmp.path().join("README"), b"r\n").unwrap();
    git(tmp.path(), &["add", "README"]);
    git(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

fn current_branch(repo: &Path) -> String {
    let out = git(repo, &["rev-parse", "--abbrev-ref", "HEAD"]);
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

#[test]
fn materialise_creates_branch_and_worktree() {
    let repo = init_repo();
    let main_branch = current_branch(repo.path());

    inboxes::materialise(repo.path()).unwrap();

    // Worktree dir + inboxes/ subdir exist on disk.
    let wt = repo.path().join(".grove-inboxes");
    assert!(wt.is_dir(), "worktree dir missing");
    assert!(wt.join("inboxes").is_dir(), "inboxes/ subdir missing");

    // Branch exists locally.
    let out = git(repo.path(), &["branch", "--list", "grove-inboxes"]);
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("grove-inboxes"), "expected grove-inboxes branch in: {s}");

    // Main repo's branch is unchanged.
    assert_eq!(current_branch(repo.path()), main_branch);
}

#[test]
fn materialise_is_idempotent() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    inboxes::materialise(repo.path()).unwrap();
    assert!(repo.path().join(".grove-inboxes/inboxes").is_dir());

    // No duplicate commits on the inbox branch (just the initial one).
    let out = git(repo.path(), &["rev-list", "--count", "grove-inboxes"]);
    let n: usize = String::from_utf8_lossy(&out.stdout).trim().parse().unwrap();
    assert_eq!(n, 1, "expected 1 commit on grove-inboxes, got {n}");
}

#[test]
fn inbox_branch_history_does_not_include_main_history() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    // The grove-inboxes branch has exactly one commit (its empty-tree init),
    // with no shared ancestor with the main branch — that's the whole point
    // of the orphan-style start.
    let out = git(repo.path(), &["rev-list", "--count", "grove-inboxes"]);
    let count: usize = String::from_utf8_lossy(&out.stdout).trim().parse().unwrap();
    assert_eq!(count, 1);

    // The commit's tree is empty (no README).
    let tree = git(repo.path(), &["ls-tree", "-r", "grove-inboxes"]);
    assert!(
        tree.stdout.is_empty(),
        "expected empty tree, got: {}",
        String::from_utf8_lossy(&tree.stdout)
    );
}

#[test]
fn append_creates_seed_file_and_commits_on_inbox_branch() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    inboxes::append(repo.path(), "future-grove", "noticed a bug in X").unwrap();

    let f = repo.path().join(".grove-inboxes/inboxes/future-grove.md");
    let body = fs::read_to_string(&f).unwrap();
    assert!(body.contains("noticed a bug in X"), "body: {body:?}");

    let log = git(repo.path(), &["log", "--oneline", "grove-inboxes"]);
    let s = String::from_utf8_lossy(&log.stdout);
    assert!(s.contains("inbox: append to future-grove"), "log: {s}");

    // The main branch is untouched by the append.
    let main_log = git(repo.path(), &["log", "--oneline", &current_branch(repo.path())]);
    let m = String::from_utf8_lossy(&main_log.stdout);
    assert!(!m.contains("inbox:"), "main branch should not see inbox commits: {m}");
}

#[test]
fn append_to_existing_inbox_appends_not_overwrites() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    inboxes::append(repo.path(), "g", "first").unwrap();
    inboxes::append(repo.path(), "g", "second").unwrap();

    let body = fs::read_to_string(repo.path().join(".grove-inboxes/inboxes/g.md")).unwrap();
    assert!(body.contains("first"), "missing first: {body:?}");
    assert!(body.contains("second"), "missing second: {body:?}");
    // Order preserved.
    assert!(
        body.find("first").unwrap() < body.find("second").unwrap(),
        "expected first before second: {body:?}"
    );
}

#[test]
fn drain_clears_file_and_commits() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    inboxes::append(repo.path(), "g", "obs").unwrap();

    inboxes::drain(repo.path(), "g").unwrap();

    let body = fs::read_to_string(repo.path().join(".grove-inboxes/inboxes/g.md")).unwrap();
    assert!(body.is_empty(), "expected empty file after drain, got: {body:?}");

    let log = git(repo.path(), &["log", "--oneline", "grove-inboxes"]);
    let s = String::from_utf8_lossy(&log.stdout);
    assert!(s.contains("inbox: drain g"), "log: {s}");
    assert!(s.contains("inbox: append to g"), "log: {s}");
}

#[test]
fn drain_on_missing_or_empty_inbox_is_noop() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    // Missing file.
    inboxes::drain(repo.path(), "never-existed").unwrap();
    assert!(!repo.path().join(".grove-inboxes/inboxes/never-existed.md").exists());

    // Empty file.
    let f = repo.path().join(".grove-inboxes/inboxes/empty.md");
    fs::write(&f, b"").unwrap();
    inboxes::drain(repo.path(), "empty").unwrap();
    // No new commit beyond the initial one.
    let out = git(repo.path(), &["rev-list", "--count", "grove-inboxes"]);
    let n: usize = String::from_utf8_lossy(&out.stdout).trim().parse().unwrap();
    assert_eq!(n, 1, "drain on empty file must not commit, got {n} commits");
}

#[test]
fn read_returns_empty_when_worktree_or_file_absent() {
    let repo = init_repo();
    // Worktree not materialised.
    assert_eq!(inboxes::read(repo.path(), "any").unwrap(), "");

    inboxes::materialise(repo.path()).unwrap();
    // Materialised but no file for this name.
    assert_eq!(inboxes::read(repo.path(), "any").unwrap(), "");

    inboxes::append(repo.path(), "any", "hello").unwrap();
    let body = inboxes::read(repo.path(), "any").unwrap();
    assert!(body.contains("hello"));
}

#[test]
fn cross_repo_append_writes_into_other_repos_inbox_only() {
    let repo_a = init_repo();
    let repo_b = init_repo();
    inboxes::materialise(repo_b.path()).unwrap();

    // From repo_a's session, address repo_b's inbox.
    inboxes::append(repo_b.path(), "future-in-b", "noticed in a").unwrap();

    let f = repo_b.path().join(".grove-inboxes/inboxes/future-in-b.md");
    assert!(fs::read_to_string(&f).unwrap().contains("noticed in a"));

    // repo_a has no inbox worktree (we never materialised it).
    assert!(!repo_a.path().join(".grove-inboxes").exists());
}

#[test]
fn append_without_materialisation_fails_with_useful_hint() {
    let repo = init_repo();
    let err = inboxes::append(repo.path(), "any", "obs").unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("grove install") || msg.contains("grove update"),
        "expected hint to run install/update, got: {msg}"
    );
}

#[test]
fn append_is_seed_compatible_when_target_grove_does_not_exist_yet() {
    // The "seed" lifecycle state: nothing addresses-side; the inbox file
    // just sits on the branch waiting for `grove start <name>`.
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    inboxes::append(repo.path(), "racket-bugs", "Pair has wrong arity in expansion").unwrap();

    // No worktree for `racket-bugs` — the seed file exists in isolation.
    assert!(!repo.path().join(".grove-worktrees/racket-bugs").exists());
    assert!(repo.path().join(".grove-inboxes/inboxes/racket-bugs.md").is_file());
}
