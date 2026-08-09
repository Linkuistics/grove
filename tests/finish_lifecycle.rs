use assert_cmd::cargo::CommandCargoExt;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};
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

fn wait_for(path: &Path) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !path.exists() {
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {}",
            path.display()
        );
        thread::sleep(Duration::from_millis(10));
    }
}

fn find_entry_named(root: &Path, name: &OsStr) -> Option<PathBuf> {
    for entry in fs::read_dir(root).ok()? {
        let entry = entry.ok()?;
        if entry.file_name() == name {
            return Some(entry.path());
        }
        if entry.file_type().ok()?.is_dir() {
            if let Some(found) = find_entry_named(&entry.path(), name) {
                return Some(found);
            }
        }
    }
    None
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

fn grove_llm_with_path(repository: &Path, arguments: &[&str], path: &str) -> Output {
    Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("PATH", path)
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
    let subject = git(&repository, &["log", "-1", "--pretty=%s"]);
    let attempt = subject
        .strip_prefix("finish-k2 (finish attempt ")
        .and_then(|subject| subject.strip_suffix("): remove completed grove task tree"))
        .expect("finish commit subject must identify the handle and attempt");
    assert_eq!(attempt.len(), 32, "{subject}");
    assert!(
        attempt.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "{subject}"
    );
    let committed = git(&repository, &["show", "--pretty=", "--name-only", "HEAD"]);
    assert!(committed.contains(".grove/FORMAT"));
    assert!(!committed.contains("staged.txt"));
    assert_eq!(
        git(&repository, &["diff", "--cached", "--name-only"]),
        "staged.txt"
    );
    assert_eq!(
        git(&repository, &["ls-files", "--stage", "--", ".grove"]),
        "",
        "the successful finish must leave no task-tree paths in the index"
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

fn assert_failed_jj_finish_restores_the_tree_and_repository(colocated: bool) {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join(if colocated {
        "failed-colocated-jj"
    } else {
        "failed-native-jj"
    });
    init_jj(&repository, colocated);
    seed_jj_terminal_grove(&repository);
    if colocated {
        fs::write(repository.join("staged.txt"), "staged version\n").unwrap();
        run("git", &repository, &["add", "staged.txt"]);
        fs::write(repository.join("staged.txt"), "working-copy version\n").unwrap();
    }

    let grove = repository.join(".grove");
    let tree_before = tree_snapshot(&grove);
    let commit_before = git_like_jj_output(
        &repository,
        &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
    );
    let index_before = colocated.then(|| {
        let index = repository.join(git(&repository, &["rev-parse", "--git-path", "index"]));
        fs::read(index).unwrap()
    });

    let real_jj = String::from_utf8(run("which", &repository, &["jj"]).stdout)
        .unwrap()
        .trim()
        .to_owned();
    let fake_bin = fixture.path().join("fake-bin");
    fs::create_dir(&fake_bin).unwrap();
    write_executable(
        &fake_bin.join("jj"),
        &format!(
            "#!/bin/sh\nfor argument in \"$@\"; do\n  if [ \"$argument\" = commit ]; then\n    printf 'forced jj commit failure\\n' >&2\n    exit 1\n  fi\ndone\nexec {real_jj:?} \"$@\"\n"
        ),
    );
    let path = format!("{}:{}", fake_bin.display(), std::env::var("PATH").unwrap());

    let output = grove_llm_with_path(&repository, &["finish-commit", "finish-k2"], &path);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("forced jj commit failure"), "{stderr}");
    assert!(grove.is_dir(), "the failed jj finish removed the task root");
    assert_eq!(tree_snapshot(&grove), tree_before, "{stderr}");
    assert_eq!(
        git_like_jj_output(
            &repository,
            &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
        ),
        commit_before
    );
    if let Some(index_before) = index_before {
        let index = repository.join(git(&repository, &["rev-parse", "--git-path", "index"]));
        assert_eq!(fs::read(index).unwrap(), index_before);
    }
}

#[test]
fn failed_native_jj_finish_restores_the_tree_and_preflight_commit() {
    assert_failed_jj_finish_restores_the_tree_and_repository(false);
}

#[test]
fn failed_colocated_jj_finish_restores_the_tree_preflight_commit_and_git_index() {
    assert_failed_jj_finish_restores_the_tree_and_repository(true);
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
    let real_git =
        String::from_utf8(Command::new("which").arg("git").output().unwrap().stdout).unwrap();
    let fake_bin = fixture.path().join("fake-bin");
    fs::create_dir(&fake_bin).unwrap();
    write_executable(
        &fake_bin.join("git"),
        r#"#!/bin/sh
case "${GIT_INDEX_FILE-}" in
  *GROVE-FINISH-AUXILIARY-git-index-success-*)
    printf '%s\n' 'forced success-index preparation failure' >&2
    exit 71
    ;;
esac
exec "$GROVE_TEST_REAL_GIT" "$@"
"#,
    );
    let path = format!(
        "{}:{}",
        fake_bin.display(),
        std::env::var_os("PATH")
            .unwrap_or_default()
            .to_string_lossy()
    );

    let output = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("PATH", path)
        .env("GROVE_TEST_REAL_GIT", real_git.trim())
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("forced success-index preparation failure"),
        "{stderr}"
    );
    assert!(!fs::read_dir(&git_directory).unwrap().any(|entry| {
        entry
            .unwrap()
            .file_name()
            .as_encoded_bytes()
            .starts_with(b"GROVE-FINISH-AUXILIARY-")
    }));
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
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("no tracked state in HEAD"), "{stderr}");
    assert_eq!(tree_snapshot(&grove), before);
}

#[test]
fn finish_preflight_refuses_special_entries_before_deleting_the_tree() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("special-entry");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    fs::write(repository.join("staged.txt"), "staged\n").unwrap();
    run("git", &repository, &["add", "staged.txt"]);
    let socket_path = repository.join(".grove/runtime.sock");
    let _listener = UnixListener::bind(&socket_path).unwrap();
    let head_before = git(&repository, &["rev-parse", "HEAD"]);
    let index_before = git(&repository, &["ls-files", "--stage"]);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported task-tree entry"), "{stderr}");
    assert!(stderr.contains("runtime.sock"), "{stderr}");
    assert!(repository.join(".grove").is_dir());
    assert!(socket_path.exists());
    assert_eq!(git(&repository, &["rev-parse", "HEAD"]), head_before);
    assert_eq!(git(&repository, &["ls-files", "--stage"]), index_before);
}

#[test]
fn finish_preflight_refuses_a_reserved_witness_collision_before_deletion() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("reserved-collision");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    let collision = repository.join(".grove/FINISHING-finish-k2");
    fs::create_dir(&collision).unwrap();
    fs::write(collision.join("foreign"), "keep\n").unwrap();
    let head_before = git(&repository, &["rev-parse", "HEAD"]);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("reserved finish transaction path"),
        "{stderr}"
    );
    assert!(stderr.contains("FINISHING-finish-k2"), "{stderr}");
    assert_eq!(
        fs::read_to_string(collision.join("foreign")).unwrap(),
        "keep\n"
    );
    assert_eq!(git(&repository, &["rev-parse", "HEAD"]), head_before);
}

#[test]
fn plain_git_finish_commit_disables_user_hooks() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("failed-git-commit");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    fs::write(repository.join("staged.txt"), "staged\n").unwrap();
    run("git", &repository, &["add", "staged.txt"]);
    run("git", &repository, &["config", "--unset", "core.hooksPath"]);
    let hook = repository.join(".git/hooks/pre-commit");
    let hook_marker = repository.join("hook-ran");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    write_executable(
        &hook,
        "#!/bin/sh\nprintf 'hook ran\\n' >hook-ran\nprintf 'blocked finish commit\\n' >&2\nexit 1\n",
    );
    let index_before = git(
        &repository,
        &["ls-files", "--stage", "--", ".", ":(exclude).grove"],
    );

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!hook_marker.exists(), "the user hook ran during finish");
    assert_eq!(
        git(
            &repository,
            &["ls-files", "--stage", "--", ".", ":(exclude).grove"],
        ),
        index_before
    );
}

#[test]
fn failed_plain_git_finish_commit_restores_the_tree_and_index() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("failed-git-commit");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    fs::write(repository.join("staged.txt"), "staged\n").unwrap();
    run("git", &repository, &["add", "staged.txt"]);
    let failing_gpg = repository.join("failing-gpg");
    write_executable(&failing_gpg, "#!/bin/sh\nexit 1\n");
    run(
        "git",
        &repository,
        &["config", "gpg.program", failing_gpg.to_str().unwrap()],
    );
    run("git", &repository, &["config", "commit.gpgsign", "true"]);
    let grove = repository.join(".grove");
    let tree_before = tree_snapshot(&grove);
    let head_before = git(&repository, &["rev-parse", "HEAD"]);
    let index_before = git(&repository, &["ls-files", "--stage"]);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    assert!(grove.is_dir(), "the failed finish removed the task root");
    assert_eq!(tree_snapshot(&grove), tree_before);
    assert_eq!(git(&repository, &["rev-parse", "HEAD"]), head_before);
    assert_eq!(git(&repository, &["ls-files", "--stage"]), index_before);
}

#[test]
fn tree_readers_refuse_a_ready_finish_transaction() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("ready-finish-transaction");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    let grove = repository.join(".grove");
    let witness = grove.join("FINISHING-finish-k2");
    let original = witness.join("original");
    fs::create_dir_all(&original).unwrap();
    for entry in fs::read_dir(&grove)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
    {
        if entry.path() != witness {
            fs::rename(entry.path(), original.join(entry.file_name())).unwrap();
        }
    }
    fs::write(witness.join("READY"), "ready\n").unwrap();

    let output = grove_llm(&repository, &["pick"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("pending Grove finish transaction"),
        "{stderr}"
    );
    assert!(stderr.contains("FINISHING-finish-k2"), "{stderr}");
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

#[test]
fn bare_driver_validates_config_before_recovering_an_interrupted_finish() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-recovers-finish");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    let grove = repository.join(".grove");

    let interrupted = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_FAIL_AT", "after-evacuation")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(!interrupted.status.success());
    let witness = grove.join("FINISHING-finish-k2");
    assert!(witness.is_dir());

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    fs::create_dir_all(home.join(".config/grove")).unwrap();
    fs::write(
        home.join(".config/grove/config.kdl"),
        "finish \"sh -c true '${prompt}'\"\n",
    )
    .unwrap();
    let invalid = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();
    assert!(!invalid.status.success());
    assert!(witness.is_dir(), "invalid config mutated recovery state");

    let launch_log = fixture.path().join("launch-log");
    let script = fixture.path().join("record-finish.sh");
    write_executable(
        &script,
        &format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" > {}\n",
            launch_log.display()
        ),
    );
    let template = format!("sh {} '${{prompt}}'", script.display());
    write_complete_config(&home, &template);

    let recovered = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    assert!(
        recovered.status.success(),
        "{}",
        String::from_utf8_lossy(&recovered.stderr)
    );
    assert!(!witness.exists());
    assert!(grove.join("02-finish-finish-k2.md").is_file());
    assert!(fs::read_to_string(launch_log)
        .unwrap()
        .contains("finish-k2"));
}

#[test]
fn bare_driver_recovers_a_committed_witness_into_the_fresh_root_contract() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-recovers-committed-finish");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);

    let interrupted = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_FAIL_AT", "after-commit")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(!interrupted.status.success());
    assert!(repository.join(".grove/FINISHING-finish-k2").is_dir());
    assert!(git(&repository, &["log", "-1", "--pretty=%s"]).contains("finish-k2"));

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let launch_log = fixture.path().join("launch-log");
    let script = fixture.path().join("record-requirements.sh");
    write_executable(
        &script,
        &format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" > {}\n",
            launch_log.display()
        ),
    );
    let template = format!("sh {} '${{prompt}}'", script.display());
    write_complete_config(&home, &template);

    let recovered = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    assert!(
        recovered.status.success(),
        "{}",
        String::from_utf8_lossy(&recovered.stderr)
    );
    assert!(repository
        .join(".grove/01-requirements-plan-k1.md")
        .is_file());
    assert!(fs::read_to_string(launch_log).unwrap().contains("plan-k1"));
    let control = repository.join(".git/grove");
    assert!(fs::read_dir(control).unwrap().all(|entry| !entry
        .unwrap()
        .file_name()
        .to_string_lossy()
        .starts_with("FINISHED-")));
}

#[test]
fn bare_driver_blocks_on_divergent_finish_recovery_without_launching() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-blocks-divergent-finish");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);

    let interrupted = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_FAIL_AT", "after-evacuation")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(!interrupted.status.success());
    let witness = repository.join(".grove/FINISHING-finish-k2");
    assert!(witness.is_dir());

    fs::write(repository.join("divergent"), "preserve\n").unwrap();
    git(&repository, &["add", "divergent"]);
    git(&repository, &["commit", "-q", "-m", "divergent"]);

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let launch_log = fixture.path().join("launch-log");
    let script = fixture.path().join("record-launch.sh");
    write_executable(
        &script,
        &format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" > {}\n",
            launch_log.display()
        ),
    );
    let template = format!("sh {} '${{prompt}}'", script.display());
    write_complete_config(&home, &template);

    let blocked = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    assert!(!blocked.status.success());
    let diagnostic = String::from_utf8_lossy(&blocked.stderr);
    assert!(diagnostic.contains("Recovery pending"), "{diagnostic}");
    assert!(diagnostic.contains("recorded start"), "{diagnostic}");
    assert!(diagnostic.contains("exact teardown result"), "{diagnostic}");
    assert!(witness.is_dir());
    assert!(!launch_log.exists());
}

#[test]
fn cleanup_marker_publication_uses_the_validated_control_directory_object() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("descriptor-bound-marker-publication");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    let barrier = fixture.path().join("marker-publication-barrier");
    let control_directory = repository.join(".git/grove");
    let validated_control_directory = repository.join(".git/grove-validated");

    let child = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env(
            "GROVE_TEST_FINISH_CLEANUP_PAUSE_AT",
            "before-marker-publication",
        )
        .env("GROVE_TEST_FINISH_CLEANUP_BARRIER", &barrier)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(["finish-commit", "finish-k2"])
        .spawn()
        .unwrap();
    wait_for(&barrier);
    fs::rename(&control_directory, &validated_control_directory).unwrap();
    fs::create_dir(&control_directory).unwrap();
    fs::remove_file(&barrier).unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(!output.status.success());
    assert!(repository.join(".grove").is_dir());
    assert!(fs::read_dir(&validated_control_directory)
        .unwrap()
        .any(|entry| entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with("GROVE-FINISH-CLEANUP-")));
    assert!(fs::read_dir(&control_directory).unwrap().all(|entry| !entry
        .unwrap()
        .file_name()
        .to_string_lossy()
        .starts_with("GROVE-FINISH-CLEANUP-")));
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert!(
        diagnostic.contains("without following symlinks"),
        "{diagnostic}"
    );
}

#[test]
fn process_cleanup_does_not_unlink_a_substituted_non_directory_entry() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("identity-bound-entry-unlink");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    fs::write(repository.join(".grove/race-entry"), "original\n").unwrap();
    let barrier = fixture.path().join("entry-unlink-barrier");
    let control_directory = repository.join(".git/grove");

    let child = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env(
            "GROVE_TEST_FINISH_CLEANUP_PAUSE_AT",
            "before-non-directory-unlink",
        )
        .env("GROVE_TEST_FINISH_CLEANUP_BARRIER", &barrier)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(["finish-commit", "finish-k2"])
        .spawn()
        .unwrap();
    wait_for(&barrier);
    let entry_name = std::ffi::OsString::from_vec(fs::read(&barrier).unwrap());
    let claimed = fs::read_dir(&control_directory)
        .unwrap()
        .map(Result::unwrap)
        .find(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("REAPING-FINISHED-finish-k2-")
        })
        .expect("claimed cleanup quarantine")
        .path();
    let entry = find_entry_named(&claimed, &entry_name).expect("paused cleanup entry");
    assert!(fs::symlink_metadata(&entry).unwrap().file_type().is_file());
    let preserved = entry.with_file_name("preserved-process-original");
    fs::rename(&entry, &preserved).unwrap();
    fs::write(&entry, "replacement\n").unwrap();
    fs::remove_file(&barrier).unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(fs::read_to_string(&entry).unwrap(), "replacement\n");
    assert!(preserved.is_file());
    assert!(claimed.is_dir());
    assert!(fs::read_dir(&control_directory).unwrap().any(|entry| entry
        .unwrap()
        .file_name()
        .to_string_lossy()
        .starts_with("GROVE-FINISH-CLEANUP-")));
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert!(diagnostic.contains("identity changed"), "{diagnostic}");
}

#[test]
fn interrupted_post_commit_cleanup_leaves_attempt_bound_reaping_evidence() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("retryable-finish-cleanup");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);

    let output = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_CLEANUP_FAIL_AT", "before-root-removal")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!repository.join(".grove").exists());
    let control_directory = repository.join(".git/grove");
    let names = fs::read_dir(&control_directory)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    let marker_name = names
        .iter()
        .find(|name| name.to_string_lossy().starts_with("GROVE-FINISH-CLEANUP-"))
        .unwrap();
    let claimed_name = names
        .iter()
        .find(|name| {
            name.to_string_lossy()
                .starts_with("REAPING-FINISHED-finish-k2-")
        })
        .unwrap();
    let marker_path = control_directory.join(marker_name);
    let claimed_path = control_directory.join(claimed_name);
    let quarantine_name = claimed_name
        .to_string_lossy()
        .strip_prefix("REAPING-")
        .unwrap()
        .to_owned();
    let quarantine_path = control_directory.join(quarantine_name);
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert!(
        diagnostic.contains("completed Grove cleanup remains"),
        "{diagnostic}"
    );
    assert!(
        diagnostic.contains(marker_path.to_string_lossy().as_ref()),
        "{diagnostic}"
    );
    assert!(
        diagnostic.contains(claimed_path.to_string_lossy().as_ref()),
        "{diagnostic}"
    );
    assert!(
        !diagnostic.contains(quarantine_path.to_string_lossy().as_ref()),
        "{diagnostic}"
    );
}
