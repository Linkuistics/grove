// Fixture-driven tests for `grove-llm pick`. Exercises the four shapes the
// existing grove has produced (root-level leaves, nested nodes,
// empty-but-for-`done/` nodes, fully-retired tree) plus the
// non-numeric-prefix and mixed cases. Each test stands up a real git repo so
// `git rev-parse --show-toplevel` resolves to the fixture path.

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

fn touch(p: &Path) {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, b"# stub\n").unwrap();
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

fn rel(stdout: &str, root: &Path) -> PathBuf {
    // Strip trailing newline and the worktree prefix; canonicalise both sides
    // because macOS resolves /var → /private/var.
    let line = stdout.lines().next().unwrap();
    let abs = PathBuf::from(line);
    let abs = abs.canonicalize().unwrap();
    let root = root.canonicalize().unwrap();
    abs.strip_prefix(&root).unwrap().to_path_buf()
}

#[test]
fn picks_first_root_level_leaf_in_prefix_order() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("020-second.md"));
    touch(&grove.join("010-first.md"));
    touch(&grove.join("030-third.md"));

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert_eq!(
        rel(&stdout, tmp.path()),
        PathBuf::from(".grove/010-first.md")
    );
}

#[test]
fn descends_into_nested_nodes_depth_first() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"));
    touch(&grove.join("010-node/BRIEF.md"));
    touch(&grove.join("010-node/020-inner.md"));
    touch(&grove.join("020-outer.md"));

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert_eq!(
        rel(&stdout, tmp.path()),
        PathBuf::from(".grove/010-node/020-inner.md")
    );
}

#[test]
fn skips_done_directory_at_root_and_inside_nodes() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    // A retired leaf sitting in done/ at the root must not be picked.
    touch(&grove.join("done/005-retired.md"));
    touch(&grove.join("010-node/done/015-also-retired.md"));
    touch(&grove.join("010-node/020-live.md"));

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert_eq!(
        rel(&stdout, tmp.path()),
        PathBuf::from(".grove/010-node/020-live.md")
    );
}

#[test]
fn empty_but_for_done_node_falls_through_to_sibling() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    // Node `010-empty` has only a BRIEF.md and a done/ subtree — no live leaves.
    touch(&grove.join("010-empty/BRIEF.md"));
    touch(&grove.join("010-empty/done/005-retired.md"));
    touch(&grove.join("020-bar.md"));

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert_eq!(rel(&stdout, tmp.path()), PathBuf::from(".grove/020-bar.md"));
}

#[test]
fn fully_retired_grove_prints_diagnostic_and_exits_zero() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"));
    touch(&grove.join("done/010-retired.md"));
    touch(&grove.join("done/020-also-retired.md"));

    let (stdout, stderr, ok) = pick_stdout(tmp.path());
    assert!(ok, "expected success exit; stderr={stderr}");
    assert!(
        stdout.trim().is_empty(),
        "expected empty stdout, got {stdout:?}"
    );
    assert!(
        stderr.contains("no live leaves"),
        "expected diagnostic, got {stderr:?}"
    );
}

#[test]
fn non_numeric_prefixed_files_sort_last() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    // A stray non-prefixed file must not steal first place from a numbered leaf.
    touch(&grove.join("NOTES.md"));
    touch(&grove.join("010-first.md"));

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert_eq!(
        rel(&stdout, tmp.path()),
        PathBuf::from(".grove/010-first.md")
    );
}

#[test]
fn brief_md_at_root_is_not_a_leaf() {
    // The grove root's BRIEF.md is the root brief, not a task — pick must
    // skip it and return the first numbered leaf, or no-live-leaves if absent.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"));

    let (stdout, stderr, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert!(stdout.trim().is_empty());
    assert!(stderr.contains("no live leaves"));
}

#[test]
fn errors_when_grove_root_absent() {
    let tmp = init_repo();
    // No `.grove/` directory at all.
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(tmp.path())
        .arg("pick")
        .output()
        .unwrap();
    assert!(!out.status.success(), "expected error exit with no .grove/");
    let stderr = String::from_utf8_lossy(&out.stderr);
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
