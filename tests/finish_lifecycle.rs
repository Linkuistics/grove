use assert_cmd::cargo::CommandCargoExt;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::os::unix::process::ExitStatusExt;
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

/// The literal first argument the **configuration** hands a fixture script for
/// the `finish` kind, and only for it. Its counterpart is [`ORDINARY_ROUTE`];
/// both are supplied by [`write_config_routing_finish_apart`].
///
/// **A fixture must not recover the session kind from `${prompt}`.** Kind is what
/// Grove routes on and configuration is keyed by it, so a per-kind template is
/// the direct discriminator — while every candidate in the prompt is indirect and
/// unsound. `*finish-k*` matches the mandate of *every* kind, because composed
/// methodology names `finish-k<key>` in the prose explaining the sentinel. Even
/// the driver's own sentence naming the **selected** leaf only identifies a
/// handle whose slug is `finish`: the slug rule reserves `BRIEF` and `DONE` and
/// not `finish` (`src/task_name.rs`), so an ordinary `NN-impl-finish-k42.md` is a
/// legal leaf whose mandate would send a non-`finish` configured session down the
/// teardown branch.
const FINISH_ROUTE: &str = "route=finish";

/// The literal first argument the configuration hands a fixture script for the
/// eighteen non-`finish` kinds. Distinct from [`FINISH_ROUTE`] rather than
/// absent, so a script's fall-through case is reached by a positive value and a
/// misrouted launch cannot look like an unrouted one.
const ORDINARY_ROUTE: &str = "route=ordinary";

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

/// Deliberately **not** `support::readiness`, which is the one home of every
/// readiness wait that has a producer to condition on. Its sole caller waits on
/// a file written by an *orphaned grandchild* after this test reaped the driver
/// between them, so there is no live handle to sample and a budget is all that
/// is left (loop-driver-readiness-deadline-k170).
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

/// Block until a paused process's barrier carries its payload, and answer with
/// it.
///
/// The seam publishes the payload in one step, so an existing barrier already
/// carries every byte. Waiting on the payload rather than on existence keeps
/// that guarantee checked from this side too: a barrier that ever appeared
/// empty here would be a publisher that had gone back to writing through the
/// live path, and the paused step's name is what the caller came for.
fn wait_for_barrier(path: &Path) -> Vec<u8> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match fs::read(path) {
            Ok(payload) if !payload.is_empty() => return payload,
            Ok(_) | Err(_) => assert!(
                Instant::now() < deadline,
                "timed out waiting for the payload of {}",
                path.display()
            ),
        }
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

/// The revision the working copy sits on, and its description — jj's answers to
/// `git rev-parse HEAD` and `git log -1 --pretty=%s`, which are what "the
/// refused run changed no history" is asserted against. Read with
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

fn write_executable(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

/// The ordinary fixture: a native jj workspace.
fn init_repo(repository: &Path) {
    init_jj(repository, false);
}

/// `git.colocate` is forced either way rather than inherited, because an
/// ambient jj config may default it on and would turn every "native" fixture
/// into a colocated one.
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

fn seed_committed_terminal_grove(repository: &Path) {
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("NOTES.md"), "notes\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# finish-test — brief\n").unwrap();
    fs::write(grove.join("01-DONE-impl-finished-k1.md"), "# finished-k1\n").unwrap();
    fs::write(repository.join("kept.txt"), "kept\n").unwrap();
    run("jj", repository, &["commit", "-m", "fixture"]);
    fs::write(
        grove.join("02-finish-finish-k2.md"),
        "# finish-k2\n\n## Goal\n\nFinish.\n",
    )
    .unwrap();
}

fn seed_jj_terminal_grove(repository: &Path) {
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

/// A complete config in which one `sh` script serves every kind, told which kind
/// it was launched for by its **own first argument** — [`FINISH_ROUTE`] for
/// `finish`, [`ORDINARY_ROUTE`] for the other eighteen — with `${prompt}` second.
///
/// This is the routing Grove actually performs, observed at the seam it performs
/// it on: the driver reads the kind from the selected leaf's filename and looks up
/// *that kind's* complete command template, so a fixture that needs to know which
/// kind ran asks the configuration to tell it rather than parsing the payload.
fn write_config_routing_finish_apart(home: &Path, script: &Path) {
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    let document = SESSION_KINDS
        .iter()
        .map(|kind| {
            let route = if *kind == "finish" {
                FINISH_ROUTE
            } else {
                ORDINARY_ROUTE
            };
            let template = format!("sh {} {route} '${{prompt}}'", script.display());
            format!("{kind} {template:?}\n")
        })
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
fn finish_commit_refuses_byte_identically_when_ordinary_work_appeared() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("work-appeared");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    fs::write(
        repository.join(".grove/03-impl-late-work-k3.md"),
        "# late-work-k3\n",
    )
    .unwrap();
    let before = tree_snapshot(&repository.join(".grove"));
    let head_before = parent_commit(&repository);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("late-work-k3"));
    assert_eq!(tree_snapshot(&repository.join(".grove")), before);
    assert_eq!(parent_commit(&repository), head_before);
}

/// Task-root absence proves nothing: with no teardown result to verify, a
/// rootless retry is a refusal, and a refusal never licenses `complete --done`.
///
/// A repository with nothing but its root commit is the degenerate case, and it
/// must read as Grove's own statement about what it needed rather than as a
/// leaked VCS usage error.
#[test]
fn rootless_finish_retry_refuses_when_no_teardown_result_exists() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("already-finished");
    init_repo(&repository);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no Grove task tree") && stderr.contains("finish-k2"),
        "{stderr}"
    );
    assert!(
        stderr.contains("is not this finish attempt's teardown commit"),
        "the empty repository must be told what was required: {stderr}"
    );
    assert!(
        !stderr.contains("Usage:") && !stderr.contains("error: unexpected"),
        "the empty repository leaked a raw VCS diagnostic: {stderr}"
    );
}

/// A near-miss is a different answer from "nothing to verify", so it names both
/// halves of the comparison it failed. The teardown commit's identity is its
/// message, so that check comes before any structural one — otherwise a
/// repository whose HEAD happens to be a root commit is told about its
/// ancestry when what it needs to know is that this is not the right commit.
#[test]
fn rootless_finish_retry_names_the_message_it_required_and_the_one_it_observed() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("near-miss");
    init_repo(&repository);
    fs::write(repository.join("unrelated.txt"), "unrelated\n").unwrap();
    run("jj", &repository, &["commit", "-m", "unrelated work"]);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("expected message") && stderr.contains("remove completed grove task tree"),
        "{stderr}"
    );
    assert!(stderr.contains(r#"observed "unrelated work""#), "{stderr}");
}

/// What a session that lost its `finish-commit` result does: run it again.
///
/// Both runs share the launch's signal nonce, so they share one finish attempt
/// identity — which is the whole basis of the retry proof. Returns each run's
/// exit status and output plus the driver's own result.
struct LostResultRun {
    first_status: String,
    first_output: String,
    second_status: String,
    second_output: String,
    driver: Output,
}

fn run_lost_finish_result_session(fixture: &Path, repository: &Path) -> LostResultRun {
    let session_log = fixture.join("session");
    fs::create_dir_all(&session_log).unwrap();
    let script = fixture.join("lost-finish-session.sh");
    write_executable(
        &script,
        &format!(
            r#"#!/bin/sh
{llm} finish-commit finish-k2 > {log}/first-out 2>&1
printf '%s\n' "$?" > {log}/first-status
{llm} finish-commit finish-k2 > {log}/second-out 2>&1
printf '%s\n' "$?" > {log}/second-status
{llm} complete --done
"#,
            llm = env!("CARGO_BIN_EXE_grove-llm"),
            log = session_log.display(),
        ),
    );
    let home = fixture.join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, &format!("sh {} '${{prompt}}'", script.display()));

    let driver = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    let read = |name: &str| fs::read_to_string(session_log.join(name)).unwrap();
    LostResultRun {
        first_status: read("first-status").trim().to_owned(),
        first_output: read("first-out"),
        second_status: read("second-status").trim().to_owned(),
        second_output: read("second-out"),
        driver,
    }
}

fn assert_lost_result_retry_was_idempotent(run: &LostResultRun) {
    assert_eq!(
        run.first_status, "0",
        "the first finish-commit failed: {}",
        run.first_output
    );
    assert_eq!(
        run.second_status, "0",
        "the retry did not verify its own lost result: {}",
        run.second_output
    );
    assert!(
        run.driver.status.success(),
        "{}",
        String::from_utf8_lossy(&run.driver.stderr)
    );
}

fn assert_lost_jj_finish_result_retry_is_idempotent(colocated: bool) {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join(if colocated {
        "lost-colocated-jj"
    } else {
        "lost-native-jj"
    });
    init_jj(&repository, colocated);
    seed_jj_terminal_grove(&repository);
    fs::remove_file(repository.join(".grove/02-finish-finish-k2.md")).unwrap();

    let run = run_lost_finish_result_session(fixture.path(), &repository);

    assert_lost_result_retry_was_idempotent(&run);
    assert!(!repository.join(".grove").exists());
    let teardown = jj_output(
        &repository,
        &["log", "-r", "@-", "--no-graph", "-T", "description"],
    );
    assert!(
        teardown.starts_with("finish-k2 (finish attempt"),
        "{teardown}"
    );
    let grandparent = jj_output(
        &repository,
        &["log", "-r", "@--", "--no-graph", "-T", "description"],
    );
    assert!(
        !grandparent.contains("finish-k2"),
        "the retry made a second teardown commit: {grandparent}"
    );
}

#[test]
fn lost_native_jj_finish_result_makes_the_same_launch_retry_idempotent() {
    assert_lost_jj_finish_result_retry_is_idempotent(false);
}

#[test]
fn lost_colocated_jj_finish_result_makes_the_same_launch_retry_idempotent() {
    assert_lost_jj_finish_result_retry_is_idempotent(true);
}

fn drive_finish_session(
    fixture: &Path,
    repository: &Path,
    home: &Path,
    script_body: &str,
) -> Output {
    let script = fixture.join("finish-session.sh");
    write_executable(&script, script_body);
    write_complete_config(home, &format!("sh {} '${{prompt}}'", script.display()));
    Command::cargo_bin("grove")
        .unwrap()
        .current_dir(repository)
        .env("HOME", home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap()
}

/// A completed grove's teardown commit belongs to the launch that made it.
///
/// Stable handles are identities only within one task tree, so a later grove in
/// the same working tree may reuse `finish-k2` — but its epoch draws a fresh
/// attempt identity, and the older commit was made under a different one. An
/// external root removal puts the new session in exactly the rootless shape the
/// retry proof answers, and it must still refuse.
#[test]
fn rootless_finish_retry_refuses_a_teardown_result_from_another_finish_attempt() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("reused-handle");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    fs::remove_file(repository.join(".grove/02-finish-finish-k2.md")).unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let llm = env!("CARGO_BIN_EXE_grove-llm");

    let first = drive_finish_session(
        fixture.path(),
        &repository,
        &home,
        &format!("#!/bin/sh\n{llm} finish-commit finish-k2 || exit $?\n{llm} complete --done\n"),
    );
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let teardown = parent_commit(&repository);

    let session_log = fixture.path().join("session");
    fs::create_dir(&session_log).unwrap();
    let second = drive_finish_session(
        fixture.path(),
        &repository,
        &home,
        &format!(
            r#"#!/bin/sh
rm -rf {repo}/.grove
{llm} finish-commit finish-k2 > {log}/out 2>&1
printf '%s\n' "$?" > {log}/status
{llm} complete --done
"#,
            repo = repository.display(),
            log = session_log.display(),
        ),
    );
    assert!(
        second.status.success(),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );

    let out = fs::read_to_string(session_log.join("out")).unwrap();
    assert_ne!(
        fs::read_to_string(session_log.join("status"))
            .unwrap()
            .trim(),
        "0",
        "an older grove's teardown satisfied a new epoch's confirmed session: {out}"
    );
    assert!(out.contains("finish attempt"), "{out}");
    assert_eq!(
        parent_commit(&repository),
        teardown,
        "the refused retry changed repository history"
    );
}

/// The proof is over the *immediate* result. Anything committed after teardown
/// moves it out of that position, and the retry then has nothing to verify —
/// rather than reaching back through history for a commit that once matched.
#[test]
fn rootless_finish_retry_refuses_a_teardown_result_that_is_no_longer_immediate() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("not-immediate");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    fs::remove_file(repository.join(".grove/02-finish-finish-k2.md")).unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let session_log = fixture.path().join("session");
    fs::create_dir(&session_log).unwrap();

    let driver = drive_finish_session(
        fixture.path(),
        &repository,
        &home,
        &format!(
            r#"#!/bin/sh
{llm} finish-commit finish-k2 > {log}/first-out 2>&1
printf '%s\n' "$?" > {log}/first-status
jj -R {repo} commit -m 'later work'
{llm} finish-commit finish-k2 > {log}/second-out 2>&1
printf '%s\n' "$?" > {log}/second-status
{llm} complete --done
"#,
            llm = env!("CARGO_BIN_EXE_grove-llm"),
            repo = repository.display(),
            log = session_log.display(),
        ),
    );
    assert!(
        driver.status.success(),
        "{}",
        String::from_utf8_lossy(&driver.stderr)
    );

    let read = |name: &str| fs::read_to_string(session_log.join(name)).unwrap();
    assert_eq!(read("first-status").trim(), "0", "{}", read("first-out"));
    let second = read("second-out");
    assert_ne!(
        read("second-status").trim(),
        "0",
        "a superseded teardown result licensed the retry: {second}"
    );
    assert!(second.contains("later work"), "{second}");
    assert_eq!(
        parent_description(&repository),
        "later work",
        "the refused retry changed repository history"
    );
}

/// Amend the teardown commit inside the launch that made it, so the forged
/// result keeps the exact message — handle *and* this launch's attempt identity
/// — and differs only in the shape the proof also requires. The message alone is
/// not the proof.
///
/// `amendment` runs in the repository and owns its own `jj squash`: squashing
/// the working copy into its parent is jj's `commit --amend`, and it keeps the
/// parent's description, which is the whole point. The still-tracked case has
/// work to do *after* the squash, so the squash cannot live in this template.
fn run_amended_teardown_retry(
    fixture: &Path,
    repository: &Path,
    amendment: &str,
) -> (String, String) {
    init_repo(repository);
    seed_committed_terminal_grove(repository);
    fs::remove_file(repository.join(".grove/02-finish-finish-k2.md")).unwrap();
    let home = fixture.join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let session_log = fixture.join("session");
    fs::create_dir(&session_log).unwrap();

    let driver = drive_finish_session(
        fixture,
        repository,
        &home,
        &format!(
            r#"#!/bin/sh
{llm} finish-commit finish-k2 > {log}/first-out 2>&1
printf '%s\n' "$?" > {log}/first-status
cd {repo} || exit 1
{amendment}
{llm} finish-commit finish-k2 > {log}/second-out 2>&1
printf '%s\n' "$?" > {log}/second-status
{llm} complete --done
"#,
            llm = env!("CARGO_BIN_EXE_grove-llm"),
            repo = repository.display(),
            log = session_log.display(),
        ),
    );
    assert!(
        driver.status.success(),
        "{}",
        String::from_utf8_lossy(&driver.stderr)
    );
    let read = |name: &str| fs::read_to_string(session_log.join(name)).unwrap();
    assert_eq!(read("first-status").trim(), "0", "{}", read("first-out"));
    assert!(
        parent_description(repository).starts_with("finish-k2 (finish attempt"),
        "the amendment did not preserve the exact teardown message"
    );
    (read("second-status").trim().to_owned(), read("second-out"))
}

#[test]
fn rootless_finish_retry_refuses_a_matching_message_that_changes_paths_outside_the_grove() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("outside-the-grove");

    let (status, output) = run_amended_teardown_retry(
        fixture.path(),
        &repository,
        "printf 'outside\\n' > outside.txt\njj squash",
    );

    assert_ne!(
        status, "0",
        "a teardown result that also changed unrelated paths licensed the retry: {output}"
    );
    assert!(
        output.contains("outside the exact `.grove/` deletion"),
        "{output}"
    );
}

#[test]
fn rootless_finish_retry_refuses_a_matching_message_that_still_tracks_the_grove() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("still-tracked");

    // Resurrect one task-tree path, so every *changed* path is still a `.grove/`
    // deletion and only the surviving tracked entry separates this from success.
    let (status, output) = run_amended_teardown_retry(
        fixture.path(),
        &repository,
        "mkdir -p .grove\nprintf 'notes\\n' > .grove/NOTES.md\njj squash\nrm -rf .grove",
    );

    assert_ne!(
        status, "0",
        "a teardown result that still tracks `.grove/` licensed the retry: {output}"
    );
    assert!(output.contains("still tracks `.grove/`"), "{output}");
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

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!repository.join(".grove").exists());
    let committed = jj_output(&repository, &["diff", "-r", "@-", "--summary"]);
    assert!(committed.contains(".grove/NOTES.md"));
    assert!(!committed.contains("outside.txt"));
    let successor = jj_output(&repository, &["diff", "-r", "@", "--summary"]);
    assert!(successor.contains("outside.txt"));
    let description = jj_output(
        &repository,
        &["log", "-r", "@-", "--no-graph", "-T", "description"],
    );
    assert!(description.contains("finish-k2"));
}

fn jj_output(repository: &Path, arguments: &[&str]) -> String {
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

    let grove = repository.join(".grove");
    let tree_before = tree_snapshot(&grove);
    let commit_before = jj_output(
        &repository,
        &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
    );
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
        jj_output(
            &repository,
            &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
        ),
        commit_before
    );
}

#[test]
fn failed_native_jj_finish_restores_the_tree_and_preflight_commit() {
    assert_failed_jj_finish_restores_the_tree_and_repository(false);
}

#[test]
fn failed_colocated_jj_finish_restores_the_tree_and_preflight_commit() {
    assert_failed_jj_finish_restores_the_tree_and_repository(true);
}

#[test]
fn an_untracked_grove_finish_is_refused_before_deleting_the_tree() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("untracked-grove");
    init_repo(&repository);
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("NOTES.md"), "notes\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# untracked-grove — brief\n").unwrap();
    fs::write(
        grove.join("01-finish-finish-k1.md"),
        "# finish-k1\n\n## Goal\n\nFinish.\n",
    )
    .unwrap();
    let before = tree_snapshot(&grove);

    let output = grove_llm(&repository, &["finish-commit", "finish-k1"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("has no tracked state"), "{stderr}");
    assert_eq!(tree_snapshot(&grove), before);
}

#[test]
fn finish_preflight_refuses_special_entries_before_deleting_the_tree() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("special-entry");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    fs::write(repository.join("unrelated.txt"), "unrelated\n").unwrap();
    let socket_path = repository.join(".grove/runtime.sock");
    let _listener = UnixListener::bind(&socket_path).unwrap();
    let head_before = parent_commit(&repository);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported task-tree entry"), "{stderr}");
    assert!(stderr.contains("runtime.sock"), "{stderr}");
    assert!(repository.join(".grove").is_dir());
    assert!(socket_path.exists());
    assert_eq!(parent_commit(&repository), head_before);
    assert_eq!(
        fs::read_to_string(repository.join("unrelated.txt")).unwrap(),
        "unrelated\n",
        "a refused finish must leave unrelated working-copy bytes alone"
    );
}

/// A reserved finish-transaction path already in the root is refused before
/// anything is deleted, and the refusal is now the **tree guard's**.
///
/// `lifecycle-k35` flipped `finish-commit` onto `ordinal-fs-tree`'s exclusive
/// guard, and `task_name` classifies `FINISHING-` and `PREPARING-FINISH-` names
/// `Verdict::Reserved` — so the library halts the tree on one wherever it sits,
/// carrying grove's own `TaskNameError` wording. That is the same condition
/// `preflight_root`'s *reserved finish transaction path* stated, one layer
/// further out, which is why this assertion moved rather than weakened:
/// `docs/ARCHITECTURE.md#library-refusals`, clause 3 — no second wording for a
/// condition the library already states. The preflight check itself stays, as
/// defence against a writer that ignored the lock, and it re-reads the root
/// through its own `O_NOFOLLOW` descriptor rather than by path.
#[test]
fn finish_commit_refuses_a_reserved_witness_collision_before_deletion() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("reserved-collision");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    let collision = repository.join(".grove/FINISHING-finish-k2");
    fs::create_dir(&collision).unwrap();
    fs::write(collision.join("foreign"), "keep\n").unwrap();
    let head_before = parent_commit(&repository);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("pending Grove finish transaction"),
        "{stderr}"
    );
    assert!(stderr.contains("FINISHING-finish-k2"), "{stderr}");
    assert_eq!(
        fs::read_to_string(collision.join("foreign")).unwrap(),
        "keep\n"
    );
    assert_eq!(parent_commit(&repository), head_before);
}

/// A task root that is a symlink rather than a real directory is refused before
/// any mutation. The paired *replaced*-root case lives in the transaction's own
/// unit tests, where the replacement can be timed against a held descriptor;
/// this one is reachable from the process seam because the shape exists before
/// the first open. Neither the link nor its target may be followed, moved, or
/// deleted — `.grove/` addressing a directory elsewhere is exactly the shape a
/// no-follow transaction must not treat as its own tree.
#[test]
fn finish_preflight_refuses_a_symlinked_task_root_before_deleting_the_tree() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("symlinked-root");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    let grove_root = repository.join(".grove");
    let store = repository.join("grove-store");
    fs::rename(&grove_root, &store).unwrap();
    std::os::unix::fs::symlink("grove-store", &grove_root).unwrap();
    let head_before = parent_commit(&repository);

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("grove root is not a directory"), "{stderr}");
    assert!(stderr.contains(".grove"), "{stderr}");
    assert!(fs::symlink_metadata(&grove_root)
        .unwrap()
        .file_type()
        .is_symlink());
    assert!(store.join("02-finish-finish-k2.md").is_file());
    assert!(store.join("NOTES.md").is_file());
    assert_eq!(parent_commit(&repository), head_before);
}

#[test]
fn tree_readers_refuse_a_ready_finish_transaction() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("ready-finish-transaction");
    init_repo(&repository);
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

/// The *unpublished* half of the same reservation. A preparing witness holds no
/// evacuated entry, so the tree beside it still looks perfectly walkable — which
/// is exactly why the refusal has to be by reserved prefix rather than by
/// whether the tree looks intact. A reader admitted here would be reading a tree
/// whose repository preparation is already in flight.
#[test]
fn tree_readers_refuse_a_preparing_finish_transaction() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("preparing-finish-transaction");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    let preparing = repository
        .join(".grove")
        .join("PREPARING-FINISH-finish-k2-11111111111111111111111111111111");
    fs::create_dir(&preparing).unwrap();

    let output = grove_llm(&repository, &["pick"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("pending Grove finish transaction"),
        "{stderr}"
    );
    assert!(stderr.contains("PREPARING-FINISH-finish-k2"), "{stderr}");
}

#[test]
fn configured_finish_target_commits_teardown_then_stops_the_loop_cleanly() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("clean-stop");
    init_repo(&repository);
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
    assert!(parent_description(&repository).contains("finish-k2"));
}

/// **The fixtures' own discriminator, under the leaf that defeats every prompt
/// substring.** `finish` is not a reserved slug (`validate_slug` reserves `BRIEF`
/// and `DONE`), so `01-impl-finish-k1.md` is an ordinary work item whose stable
/// handle is `finish-k1` — and every candidate marker inside `${prompt}` matches
/// it, the bare `*finish-k*` glob and the driver's own sentence naming the
/// selected leaf alike.
///
/// This is a claim about the **test harness**, and it belongs in the suite that
/// depends on it, because a misrouted fixture does not fail: it runs the teardown
/// branch and then asserts happily about a session that never happened.
///
/// The discriminating assertion is therefore the **launch log** — under a prompt
/// substring it stays empty, because the teardown branch consumed the launch. The
/// surviving leaf is a weaker claim deliberately kept: `finish-commit` refuses a
/// selection whose kind is not `finish` (`src/tree_lifecycle.rs`, *cannot finish
/// while live work remains*), so a misroute here is a lying fixture rather than a
/// destroyed grove, and this pins that second line of defence rather than the
/// routing.
#[test]
fn the_finish_route_is_not_taken_by_an_ordinary_leaf_whose_slug_is_finish() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("ordinary-finish-slug");
    init_repo(&repository);
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("NOTES.md"), "notes\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# ordinary-finish-slug — brief\n").unwrap();
    fs::write(grove.join("01-impl-finish-k1.md"), "# finish-k1\n").unwrap();
    run("jj", &repository, &["commit", "-m", "fixture"]);
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let launch_log = fixture.path().join("launch-log");
    let script = fixture.path().join("route.sh");
    write_executable(
        &script,
        &format!(
            "#!/bin/sh\ncase \"$1\" in {finish}) {llm} finish-commit finish-k1; exit $?;; esac\nprintf '%s\\n' \"$2\" > {log}\n",
            finish = FINISH_ROUTE,
            llm = env!("CARGO_BIN_EXE_grove-llm"),
            log = launch_log.display(),
        ),
    );
    write_config_routing_finish_apart(&home, &script);

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
    let mandate = fs::read_to_string(&launch_log).unwrap_or_default();
    assert!(
        mandate.contains("finish-k1"),
        "the ordinary route must have run and received the mandate: {mandate:?}"
    );
    assert!(
        grove.join("01-impl-finish-k1.md").is_file(),
        "an ordinary leaf was sent down the teardown branch"
    );
}

/// Teardown succeeds, then the configured child dies without signalling. The
/// deleted tree is the *one* piece of evidence that could tempt a driver into
/// inferring `done` — root absence is what a finished grove looks like — so the
/// interesting assertion is that nothing about this run is special: it takes the
/// ordinary no-signal path, reporting the child's real status and elapsed time
/// and stopping. The next invocation then reads that same absence as an ordinary
/// fresh grove rather than as a finish to resume.
#[test]
fn a_no_signal_exit_after_successful_teardown_stops_and_then_starts_a_fresh_grove() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("teardown-then-no-signal");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    fs::remove_file(repository.join(".grove/02-finish-finish-k2.md")).unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let launch_log = fixture.path().join("launch-log");
    let script = fixture.path().join("teardown-then-die.sh");
    write_executable(
        &script,
        &format!(
            "#!/bin/sh\ncase \"$1\" in {finish}) {llm} finish-commit finish-k2 || exit $?; exit 23;; esac\nprintf '%s\\n' \"$2\" > {log}\n",
            finish = FINISH_ROUTE,
            llm = env!("CARGO_BIN_EXE_grove-llm"),
            log = launch_log.display(),
        ),
    );
    write_config_routing_finish_apart(&home, &script);

    let stopped = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    assert!(
        stopped.status.success(),
        "{}",
        String::from_utf8_lossy(&stopped.stderr)
    );
    let stderr = String::from_utf8_lossy(&stopped.stderr);
    assert!(stderr.contains("status exit status: 23"), "{stderr}");
    assert!(stderr.contains("elapsed "), "{stderr}");
    assert!(!stderr.contains("grove finished"), "{stderr}");
    assert!(!repository.join(".grove").exists());
    assert!(parent_description(&repository).contains("finish-k2"));
    assert!(
        !launch_log.exists(),
        "the driver relaunched after no signal"
    );

    let restarted = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    assert!(
        restarted.status.success(),
        "{}",
        String::from_utf8_lossy(&restarted.stderr)
    );
    assert!(repository
        .join(".grove/01-requirements-plan-k1.md")
        .is_file());
    assert!(fs::read_to_string(&launch_log).unwrap().contains("plan-k1"));
}

/// The one post-commit shape that could pass for a durable finish receipt: the
/// child committed teardown and left `done` in the signal channel, and no driver
/// ever interpreted it. The child makes that deterministic rather than raced —
/// it kills its parent and waits for the pid to be reaped *before* writing — so
/// the abandoned `done` provably outlives the only process entitled to read it.
/// What the replacement must do is not read it at all: exclusive epoch handoff
/// and channel cleanup happen at lease acquisition, above every lifecycle
/// mutation, so the absent task root is classified as the fresh grove it is.
#[test]
fn a_done_signal_abandoned_by_a_killed_driver_reinitializes_instead_of_finishing() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("done-signal-driver-death");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    fs::remove_file(repository.join(".grove/02-finish-finish-k2.md")).unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();

    let launch_log = fixture.path().join("launch-log");
    let signal_log = fixture.path().join("signal-path");
    let signalled = fixture.path().join("signalled");
    let script = fixture.path().join("teardown-then-kill-the-driver.sh");
    write_executable(
        &script,
        &format!(
            r#"#!/bin/sh
case "$1" in
{finish})
  {llm} finish-commit finish-k2 || exit $?
  printf '%s\n' "$GROVE_SIGNAL_FILE" > {signal_log}
  driver=$PPID
  kill -9 "$driver"
  while kill -0 "$driver" 2>/dev/null; do sleep 0.01; done
  printf 'done\n' > "$GROVE_SIGNAL_FILE"
  : > {signalled}
  exit 0
  ;;
esac
printf '%s\n' "$2" > {launch_log}
"#,
            finish = FINISH_ROUTE,
            llm = env!("CARGO_BIN_EXE_grove-llm"),
            signal_log = signal_log.display(),
            signalled = signalled.display(),
            launch_log = launch_log.display(),
        ),
    );
    write_config_routing_finish_apart(&home, &script);

    // Spawn rather than `output()`. The child polls until its parent's pid stops
    // answering `kill -0`, and a SIGKILLed process keeps answering while it is
    // an unreaped zombie. `output()` reaps only after draining both pipes to
    // EOF, which this still-waiting child holds open — so the test itself must
    // be the reaper, or the two deadlock.
    let mut driver = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let killed = driver.wait().unwrap();
    wait_for(&signalled);

    assert_eq!(
        killed.signal(),
        Some(libc::SIGKILL),
        "the driver was not killed mid-launch"
    );
    assert!(!repository.join(".grove").exists());
    assert!(parent_description(&repository).contains("finish-k2"));

    let abandoned_signal = PathBuf::from(fs::read_to_string(&signal_log).unwrap().trim());
    assert_eq!(fs::read_to_string(&abandoned_signal).unwrap(), "done\n");
    let epoch_path = repository.join(".jj/grove/session.epoch");
    let abandoned_epoch = fs::read_to_string(&epoch_path).unwrap();
    assert!(
        abandoned_epoch.starts_with("state=active\n"),
        "the killed driver did not leave its launch epoch active: {abandoned_epoch:?}"
    );
    assert!(
        abandoned_epoch.contains("signal-path-hex="),
        "{abandoned_epoch:?}"
    );

    let restarted = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    assert!(
        restarted.status.success(),
        "{}",
        String::from_utf8_lossy(&restarted.stderr)
    );
    let diagnostic = String::from_utf8_lossy(&restarted.stderr);
    assert!(!diagnostic.contains("grove finished"), "{diagnostic}");
    assert!(
        repository
            .join(".grove/01-requirements-plan-k1.md")
            .is_file(),
        "{diagnostic}"
    );
    assert!(fs::read_to_string(&launch_log).unwrap().contains("plan-k1"));
    assert!(
        !abandoned_signal.exists(),
        "the abandoned completion channel survived crash handoff"
    );
    assert!(
        control_entries_starting_with(repository.join(".jj/grove").as_path(), "signal-")
            .is_empty()
    );
    let handed_over = fs::read_to_string(&epoch_path).unwrap();
    assert!(
        handed_over.starts_with("state=inactive\n"),
        "{handed_over:?}"
    );
    assert!(!handed_over.contains("signal-path-hex="), "{handed_over:?}");
}

/// The post-finish half of the orphaned-guard contract. A real driver commits
/// teardown and exits, leaving the rootless tree and its stable epoch file. A
/// shared guard is then held on that file, standing in for a tree command the
/// dead driver's session admitted and that outlived it; what is under test is
/// the *replacement's* response, which the holder's identity cannot change.
/// Because lease acquisition sits above every lifecycle mutation, the blocked
/// replacement's stop is proven by the task tree it never creates — recreating
/// `.grove/` here would be worse than stopping, since the next driver would
/// find a grove nobody started.
#[test]
fn a_shared_epoch_guard_blocks_the_post_finish_replacement_without_creating_a_tree() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("post-finish-orphan-guard");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    fs::remove_file(repository.join(".grove/02-finish-finish-k2.md")).unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();

    let launch_log = fixture.path().join("launch-log");
    let script = fixture.path().join("teardown-then-exit.sh");
    write_executable(
        &script,
        &format!(
            "#!/bin/sh\ncase \"$1\" in {finish}) {llm} finish-commit finish-k2; exit $?;; esac\nprintf '%s\\n' \"$2\" > {log}\n",
            finish = FINISH_ROUTE,
            llm = env!("CARGO_BIN_EXE_grove-llm"),
            log = launch_log.display(),
        ),
    );
    write_config_routing_finish_apart(&home, &script);

    let torn_down = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();
    assert!(
        torn_down.status.success(),
        "{}",
        String::from_utf8_lossy(&torn_down.stderr)
    );
    assert!(!repository.join(".grove").exists());

    let epoch_path = repository.join(".jj/grove/session.epoch");
    let orphan_guard = File::open(&epoch_path).unwrap();
    assert_eq!(
        unsafe { libc::flock(orphan_guard.as_raw_fd(), libc::LOCK_SH) },
        0,
        "holding the orphan's shared epoch guard failed"
    );

    let started = Instant::now();
    let blocked = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();
    let blocked_elapsed = started.elapsed();

    assert!(
        !blocked.status.success(),
        "a held epoch guard let the replacement proceed"
    );
    let diagnostic = String::from_utf8_lossy(&blocked.stderr);
    assert!(
        diagnostic.contains("waiting for exclusive session epoch lock for driver acquisition"),
        "{diagnostic}"
    );
    assert!(
        diagnostic.contains(
            "timed out after 30s waiting for exclusive session epoch lock for driver acquisition"
        ),
        "{diagnostic}"
    );
    assert!(
        !repository.join(".grove").exists(),
        "a blocked replacement created a task tree: {diagnostic}"
    );
    assert!(!launch_log.exists(), "a blocked replacement launched");
    assert!(
        blocked_elapsed >= Duration::from_secs(30),
        "the fixed bound fired early: {blocked_elapsed:?}"
    );

    drop(orphan_guard);

    let fresh = Command::cargo_bin("grove")
        .unwrap()
        .current_dir(&repository)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    assert!(
        fresh.status.success(),
        "{}",
        String::from_utf8_lossy(&fresh.stderr)
    );
    assert!(repository
        .join(".grove/01-requirements-plan-k1.md")
        .is_file());
    assert!(fs::read_to_string(&launch_log).unwrap().contains("plan-k1"));
    let handed_over = fs::read_to_string(&epoch_path).unwrap();
    assert!(
        handed_over.starts_with("state=inactive\n"),
        "{handed_over:?}"
    );
}

#[test]
fn bare_driver_validates_config_before_recovering_an_interrupted_finish() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-recovers-finish");
    init_repo(&repository);
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
fn restart_recovers_a_process_exit_after_preparing_witness_publication() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-recovers-preparing-finish");
    init_repo(&repository);
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
fn restart_recovers_a_process_exit_after_repository_preparation() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-recovers-prepared-finish");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);

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
fn restart_recovers_each_preparing_witness_materialization_state() {
    for checkpoint in ["after-recovery-tree", "after-manifest", "after-ready"] {
        let fixture = TempDir::new().unwrap();
        let repository = fixture.path().join(format!("driver-recovers-{checkpoint}"));
        init_repo(&repository);
        seed_committed_terminal_grove(&repository);

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
        assert!(
            repository.join(".grove/02-finish-finish-k2.md").is_file(),
            "{checkpoint}"
        );
    }
}

#[test]
fn restart_recovers_a_ready_witness_before_evacuation() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-recovers-ready-finish");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);

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
    assert!(repository.join(".grove/NOTES.md").is_file());
    assert!(repository.join(".grove/02-finish-finish-k2.md").is_file());

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
    assert!(repository.join(".grove/02-finish-finish-k2.md").is_file());
}

#[test]
fn colocated_jj_restart_recovers_each_pre_evacuation_publication_state() {
    for checkpoint in [
        "after-preparing-witness",
        "after-repository-preparation",
        "after-recovery-tree",
        "after-manifest",
        "after-ready",
        "after-ready-witness",
    ] {
        let fixture = TempDir::new().unwrap();
        let repository = fixture.path().join(format!("jj-recovers-{checkpoint}"));
        init_jj(&repository, true);
        seed_jj_terminal_grove(&repository);
        fs::write(repository.join("unrelated.txt"), "unrelated\n").unwrap();
        let commit_before = jj_output(
            &repository,
            &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
        );

        let interrupted = Command::cargo_bin("grove-llm")
            .unwrap()
            .current_dir(&repository)
            .env_remove("GROVE_SIGNAL_FILE")
            .env("GROVE_TEST_FINISH_EXIT_AT", checkpoint)
            .args(["finish-commit", "finish-k2"])
            .output()
            .unwrap();

        assert!(!interrupted.status.success(), "{checkpoint}");

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
        assert_eq!(
            jj_output(
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
    init_repo(&repository);
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
    assert!(parent_description(&repository).contains("finish-k2"));

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
    let control = repository.join(".jj/grove");
    assert!(fs::read_dir(control).unwrap().all(|entry| !entry
        .unwrap()
        .file_name()
        .to_string_lossy()
        .starts_with("FINISHED-")));
}

fn control_entries_starting_with(control_directory: &Path, prefix: &str) -> Vec<PathBuf> {
    let mut matches = fs::read_dir(control_directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with(prefix))
        })
        .collect::<Vec<_>>();
    matches.sort();
    matches
}

fn configure_fresh_requirements_launch(fixture: &Path, home: &Path) -> PathBuf {
    fs::create_dir_all(home.join(".codex")).unwrap();
    let launch_log = fixture.join("launch-log");
    let script = fixture.join("record-requirements.sh");
    write_executable(
        &script,
        &format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" > {}\n",
            launch_log.display()
        ),
    );
    write_complete_config(home, &format!("sh {} '${{prompt}}'", script.display()));
    launch_log
}

/// The atomic rename is the transaction's only transition to task-root absence,
/// so a death on its near side must leave the whole evacuated tree in the task
/// root under its blocking witness — never a half-moved shape a reader could
/// walk.
#[test]
fn a_death_before_the_quarantine_rename_keeps_the_complete_in_tree_witness() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("death-before-quarantine-rename");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);

    let interrupted = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_EXIT_AT", "before-quarantine-handoff")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();

    assert!(!interrupted.status.success());
    let grove_root = repository.join(".grove");
    let witness = grove_root.join("FINISHING-finish-k2");
    let control_directory = repository.join(".jj/grove");
    assert_eq!(
        fs::read_dir(&grove_root)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect::<Vec<_>>(),
        vec![OsString::from("FINISHING-finish-k2")],
        "the task root must hold nothing but its blocking witness"
    );
    for entry in [
        "NOTES.md",
        "BRIEF.md",
        "01-DONE-impl-finished-k1.md",
        "02-finish-finish-k2.md",
    ] {
        assert!(
            witness.join("original").join(entry).is_file(),
            "the witness lost {entry}"
        );
    }
    assert_eq!(
        control_entries_starting_with(&control_directory, "GROVE-FINISH-CLEANUP-").len(),
        1,
        "the published cleanup marker is the handoff's only control-side state"
    );
    assert!(control_entries_starting_with(&control_directory, "FINISHED-").is_empty());

    let home = fixture.path().join("home");
    let launch_log = configure_fresh_requirements_launch(fixture.path(), &home);
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
    assert!(grove_root.join("01-requirements-plan-k1.md").is_file());
    assert!(!witness.exists());
    assert!(control_entries_starting_with(&control_directory, "GROVE-FINISH-CLEANUP-").is_empty());
    assert!(fs::read_to_string(launch_log).unwrap().contains("plan-k1"));
}

/// The far side of the same rename: an absent task root, a complete quarantine
/// holding the witness, and cleanup evidence a later driver can reap.
#[test]
fn a_death_after_the_quarantine_rename_leaves_the_complete_quarantine() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("death-after-quarantine-rename");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);

    let interrupted = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(&repository)
        .env_remove("GROVE_SIGNAL_FILE")
        .env("GROVE_TEST_FINISH_EXIT_AT", "after-quarantine-handoff")
        .args(["finish-commit", "finish-k2"])
        .output()
        .unwrap();

    assert!(!interrupted.status.success());
    assert!(!repository.join(".grove").exists());
    let control_directory = repository.join(".jj/grove");
    let quarantines = control_entries_starting_with(&control_directory, "FINISHED-finish-k2-");
    let [quarantine] = quarantines.as_slice() else {
        panic!("the rename must leave exactly one complete quarantine, found {quarantines:?}");
    };
    for entry in [
        "NOTES.md",
        "BRIEF.md",
        "01-DONE-impl-finished-k1.md",
        "02-finish-finish-k2.md",
    ] {
        assert!(
            quarantine
                .join("FINISHING-finish-k2/original")
                .join(entry)
                .is_file(),
            "the quarantine lost {entry}"
        );
    }
    assert_eq!(
        control_entries_starting_with(&control_directory, "GROVE-FINISH-CLEANUP-").len(),
        1
    );

    let home = fixture.path().join("home");
    let launch_log = configure_fresh_requirements_launch(fixture.path(), &home);
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
    assert!(repository
        .join(".grove/01-requirements-plan-k1.md")
        .is_file());
    assert!(control_entries_starting_with(&control_directory, "FINISHED-finish-k2-").is_empty());
    assert!(control_entries_starting_with(&control_directory, "GROVE-FINISH-CLEANUP-").is_empty());
    assert!(fs::read_to_string(launch_log).unwrap().contains("plan-k1"));
}

#[test]
fn bare_driver_blocks_on_divergent_finish_recovery_without_launching() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("driver-blocks-divergent-finish");
    init_repo(&repository);
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
    run("jj", &repository, &["commit", "-m", "divergent"]);

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

/// The readiness flake, reproduced from the consuming side without touching
/// any timing the shipped code controls: stage the exact partial state a
/// create-then-write publication leaves — the barrier present and empty — and
/// prove the waiter every barrier test here goes through never hands that back
/// as a paused step's payload. A waiter that answers on existence alone
/// returns the empty payload immediately, whatever the two threads do.
#[test]
fn a_barrier_waiter_blocks_until_the_payload_is_complete() {
    let fixture = TempDir::new().unwrap();
    let barrier = fixture.path().join("staged-empty-barrier");
    fs::write(&barrier, b"").unwrap();
    let observed = barrier.clone();

    let waiter = thread::spawn(move || wait_for_barrier(&observed));
    // Widens the window in which the waiter may mis-read the staged empty
    // barrier. A waiter that blocks on an empty payload cannot fail this
    // however short the window is; one that does not, fails it however long.
    thread::sleep(Duration::from_millis(50));
    let staged = fixture.path().join("staged-payload");
    fs::write(&staged, "race-entry").unwrap();
    fs::rename(&staged, &barrier).unwrap();

    assert_eq!(waiter.join().unwrap(), b"race-entry".to_vec());
}

#[test]
fn cleanup_marker_publication_uses_the_validated_control_directory_object() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("descriptor-bound-marker-publication");
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    let barrier = fixture.path().join("marker-publication-barrier");
    let control_directory = repository.join(".jj/grove");
    let validated_control_directory = repository.join(".jj/grove-validated");

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
    wait_for_barrier(&barrier);
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
    init_repo(&repository);
    seed_committed_terminal_grove(&repository);
    fs::write(repository.join(".grove/race-entry"), "original\n").unwrap();
    let barrier = fixture.path().join("entry-unlink-barrier");
    let control_directory = repository.join(".jj/grove");

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
    let entry_name = std::ffi::OsString::from_vec(wait_for_barrier(&barrier));
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
    init_repo(&repository);
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
    let control_directory = repository.join(".jj/grove");
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
    init_repo(&repository);
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

    let control_directory = repository.join(".jj/grove");
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
    init_repo(&repository);
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
    let control_directory = repository.join(".jj/grove");
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
    init_repo(&repository);
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
    let control_directory = repository.join(".jj/grove");
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
    run("jj", &repository, &["commit", "-m", "divergent"]);

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
    init_repo(&repository);
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
    let control_directory = repository.join(".jj/grove");
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
    init_repo(&repository);
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

    let control_directory = repository.join(".jj/grove");
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
fn persistent_cleanup_failure_warns_and_retries_without_blocking_fresh_lifecycle() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("persistent-driver-cleanup-failure");
    init_repo(&repository);
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
    let control_directory = repository.join(".jj/grove");
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
