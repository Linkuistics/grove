// Fixture-driven tests for `grove-llm pick` on the **new flat dotted-decimal
// scheme** (ADR-0033/0034). The tree is a flat list of files directly in
// `.grove/` — `<position>-[<key>]-<slug>[.BRIEF|.DONE].md` — so `pick` is a
// single version-sorted scan returning the first live leaf (not a brief, not
// `.DONE`). Each test stands up a real git repo so `git rev-parse
// --show-toplevel` resolves to the fixture path.

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

fn touch(root: &Path, name: &str) {
    fs::create_dir_all(root).unwrap();
    fs::write(root.join(name), b"# stub\n").unwrap();
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
fn picks_first_live_leaf_in_version_sort_order() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "2-[2]-second.md");
    touch(&grove, "1-[1]-first.md");
    touch(&grove, "10-[3]-tenth.md");

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    // Version sort: 1 < 2 < 10 (numeric per-segment, not lexical).
    assert_eq!(name_of(&stdout), "1-[1]-first.md");
}

#[test]
fn descends_subtree_via_position_prefix_not_directories() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    touch(&grove, "1-[1]-node.BRIEF.md");
    touch(&grove, "1.1-[2]-inner.md");
    touch(&grove, "2-[3]-outer.md");

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    // Position order: [] < [1] < [1,1] < [2]; briefs are skipped, so the first
    // live leaf is 1.1 (under node 1), before the sibling 2.
    assert_eq!(name_of(&stdout), "1.1-[2]-inner.md");
}

#[test]
fn skips_retired_done_leaves() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "1-[1]-retired.DONE.md");
    touch(&grove, "2-[2]-live.md");

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert_eq!(name_of(&stdout), "2-[2]-live.md");
}

#[test]
fn fully_retired_grove_prints_diagnostic_and_exits_zero() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "1-[1]-done.DONE.md");

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
fn foreign_files_sort_last_and_are_not_leaves() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "README.md");
    touch(&grove, "1-[1]-real.md");

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert_eq!(name_of(&stdout), "1-[1]-real.md");
}

#[test]
fn root_brief_is_not_a_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    touch(&grove, "1-[1]-only.md");

    let (stdout, _, ok) = pick_stdout(tmp.path());
    assert!(ok);
    assert_eq!(name_of(&stdout), "1-[1]-only.md");
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
