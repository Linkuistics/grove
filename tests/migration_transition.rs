use grove::tree_lifecycle::{transition_to_current, CurrentTransition};
use grove::tree_read;
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
    assert_eq!(tree_read::pick(&grove_root).unwrap(), Some(migrated_leaf));
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
