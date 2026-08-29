// `grove-llm finish-commit`, against the **plain commit**.
//
// This file replaces `tests/finish_lifecycle.rs` (4,144 lines at its largest,
// 2,861 after the Git lane went), and the ratio is the point: almost all of that
// suite asserted a hand-built transaction — witness materialization states,
// evacuation and rollback, quarantine disposal, recovery from an interrupted
// attempt, lost-result proofs — and every one of those things is gone with the
// transaction that had them (`delete-finish-transaction-k8`). jj snapshots the
// working copy before every command and its operation log is the transaction
// record, so grove takes a commit and asserts nothing about undoing it.
//
// What is left is what only grove can say, and it is all still here: that the
// live leaf is the driver-owned finish leaf the caller named, that no ordinary
// work slipped in, that only `.grove/` is deleted and committed, and that every
// refusal leaves the tree standing and names the way out.

use assert_cmd::cargo::CommandCargoExt;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

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

fn jj_output(repository: &Path, arguments: &[&str]) -> String {
    String::from_utf8(run("jj", repository, arguments).stdout)
        .unwrap()
        .trim()
        .to_owned()
}

/// The revision the working copy sits on, and its description — read with
/// `--ignore-working-copy` so the reading cannot be the mutation it checks for.
fn parent_commit(repository: &Path) -> String {
    jj_output(
        repository,
        &[
            "--ignore-working-copy",
            "log",
            "-r",
            "@-",
            "--no-graph",
            "-T",
            "commit_id",
        ],
    )
}

fn parent_description(repository: &Path) -> String {
    jj_output(
        repository,
        &[
            "--ignore-working-copy",
            "log",
            "-r",
            "@-",
            "--no-graph",
            "-T",
            "description",
        ],
    )
}

/// `git.colocate` is forced either way rather than inherited, because an ambient
/// jj config may default it on and would turn every "native" fixture into a
/// colocated one.
fn init_jj(repository: &Path, colocated: bool) {
    fs::create_dir_all(repository).unwrap();
    if colocated {
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

fn init_repo(repository: &Path) {
    init_jj(repository, false);
}

/// A terminal grove: everything done but one live `finish` leaf, with the tree
/// tracked at `@-` and one unrelated working-copy change outside it.
fn seed_terminal_grove(repository: &Path) {
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("NOTES.md"), "notes\n").unwrap();
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

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Every path under `.grove/`, relative and sorted, with file bodies — enough to
/// catch a refusing verb that still wrote or removed something.
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
            if path.is_dir() {
                snapshot.push((relative, None));
                walk(root, &path, snapshot);
            } else {
                snapshot.push((relative, Some(fs::read(&path).unwrap())));
            }
        }
    }
    let mut snapshot = Vec::new();
    walk(root, root, &mut snapshot);
    snapshot
}

// ---------------------------------------------------------------------------
// What the commit is

/// The whole of the happy path: `.grove/` is deleted, one commit records that
/// deletion, and its message names the handle. Asserted on both jj shapes,
/// because the plain commit is the only lane there is and a colocated repo's Git
/// index is jj's business rather than grove's — nothing here backs it up or
/// restores it.
fn assert_finish_commit_records_only_the_teardown(colocated: bool) {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("repository");
    init_jj(&repository, colocated);
    seed_terminal_grove(&repository);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(!repository.join(".grove").exists(), "the tree survived");
    assert_eq!(
        parent_description(&repository),
        "finish-k2: remove completed grove task tree"
    );
    // Path-scoped: the commit holds `.grove/` deletions and nothing else.
    let changed = jj_output(
        &repository,
        &["--ignore-working-copy", "diff", "-r", "@-", "--summary"],
    );
    assert!(!changed.is_empty(), "the teardown commit is empty");
    for line in changed.lines() {
        assert!(
            line.starts_with("D .grove/"),
            "the teardown commit changed something outside `.grove/`: {changed}"
        );
    }
}

#[test]
fn native_jj_finish_commit_records_only_the_teardown() {
    assert_finish_commit_records_only_the_teardown(false);
}

#[test]
fn colocated_jj_finish_commit_records_only_the_teardown() {
    assert_finish_commit_records_only_the_teardown(true);
}

/// The reason the commit is fileset-scoped rather than a bare `jj commit`: a
/// session's unrelated working-copy edits are still uncommitted work, and
/// sweeping them into the teardown would commit on the operator's behalf.
fn assert_finish_commit_preserves_other_work(colocated: bool) {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("repository");
    init_jj(&repository, colocated);
    seed_terminal_grove(&repository);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);
    assert!(output.status.success(), "{}", stderr(&output));

    assert_eq!(
        fs::read_to_string(repository.join("outside.txt")).unwrap(),
        "after\n",
        "the unrelated working-copy edit was reverted"
    );
    let still_uncommitted = jj_output(&repository, &["--ignore-working-copy", "diff", "--summary"]);
    assert!(
        still_uncommitted.contains("outside.txt"),
        "the unrelated edit was swept into the teardown commit: {still_uncommitted:?}"
    );
}

#[test]
fn native_jj_finish_commit_preserves_unrelated_working_copy_changes() {
    assert_finish_commit_preserves_other_work(false);
}

#[test]
fn colocated_jj_finish_commit_preserves_unrelated_working_copy_changes() {
    assert_finish_commit_preserves_other_work(true);
}

// ---------------------------------------------------------------------------
// What the verb still revalidates
//
// These four are the *tree and VCS facts* the leaf that deleted the transaction
// deliberately kept. Each asserts the same two things: the refusal is legible,
// and the tree is untouched — a verb that refuses after deleting would have
// destroyed the thing it declined to act on.

#[test]
fn finish_commit_refuses_when_ordinary_work_appeared() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("repository");
    init_repo(&repository);
    seed_terminal_grove(&repository);
    let grove = repository.join(".grove");
    fs::write(grove.join("03-impl-late-k3.md"), "# late-k3\n").unwrap();
    let before = tree_snapshot(&grove);
    let head = parent_commit(&repository);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success(), "late work was torn down");
    let error = stderr(&output);
    assert!(
        error.contains("cannot finish while live work remains"),
        "{error}"
    );
    assert!(
        error.contains("late-k3"),
        "the refusal named no work: {error}"
    );
    assert_eq!(tree_snapshot(&grove), before);
    assert_eq!(parent_commit(&repository), head, "history moved");
}

#[test]
fn finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("repository");
    init_repo(&repository);
    seed_terminal_grove(&repository);
    let grove = repository.join(".grove");
    let before = tree_snapshot(&grove);

    let output = grove_llm(&repository, &["finish-commit", "finish-k99"]);

    assert!(
        !output.status.success(),
        "a foreign handle tore the tree down"
    );
    let error = stderr(&output);
    assert!(error.contains("finish-k99"), "{error}");
    assert!(error.contains("finish-k2"), "{error}");
    assert_eq!(tree_snapshot(&grove), before);
}

/// A `.grove` symlinked at a directory elsewhere is refused **unfollowed**. The
/// transaction refused it because it opened the root no-follow; the plain commit
/// refuses it because a verb that deletes a tree may not delete one that is not
/// its own.
#[test]
fn finish_commit_refuses_a_symlinked_task_root_before_deleting_anything() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("repository");
    init_repo(&repository);
    let elsewhere = fixture.path().join("elsewhere");
    fs::create_dir_all(&elsewhere).unwrap();
    fs::write(elsewhere.join("BRIEF.md"), "# elsewhere — brief\n").unwrap();
    fs::write(elsewhere.join("02-finish-finish-k2.md"), "# finish-k2\n").unwrap();
    std::os::unix::fs::symlink(&elsewhere, repository.join(".grove")).unwrap();

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success(), "a symlinked root was torn down");
    assert!(
        stderr(&output).contains("grove root is not a directory"),
        "{}",
        stderr(&output)
    );
    assert!(
        elsewhere.join("BRIEF.md").exists(),
        "the target was deleted"
    );
}

/// The one precondition the deletion has, and the reason it is not a surviving
/// piece of the transaction: jj can only restore what it tracks, so an untracked
/// tree is refused rather than deleted into a state no operation-log command
/// could undo.
#[test]
fn finish_commit_refuses_an_untracked_task_tree_naming_how_to_track_it() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("repository");
    init_repo(&repository);
    fs::write(repository.join(".gitignore"), ".grove/\n").unwrap();
    run("jj", &repository, &["commit", "-m", "ignore the grove"]);
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# untracked — brief\n").unwrap();
    fs::write(grove.join("01-finish-finish-k1.md"), "# finish-k1\n").unwrap();
    let before = tree_snapshot(&grove);

    let output = grove_llm(&repository, &["finish-commit", "finish-k1"]);

    assert!(!output.status.success(), "an untracked tree was deleted");
    let error = stderr(&output);
    assert!(error.contains("could not be undone"), "{error}");
    assert!(
        error.contains("jj commit"),
        "the refusal named no remedy: {error}"
    );
    assert_eq!(tree_snapshot(&grove), before);
}

// ---------------------------------------------------------------------------
// What a failure says now

/// Task-root absence used to be routed to a proof that the repository's
/// immediate result *was* this attempt's teardown commit, because a death inside
/// the transaction exposed exactly that shape. There is no transaction to die
/// inside, so absence is a plain refusal — and the remedy it names is the
/// operation log's, not grove's.
#[test]
fn finish_commit_on_an_absent_tree_names_the_operation_log() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("repository");
    init_repo(&repository);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("no Grove task tree"), "{error}");
    assert!(error.contains("jj op log"), "{error}");
    assert!(error.contains("jj undo"), "{error}");
}

/// The precondition gate is the seam's, and it runs before anything is removed.
#[test]
fn finish_commit_refuses_a_working_tree_that_is_not_jj_enabled() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("repository");
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# no-vcs — brief\n").unwrap();
    fs::write(grove.join("01-finish-finish-k1.md"), "# finish-k1\n").unwrap();
    let before = tree_snapshot(&grove);

    let output = grove_llm(&repository, &["finish-commit", "finish-k1"]);

    assert!(!output.status.success(), "a VCS-less tree was torn down");
    let error = stderr(&output);
    assert!(error.contains("not a Jujutsu working tree"), "{error}");
    assert_eq!(tree_snapshot(&grove), before);
}
