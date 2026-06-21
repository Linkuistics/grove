// Fixture-driven tests for `grove-llm brief-chain` on the **new flat
// dotted-decimal scheme** (ADR-0033/0034). A leaf's brief chain is collected by
// **id-prefix** in the flat namespace, not by directory ascent: a leaf at
// position `[a,b,c]` pulls the briefs at `[]` (root `BRIEF.md`), `[a]`, and
// `[a,b]`, root→leaf. A missing brief at any level is skipped silently.

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

/// The chain's filenames, root→leaf, for terse assertions.
fn names(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .map(|l| {
            PathBuf::from(l)
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
    touch(&grove, "1-[1]-leaf.md");

    let (stdout, _, ok) = run(tmp.path(), &["brief-chain", ".grove/1-[1]-leaf.md"]);
    assert!(ok);
    assert_eq!(names(&stdout), vec!["BRIEF.md"]);
}

#[test]
fn leaf_two_levels_deep_returns_root_and_ancestor_node_briefs() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    touch(&grove, "1-[1]-outer.BRIEF.md");
    touch(&grove, "1.1-[2]-inner.BRIEF.md");
    touch(&grove, "1.1.1-[3]-leaf.md");

    let (stdout, _, ok) = run(tmp.path(), &["brief-chain", ".grove/1.1.1-[3]-leaf.md"]);
    assert!(ok);
    assert_eq!(
        names(&stdout),
        vec!["BRIEF.md", "1-[1]-outer.BRIEF.md", "1.1-[2]-inner.BRIEF.md"]
    );
}

#[test]
fn missing_intermediate_brief_is_skipped_silently() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    // No brief at position [1] — the [1] level is simply absent from the chain.
    touch(&grove, "1.1-[2]-inner.BRIEF.md");
    touch(&grove, "1.1.1-[3]-leaf.md");

    let (stdout, _, ok) = run(tmp.path(), &["brief-chain", ".grove/1.1.1-[3]-leaf.md"]);
    assert!(ok);
    assert_eq!(names(&stdout), vec!["BRIEF.md", "1.1-[2]-inner.BRIEF.md"]);
}

#[test]
fn no_arg_form_uses_picks_next_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    touch(&grove, "1-[1]-node.BRIEF.md");
    touch(&grove, "1.1-[2]-first.md");

    let (stdout, _, ok) = run(tmp.path(), &["brief-chain"]);
    assert!(ok);
    // pick's next leaf is 1.1; its chain is the root brief + the node-1 brief.
    assert_eq!(names(&stdout), vec!["BRIEF.md", "1-[1]-node.BRIEF.md"]);
}

#[test]
fn missing_root_brief_yields_empty_chain() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    // No root BRIEF.md.
    touch(&grove, "1-[1]-leaf.md");

    let (stdout, _, ok) = run(tmp.path(), &["brief-chain", ".grove/1-[1]-leaf.md"]);
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
