mod support;

use grove::cli::{RetireArgs, StartArgs};
use grove::launch;
use std::fs;
use std::os::unix::fs::PermissionsExt;
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
fn do_reports_readiness_in_no_launch_mode() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();

    // `do` never creates or touches git topology (user-owned-worktrees) — cwd's
    // toplevel *is* the worktree, and the grove name is its basename.
    launch::do_grove(&StartArgs {
        harness: None,
        no_launch: true,
    })
    .unwrap();
}

#[test]
fn do_runs_from_a_linked_worktree() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();

    // Precondition is "any git working tree" (user-owned-worktrees): a linked
    // worktree the test creates itself, not a `.grove-worktrees/<name>/` grove
    // ever provisions.
    let linked = repo.path().join("scratch-worktree");
    let status = Command::new("git")
        .args(["-C", repo.path().to_str().unwrap(), "worktree", "add"])
        .arg(&linked)
        .arg("-b")
        .arg("feature")
        .status()
        .unwrap();
    assert!(status.success(), "git worktree add failed");

    std::env::set_current_dir(&linked).unwrap();

    launch::do_grove(&StartArgs {
        harness: None,
        no_launch: true,
    })
    .unwrap();
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
fn do_is_idempotent_on_the_same_working_tree() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();

    // `do` is the sole entry verb for every state (do-is-sole-lifecycle-verb):
    // running it again from the same working tree must still succeed.
    launch::do_grove(&StartArgs {
        harness: None,
        no_launch: true,
    })
    .unwrap();
    launch::do_grove(&StartArgs {
        harness: None,
        no_launch: true,
    })
    .unwrap();
}

#[test]
fn do_migrates_an_old_tree_on_adoption_before_driving() {
    // `grove do` must flip an old-format `.grove/` to the v2 directory scheme
    // (committed) before driving — even in no-launch mode, which runs the adoption
    // setup then returns without launching a session.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();

    let worktree = repo.path();
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
        harness: None,
        no_launch: true,
    })
    .unwrap();

    // The old directory layout is gone; the v2 keyed files are present (keys
    // assigned in DFS pre-order: the retired `old` leaf k1, the live one k2).
    assert!(
        !worktree.join(".grove/done").exists(),
        "old done/ dir should be migrated away"
    );
    assert!(
        worktree.join(".grove/01-DONE-old-k1.md").exists(),
        "retired leaf should be migrated to its v2 name"
    );
    assert!(
        worktree.join(".grove/02-live-k2.md").exists(),
        "live leaf should be migrated to its v2 name"
    );
}

#[test]
fn no_launch_does_not_stamp_the_grove() {
    // B3: `--no-launch` is documented as "report readiness, don't exec" — a
    // dry run. It must not durably rebind the grove, even with an explicit
    // `--harness` (the one case that would otherwise always stamp).
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let name = repo
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    launch::do_grove(&StartArgs {
        harness: Some("claude".into()),
        no_launch: true,
    })
    .unwrap();

    assert!(
        !grove::harness_stamp::path(repo.path(), &name).exists(),
        "a documented dry run must not permanently rebind the grove"
    );
}

#[test]
fn a_failed_provision_never_leaves_a_permanent_stamp() {
    // B4: the stamp must only be written once every step that could bail
    // (provisioning, PATH resolution) has already succeeded — otherwise a
    // failed bind is permanent, with no verb to clear it.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let name = repo
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    // init_repo() points GROVE_SKILL_DIR at <repo>/global-skill; pre-populate
    // it with foreign, unstamped content so provisioning refuses to touch it.
    let skill_dir = repo.path().join("global-skill");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("precious.txt"), "not ours").unwrap();

    let result = launch::do_grove(&StartArgs {
        harness: Some("claude".into()),
        no_launch: false,
    });

    assert!(result.is_err(), "a foreign skill dir must still bail");
    assert!(
        !grove::harness_stamp::path(repo.path(), &name).exists(),
        "a failed provision must not leave a permanent bad binding (B4)"
    );
}

#[test]
fn do_fails_preflight_when_a_per_kind_override_binary_is_missing() {
    // harness-spawn-preflight-k8: the pre-flight check used to validate only
    // the stamped harness's binary. A per-kind `GROVE_<KIND>_HARNESS`
    // override whose binary is missing must now fail *here* — before the
    // loop starts — not mid-loop on the first leaf of that kind.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let name = repo
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let fake_claude = repo.path().join("fake-claude.sh");
    fs::write(&fake_claude, "#!/bin/sh\nexit 0\n").unwrap();
    let mut perms = fs::metadata(&fake_claude).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&fake_claude, perms).unwrap();

    let missing_pi = repo.path().join("no-such-pi");

    let mut env = support::EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake_claude)
        .set("GROVE_HARNESS_BIN_PI", &missing_pi)
        .set("GROVE_REVIEW_HARNESS", "pi");

    let result = launch::do_grove(&StartArgs {
        harness: None,
        no_launch: false,
    });

    let err = result.expect_err("a missing rerouted harness binary must fail pre-flight");
    let msg = err.to_string();
    assert!(
        msg.contains("GROVE_REVIEW_HARNESS"),
        "diagnostic must name the override var (got: {msg:?})"
    );
    assert!(
        msg.contains(&missing_pi.display().to_string()),
        "diagnostic must name the missing binary (got: {msg:?})"
    );

    assert!(
        !grove::harness_stamp::path(repo.path(), &name).exists(),
        "a pre-flight failure must not leave a permanent stamp (B4)"
    );
}

#[test]
fn retire_resolves_a_bare_node_path_in_worktree() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();

    // `retire` takes a bare in-worktree node path — no `<name>/` prefix. The
    // two-part `<name>/<node-path>` addressing died with the canonical
    // `.grove-worktrees/<name>/` layout (user-owned-worktrees).
    launch::retire(&RetireArgs {
        path: "03-some-leaf-k3".into(),
        harness: None,
        no_launch: true,
    })
    .unwrap();
}
