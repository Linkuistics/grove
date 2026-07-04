// Fixture-driven tests for `grove-llm pick` on the **v2 directory scheme**
// (task-tree-scheme). The tree is a real directory tree under `.grove/`: a node is a
// directory `NN-<slug>-k<key>/` holding a `BRIEF.md` + numbered children; leaves
// are files `NN-[DONE-]<slug>-k<key>.md`. `pick` is a recursive depth-first
// pre-order walk returning the first live leaf (not a brief, not `DONE`). Each
// test stands up a real git repo so `git rev-parse --show-toplevel` resolves to
// the fixture path.

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
    Pcmd::new("git")
        .args(["-C"])
        .arg(tmp.path())
        .args(["commit", "--allow-empty", "-m", "init"])
        .status()
        .unwrap();
    tmp
}

/// Write a leaf/brief file (creating parent dirs as needed).
fn touch(dir: &Path, name: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join(name), b"# stub\n").unwrap();
}

/// Create a node directory, returning its path (for nesting children inside).
fn mknode(dir: &Path, name: &str) -> PathBuf {
    let p = dir.join(name);
    fs::create_dir_all(&p).unwrap();
    p
}

fn pick_stdout(cwd: &Path) -> (String, String, bool) {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(cwd)
        .arg("pick")
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

fn name_of(stdout: &str) -> String {
    let line = stdout.lines().next().expect("expected a path on stdout");
    PathBuf::from(line)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned()
}

#[test]
fn picks_first_live_leaf_in_numeric_order() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "02-second-k2.md");
    touch(&grove, "01-first-k1.md");
    touch(&grove, "10-tenth-k3.md");

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    // Numeric per-level order: 1 < 2 < 10.
    assert_eq!(name_of(&stdout), "01-first-k1.md");
}

#[test]
fn descends_a_node_directory_in_preorder() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    let node = mknode(&grove, "01-node-k1");
    touch(&node, "BRIEF.md");
    touch(&node, "01-inner-k2.md");
    touch(&grove, "02-outer-k3.md");

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    // Pre-order: the node at position 01 is fully explored before the sibling
    // leaf at 02, so its first live child wins; briefs are skipped.
    assert_eq!(name_of(&stdout), "01-inner-k2.md");
}

#[test]
fn skips_retired_done_leaves() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-DONE-retired-k1.md");
    touch(&grove, "02-live-k2.md");

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert_eq!(name_of(&stdout), "02-live-k2.md");
}

#[test]
fn fully_retired_grove_prints_diagnostic_and_exits_zero() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-DONE-done-k1.md");

    let (stdout, stderr, ok) = pick_stdout(tmp.path());
    assert!(ok, "fully-retired grove must still exit zero");
    assert!(
        stdout.trim().is_empty(),
        "expected empty stdout, got {stdout:?}"
    );
    assert!(
        stderr.contains("no live leaves"),
        "expected finish diagnostic, got {stderr:?}"
    );
}

#[test]
fn foreign_files_are_not_leaves() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "README.md");
    touch(&grove, "01-real-k1.md");

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert_eq!(name_of(&stdout), "01-real-k1.md");
}

#[test]
fn root_brief_is_not_a_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    touch(&grove, "01-only-k1.md");

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert_eq!(name_of(&stdout), "01-only-k1.md");
}

#[test]
fn errors_when_grove_root_absent() {
    let tmp = init_repo();
    // No `.grove/` directory at all.
    let (_, stderr, ok) = pick_stdout(tmp.path());
    assert!(!ok, "expected error exit with no .grove/");
    assert!(
        stderr.contains("grove root not found"),
        "expected diagnostic, got {stderr:?}"
    );
}

#[test]
fn pick_help_listed_in_grove_llm_help() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("pick"), "grove-llm --help missing pick: {s}");
}

#[test]
fn grove_help_does_not_list_pick() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        !s.contains(" pick "),
        "grove --help leaked pick from the LLM surface: {s}"
    );
}
