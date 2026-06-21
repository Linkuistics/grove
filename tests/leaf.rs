// Fixture-driven tests for `grove-llm leaf-add` and `grove-llm leaf-insert` on
// the **new flat dotted-decimal scheme** (ADR-0033/0034). Nodes are addressed by
// their dotted **id**, not a directory path:
//
//   - `leaf-add <parent-id> <slug>`  appends a child at `<parent-id>.<next>`
//     (root parent `.`) with a fresh permanent key.
//   - `leaf-insert <target-id> <slug>` inserts at exactly `<target-id>`, shifting
//     the occupant and later siblings up by one — and the shift cascades through
//     whole subtrees, rewriting only the position (filename + `# …` header).
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

fn touch(p: &Path, body: &str) {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, body.as_bytes()).unwrap();
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
        PathBuf::from(".grove/1-[1]-first-step.md")
    );
    let body = read(tmp.path(), ".grove/1-[1]-first-step.md");
    assert!(body.starts_with("# 1-[1]-first-step\n"), "header: {body:?}");
    assert!(body.contains("**Kind:** work\n"));
    assert!(body.contains("## Goal\n"));
}

#[test]
fn add_to_nonempty_root_uses_next_gapless_position_and_fresh_key() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    touch(&grove.join("1-[1]-existing.md"), "# 1-[1]-existing\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", ".", "second"]);
    assert!(ok);
    // Next root child is position 2; fresh key is max key (1) + 1 = 2.
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/2-[2]-second.md")
    );
}

#[test]
fn add_under_a_node_uses_dotted_child_position() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("1-[1]-node.BRIEF.md"), "# 1-[1]-node — brief\n");
    touch(&grove.join("1.1-[2]-child.md"), "# 1.1-[2]-child\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", "1", "second"]);
    assert!(ok);
    // Next child under node 1 is 1.2; fresh key is max (2) + 1 = 3.
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/1.2-[3]-second.md")
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

    let (_, stderr, ok) = run(tmp.path(), &["leaf-add", "9", "orphan"]);
    assert!(!ok, "expected error adding under a nonexistent parent");
    assert!(
        stderr.contains("parent node 9 not found"),
        "expected parent-not-found diagnostic, got {stderr:?}"
    );
}

// ---------------------------------------------------------------------------
// leaf-insert

#[test]
fn insert_at_start_shifts_root_siblings_up_by_one() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("1-[1]-a.md"), "# 1-[1]-a\n");
    touch(&grove.join("2-[2]-b.md"), "# 2-[2]-b\n");
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-insert", "1", "fresh"]);
    assert!(ok, "leaf-insert failed: {stderr}");
    // New leaf lands at position 1 with a fresh key (max 2 + 1 = 3).
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/1-[3]-fresh.md")
    );
    // The occupant and its later sibling each shift up by one; keys preserved.
    assert!(
        exists(tmp.path(), ".grove/2-[1]-a.md"),
        "a not shifted to 2"
    );
    assert!(
        exists(tmp.path(), ".grove/3-[2]-b.md"),
        "b not shifted to 3"
    );
    assert!(
        !exists(tmp.path(), ".grove/1-[1]-a.md"),
        "old a still present"
    );
    // Renumber summary goes to stderr (stdout stays just the new path).
    assert!(
        stderr.contains("renumber"),
        "expected renumber summary on stderr, got {stderr:?}"
    );
}

#[test]
fn insert_cascades_through_whole_subtree() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("1-[1]-node.BRIEF.md"), "# 1-[1]-node — brief\n");
    touch(&grove.join("1.1-[2]-inner.md"), "# 1.1-[2]-inner\n");
    touch(&grove.join("2-[3]-outer.md"), "# 2-[3]-outer\n");
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-insert", "1", "fresh"]);
    assert!(ok, "leaf-insert failed: {stderr}");
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/1-[4]-fresh.md")
    );
    // node 1 → 2 drags its child 1.1 → 2.1; the unrelated sibling 2 → 3.
    assert!(
        exists(tmp.path(), ".grove/2-[1]-node.BRIEF.md"),
        "node not bumped"
    );
    assert!(
        exists(tmp.path(), ".grove/2.1-[2]-inner.md"),
        "child not dragged"
    );
    assert!(
        exists(tmp.path(), ".grove/3-[3]-outer.md"),
        "outer not bumped"
    );
    // The dragged brief's `# …` header position is rewritten to match.
    let brief = read(tmp.path(), ".grove/2-[1]-node.BRIEF.md");
    assert!(
        brief.starts_with("# 2-[1]-node"),
        "brief header position not rewritten: {brief:?}"
    );
}

#[test]
fn insert_at_end_degenerates_to_add() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("1-[1]-a.md"), "# 1-[1]-a\n");
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-insert", "2", "tail"]);
    assert!(ok);
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/2-[2]-tail.md")
    );
    assert!(
        stderr.contains("no siblings to renumber"),
        "expected empty-renumber note, got {stderr:?}"
    );
    // The pre-existing leaf is untouched.
    assert!(exists(tmp.path(), ".grove/1-[1]-a.md"));
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
