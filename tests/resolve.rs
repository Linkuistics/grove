// Fixture-driven tests for `grove-llm resolve <ref>` on the **v2 directory
// scheme** (task-tree-scheme). The tree is a real directory tree under `.grove/`: a node
// is a directory `NN-<slug>-k<key>/` of numbered children, optionally headed by a
// `BRIEF.md`;
// leaves are files `NN-[DONE-]<slug>-k<key>.md`. `resolve` turns a reference into
// the current path of the entity it names, searching the whole tree — live
// leaves, retired (`DONE`) leaves, and node directories alike:
//
//   - `[n]` / `n`        → the unique entity whose permanent key is `n`.
//   - `[n]-slug`         → same; the slug part is decorative.
//   - bare slug          → 0 ⇒ not found, 1 ⇒ found, >1 ⇒ ambiguous (list keys).
//   - `<slug>-k<key>`    → the full canonical handle; the terminal `-k<key>` is
//                          read as the key, the slug decorative.
//
// A node resolves to its **directory** path (the dir name carries the key); a
// leaf resolves to its file path. `NotFound`/`Ambiguous` are pick-style — empty
// stdout, a diagnostic on stderr, exit zero; only a malformed bracket ref is a
// non-zero error. Each test stands up a real git repo so
// `git rev-parse --show-toplevel` resolves to the fixture path.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

mod support;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    support::init_jj_repo(tmp.path());
    let grove = tmp.path().join(".grove");
    fs::create_dir_all(&grove).unwrap();
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

/// The parent directory name of the path on the first line of `stdout` — for
/// asserting where a nested entry lives.
fn parent_of(stdout: &str) -> String {
    let line = stdout.lines().next().expect("expected a path on stdout");
    PathBuf::from(line)
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned()
}

#[test]
fn resolve_by_key_bracketed_and_bare() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-impl-alpha-k1.md");
    touch(&grove, "02-impl-beta-k2.md");

    let (stdout, _, ok) = run(tmp.path(), &["resolve", "1"]);
    assert!(ok);
    assert_eq!(name_of(&stdout), "01-impl-alpha-k1.md");

    let (stdout, _, ok) = run(tmp.path(), &["resolve", "[2]"]);
    assert!(ok);
    assert_eq!(name_of(&stdout), "02-impl-beta-k2.md");
}

#[test]
fn resolve_by_unique_slug() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-impl-alpha-k1.md");
    touch(&grove, "02-impl-beta-k2.md");

    let (stdout, _, ok) = run(tmp.path(), &["resolve", "beta"]);
    assert!(ok);
    assert_eq!(name_of(&stdout), "02-impl-beta-k2.md");
}

#[test]
fn resolve_key_resolves_a_node_to_its_directory() {
    // A node's identity rides in its directory name, so a key reference to a node
    // resolves to the directory path (append /BRIEF.md to read its charter).
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    let node = mknode(&grove, "01-design-k1");
    touch(&node, "BRIEF.md");
    touch(&node, "01-impl-inner-k2.md");

    let (stdout, _, ok) = run(tmp.path(), &["resolve", "[1]"]);
    assert!(ok);
    assert_eq!(name_of(&stdout), "01-design-k1");
}

#[test]
fn resolve_finds_a_nested_leaf_by_key() {
    // Key search recurses into node directories: a leaf inside a node resolves to
    // its file path under that node's directory.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    let node = mknode(&grove, "01-design-k1");
    touch(&node, "BRIEF.md");
    touch(&node, "01-impl-inner-k2.md");

    let (stdout, _, ok) = run(tmp.path(), &["resolve", "2"]);
    assert!(ok);
    assert_eq!(name_of(&stdout), "01-impl-inner-k2.md");
    assert_eq!(parent_of(&stdout), "01-design-k1");
}

#[test]
fn resolve_ambiguous_slug_lists_keys_on_stderr() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-impl-dup-k1.md");
    touch(&grove, "02-impl-dup-k2.md");

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
    touch(&grove, "01-impl-alpha-k1.md");

    let (stdout, stderr, ok) = run(tmp.path(), &["resolve", "nope"]);
    assert!(ok, "not-found is pick-style, not an error");
    assert!(
        stdout.trim().is_empty(),
        "expected empty stdout, got {stdout:?}"
    );
    assert!(
        stderr.contains("no entry matches"),
        "expected not-found diagnostic, got {stderr:?}"
    );
}

#[test]
fn resolve_finds_retired_leaf_with_note() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-DONE-impl-gone-k1.md");

    let (stdout, stderr, ok) = run(tmp.path(), &["resolve", "1"]);
    assert!(ok);
    assert_eq!(name_of(&stdout), "01-DONE-impl-gone-k1.md");
    assert!(
        stderr.contains("retired"),
        "expected retired note on stderr, got {stderr:?}"
    );
}

#[test]
fn resolve_by_full_slug_handle_finds_by_terminal_key() {
    // task-tree-scheme §5's canonical commit/prose handle is `<slug>-k<key>`; resolve
    // accepts it directly — the terminal `-k<key>` is read as the key, the slug
    // decorative — so the handle round-trips back to a path.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-impl-alpha-k1.md");
    touch(&grove, "02-impl-beta-k2.md");

    let (stdout, _, ok) = run(tmp.path(), &["resolve", "beta-k2"]);
    assert!(ok);
    assert_eq!(name_of(&stdout), "02-impl-beta-k2.md");
}

#[test]
fn resolve_malformed_bracket_ref_errors() {
    // A bracketed-but-malformed key is a reference error (non-zero), distinct from
    // a valid-but-unmatched reference (which is the pick-style NotFound).
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-impl-alpha-k1.md");

    let (_, _, ok) = run(tmp.path(), &["resolve", "[abc]"]);
    assert!(
        !ok,
        "[abc] is a malformed reference, expected non-zero exit"
    );

    let (_, _, ok) = run(tmp.path(), &["resolve", "[4"]);
    assert!(!ok, "[4 is an unclosed reference, expected non-zero exit");
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
