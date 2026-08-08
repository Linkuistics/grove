use grove::repo::commit_session_kind_migration;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

const TRANSACTION: &str = ".grove/MIGRATING-session-kinds";

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

fn run(binary: &str, repository: &Path, arguments: &[&str]) {
    let output = command_output(binary, repository, arguments);
    assert!(
        output.status.success(),
        "{binary} {arguments:?} failed in {}: {}",
        repository.display(),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_output(repository: &Path, arguments: &[&str]) -> String {
    let output = command_output("git", repository, arguments);
    assert!(
        output.status.success(),
        "git {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn jj_output(repository: &Path, arguments: &[&str]) -> String {
    let output = command_output("jj", repository, arguments);
    assert!(
        output.status.success(),
        "jj {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn write(path: &Path, body: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

fn init_git(repository: &Path) {
    run("git", repository, &["init", "-q", "."]);
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

fn init_jj(repository: &Path, colocated: bool) {
    if colocated {
        init_git(repository);
        run(
            "jj",
            repository,
            &["git", "init", "--colocate", "--quiet", "."],
        );
    } else {
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
    }
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

fn commit_git_fixture(repository: &Path) {
    run("git", repository, &["add", "-A"]);
    run("git", repository, &["commit", "-q", "-m", "fixture"]);
}

fn git_index_path(repository: &Path) -> std::path::PathBuf {
    let path = std::path::PathBuf::from(
        git_output(repository, &["rev-parse", "--git-path", "index"])
            .trim()
            .to_owned(),
    );
    if path.is_absolute() {
        path
    } else {
        repository.join(path)
    }
}

#[test]
fn plain_git_migration_commit_preserves_unrelated_index_and_worktree_changes() {
    let temporary = TempDir::new().unwrap();
    let repository = temporary.path();
    init_git(repository);
    write(&repository.join(".grove/legacy.md"), "legacy\n");
    write(&repository.join("staged.txt"), "before staged\n");
    write(&repository.join("working.txt"), "before working\n");
    commit_git_fixture(repository);

    fs::remove_file(repository.join(".grove/legacy.md")).unwrap();
    write(
        &repository.join(".grove/01-impl-migrated-k1.md"),
        "# migrated-k1\n",
    );
    write(&repository.join(".grove/FORMAT"), "session-kinds-v1\n");
    write(
        &repository.join(format!("{TRANSACTION}/manifest")),
        "recovery witness\n",
    );
    write(&repository.join("staged.txt"), "after staged\n");
    run("git", repository, &["add", "staged.txt"]);
    write(&repository.join("working.txt"), "after working\n");

    commit_session_kind_migration(repository, "test-grove").unwrap();

    let mut committed_paths: Vec<_> = git_output(
        repository,
        &["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"],
    )
    .lines()
    .map(str::to_owned)
    .collect();
    committed_paths.sort();
    assert_eq!(
        committed_paths,
        vec![
            ".grove/01-impl-migrated-k1.md",
            ".grove/FORMAT",
            ".grove/legacy.md",
        ]
    );
    assert_eq!(
        git_output(repository, &["diff", "--cached", "--name-only"]),
        "staged.txt\n"
    );
    assert_eq!(
        git_output(repository, &["diff", "--name-only"]),
        "working.txt\n"
    );
    assert_eq!(
        git_output(repository, &["show", "HEAD:staged.txt"]),
        "before staged\n"
    );
    assert_eq!(
        git_output(repository, &["show", ":staged.txt"]),
        "after staged\n"
    );
    assert!(repository.join(TRANSACTION).is_dir());
    assert!(
        !git_output(repository, &["ls-tree", "-r", "--name-only", "HEAD"])
            .contains("MIGRATING-session-kinds")
    );
}

#[test]
fn plain_git_migration_commit_supports_an_unborn_repository() {
    let temporary = TempDir::new().unwrap();
    let repository = temporary.path();
    init_git(repository);
    write(
        &repository.join(".grove/01-impl-migrated-k1.md"),
        "# migrated-k1\n",
    );
    write(&repository.join(".grove/FORMAT"), "session-kinds-v1\n");
    write(
        &repository.join(format!("{TRANSACTION}/manifest")),
        "recovery witness\n",
    );
    write(&repository.join("outside-staged.txt"), "outside\n");
    run("git", repository, &["add", "outside-staged.txt"]);

    commit_session_kind_migration(repository, "new-grove").unwrap();

    let mut head_paths: Vec<_> = git_output(repository, &["ls-tree", "-r", "--name-only", "HEAD"])
        .lines()
        .map(str::to_owned)
        .collect();
    head_paths.sort();
    assert_eq!(
        head_paths,
        vec![".grove/01-impl-migrated-k1.md", ".grove/FORMAT"]
    );
    assert_eq!(
        git_output(repository, &["diff", "--cached", "--name-only"]),
        "outside-staged.txt\n"
    );
    assert!(repository.join(TRANSACTION).is_dir());
}

#[test]
fn plain_git_commit_failure_restores_the_prior_index_for_recovery() {
    let temporary = TempDir::new().unwrap();
    let repository = temporary.path();
    init_git(repository);
    write(&repository.join(".grove/legacy.md"), "legacy\n");
    write(&repository.join("outside-staged.txt"), "before outside\n");
    commit_git_fixture(repository);

    fs::remove_file(repository.join(".grove/legacy.md")).unwrap();
    write(
        &repository.join(".grove/01-impl-migrated-k1.md"),
        "# migrated-k1\n",
    );
    write(&repository.join(".grove/FORMAT"), "session-kinds-v1\n");
    write(
        &repository.join(format!("{TRANSACTION}/manifest")),
        "recovery witness\n",
    );
    write(&repository.join("outside-staged.txt"), "after outside\n");
    run("git", repository, &["add", "outside-staged.txt"]);
    let index_before = git_output(repository, &["ls-files", "--stage"]);
    run("git", repository, &["config", "commit.gpgSign", "true"]);
    run("git", repository, &["config", "gpg.program", "false"]);

    let error = commit_session_kind_migration(repository, "test-grove").unwrap_err();

    let diagnostic = format!("{error:#}");
    assert!(diagnostic.contains("git commit --only"), "{diagnostic}");
    assert_eq!(
        git_output(repository, &["ls-files", "--stage"]),
        index_before
    );
    assert!(repository.join(TRANSACTION).is_dir());
}

#[test]
fn plain_git_post_commit_recovery_reuses_the_migration_commit() {
    let temporary = TempDir::new().unwrap();
    let repository = temporary.path();
    init_git(repository);
    write(&repository.join(".grove/legacy.md"), "legacy\n");
    commit_git_fixture(repository);
    fs::remove_file(repository.join(".grove/legacy.md")).unwrap();
    write(
        &repository.join(".grove/01-impl-migrated-k1.md"),
        "# migrated-k1\n",
    );
    write(&repository.join(".grove/FORMAT"), "session-kinds-v1\n");
    write(
        &repository.join(format!("{TRANSACTION}/manifest")),
        "recovery witness\n",
    );
    commit_session_kind_migration(repository, "test-grove").unwrap();
    let committed_head = git_output(repository, &["rev-parse", "HEAD"]);

    commit_session_kind_migration(repository, "test-grove").unwrap();

    assert_eq!(
        git_output(repository, &["rev-parse", "HEAD"]),
        committed_head
    );
    assert!(repository.join(TRANSACTION).is_dir());
}

#[test]
fn plain_git_recovery_restores_an_interrupted_precommit_index_backup() {
    let temporary = TempDir::new().unwrap();
    let repository = temporary.path();
    init_git(repository);
    write(&repository.join(".grove/legacy.md"), "legacy\n");
    write(&repository.join("outside-staged.txt"), "before outside\n");
    commit_git_fixture(repository);
    fs::remove_file(repository.join(".grove/legacy.md")).unwrap();
    write(
        &repository.join(".grove/01-impl-migrated-k1.md"),
        "# migrated-k1\n",
    );
    write(&repository.join(".grove/FORMAT"), "session-kinds-v1\n");
    write(
        &repository.join(format!("{TRANSACTION}/manifest")),
        "recovery witness\n",
    );
    write(&repository.join("outside-staged.txt"), "after outside\n");
    run("git", repository, &["add", "outside-staged.txt"]);
    let index_path = git_index_path(repository);
    let backup_path = index_path.with_file_name("grove-session-kind-migration-index");
    fs::copy(&index_path, &backup_path).unwrap();
    run(
        "git",
        repository,
        &[
            "add",
            "-A",
            "--",
            ".grove",
            ":(exclude).grove/MIGRATING-session-kinds",
        ],
    );

    commit_session_kind_migration(repository, "test-grove").unwrap();

    assert!(!backup_path.exists());
    assert_eq!(
        git_output(repository, &["diff", "--cached", "--name-only"]),
        "outside-staged.txt\n"
    );
}

fn exercise_jj_migration_commit(repository: &Path) {
    write(&repository.join(".grove/legacy.md"), "legacy\n");
    write(&repository.join("outside.txt"), "before outside\n");
    run("jj", repository, &["commit", "-m", "fixture"]);

    fs::remove_file(repository.join(".grove/legacy.md")).unwrap();
    write(
        &repository.join(".grove/01-impl-migrated-k1.md"),
        "# migrated-k1\n",
    );
    write(&repository.join(".grove/FORMAT"), "session-kinds-v1\n");
    write(
        &repository.join(format!("{TRANSACTION}/manifest")),
        "recovery witness\n",
    );
    write(&repository.join("outside.txt"), "after outside\n");

    commit_session_kind_migration(repository, "test-grove").unwrap();

    let committed = jj_output(repository, &["diff", "-r", "@-", "--summary"]);
    assert!(committed.contains(".grove/01-impl-migrated-k1.md"));
    assert!(committed.contains(".grove/FORMAT"));
    assert!(committed.contains(".grove/legacy.md"));
    assert!(!committed.contains("MIGRATING-session-kinds"));
    assert!(!committed.contains("outside.txt"));

    let successor = jj_output(repository, &["diff", "-r", "@", "--summary"]);
    assert!(successor.contains("MIGRATING-session-kinds"));
    assert!(successor.contains("outside.txt"));
}

#[test]
fn native_jj_migration_commit_leaves_unrelated_changes_in_the_successor() {
    let temporary = TempDir::new().unwrap();
    init_jj(temporary.path(), false);

    exercise_jj_migration_commit(temporary.path());

    assert!(!temporary.path().join(".git").exists());
}

#[test]
fn native_jj_post_commit_recovery_reuses_the_migration_commit() {
    let temporary = TempDir::new().unwrap();
    let repository = temporary.path();
    init_jj(repository, false);
    write(&repository.join(".grove/legacy.md"), "legacy\n");
    run("jj", repository, &["commit", "-m", "fixture"]);
    fs::remove_file(repository.join(".grove/legacy.md")).unwrap();
    write(
        &repository.join(".grove/01-impl-migrated-k1.md"),
        "# migrated-k1\n",
    );
    write(&repository.join(".grove/FORMAT"), "session-kinds-v1\n");
    write(
        &repository.join(format!("{TRANSACTION}/manifest")),
        "recovery witness\n",
    );
    commit_session_kind_migration(repository, "test-grove").unwrap();
    let committed_id = jj_output(
        repository,
        &["log", "-r", "@-", "--no-graph", "-T", "commit_id"],
    );

    commit_session_kind_migration(repository, "test-grove").unwrap();

    assert_eq!(
        jj_output(
            repository,
            &["log", "-r", "@-", "--no-graph", "-T", "commit_id"],
        ),
        committed_id
    );
    assert!(repository.join(TRANSACTION).is_dir());
}

#[test]
fn colocated_jj_migration_commit_does_not_mutate_the_git_index() {
    let temporary = TempDir::new().unwrap();
    let repository = temporary.path();
    init_jj(repository, true);
    write(&repository.join(".grove/legacy.md"), "legacy\n");
    write(&repository.join("outside.txt"), "before outside\n");
    run("jj", repository, &["commit", "-m", "fixture"]);
    fs::remove_file(repository.join(".grove/legacy.md")).unwrap();
    write(
        &repository.join(".grove/01-impl-migrated-k1.md"),
        "# migrated-k1\n",
    );
    write(&repository.join(".grove/FORMAT"), "session-kinds-v1\n");
    write(
        &repository.join(format!("{TRANSACTION}/manifest")),
        "recovery witness\n",
    );
    write(&repository.join("outside.txt"), "after outside\n");
    let index_before = git_output(repository, &["ls-files", "--stage"]);

    commit_session_kind_migration(repository, "test-grove").unwrap();

    let committed = jj_output(repository, &["diff", "-r", "@-", "--summary"]);
    assert!(committed.contains(".grove/01-impl-migrated-k1.md"));
    assert!(committed.contains(".grove/FORMAT"));
    assert!(committed.contains(".grove/legacy.md"));
    assert!(!committed.contains("MIGRATING-session-kinds"));
    assert!(!committed.contains("outside.txt"));
    let successor = jj_output(repository, &["diff", "-r", "@", "--summary"]);
    assert!(successor.contains("MIGRATING-session-kinds"));
    assert!(successor.contains("outside.txt"));
    assert_eq!(
        git_output(repository, &["ls-files", "--stage"]),
        index_before
    );
}

#[test]
fn colocated_jj_post_commit_recovery_reuses_the_commit_and_restores_the_index() {
    let temporary = TempDir::new().unwrap();
    let repository = temporary.path();
    init_jj(repository, true);
    write(&repository.join(".grove/legacy.md"), "legacy\n");
    write(&repository.join("outside.txt"), "before outside\n");
    run("jj", repository, &["commit", "-m", "fixture"]);
    fs::remove_file(repository.join(".grove/legacy.md")).unwrap();
    write(
        &repository.join(".grove/01-impl-migrated-k1.md"),
        "# migrated-k1\n",
    );
    write(&repository.join(".grove/FORMAT"), "session-kinds-v1\n");
    write(
        &repository.join(format!("{TRANSACTION}/manifest")),
        "recovery witness\n",
    );
    write(&repository.join("outside.txt"), "after outside\n");
    let index_before = git_output(repository, &["ls-files", "--stage"]);
    let index_path = git_index_path(repository);
    let backup_path = index_path.with_file_name("grove-session-kind-migration-index");
    fs::copy(&index_path, &backup_path).unwrap();
    run(
        "jj",
        repository,
        &[
            "commit",
            "-m",
            "grove(test-grove): migrate task tree to session-kind filenames",
            "root:.grove ~ root:.grove/MIGRATING-session-kinds",
        ],
    );
    let committed_id = jj_output(
        repository,
        &["log", "-r", "@-", "--no-graph", "-T", "commit_id"],
    );

    commit_session_kind_migration(repository, "test-grove").unwrap();

    assert!(!backup_path.exists());
    assert_eq!(
        jj_output(
            repository,
            &["log", "-r", "@-", "--no-graph", "-T", "commit_id"],
        ),
        committed_id
    );
    assert_eq!(
        git_output(repository, &["ls-files", "--stage"]),
        index_before
    );
    let successor = jj_output(repository, &["diff", "-r", "@", "--summary"]);
    assert!(successor.contains("MIGRATING-session-kinds"));
    assert!(successor.contains("outside.txt"));
}

#[test]
fn colocated_jj_recovery_restores_an_interrupted_index_backup() {
    let temporary = TempDir::new().unwrap();
    let repository = temporary.path();
    init_jj(repository, true);
    write(&repository.join(".grove/legacy.md"), "legacy\n");
    write(&repository.join("outside.txt"), "before outside\n");
    run("jj", repository, &["commit", "-m", "fixture"]);
    fs::remove_file(repository.join(".grove/legacy.md")).unwrap();
    write(
        &repository.join(".grove/01-impl-migrated-k1.md"),
        "# migrated-k1\n",
    );
    write(&repository.join(".grove/FORMAT"), "session-kinds-v1\n");
    write(
        &repository.join(format!("{TRANSACTION}/manifest")),
        "recovery witness\n",
    );
    write(&repository.join("outside.txt"), "after outside\n");
    let index_before = git_output(repository, &["ls-files", "--stage"]);
    let index_path = git_index_path(repository);
    let backup_path = index_path.with_file_name("grove-session-kind-migration-index");
    fs::copy(&index_path, &backup_path).unwrap();
    run("jj", repository, &["status"]);

    commit_session_kind_migration(repository, "test-grove").unwrap();

    assert!(!backup_path.exists());
    assert_eq!(
        git_output(repository, &["ls-files", "--stage"]),
        index_before
    );
    let committed = jj_output(repository, &["diff", "-r", "@-", "--summary"]);
    assert!(committed.contains(".grove/01-impl-migrated-k1.md"));
    assert!(!committed.contains("MIGRATING-session-kinds"));
    assert!(!committed.contains("outside.txt"));
}
