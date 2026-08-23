// Fixture-driven tests for `grove-llm leaf-decompose` and `grove-llm
// leaf-retire` on the current witnessed directory scheme (task-tree-scheme):
//
//   - `leaf-decompose <leaf-path> <first-child-slug>` converts a live leaf file
//     `NN-<kind>-<slug>-k<key>.md` into a node DIRECTORY `NN-<slug>-k<key>/` (**key
//     preserved**), `git mv`ing the leaf body in as the node's `BRIEF.md` (its
//     `# <slug>-k<key>` header retitled ` — brief`) and atomically growing a
//     first child `01-<kind>-<first-child-slug>-k<new>.md` so a node is never childless.
//   - `leaf-retire <leaf-path>` adds a `DONE` infix in place
//     (`NN-<kind>-<slug>-k<key>.md` → `NN-DONE-<kind>-<slug>-k<key>.md`), keeping the retired
//     leaf in its directory (no `done/` directory); the file body is untouched.
//     It marks through `ordinal-fs-tree`'s `rewrite`, whose rename is
//     `rename(2)` on every lane — so the repo below is what makes the Git-lane
//     consequence assertable, not what the verb needs to work.
//   - both terminal-marking verbs name the session's remaining steps — commit,
//     then `grove-llm complete` — on stderr, leaving stdout as the parsed path
//     data it already was.
//
// Each test stands up a real git repo. `leaf-decompose` still moves entries with
// `git mv` where they are tracked, so it needs one; the marking verbs no longer
// do, and for them the repo is the *instrument* — see *What Git shows between
// the verb and the commit* below.

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
    let grove = repo.join(".grove");
    if grove.is_dir() {
        fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    }
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
        &grove.join("01-planning-target-k1.md"),
        "# target-k1\n\nbody body\n",
    );
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/01-planning-target-k1.md", "sub"],
    );
    assert!(ok, "leaf-decompose failed");

    // stdout: the new node brief path, then the first child path.
    assert_eq!(
        rel_line(&stdout, tmp.path(), 0),
        PathBuf::from(".grove/01-target-k1/BRIEF.md")
    );
    assert_eq!(
        rel_line(&stdout, tmp.path(), 1),
        PathBuf::from(".grove/01-target-k1/01-planning-sub-k2.md")
    );

    // The leaf became a node directory, **key preserved** (k1); the old leaf
    // file is gone, replaced by the directory + its BRIEF.md.
    assert!(exists(tmp.path(), ".grove/01-target-k1/BRIEF.md"));
    assert!(!exists(tmp.path(), ".grove/01-planning-target-k1.md"));
    // The first child exists so the node is never childless.
    assert!(exists(
        tmp.path(),
        ".grove/01-target-k1/01-planning-sub-k2.md"
    ));

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
fn decompose_with_no_kind_flag_gives_the_first_child_the_parent_leafs_kind() {
    // task-kind-taxonomy: the first child inherits the decomposed leaf's own
    // kind (here `research-a`) when `--kind` is not given.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(
        &grove.join("01-research-a-target-k1.md"),
        "# target-k1\n\nbody body\n",
    );
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/01-research-a-target-k1.md", "sub"],
    );
    assert!(ok, "leaf-decompose failed");
    let child = rel_line(&stdout, tmp.path(), 1);
    assert_eq!(
        child,
        PathBuf::from(".grove/01-target-k1/01-research-a-sub-k2.md")
    );
    assert!(!read(tmp.path(), child.to_str().unwrap()).contains("**Kind:**"));
}

#[test]
fn decompose_kind_flag_overrides_the_parent_leafs_kind() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(
        &grove.join("01-research-b-target-k1.md"),
        "# target-k1\n\nbody body\n",
    );
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &[
            "leaf-decompose",
            ".grove/01-research-b-target-k1.md",
            "sub",
            "--kind",
            "review-impl",
        ],
    );
    assert!(ok, "leaf-decompose failed");
    let child = rel_line(&stdout, tmp.path(), 1);
    assert_eq!(
        child,
        PathBuf::from(".grove/01-target-k1/01-review-impl-sub-k2.md")
    );
    assert!(!read(tmp.path(), child.to_str().unwrap()).contains("**Kind:**"));
}

#[test]
fn decompose_does_not_copy_legacy_body_routing_to_the_first_child() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(
        &grove.join("01-research-a-target-k1.md"),
        "# target-k1\n\n**Kind:** research-b\n**Harness:** codex\n\nbody body\n",
    );
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/01-research-a-target-k1.md", "sub"],
    );
    assert!(ok, "leaf-decompose failed");
    let child = rel_line(&stdout, tmp.path(), 1);
    let body = read(tmp.path(), child.to_str().unwrap());
    assert!(!body.contains("**Harness:**"), "got {body:?}");
    assert!(!body.contains("**Kind:**"), "got {body:?}");
}

#[test]
fn decompose_of_an_undeclared_leaf_writes_no_harness_line() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(
        &grove.join("01-impl-target-k1.md"),
        "# target-k1\n\nbody body\n",
    );
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/01-impl-target-k1.md", "sub"],
    );
    assert!(ok, "leaf-decompose failed");
    let child = rel_line(&stdout, tmp.path(), 1);
    let body = read(tmp.path(), child.to_str().unwrap());
    assert!(!body.contains("Harness"), "got {body:?}");
}

#[test]
fn decompose_ignores_an_unknown_legacy_body_harness() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(
        &grove.join("01-research-b-target-k1.md"),
        "# target-k1\n\n**Harness:** codx\n",
    );
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/01-research-b-target-k1.md", "sub"],
    );
    assert!(
        ok,
        "body routing must not affect current decompose: {stderr}"
    );
    assert_eq!(
        rel_line(&stdout, tmp.path(), 1),
        PathBuf::from(".grove/01-target-k1/01-research-b-sub-k2.md")
    );
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
    touch(&grove.join("01-DONE-impl-old-k1.md"), "# old-k1\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-decompose", ".grove/01-DONE-impl-old-k1.md", "x"],
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
    touch(&grove.join("01-impl-target-k1.md"), "# target-k1\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-retire", ".grove/01-impl-target-k1.md"]);
    assert!(ok, "leaf-retire failed");
    assert_eq!(
        rel_line(&stdout, tmp.path(), 0),
        PathBuf::from(".grove/01-DONE-impl-target-k1.md")
    );
    assert!(exists(tmp.path(), ".grove/01-DONE-impl-target-k1.md"));
    assert!(!exists(tmp.path(), ".grove/01-impl-target-k1.md"));
    // The DONE infix is filename-only — the body is byte-identical.
    assert_eq!(
        read(tmp.path(), ".grove/01-DONE-impl-target-k1.md"),
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
    touch(&grove.join("01-DONE-impl-old-k1.md"), "# old-k1\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-retire", ".grove/01-DONE-impl-old-k1.md"],
    );
    assert!(!ok, "retire must refuse an already-retired leaf");
    assert!(
        stderr.contains("already retired") || stderr.contains("DONE"),
        "expected already-retired diagnostic, got {stderr:?}"
    );
}

// ---------------------------------------------------------------------------
// What Git shows between the verb and the commit — question 1
//
// `leaf-retire` and `leaf-prune` mark through `ordinal-fs-tree`'s `rewrite`, and
// the library renames with `rename(2)` and detects no repository. Grove does not
// stage afterwards (`docs/adr/grove-does-not-stage-its-own-renames.md`), so on
// the **Git** lane a tracked leaf's mark is no longer a `git mv`, and what an
// operator sees before the commit changed. That is the whole of what question 1
// is about, and this working tree is Jujutsu — so it is asserted here against a
// real Git repository rather than observed.

/// `git status --porcelain` over `.grove`, sorted. Index and worktree columns
/// included, because the whole question is which of the two moved.
fn git_status(repo: &Path) -> Vec<String> {
    let out = Pcmd::new("git")
        .arg("-C")
        .arg(repo)
        .args(["status", "--porcelain", "--", ".grove"])
        .output()
        .unwrap();
    let mut lines: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_string)
        .collect();
    lines.sort();
    lines
}

/// `git diff --name-status -M` between the last two commits, over `.grove`.
fn git_last_commit_names(repo: &Path) -> Vec<String> {
    let out = Pcmd::new("git")
        .arg("-C")
        .arg(repo)
        .args([
            "diff",
            "--name-status",
            "-M",
            "HEAD~1",
            "HEAD",
            "--",
            ".grove",
        ])
        .output()
        .unwrap();
    let mut lines: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_string)
        .collect();
    lines.sort();
    lines
}

#[test]
fn retiring_a_tracked_leaf_shows_a_deletion_and_an_untracked_file_not_a_rename() {
    // The observable question 1 is about, stated as an assertion. Before the
    // flip this was a staged `R  old -> new` and nothing else; now Git's index
    // still holds the old path and the new one is untracked.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-impl-target-k1.md"), "# target-k1\n");
    stage_all(tmp.path());
    assert!(
        git_status(tmp.path()).is_empty(),
        "the fixture must start clean, or the assertion below proves nothing"
    );

    let (_, _, ok) = run(tmp.path(), &["leaf-retire", ".grove/01-impl-target-k1.md"]);
    assert!(ok, "leaf-retire failed");

    assert_eq!(
        git_status(tmp.path()),
        vec![
            " D .grove/01-impl-target-k1.md".to_string(),
            "?? .grove/01-DONE-impl-target-k1.md".to_string(),
        ],
        "an unstaged deletion plus an untracked file — not `R  old -> new`"
    );
}

#[test]
fn a_commit_that_stages_the_whole_tree_still_records_the_retire_as_a_rename() {
    // The other half, and the reason accepting the changed status costs nothing
    // at the commit: both lanes commit byte-identical trees, and Git infers
    // renames at diff time by content similarity.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-impl-target-k1.md"), "# target-k1\n");
    stage_all(tmp.path());

    let (_, _, ok) = run(tmp.path(), &["leaf-retire", ".grove/01-impl-target-k1.md"]);
    assert!(ok, "leaf-retire failed");
    git(tmp.path(), &["add", "-A"]);
    git(tmp.path(), &["commit", "-m", "retire target-k1"]);

    assert_eq!(
        git_last_commit_names(tmp.path()),
        vec!["R100\t.grove/01-impl-target-k1.md\t.grove/01-DONE-impl-target-k1.md".to_string(),],
        "staging the tree records one rename"
    );
    assert!(git_status(tmp.path()).is_empty(), "and leaves it clean");
}

#[test]
fn a_commit_that_stages_only_tracked_paths_records_the_retire_as_a_deletion() {
    // The hazard, asserted rather than warned about. `git commit -a` stages
    // modifications and deletions of *tracked* files and never an untracked one,
    // so it records the live name's disappearance and not the DONE name's
    // arrival — the whole of what `docs/adr/grove-does-not-stage-its-own-renames.md`
    // asks a session to avoid, and the reason `references/commit.md` now says to
    // stage the tree. Nothing is lost from the working copy; the commit is wrong,
    // not the tree.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-impl-target-k1.md"), "# target-k1\n");
    stage_all(tmp.path());

    let (_, _, ok) = run(tmp.path(), &["leaf-retire", ".grove/01-impl-target-k1.md"]);
    assert!(ok, "leaf-retire failed");
    git(tmp.path(), &["commit", "-a", "-m", "retire target-k1"]);

    assert_eq!(
        git_last_commit_names(tmp.path()),
        vec!["D\t.grove/01-impl-target-k1.md".to_string()],
        "the deletion alone — the retired leaf never reached history"
    );
    assert_eq!(
        git_status(tmp.path()),
        vec!["?? .grove/01-DONE-impl-target-k1.md".to_string()],
        "and the retired leaf is still sitting there untracked"
    );
}

#[test]
fn pruning_a_node_leaves_every_mark_unstaged_the_same_way() {
    // The bulk case reaches Git exactly as the single one does: N plain renames,
    // none of them staged. Asserted because `leaf-prune` is the verb whose
    // marking loop changed shape, and a per-mark staging step slipping back in
    // would be invisible in the single-leaf test.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    let node = mknode(&grove, "01-build-k1", "build-k1");
    touch(&node.join("01-impl-a-k2.md"), "# a-k2\n");
    touch(&node.join("02-impl-b-k3.md"), "# b-k3\n");
    stage_all(tmp.path());

    let (_, _, ok) = run(tmp.path(), &["leaf-prune", ".grove/01-build-k1"]);
    assert!(ok, "leaf-prune failed");

    assert_eq!(
        git_status(tmp.path()),
        vec![
            " D .grove/01-build-k1/01-impl-a-k2.md".to_string(),
            " D .grove/01-build-k1/02-impl-b-k3.md".to_string(),
            "?? .grove/01-build-k1/01-ABANDONED-impl-a-k2.md".to_string(),
            "?? .grove/01-build-k1/02-ABANDONED-impl-b-k3.md".to_string(),
        ],
    );
}

// ---------------------------------------------------------------------------
// The next-steps reminder
//
// `leaf-retire` and `leaf-prune` are the terminal-marking pair and the last
// grove verbs a session runs, so each names the two steps that follow — commit,
// then `grove-llm complete` — on **stderr**, at the moment of decision. stdout
// stays data: callers parse the printed paths.

/// Both halves of the reminder, in order, and nothing on stdout but paths.
fn assert_next_steps(verb: &str, stdout: &str, stderr: &str, renames: &str) {
    let commit = stderr
        .find("commit this session's work")
        .unwrap_or_else(|| panic!("{verb}: no commit step on stderr: {stderr:?}"));
    let signal = stderr
        .find("`grove-llm complete`")
        .unwrap_or_else(|| panic!("{verb}: no completion step on stderr: {stderr:?}"));
    assert!(
        commit < signal,
        "{verb}: the two steps must be named in order: {stderr:?}"
    );
    assert!(
        stderr.contains("last action"),
        "{verb}: `complete` must be named as the last action: {stderr:?}"
    );
    assert!(
        stderr.contains(renames),
        "{verb}: expected {renames:?} in the reminder: {stderr:?}"
    );
    assert!(
        !stdout.contains("complete"),
        "{verb}: the reminder must not reach stdout (it is parsed): {stdout:?}"
    );
}

#[test]
fn retire_names_the_remaining_steps_on_stderr() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-impl-target-k1.md"), "# target-k1\n");
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-retire", ".grove/01-impl-target-k1.md"]);
    assert!(ok, "leaf-retire failed: {stderr}");
    assert_next_steps("leaf-retire", &stdout, &stderr, "this rename");
    // stdout is still exactly the one destination path callers parse.
    assert_eq!(
        stdout.lines().count(),
        1,
        "stdout must stay one path: {stdout:?}"
    );
}

#[test]
fn prune_of_one_leaf_names_the_remaining_steps_on_stderr() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-impl-target-k1.md"), "# target-k1\n");
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-prune", ".grove/01-impl-target-k1.md"]);
    assert!(ok, "leaf-prune failed: {stderr}");
    assert_next_steps("leaf-prune", &stdout, &stderr, "this rename");
}

#[test]
fn prune_of_a_node_reminds_once_for_the_whole_bulk_mark() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    let node = mknode(&grove, "01-node-k1", "node-k1");
    touch(&node.join("01-impl-alpha-k2.md"), "# alpha-k2\n");
    touch(&node.join("02-impl-beta-k3.md"), "# beta-k3\n");
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-prune", ".grove/01-node-k1"]);
    assert!(ok, "leaf-prune failed: {stderr}");
    assert_next_steps("leaf-prune", &stdout, &stderr, "these renames");
    assert_eq!(
        stderr.matches("two steps remain").count(),
        1,
        "one bulk mark ends one session, so it earns one reminder: {stderr:?}"
    );
}

#[test]
fn prune_that_marks_nothing_stays_quiet() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    let node = mknode(&grove, "01-node-k1", "node-k1");
    touch(&node.join("01-DONE-impl-alpha-k2.md"), "# alpha-k2\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-prune", ".grove/01-node-k1"]);
    assert!(ok, "leaf-prune failed: {stderr}");
    assert!(
        stderr.contains("nothing live to mark"),
        "expected the no-op advisory: {stderr:?}"
    );
    assert!(
        !stderr.contains("grove-llm complete"),
        "a prune that ended no work must not tell the session to close: {stderr:?}"
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
