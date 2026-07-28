// Fixture-driven tests for the **jj path** through every tree-mutation verb.
//
// grove is jj-first (`src/repo.rs`, ADR *symmetric-vcs-rule*): a `.jj/`
// directory heading the working tree picks jj plumbing even when a `.git` sits
// beside it. Two seams carry that decision — `repo::vcs_of`, which working tree
// a verb resolves (covered by `tests/repo.rs`), and `tree_rename::rename_entry`,
// how an entry moves (covered in-process by `src/tree_rename.rs`). What neither
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
//     would stage into an index jj ignores (jj snapshots the working copy).
//
// `root-init` and `leaf-add` get no colocated twin deliberately: they only write
// new files and consult no VCS beyond resolving the worktree, so a colocated
// case would assert nothing its jj-native case does not.

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

/// Drive the real `grove` binary (the human verbs — `migrate` lives here).
fn grove(repo: &Path, args: &[&str]) -> (String, String, bool) {
    bin("grove", repo, args)
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
    touch(
        &g.join("01-alpha-k1.md"),
        "# alpha-k1\n\n**Kind:** impl\n\nalpha body\n",
    );
    touch(
        &g.join("02-node-k2/BRIEF.md"),
        "# node-k2 — brief\n\n## Goal\n",
    );
    touch(
        &g.join("02-node-k2/01-child-k3.md"),
        "# child-k3\n\n**Kind:** impl\n\nchild body\n",
    );
    touch(
        &g.join("02-node-k2/02-sibling-k4.md"),
        "# sibling-k4\n\n**Kind:** research\n\nsibling body\n",
    );
    touch(
        &g.join("03-omega-k5.md"),
        "# omega-k5\n\n**Kind:** design\n\nomega body\n",
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
        PathBuf::from(".grove/01-plan-k1.md")
    );
    assert!(
        read(repo, ".grove/01-plan-k1.md").contains("**Kind:** requirements"),
        "the bootstrap leaf is a requirements task (fresh-grove-start-contract)"
    );
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
        PathBuf::from(".grove/04-next-k6.md"),
        "appended at the next free position with max-key + 1"
    );
    assert!(exists(repo, ".grove/04-next-k6.md"));
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
        PathBuf::from(".grove/01-urgent-k6.md")
    );
    assert_eq!(
        tree(repo),
        vec![
            "01-urgent-k6.md",
            "02-alpha-k1.md",
            "03-node-k2/01-child-k3.md",
            "03-node-k2/02-sibling-k4.md",
            "03-node-k2/BRIEF.md",
            "04-omega-k5.md",
            "BRIEF.md",
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

    let (stdout, stderr, ok) = llm(repo, &["leaf-decompose", ".grove/01-alpha-k1.md", "sub"]);
    assert!(ok, "leaf-decompose failed: {stderr}");

    assert_eq!(
        rel_line(&stdout, repo, 0),
        PathBuf::from(".grove/01-alpha-k1/BRIEF.md")
    );
    assert_eq!(
        rel_line(&stdout, repo, 1),
        PathBuf::from(".grove/01-alpha-k1/01-sub-k6.md")
    );
    // The leaf file became the node's brief — same key, retitled header, body carried.
    assert!(!exists(repo, ".grove/01-alpha-k1.md"));
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

    let (stdout, stderr, ok) = llm(repo, &["leaf-retire", ".grove/02-node-k2/01-child-k3.md"]);
    assert!(ok, "leaf-retire failed: {stderr}");

    assert_eq!(
        rel_line(&stdout, repo, 0),
        PathBuf::from(".grove/02-node-k2/01-DONE-child-k3.md")
    );
    assert!(exists(repo, ".grove/02-node-k2/01-DONE-child-k3.md"));
    assert!(!exists(repo, ".grove/02-node-k2/01-child-k3.md"));
    // The infix is filename-only — the body is byte-identical.
    assert_eq!(
        read(repo, ".grove/02-node-k2/01-DONE-child-k3.md"),
        "# child-k3\n\n**Kind:** impl\n\nchild body\n"
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
            PathBuf::from(".grove/02-node-k2/01-ABANDONED-child-k3.md"),
            PathBuf::from(".grove/02-node-k2/02-ABANDONED-sibling-k4.md"),
        ]
    );
    assert!(exists(repo, ".grove/02-node-k2/01-ABANDONED-child-k3.md"));
    assert!(exists(repo, ".grove/02-node-k2/02-ABANDONED-sibling-k4.md"));
    assert!(
        exists(repo, ".grove/02-node-k2/BRIEF.md"),
        "a node is never marked — its brief stays exactly where it is"
    );
}

#[test]
fn migrate_converts_an_old_tree_in_a_jj_native_tree() {
    // `grove migrate` moves the whole tree at once and — unlike the adoption hook
    // — makes no commit, so this is the rename set alone, in a tree with no git.
    let tmp = jj_native();
    let repo = tmp.path();
    let g = repo.join(".grove");
    touch(&g.join("BRIEF.md"), "# proj — brief\n");
    touch(&g.join("done/010-first.md"), "# 010-first\n\nbody one\n");
    touch(&g.join("020-second/BRIEF.md"), "# 020-second — brief\n");
    touch(
        &g.join("020-second/010-child.md"),
        "# 010-child\n\nchild body\n",
    );

    let (stdout, stderr, ok) = grove(repo, &["migrate"]);
    assert!(ok, "grove migrate failed: {stderr}");

    assert_eq!(
        tree(repo),
        vec![
            "01-DONE-first-k1.md",
            "02-second-k2/01-child-k3.md",
            "02-second-k2/BRIEF.md",
            "BRIEF.md",
        ],
    );
    assert!(
        stdout.contains("migrated"),
        "expected a rename summary, got {stdout:?}"
    );
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

    let (_, stderr, ok) = llm(repo, &["leaf-retire", ".grove/01-alpha-k1.md"]);
    assert!(ok, "leaf-retire failed: {stderr}");

    // jj compacts a detected rename to `R <dir>/{<old> => <new>}`, so match the
    // two names rather than a whole path. The leading `R` is the point: with no
    // staging step at all, jj's snapshot saw a *rename*, not a delete plus an add.
    let diff = run_jj(repo, &["diff", "--summary", "--no-pager"]);
    assert!(
        diff.starts_with('R')
            && diff.contains("01-alpha-k1.md")
            && diff.contains("01-DONE-alpha-k1.md"),
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

#[test]
fn leaf_insert_in_a_colocated_tree_leaves_the_git_index_alone() {
    let tmp = colocated(build_grove);
    let repo = tmp.path();
    let before = git_index(repo);

    let (_, stderr, ok) = llm(repo, &["leaf-insert", "[1]", "urgent"]);
    assert!(ok, "leaf-insert failed: {stderr}");

    assert!(
        exists(repo, ".grove/02-alpha-k1.md"),
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

    let (_, stderr, ok) = llm(repo, &["leaf-decompose", ".grove/01-alpha-k1.md", "sub"]);
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

    let (_, stderr, ok) = llm(repo, &["leaf-retire", ".grove/01-alpha-k1.md"]);
    assert!(ok, "leaf-retire failed: {stderr}");

    assert!(exists(repo, ".grove/01-DONE-alpha-k1.md"));
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

    assert!(exists(repo, ".grove/02-node-k2/01-ABANDONED-child-k3.md"));
    assert_eq!(
        git_index(repo),
        before,
        "jj-first: git's index is untouched"
    );
}

#[test]
fn migrate_in_a_colocated_tree_leaves_the_git_index_alone() {
    let tmp = colocated(|repo| {
        let g = repo.join(".grove");
        touch(&g.join("BRIEF.md"), "# proj — brief\n");
        touch(&g.join("done/010-first.md"), "# 010-first\n\nbody one\n");
        touch(&g.join("020-second/BRIEF.md"), "# 020-second — brief\n");
        touch(
            &g.join("020-second/010-child.md"),
            "# 010-child\n\nchild body\n",
        );
    });
    let repo = tmp.path();
    let before = git_index(repo);

    let (_, stderr, ok) = grove(repo, &["migrate"]);
    assert!(ok, "grove migrate failed: {stderr}");

    assert_eq!(
        tree(repo),
        vec![
            "01-DONE-first-k1.md",
            "02-second-k2/01-child-k3.md",
            "02-second-k2/BRIEF.md",
            "BRIEF.md",
        ],
    );
    assert_eq!(
        git_index(repo),
        before,
        "jj-first: git's index is untouched"
    );
}
