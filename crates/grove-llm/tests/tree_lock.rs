use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use tempfile::TempDir;

mod support;

const WAITING_DIAGNOSTIC: &str = "waiting for active Grove tree operation";

fn init_repo() -> TempDir {
    let temporary_directory = TempDir::new().unwrap();
    support::init_jj_repo(temporary_directory.path());
    temporary_directory
}

fn seed_current_grove(worktree: &Path) {
    let output = Command::new(support::grove_llm())
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
        let child = Command::new(support::grove_llm())
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
    assert!(worktree
        .path()
        .join(".grove/01-requirements--plan-k1.md")
        .is_file());
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
        String::from_utf8_lossy(&output.stdout).contains("01-requirements--plan-k1.md"),
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
        &["leaf-add", ".", "later", "--kind", "impl"],
        worktree.path().join("writer.stderr"),
    );
    writer.wait_until_contended();
    assert!(
        !worktree.path().join(".grove/02-impl--later-k2.md").exists(),
        "leaf-add mutated the tree before acquiring the exclusive worktree lock"
    );
    unlock_worktree(&external_guard);

    let (output, stderr) = writer.finish();
    assert!(output.status.success(), "{stderr}");
    assert_eq!(stderr.matches(WAITING_DIAGNOSTIC).count(), 1);
    assert!(worktree
        .path()
        .join(".grove/02-impl--later-k2.md")
        .is_file());
}

#[test]
fn worktree_readers_share_the_lock_without_reporting_contention() {
    let worktree = init_repo();
    seed_current_grove(worktree.path());
    let external_guard = lock_worktree(worktree.path(), libc::LOCK_SH);

    let output = Command::new(support::grove_llm())
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

/// **No Grove reader observes a promotion's intermediate state**, held as a
/// property of the source rather than of a race that would have to be provoked.
///
/// A promotion is the one operation whose intermediate state breaks an invariant:
/// the node exists before the leaf's content can move into it, and it carries the
/// leaf's own ordinal and key, so between those two effects both are on disk
/// sharing both (`docs/ordinal-fs-tree/ARCHITECTURE.md`, *Promotion is not atomic
/// against the invariants*). The library's invariants therefore hold of
/// **quiescent** trees, and the exclusive lock is what makes that safe — for
/// *cooperating* readers.
///
/// Grove's readers cooperate for one reason now, and it is the stronger one:
/// **there is only one lock.** The library's own `flock` is taken from exactly
/// one module, so no snapshot exists that was not taken under it — and Grove no
/// longer keeps a guard beside it. It used to, for the two things the library
/// could not do (create the root, create its distinguished child), and the two
/// `flock`ed the same directory through different open file descriptions, so
/// they had to run in *phases* rather than nested. `collapse-tree-access-k13`
/// deleted that layer: the store answers an absent tree with a `Vacancy` that
/// holds the exclusive lock, and creates the root, the charter and the first
/// leaf under it.
///
/// Enumerated rather than listed: the scan is every `.rs` file grove ships
/// (`support::grove_sources`), so a verb that reaches for `ordinal_fs_tree::fs::`
/// in a module of its own fails here whether or not anyone remembered to add it.
/// The control is the count — the call sites must still exist, so a rename that
/// hides them fails too. Since `loop-crate-driver-k22` that walk covers **five**
/// packages under `crates/` and no repository-root `src/` at all, which is the
/// narrowing that would otherwise have made this pass by looking in the wrong
/// place.
#[test]
fn the_librarys_tree_lock_is_taken_from_exactly_one_module() {
    let mut callers: Vec<(String, usize)> = Vec::new();
    for (relative, body) in support::grove_sources() {
        // Any **non-comment** mention of the library's `fs` module, and not just
        // the turbofished call spelling: a caller writing `use ordinal_fs_tree::fs;`
        // and then `fs::write(...)` would evade a narrower pattern, and that is
        // exactly the caller this test exists to catch. Doc links are skipped
        // because they name the module without reaching it.
        let hits = body
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .filter(|line| line.contains("ordinal_fs_tree::fs"))
            .count();
        if hits > 0 {
            callers.push((relative, hits));
        }
    }
    callers.sort();
    assert_eq!(
        callers,
        vec![("crates/grove-loop/src/task_tree.rs".to_string(), 5)],
        "the library's lock is `task_tree`'s to take: two guard type aliases, the \
         two acquisitions themselves — shared and exclusive — and the one import \
         of `Reading`/`Writing`/`Vacancy`, which are the shapes those acquisitions \
         now answer with. Every reader in Grove goes through them, and the vacancy \
         is re-exported from there so the one verb group that creates a tree does \
         not reach for the module itself. The count is the control — a pattern \
         that stopped matching would leave this empty rather than clean."
    );
}

/// **Grove never waits on a lock of its own.**
///
/// `collapse-tree-access-k13` deleted the second layer, and the failure mode it
/// removes is why that layer could never simply be *added to*: two open file
/// descriptions on one directory do not share a `flock`, so a verb holding
/// Grove's guard that called into the library blocked forever — on itself. The
/// scaffold's two phases were the workaround, and the torn tree they could leave
/// behind was the price.
///
/// What survives is the property rather than an absence: **every `flock` Grove
/// takes in production is non-blocking**. Waiting belongs to the store, on the
/// store's own lock, and nothing else grove ships may wait at all. Grove's two
/// remaining lockers both satisfy it by design — the one-driver-per-workspace
/// lease *refuses* a contender rather than queueing behind it, and
/// `task_tree`'s contention probe acquires non-blockingly only to learn whether
/// to print, then releases. A blocking acquisition anywhere here is the deleted
/// layer growing back, and its symptom is a hang rather than a failure, which is
/// exactly the kind of regression a test has to catch before a human does.
///
/// Enumerated, like its neighbour above: every `.rs` file grove ships is
/// scanned, so a module that grows a guard of its own is checked whether or not
/// anyone thought to look. Unit tests are cut at their `#[cfg(test)]` boundary —
/// a fixture that *deliberately* blocks to provoke contention is not production
/// waiting on itself.
#[test]
fn no_production_lock_grove_takes_for_itself_ever_blocks() {
    let mut blocking: Vec<String> = Vec::new();
    let mut checked = 0_usize;
    for (relative, body) in support::grove_sources() {
        // A whole file of tests, named as one. Nothing in it is production.
        if relative.ends_with("tests.rs") {
            continue;
        }
        let production = body
            .split_once("#[cfg(test)]")
            .map_or(body.as_str(), |(before, _)| before);
        for (number, line) in production.lines().enumerate() {
            if line.trim_start().starts_with("//") || !line.contains("libc::flock") {
                continue;
            }
            checked += 1;
            if !line.contains("LOCK_NB") && !line.contains("LOCK_UN") {
                blocking.push(format!("{relative}:{}: {}", number + 1, line.trim()));
            }
        }
    }

    assert!(
        checked >= 2,
        "the scan matched {checked} `flock` calls, and Grove has at least the \
         lease's and the contention probe's — a mis-scoped scan reports a clean \
         tree for the wrong reason"
    );
    assert!(
        blocking.is_empty(),
        "these wait on a lock Grove took for itself. Waiting is the store's, on \
         the store's own lock; a second layer beside it deadlocks the process \
         against its own descriptor, which is what `collapse-tree-access-k13` \
         deleted and what a hang rather than a failure looks like:\n{}",
        blocking.join("\n")
    );
}
