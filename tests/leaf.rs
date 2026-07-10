// Fixture-driven tests for `grove-llm leaf-add` and `grove-llm leaf-insert` on
// the **v2 directory scheme** (task-tree-scheme). A parent/target is addressed by its
// permanent **key** (`[n]` / `n` / `<slug>-k<key>`) or its **path** — not a
// dotted id:
//
//   - `leaf-add <parent> <slug>`  appends a child under the node directory at the
//     next gapless per-level position (root parent `.`) with a fresh key.
//   - `leaf-insert <target> <slug>` inserts at the slot the existing target holds,
//     shifting the target and later siblings up one position. Because the
//     hierarchy lives in directories, a shift is a `git mv` of each sibling
//     directory + the subtree riding along, and in-file headers are position-free
//     (`# <slug>-k<key>`), so the renumber rewrites **zero file contents**.
//     (Appending past the last sibling is `leaf-add`'s job — target must exist.)
//
// Each test stands up a real git repo so the verb's `git mv` calls have tracked
// files to operate on.
//
// - stdout: the new leaf's absolute path, single line.
// - stderr: renumber summary and cross-reference candidates (for insert).

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as Pcmd;
use tempfile::TempDir;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Pcmd::new("git")
        .arg("init")
        .arg(tmp.path())
        .status()
        .unwrap();
    git(
        tmp.path(),
        &["config", "user.email", "grove-test@example.com"],
    );
    git(tmp.path(), &["config", "user.name", "Grove Test"]);
    git(tmp.path(), &["config", "core.hooksPath", "/dev/null"]);
    fs::write(tmp.path().join("README"), b"r\n").unwrap();
    git(tmp.path(), &["add", "README"]);
    git(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

fn git(repo: &Path, args: &[&str]) {
    Pcmd::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .status()
        .unwrap();
}

/// Write a leaf/brief file (creating parents).
fn touch(p: &Path, body: &str) {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, body.as_bytes()).unwrap();
}

/// Create a node directory holding a `BRIEF.md`, returning the directory path.
fn mknode(dir: &Path, name: &str, handle: &str) -> PathBuf {
    let p = dir.join(name);
    fs::create_dir_all(&p).unwrap();
    fs::write(p.join("BRIEF.md"), format!("# {handle} — brief\n")).unwrap();
    p
}

fn stage_all(repo: &Path) {
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-m", "fixture"]);
}

fn run(repo: &Path, args: &[&str]) -> (String, String, bool) {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(repo)
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

fn rel_path(stdout: &str, repo: &Path) -> PathBuf {
    let line = stdout
        .lines()
        .next()
        .expect("expected new leaf path on stdout");
    let abs = PathBuf::from(line).canonicalize().unwrap();
    let root = repo.canonicalize().unwrap();
    abs.strip_prefix(&root).unwrap().to_path_buf()
}

fn read(repo: &Path, rel: &str) -> String {
    fs::read_to_string(repo.join(rel)).unwrap()
}

fn exists(repo: &Path, rel: &str) -> bool {
    repo.join(rel).exists()
}

// ---------------------------------------------------------------------------
// leaf-add

#[test]
fn add_to_empty_root_uses_position_one() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", ".", "first-step"]);
    assert!(ok, "leaf-add failed");
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/01-first-step-k1.md")
    );
    let body = read(tmp.path(), ".grove/01-first-step-k1.md");
    // The in-file header is the position-free handle `# <slug>-k<key>`.
    assert!(body.starts_with("# first-step-k1\n"), "header: {body:?}");
    assert!(body.contains("**Kind:** work\n"));
    assert!(body.contains("## Goal\n"));
}

#[test]
fn add_to_nonempty_root_uses_next_position_and_fresh_key() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    touch(&grove.join("01-existing-k1.md"), "# existing-k1\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", ".", "second"]);
    assert!(ok);
    // Next root child is position 02; fresh key is max key (1) + 1 = 2.
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/02-second-k2.md")
    );
}

#[test]
fn add_under_a_node_by_key_uses_child_position() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    let node = mknode(&grove, "01-node-k1", "node-k1");
    touch(&node.join("01-child-k2.md"), "# child-k2\n");
    stage_all(tmp.path());

    // Parent referenced by its permanent key `1`.
    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", "1", "second"]);
    assert!(ok);
    // Next child under the node is position 02; fresh key is max (2) + 1 = 3.
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/01-node-k1/02-second-k3.md")
    );
}

#[test]
fn add_under_a_node_by_path() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    mknode(&grove, "01-node-k1", "node-k1");
    stage_all(tmp.path());

    // Parent referenced by its grove-root-relative directory path.
    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", "01-node-k1", "only"]);
    assert!(ok, "leaf-add by path failed");
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/01-node-k1/01-only-k2.md")
    );
}

#[test]
fn add_with_planning_kind_writes_planning_in_template() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-add", ".", "plan-it", "--kind", "planning"],
    );
    assert!(ok);
    let body = read(tmp.path(), rel_path(&stdout, tmp.path()).to_str().unwrap());
    assert!(body.contains("**Kind:** planning\n"), "got {body:?}");
}

#[test]
fn add_with_review_kind_succeeds() {
    // task-kind-taxonomy: `review` is one of the five, not just `work`/`planning`.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", ".", "x", "--kind", "review"]);
    assert!(ok);
    let body = read(tmp.path(), rel_path(&stdout, tmp.path()).to_str().unwrap());
    assert!(body.contains("**Kind:** review\n"), "got {body:?}");
}

#[test]
fn add_rejects_an_unrecognised_kind_listing_the_five() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-add", ".", "x", "--kind", "reserch"]);
    assert!(!ok, "an unrecognised --kind must be rejected at write time");
    for label in ["planning", "research", "prototype", "work", "review"] {
        assert!(
            stderr.contains(label),
            "error must list {label:?}, got {stderr:?}"
        );
    }
}

#[test]
fn add_rejects_invalid_slug() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-add", ".", "Bad Slug"]);
    assert!(!ok, "expected invalid-slug rejection");
    assert!(
        stderr.contains("slug") || stderr.contains("lowercase"),
        "expected slug diagnostic, got {stderr:?}"
    );
}

#[test]
fn add_under_nonexistent_parent_errors() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    // Key 9 resolves to nothing (not a path, not a key) → the ref-or-path mapping
    // reports it could not be resolved.
    let (_, stderr, ok) = run(tmp.path(), &["leaf-add", "9", "orphan"]);
    assert!(!ok, "expected error adding under a nonexistent parent");
    assert!(
        stderr.contains("no entry matches") && stderr.contains('9'),
        "expected not-found diagnostic, got {stderr:?}"
    );
}

// ---------------------------------------------------------------------------
// leaf-insert

#[test]
fn insert_at_start_shifts_root_siblings_up_by_one() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-a-k1.md"), "# a-k1\n");
    touch(&grove.join("02-b-k2.md"), "# b-k2\n");
    stage_all(tmp.path());

    // Target the entry holding key 1 (leaf `a`).
    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-insert", "1", "fresh"]);
    assert!(ok, "leaf-insert failed: {stderr}");
    // New leaf lands at position 01 with a fresh key (max 2 + 1 = 3).
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/01-fresh-k3.md")
    );
    // The occupant and its later sibling each shift up by one; keys preserved.
    assert!(
        exists(tmp.path(), ".grove/02-a-k1.md"),
        "a not shifted to 02"
    );
    assert!(
        exists(tmp.path(), ".grove/03-b-k2.md"),
        "b not shifted to 03"
    );
    assert!(
        !exists(tmp.path(), ".grove/01-a-k1.md"),
        "old a still present"
    );
    // Renumber summary goes to stderr (stdout stays just the new path).
    assert!(
        stderr.contains("renumber"),
        "expected renumber summary on stderr, got {stderr:?}"
    );
}

#[test]
fn insert_cascades_a_node_subtree_with_position_free_headers() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    let node = mknode(&grove, "01-node-k1", "node-k1");
    touch(&node.join("01-inner-k2.md"), "# inner-k2\n");
    touch(&grove.join("02-outer-k3.md"), "# outer-k3\n");
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-insert", "1", "fresh"]);
    assert!(ok, "leaf-insert failed: {stderr}");
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/01-fresh-k4.md")
    );
    // node 01 → 02 drags its whole subtree (the child rides along, name and key
    // unchanged); the unrelated sibling 02 → 03.
    assert!(
        exists(tmp.path(), ".grove/02-node-k1/01-inner-k2.md"),
        "node dir + child not moved as a unit"
    );
    assert!(
        exists(tmp.path(), ".grove/03-outer-k3.md"),
        "outer not bumped"
    );
    assert!(
        !exists(tmp.path(), ".grove/01-node-k1"),
        "old node dir still present"
    );
    // The renumber rewrites ZERO file contents — the dragged brief's position-free
    // header is byte-identical (v2's "cascade collapse", task-tree-scheme).
    assert_eq!(
        read(tmp.path(), ".grove/02-node-k1/BRIEF.md"),
        "# node-k1 — brief\n",
        "position-free header must not be rewritten on renumber"
    );
}

#[test]
fn insert_requires_an_existing_target() {
    // In v2, inserting past the last sibling is `leaf-add`'s job — `leaf-insert`'s
    // target must exist. A nonexistent target is an error (not a silent append).
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-a-k1.md"), "# a-k1\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-insert", "9", "tail"]);
    assert!(!ok, "leaf-insert at a nonexistent target should error");
    assert!(
        stderr.contains("no entry matches"),
        "expected not-found diagnostic, got {stderr:?}"
    );
    // The pre-existing leaf is untouched.
    assert!(exists(tmp.path(), ".grove/01-a-k1.md"));
}

// ---------------------------------------------------------------------------
// CLI surface

#[test]
fn leaf_add_and_insert_listed_in_grove_llm_help() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("leaf-add"), "missing leaf-add: {s}");
    assert!(s.contains("leaf-insert"), "missing leaf-insert: {s}");
}

#[test]
fn grove_binary_does_not_expose_leaf_verbs() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(!s.contains("leaf-add"), "grove --help leaked leaf-add: {s}");
    assert!(
        !s.contains("leaf-insert"),
        "grove --help leaked leaf-insert: {s}"
    );
}
