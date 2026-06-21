// End-to-end tests for the human verb `grove migrate` (060/020): the one-time,
// in-place conversion of an old `NNN-slug/` + `done/` tree to the new flat
// dotted-decimal scheme (ADR-0033/0034). These drive the real `grove` binary on
// a real git repo, so they exercise what the in-process unit tests in
// `src/migrate.rs` cannot: the `git mv`s, the on-disk header rewrites, the
// no-commit contract, the empty-directory cleanup, idempotency, and that the
// migrated tree drives correctly under `grove-llm pick` / `brief-chain`.

use assert_cmd::Command;
use std::fs;
use std::path::Path;
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

/// Write a file (creating parents) with the given body.
fn touch(p: &Path, body: &str) {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, body.as_bytes()).unwrap();
}

fn stage_commit(repo: &Path) {
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-q", "-m", "fixture"]);
}

fn head(repo: &Path) -> String {
    let out = Pcmd::new("git")
        .arg("-C")
        .arg(repo)
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn head_subject(repo: &Path) -> String {
    let out = Pcmd::new("git")
        .arg("-C")
        .arg(repo)
        .args(["log", "-1", "--format=%s"])
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn porcelain(repo: &Path) -> String {
    let out = Pcmd::new("git")
        .arg("-C")
        .arg(repo)
        .args(["status", "--porcelain"])
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn run_migrate(repo: &Path) -> (String, String, bool) {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(repo)
        .arg("migrate")
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

fn exists(repo: &Path, rel: &str) -> bool {
    repo.join(".grove").join(rel).exists()
}

fn read(repo: &Path, rel: &str) -> String {
    fs::read_to_string(repo.join(".grove").join(rel)).unwrap()
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
                let rel = entry
                    .path()
                    .strip_prefix(base)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                out.push(rel.replace('\\', "/"));
            }
        }
    }
    let mut out = Vec::new();
    let grove = repo.join(".grove");
    walk(&grove, &grove, &mut out);
    out.sort();
    out
}

/// Stand up a representative old-format fixture exercising every shape: a retired
/// root leaf, a partially-retired node (brief live + one live + one done child),
/// a fully-retired node (brief in `done/`), a live root leaf, and a foreign file.
fn build_old_fixture(repo: &Path) {
    let g = repo.join(".grove");
    touch(&g.join("BRIEF.md"), "# proj — brief\n\n## Goal\n");
    touch(&g.join("done/010-first.md"), "# 010-first\n\nbody one\n");
    touch(
        &g.join("020-second/BRIEF.md"),
        "# 020-second — brief\n\n## Goal\n",
    );
    touch(
        &g.join("020-second/020-live-child.md"),
        "# 020-live-child\n\nlive body\n",
    );
    touch(
        &g.join("done/020-second/010-done-child.md"),
        "# 010-done-child\n\ndone body\n",
    );
    touch(
        &g.join("done/030-old-node/BRIEF.md"),
        "# 030-old-node — brief\n\n## Goal\n",
    );
    touch(
        &g.join("done/030-old-node/010-grandchild.md"),
        "# 010-grandchild\n\ngc body\n",
    );
    touch(&g.join("040-last.md"), "# 040-last\n\nlast body\n");
    touch(&g.join("README.md"), "not a grove file\n");
}

/// The new flat tree the fixture migrates to (`BRIEF.md` stays; foreign README
/// stays; everything else is renamed flat with positions, keys, and markers).
fn expected_new_tree() -> Vec<String> {
    let mut v: Vec<String> = [
        "BRIEF.md",
        "README.md",
        "1-[1]-first.DONE.md",
        "2-[2]-second.BRIEF.md",
        "2.1-[3]-done-child.DONE.md",
        "2.2-[4]-live-child.md",
        "3-[5]-old-node.BRIEF.md",
        "3.1-[6]-grandchild.DONE.md",
        "4-[7]-last.md",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    v.sort();
    v
}

// ---------------------------------------------------------------------------

#[test]
fn migrate_converts_the_full_tree_in_place() {
    let repo = init_repo();
    build_old_fixture(repo.path());
    stage_commit(repo.path());

    let (stdout, stderr, ok) = run_migrate(repo.path());
    assert!(ok, "grove migrate failed: {stderr}");

    assert_eq!(
        tree(repo.path()),
        expected_new_tree(),
        "migrated tree shape"
    );
    // A summary of the renames is printed.
    assert!(
        stdout.contains("migrated"),
        "expected a summary, got {stdout:?}"
    );
    assert!(
        stdout.contains("1-[1]-first.DONE.md"),
        "summary should list renames, got {stdout:?}"
    );
}

#[test]
fn migrate_rewrites_headers_and_preserves_bodies() {
    let repo = init_repo();
    build_old_fixture(repo.path());
    stage_commit(repo.path());
    assert!(run_migrate(repo.path()).2);

    // A retired leaf: header retitled to its new stem (no `.DONE` in the header),
    // body byte-preserved.
    assert_eq!(
        read(repo.path(), "1-[1]-first.DONE.md"),
        "# 1-[1]-first\n\nbody one\n"
    );
    // A node brief: ` — brief` tail preserved.
    assert_eq!(
        read(repo.path(), "2-[2]-second.BRIEF.md")
            .lines()
            .next()
            .unwrap(),
        "# 2-[2]-second — brief"
    );
    // A live leaf.
    assert_eq!(
        read(repo.path(), "4-[7]-last.md"),
        "# 4-[7]-last\n\nlast body\n"
    );
    // The root brief is untouched (unkeyed singleton).
    assert_eq!(
        read(repo.path(), "BRIEF.md").lines().next().unwrap(),
        "# proj — brief"
    );
}

#[test]
fn migrate_is_a_reviewable_change_with_no_commit() {
    let repo = init_repo();
    build_old_fixture(repo.path());
    stage_commit(repo.path());
    let before = head(repo.path());

    assert!(run_migrate(repo.path()).2);

    assert_eq!(
        head(repo.path()),
        before,
        "migrate must not create a commit"
    );
    assert!(
        !porcelain(repo.path()).trim().is_empty(),
        "migrate must leave a dirty working tree for review"
    );
}

#[test]
fn migrate_removes_emptied_directories_but_keeps_foreign_files() {
    let repo = init_repo();
    build_old_fixture(repo.path());
    // A foreign file inside the done mirror must survive (and keep its dir alive).
    touch(&repo.path().join(".grove/done/notes.txt"), "keep me\n");
    stage_commit(repo.path());
    assert!(run_migrate(repo.path()).2);

    // Old node dirs are gone; the live root README stays.
    assert!(
        !exists(repo.path(), "020-second"),
        "old node dir should be removed"
    );
    assert!(!exists(repo.path(), "030-old-node"));
    assert!(exists(repo.path(), "README.md"), "foreign root file kept");
    // The done mirror survives only because it still holds the foreign notes.txt.
    assert!(
        exists(repo.path(), "done/notes.txt"),
        "foreign done file kept"
    );
}

#[test]
fn migrate_is_idempotent_noop_on_an_already_new_tree() {
    let repo = init_repo();
    build_old_fixture(repo.path());
    stage_commit(repo.path());
    assert!(run_migrate(repo.path()).2);
    // Commit the migrated tree so the second run starts clean.
    stage_commit(repo.path());
    let before = head(repo.path());

    let (_stdout, stderr, ok) = run_migrate(repo.path());
    assert!(ok, "second migrate should succeed");
    assert!(
        stderr.contains("already new-format"),
        "expected already-new diagnostic, got {stderr:?}"
    );
    assert_eq!(head(repo.path()), before);
    assert!(
        porcelain(repo.path()).trim().is_empty(),
        "no changes on a new tree"
    );
}

#[test]
fn migrate_noop_when_no_grove_dir() {
    let repo = init_repo();
    fs::write(repo.path().join("README"), b"r\n").unwrap();
    stage_commit(repo.path());

    let (_stdout, stderr, ok) = run_migrate(repo.path());
    assert!(ok, "migrate with no .grove/ should be a clean no-op");
    assert!(
        stderr.contains("no .grove/"),
        "expected no-.grove diagnostic, got {stderr:?}"
    );
}

#[test]
fn migrated_tree_drives_correctly_under_pick_and_brief_chain() {
    let repo = init_repo();
    build_old_fixture(repo.path());
    stage_commit(repo.path());
    assert!(run_migrate(repo.path()).2);

    // pick returns the first live leaf in the new version-sort order: position
    // [2,2] (the live child) precedes [4] (the live root leaf).
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(repo.path())
        .arg("pick")
        .output()
        .unwrap();
    let picked = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert!(
        picked.ends_with("/.grove/2.2-[4]-live-child.md"),
        "pick should return the first live leaf, got {picked:?}"
    );

    // brief-chain for that leaf is root brief → the node's brief.
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(repo.path())
        .args(["brief-chain", &picked])
        .output()
        .unwrap();
    let chain: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|l| {
            Path::new(l)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(chain, vec!["BRIEF.md", "2-[2]-second.BRIEF.md"]);
}

// ---------------------------------------------------------------------------
// CLI surface

#[test]
fn migrate_accepts_an_explicit_worktree_path() {
    // `grove migrate <path>` migrates `<path>/.grove`, run from anywhere.
    let repo = init_repo();
    build_old_fixture(repo.path());
    stage_commit(repo.path());
    let elsewhere = TempDir::new().unwrap();

    let out = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(elsewhere.path())
        .arg("migrate")
        .arg(repo.path())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "migrate <path> failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(tree(repo.path()), expected_new_tree());
}

#[test]
fn grove_binary_exposes_migrate() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        s.contains("migrate"),
        "grove --help should list migrate: {s}"
    );
}

#[test]
fn grove_llm_binary_does_not_expose_migrate() {
    // `migrate` is a human verb (noun-less, on `grove`), not an LLM verb.
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        !s.contains("migrate"),
        "grove-llm --help leaked migrate: {s}"
    );
}

// ---------------------------------------------------------------------------
// adoption (060/030): `grove do` migrates an old tree on adoption, committing the
// conversion as one reviewable commit *before* driving — so the loop sees only
// new format. `migrate::migrate_on_adoption` is the unit the loop driver calls;
// these tests exercise its commit + idempotency contract directly.

#[test]
fn adoption_migrates_and_commits_an_old_tree() {
    let repo = init_repo();
    build_old_fixture(repo.path());
    stage_commit(repo.path());
    let before = head(repo.path());

    let outcome = grove::migrate::migrate_on_adoption(repo.path(), "demo").unwrap();
    assert!(
        matches!(outcome, grove::migrate::Outcome::Migrated(_)),
        "old tree should report Migrated, got {outcome:?}"
    );

    // The tree is converted to the new flat scheme...
    assert_eq!(
        tree(repo.path()),
        expected_new_tree(),
        "migrated tree shape"
    );
    // ...and the conversion is committed (head advanced, working tree clean).
    assert_ne!(head(repo.path()), before, "adoption must create a commit");
    assert!(
        porcelain(repo.path()).trim().is_empty(),
        "adoption commit must leave a clean working tree"
    );
    // The commit is clear and self-describing (names the grove + the scheme).
    assert_eq!(
        head_subject(repo.path()),
        "grove(demo): migrate task tree to dotted-decimal scheme"
    );
}

#[test]
fn adopted_tree_drives_under_pick() {
    // After adoption, the committed new-format tree drives correctly: `pick`
    // returns the first live leaf in the new version-sort order.
    let repo = init_repo();
    build_old_fixture(repo.path());
    stage_commit(repo.path());
    grove::migrate::migrate_on_adoption(repo.path(), "demo").unwrap();

    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(repo.path())
        .arg("pick")
        .output()
        .unwrap();
    let picked = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert!(
        picked.ends_with("/.grove/2.2-[4]-live-child.md"),
        "adopted tree should drive under pick, got {picked:?}"
    );
}

#[test]
fn adoption_is_a_noop_on_an_already_new_tree() {
    let repo = init_repo();
    build_old_fixture(repo.path());
    stage_commit(repo.path());
    // First adoption migrates + commits.
    assert!(matches!(
        grove::migrate::migrate_on_adoption(repo.path(), "demo").unwrap(),
        grove::migrate::Outcome::Migrated(_)
    ));
    let before = head(repo.path());

    // Second adoption: already new-format → no commit, no churn.
    let outcome = grove::migrate::migrate_on_adoption(repo.path(), "demo").unwrap();
    assert!(
        matches!(outcome, grove::migrate::Outcome::AlreadyNew),
        "new tree should report AlreadyNew, got {outcome:?}"
    );
    assert_eq!(head(repo.path()), before, "no second commit on a new tree");
    assert!(
        porcelain(repo.path()).trim().is_empty(),
        "no churn on a new tree"
    );
}

#[test]
fn adoption_is_a_noop_when_no_grove_dir() {
    // A worktree with no `.grove/` at all (e.g. a freshly-created grove before
    // root-init) is a clean no-op — adoption never spuriously commits.
    let repo = init_repo();
    fs::write(repo.path().join("README"), b"r\n").unwrap();
    stage_commit(repo.path());
    let before = head(repo.path());

    let outcome = grove::migrate::migrate_on_adoption(repo.path(), "demo").unwrap();
    assert!(
        matches!(outcome, grove::migrate::Outcome::NothingToMigrate),
        "absent .grove/ should report NothingToMigrate, got {outcome:?}"
    );
    assert_eq!(
        head(repo.path()),
        before,
        "no commit when nothing to migrate"
    );
}
