// Fixture-driven tests for `grove-llm resolve <ref>` — the new eighth verb of
// the flat dotted-decimal scheme (ADR-0033/0034). `resolve` turns a reference
// into the current file path, searching live **and** `.DONE` files:
//
//   - `[n]` / `n`      → the unique file whose permanent key is `n`.
//   - `[n]-slug`       → same; the slug part is decorative.
//   - bare slug        → 0 ⇒ not found, 1 ⇒ found, >1 ⇒ ambiguous (list keys).
//
// A `NotFound` is pick-style — empty stdout, a diagnostic on stderr, exit zero.
// Each test stands up a real git repo so `git rev-parse --show-toplevel`
// resolves to the fixture path.

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

fn run(cwd: &Path, args: &[&str]) -> (String, String, bool) {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(cwd)
        .args(args)
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
fn resolve_by_key_bracketed_and_bare() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "1-[1]-alpha.md");
    touch(&grove, "2-[2]-beta.md");

    let (stdout, _, ok) = run(tmp.path(), &["resolve", "1"]);
    assert!(ok);
    assert_eq!(name_of(&stdout), "1-[1]-alpha.md");

    let (stdout, _, ok) = run(tmp.path(), &["resolve", "[2]"]);
    assert!(ok);
    assert_eq!(name_of(&stdout), "2-[2]-beta.md");
}

#[test]
fn resolve_by_unique_slug() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "1-[1]-alpha.md");
    touch(&grove, "2-[2]-beta.md");

    let (stdout, _, ok) = run(tmp.path(), &["resolve", "beta"]);
    assert!(ok);
    assert_eq!(name_of(&stdout), "2-[2]-beta.md");
}

#[test]
fn resolve_ambiguous_slug_lists_keys_on_stderr() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "1-[1]-dup.md");
    touch(&grove, "2-[2]-dup.md");

    let (stdout, stderr, ok) = run(tmp.path(), &["resolve", "dup"]);
    assert!(ok, "ambiguous resolve is not an error");
    assert!(
        stdout.trim().is_empty(),
        "expected empty stdout, got {stdout:?}"
    );
    assert!(
        stderr.contains("ambiguous"),
        "expected ambiguity note, got {stderr:?}"
    );
    assert!(
        stderr.contains("[1]") && stderr.contains("[2]"),
        "keys not listed: {stderr:?}"
    );
}

#[test]
fn resolve_not_found_exits_zero_with_diagnostic() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "1-[1]-alpha.md");

    let (stdout, stderr, ok) = run(tmp.path(), &["resolve", "nope"]);
    assert!(ok, "not-found is pick-style, not an error");
    assert!(
        stdout.trim().is_empty(),
        "expected empty stdout, got {stdout:?}"
    );
    assert!(
        stderr.contains("no file matches"),
        "expected not-found diagnostic, got {stderr:?}"
    );
}

#[test]
fn resolve_finds_retired_leaf_with_note() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "1-[1]-gone.DONE.md");

    let (stdout, stderr, ok) = run(tmp.path(), &["resolve", "1"]);
    assert!(ok);
    assert_eq!(name_of(&stdout), "1-[1]-gone.DONE.md");
    assert!(
        stderr.contains("retired"),
        "expected retired note on stderr, got {stderr:?}"
    );
}

#[test]
fn resolve_listed_in_grove_llm_help() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        s.contains("resolve"),
        "grove-llm --help missing resolve: {s}"
    );
}

#[test]
fn grove_help_does_not_list_resolve() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        !s.contains(" resolve "),
        "grove --help leaked resolve from the LLM surface: {s}"
    );
}
