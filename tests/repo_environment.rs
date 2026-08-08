mod support;

use grove::repo::{main_repo_of, workspace_control};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn init_git_repo(path: &Path) {
    run_git(path, &["init", "-q", "."]);
}

fn run_git(path: &Path, arguments: &[&str]) {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn canon(path: &Path) -> PathBuf {
    path.canonicalize().unwrap()
}

#[test]
fn workspace_control_ignores_git_discovery_and_temporary_directory_environment() {
    let tmp = TempDir::new().unwrap();
    let intended = tmp.path().join("intended");
    let foreign = tmp.path().join("foreign");
    let fake_tmp = tmp.path().join("ambient-tmp");
    fs::create_dir_all(&intended).unwrap();
    fs::create_dir_all(&foreign).unwrap();
    fs::create_dir_all(&fake_tmp).unwrap();
    init_git_repo(&intended);
    init_git_repo(&foreign);
    run_git(
        &intended,
        &["config", "core.worktree", foreign.to_str().unwrap()],
    );
    let nested = intended.join("src");
    fs::create_dir_all(&nested).unwrap();
    let mut env = support::EnvGuard::new();
    env.set("GIT_DIR", foreign.join(".git"))
        .set("GIT_WORK_TREE", &foreign)
        .set("GIT_COMMON_DIR", foreign.join(".git"))
        .set("TMPDIR", &fake_tmp);

    let control = workspace_control(&nested).unwrap();

    assert_eq!(control.worktree_root(), canon(&intended));
    assert_eq!(
        control.control_dir(),
        canon(&intended.join(".git")).join("grove")
    );
    assert_eq!(main_repo_of(&nested).unwrap(), canon(&intended));
}
