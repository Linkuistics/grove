mod support;

use grove::repo::{main_repo_of, workspace_control};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// A **colocated** jj repository, so the `GIT_*` selectors below are live
/// rather than inert: a colocated tree has a real `.git` for them to point
/// somewhere else, which is what makes the assertion worth making.
fn init_colocated_repo(path: &Path) {
    support::jj(path, &["git", "init", "--colocate", "--quiet", "."]);
}

fn canon(path: &Path) -> PathBuf {
    path.canonicalize().unwrap()
}

#[test]
fn workspace_control_ignores_repository_selection_and_temporary_directory_environment() {
    let tmp = TempDir::new().unwrap();
    let intended = tmp.path().join("intended");
    let foreign = tmp.path().join("foreign");
    let fake_tmp = tmp.path().join("ambient-tmp");
    fs::create_dir_all(&intended).unwrap();
    fs::create_dir_all(&foreign).unwrap();
    fs::create_dir_all(&fake_tmp).unwrap();
    init_colocated_repo(&intended);
    init_colocated_repo(&foreign);
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
        canon(&intended).join(".jj/grove")
    );
    assert_eq!(main_repo_of(&nested).unwrap(), canon(&intended));
}
