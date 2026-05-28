// Fixture-driven tests for `grove-llm leaf-decompose` and
// `grove-llm leaf-retire`. Each test stands up a real git repo with a
// `.grove/<node>/` directory so the verbs' `git mv` calls have something to
// operate on. Output contract for both verbs:
//
// - stdout: the destination's absolute path, single line.
// - stderr: anyhow error context on failure; otherwise empty.

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
    let line = stdout.lines().next().expect("expected destination path on stdout");
    let abs = PathBuf::from(line).canonicalize().unwrap();
    let root = repo.canonicalize().unwrap();
    abs.strip_prefix(&root).unwrap().to_path_buf()
}

fn read(repo: &Path, rel: &str) -> String {
    fs::read_to_string(repo.join(rel)).unwrap()
}

// ---------------------------------------------------------------------------
// leaf-decompose

#[test]
fn decompose_converts_leaf_into_node_with_brief() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# 020-target — brief\n");
    touch(
        &node.join("050-big-leaf.md"),
        "# 050-big-leaf\n\n**Kind:** planning\n\nbody body\n",
    );
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/020-target/050-big-leaf.md"],
    );
    assert!(ok);
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/020-target/050-big-leaf/BRIEF.md")
    );
    // Original leaf file is gone, replaced by the directory.
    assert!(!tmp
        .path()
        .join(".grove/020-target/050-big-leaf.md")
        .exists());
    assert!(tmp.path().join(".grove/020-target/050-big-leaf").is_dir());

    // First line was retitled; body content is otherwise preserved.
    let brief = read(tmp.path(), ".grove/020-target/050-big-leaf/BRIEF.md");
    assert!(
        brief.starts_with("# 050-big-leaf — brief\n"),
        "expected retitled header, got {brief:?}"
    );
    assert!(brief.contains("**Kind:** planning\n"));
    assert!(brief.contains("body body\n"));
}

#[test]
fn decompose_accepts_path_relative_to_grove_root() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# 020-target — brief\n");
    touch(&node.join("030-leaf.md"), "# 030-leaf\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-decompose", "020-target/030-leaf.md"],
    );
    assert!(ok, "expected success, got nothing");
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/020-target/030-leaf/BRIEF.md")
    );
}

#[test]
fn decompose_collides_when_directory_already_exists() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# 020-target — brief\n");
    touch(&node.join("050-clash.md"), "# 050-clash\n");
    // Pre-existing same-stem directory makes decompose impossible.
    fs::create_dir_all(node.join("050-clash")).unwrap();
    touch(&node.join("050-clash/.gitkeep"), "");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/020-target/050-clash.md"],
    );
    assert!(!ok);
    assert!(
        stderr.contains("already exists"),
        "expected collision diagnostic, got {stderr:?}"
    );
    // Source leaf is untouched.
    assert!(tmp.path().join(".grove/020-target/050-clash.md").is_file());
}

#[test]
fn decompose_rejects_non_md_file() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# 020-target — brief\n");
    touch(&node.join("notes.txt"), "stuff\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/020-target/notes.txt"],
    );
    assert!(!ok);
    assert!(
        stderr.contains(".md"),
        "expected .md extension diagnostic, got {stderr:?}"
    );
}

#[test]
fn decompose_rejects_brief_md() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# 020-target — brief\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/020-target/BRIEF.md"],
    );
    assert!(!ok);
    assert!(
        stderr.contains("BRIEF.md"),
        "expected BRIEF.md diagnostic, got {stderr:?}"
    );
}

#[test]
fn decompose_leaves_hand_edited_title_alone() {
    let tmp = init_repo();
    let node = tmp.path().join(".grove/020-target");
    touch(&node.join("BRIEF.md"), "# 020-target — brief\n");
    touch(
        &node.join("050-hand-titled.md"),
        "# Custom hand-edited title\n\nbody\n",
    );
    stage_all(tmp.path());

    let (_, _, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/020-target/050-hand-titled.md"],
    );
    assert!(ok);
    let brief = read(tmp.path(), ".grove/020-target/050-hand-titled/BRIEF.md");
    assert!(
        brief.starts_with("# Custom hand-edited title\n"),
        "expected hand title preserved, got {brief:?}"
    );
}

// ---------------------------------------------------------------------------
// leaf-retire

#[test]
fn retire_moves_root_level_leaf_into_done() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# root brief\n");
    touch(&grove.join("010-top.md"), "# 010-top\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-retire", ".grove/010-top.md"]);
    assert!(ok);
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/done/010-top.md")
    );
    assert!(!tmp.path().join(".grove/010-top.md").exists());
    assert!(tmp.path().join(".grove/done/010-top.md").is_file());
}

#[test]
fn retire_preserves_relative_path_for_nested_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# root brief\n");
    let node = grove.join("020-outer/030-inner");
    touch(&node.join("BRIEF.md"), "# 030-inner — brief\n");
    touch(&node.join("050-deep.md"), "# 050-deep\n\nbody\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-retire", ".grove/020-outer/030-inner/050-deep.md"],
    );
    assert!(ok);
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/done/020-outer/030-inner/050-deep.md")
    );
    // Body content is preserved.
    let moved = read(
        tmp.path(),
        ".grove/done/020-outer/030-inner/050-deep.md",
    );
    assert!(moved.contains("body\n"));
    // Original is gone; the intermediate directories were created under done/.
    assert!(!tmp
        .path()
        .join(".grove/020-outer/030-inner/050-deep.md")
        .exists());
    assert!(tmp
        .path()
        .join(".grove/done/020-outer/030-inner")
        .is_dir());
}

#[test]
fn retire_accepts_path_relative_to_grove_root() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# root brief\n");
    touch(&grove.join("010-top.md"), "# 010-top\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-retire", "010-top.md"]);
    assert!(ok);
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/done/010-top.md")
    );
}

#[test]
fn retire_refuses_destination_collision() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# root brief\n");
    touch(&grove.join("010-top.md"), "# 010-top\n");
    touch(&grove.join("done/010-top.md"), "# prior\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-retire", ".grove/010-top.md"]);
    assert!(!ok);
    assert!(
        stderr.contains("already exists"),
        "expected destination-collision diagnostic, got {stderr:?}"
    );
    // Source is untouched.
    assert!(tmp.path().join(".grove/010-top.md").is_file());
}

#[test]
fn retire_refuses_a_leaf_already_under_done() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# root brief\n");
    touch(&grove.join("done/010-already.md"), "# 010-already\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-retire", ".grove/done/010-already.md"],
    );
    assert!(!ok);
    assert!(
        stderr.contains("done"),
        "expected already-under-done diagnostic, got {stderr:?}"
    );
}

#[test]
fn retire_rejects_brief_md() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# root brief\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-retire", ".grove/BRIEF.md"]);
    assert!(!ok);
    assert!(
        stderr.contains("BRIEF.md"),
        "expected BRIEF.md diagnostic, got {stderr:?}"
    );
}

// ---------------------------------------------------------------------------
// CLI surface

#[test]
fn leaf_decompose_and_retire_listed_in_grove_llm_help() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("leaf-decompose"), "missing leaf-decompose: {s}");
    assert!(s.contains("leaf-retire"), "missing leaf-retire: {s}");
}

#[test]
fn grove_binary_does_not_expose_leaf_decompose_or_retire() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        !s.contains("leaf-decompose"),
        "grove --help leaked leaf-decompose: {s}"
    );
    assert!(
        !s.contains("leaf-retire"),
        "grove --help leaked leaf-retire: {s}"
    );
}
