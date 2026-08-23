mod support;

use grove::task_tree;
use grove::tree_lifecycle::{transition_to_current, CurrentTransition};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

#[derive(Clone, Copy, Debug)]
enum RepositoryKind {
    Git,
    NativeJj,
    ColocatedJj,
}

fn command_output(binary: &str, repository: &Path, arguments: &[&str]) -> Output {
    Command::new(binary)
        .current_dir(repository)
        .args(arguments)
        .output()
        .unwrap_or_else(|error| {
            panic!(
                "running {binary} {arguments:?} in {}: {error}",
                repository.display()
            )
        })
}

fn run(binary: &str, repository: &Path, arguments: &[&str]) -> String {
    let output = command_output(binary, repository, arguments);
    assert!(
        output.status.success(),
        "{binary} {arguments:?} failed in {}: {}",
        repository.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn initialize(repository: &Path, kind: RepositoryKind) {
    match kind {
        RepositoryKind::Git => {
            run("git", repository, &["init", "-q", "."]);
            configure_git(repository);
        }
        RepositoryKind::NativeJj => {
            run(
                "jj",
                repository,
                &[
                    "--config",
                    "git.colocate=false",
                    "git",
                    "init",
                    "--quiet",
                    ".",
                ],
            );
            configure_jj(repository);
        }
        RepositoryKind::ColocatedJj => {
            run("git", repository, &["init", "-q", "."]);
            configure_git(repository);
            run(
                "jj",
                repository,
                &["git", "init", "--colocate", "--quiet", "."],
            );
            configure_jj(repository);
        }
    }
}

fn configure_git(repository: &Path) {
    run("git", repository, &["config", "user.name", "Grove Test"]);
    run(
        "git",
        repository,
        &["config", "user.email", "grove-test@example.com"],
    );
    run(
        "git",
        repository,
        &["config", "core.hooksPath", "/dev/null"],
    );
}

fn configure_jj(repository: &Path) {
    run(
        "jj",
        repository,
        &[
            "config",
            "set",
            "--workspace",
            "user.name",
            "\"Grove Test\"",
        ],
    );
    run(
        "jj",
        repository,
        &[
            "config",
            "set",
            "--workspace",
            "user.email",
            "\"grove-test@example.com\"",
        ],
    );
}

fn write(path: &Path, body: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

fn seed_legacy_tree(repository: &Path) {
    write(&repository.join(".grove/BRIEF.md"), "# fixture — brief\n");
    write(
        &repository.join(".grove/01-task-k1.md"),
        "# task-k1\n\n**Kind:** impl\n\n## Goal\nShip.\n",
    );
}

fn migration_description(repository: &Path, kind: RepositoryKind) -> String {
    match kind {
        RepositoryKind::Git => run("git", repository, &["log", "-1", "--format=%s"]),
        RepositoryKind::NativeJj | RepositoryKind::ColocatedJj => run(
            "jj",
            repository,
            &["log", "-r", "@-", "--no-graph", "-T", "description"],
        ),
    }
}

fn assert_successful_transition_and_restart(kind: RepositoryKind) {
    let temporary = TempDir::new().unwrap();
    let repository = temporary.path().join("fixture");
    fs::create_dir(&repository).unwrap();
    initialize(&repository, kind);
    seed_legacy_tree(&repository);

    assert_eq!(
        transition_to_current(&repository).unwrap(),
        CurrentTransition::Migrated
    );

    let grove_root = repository.join(".grove");
    let migrated_leaf = grove_root.join("01-impl-task-k1.md");
    assert_eq!(
        fs::read_to_string(grove_root.join("FORMAT")).unwrap(),
        "session-kinds-v1\n"
    );
    assert!(!grove_root.join("01-task-k1.md").exists());
    assert_eq!(task_tree::pick(&grove_root).unwrap(), Some(migrated_leaf));
    assert_eq!(
        migration_description(&repository, kind),
        "grove(fixture): migrate task tree to session-kind filenames\n"
    );

    assert_eq!(
        transition_to_current(&repository).unwrap(),
        CurrentTransition::AlreadyCurrent
    );
    assert_eq!(
        migration_description(&repository, kind),
        "grove(fixture): migrate task tree to session-kind filenames\n"
    );
}

fn assert_unknown_format_refusal(kind: RepositoryKind) {
    let temporary = TempDir::new().unwrap();
    let repository = temporary.path().join("fixture");
    fs::create_dir(&repository).unwrap();
    initialize(&repository, kind);
    let leaf = repository.join(".grove/01-impl-task-k1.md");
    write(&repository.join(".grove/BRIEF.md"), "# fixture — brief\n");
    write(&leaf, "# task-k1\n");
    write(&repository.join(".grove/FORMAT"), "session-kinds-v99\n");

    let error = transition_to_current(&repository).unwrap_err();

    assert!(
        format!("{error:#}").contains("unsupported Grove tree format"),
        "unexpected error: {error:#}"
    );
    assert_eq!(fs::read_to_string(leaf).unwrap(), "# task-k1\n");
    assert_eq!(
        fs::read_to_string(repository.join(".grove/FORMAT")).unwrap(),
        "session-kinds-v99\n"
    );
    assert!(!repository.join(".grove/MIGRATING-session-kinds").exists());
}

/// A repository selected by the *caller's* context must not capture the
/// migration commit. `current_dir` alone does not achieve that: Git reads
/// `GIT_DIR` / `GIT_WORK_TREE` / `GIT_COMMON_DIR` and a repository-local
/// `core.worktree` in preference to where the process happens to be standing.
/// The guard is `repo::anchor_git_worktree_environment`, applied to every
/// internal Git child; it drops the ambient selectors *and* pins
/// `GIT_WORK_TREE`, which is the half that overrides `core.worktree`.
///
/// The hazard is live rather than hypothetical: a personal launch environment
/// is exactly what `grove` inherits, and both failure modes are silent — the
/// intended tree still looks migrated on disk either way.
///
/// So the assertions are split by failure mode, and each was checked against a
/// deliberately unanchored build rather than assumed. Redirecting the
/// *repository* moves the commit somewhere else entirely, which "`HEAD`
/// advanced here, not there" catches. Redirecting only the *work tree* leaves
/// `intended`'s `HEAD` advancing perfectly normally while the commit records a
/// tree staged from `foreign` — visible only in the commit's content, which is
/// why that assertion is here and is not redundant with the first.
///
/// `scrub_internal_child_env` runs first and covers selectors the anchor does
/// not name (`GIT_INDEX_FILE`); on this path the anchor subsumes it, so it is
/// defence in depth and `launch.rs`'s own unit tests are what pin it.
#[test]
fn migration_commits_in_the_leased_worktree_despite_a_foreign_git_context() {
    let temporary = TempDir::new().unwrap();
    let intended = temporary.path().join("intended");
    let foreign = temporary.path().join("foreign");
    fs::create_dir(&intended).unwrap();
    fs::create_dir(&foreign).unwrap();
    initialize(&intended, RepositoryKind::Git);
    initialize(&foreign, RepositoryKind::Git);
    seed_legacy_tree(&intended);
    run("git", &intended, &["add", "-A"]);
    run(
        "git",
        &intended,
        &["commit", "-q", "-m", "seed legacy grove"],
    );
    let intended_head_before = run("git", &intended, &["rev-parse", "HEAD"]);
    fs::write(foreign.join("unrelated"), "unrelated\n").unwrap();
    run("git", &foreign, &["add", "-A"]);
    run("git", &foreign, &["commit", "-q", "-m", "foreign seed"]);
    let foreign_head_before = run("git", &foreign, &["rev-parse", "HEAD"]);

    // Both kinds of redirection at once: the repository-local setting and the
    // three ambient selectors.
    run(
        "git",
        &intended,
        &["config", "core.worktree", foreign.to_str().unwrap()],
    );
    let mut environment = support::EnvGuard::new();
    environment
        .set("GIT_DIR", foreign.join(".git"))
        .set("GIT_WORK_TREE", &foreign)
        .set("GIT_COMMON_DIR", foreign.join(".git"));

    let transition = transition_to_current(&intended);

    drop(environment);
    assert_eq!(transition.unwrap(), CurrentTransition::Migrated);
    assert!(intended.join(".grove/01-impl-task-k1.md").exists());
    // The scrub half: an unscrubbed `GIT_DIR` puts the commit in `foreign`.
    assert_ne!(
        run("git", &intended, &["rev-parse", "HEAD"]),
        intended_head_before,
        "the migration must be committed in the leased worktree"
    );
    assert_eq!(
        run("git", &foreign, &["rev-parse", "HEAD"]),
        foreign_head_before,
        "the repository selected by the environment must be untouched"
    );
    // The anchor half: an unanchored child honours `core.worktree` and stages
    // from `foreign`, where `.grove/` does not exist — so `HEAD` still advances
    // in the right repository while recording the wrong (empty) tree.
    let recorded = run(
        "git",
        &intended,
        &["show", "--name-only", "--format=", "HEAD"],
    );
    assert!(
        recorded.contains(".grove/01-impl-task-k1.md"),
        "the migration commit must record the migrated tree, got: {recorded:?}"
    );
}

#[test]
fn plain_git_transition_migrates_commits_picks_and_restarts() {
    assert_successful_transition_and_restart(RepositoryKind::Git);
}

#[test]
fn native_jj_transition_migrates_commits_picks_and_restarts() {
    assert_successful_transition_and_restart(RepositoryKind::NativeJj);
}

#[test]
fn colocated_jj_transition_migrates_commits_picks_and_restarts() {
    assert_successful_transition_and_restart(RepositoryKind::ColocatedJj);
}

#[test]
fn plain_git_transition_refuses_an_unknown_format_without_mutation() {
    assert_unknown_format_refusal(RepositoryKind::Git);
}

#[test]
fn native_jj_transition_refuses_an_unknown_format_without_mutation() {
    assert_unknown_format_refusal(RepositoryKind::NativeJj);
}

#[test]
fn colocated_jj_transition_refuses_an_unknown_format_without_mutation() {
    assert_unknown_format_refusal(RepositoryKind::ColocatedJj);
}
