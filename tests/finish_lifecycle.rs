use assert_cmd::cargo::CommandCargoExt;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

const SESSION_KINDS: &[&str] = &[
    "requirements",
    "design",
    "planning",
    "prototype",
    "impl",
    "research-a",
    "research-b",
    "combine-research",
    "finish",
    "review-requirements",
    "review-design",
    "review-planning",
    "review-prototype",
    "review-impl",
    "integrate-review-requirements",
    "integrate-review-design",
    "integrate-review-planning",
    "integrate-review-prototype",
    "integrate-review-impl",
];

fn run(program: &str, current_dir: &Path, arguments: &[&str]) -> Output {
    let output = Command::new(program)
        .current_dir(current_dir)
        .args(arguments)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{program} {arguments:?} failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn git(repository: &Path, arguments: &[&str]) -> String {
    String::from_utf8(run("git", repository, arguments).stdout)
        .unwrap()
        .trim()
        .to_string()
}

fn write_executable(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

fn init_git(repository: &Path) {
    fs::create_dir_all(repository).unwrap();
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
    fs::create_dir_all(repository).unwrap();
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

fn seed_committed_terminal_grove(repository: &Path) {
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# finish-test — brief\n").unwrap();
    fs::write(grove.join("01-DONE-impl-finished-k1.md"), "# finished-k1\n").unwrap();
    fs::write(repository.join("kept.txt"), "kept\n").unwrap();
    run("git", repository, &["add", "-A"]);
    run("git", repository, &["commit", "-q", "-m", "fixture"]);
    fs::write(
        grove.join("02-finish-finish-k2.md"),
        "# finish-k2\n\n## Goal\n\nFinish.\n",
    )
    .unwrap();
}

fn seed_jj_terminal_grove(repository: &Path) {
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# finish-test — brief\n").unwrap();
    fs::write(grove.join("01-DONE-impl-finished-k1.md"), "# finished-k1\n").unwrap();
    fs::write(repository.join("outside.txt"), "before\n").unwrap();
    run("jj", repository, &["commit", "-m", "fixture"]);
    fs::write(
        grove.join("02-finish-finish-k2.md"),
        "# finish-k2\n\n## Goal\n\nFinish.\n",
    )
    .unwrap();
    fs::write(repository.join("outside.txt"), "after\n").unwrap();
}

fn grove_llm(repository: &Path, arguments: &[&str]) -> Output {
    Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .args(arguments)
        .output()
        .unwrap()
}

fn write_complete_config(home: &Path, template: &str) {
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    let document = SESSION_KINDS
        .iter()
        .map(|kind| format!("{kind} {template:?}\n"))
        .collect::<String>();
    fs::write(config_dir.join("config.kdl"), document).unwrap();
}

fn tree_snapshot(root: &Path) -> Vec<(String, Option<Vec<u8>>)> {
    fn walk(root: &Path, path: &Path, snapshot: &mut Vec<(String, Option<Vec<u8>>)>) {
        let mut entries = fs::read_dir(path)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        entries.sort_by_key(fs::DirEntry::file_name);
        for entry in entries {
            let path = entry.path();
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if entry.file_type().unwrap().is_dir() {
                snapshot.push((relative, None));
                walk(root, &path, snapshot);
            } else {
                snapshot.push((relative, Some(fs::read(path).unwrap())));
            }
        }
    }

    let mut snapshot = Vec::new();
    walk(root, root, &mut snapshot);
    snapshot
}

#[test]
fn plain_git_finish_commit_deletes_only_the_grove_and_preserves_other_work() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("plain-git");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    fs::write(repository.join("staged.txt"), "staged\n").unwrap();
    run("git", &repository, &["add", "staged.txt"]);
    fs::write(repository.join("unstaged.txt"), "unstaged\n").unwrap();

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!repository.join(".grove").exists());
    assert!(git(&repository, &["log", "-1", "--pretty=%s"]).contains("finish-k2"));
    let committed = git(&repository, &["show", "--pretty=", "--name-only", "HEAD"]);
    assert!(committed.contains(".grove/FORMAT"));
    assert!(!committed.contains("staged.txt"));
    assert_eq!(
        git(&repository, &["diff", "--cached", "--name-only"]),
        "staged.txt"
    );
    assert_eq!(
        fs::read_to_string(repository.join("unstaged.txt")).unwrap(),
        "unstaged\n"
    );
}

#[test]
fn finish_commit_refuses_byte_identically_when_ordinary_work_appeared() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("work-appeared");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    fs::write(
        repository.join(".grove/03-impl-late-work-k3.md"),
        "# late-work-k3\n",
    )
    .unwrap();
    let before = tree_snapshot(&repository.join(".grove"));
    let head_before = git(&repository, &["rev-parse", "HEAD"]);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("late-work-k3"));
    assert_eq!(tree_snapshot(&repository.join(".grove")), before);
    assert_eq!(git(&repository, &["rev-parse", "HEAD"]), head_before);
}

#[test]
fn finish_commit_refuses_pending_transactions_before_deleting_the_tree() {
    for (witness_name, expected_diagnostic) in [
        (
            "MIGRATING-session-kinds",
            "pending Grove session-kind migration",
        ),
        (
            "PROMOTING-finish-chain-k3",
            "pending Grove promotion transaction",
        ),
    ] {
        let fixture = TempDir::new().unwrap();
        let repository = fixture.path().join(witness_name);
        init_git(&repository);
        seed_committed_terminal_grove(&repository);
        let grove = repository.join(".grove");
        fs::create_dir(grove.join(witness_name)).unwrap();
        let before = tree_snapshot(&grove);
        let head_before = git(&repository, &["rev-parse", "HEAD"]);

        let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

        assert!(!output.status.success(), "{witness_name} was admitted");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(expected_diagnostic),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(tree_snapshot(&grove), before);
        assert_eq!(git(&repository, &["rev-parse", "HEAD"]), head_before);
    }
}

#[test]
fn finish_commit_refuses_an_unknown_tree_format_before_deleting_the_tree() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("unknown-format");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    let grove = repository.join(".grove");
    fs::write(grove.join("FORMAT"), "session-kinds-v2\n").unwrap();
    let before = tree_snapshot(&grove);
    let head_before = git(&repository, &["rev-parse", "HEAD"]);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("unsupported Grove tree format"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(tree_snapshot(&grove), before);
    assert_eq!(git(&repository, &["rev-parse", "HEAD"]), head_before);
}

#[test]
fn retrying_finish_commit_after_teardown_reports_already_finished() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("already-finished");
    init_git(&repository);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("this grove is already finished"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_jj_finish_commit_preserves_other_work(colocated: bool) {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join(if colocated {
        "colocated-jj"
    } else {
        "native-jj"
    });
    init_jj(&repository, colocated);
    seed_jj_terminal_grove(&repository);
    if colocated {
        fs::write(repository.join("staged.txt"), "staged version\n").unwrap();
        run("git", &repository, &["add", "staged.txt"]);
        fs::write(repository.join("staged.txt"), "working-copy version\n").unwrap();
    }
    let outside_git_index_before = colocated.then(|| {
        git(
            &repository,
            &["ls-files", "--stage", "--", ".", ":(exclude).grove"],
        )
    });

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!repository.join(".grove").exists());
    let committed = git_like_jj_output(&repository, &["diff", "-r", "@-", "--summary"]);
    assert!(committed.contains(".grove/FORMAT"));
    assert!(!committed.contains("outside.txt"));
    let successor = git_like_jj_output(&repository, &["diff", "-r", "@", "--summary"]);
    assert!(successor.contains("outside.txt"));
    let description = git_like_jj_output(
        &repository,
        &["log", "-r", "@-", "--no-graph", "-T", "description"],
    );
    assert!(description.contains("finish-k2"));
    if let Some(index_before) = outside_git_index_before {
        assert_eq!(
            git(
                &repository,
                &["ls-files", "--stage", "--", ".", ":(exclude).grove"],
            ),
            index_before
        );
        assert_eq!(
            git(&repository, &["ls-files", "--stage", "--", ".grove"]),
            "",
            "the colocated Git index must not re-stage the deleted grove"
        );
    }
}

fn git_like_jj_output(repository: &Path, arguments: &[&str]) -> String {
    String::from_utf8(run("jj", repository, arguments).stdout)
        .unwrap()
        .trim()
        .to_string()
}

#[test]
fn native_jj_finish_commit_preserves_unrelated_working_copy_changes() {
    assert_jj_finish_commit_preserves_other_work(false);
}

#[test]
fn colocated_jj_finish_commit_preserves_unrelated_work_and_the_git_index() {
    assert_jj_finish_commit_preserves_other_work(true);
}

#[test]
fn colocated_jj_finish_refuses_before_commit_when_success_index_cannot_be_prepared() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("colocated-jj-index-failure");
    init_jj(&repository, true);
    seed_jj_terminal_grove(&repository);
    fs::write(repository.join("staged.txt"), "staged version\n").unwrap();
    run("git", &repository, &["add", "staged.txt"]);
    fs::write(repository.join("staged.txt"), "working-copy version\n").unwrap();
    let outside_index_before = git(
        &repository,
        &["ls-files", "--stage", "--", ".", ":(exclude).grove"],
    );
    let parent_before = git_like_jj_output(
        &repository,
        &["log", "-r", "@-", "--no-graph", "-T", "commit_id"],
    );
    let git_directory = repository.join(git(&repository, &["rev-parse", "--git-dir"]));
    fs::write(git_directory.join("grove-finish-index.lock"), "occupied\n").unwrap();
    fs::write(
        git_directory.join("grove-finish-success-index.lock"),
        "occupied\n",
    )
    .unwrap();

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    assert_eq!(
        git(
            &repository,
            &["ls-files", "--stage", "--", ".", ":(exclude).grove"],
        ),
        outside_index_before
    );
    assert_eq!(
        git_like_jj_output(
            &repository,
            &["log", "-r", "@-", "--no-graph", "-T", "commit_id"],
        ),
        parent_before,
        "index preparation must fail before jj records the finish commit"
    );
}

#[test]
fn plain_git_unborn_finish_is_refused_before_deleting_the_tree() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("unborn-git");
    init_git(&repository);
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# unborn-git — brief\n").unwrap();
    fs::write(
        grove.join("01-finish-finish-k1.md"),
        "# finish-k1\n\n## Goal\n\nFinish.\n",
    )
    .unwrap();
    let before = tree_snapshot(&grove);

    let output = grove_llm(&repository, &["finish-commit", "finish-k1"]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("no tracked state in HEAD"));
    assert_eq!(tree_snapshot(&grove), before);
}

#[test]
fn failed_plain_git_finish_commit_restores_the_preexisting_index() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("failed-git-commit");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    fs::write(repository.join("staged.txt"), "staged\n").unwrap();
    run("git", &repository, &["add", "staged.txt"]);
    run("git", &repository, &["config", "--unset", "core.hooksPath"]);
    let hook = repository.join(".git/hooks/pre-commit");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    write_executable(
        &hook,
        "#!/bin/sh\nprintf 'blocked finish commit\\n' >&2\nexit 1\n",
    );
    let index_before = git(&repository, &["ls-files", "--stage"]);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("blocked finish commit"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(git(&repository, &["ls-files", "--stage"]), index_before);
}

#[test]
fn configured_finish_target_commits_teardown_then_stops_the_loop_cleanly() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("clean-stop");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    fs::remove_file(repository.join(".grove/02-finish-finish-k2.md")).unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let script = fixture.path().join("finish-session.sh");
    fs::write(
        &script,
        "#!/bin/sh\n\"$1\" finish-commit finish-k2 || exit $?\n\"$1\" complete --done\n",
    )
    .unwrap();
    let template = format!(
        "sh {} {} '${{prompt}}'",
        script.display(),
        env!("CARGO_BIN_EXE_grove-llm")
    );
    write_complete_config(&home, &template);

    let output = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!repository.join(".grove").exists());
    assert!(String::from_utf8_lossy(&output.stderr).contains("grove finished"));
    assert!(git(&repository, &["log", "-1", "--pretty=%s"]).contains("finish-k2"));
}
