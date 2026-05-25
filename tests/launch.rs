mod common;

use common::{fixture_tarball, StubFetcher};
use grove::cli::{InstallArgs, NameArgs, StartArgs};
use grove::install::{run_with_fetcher, Mode};
use grove::launch;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

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
        },
        Mode::Install,
        &fetcher,
    )
    .unwrap();
    tmp
}

#[test]
fn start_creates_worktree_in_no_launch_mode() {
    let repo = init_repo_with_grove_installed();
    std::env::set_current_dir(repo.path()).unwrap();

    launch::start(&StartArgs {
        name: "auth".into(),
        start_point: Some("main".into()),
        harness: None,
        no_launch: true,
    })
    .unwrap();

    assert!(repo.path().join("worktrees/auth-grove").is_dir());
}

#[test]
fn continue_errors_when_no_worktree() {
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
