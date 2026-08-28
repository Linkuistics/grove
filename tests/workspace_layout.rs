// The workspace layout preflight (src/driver_lease.rs, ADR
// *supported-workspace-layouts*), driven end to end through the real bare
// `grove` process and the real `grove-llm finish-commit`.
//
// The layout matrix is the subject here, so the fixtures are whole
// workspaces — plain checkouts, linked Git worktrees, native, colocated and
// secondary jj workspaces, and symlinked markers — rather than the lease
// internals `tests/driver_lease.rs` owns.
//
// **One operand cannot be staged portably: a second filesystem.** Mounting one
// needs privileges on Linux and a disk image on macOS, so
// `GROVE_TEST_FOREIGN_FILESYSTEM` (`repo::measured_device`) names a directory
// whose measurements report a distinct filesystem. Everything else in each case
// is real: the resolver walks a real marker, the driver creates a real control
// directory, and the refusal is the production diagnostic on the production
// path. The seam is a *path*, so each test has to name the exact directory
// resolution landed on — a run cannot pass while the resolver went elsewhere.

mod support;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

const FOREIGN: &str = "GROVE_TEST_FOREIGN_FILESYSTEM";
const OWN_GROVE_LLM: &str = env!("CARGO_BIN_EXE_grove-llm");

const SESSION_KINDS: &[&str] = &[
    "requirements",
    "review-requirements",
    "integrate-review-requirements",
    "design",
    "review-design",
    "integrate-review-design",
    "planning",
    "review-planning",
    "integrate-review-planning",
    "prototype",
    "review-prototype",
    "integrate-review-prototype",
    "impl",
    "review-impl",
    "integrate-review-impl",
    "research-a",
    "research-b",
    "combine-research",
    "finish",
];

// ---- fixtures -------------------------------------------------------------

fn run(program: &str, directory: &Path, arguments: &[&str]) -> String {
    let output = Command::new(program)
        .current_dir(directory)
        .args(arguments)
        .output()
        .unwrap_or_else(|error| panic!("running {program} {arguments:?}: {error}"));
    assert!(
        output.status.success(),
        "{program} {arguments:?} failed in {}:\n{}",
        directory.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn init_git(path: &Path) {
    fs::create_dir_all(path).unwrap();
    run("git", path, &["init", "-q", "."]);
    run("git", path, &["config", "user.name", "Grove Test"]);
    run("git", path, &["config", "user.email", "t@example.com"]);
    run("git", path, &["config", "core.hooksPath", "/dev/null"]);
}

/// `git.colocate=false` is forced because the ambient jj config may default
/// colocation on, which would silently turn a "native" fixture into a colocated
/// one — and the two resolve through different markers.
fn init_jj(path: &Path, colocated: bool) {
    fs::create_dir_all(path).unwrap();
    if colocated {
        init_git(path);
        run("jj", path, &["git", "init", "--colocate", "--quiet", "."]);
    } else {
        run(
            "jj",
            path,
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
        path,
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
        path,
        &[
            "config",
            "set",
            "--workspace",
            "user.email",
            "\"t@example.com\"",
        ],
    );
}

/// A linked Git worktree of `main`, plus the canonical gitdir its `.git` **file**
/// names — the indirection that makes this family the one whose devices can
/// differ. The target is read from the marker rather than assumed, so a fixture
/// cannot drift from what the resolver will find.
fn linked_worktree(main: &Path, name: &str) -> (PathBuf, PathBuf) {
    run(
        "git",
        main,
        &["commit", "-q", "--allow-empty", "-m", "seed"],
    );
    let worktree = main.parent().unwrap().join(name);
    run(
        "git",
        main,
        &["worktree", "add", "-q", worktree.to_str().unwrap()],
    );
    let marker = fs::read_to_string(worktree.join(".git")).unwrap();
    let target = marker.strip_prefix("gitdir: ").unwrap().trim();
    let gitdir = worktree.join(target).canonicalize().unwrap();
    (worktree.canonicalize().unwrap(), gitdir)
}

fn write_executable(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

fn shell_quote(path: &Path) -> String {
    let value = path.to_str().unwrap();
    assert!(!value.contains('\''), "test fixture path contains a quote");
    format!("'{value}'")
}

/// A configured command that records that it ran and then exits without
/// signalling, so the loop drives exactly one session and stops.
fn recording_command(fixture: &Path, name: &str) -> (PathBuf, PathBuf) {
    let receipt = fixture.join(format!("{name}-session-ran"));
    let script = fixture.join(format!("{name}-command.sh"));
    write_executable(
        &script,
        &format!("#!/bin/sh\nprintf ran > {}\n", shell_quote(&receipt)),
    );
    (script, receipt)
}

/// A complete personal config, so configuration validation would pass if it
/// were ever reached.
fn write_complete_config(home: &Path, command: &Path) {
    fs::create_dir_all(home.join(".codex")).unwrap();
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    let template = format!("{} '${{prompt}}'", shell_quote(command));
    let document = SESSION_KINDS
        .iter()
        .map(|kind| format!("{kind} {template:?}\n"))
        .collect::<String>();
    fs::write(config_dir.join("config.kdl"), document).unwrap();
}

/// A `$HOME` with **no** configuration at all. A driver that reached
/// configuration validation would fail with `Grove configuration is missing`,
/// which is what makes "the layout refusal precedes it" an observable claim
/// rather than a reading of the source order.
fn write_no_config(home: &Path) {
    fs::create_dir_all(home.join(".codex")).unwrap();
}

fn plant_tree(worktree: &Path, leaf: &str) {
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# layout — brief\n").unwrap();
    fs::write(grove.join(leaf), "# planted\n").unwrap();
}

fn grove_driver(worktree: &Path, home: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_grove"));
    command.current_dir(worktree);
    for name in support::grove_env_names() {
        command.env_remove(name);
    }
    command.env_remove(FOREIGN);
    command.env("HOME", home);
    command
}

fn grove_llm(worktree: &Path, arguments: &[&str]) -> Command {
    let mut command = Command::new(OWN_GROVE_LLM);
    command.current_dir(worktree).args(arguments);
    command.env_remove("GROVE_SIGNAL_FILE");
    command.env_remove(FOREIGN);
    command
}

fn stderr_of(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Every claim an unsupported-layout refusal has to carry, checked in one place
/// so each case below states only what makes it different.
fn assert_layout_refusal(
    output: &Output,
    worktree_root: &Path,
    control_dir: &Path,
    marker_fragments: &[&str],
) {
    let stderr = stderr_of(output);
    assert!(
        !output.status.success(),
        "an unfinishable layout must refuse:\n{stderr}"
    );
    for expected in [
        "unsupported workspace layout",
        &worktree_root.display().to_string(),
        &control_dir.display().to_string(),
        "same filesystem as the repository",
        "administration directory is inside the working tree",
        "Nothing was created or changed",
    ] {
        assert!(stderr.contains(expected), "{expected:?} missing:\n{stderr}");
    }
    for fragment in marker_fragments {
        assert!(
            stderr.contains(fragment),
            "the marker that produced the resolution is not named ({fragment:?}):\n{stderr}"
        );
    }
    assert_eq!(
        stderr.matches("(filesystem ").count(),
        2,
        "both ends must name their filesystem:\n{stderr}"
    );
    assert!(
        !stderr.contains("creating Grove control directory"),
        "an unsupported layout must be distinguishable from an unwritable \
         control directory:\n{stderr}"
    );
    assert!(
        !stderr.contains("Grove configuration is missing"),
        "the layout refusal must precede configuration validation:\n{stderr}"
    );
    assert!(
        !control_dir.join("driver.lease").exists(),
        "a refused layout must not leave a lease behind"
    );
    assert!(
        !control_dir.join("session.epoch").exists(),
        "a refused layout must not install a session epoch"
    );
}

/// Every path under `root`, with file contents — the byte-identical claim.
fn snapshot(root: &Path) -> Vec<(PathBuf, Option<Vec<u8>>)> {
    fn walk(root: &Path, at: &Path, into: &mut Vec<(PathBuf, Option<Vec<u8>>)>) {
        let mut entries = fs::read_dir(at)
            .unwrap()
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let relative = path.strip_prefix(root).unwrap().to_path_buf();
            if entry.file_type().unwrap().is_dir() {
                into.push((relative, None));
                walk(root, &path, into);
            } else {
                into.push((relative, Some(fs::read(&path).unwrap())));
            }
        }
    }
    let mut entries = Vec::new();
    walk(root, root, &mut entries);
    entries
}

// ---- the admitted layouts -------------------------------------------------

// Grove measures every layout rather than trusting the marker's kind, so the
// admitted half of the matrix has to be driven too: a preflight that refused a
// plain checkout, a same-device linked worktree, or any jj shape would make the
// product unusable while every refusal test above still passed.
#[test]
fn every_admitted_layout_drives_a_session_normally() {
    let fixture = TempDir::new().unwrap();

    let plain = fixture.path().join("plain");
    init_git(&plain);

    let linked_main = fixture.path().join("linked-main/main");
    init_git(&linked_main);
    let (linked, _) = linked_worktree(&linked_main, "linked");

    let native = fixture.path().join("native-jj");
    init_jj(&native, false);

    let colocated = fixture.path().join("colocated-jj");
    init_jj(&colocated, true);

    let jj_main = fixture.path().join("secondary-main/main");
    init_jj(&jj_main, false);
    let secondary = fixture.path().join("secondary-main/secondary");
    run(
        "jj",
        &jj_main,
        &["workspace", "add", "--quiet", secondary.to_str().unwrap()],
    );

    for (name, worktree) in [
        ("plain-checkout", plain),
        ("linked-git-worktree", linked),
        ("native-jj", native),
        ("colocated-jj", colocated),
        ("secondary-jj-workspace", secondary),
    ] {
        let home = fixture.path().join(format!("{name}-home"));
        let (command, receipt) = recording_command(fixture.path(), name);
        write_complete_config(&home, &command);
        plant_tree(&worktree, "01-impl-subject-k1.md");

        let output = grove_driver(&worktree, &home).output().unwrap();

        let stderr = stderr_of(&output);
        assert!(
            output.status.success(),
            "{name} is an admitted layout but the driver failed:\n{stderr}"
        );
        assert!(
            !stderr.contains("unsupported workspace layout"),
            "{name} is an admitted layout but was refused:\n{stderr}"
        );
        assert!(
            receipt.exists(),
            "{name} never reached a configured session:\n{stderr}"
        );
    }
}

// ---- the refused family ---------------------------------------------------

// The at-risk family, in the shape where the cost of discovering it late is
// highest: a workspace with no task tree yet. The refusal has to land before
// configuration validation (proved by an empty `$HOME`) and before anything
// creates `.grove/`, so the operator pays nothing recoverable.
#[test]
fn a_cross_device_linked_worktree_is_refused_before_config_validation_or_any_tree() {
    let fixture = TempDir::new().unwrap();
    let main = fixture.path().join("main");
    init_git(&main);
    let (worktree, gitdir) = linked_worktree(&main, "linked");
    let home = fixture.path().join("home");
    write_no_config(&home);
    let head_before = run("git", &worktree, &["rev-parse", "HEAD"]);

    let output = grove_driver(&worktree, &home)
        .env(FOREIGN, &gitdir)
        .output()
        .unwrap();

    assert_layout_refusal(
        &output,
        &worktree,
        &gitdir.join("grove"),
        &[
            &format!("the `.git` file {}", worktree.join(".git").display()),
            &format!("naming gitdir {}", gitdir.display()),
        ],
    );
    assert!(
        !worktree.join(".grove").exists(),
        "a refused layout must not create a task tree"
    );
    assert_eq!(
        run("git", &worktree, &["rev-parse", "HEAD"]),
        head_before,
        "a refused layout must author no revision"
    );
}

// The same refusal against a workspace that already holds months of work. The
// tree is the thing an operator would lose if the stop were not clean, so it is
// compared byte for byte rather than merely checked for existence.
#[test]
fn a_cross_device_refusal_leaves_an_existing_tree_byte_identical() {
    let fixture = TempDir::new().unwrap();
    let main = fixture.path().join("main");
    init_git(&main);
    let (worktree, gitdir) = linked_worktree(&main, "linked");
    plant_tree(&worktree, "01-impl-subject-k1.md");
    run("git", &worktree, &["add", "-A"]);
    run("git", &worktree, &["commit", "-q", "-m", "seed grove"]);
    let home = fixture.path().join("home");
    let (command, receipt) = recording_command(fixture.path(), "existing");
    write_complete_config(&home, &command);
    let before = snapshot(&worktree.join(".grove"));
    let head_before = run("git", &worktree, &["rev-parse", "HEAD"]);

    let output = grove_driver(&worktree, &home)
        .env(FOREIGN, &gitdir)
        .output()
        .unwrap();

    assert_layout_refusal(
        &output,
        &worktree,
        &gitdir.join("grove"),
        &[&format!("naming gitdir {}", gitdir.display())],
    );
    assert_eq!(
        snapshot(&worktree.join(".grove")),
        before,
        "a refused layout must leave the task tree byte-identical"
    );
    assert_eq!(
        run("git", &worktree, &["rev-parse", "HEAD"]),
        head_before,
        "a refused layout must author no revision"
    );
    assert!(
        !receipt.exists(),
        "a refused layout must launch no configured session"
    );
}

// A symlinked marker leaves the working tree without changing the marker's
// kind, so a preflight that classified layouts from the table instead of
// measuring them would admit both of these. `.jj/` and a `.git/` directory are
// exactly the two the table calls in-root.
#[test]
fn a_symlinked_marker_onto_another_filesystem_is_refused_on_the_same_path() {
    let fixture = TempDir::new().unwrap();

    let git_tree = fixture.path().join("git-tree");
    init_git(&git_tree);
    let elsewhere_git = fixture.path().join("elsewhere/git-dir");
    fs::create_dir_all(elsewhere_git.parent().unwrap()).unwrap();
    fs::rename(git_tree.join(".git"), &elsewhere_git).unwrap();
    std::os::unix::fs::symlink(&elsewhere_git, git_tree.join(".git")).unwrap();

    let jj_tree = fixture.path().join("jj-tree");
    init_jj(&jj_tree, false);
    let elsewhere_jj = fixture.path().join("elsewhere/jj-dir");
    fs::rename(jj_tree.join(".jj"), &elsewhere_jj).unwrap();
    std::os::unix::fs::symlink(&elsewhere_jj, jj_tree.join(".jj")).unwrap();

    for (name, worktree, target, control_dir, marker) in [
        (
            "git",
            git_tree.canonicalize().unwrap(),
            elsewhere_git.canonicalize().unwrap(),
            elsewhere_git.canonicalize().unwrap().join("grove"),
            "the `.git` directory",
        ),
        (
            "jj",
            jj_tree.canonicalize().unwrap(),
            elsewhere_jj.canonicalize().unwrap(),
            // jj resolution deliberately does not follow the marker, so the
            // control directory is named *through* the symlink even though the
            // measurement resolves past it.
            jj_tree.canonicalize().unwrap().join(".jj/grove"),
            "the `.jj` directory",
        ),
    ] {
        let home = fixture.path().join(format!("{name}-home"));
        let (command, _) = recording_command(fixture.path(), name);
        write_complete_config(&home, &command);

        let output = grove_driver(&worktree, &home)
            .env(FOREIGN, &target)
            .output()
            .unwrap();

        assert_layout_refusal(&output, &worktree, &control_dir, &[marker]);
        assert!(
            !worktree.join(".grove").exists(),
            "{name}: a refused layout must not create a task tree"
        );
    }
}

// The refusal costs nothing recoverable, which is only true if repairing the
// layout is enough to continue. Nothing durable may record the earlier verdict.
#[test]
fn a_repaired_layout_resumes_normally() {
    let fixture = TempDir::new().unwrap();
    let main = fixture.path().join("main");
    init_git(&main);
    let (worktree, gitdir) = linked_worktree(&main, "linked");
    plant_tree(&worktree, "01-impl-subject-k1.md");
    let home = fixture.path().join("home");
    let (command, receipt) = recording_command(fixture.path(), "repaired");
    write_complete_config(&home, &command);

    let refused = grove_driver(&worktree, &home)
        .env(FOREIGN, &gitdir)
        .output()
        .unwrap();
    assert!(!refused.status.success(), "{}", stderr_of(&refused));

    let output = grove_driver(&worktree, &home).output().unwrap();

    let stderr = stderr_of(&output);
    assert!(
        output.status.success(),
        "a repaired layout must resume:\n{stderr}"
    );
    assert!(
        receipt.exists(),
        "the resumed run never launched:\n{stderr}"
    );
}

// Ambient `grove-llm` tree verbs allocate no quarantine, so they gain no layout
// check — an operator can still read and grow the tree of a workspace the driver
// refuses, which is what makes the refusal a stop rather than a lockout.
#[test]
fn ambient_tree_verbs_are_unaffected_by_an_unsupported_layout() {
    let fixture = TempDir::new().unwrap();
    let main = fixture.path().join("main");
    init_git(&main);
    let (worktree, gitdir) = linked_worktree(&main, "linked");
    plant_tree(&worktree, "01-impl-subject-k1.md");

    let picked = grove_llm(&worktree, &["pick"])
        .env(FOREIGN, &gitdir)
        .output()
        .unwrap();
    let grown = grove_llm(&worktree, &["leaf-add", ".", "later"])
        .env(FOREIGN, &gitdir)
        .output()
        .unwrap();

    assert!(picked.status.success(), "{}", stderr_of(&picked));
    assert!(
        String::from_utf8_lossy(&picked.stdout).contains("01-impl-subject-k1.md"),
        "pick must still walk the tree of a workspace the driver refuses"
    );
    assert!(grown.status.success(), "{}", stderr_of(&grown));
    assert!(worktree.join(".grove/02-impl-later-k2.md").is_file());
}

// ---- independence of the two preflights -----------------------------------

/// A plain-Git grove whose only live leaf is a finish leaf, ready for a real
/// `grove-llm finish-commit`.
fn terminal_grove(repository: &Path) {
    init_git(repository);
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# layout — brief\n").unwrap();
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

fn assert_finish_refused(output: &Output, repository: &Path, head_before: &str) {
    let stderr = stderr_of(output);
    assert!(
        !output.status.success(),
        "finish must refuse a cross-device handoff:\n{stderr}"
    );
    assert!(
        stderr.contains("atomic quarantine"),
        "the finish gate must speak for itself:\n{stderr}"
    );
    assert!(
        repository.join(".grove/02-finish-finish-k2.md").is_file(),
        "a refused finish must leave the live tree in place"
    );
    assert_eq!(
        run("git", repository, &["rev-parse", "HEAD"]),
        head_before,
        "a refused finish must author no revision"
    );
}

// The acquisition preflight is an early warning, never a licence: layout is
// mutable while the lease is held, so a workspace that passed at start-up and
// became cross-device before teardown must still be refused at the finish gate.
#[test]
fn a_layout_that_passes_acquisition_is_still_refused_when_it_changes_before_finish() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("repository");
    terminal_grove(&repository);
    let home = fixture.path().join("home");
    let (command, receipt) = recording_command(fixture.path(), "passes");
    write_complete_config(&home, &command);
    let head_before = run("git", &repository, &["rev-parse", "HEAD"]);

    let driven = grove_driver(&repository, &home).output().unwrap();
    assert!(driven.status.success(), "{}", stderr_of(&driven));
    assert!(
        receipt.exists(),
        "acquisition must have admitted this layout"
    );

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"])
        .env(FOREIGN, repository.join(".git"))
        .output()
        .unwrap();

    assert_finish_refused(&output, &repository, &head_before);
}

// The two preflights compare different operands, and this is the case that
// separates them: acquisition measures the *control directory*, while the rename
// moves `.grove/` itself. A task root on its own filesystem is therefore
// invisible at acquisition and correctly refused at finish — which is why
// carrying the startup verdict forward would be wrong rather than merely
// redundant.
#[test]
fn a_task_root_on_its_own_filesystem_passes_acquisition_and_is_refused_at_finish() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("repository");
    terminal_grove(&repository);
    let home = fixture.path().join("home");
    let (command, receipt) = recording_command(fixture.path(), "mountpoint");
    write_complete_config(&home, &command);
    let task_root = repository.join(".grove");
    let head_before = run("git", &repository, &["rev-parse", "HEAD"]);

    let driven = grove_driver(&repository, &home)
        .env(FOREIGN, &task_root)
        .output()
        .unwrap();

    let stderr = stderr_of(&driven);
    assert!(
        driven.status.success(),
        "a task root on its own filesystem is not an acquisition-time fact:\n{stderr}"
    );
    assert!(
        !stderr.contains("unsupported workspace layout"),
        "acquisition must not measure the task root:\n{stderr}"
    );
    assert!(receipt.exists(), "the driver never launched:\n{stderr}");

    let output = grove_llm(&repository, &["finish-commit", "finish-k2"])
        .env(FOREIGN, &task_root)
        .output()
        .unwrap();

    assert_finish_refused(&output, &repository, &head_before);
}

// `finish-commit` is separately invocable, including by an operator retrying a
// blocked transaction, so it can attest nothing about which driver validated
// what. Each invocation re-measures, and nothing on disk records that a
// workspace once passed.
#[test]
fn an_operator_retry_re_measures_with_no_durable_capability_marker() {
    let fixture = TempDir::new().unwrap();
    let repository = fixture.path().join("repository");
    terminal_grove(&repository);
    let head_before = run("git", &repository, &["rev-parse", "HEAD"]);

    let refused = grove_llm(&repository, &["finish-commit", "finish-k2"])
        .env(FOREIGN, repository.join(".git"))
        .output()
        .unwrap();
    assert_finish_refused(&refused, &repository, &head_before);

    let retried = grove_llm(&repository, &["finish-commit", "finish-k2"])
        .output()
        .unwrap();

    assert!(
        retried.status.success(),
        "a retry must perform its own comparison rather than reuse a verdict:\n{}",
        stderr_of(&retried)
    );
    assert!(!repository.join(".grove").exists());
    // The control directory is where a capability marker would have to live —
    // it is the only untracked, workspace-scoped place Grove writes.
    let control = repository.join(".git/grove");
    if control.is_dir() {
        for entry in fs::read_dir(&control).unwrap() {
            let name = entry.unwrap().file_name().to_string_lossy().into_owned();
            assert!(
                name == "driver.lease"
                    || name == "session.epoch"
                    || name == "internal-hooks-empty"
                    || name.starts_with("signal-")
                    || name.starts_with("GROVE-FINISH-"),
                "unrecognised control-directory entry {name:?} — a durable layout \
                 verdict is exactly what this workspace must not carry"
            );
        }
    }
}
