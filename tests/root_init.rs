// Fixture-driven tests for `grove-llm root-init` on the **v2 directory scheme**
// (task-tree-scheme). Each test stands up a real git repo so `git rev-parse
// --show-toplevel` resolves to the fixture path, then runs the verb against a
// worktree that has *no* `.grove/` yet — the fresh-grove case the verb exists to
// scaffold.
//
// The verb's output contract:
// - stdout: two absolute paths: root `BRIEF.md`, then the first requirements
//   leaf.
// - working-tree change only — makes NO git commit; the scaffold is left
//   untracked for the first session's commit to fold in.
// - refuses (non-zero exit) if `.grove/` already exists.
// - rejects an invalid slug before any mkdir, so no stray `.grove/` is left.
//
// The load-bearing invariant (root BRIEF brief evidence item 4): after
// `root-init`, `grove-llm pick` must return the new leaf — NOT report the
// grove as done. A root brief with no leaves would look finished; the first
// leaf is what makes a newborn grove distinguishable from a retired one.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

mod support;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    support::init_jj_repo(tmp.path());
    fs::write(tmp.path().join("README"), b"r\n").unwrap();
    support::jj(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

fn run(repo: &Path, args: &[&str]) -> (String, String, bool) {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .env("HOME", support::fixture_home())
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

// ---------------------------------------------------------------------------
// happy path

#[test]
fn root_init_creates_root_brief_and_first_requirements_leaf() {
    let tmp = init_repo();

    let (stdout, _, ok) = run(tmp.path(), &["root-init"]);
    assert!(ok, "root-init failed");

    // stdout: the root BRIEF.md first, then the leaf.
    assert_eq!(
        rel_line(&stdout, tmp.path(), 0),
        PathBuf::from(".grove/BRIEF.md")
    );
    assert_eq!(
        rel_line(&stdout, tmp.path(), 1),
        PathBuf::from(".grove/01-requirements--plan-k1.md")
    );
    assert_eq!(
        stdout.lines().count(),
        2,
        "two paths and no more: {stdout:?}"
    );

    // Both entries exist on disk.
    assert!(tmp.path().join(".grove/BRIEF.md").is_file());
    assert!(tmp
        .path()
        .join(".grove/01-requirements--plan-k1.md")
        .is_file());

    // The root brief carries the BRIEF-FORMAT headers and a `— brief` title.
    let brief = read(tmp.path(), ".grove/BRIEF.md");
    assert!(
        brief.contains("— brief\n"),
        "brief missing `— brief` title: {brief:?}"
    );
    assert!(brief.contains("## Goal\n"), "brief missing Goal: {brief:?}");
    assert!(
        brief.contains("## Done when\n"),
        "brief missing Done when: {brief:?}"
    );

    // The first leaf is a *requirements* task per the fresh-grove contract: the
    // bootstrap session's only input is the human's own words (the HITL rule),
    // and the `start` launch is routed on this kind with no leaf to peek. Its
    // header is the position-free handle `# <slug>-k<key>` (the v2 form
    // `leaf_add` writes).
    let leaf = read(tmp.path(), ".grove/01-requirements--plan-k1.md");
    assert!(
        leaf.starts_with("# plan-k1\n"),
        "leaf header wrong: {leaf:?}"
    );
    assert!(
        !leaf.contains("**Kind:**"),
        "kind leaked into body: {leaf:?}"
    );
}

#[test]
fn root_init_default_slug_is_plan() {
    let tmp = init_repo();
    let (stdout, _, ok) = run(tmp.path(), &["root-init"]);
    assert!(ok);
    assert_eq!(
        rel_line(&stdout, tmp.path(), 1),
        PathBuf::from(".grove/01-requirements--plan-k1.md")
    );
}

#[test]
fn root_init_custom_slug_names_the_first_leaf() {
    let tmp = init_repo();
    let (stdout, _, ok) = run(tmp.path(), &["root-init", "design-the-api"]);
    assert!(ok);
    assert_eq!(
        rel_line(&stdout, tmp.path(), 1),
        PathBuf::from(".grove/01-requirements--design-the-api-k1.md")
    );
    let leaf = read(tmp.path(), ".grove/01-requirements--design-the-api-k1.md");
    assert!(!leaf.contains("**Kind:**"), "got {leaf:?}");
}

#[test]
fn root_init_rejects_invalid_slug_without_creating_grove() {
    let tmp = init_repo();
    let (_, stderr, ok) = run(tmp.path(), &["root-init", "Bad Slug"]);
    assert!(!ok);
    assert!(
        stderr.contains("slug") || stderr.contains("lowercase"),
        "expected slug validation error, got {stderr:?}"
    );
    // No stray `.grove/` left behind — validation happens before any mkdir.
    assert!(
        !tmp.path().join(".grove").exists(),
        "left a stray .grove/ behind"
    );
}

// ---------------------------------------------------------------------------
// no-clobber guard

#[test]
fn root_init_refuses_when_grove_already_exists() {
    let tmp = init_repo();
    fs::create_dir_all(tmp.path().join(".grove")).unwrap();
    fs::write(
        tmp.path().join(".grove/BRIEF.md"),
        "# existing — brief\n".as_bytes(),
    )
    .unwrap();
    support::jj(tmp.path(), &["commit", "-m", "pre-existing grove"]);

    let (_, stderr, ok) = run(tmp.path(), &["root-init"]);
    assert!(!ok, "root-init clobbered an existing grove");
    assert!(
        stderr.contains("already exists"),
        "expected already-exists diagnostic, got {stderr:?}"
    );
    // The existing brief is untouched.
    assert_eq!(
        read(tmp.path(), ".grove/BRIEF.md"),
        "# existing — brief\n".to_string()
    );
}

// ---------------------------------------------------------------------------
// the load-bearing invariant: pick returns the new leaf, not "done"

#[test]
fn after_root_init_pick_returns_the_new_leaf_not_done() {
    let tmp = init_repo();
    let (_, _, ok) = run(tmp.path(), &["root-init"]);
    assert!(ok);

    let (stdout, stderr, ok) = run(tmp.path(), &["pick"]);
    assert!(ok);
    // pick must name the leaf on stdout...
    assert_eq!(
        rel_line(&stdout, tmp.path(), 0),
        PathBuf::from(".grove/01-requirements--plan-k1.md"),
        "pick did not return the scaffolded leaf"
    );
    // ...and must NOT report the grove as done (the empty-but-briefed trap).
    assert!(
        !stderr.contains("no live leaves") && !stderr.contains("this grove is done"),
        "fresh grove mis-reported as done: {stderr:?}"
    );
}

// ---------------------------------------------------------------------------
// no commit — working-tree change only

#[test]
fn root_init_makes_no_commit() {
    let tmp = init_repo();
    let before = support::jj(
        tmp.path(),
        &[
            "--ignore-working-copy",
            "log",
            "-r",
            "@-",
            "--no-graph",
            "-T",
            "commit_id",
        ],
    );

    let (_, _, ok) = run(tmp.path(), &["root-init"]);
    assert!(ok);

    let after = support::jj(
        tmp.path(),
        &[
            "--ignore-working-copy",
            "log",
            "-r",
            "@-",
            "--no-graph",
            "-T",
            "commit_id",
        ],
    );
    assert_eq!(before, after, "root-init unexpectedly committed");
    // The scaffold is on disk and is *not* in the revision the working copy sits
    // on. jj has no untracked-file state to assert instead: it snapshots the
    // working copy on the next command, so "nothing was committed" is a claim
    // about `@-`, and the scaffold's presence is a claim about the filesystem.
    assert!(tmp.path().join(".grove/BRIEF.md").is_file());
    assert_eq!(
        support::jj(
            tmp.path(),
            &[
                "--ignore-working-copy",
                "file",
                "list",
                "-r",
                "@-",
                "root:.grove"
            ],
        ),
        "",
        "root-init left `.grove/` in the committed revision"
    );
}

// ---------------------------------------------------------------------------
// CLI surface

#[test]
fn root_init_listed_in_grove_llm_help() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .env("HOME", support::fixture_home())
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("root-init"), "missing root-init: {s}");
}

#[test]
fn grove_binary_does_not_expose_root_init() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        !s.contains("root-init"),
        "grove --help leaked root-init: {s}"
    );
}
