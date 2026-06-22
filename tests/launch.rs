use grove::cli::{NameArgs, StartArgs};
use grove::launch;
use std::fs;
use std::process::Command;
use std::sync::Mutex;
use tempfile::TempDir;

// These tests all mutate process-global cwd; serialize so cargo's parallel
// test runner doesn't have one test's cwd swept out from under another's
// repo::resolve(None) call.
static CWD_LOCK: Mutex<()> = Mutex::new(());

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    // `do_grove` provisions the global skill on launch (070/010); `load_prompt`
    // reads its launcher prompts from that same global dir (the 9.3 repoint).
    // Point both at a throwaway dir inside the repo so the suite never touches
    // the real ~/.claude/skills/grove. Safe under CWD_LOCK (all callers
    // serialize), and unprovisioned until a `do` runs (so the provision test can
    // assert it is created).
    std::env::set_var("GROVE_SKILL_DIR", tmp.path().join("global-skill"));
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
            "-C",
            tmp.path().to_str().unwrap(),
            "commit",
            "-m",
            "init",
            "--no-verify",
        ])
        .status()
        .unwrap();

    fs::create_dir_all(tmp.path().join(".claude")).unwrap();
    tmp
}

#[test]
fn start_creates_worktree_in_no_launch_mode() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
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
    let repo = init_repo();
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
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();

    // `do` is the sole entry verb; on an unknown grove it takes the start
    // path and honours --start-point (StartArgs.start_point).
    launch::do_grove(&StartArgs {
        name: "fresh".into(),
        start_point: Some("main".into()),
        harness: None,
        no_launch: true,
    })
    .unwrap();

    assert!(repo.path().join(".grove-worktrees/fresh").is_dir());
}

#[test]
fn do_provisions_the_global_skill_on_launch() {
    // `grove do` extracts the binary-embedded methodology to the global personal
    // skill dir (070/010), so the launched session's skill matches the binary.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();

    // The helper points GROVE_SKILL_DIR at <repo>/global-skill; it should not
    // exist until `do` provisions it.
    let skill_dir = repo.path().join("global-skill");
    assert!(!skill_dir.exists(), "skill dir absent before `do`");

    launch::do_grove(&StartArgs {
        name: "prov".into(),
        start_point: Some("main".into()),
        harness: None,
        no_launch: true,
    })
    .unwrap();

    assert!(
        skill_dir.join("SKILL.md").is_file(),
        "do provisions the embedded methodology to the global skill dir"
    );
}

#[test]
fn do_continues_when_worktree_is_live() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
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
    launch::do_grove(&StartArgs {
        name: "alive".into(),
        start_point: None,
        harness: None,
        no_launch: true,
    })
    .unwrap();
}

#[test]
fn do_migrates_an_old_tree_on_adoption_before_driving() {
    // `grove do` must flip an old-format `.grove/` to the new scheme (committed)
    // before driving — even in no-launch mode, which runs the adoption setup then
    // returns without launching a session.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();

    // Stand up a live worktree, then plant an old-format tree in it and commit.
    launch::start(&StartArgs {
        name: "legacy".into(),
        start_point: Some("main".into()),
        harness: None,
        no_launch: true,
    })
    .unwrap();
    let worktree = repo.path().join(".grove-worktrees/legacy");
    fs::create_dir_all(worktree.join(".grove/done")).unwrap();
    fs::write(worktree.join(".grove/BRIEF.md"), "# proj — brief\n").unwrap();
    fs::write(worktree.join(".grove/done/010-old.md"), "# 010-old\n").unwrap();
    fs::write(worktree.join(".grove/020-live.md"), "# 020-live\n").unwrap();
    Command::new("git")
        .args(["-C", worktree.to_str().unwrap(), "add", "-A"])
        .status()
        .unwrap();
    Command::new("git")
        .args([
            "-C",
            worktree.to_str().unwrap(),
            "commit",
            "-q",
            "-m",
            "plant old tree",
            "--no-verify",
        ])
        .status()
        .unwrap();

    launch::do_grove(&StartArgs {
        name: "legacy".into(),
        start_point: None,
        harness: None,
        no_launch: true,
    })
    .unwrap();

    // The old directory layout is gone; the new flat keyed files are present.
    assert!(
        !worktree.join(".grove/done").exists(),
        "old done/ dir should be migrated away"
    );
    assert!(
        worktree.join(".grove/1-[1]-old.DONE.md").exists(),
        "retired leaf should be migrated to its new flat name"
    );
    assert!(
        worktree.join(".grove/2-[2]-live.md").exists(),
        "live leaf should be migrated to its new flat name"
    );
}

#[test]
fn do_reattaches_orphaned_worktree() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
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

    launch::do_grove(&StartArgs {
        name: "orphan".into(),
        start_point: None,
        harness: None,
        no_launch: true,
    })
    .unwrap();

    assert!(repo.path().join(".grove-worktrees/orphan").is_dir());
}
