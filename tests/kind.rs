// Fixture-driven tests for `grove-llm kind` on the **v2 directory scheme**
// (task-tree-scheme). `kind` prints a leaf's task kind — one of the closed five
// (`planning`, `research`, `prototype`, `work`, `review` — ADR
// `task-kind-taxonomy`) — read from its `**Kind:**` line through
// `leaf::Kind::parse` (the single source of truth). It is the primitive the
// self-driving loop uses to choose each session's launch model by the picked
// leaf's kind (model-per-task-kind). With no argument it reads `pick`'s next
// live leaf; on an empty grove it emits the standard "no live leaves"
// diagnostic on stderr and exits 0 (mirroring `brief-chain`). Reading
// **degrades**: a missing or unrecognised `**Kind:**` line warns on stderr and
// is treated as `work` rather than erroring (write still gates — see
// `tests/leaf.rs`'s invalid-`--kind` coverage). Each test stands up a real git
// repo so `git rev-parse --show-toplevel` resolves to the fixture path.

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

/// Write a leaf task file with the given `**Kind:**` label, creating parent dirs.
fn touch_leaf(dir: &Path, name: &str, kind_label: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(
        dir.join(name),
        format!("# stub\n\n**Kind:** {kind_label}\n\n## Goal\n").as_bytes(),
    )
    .unwrap();
}

/// Write a bare file (no `**Kind:**` line), creating parent dirs.
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

#[test]
fn kind_of_a_work_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-build-k1.md", "work");

    let (stdout, _, ok) = run(tmp.path(), &["kind", ".grove/01-build-k1.md"]);
    assert!(ok);
    assert_eq!(stdout, "work\n");
}

#[test]
fn kind_of_a_planning_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-design-k1.md", "planning");

    let (stdout, _, ok) = run(tmp.path(), &["kind", ".grove/01-design-k1.md"]);
    assert!(ok);
    assert_eq!(stdout, "planning\n");
}

#[test]
fn kind_of_the_three_newer_kinds() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    for (name, label) in [
        ("01-a-k1.md", "research"),
        ("02-b-k2.md", "prototype"),
        ("03-c-k3.md", "review"),
    ] {
        touch_leaf(&grove, name, label);
        let (stdout, _, ok) = run(tmp.path(), &["kind", &format!(".grove/{name}")]);
        assert!(ok);
        assert_eq!(stdout, format!("{label}\n"));
    }
}

#[test]
fn no_arg_form_reads_picks_next_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    let node = mknode(&grove, "01-node-k1");
    touch(&node, "BRIEF.md");
    // pick's next live leaf is the node's first child — a planning leaf.
    touch_leaf(&node, "01-first-k2.md", "planning");

    let (stdout, _, ok) = run(tmp.path(), &["kind"]);
    assert!(ok);
    assert_eq!(stdout, "planning\n");
}

#[test]
fn empty_grove_prints_no_live_leaves_on_stderr_and_exits_zero() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md"); // brief only — no live leaf

    let (stdout, stderr, ok) = run(tmp.path(), &["kind"]);
    assert!(ok, "empty grove must exit 0");
    assert!(stdout.is_empty(), "stdout must be empty, got {stdout:?}");
    assert!(
        stderr.contains("no live leaves"),
        "expected the standard diagnostic, got {stderr:?}"
    );
}

#[test]
fn missing_kind_line_degrades_to_work_with_a_warning() {
    // Read degrades (task-kind-taxonomy): a leaf with no `**Kind:**` line at
    // all is treated as `work`, warning on stderr but still exiting 0, so a
    // hand-edited or foreign task file can never jam the self-driving loop.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-broken-k1.md"); // `# stub` only — no `**Kind:**` line

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-broken-k1.md"]);
    assert!(ok, "a missing Kind line must degrade, not error");
    assert_eq!(stdout, "work\n");
    assert!(
        stderr.contains("01-broken-k1.md"),
        "warning must name the file, got {stderr:?}"
    );
    assert!(
        !stderr.contains("panicked"),
        "must not panic, got {stderr:?}"
    );
}

#[test]
fn garbled_kind_token_degrades_to_work_with_a_warning() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-broken-k1.md", "sideways");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-broken-k1.md"]);
    assert!(ok, "an unrecognised kind token must degrade, not error");
    assert_eq!(stdout, "work\n");
    assert!(
        stderr.contains("01-broken-k1.md"),
        "warning must name the file, got {stderr:?}"
    );
}

#[test]
fn a_kind_hand_edited_to_reserch_degrades_to_work_with_exit_zero() {
    // The Done-when example verbatim: a typo'd kind must never error.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-broken-k1.md", "reserch");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-broken-k1.md"]);
    assert!(ok, "exit 0 even on a typo'd kind");
    assert_eq!(stdout, "work\n");
    assert!(!stderr.is_empty(), "a warning must still be printed");
}

#[test]
fn kind_listed_in_grove_llm_help() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("kind"), "grove-llm --help missing kind: {s}");
}

#[test]
fn grove_help_does_not_list_kind() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        !s.contains("Print a leaf's task"),
        "grove --help leaked the kind verb from the LLM surface: {s}"
    );
}
