use assert_cmd::cargo::CommandCargoExt;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::os::fd::AsRawFd;
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

fn git_index_path(repository: &Path) -> PathBuf {
    let path = PathBuf::from(git(repository, &["rev-parse", "--git-path", "index"]));
    if path.is_absolute() {
        path
    } else {
        repository.join(path)
    }
}

fn auxiliary_markers(repository: &Path) -> Vec<PathBuf> {
    let mut markers = fs::read_dir(git_index_path(repository).parent().unwrap())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            let name = path.file_name().unwrap().to_string_lossy();
            name.starts_with("GROVE-FINISH-AUXILIARY-") && name.ends_with(".json")
        })
        .collect::<Vec<_>>();
    markers.sort();
    markers
}

fn auxiliary_artifact(marker: &Path) -> PathBuf {
    let name = marker.file_name().unwrap().to_string_lossy();
    marker.with_file_name(name.strip_suffix(".json").unwrap())
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
fn plain_git_restart_recovers_a_process_exit_after_preparing_witness_publication() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-recovers-preparing-finish");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);

    let interrupted = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_EXIT_AT", "after-preparing-witness")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();

    assert!(!interrupted.status.success());
    let preparing = fs::read_dir(repository.join(".grove"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("PREPARING-FINISH-finish-k2-")
        })
        .expect("process exit must leave the atomically published preparing witness");
    assert!(preparing.is_dir());
    assert!(auxiliary_markers(&repository).is_empty());

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, "sh -c true '${prompt}'");
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
    assert!(!preparing.exists());
    assert!(repository.join(".grove/02-finish-finish-k2.md").is_file());
}

#[test]
fn plain_git_restart_recovers_a_process_exit_after_repository_preparation() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-recovers-prepared-finish");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    let index_before = fs::read(git_index_path(&repository)).unwrap();

    let interrupted = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_EXIT_AT", "after-repository-preparation")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();

    assert!(!interrupted.status.success());
    let preparing = fs::read_dir(repository.join(".grove"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("PREPARING-FINISH-finish-k2-")
        })
        .expect("process exit must leave the repository preparation owner");
    assert!(preparing.is_dir());
    assert_eq!(auxiliary_markers(&repository).len(), 1);

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, "sh -c true '${prompt}'");
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
    assert!(!preparing.exists());
    assert!(auxiliary_markers(&repository).is_empty());
    assert_eq!(fs::read(git_index_path(&repository)).unwrap(), index_before);
    assert!(repository.join(".grove/02-finish-finish-k2.md").is_file());
}

#[test]
fn plain_git_restart_recovers_each_preparing_witness_materialization_state() {
    for checkpoint in ["after-recovery-tree", "after-manifest", "after-ready"] {
        let fixture = TempDir::new().unwrap();
        let repository = fixture.path().join(format!("driver-recovers-{checkpoint}"));
        init_git(&repository);
        seed_committed_terminal_grove(&repository);
        let index_before = fs::read(git_index_path(&repository)).unwrap();

        let interrupted = Command::cargo_bin("grove-llm")
            .unwrap()
            .current_dir(&repository)
            .env_remove("GROVE_SIGNAL_FILE")
            .env("GROVE_TEST_FINISH_EXIT_AT", checkpoint)
            .args(["finish-commit", "finish-k2"])
            .output()
            .unwrap();

        assert!(!interrupted.status.success(), "{checkpoint}");
        let preparing = fs::read_dir(repository.join(".grove"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("PREPARING-FINISH-finish-k2-")
            })
            .unwrap_or_else(|| panic!("{checkpoint} did not retain preparing state"));
        assert_eq!(auxiliary_markers(&repository).len(), 1, "{checkpoint}");

        let home = fixture.path().join("home");
        fs::create_dir_all(home.join(".codex")).unwrap();
        write_complete_config(&home, "sh -c true '${prompt}'");
        let recovered = Command::cargo_bin("grove")
            .unwrap()
            .current_dir(&repository)
            .env("HOME", &home)
            .env_remove("GROVE_SIGNAL_FILE")
            .output()
            .unwrap();

        assert!(
            recovered.status.success(),
            "{checkpoint}: {}",
            String::from_utf8_lossy(&recovered.stderr)
        );
        assert!(!preparing.exists(), "{checkpoint}");
        assert!(auxiliary_markers(&repository).is_empty(), "{checkpoint}");
        assert_eq!(
            fs::read(git_index_path(&repository)).unwrap(),
            index_before,
            "{checkpoint}"
        );
        assert!(
            repository.join(".grove/02-finish-finish-k2.md").is_file(),
            "{checkpoint}"
        );
    }
}

#[test]
fn plain_git_restart_recovers_a_ready_witness_before_evacuation() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-recovers-ready-finish");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);
    let index_before = fs::read(git_index_path(&repository)).unwrap();

    let interrupted = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_EXIT_AT", "after-ready-witness")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();

    assert!(!interrupted.status.success());
    let witness = repository.join(".grove/FINISHING-finish-k2");
    assert!(witness.is_dir());
    assert!(repository.join(".grove/FORMAT").is_file());
    assert!(repository.join(".grove/02-finish-finish-k2.md").is_file());
    assert_eq!(auxiliary_markers(&repository).len(), 1);

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, "sh -c true '${prompt}'");
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
    assert!(auxiliary_markers(&repository).is_empty());
    assert_eq!(fs::read(git_index_path(&repository)).unwrap(), index_before);
    assert!(repository.join(".grove/02-finish-finish-k2.md").is_file());
}

#[test]
fn colocated_jj_restart_recovers_each_pre_evacuation_publication_state() {
    for (checkpoint, expected_auxiliaries) in [
        ("after-preparing-witness", 0),
        ("after-repository-preparation", 2),
        ("after-recovery-tree", 2),
        ("after-manifest", 2),
        ("after-ready", 2),
        ("after-ready-witness", 2),
    ] {
        let fixture = TempDir::new().unwrap();
        let repository = fixture.path().join(format!("jj-recovers-{checkpoint}"));
        init_jj(&repository, true);
        seed_jj_terminal_grove(&repository);
        fs::write(repository.join("staged.txt"), "working-copy version\n").unwrap();
        let commit_before = git_like_jj_output(
            &repository,
            &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
        );
        fs::write(repository.join("staged.txt"), "staged version\n").unwrap();
        run("git", &repository, &["add", "staged.txt"]);
        fs::write(repository.join("staged.txt"), "working-copy version\n").unwrap();
        let index_before = fs::read(git_index_path(&repository)).unwrap();

        let interrupted = Command::cargo_bin("grove-llm")
            .unwrap()
            .current_dir(&repository)
            .env_remove("GROVE_SIGNAL_FILE")
            .env("GROVE_TEST_FINISH_EXIT_AT", checkpoint)
            .args(["finish-commit", "finish-k2"])
            .output()
            .unwrap();

        assert!(!interrupted.status.success(), "{checkpoint}");
        assert_eq!(
            auxiliary_markers(&repository).len(),
            expected_auxiliaries,
            "{checkpoint}"
        );

        let home = fixture.path().join("home");
        fs::create_dir_all(home.join(".codex")).unwrap();
        write_complete_config(&home, "sh -c true '${prompt}'");
        let recovered = Command::cargo_bin("grove")
            .unwrap()
            .current_dir(&repository)
            .env("HOME", &home)
            .env_remove("GROVE_SIGNAL_FILE")
            .output()
            .unwrap();

        assert!(
            recovered.status.success(),
            "{checkpoint}: {}",
            String::from_utf8_lossy(&recovered.stderr)
        );
        assert!(auxiliary_markers(&repository).is_empty(), "{checkpoint}");
        assert_eq!(
            fs::read(git_index_path(&repository)).unwrap(),
            index_before,
            "{checkpoint}"
        );
        assert_eq!(
            git_like_jj_output(
                &repository,
                &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
            ),
            commit_before,
            "{checkpoint}"
        );
        assert!(
            repository.join(".grove/02-finish-finish-k2.md").is_file(),
            "{checkpoint}"
        );
    }
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

#[test]
fn bare_driver_reaps_orphaned_quarantine_after_valid_config_and_starts_fresh_grove() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-reaps-finish-cleanup");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);

    let interrupted = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_CLEANUP_FAIL_AT", "before-root-removal")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(
        interrupted.status.success(),
        "{}",
        String::from_utf8_lossy(&interrupted.stderr)
    );
    assert!(!repository.join(".grove").exists());

    let control_directory = repository.join(".git/grove");
    let cleanup_evidence_count = || {
        fs::read_dir(&control_directory)
            .unwrap()
            .filter(|entry| {
                let name = entry.as_ref().unwrap().file_name();
                let name = name.to_string_lossy();
                name.starts_with("GROVE-FINISH-CLEANUP-")
                    || name.starts_with("FINISHED-")
                    || name.starts_with("REAPING-FINISHED-")
            })
            .count()
    };
    assert_eq!(cleanup_evidence_count(), 2);

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    fs::create_dir_all(home.join(".config/grove")).unwrap();
    fs::write(
        home.join(".config/grove/config.kdl"),
        "requirements \"sh -c true '${prompt}'\"\n",
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
    assert_eq!(cleanup_evidence_count(), 2);
    assert!(!repository.join(".grove").exists());

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
    assert_eq!(cleanup_evidence_count(), 0);
    assert!(repository
        .join(".grove/01-requirements-plan-k1.md")
        .is_file());
    assert!(fs::read_to_string(launch_log).unwrap().contains("plan-k1"));
}

#[test]
fn bare_driver_waits_for_the_universal_tree_lock_before_reaping() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-locks-finish-cleanup");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);

    let interrupted = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_CLEANUP_FAIL_AT", "before-root-removal")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(interrupted.status.success());
    let control_directory = repository.join(".git/grove");
    let cleanup_marker = fs::read_dir(&control_directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("GROVE-FINISH-CLEANUP-")
        })
        .unwrap();

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, "sh -c true '${prompt}'");

    let worktree_directory = File::open(&repository).unwrap();
    let locked = unsafe { libc::flock(worktree_directory.as_raw_fd(), libc::LOCK_EX) };
    assert_eq!(locked, 0);
    let stderr_path = fixture.path().join("driver.stderr");
    let stderr = File::create(&stderr_path).unwrap();
    let mut child = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .stdout(Stdio::piped())
        .stderr(Stdio::from(stderr))
        .spawn()
        .unwrap();

    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let diagnostic = fs::read_to_string(&stderr_path).unwrap_or_default();
        if diagnostic.contains("waiting for active Grove tree operation") {
            break;
        }
        if let Some(status) = child.try_wait().unwrap() {
            panic!("driver exited before waiting for the tree lock ({status}): {diagnostic}");
        }
        assert!(
            Instant::now() < deadline,
            "driver did not report tree-lock contention: {diagnostic}"
        );
        thread::sleep(Duration::from_millis(10));
    }
    let evidence_survived_contention = cleanup_marker.is_file();

    let unlocked = unsafe { libc::flock(worktree_directory.as_raw_fd(), libc::LOCK_UN) };
    assert_eq!(unlocked, 0);
    let output = child.wait_with_output().unwrap();
    let diagnostic = fs::read_to_string(&stderr_path).unwrap();
    assert!(output.status.success(), "{diagnostic}");
    assert!(
        evidence_survived_contention,
        "cleanup ran before the universal tree lock was acquired"
    );
    assert!(!cleanup_marker.exists());
}

#[test]
fn bare_driver_reaps_old_attempt_but_preserves_exact_in_tree_cleanup_owner() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-distinguishes-cleanup-attempts");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);

    let old_finish = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_CLEANUP_FAIL_AT", "before-root-removal")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(
        old_finish.status.success(),
        "{}",
        String::from_utf8_lossy(&old_finish.stderr)
    );
    let control_directory = repository.join(".git/grove");
    let cleanup_markers = || {
        fs::read_dir(&control_directory)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("GROVE-FINISH-CLEANUP-")
            })
            .collect::<Vec<_>>()
    };
    let old_marker = cleanup_markers().pop().unwrap();
    let old_attempt = old_marker
        .file_name()
        .unwrap()
        .to_string_lossy()
        .trim_start_matches("GROVE-FINISH-CLEANUP-")
        .trim_end_matches(".json")
        .to_owned();

    seed_committed_terminal_grove(&repository);
    let current_finish = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env(
            "GROVE_TEST_FINISH_CLEANUP_FAIL_AT",
            "after-marker-publication",
        )
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(!current_finish.status.success());
    assert!(repository.join(".grove/FINISHING-finish-k2").is_dir());
    assert_eq!(cleanup_markers().len(), 2);

    fs::write(repository.join("divergent"), "preserve\n").unwrap();
    git(&repository, &["add", "divergent"]);
    git(&repository, &["commit", "-q", "-m", "divergent"]);

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, "sh -c true '${prompt}'");
    let blocked = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    assert!(!blocked.status.success());
    assert!(String::from_utf8_lossy(&blocked.stderr).contains("Recovery pending"));
    let remaining_markers = cleanup_markers();
    assert_eq!(remaining_markers.len(), 1);
    assert_ne!(remaining_markers[0], old_marker);
    assert!(repository.join(".grove/FINISHING-finish-k2").is_dir());
    assert!(fs::read_dir(&control_directory).unwrap().all(|entry| !entry
        .unwrap()
        .file_name()
        .to_string_lossy()
        .contains(&old_attempt)));
}

#[test]
fn corrupt_in_tree_owner_leaves_every_cleanup_candidate_untouched() {
    use std::os::unix::fs::symlink;

    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-refuses-corrupt-cleanup-owner");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);

    let old_finish = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_CLEANUP_FAIL_AT", "before-root-removal")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(old_finish.status.success());
    let control_directory = repository.join(".git/grove");
    let old_marker = fs::read_dir(&control_directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("GROVE-FINISH-CLEANUP-")
        })
        .unwrap();

    seed_committed_terminal_grove(&repository);
    let current_finish = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env(
            "GROVE_TEST_FINISH_CLEANUP_FAIL_AT",
            "after-marker-publication",
        )
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(!current_finish.status.success());
    let witness = repository.join(".grove/FINISHING-finish-k2");
    let manifest = witness.join("MANIFEST.json");
    let external_manifest = fixture.path().join("external-manifest.json");
    fs::copy(&manifest, &external_manifest).unwrap();
    fs::remove_file(&manifest).unwrap();
    symlink(&external_manifest, &manifest).unwrap();

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
    assert!(diagnostic.contains("cleanup ownership"), "{diagnostic}");
    assert!(diagnostic.contains("MANIFEST.json"), "{diagnostic}");
    assert!(old_marker.is_file());
    assert!(witness.is_dir());
    assert!(!launch_log.exists());
}

#[test]
fn invalid_in_tree_attempt_identity_leaves_every_cleanup_candidate_untouched() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-refuses-invalid-cleanup-owner");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);

    let old_finish = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_CLEANUP_FAIL_AT", "before-root-removal")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(old_finish.status.success());

    seed_committed_terminal_grove(&repository);
    let current_finish = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env(
            "GROVE_TEST_FINISH_CLEANUP_FAIL_AT",
            "after-marker-publication",
        )
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(!current_finish.status.success());

    let control_directory = repository.join(".git/grove");
    let cleanup_markers = || {
        fs::read_dir(&control_directory)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("GROVE-FINISH-CLEANUP-")
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(cleanup_markers().len(), 2);

    let manifest_path = repository.join(".grove/FINISHING-finish-k2/MANIFEST.json");
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&manifest_path).unwrap()).unwrap();
    manifest["attempt_identity"] = serde_json::json!("not-a-128-bit-identity");
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .unwrap();

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, "sh -c true '${prompt}'");
    let blocked = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    assert!(!blocked.status.success());
    assert_eq!(
        cleanup_markers().len(),
        2,
        "invalid ownership must block all reaping"
    );
    let diagnostic = String::from_utf8_lossy(&blocked.stderr);
    assert!(diagnostic.contains("128-bit hexadecimal"), "{diagnostic}");
}

#[test]
fn bare_driver_reaps_orphans_and_ignores_abandoned_signals_in_jj_workspaces() {
    for (label, colocated) in [("native", false), ("colocated", true)] {
        let fixture = TempDir::new().unwrap();
        let repository = fixture.path().join(format!("{label}-jj-driver-reaping"));
        init_jj(&repository, colocated);
        seed_jj_terminal_grove(&repository);

        let interrupted = Command::cargo_bin("grove-llm")
            .unwrap()
            .current_dir(&repository)
            .env_remove("GROVE_SIGNAL_FILE")
            .env("GROVE_TEST_FINISH_CLEANUP_FAIL_AT", "before-root-removal")
            .args(["finish-commit", "finish-k2"])
            .output()
            .unwrap();
        assert!(
            interrupted.status.success(),
            "{label}: {}",
            String::from_utf8_lossy(&interrupted.stderr)
        );
        assert!(!repository.join(".grove").exists(), "{label}");

        let control_directory = repository.join(".jj/grove");
        let abandoned_signal = control_directory.join("signal-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        fs::write(&abandoned_signal, "done\n").unwrap();
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
            "{label}: {}",
            String::from_utf8_lossy(&recovered.stderr)
        );
        assert!(repository
            .join(".grove/01-requirements-plan-k1.md")
            .is_file());
        assert!(fs::read_to_string(&launch_log).unwrap().contains("plan-k1"));
        assert!(!abandoned_signal.exists(), "{label}");
        assert!(fs::read_dir(&control_directory).unwrap().all(|entry| {
            let name = entry.unwrap().file_name();
            let name = name.to_string_lossy();
            !name.starts_with("GROVE-FINISH-CLEANUP-")
                && !name.starts_with("FINISHED-")
                && !name.starts_with("REAPING-FINISHED-")
        }));
        let diagnostic = String::from_utf8_lossy(&recovered.stderr);
        assert!(
            diagnostic.contains("without a completion signal"),
            "{label}: {diagnostic}"
        );
        assert!(
            !diagnostic.contains("grove finished"),
            "{label}: {diagnostic}"
        );
    }
}

#[test]
fn linked_git_driver_reaps_its_auxiliary_without_following_an_ambient_index() {
    let fixture = TempDir::new().unwrap();
    let main_repository = fixture.path().join("main");
    init_git(&main_repository);
    fs::write(main_repository.join("README"), "fixture\n").unwrap();
    run("git", &main_repository, &["add", "README"]);
    run("git", &main_repository, &["commit", "-q", "-m", "base"]);
    let linked = fixture.path().join("linked");
    run(
        "git",
        &main_repository,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "cleanup-linked",
            linked.to_str().unwrap(),
        ],
    );
    seed_committed_terminal_grove(&linked);

    let failed = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&linked)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_FAIL_AT", "after-evacuation")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(!failed.status.success());
    assert!(linked.join(".grove/FINISHING-finish-k2").is_dir());
    let markers = auxiliary_markers(&linked);
    assert_eq!(markers.len(), 1);
    let actual_marker = markers[0].clone();
    let actual_artifact = auxiliary_artifact(&actual_marker);
    fs::remove_dir_all(linked.join(".grove")).unwrap();

    let foreign_directory = fixture.path().join("foreign-gitdir");
    fs::create_dir(&foreign_directory).unwrap();
    let foreign_index = foreign_directory.join("index");
    fs::copy(git_index_path(&linked), &foreign_index).unwrap();
    let foreign_artifact = foreign_directory.join(actual_artifact.file_name().unwrap());
    fs::hard_link(&actual_artifact, &foreign_artifact).unwrap();
    let foreign_marker = foreign_directory.join(actual_marker.file_name().unwrap());
    fs::copy(&actual_marker, &foreign_marker).unwrap();

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, "sh -c true '${prompt}'");
    let recovered = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&linked)
        .env("HOME", &home)
        .env("GIT_INDEX_FILE", &foreign_index)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    assert!(
        recovered.status.success(),
        "{}",
        String::from_utf8_lossy(&recovered.stderr)
    );
    assert!(!actual_marker.exists());
    assert!(!actual_artifact.exists());
    assert!(foreign_marker.is_file());
    assert!(foreign_artifact.is_file());
}

#[test]
fn colocated_jj_driver_preserves_owned_auxiliary_then_reaps_it_after_owner_removal() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("colocated-owned-auxiliary");
    init_jj(&repository, true);
    seed_jj_terminal_grove(&repository);

    let failed = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_FAIL_AT", "after-evacuation")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(!failed.status.success());
    assert!(repository.join(".grove/FINISHING-finish-k2").is_dir());
    let markers = auxiliary_markers(&repository);
    assert_eq!(markers.len(), 2);
    let artifacts = markers
        .iter()
        .map(|marker| auxiliary_artifact(marker))
        .collect::<Vec<_>>();

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, "sh -c true '${prompt}'");
    fs::write(repository.join("divergent"), "preserve\n").unwrap();
    run(
        "jj",
        &repository,
        &["commit", "-m", "divergent", "divergent"],
    );
    let blocked = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();
    assert!(!blocked.status.success());
    assert!(
        markers.iter().all(|marker| marker.is_file()),
        "the matching in-tree owner was ignored"
    );
    assert!(artifacts.iter().all(|artifact| artifact.is_file()));

    fs::remove_dir_all(repository.join(".grove")).unwrap();
    let reaped = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();
    assert!(
        reaped.status.success(),
        "{}",
        String::from_utf8_lossy(&reaped.stderr)
    );
    assert!(markers.iter().all(|marker| !marker.exists()));
    assert!(artifacts.iter().all(|artifact| !artifact.exists()));
}

#[test]
fn persistent_auxiliary_failure_warns_and_retries_without_blocking_the_driver() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("persistent-auxiliary-failure");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);

    let failed = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_FAIL_AT", "after-evacuation")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(!failed.status.success());
    fs::remove_dir_all(repository.join(".grove")).unwrap();
    let markers = auxiliary_markers(&repository);
    assert_eq!(markers.len(), 1);
    let marker = markers[0].clone();
    let artifact = auxiliary_artifact(&marker);
    let preserved = artifact.with_file_name("preserved-original-index-auxiliary");
    fs::rename(&artifact, &preserved).unwrap();
    fs::write(&artifact, "replacement\n").unwrap();

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, "sh -c true '${prompt}'");
    for invocation in 1..=2 {
        let output = Command::cargo_bin("grove")
            .unwrap()
            .current_dir(&repository)
            .env("HOME", &home)
            .env_remove("GROVE_SIGNAL_FILE")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "invocation {invocation}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let diagnostic = String::from_utf8_lossy(&output.stderr);
        assert!(
            diagnostic.contains("could not complete orphaned finish cleanup"),
            "invocation {invocation}: {diagnostic}"
        );
        assert!(
            diagnostic.contains("identity does not match"),
            "{diagnostic}"
        );
        assert!(marker.is_file());
        assert_eq!(fs::read_to_string(&artifact).unwrap(), "replacement\n");
        assert!(preserved.is_file());
    }
}

#[test]
fn persistent_cleanup_failure_warns_and_retries_without_blocking_fresh_lifecycle() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("persistent-driver-cleanup-failure");
    init_git(&repository);
    seed_committed_terminal_grove(&repository);

    let interrupted = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_CLEANUP_FAIL_AT", "before-root-removal")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();
    assert!(interrupted.status.success());
    let control_directory = repository.join(".git/grove");
    let marker = fs::read_dir(&control_directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("GROVE-FINISH-CLEANUP-")
        })
        .unwrap();
    let claimed = fs::read_dir(&control_directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("REAPING-FINISHED-")
        })
        .unwrap();
    let preserved = control_directory.join("preserved-original-quarantine");
    fs::rename(&claimed, &preserved).unwrap();
    fs::create_dir(&claimed).unwrap();
    fs::write(claimed.join("replacement"), "keep\n").unwrap();

    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let launch_log = fixture.path().join("launch-log");
    let script = fixture.path().join("record-session.sh");
    write_executable(
        &script,
        &format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> {}\n",
            launch_log.display()
        ),
    );
    let template = format!("sh {} '${{prompt}}'", script.display());
    write_complete_config(&home, &template);

    for invocation in 1..=2 {
        let output = Command::cargo_bin("grove")
            .unwrap()
            .current_dir(&repository)
            .env("HOME", &home)
            .env_remove("GROVE_SIGNAL_FILE")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "invocation {invocation}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let diagnostic = String::from_utf8_lossy(&output.stderr);
        assert!(
            diagnostic.contains("could not complete orphaned finish cleanup"),
            "invocation {invocation}: {diagnostic}"
        );
        assert!(
            diagnostic.contains("identity does not match"),
            "{diagnostic}"
        );
        assert!(marker.is_file());
        assert_eq!(
            fs::read_to_string(claimed.join("replacement")).unwrap(),
            "keep\n"
        );
        assert!(preserved.is_dir());
    }

    assert!(repository
        .join(".grove/01-requirements-plan-k1.md")
        .is_file());
    assert_eq!(
        fs::read_to_string(launch_log)
            .unwrap()
            .matches("Grove mandate:")
            .count(),
        2
    );
}
