mod common;

use grove::cli::UninstallArgs;
use grove::uninstall::run;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::new("git")
        .arg("init")
        .arg(tmp.path())
        .status()
        .unwrap();
    fs::create_dir_all(tmp.path().join(".claude/skills/grove")).unwrap();
    fs::write(tmp.path().join(".claude/skills/grove/SKILL.md"), "x").unwrap();
    tmp
}

#[test]
fn uninstall_removes_grove_dir() {
    let repo = init_repo();

    run(&UninstallArgs {
        repo: Some(repo.path().to_path_buf()),
        harnesses: vec![],
        force: false,
    })
    .unwrap();

    assert!(!repo.path().join(".claude/skills/grove").exists());
}

#[test]
fn uninstall_refuses_when_live_groves_exist() {
    let repo = init_repo();
    fs::create_dir_all(repo.path().join(".grove-worktrees/auth")).unwrap();

    let err = run(&UninstallArgs {
        repo: Some(repo.path().to_path_buf()),
        harnesses: vec![],
        force: false,
    })
    .unwrap_err();
    assert!(err.to_string().contains("live groves exist"));
    assert!(repo.path().join(".claude/skills/grove").exists());
}

#[test]
fn uninstall_force_overrides_live_groves_check() {
    let repo = init_repo();
    fs::create_dir_all(repo.path().join(".grove-worktrees/auth")).unwrap();

    run(&UninstallArgs {
        repo: Some(repo.path().to_path_buf()),
        harnesses: vec![],
        force: true,
    })
    .unwrap();

    assert!(!repo.path().join(".claude/skills/grove").exists());
}
