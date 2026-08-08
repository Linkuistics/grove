use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use tempfile::TempDir;

const WAITING_DIAGNOSTIC: &str = "waiting for active Grove tree operation";

fn init_repo() -> TempDir {
    let temporary_directory = TempDir::new().unwrap();
    let status = Command::new("git")
        .arg("init")
        .arg("-q")
        .arg(temporary_directory.path())
        .status()
        .unwrap();
    assert!(status.success());
    temporary_directory
}

fn seed_current_grove(worktree: &Path) {
    let output = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .current_dir(worktree)
        .arg("root-init")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "root-init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn lock_worktree(worktree: &Path, operation: libc::c_int) -> File {
    let directory = File::open(worktree).unwrap();
    let result = unsafe { libc::flock(directory.as_raw_fd(), operation) };
    assert_eq!(result, 0, "failed to lock {}", worktree.display());
    directory
}

fn unlock_worktree(directory: &File) {
    let result = unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_UN) };
    assert_eq!(result, 0, "failed to unlock worktree");
}

struct WaitingChild {
    child: Option<Child>,
    stderr_path: PathBuf,
}

impl WaitingChild {
    fn spawn(worktree: &Path, args: &[&str], stderr_path: PathBuf) -> Self {
        let stderr = File::create(&stderr_path).unwrap();
        let child = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
            .current_dir(worktree)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::from(stderr))
            .spawn()
            .unwrap();
        Self {
            child: Some(child),
            stderr_path,
        }
    }

    fn wait_until_contended(&mut self) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let stderr = fs::read_to_string(&self.stderr_path).unwrap_or_default();
            if stderr.contains(WAITING_DIAGNOSTIC) {
                return;
            }
            if let Some(status) = self.child.as_mut().unwrap().try_wait().unwrap() {
                panic!(
                    "tree operation exited before waiting for the worktree lock ({status}): {stderr}"
                );
            }
            assert!(
                Instant::now() < deadline,
                "tree operation did not report worktree-lock contention: {stderr}"
            );
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn finish(mut self) -> (std::process::Output, String) {
        let output = self.child.take().unwrap().wait_with_output().unwrap();
        let stderr = fs::read_to_string(&self.stderr_path).unwrap();
        (output, stderr)
    }
}

impl Drop for WaitingChild {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[test]
fn concurrent_root_initializers_wait_before_observing_or_creating_the_grove() {
    let worktree = init_repo();
    let external_guard = lock_worktree(worktree.path(), libc::LOCK_EX);

    let mut first = WaitingChild::spawn(
        worktree.path(),
        &["root-init"],
        worktree.path().join("first.stderr"),
    );
    let mut second = WaitingChild::spawn(
        worktree.path(),
        &["root-init"],
        worktree.path().join("second.stderr"),
    );

    first.wait_until_contended();
    second.wait_until_contended();
    assert!(
        !worktree.path().join(".grove").exists(),
        "root-init inspected or mutated the tree before acquiring the worktree lock"
    );

    unlock_worktree(&external_guard);
    let (first_output, first_stderr) = first.finish();
    let (second_output, second_stderr) = second.finish();
    let success_count = [first_output.status, second_output.status]
        .into_iter()
        .filter(std::process::ExitStatus::success)
        .count();
    assert_eq!(success_count, 1, "{first_stderr}\n{second_stderr}");
    assert_eq!(first_stderr.matches(WAITING_DIAGNOSTIC).count(), 1);
    assert_eq!(second_stderr.matches(WAITING_DIAGNOSTIC).count(), 1);
    assert!(worktree.path().join(".grove/FORMAT").is_file());
}

#[test]
fn reader_through_a_symlink_alias_waits_for_the_same_worktree_identity() {
    let worktree = init_repo();
    seed_current_grove(worktree.path());
    let alias_directory = TempDir::new().unwrap();
    let alias = alias_directory.path().join("worktree-alias");
    symlink(worktree.path(), &alias).unwrap();
    let external_guard = lock_worktree(worktree.path(), libc::LOCK_EX);

    let mut reader = WaitingChild::spawn(
        &alias,
        &["pick"],
        alias_directory.path().join("reader.stderr"),
    );
    reader.wait_until_contended();
    unlock_worktree(&external_guard);

    let (output, stderr) = reader.finish();
    assert!(output.status.success(), "{stderr}");
    assert_eq!(stderr.matches(WAITING_DIAGNOSTIC).count(), 1);
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("01-requirements-plan-k1.md"),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn mutator_waits_for_a_shared_worktree_reader_before_allocating_a_leaf() {
    let worktree = init_repo();
    seed_current_grove(worktree.path());
    let external_guard = lock_worktree(worktree.path(), libc::LOCK_SH);

    let mut writer = WaitingChild::spawn(
        worktree.path(),
        &["leaf-add", ".", "later"],
        worktree.path().join("writer.stderr"),
    );
    writer.wait_until_contended();
    assert!(
        !worktree.path().join(".grove/02-impl-later-k2.md").exists(),
        "leaf-add mutated the tree before acquiring the exclusive worktree lock"
    );
    unlock_worktree(&external_guard);

    let (output, stderr) = writer.finish();
    assert!(output.status.success(), "{stderr}");
    assert_eq!(stderr.matches(WAITING_DIAGNOSTIC).count(), 1);
    assert!(worktree.path().join(".grove/02-impl-later-k2.md").is_file());
}

#[test]
fn worktree_readers_share_the_lock_without_reporting_contention() {
    let worktree = init_repo();
    seed_current_grove(worktree.path());
    let external_guard = lock_worktree(worktree.path(), libc::LOCK_SH);

    let output = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .current_dir(worktree.path())
        .arg("pick")
        .output()
        .unwrap();
    unlock_worktree(&external_guard);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !String::from_utf8_lossy(&output.stderr).contains(WAITING_DIAGNOSTIC),
        "shared readers should not contend: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
