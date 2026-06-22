// Fixture-driven tests for `grove-llm leaf-decompose` and `grove-llm
// leaf-retire` on the **v2 directory scheme** (ADR-0035):
//
//   - `leaf-decompose <leaf-path> <first-child-slug>` converts a live leaf file
//     `NN-<slug>-k<key>.md` into a node DIRECTORY `NN-<slug>-k<key>/` (**key
//     preserved**), `git mv`ing the leaf body in as the node's `BRIEF.md` (its
//     `# <slug>-k<key>` header retitled ` — brief`) and atomically growing a
//     first child `01-<first-child-slug>-k<new>.md` so a node is never childless.
//   - `leaf-retire <leaf-path>` adds a `DONE` infix in place
//     (`NN-<slug>-k<key>.md` → `NN-DONE-<slug>-k<key>.md`), keeping the retired
//     leaf in its directory (no `done/` directory); the file body is untouched.
//
// Each test stands up a real git repo so the verb's `git mv` calls have tracked
// files to operate on.

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
    git(
        tmp.path(),
        &["config", "user.email", "grove-test@example.com"],
    );
    git(tmp.path(), &["config", "user.name", "Grove Test"]);
    git(tmp.path(), &["config", "core.hooksPath", "/dev/null"]);
    fs::write(tmp.path().join("README"), b"r\n").unwrap();
    git(tmp.path(), &["add", "README"]);
    git(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

fn git(repo: &Path, args: &[&str]) {
    Pcmd::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .status()
        .unwrap();
}

/// Write a leaf/brief file (creating parent dirs as needed).
fn touch(p: &Path, body: &str) {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, body.as_bytes()).unwrap();
}

/// Create a node directory holding a `BRIEF.md`, returning the directory path.
fn mknode(dir: &Path, name: &str, handle: &str) -> PathBuf {
    let p = dir.join(name);
    fs::create_dir_all(&p).unwrap();
    fs::write(p.join("BRIEF.md"), format!("# {handle} — brief\n")).unwrap();
    p
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

fn rel_line(stdout: &str, repo: &Path, n: usize) -> PathBuf {
    let line = stdout.lines().nth(n).expect("expected path line on stdout");
    let abs = PathBuf::from(line).canonicalize().unwrap();
    let root = repo.canonicalize().unwrap();
    abs.strip_prefix(&root).unwrap().to_path_buf()
}

fn read(repo: &Path, rel: &str) -> String {
    fs::read_to_string(repo.join(rel)).unwrap()
}

fn exists(repo: &Path, rel: &str) -> bool {
    repo.join(rel).exists()
}

// ---------------------------------------------------------------------------
// leaf-decompose

#[test]
fn decompose_converts_leaf_into_node_directory_with_first_child() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(
        &grove.join("01-target-k1.md"),
        "# target-k1\n\n**Kind:** planning\n\nbody body\n",
    );
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/01-target-k1.md", "sub"],
    );
    assert!(ok, "leaf-decompose failed");

    // stdout: the new node brief path, then the first child path.
    assert_eq!(
        rel_line(&stdout, tmp.path(), 0),
        PathBuf::from(".grove/01-target-k1/BRIEF.md")
    );
    assert_eq!(
        rel_line(&stdout, tmp.path(), 1),
        PathBuf::from(".grove/01-target-k1/01-sub-k2.md")
    );

    // The leaf became a node directory, **key preserved** (k1); the old leaf
    // file is gone, replaced by the directory + its BRIEF.md.
    assert!(exists(tmp.path(), ".grove/01-target-k1/BRIEF.md"));
    assert!(!exists(tmp.path(), ".grove/01-target-k1.md"));
    // The first child exists so the node is never childless.
    assert!(exists(tmp.path(), ".grove/01-target-k1/01-sub-k2.md"));

    // The brief's position-free handle header is retitled with ` — brief`; the
    // rest of the body carries over verbatim.
    let brief = read(tmp.path(), ".grove/01-target-k1/BRIEF.md");
    assert!(
        brief.starts_with("# target-k1 — brief\n"),
        "brief not retitled: {brief:?}"
    );
    assert!(brief.contains("body body"), "brief body lost: {brief:?}");
}

#[test]
fn decompose_rejects_a_brief() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    // A node directory's BRIEF.md is the brief — decomposing it is nonsensical
    // (it is already a node).
    mknode(&grove, "01-node-k1", "node-k1");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/01-node-k1/BRIEF.md", "x"],
    );
    assert!(!ok, "decompose must refuse a brief");
    assert!(
        stderr.contains("brief"),
        "expected brief diagnostic, got {stderr:?}"
    );
}

#[test]
fn decompose_rejects_a_retired_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-DONE-old-k1.md"), "# old-k1\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/01-DONE-old-k1.md", "x"],
    );
    assert!(!ok, "decompose must refuse a retired leaf");
    assert!(
        stderr.contains("retired") || stderr.contains("DONE"),
        "expected retired diagnostic, got {stderr:?}"
    );
}

// ---------------------------------------------------------------------------
// leaf-retire

#[test]
fn retire_adds_done_infix_in_place() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-target-k1.md"), "# target-k1\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-retire", ".grove/01-target-k1.md"]);
    assert!(ok, "leaf-retire failed");
    assert_eq!(
        rel_line(&stdout, tmp.path(), 0),
        PathBuf::from(".grove/01-DONE-target-k1.md")
    );
    assert!(exists(tmp.path(), ".grove/01-DONE-target-k1.md"));
    assert!(!exists(tmp.path(), ".grove/01-target-k1.md"));
    // The DONE infix is filename-only — the body is byte-identical.
    assert_eq!(
        read(tmp.path(), ".grove/01-DONE-target-k1.md"),
        "# target-k1\n"
    );
}

#[test]
fn retire_refuses_a_brief() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    mknode(&grove, "01-node-k1", "node-k1");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-retire", ".grove/01-node-k1/BRIEF.md"]);
    assert!(!ok, "retire must refuse a brief");
    assert!(
        stderr.contains("brief"),
        "expected brief diagnostic, got {stderr:?}"
    );
}

#[test]
fn retire_refuses_an_already_done_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-DONE-old-k1.md"), "# old-k1\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-retire", ".grove/01-DONE-old-k1.md"]);
    assert!(!ok, "retire must refuse an already-retired leaf");
    assert!(
        stderr.contains("already retired") || stderr.contains("DONE"),
        "expected already-retired diagnostic, got {stderr:?}"
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
