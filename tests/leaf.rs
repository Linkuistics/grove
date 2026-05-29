// Fixture-driven tests for `grove-llm leaf-add` and `grove-llm leaf-insert`.
// Each test stands up a real git repo with a `.grove/<node>/` directory so
// the verb's `git mv` calls have something to operate on. The verb's output
// contract:
//
// - stdout: the new leaf's absolute path, single line.
// - stderr: renumber summary and cross-reference candidates (for insert).
//
// All file content checks use string equality where helpful so any template
// drift surfaces loudly.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as Pcmd;
use tempfile::TempDir;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Pcmd::new("git").arg("init").arg(tmp.path()).status().unwrap();
    git(tmp.path(), &["config", "user.email", "grove-test@example.com"]);
    git(tmp.path(), &["config", "user.name", "Grove Test"]);
    git(tmp.path(), &["config", "core.hooksPath", "/dev/null"]);
    fs::write(tmp.path().join("README"), b"r\n").unwrap();
    git(tmp.path(), &["add", "README"]);
    git(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

fn git(repo: &Path, args: &[&str]) {
    Pcmd::new("git").arg("-C").arg(repo).args(args).status().unwrap();
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
    let line = stdout.lines().next().expect("expected new leaf path on stdout");
    let abs = PathBuf::from(line).canonicalize().unwrap();
    let root = repo.canonicalize().unwrap();
    abs.strip_prefix(&root).unwrap().to_path_buf()
}

fn read(repo: &Path, rel: &str) -> String {
    fs::read_to_string(repo.join(rel)).unwrap()
}

// ---------------------------------------------------------------------------
// leaf-add

#[test]
fn add_to_empty_node_uses_010_prefix() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    fs::create_dir_all(&node).unwrap();
    touch(&node.join("BRIEF.md"), "# 020-target — brief\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-add", "first-step", "--node", ".grove/020-target"],
    );
    assert!(ok);
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/020-target/010-first-step.md")
    );
    let body = read(tmp.path(), ".grove/020-target/010-first-step.md");
    assert!(body.starts_with("# 010-first-step\n"));
    assert!(body.contains("**Kind:** work\n"));
    assert!(body.contains("## Goal\n"));
}

#[test]
fn add_to_nonempty_node_uses_next_free_prefix() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# 020-target — brief\n");
    touch(&node.join("010-first.md"), "# 010-first\n\n**Kind:** work\n");
    touch(&node.join("020-second.md"), "# 020-second\n\n**Kind:** work\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-add", "third", "--node", ".grove/020-target"],
    );
    assert!(ok);
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/020-target/030-third.md")
    );
}

#[test]
fn add_with_explicit_prefix_to_free_slot_succeeds() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# brief\n");
    touch(&node.join("010-first.md"), "# 010-first\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-add", "explicit", "--prefix", "050", "--node", ".grove/020-target"],
    );
    assert!(ok);
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/020-target/050-explicit.md")
    );
}

#[test]
fn add_with_explicit_prefix_collision_errors() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# brief\n");
    touch(&node.join("010-first.md"), "# 010-first\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-add", "boom", "--prefix", "010", "--node", ".grove/020-target"],
    );
    assert!(!ok);
    assert!(
        stderr.contains("already taken"),
        "expected `already taken` diagnostic, got {stderr:?}"
    );
}

#[test]
fn add_rejects_invalid_slug() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    fs::create_dir_all(&node).unwrap();
    touch(&node.join("BRIEF.md"), "# brief\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-add", "Bad Slug", "--node", ".grove/020-target"],
    );
    assert!(!ok);
    assert!(
        stderr.contains("slug") || stderr.contains("lowercase"),
        "expected slug validation error, got {stderr:?}"
    );
}

#[test]
fn add_with_planning_kind_writes_planning_in_template() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    fs::create_dir_all(&node).unwrap();
    touch(&node.join("BRIEF.md"), "# brief\n");
    stage_all(tmp.path());

    let (_, _, ok) = run(
        tmp.path(),
        &["leaf-add", "design", "--kind", "planning", "--node", ".grove/020-target"],
    );
    assert!(ok);
    let body = read(tmp.path(), ".grove/020-target/010-design.md");
    assert!(body.contains("**Kind:** planning\n"), "got {body:?}");
}

#[test]
fn add_without_node_defaults_to_grove_root() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# root — brief\n");
    stage_all(tmp.path());

    // Run from the worktree root with no --node: the leaf must land under
    // `.grove/` (where `pick` walks), not at the worktree root.
    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", "stray"]);
    assert!(ok, "leaf-add without --node failed");
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/010-stray.md"),
        "leaf must default to .grove/, not the worktree root"
    );
    assert!(
        !tmp.path().join("010-stray.md").exists(),
        "leaf leaked to the worktree root"
    );
}

#[test]
fn add_with_grove_root_relative_node_resolves_under_grove() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# 020-target — brief\n");
    stage_all(tmp.path());

    // `--node 020-target` (no `.grove/` prefix) is grove-root-relative, matching
    // the leaf_ops resolution. It must resolve to `.grove/020-target`.
    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", "first", "--node", "020-target"]);
    assert!(ok, "leaf-add with grove-root-relative --node failed");
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/020-target/010-first.md")
    );
}

// ---------------------------------------------------------------------------
// leaf-insert

#[test]
fn insert_at_start_shifts_all_siblings_up_by_ten() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# 020-target — brief\n");
    touch(&node.join("010-alpha.md"), "# 010-alpha\n\nbody\n");
    touch(&node.join("020-beta.md"), "# 020-beta\n\nbody\n");
    touch(&node.join("030-gamma.md"), "# 030-gamma\n\nbody\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-insert", "010-zero", "--node", ".grove/020-target"],
    );
    assert!(ok);
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/020-target/010-zero.md")
    );

    // All three originals shifted up by ten and their headers rewritten.
    assert!(tmp.path().join(".grove/020-target/010-zero.md").is_file());
    assert!(tmp.path().join(".grove/020-target/020-alpha.md").is_file());
    assert!(tmp.path().join(".grove/020-target/030-beta.md").is_file());
    assert!(tmp.path().join(".grove/020-target/040-gamma.md").is_file());
    assert!(!tmp.path().join(".grove/020-target/010-alpha.md").exists());

    assert!(read(tmp.path(), ".grove/020-target/020-alpha.md").starts_with("# 020-alpha\n"));
    assert!(read(tmp.path(), ".grove/020-target/030-beta.md").starts_with("# 030-beta\n"));
    assert!(read(tmp.path(), ".grove/020-target/040-gamma.md").starts_with("# 040-gamma\n"));
}

#[test]
fn insert_in_middle_shifts_only_siblings_at_or_after() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# brief\n");
    touch(&node.join("010-alpha.md"), "# 010-alpha\n");
    touch(&node.join("020-beta.md"), "# 020-beta\n");
    touch(&node.join("030-gamma.md"), "# 030-gamma\n");
    stage_all(tmp.path());

    let (_, _, ok) = run(
        tmp.path(),
        &["leaf-insert", "020-mid", "--node", ".grove/020-target"],
    );
    assert!(ok);
    // 010 stays put; 020/030 shift.
    assert!(tmp.path().join(".grove/020-target/010-alpha.md").is_file());
    assert!(tmp.path().join(".grove/020-target/020-mid.md").is_file());
    assert!(tmp.path().join(".grove/020-target/030-beta.md").is_file());
    assert!(tmp.path().join(".grove/020-target/040-gamma.md").is_file());
    assert!(read(tmp.path(), ".grove/020-target/030-beta.md").starts_with("# 030-beta\n"));
}

#[test]
fn insert_at_end_degenerates_to_add() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# brief\n");
    touch(&node.join("010-alpha.md"), "# 010-alpha\n");
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["leaf-insert", "020-end", "--node", ".grove/020-target"],
    );
    assert!(ok);
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/020-target/020-end.md")
    );
    assert!(
        stderr.contains("no siblings to renumber"),
        "expected no-renumber diagnostic, got {stderr:?}"
    );
}

#[test]
fn insert_shifts_directory_siblings_and_their_brief_headers() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# 020-target — brief\n");
    // A decomposed sibling: a directory `030-decomposed/` with its own BRIEF.md.
    touch(
        &node.join("030-decomposed/BRIEF.md"),
        "# 030-decomposed — brief\n\nbody\n",
    );
    touch(
        &node.join("030-decomposed/010-inner.md"),
        "# 010-inner\n",
    );
    stage_all(tmp.path());

    let (_, _, ok) = run(
        tmp.path(),
        &["leaf-insert", "020-front", "--node", ".grove/020-target"],
    );
    assert!(ok, "leaf-insert failed");

    // Directory itself was renamed from 030 -> 040.
    assert!(tmp.path().join(".grove/020-target/040-decomposed").is_dir());
    assert!(!tmp.path().join(".grove/020-target/030-decomposed").exists());

    // The inner BRIEF.md's first-line header was rewritten.
    let brief = read(tmp.path(), ".grove/020-target/040-decomposed/BRIEF.md");
    assert!(
        brief.starts_with("# 040-decomposed — brief\n"),
        "expected updated header, got {brief:?}"
    );
    // Inner leaves under the renamed dir are not renumbered — only the
    // directory itself is, because it sits at the *parent* level.
    assert!(tmp
        .path()
        .join(".grove/020-target/040-decomposed/010-inner.md")
        .is_file());
}

#[test]
fn insert_overflow_at_990_errors_without_any_renames() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# brief\n");
    touch(&node.join("980-near-cap.md"), "# 980-near-cap\n");
    touch(&node.join("990-cap.md"), "# 990-cap\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-insert", "980-cant", "--node", ".grove/020-target"],
    );
    assert!(!ok);
    assert!(
        stderr.contains("overflow") || stderr.contains("990"),
        "expected overflow diagnostic, got {stderr:?}"
    );
    // Both originals must still be in place — refuse-before-rename.
    assert!(tmp.path().join(".grove/020-target/980-near-cap.md").is_file());
    assert!(tmp.path().join(".grove/020-target/990-cap.md").is_file());
}

#[test]
fn insert_surfaces_cross_references_on_stderr() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(
        &node.join("BRIEF.md"),
        "# brief\n\nsee 040-old for details and 050-also-old.\n",
    );
    touch(&node.join("040-old.md"), "# 040-old\n\nsee 050-also-old\n");
    touch(&node.join("050-also-old.md"), "# 050-also-old\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-insert", "040-new", "--node", ".grove/020-target"],
    );
    assert!(ok);
    assert!(
        stderr.contains("cross-references to review"),
        "expected cross-refs header, got {stderr:?}"
    );
    // The brief contains references to both old prefixes (040, 050) — both
    // should appear in the surfaced output.
    assert!(stderr.contains("040-"), "expected 040- in stderr, got {stderr:?}");
    assert!(stderr.contains("050-"), "expected 050- in stderr, got {stderr:?}");
}

#[test]
fn insert_without_node_defaults_to_grove_root() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# root — brief\n");
    touch(&grove.join("010-alpha.md"), "# 010-alpha\n");
    stage_all(tmp.path());

    // Run from the worktree root with no --node: insert targets `.grove/`.
    let (stdout, _, ok) = run(tmp.path(), &["leaf-insert", "010-zero"]);
    assert!(ok, "leaf-insert without --node failed");
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/010-zero.md"),
        "inserted leaf must default to .grove/, not the worktree root"
    );
    // The pre-existing sibling shifted up by ten, inside `.grove/`.
    assert!(tmp.path().join(".grove/020-alpha.md").is_file());
    assert!(!tmp.path().join("010-zero.md").exists(), "leaf leaked to worktree root");
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
fn leaf_add_help_no_longer_claims_cwd_default() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .args(["leaf-add", "--help"])
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        !s.contains("current working directory"),
        "leaf-add --help still claims the cwd default: {s}"
    );
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
    assert!(!s.contains("leaf-insert"), "grove --help leaked leaf-insert: {s}");
}
