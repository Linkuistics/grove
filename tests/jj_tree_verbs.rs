// Fixture-driven tests for the **jj path** through every tree-mutation verb.
//
// grove is jj-first (`src/repo.rs`, symmetric-vcs-rule): a `.jj/`
// directory heading the working tree picks jj plumbing even when a `.git` sits
// beside it. One seam carries that decision now — `repo::vcs_of`, which working
// tree a verb resolves (covered by `tests/repo.rs`). The second used to be
// `tree_rename::rename_entry`, and since `promotion-k34` it has **no production
// caller left**: every entry that moves, moves inside an `ordinal-fs-tree`
// operation, which renames with `rename(2)` and consults no VCS at all. The
// module survives until `sweep-k37` deletes it. What neither seam
// can show is that **every verb actually routes through them**: a verb carrying
// its own git-only side path — a stray `git mv`, a git-first root resolution —
// would pass both unit suites and still fail in a jj tree. So these drive the
// real binaries end to end, once per verb, against two fixtures:
//
//   * **jj-native** — a `.jj/` with no `.git/` anywhere, so a git fallback fails
//     outright instead of quietly working. This is the fixture that proves a
//     verb needs no git at all.
//   * **colocated** — `.jj/` beside a `.git/` whose index already holds the
//     tree, where jj-first is a *choice* rather than the only option: the rename
//     must be plain and git's index must come out untouched, because a `git mv`
//     would stage into an index jj ignores (jj snapshots the working copy). For
//     the verbs the flip has moved onto `ordinal-fs-tree` it is no longer a
//     choice — the library renames plainly everywhere — and the colocated case
//     is then a guard against a verb growing a `git mv` of its own rather than a
//     test of the dispatch. See the section header below.
//
// `root-init` and `leaf-add` get no colocated twin deliberately: they only write
// new files and consult no VCS beyond resolving the worktree, so a colocated
// case would assert nothing its jj-native case does not. That was already true
// of `leaf-add` when it allocated through `tree_grow`, and it stayed true when
// `growing-k33` moved it onto `append`.
//
// The two lifecycle-transition cases are the exception to "drive the binaries":
// they call the library, because the transition is not a verb anyone types — the
// driver runs it before it selects anything. The claim is the same one, made
// about the one mutation with no command-line spelling.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as Pcmd;
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Fixtures

/// A jj-native working tree: `.jj/` and no `.git/` anywhere. `git.colocate=false`
/// is forced because the ambient jj config may default colocation on, which would
/// silently turn this fixture into the colocated one and hide a git fallback.
fn jj_native() -> TempDir {
    let tmp = TempDir::new().unwrap();
    run_jj(
        tmp.path(),
        &[
            "--config",
            "git.colocate=false",
            "git",
            "init",
            "--quiet",
            ".",
        ],
    );
    assert!(
        !tmp.path().join(".git").exists(),
        "jj_native fixture must have no .git/ — a git fallback would go unnoticed"
    );
    tmp
}

/// A colocated working tree whose `.grove/` is **committed to git first**, so
/// git's index holds every entry under its pre-rename name. That ordering is what
/// makes the index assertions falsifiable: with the entries untracked, a git-first
/// implementation would take the plain-rename branch too and the test would pass
/// for the wrong reason.
fn colocated(build: impl FnOnce(&Path)) -> TempDir {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    run("git", repo, &["init", "-q", "."]);
    run("git", repo, &["config", "user.email", "t@example.com"]);
    run("git", repo, &["config", "user.name", "Grove Test"]);
    run("git", repo, &["config", "core.hooksPath", "/dev/null"]);
    build(repo);
    run("git", repo, &["add", "-A"]);
    run("git", repo, &["commit", "-q", "-m", "fixture"]);
    run_jj(repo, &["git", "init", "--colocate", "--quiet", "."]);
    tmp
}

// ---------------------------------------------------------------------------
// Process helpers

fn run(bin: &str, dir: &Path, args: &[&str]) -> String {
    let out = Pcmd::new(bin)
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("running {bin} {args:?}: {e} (is {bin} installed?)"));
    assert!(
        out.status.success(),
        "{bin} {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Run jj with a test-local identity, so no global config is required.
fn run_jj(dir: &Path, args: &[&str]) -> String {
    let mut full = vec![
        "--config",
        "user.name=Test",
        "--config",
        "user.email=t@example.com",
    ];
    full.extend_from_slice(args);
    run("jj", dir, &full)
}

/// Drive the real `grove-llm` binary from the worktree root, as a session does.
fn llm(repo: &Path, args: &[&str]) -> (String, String, bool) {
    bin("grove-llm", repo, args)
}

fn bin(name: &str, repo: &Path, args: &[&str]) -> (String, String, bool) {
    let out = Command::cargo_bin(name)
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

// ---------------------------------------------------------------------------
// Filesystem / VCS inspection

/// Write a file, creating parent directories.
fn touch(p: &Path, body: &str) {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, body.as_bytes()).unwrap();
}

fn exists(repo: &Path, rel: &str) -> bool {
    repo.join(rel).exists()
}

fn read(repo: &Path, rel: &str) -> String {
    fs::read_to_string(repo.join(rel)).unwrap()
}

/// Every regular file under `.grove/`, as grove-root-relative slash paths, sorted.
fn tree(repo: &Path) -> Vec<String> {
    fn walk(dir: &Path, base: &Path, out: &mut Vec<String>) {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let ft = entry.file_type().unwrap();
            if ft.is_dir() {
                walk(&entry.path(), base, out);
            } else if ft.is_file() {
                out.push(
                    entry
                        .path()
                        .strip_prefix(base)
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }
    let mut out = Vec::new();
    let grove = repo.join(".grove");
    walk(&grove, &grove, &mut out);
    out.sort();
    out
}

/// The paths git holds **in its index** under `.grove/`, sorted. Distinct from
/// [`tree`] (what is on disk): a `git mv` moves the index entry and shows here
/// under the new name, while a plain rename leaves the old entry standing.
fn git_index(repo: &Path) -> Vec<String> {
    let mut v: Vec<String> = run("git", repo, &["ls-files", "--", ".grove"])
        .lines()
        .map(|s| s.to_string())
        .collect();
    v.sort();
    v
}

/// The first stdout line as a worktree-relative path — every tree verb prints
/// absolute paths, and the fixture root may be a symlink (`/var` → `/private/var`
/// on macOS), so both sides are canonicalized before stripping.
fn rel_line(stdout: &str, repo: &Path, n: usize) -> PathBuf {
    let line = stdout
        .lines()
        .nth(n)
        .expect("expected a path line on stdout");
    PathBuf::from(line)
        .canonicalize()
        .unwrap()
        .strip_prefix(repo.canonicalize().unwrap())
        .unwrap()
        .to_path_buf()
}

/// A `.grove/` holding a root brief, a node directory with two live children, and
/// two root-level leaves — enough shape for insert, decompose, retire and prune.
fn build_grove(repo: &Path) {
    let g = repo.join(".grove");
    touch(&g.join("BRIEF.md"), "# proj — brief\n\n## Goal\n");
    touch(&g.join("FORMAT"), "session-kinds-v1\n");
    touch(&g.join("01-impl-alpha-k1.md"), "# alpha-k1\n\nalpha body\n");
    touch(
        &g.join("02-node-k2/BRIEF.md"),
        "# node-k2 — brief\n\n## Goal\n",
    );
    touch(
        &g.join("02-node-k2/01-impl-child-k3.md"),
        "# child-k3\n\nchild body\n",
    );
    touch(
        &g.join("02-node-k2/02-research-a-sibling-k4.md"),
        "# sibling-k4\n\nsibling body\n",
    );
    touch(
        &g.join("03-design-omega-k5.md"),
        "# omega-k5\n\nomega body\n",
    );
}

// ---------------------------------------------------------------------------
// jj-native: every tree-mutation verb, with no git available to fall back on

#[test]
fn root_init_scaffolds_a_grove_in_a_jj_native_tree() {
    // The one verb that runs *below* the others' floor, so it is also the one
    // that proves worktree resolution reaches a jj tree at all: `grove_paths`
    // resolves the workspace root from `.jj/` with no `git rev-parse` available.
    let tmp = jj_native();
    let repo = tmp.path();

    let (stdout, stderr, ok) = llm(repo, &["root-init"]);
    assert!(ok, "root-init failed: {stderr}");

    assert_eq!(rel_line(&stdout, repo, 0), PathBuf::from(".grove/BRIEF.md"));
    assert_eq!(
        rel_line(&stdout, repo, 1),
        PathBuf::from(".grove/01-requirements-plan-k1.md")
    );
    assert_eq!(rel_line(&stdout, repo, 2), PathBuf::from(".grove/FORMAT"));
    assert_eq!(read(repo, ".grove/FORMAT"), "session-kinds-v1\n");
    assert!(!read(repo, ".grove/01-requirements-plan-k1.md").contains("**Kind:**"));
    assert!(
        !exists(repo, ".git"),
        "root-init must not conjure a git repo in a jj tree"
    );
}

#[test]
fn leaf_add_appends_a_child_in_a_jj_native_tree() {
    let tmp = jj_native();
    let repo = tmp.path();
    build_grove(repo);

    let (stdout, stderr, ok) = llm(repo, &["leaf-add", ".", "next"]);
    assert!(ok, "leaf-add failed: {stderr}");

    assert_eq!(
        rel_line(&stdout, repo, 0),
        PathBuf::from(".grove/04-impl-next-k6.md"),
        "appended at the next free position with max-key + 1"
    );
    assert!(exists(repo, ".grove/04-impl-next-k6.md"));
}

#[test]
fn leaf_insert_renumbers_siblings_in_a_jj_native_tree() {
    // The rename-heaviest verb: it shifts every later sibling, including a node
    // *directory* whose whole subtree rides along. In a jj tree each shift is a
    // plain `fs::rename` — `git mv` is not merely unstaged here, it is unavailable.
    let tmp = jj_native();
    let repo = tmp.path();
    build_grove(repo);

    let (stdout, stderr, ok) = llm(repo, &["leaf-insert", "[1]", "urgent"]);
    assert!(ok, "leaf-insert failed: {stderr}");

    assert_eq!(
        rel_line(&stdout, repo, 0),
        PathBuf::from(".grove/01-impl-urgent-k6.md")
    );
    assert_eq!(
        tree(repo),
        vec![
            "01-impl-urgent-k6.md",
            "02-impl-alpha-k1.md",
            "03-node-k2/01-impl-child-k3.md",
            "03-node-k2/02-research-a-sibling-k4.md",
            "03-node-k2/BRIEF.md",
            "04-design-omega-k5.md",
            "BRIEF.md",
            "FORMAT",
        ],
        "every sibling shifted up one, the node's subtree riding along untouched"
    );
    assert!(
        stderr.contains("renumbered 3 siblings"),
        "expected the renumber summary on stderr, got {stderr:?}"
    );
}

#[test]
fn leaf_decompose_promotes_a_leaf_to_a_node_in_a_jj_native_tree() {
    let tmp = jj_native();
    let repo = tmp.path();
    build_grove(repo);

    let (stdout, stderr, ok) = llm(
        repo,
        &["leaf-decompose", ".grove/01-impl-alpha-k1.md", "sub"],
    );
    assert!(ok, "leaf-decompose failed: {stderr}");

    assert_eq!(
        rel_line(&stdout, repo, 0),
        PathBuf::from(".grove/01-alpha-k1/BRIEF.md")
    );
    assert_eq!(
        rel_line(&stdout, repo, 1),
        PathBuf::from(".grove/01-alpha-k1/01-impl-sub-k6.md")
    );
    // The leaf file became the node's brief — same key, retitled header, body carried.
    assert!(!exists(repo, ".grove/01-impl-alpha-k1.md"));
    let brief = read(repo, ".grove/01-alpha-k1/BRIEF.md");
    assert!(
        brief.starts_with("# alpha-k1 — brief\n"),
        "brief not retitled: {brief:?}"
    );
    assert!(brief.contains("alpha body"), "brief body lost: {brief:?}");
}

#[test]
fn leaf_retire_marks_done_in_place_in_a_jj_native_tree() {
    let tmp = jj_native();
    let repo = tmp.path();
    build_grove(repo);

    let (stdout, stderr, ok) = llm(
        repo,
        &["leaf-retire", ".grove/02-node-k2/01-impl-child-k3.md"],
    );
    assert!(ok, "leaf-retire failed: {stderr}");

    assert_eq!(
        rel_line(&stdout, repo, 0),
        PathBuf::from(".grove/02-node-k2/01-DONE-impl-child-k3.md")
    );
    assert!(exists(repo, ".grove/02-node-k2/01-DONE-impl-child-k3.md"));
    assert!(!exists(repo, ".grove/02-node-k2/01-impl-child-k3.md"));
    // The infix is filename-only — the body is byte-identical.
    assert_eq!(
        read(repo, ".grove/02-node-k2/01-DONE-impl-child-k3.md"),
        "# child-k3\n\nchild body\n"
    );
}

#[test]
fn reviewed_producer_retirement_does_not_write_body_routing_in_a_jj_native_tree() {
    let tmp = jj_native();
    let repo = tmp.path();
    let grove = repo.join(".grove");
    touch(&grove.join("FORMAT"), "session-kinds-v1\n");
    // Flat siblings, which is what a review chain is (flat-lazy-review): the
    // review was cut by the producer's own session as an ordinary `leaf-add`.
    touch(&grove.join("01-impl-build-k1.md"), "# build-k1\n");
    touch(
        &grove.join("02-review-impl-build-k2.md"),
        "# build-k2\n\n**Reviews:** build-k1\n",
    );
    let output = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(repo)
        .args(["leaf-retire", ".grove/01-impl-build-k1.md"])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "leaf-retire failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(exists(repo, ".grove/01-DONE-impl-build-k1.md"));
    // Byte-equality rather than a `!contains` on a retired marker: the claim is
    // that retiring a producer writes nothing into the review that names it, and
    // that holds for whatever a future sibling write might say.
    assert_eq!(
        read(repo, ".grove/02-review-impl-build-k2.md"),
        "# build-k2\n\n**Reviews:** build-k1\n"
    );
    assert!(
        !exists(repo, ".git"),
        "a jj-native retirement must stage through jj, not fall back to git"
    );
}

#[test]
fn leaf_prune_marks_a_whole_subtree_abandoned_in_a_jj_native_tree() {
    // The bulk-rename case: one prune of a node marks every live leaf beneath it,
    // so a single verb call issues several renames in a jj tree.
    let tmp = jj_native();
    let repo = tmp.path();
    build_grove(repo);

    let (stdout, stderr, ok) = llm(repo, &["leaf-prune", ".grove/02-node-k2"]);
    assert!(ok, "leaf-prune failed: {stderr}");

    let marked: Vec<PathBuf> = (0..2).map(|n| rel_line(&stdout, repo, n)).collect();
    assert_eq!(
        marked,
        vec![
            PathBuf::from(".grove/02-node-k2/01-ABANDONED-impl-child-k3.md"),
            PathBuf::from(".grove/02-node-k2/02-ABANDONED-research-a-sibling-k4.md"),
        ]
    );
    assert!(exists(
        repo,
        ".grove/02-node-k2/01-ABANDONED-impl-child-k3.md"
    ));
    assert!(exists(
        repo,
        ".grove/02-node-k2/02-ABANDONED-research-a-sibling-k4.md"
    ));
    assert!(
        exists(repo, ".grove/02-node-k2/BRIEF.md"),
        "a node is never marked — its brief stays exactly where it is"
    );
}

/// Lay down a **kind-less v2** tree — the one legacy layout still migrated. Its
/// directories are already the v2 shape; what is legacy is that its leaves carry
/// no session-kind segment and its bodies still carry `**Kind:**` markers.
///
/// So the conversion this drives is a rename **inside each directory** plus a
/// body rewrite, which is the whole of what migration does now. It replaced a
/// v1-flat fixture, which in turn replaced an `NNN-slug/` one, as each of those
/// layouts was withdrawn; the property that went with them is relocation — no
/// directory is created or removed here, and `expected_migrated_tree` names the
/// same directory it started in.
fn build_kindless_v2_grove(repo: &Path) {
    let g = repo.join(".grove");
    touch(&g.join("BRIEF.md"), "# proj — brief\n");
    touch(
        &g.join("01-DONE-first-k1.md"),
        "# first-k1\n\n**Kind:** impl\n\nbody one\n",
    );
    touch(&g.join("02-second-k2/BRIEF.md"), "# second-k2 — brief\n");
    touch(
        &g.join("02-second-k2/01-child-k3.md"),
        "# child-k3\n\nchild body\n",
    );
}

/// The current-format shape `build_kindless_v2_grove` must lower to. The child
/// body carries no `**Kind:**` marker, so it takes the read-side default `impl`;
/// the retired root leaf declares `impl` and keeps it, minus the marker line.
fn expected_migrated_tree() -> Vec<String> {
    [
        "01-DONE-impl-first-k1.md",
        "02-second-k2/01-impl-child-k3.md",
        "02-second-k2/BRIEF.md",
        "BRIEF.md",
        "FORMAT",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

#[test]
fn the_lifecycle_transition_migrates_a_legacy_tree_in_a_jj_native_tree() {
    // The pre-v2 layout conversion, in a tree with no git anywhere — so a git
    // fallback in the rename seam or the commit seam fails outright rather than
    // quietly working. This is the same claim the other cases in this file make
    // for the agent verbs, made for the one mutation the *driver* performs
    // before it selects anything.
    let tmp = jj_native();
    let repo = tmp.path();
    build_kindless_v2_grove(repo);

    assert_eq!(
        grove::tree_lifecycle::transition_to_current(repo).unwrap(),
        grove::tree_lifecycle::CurrentTransition::Migrated
    );

    assert_eq!(tree(repo), expected_migrated_tree());
}

#[test]
fn jjs_working_copy_snapshots_the_renames_a_verb_made() {
    // The premise the plain rename rests on: jj needs no staging step, because
    // every jj command snapshots the working copy. Committed first so the moves
    // show as changes against a parent rather than as a tree of new files.
    let tmp = jj_native();
    let repo = tmp.path();
    build_grove(repo);
    run_jj(repo, &["commit", "-m", "fixture"]);

    let (_, stderr, ok) = llm(repo, &["leaf-retire", ".grove/01-impl-alpha-k1.md"]);
    assert!(ok, "leaf-retire failed: {stderr}");

    // jj compacts a detected rename to `R <dir>/{<old> => <new>}`, so match the
    // two names rather than a whole path. The leading `R` is the point: with no
    // staging step at all, jj's snapshot saw a *rename*, not a delete plus an add.
    let diff = run_jj(repo, &["diff", "--summary", "--no-pager"]);
    assert!(
        diff.starts_with('R')
            && diff.contains("01-impl-alpha-k1.md")
            && diff.contains("01-DONE-impl-alpha-k1.md"),
        "jj's snapshot must see the rename with no staging step, got {diff:?}"
    );
}

// ---------------------------------------------------------------------------
// Colocated: `.jj/` wins over the `.git/` beside it
//
// One test per rename-shaped verb. The property under test is the same in each —
// the rename is plain and git's index comes out untouched — but it is asserted
// per verb because what it guards against is a *verb* reaching for `git mv`
// directly, which no single test of the primitive can rule out.
//
// **All of them now hold for a second, stronger reason, and the assertions are
// unchanged.** Every rename-shaped verb runs through `ordinal-fs-tree` — the
// marks through `rewrite`, the shift through `insert`, and since `promotion-k34`
// `leaf-decompose` through `promote`, whose middle effect is the leaf's own file
// moving into the node it now sits in. The library renames with `rename(2)` and
// detects no repository at all
// (`docs/adr/grove-does-not-stage-its-own-renames.md`), so the rename is plain on
// *every* lane rather than plain because this one is jj. The tests stay exactly
// as they were: what they guard against is a verb reaching for `git mv` directly,
// and that is worth guarding whether the verb could have had a reason to or not.
// **Nothing in this file discriminates the dispatch any more**, which is the
// migrate stage arriving rather than a gap — the git-lane cases in
// `tests/leaf_ops.rs` and `src/task_grow/tests.rs` assert the same plainness on
// the lane where it used to be false, and are where the property is now
// falsifiable. Read them together.

#[test]
fn leaf_insert_in_a_colocated_tree_leaves_the_git_index_alone() {
    let tmp = colocated(build_grove);
    let repo = tmp.path();
    let before = git_index(repo);

    let (_, stderr, ok) = llm(repo, &["leaf-insert", "[1]", "urgent"]);
    assert!(ok, "leaf-insert failed: {stderr}");

    assert!(
        exists(repo, ".grove/02-impl-alpha-k1.md"),
        "the renumber must still happen on disk"
    );
    assert_eq!(
        git_index(repo),
        before,
        "jj-first: no git mv may stage a rename into an index jj ignores"
    );
}

#[test]
fn leaf_decompose_in_a_colocated_tree_leaves_the_git_index_alone() {
    let tmp = colocated(build_grove);
    let repo = tmp.path();
    let before = git_index(repo);

    let (_, stderr, ok) = llm(
        repo,
        &["leaf-decompose", ".grove/01-impl-alpha-k1.md", "sub"],
    );
    assert!(ok, "leaf-decompose failed: {stderr}");

    assert!(exists(repo, ".grove/01-alpha-k1/BRIEF.md"));
    assert_eq!(
        git_index(repo),
        before,
        "jj-first: git's index is untouched"
    );
}

#[test]
fn leaf_retire_in_a_colocated_tree_leaves_the_git_index_alone() {
    let tmp = colocated(build_grove);
    let repo = tmp.path();
    let before = git_index(repo);

    let (_, stderr, ok) = llm(repo, &["leaf-retire", ".grove/01-impl-alpha-k1.md"]);
    assert!(ok, "leaf-retire failed: {stderr}");

    assert!(exists(repo, ".grove/01-DONE-impl-alpha-k1.md"));
    assert_eq!(
        git_index(repo),
        before,
        "jj-first: git's index is untouched"
    );
}

#[test]
fn leaf_prune_in_a_colocated_tree_leaves_the_git_index_alone() {
    let tmp = colocated(build_grove);
    let repo = tmp.path();
    let before = git_index(repo);

    let (_, stderr, ok) = llm(repo, &["leaf-prune", ".grove/02-node-k2"]);
    assert!(ok, "leaf-prune failed: {stderr}");

    assert!(exists(
        repo,
        ".grove/02-node-k2/01-ABANDONED-impl-child-k3.md"
    ));
    assert_eq!(
        git_index(repo),
        before,
        "jj-first: git's index is untouched"
    );
}

#[test]
fn the_lifecycle_transition_in_a_colocated_tree_leaves_the_git_index_alone() {
    // jj-first where git is genuinely available: the conversion must move
    // entries with a plain rename, since a `git mv` would stage into an index
    // jj ignores. The transition commits as well as renames, and the *commit*
    // half of the same claim — that a colocated jj commit leaves git's index
    // where the user left it — is `tests/migration_commit.rs`'s subject; here
    // the index is the fixture's own, staged before the transition runs.
    let tmp = colocated(build_kindless_v2_grove);
    let repo = tmp.path();
    let before = git_index(repo);

    assert_eq!(
        grove::tree_lifecycle::transition_to_current(repo).unwrap(),
        grove::tree_lifecycle::CurrentTransition::Migrated
    );

    assert_eq!(tree(repo), expected_migrated_tree());
    assert_eq!(
        git_index(repo),
        before,
        "jj-first: git's index is untouched"
    );
}
