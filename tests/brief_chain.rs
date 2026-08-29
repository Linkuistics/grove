// Fixture-driven tests for `grove-llm brief-chain` on the **v2 directory scheme**
// (task-tree-scheme). The tree is a real directory tree under `.grove/`: a node is a
// directory `NN-<slug>-k<key>/` of numbered children, optionally headed by a
// `BRIEF.md`; leaves
// are files `NN-[DONE-]<slug>-k<key>.md`. A leaf's brief chain is collected by
// **directory ascent**: the `BRIEF.md` of each of the leaf's ancestor
// directories, from the grove root down to the leaf's containing directory,
// root→leaf order. A directory level with no `BRIEF.md` is skipped silently.
// Every brief is named `BRIEF.md`, so the assertions key on the **parent
// directory name** of each printed path (mirroring the unit tests in
// src/task_tree.rs). Each test stands up a real jj repo, because every verb
// resolves its grove root through the jj workspace gate.

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

/// The chain's **parent directory names**, root→leaf, for terse assertions.
/// Every brief is named `BRIEF.md`, so the containing directory is what
/// distinguishes them.
fn parent_names(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .map(|l| {
            let p = PathBuf::from(l);
            // The file itself is always `BRIEF.md`; sanity-check it here so a
            // wrong filename surfaces loudly rather than as a parent mismatch.
            assert_eq!(
                p.file_name().unwrap().to_string_lossy(),
                "BRIEF.md",
                "every chain entry must be a BRIEF.md; got {l:?}"
            );
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
    touch(&grove, "BRIEF.md");
    touch(&grove, "01-impl-leaf-k1.md");

    let (stdout, _, ok) = run(tmp.path(), &["brief-chain", ".grove/01-impl-leaf-k1.md"]);
    assert!(ok);
    assert_eq!(parent_names(&stdout), vec![".grove"]);
}

#[test]
fn leaf_two_levels_deep_returns_root_and_ancestor_node_briefs() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    let outer = mknode(&grove, "01-outer-k1");
    touch(&outer, "BRIEF.md");
    let inner = mknode(&outer, "01-inner-k2");
    touch(&inner, "BRIEF.md");
    touch(&inner, "01-impl-leaf-k3.md");

    let (stdout, _, ok) = run(
        tmp.path(),
        &[
            "brief-chain",
            ".grove/01-outer-k1/01-inner-k2/01-impl-leaf-k3.md",
        ],
    );
    assert!(ok);
    // Directory ascent, root→leaf: the root, then each ancestor node dir.
    assert_eq!(
        parent_names(&stdout),
        vec![".grove", "01-outer-k1", "01-inner-k2"]
    );
}

#[test]
fn missing_intermediate_brief_is_skipped_silently() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    // No BRIEF.md in `01-outer` — that level is simply absent from the chain.
    let outer = mknode(&grove, "01-outer-k1");
    let inner = mknode(&outer, "01-inner-k2");
    touch(&inner, "BRIEF.md");
    touch(&inner, "01-impl-leaf-k3.md");

    let (stdout, _, ok) = run(
        tmp.path(),
        &[
            "brief-chain",
            ".grove/01-outer-k1/01-inner-k2/01-impl-leaf-k3.md",
        ],
    );
    assert!(ok);
    assert_eq!(parent_names(&stdout), vec![".grove", "01-inner-k2"]);
}

#[test]
fn no_arg_form_uses_picks_next_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    let node = mknode(&grove, "01-node-k1");
    touch(&node, "BRIEF.md");
    touch(&node, "01-impl-first-k2.md");

    let (stdout, _, ok) = run(tmp.path(), &["brief-chain"]);
    assert!(ok);
    // pick's next live leaf is the node's first child; its chain is the root
    // brief + the node-1 brief.
    assert_eq!(parent_names(&stdout), vec![".grove", "01-node-k1"]);
}

#[test]
fn missing_root_brief_yields_empty_chain() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    // No root BRIEF.md.
    touch(&grove, "01-impl-leaf-k1.md");

    let (stdout, _, ok) = run(tmp.path(), &["brief-chain", ".grove/01-impl-leaf-k1.md"]);
    assert!(ok);
    assert!(
        stdout.trim().is_empty(),
        "expected empty chain, got {stdout:?}"
    );
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
