//! **No tree verb asks whether a kind can be launched.**
//!
//! Grove has no launch configuration, and the owner's harness-dispatch policy
//! is consulted when a leaf launches and nowhere earlier
//! (`docs/specs/harness-selection-and-execution.md`, *A refusal*). So each verb
//! that writes a leaf writes one of any well-formed kind under a HOME that
//! holds no policy at all, and under one that holds the files Grove used to
//! read, which are never read and refuse nothing.
//!
//! Each of these writes was once refused: a kind with no launch template
//! stopped the verb before it touched the tree. The launch-time half of the
//! rule that replaced that is `crates/grove/tests/loop_driver.rs`'s.

use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

mod support;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    support::init_jj_repo(tmp.path());
    tmp
}

fn run(repo: &Path, home: &Path, args: &[&str]) -> (String, bool) {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .env("HOME", home)
        .current_dir(repo)
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

/// The five verbs that write a leaf, each with a kind no methodology declares
/// and no policy could have been asked about.
fn write_with_every_verb(repo: &Path, home: &Path) {
    for (verb, written) in [
        (vec!["root-init", "plan"], "01-requirements--plan-k1.md"),
        (
            vec!["leaf-add", "--kind", "unheard-of", ".", "shape"],
            "02-unheard-of--shape-k2.md",
        ),
        (
            vec!["leaf-insert", "--kind", "another", "shape-k2", "check"],
            "02-another--check-k3.md",
        ),
        (
            vec![
                "leaf-add", ".", "survey", "--kind", "first", "--kind", "second",
            ],
            "05-second--survey-k5.md",
        ),
        (
            vec!["leaf-decompose", ".grove/02-another--check-k3.md", "part"],
            "02-k3/01-another--part-k6.md",
        ),
        (
            vec![
                "leaf-decompose",
                "--kind",
                "third",
                ".grove/01-requirements--plan-k1.md",
                "part",
            ],
            "01-k1/01-third--part-k7.md",
        ),
    ] {
        let (stderr, ok) = run(repo, home, &verb);
        assert!(ok, "{verb:?}: {stderr}");
        assert!(
            repo.join(".grove").join(written).is_file(),
            "{verb:?} did not write {written}: {stderr}"
        );
    }
}

#[test]
fn every_leaf_writing_verb_writes_any_kind_with_no_policy_installed() {
    let repo = init_repo();
    let home = TempDir::new().unwrap();

    write_with_every_verb(repo.path(), home.path());

    assert_eq!(
        fs::read_dir(home.path()).unwrap().count(),
        0,
        "a tree verb reads and writes nothing under HOME"
    );
}

/// The files Grove used to read, in the states it used to refuse: a personal
/// file that does not parse, and a tracked per-checkout file that does not
/// either. Every verb writes as it does without them.
#[test]
fn old_configuration_files_refuse_no_tree_verb() {
    let repo = init_repo();
    let home = TempDir::new().unwrap();
    let personal = home.path().join(".config/grove/config.kdl");
    fs::create_dir_all(personal.parent().unwrap()).unwrap();
    fs::write(&personal, "not valid configuration").unwrap();
    fs::write(repo.path().join(".grove.kdl"), "not valid configuration").unwrap();
    // The listing snapshots the working copy, which tracks the file.
    let files = support::jj(repo.path(), &["file", "list"]);
    assert!(files.lines().any(|file| file == ".grove.kdl"), "{files}");

    write_with_every_verb(repo.path(), home.path());
}
