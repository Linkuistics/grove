mod support;

use grove::tree_migrate::{self, Outcome};
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn run_git(worktree: &Path, arguments: &[&str]) {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(worktree)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_stdout(worktree: &Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(worktree)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

#[test]
fn adoption_migration_commits_in_on_disk_worktree_despite_foreign_git_environment() {
    let tmp = TempDir::new().unwrap();
    let intended = tmp.path().join("intended");
    let foreign = tmp.path().join("foreign");
    fs::create_dir_all(&intended).unwrap();
    fs::create_dir_all(&foreign).unwrap();
    run_git(&intended, &["init", "-q"]);
    run_git(&foreign, &["init", "-q"]);
    run_git(&intended, &["config", "user.name", "Test"]);
    run_git(&intended, &["config", "user.email", "test@example.com"]);
    fs::create_dir_all(intended.join(".grove")).unwrap();
    fs::write(intended.join(".grove/BRIEF.md"), "# intended — brief\n").unwrap();
    fs::write(
        intended.join(".grove/1-[1]-test.md"),
        "# test-k1\n\n**Kind:** impl\n",
    )
    .unwrap();
    run_git(&intended, &["add", ".grove"]);
    run_git(&intended, &["commit", "-q", "-m", "seed legacy grove"]);
    let head_before = git_stdout(&intended, &["rev-parse", "HEAD"]);
    run_git(
        &intended,
        &["config", "core.worktree", foreign.to_str().unwrap()],
    );
    let mut environment = support::EnvGuard::new();
    environment
        .set("GIT_DIR", foreign.join(".git"))
        .set("GIT_WORK_TREE", &foreign)
        .set("GIT_COMMON_DIR", foreign.join(".git"));

    let outcome = tree_migrate::migrate_on_adoption(&intended, "intended").unwrap();

    drop(environment);
    assert!(matches!(outcome, Outcome::Migrated(_)));
    assert!(intended.join(".grove/01-test-k1.md").exists());
    assert!(!intended.join(".grove/1-[1]-test.md").exists());
    assert_ne!(git_stdout(&intended, &["rev-parse", "HEAD"]), head_before);
}
