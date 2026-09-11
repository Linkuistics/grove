// Fixture-driven tests for `grove-llm brief-chain` on Grove's node-file grammar.
// Every level requires a node file: `_BRIEF.md` at the root, `_<slug>.md`
// inside each `NN-k<key>/` directory. Leaves are
// `NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md`.
// The chain contains the root and every ancestor node file, root first;
// missing or competing files refuse the guarded read. Tests use real jj repos.

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
    fs::write(grove.join("_BRIEF.md"), "root brief").unwrap();
    tmp
}

/// Write a leaf/brief file (creating parent dirs as needed).
fn touch(dir: &Path, name: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join(name), b"# stub\n").unwrap();
}

/// Create a node directory, returning its path (for nesting children inside).
fn mknode(dir: &Path, name: &str, slug: &str) -> PathBuf {
    let p = dir.join(name);
    fs::create_dir_all(&p).unwrap();
    fs::write(p.join(format!("_{slug}.md")), "node brief").unwrap();
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

/// The chain's **parent directory names**, root→leaf, for terse assertions.
/// Node-file names and their containing directories are both visible.
fn parent_names(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .map(|l| {
            let p = PathBuf::from(l);
            p.parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

#[test]
fn leaf_at_root_returns_only_root_brief() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "_BRIEF.md");
    touch(&grove, "01-impl--leaf-k1.md");

    let (stdout, _, ok) = run(tmp.path(), &["brief-chain", ".grove/01-impl--leaf-k1.md"]);
    assert!(ok);
    assert_eq!(parent_names(&stdout), vec![".grove"]);
}

#[test]
fn leaf_two_levels_deep_returns_root_and_ancestor_node_briefs() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "_BRIEF.md");
    let outer = mknode(&grove, "01-k1", "outer");
    touch(&outer, "_outer.md");
    let inner = mknode(&outer, "01-k2", "inner");
    touch(&inner, "_inner.md");
    touch(&inner, "01-impl--leaf-k3.md");

    let (stdout, _, ok) = run(
        tmp.path(),
        &["brief-chain", ".grove/01-k1/01-k2/01-impl--leaf-k3.md"],
    );
    assert!(ok);
    // Directory ascent, root→leaf: the root, then each ancestor node dir.
    assert_eq!(parent_names(&stdout), vec![".grove", "01-k1", "01-k2"]);
}

#[test]
fn missing_intermediate_node_file_refuses() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "_BRIEF.md");
    // Remove the outer node file after constructing a valid fixture.
    let outer = mknode(&grove, "01-k1", "outer");
    let inner = mknode(&outer, "01-k2", "inner");
    touch(&inner, "_inner.md");
    touch(&inner, "01-impl--leaf-k3.md");

    fs::remove_file(outer.join("_outer.md")).unwrap();
    let (stdout, stderr, ok) = run(tmp.path(), &["brief-chain"]);
    assert!(!ok);
    assert!(stdout.is_empty());
    assert!(stderr.contains("_<slug>.md"), "{stderr}");
}

#[test]
fn no_arg_form_uses_picks_next_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "_BRIEF.md");
    let node = mknode(&grove, "01-k1", "node");
    touch(&node, "_node.md");
    touch(&node, "01-impl--first-k2.md");

    let (stdout, _, ok) = run(tmp.path(), &["brief-chain"]);
    assert!(ok);
    // pick's next live leaf is the node's first child; its chain is the root
    // brief + the node-1 brief.
    assert_eq!(parent_names(&stdout), vec![".grove", "01-k1"]);
}

#[test]
fn missing_root_node_file_refuses() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-impl--leaf-k1.md");
    fs::remove_file(grove.join("_BRIEF.md")).unwrap();
    let (stdout, stderr, ok) = run(tmp.path(), &["brief-chain"]);
    assert!(!ok);
    assert!(stdout.is_empty());
    assert!(stderr.contains("_BRIEF.md"), "{stderr}");
}

#[test]
fn brief_chain_listed_in_grove_llm_help() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        s.contains("brief-chain"),
        "grove-llm --help missing brief-chain: {s}"
    );
}

#[test]
fn grove_help_does_not_list_brief_chain() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        !s.contains("brief-chain"),
        "grove --help leaked brief-chain from the LLM surface: {s}"
    );
}
