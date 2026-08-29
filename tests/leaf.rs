// Fixture-driven tests for `grove-llm leaf-add` and `grove-llm leaf-insert` on
// the current witnessed directory scheme (task-tree-scheme). A parent/target is addressed by its
// permanent **key** (`[n]` / `n` / `<slug>-k<key>`) or its **path** — not a
// dotted id:
//
//   - `leaf-add <parent> <slug>`  appends a child under the node directory at the
//     next gapless per-level position (root parent `.`) with a fresh key.
//   - `leaf-insert <target> <slug>` inserts at the slot the existing target holds,
//     shifting the target and later siblings up one position. Because the
//     hierarchy lives in directories, a shift is one plain rename of each sibling
//     directory + the subtree riding along, and in-file headers are position-free
//     (`# <slug>-k<key>`), so the renumber rewrites **zero file contents**.
//     (Appending past the last sibling is `leaf-add`'s job — target must exist.)
//
// Each test stands up a real git repo, and since `growing-k33` that repo is the
// **instrument** rather than a prerequisite: both verbs run through
// `ordinal-fs-tree`, which renames with `rename(2)` and stages nothing
// (`docs/adr/grove-does-not-stage-its-own-renames.md`), so nothing here needs
// tracked files to operate on. What the repo buys is that the fixtures are the
// ones a real session produces. The Git-lane consequence — a deletion at the
// old name beside an untracked file at the new one — is asserted where the
// index is readable, in `src/task_grow/tests.rs`.
//
// - stdout: the new leaf's absolute path, single line.
// - stderr: renumber summary and cross-reference candidates (for insert).

mod support;

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    support::init_jj_repo(tmp.path());
    fs::write(tmp.path().join("README"), b"r\n").unwrap();
    support::jj(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

/// Write a leaf/brief file (creating parents).
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

/// Commit the fixture tree, putting the entries in the working-copy commit's
/// parent — the state a real session's tree is in, and the one in which a
/// rename that recorded anything of its own would be visible.
fn stage_all(repo: &Path) {
    support::jj(repo, &["commit", "-m", "fixture"]);
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

fn rel_path(stdout: &str, repo: &Path) -> PathBuf {
    let line = stdout
        .lines()
        .next()
        .expect("expected new leaf path on stdout");
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
// leaf-add

#[test]
fn add_to_empty_root_uses_position_one() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", ".", "first-step"]);
    assert!(ok, "leaf-add failed");
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/01-impl-first-step-k1.md")
    );
    let body = read(tmp.path(), ".grove/01-impl-first-step-k1.md");
    // The in-file header is the position-free handle `# <slug>-k<key>`.
    assert!(body.starts_with("# first-step-k1\n"), "header: {body:?}");
    assert!(!body.contains("**Kind:**"));
    assert!(body.contains("## Goal\n"));
}

#[test]
fn add_to_nonempty_root_uses_next_position_and_fresh_key() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    touch(&grove.join("01-impl-existing-k1.md"), "# existing-k1\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", ".", "second"]);
    assert!(ok);
    // Next root child is position 02; fresh key is max key (1) + 1 = 2.
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/02-impl-second-k2.md")
    );
}

#[test]
fn add_under_a_node_by_key_uses_child_position() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    let node = mknode(&grove, "01-node-k1", "node-k1");
    touch(&node.join("01-impl-child-k2.md"), "# child-k2\n");
    stage_all(tmp.path());

    // Parent referenced by its permanent key `1`.
    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", "1", "second"]);
    assert!(ok);
    // Next child under the node is position 02; fresh key is max (2) + 1 = 3.
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/01-node-k1/02-impl-second-k3.md")
    );
}

#[test]
fn add_under_a_node_by_path() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    mknode(&grove, "01-node-k1", "node-k1");
    stage_all(tmp.path());

    // Parent referenced by its grove-root-relative directory path.
    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", "01-node-k1", "only"]);
    assert!(ok, "leaf-add by path failed");
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/01-node-k1/01-impl-only-k2.md")
    );
}

#[test]
fn add_with_planning_kind_writes_planning_in_filename() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(
        tmp.path(),
        &["leaf-add", ".", "plan-it", "--kind", "planning"],
    );
    assert!(ok);
    let path = rel_path(&stdout, tmp.path());
    assert_eq!(path, PathBuf::from(".grove/01-planning-plan-it-k1.md"));
    let body = read(tmp.path(), path.to_str().unwrap());
    assert!(!body.contains("**Kind:**"), "got {body:?}");
}

#[test]
fn add_accepts_every_non_reserved_kind() {
    // task-kind-taxonomy: the whole parameterised set is writable, including the
    // hyphenated `review-*` and `integrate-review-*` steps — the labels most
    // likely to be mangled between the CLI, the enum, and the leaf template.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    for (i, label) in support::KIND_LABELS
        .iter()
        .filter(|label| **label != "finish")
        .enumerate()
    {
        let slug = format!("x{i}");
        let (stdout, stderr, ok) = run(tmp.path(), &["leaf-add", ".", &slug, "--kind", label]);
        assert!(ok, "--kind {label} rejected: {stderr:?}");
        let path = rel_path(&stdout, tmp.path());
        assert!(
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .contains(&format!("-{label}-{slug}-")),
            "got {path:?}"
        );
        assert!(!read(tmp.path(), path.to_str().unwrap()).contains("**Kind:**"));
    }
}

#[test]
fn add_rejects_an_unrecognised_kind_listing_every_kind() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-add", ".", "x", "--kind", "reserch"]);
    assert!(!ok, "an unrecognised --kind must be rejected at write time");
    for label in support::KIND_LABELS {
        assert!(
            stderr.contains(label),
            "error must list {label:?}, got {stderr:?}"
        );
    }
}

#[test]
fn add_refuses_the_retired_work_kind_and_names_its_replacement() {
    // The write half of the `work` → `impl` rename (task-kind-taxonomy). Write
    // gates where read aliases: a human is present at authoring time, and the
    // error has to retrain rather than merely reject — "not in the list" is
    // useless advice for a word that was correct last week.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-add", ".", "x", "--kind", "work"]);
    assert!(!ok, "`--kind work` must be refused on write");
    assert!(
        stderr.contains("impl"),
        "the error must name the replacement, got {stderr:?}"
    );
    assert!(
        !exists(tmp.path(), ".grove/01-impl-x-k1.md"),
        "a refused --kind must not leave a leaf behind"
    );
}

#[test]
fn add_defaults_to_impl_when_no_kind_is_given() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", ".", "x"]);
    assert!(ok);
    let path = rel_path(&stdout, tmp.path());
    assert_eq!(path, PathBuf::from(".grove/01-impl-x-k1.md"));
    assert!(!read(tmp.path(), path.to_str().unwrap()).contains("**Kind:**"));
}

// ── Removed per-leaf routing declarations ──────────────────────────────────

#[test]
fn add_rejects_the_removed_harness_flag_without_writing() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &[
            "leaf-add",
            ".",
            "survey",
            "--kind",
            "research-a",
            "--harness",
            "codex",
        ],
    );
    assert!(!ok, "the removed --harness flag was accepted");
    assert!(
        stderr.contains("unexpected argument '--harness'"),
        "got {stderr:?}"
    );
    assert!(stdout.is_empty());
    assert!(!exists(tmp.path(), ".grove/01-research-a-survey-k1.md"));
}

#[test]
fn add_without_a_harness_writes_no_harness_line_at_all() {
    // The common case, and a hard requirement rather than a nicety: an empty
    // `**Harness:**` line is a *refusal* on the read side, so a template that
    // always emitted one would make every leaf grove creates unlaunchable.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", ".", "x"]);
    assert!(ok);
    let body = read(tmp.path(), rel_path(&stdout, tmp.path()).to_str().unwrap());
    assert!(!body.contains("Harness"), "got {body:?}");
}

#[test]
fn add_help_exposes_no_harness_flag() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (stdout, _, ok) = run(tmp.path(), &["leaf-add", "--help"]);
    assert!(ok);
    assert!(!stdout.contains("--harness"), "got {stdout:?}");
}

#[test]
fn insert_rejects_the_removed_harness_flag_without_writing() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    touch(&grove.join("01-impl-a-k1.md"), "# a-k1\n");
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &[
            "leaf-insert",
            "01-impl-a-k1.md",
            "survey",
            "--kind",
            "research-a",
            "--harness",
            "pi",
        ],
    );
    assert!(!ok, "the removed --harness flag was accepted");
    assert!(
        stderr.contains("unexpected argument '--harness'"),
        "got {stderr:?}"
    );
    assert!(stdout.is_empty());
    assert!(exists(tmp.path(), ".grove/01-impl-a-k1.md"));
}

#[test]
fn add_rejects_invalid_slug() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-add", ".", "Bad Slug"]);
    assert!(!ok, "expected invalid-slug rejection");
    assert!(
        stderr.contains("slug") || stderr.contains("lowercase"),
        "expected slug diagnostic, got {stderr:?}"
    );
}

#[test]
fn add_under_nonexistent_parent_errors() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"), "# demo — brief\n");
    stage_all(tmp.path());

    // Key 9 resolves to nothing (not a path, not a key) → the ref-or-path mapping
    // reports it could not be resolved.
    let (_, stderr, ok) = run(tmp.path(), &["leaf-add", "9", "orphan"]);
    assert!(!ok, "expected error adding under a nonexistent parent");
    assert!(
        stderr.contains("no entry matches") && stderr.contains('9'),
        "expected not-found diagnostic, got {stderr:?}"
    );
}

// ---------------------------------------------------------------------------
// leaf-insert

#[test]
fn insert_at_start_shifts_root_siblings_up_by_one() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-impl-a-k1.md"), "# a-k1\n");
    touch(&grove.join("02-design-b-k2.md"), "# b-k2\n");
    stage_all(tmp.path());

    // Target the entry holding key 1 (leaf `a`).
    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-insert", "1", "fresh"]);
    assert!(ok, "leaf-insert failed: {stderr}");
    // New leaf lands at position 01 with a fresh key (max 2 + 1 = 3).
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/01-impl-fresh-k3.md")
    );
    // The occupant and its later sibling each shift up by one; keys preserved.
    assert!(
        exists(tmp.path(), ".grove/02-impl-a-k1.md"),
        "a not shifted to 02"
    );
    assert!(
        exists(tmp.path(), ".grove/03-design-b-k2.md"),
        "b not shifted to 03"
    );
    assert!(
        !exists(tmp.path(), ".grove/01-impl-a-k1.md"),
        "old a still present"
    );
    // Renumber summary goes to stderr (stdout stays just the new path).
    assert!(
        stderr.contains("renumber"),
        "expected renumber summary on stderr, got {stderr:?}"
    );
}

#[test]
fn insert_cascades_a_node_subtree_with_position_free_headers() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    let node = mknode(&grove, "01-node-k1", "node-k1");
    touch(&node.join("01-prototype-inner-k2.md"), "# inner-k2\n");
    touch(&grove.join("02-design-outer-k3.md"), "# outer-k3\n");
    stage_all(tmp.path());

    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-insert", "1", "fresh"]);
    assert!(ok, "leaf-insert failed: {stderr}");
    assert_eq!(
        rel_path(&stdout, tmp.path()),
        PathBuf::from(".grove/01-impl-fresh-k4.md")
    );
    // node 01 → 02 drags its whole subtree (the child rides along, name and key
    // unchanged); the unrelated sibling 02 → 03.
    assert!(
        exists(tmp.path(), ".grove/02-node-k1/01-prototype-inner-k2.md"),
        "node dir + child not moved as a unit"
    );
    assert!(
        exists(tmp.path(), ".grove/03-design-outer-k3.md"),
        "outer not bumped"
    );
    assert!(
        !exists(tmp.path(), ".grove/01-node-k1"),
        "old node dir still present"
    );
    // The renumber rewrites ZERO file contents — the dragged brief's position-free
    // header is byte-identical (v2's "cascade collapse", task-tree-scheme).
    assert_eq!(
        read(tmp.path(), ".grove/02-node-k1/BRIEF.md"),
        "# node-k1 — brief\n",
        "position-free header must not be rewritten on renumber"
    );
}

#[test]
fn insert_requires_an_existing_target() {
    // In v2, inserting past the last sibling is `leaf-add`'s job — `leaf-insert`'s
    // target must exist. A nonexistent target is an error (not a silent append).
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("01-impl-a-k1.md"), "# a-k1\n");
    stage_all(tmp.path());

    let (_, stderr, ok) = run(tmp.path(), &["leaf-insert", "9", "tail"]);
    assert!(!ok, "leaf-insert at a nonexistent target should error");
    assert!(
        stderr.contains("no entry matches"),
        "expected not-found diagnostic, got {stderr:?}"
    );
    // The pre-existing leaf is untouched.
    assert!(exists(tmp.path(), ".grove/01-impl-a-k1.md"));
}

// ---------------------------------------------------------------------------
// CLI surface

#[test]
fn leaf_add_and_insert_listed_in_grove_llm_help() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .env("HOME", support::fixture_home())
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("leaf-add"), "missing leaf-add: {s}");
    assert!(s.contains("leaf-insert"), "missing leaf-insert: {s}");
}

#[test]
fn grove_binary_does_not_expose_leaf_verbs() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(!s.contains("leaf-add"), "grove --help leaked leaf-add: {s}");
    assert!(
        !s.contains("leaf-insert"),
        "grove --help leaked leaf-insert: {s}"
    );
}
