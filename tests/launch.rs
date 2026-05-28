mod common;

use common::{fixture_tarball, StubFetcher};
use grove::cli::{InstallArgs, NameArgs, StartArgs};
use grove::install::{run_with_fetcher, Mode};
use grove::launch;
use std::fs;
use std::process::Command;
use std::sync::Mutex;
use tempfile::TempDir;

// These tests all mutate process-global cwd; serialize so cargo's parallel
// test runner doesn't have one test's cwd swept out from under another's
// repo::resolve(None) call.
static CWD_LOCK: Mutex<()> = Mutex::new(());

fn init_repo_with_grove_installed() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::new("git")
        .args(["init", "-b", "main"])
        .arg(tmp.path())
        .status()
        .unwrap();
    fs::write(tmp.path().join("README.md"), "x").unwrap();
    Command::new("git")
        .args(["-C", tmp.path().to_str().unwrap(), "add", "."])
        .status()
        .unwrap();
    Command::new("git")
        .args([
            "-C", tmp.path().to_str().unwrap(),
            "commit", "-m", "init", "--no-verify",
        ])
        .status()
        .unwrap();

    fs::create_dir_all(tmp.path().join(".claude")).unwrap();
    let fetcher = StubFetcher {
        latest: "v0.1.0".into(),
        tarball: fixture_tarball(
            "0.1.0",
            &[
                ("content/SKILL.md", b"x"),
                ("content/prompts/start.md", b"start {{NAME}}"),
                ("content/prompts/continue.md", b"continue {{NAME}}"),
            ],
        ),
    };
    run_with_fetcher(
        &InstallArgs {
            repo: Some(tmp.path().to_path_buf()),
            harnesses: vec![],
            version: Some("v0.1.0".into()),
            no_commit: true,
            message: None,
        },
        Mode::Install,
        &fetcher,
    )
    .unwrap();
    tmp
}

#[test]
fn start_creates_worktree_in_no_launch_mode() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo_with_grove_installed();
    std::env::set_current_dir(repo.path()).unwrap();

    launch::start(&StartArgs {
        name: "auth".into(),
        start_point: Some("main".into()),
        harness: None,
        no_launch: true,
    })
    .unwrap();

    assert!(repo.path().join(".grove-worktrees/auth").is_dir());
}

#[test]
fn continue_errors_when_no_worktree() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo_with_grove_installed();
    std::env::set_current_dir(repo.path()).unwrap();

    let err = launch::continue_grove(&NameArgs {
        name: "ghost".into(),
        harness: None,
        no_launch: true,
    })
    .unwrap_err();
    assert!(err.to_string().contains("no worktree for grove"));
}

#[test]
fn do_starts_when_grove_is_unknown() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo_with_grove_installed();
    std::env::set_current_dir(repo.path()).unwrap();

    launch::do_grove(&NameArgs {
        name: "fresh".into(),
        harness: None,
        no_launch: true,
    })
    .unwrap();

    assert!(repo.path().join(".grove-worktrees/fresh").is_dir());
}

#[test]
fn do_continues_when_worktree_is_live() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo_with_grove_installed();
    std::env::set_current_dir(repo.path()).unwrap();

    launch::start(&StartArgs {
        name: "alive".into(),
        start_point: Some("main".into()),
        harness: None,
        no_launch: true,
    })
    .unwrap();

    // Worktree exists; `do` must succeed (the continue path in no-launch
    // mode is a no-op past the worktree presence check).
    launch::do_grove(&NameArgs {
        name: "alive".into(),
        harness: None,
        no_launch: true,
    })
    .unwrap();
}

#[test]
fn do_reattaches_orphaned_worktree() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo_with_grove_installed();
    std::env::set_current_dir(repo.path()).unwrap();

    launch::start(&StartArgs {
        name: "orphan".into(),
        start_point: Some("main".into()),
        harness: None,
        no_launch: true,
    })
    .unwrap();

    // Simulate user manually deleting the worktree: drop the directory
    // and prune git's stale worktree entry.
    fs::remove_dir_all(repo.path().join(".grove-worktrees/orphan")).unwrap();
    Command::new("git")
        .args(["-C", repo.path().to_str().unwrap(), "worktree", "prune"])
        .status()
        .unwrap();
    assert!(!repo.path().join(".grove-worktrees/orphan").exists());

    launch::do_grove(&NameArgs {
        name: "orphan".into(),
        harness: None,
        no_launch: true,
    })
    .unwrap();

    assert!(repo.path().join(".grove-worktrees/orphan").is_dir());
}
