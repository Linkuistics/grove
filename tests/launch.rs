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

/// Plant a fake argv-logging `codex` on PATH plus the retire prompt under
/// `skill_root` (the dir init helpers point GROVE_SKILL_DIR into), run a
/// codex `retire` from the current cwd, and return the logged argv line.
/// `exec_harness` has no bin seam — it execs `harness.exec_bin` through
/// PATH — which is why the fake goes there. Callers hold CWD_LOCK.
fn codex_retire_argv(skill_root: &std::path::Path) -> String {
    let bindir = skill_root.join("bin");
    fs::create_dir_all(&bindir).unwrap();
    let log = skill_root.join("log");
    let fake = bindir.join("codex");
    fs::write(
        &fake,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"{}\"\nexit 0\n",
            log.display()
        ),
    )
    .unwrap();
    let mut perms = fs::metadata(&fake).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&fake, perms).unwrap();

    // `retire` (unlike `do`) does not provision; plant its prompt by hand in
    // the global skill dir the init helper pointed GROVE_SKILL_DIR at.
    let prompts = skill_root.join("global-skill/prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("retire.md"), "RETIRE {{NODE_PATH}}").unwrap();

    let path_var = format!(
        "{}:{}",
        bindir.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let mut env = support::EnvGuard::new();
    env.set("PATH", &path_var);

    launch::retire(&RetireArgs {
        path: "01-x-k1".into(),
        harness: Some("codex".into()),
        no_launch: false,
    })
    .unwrap();

    fs::read_to_string(&log).unwrap()
}

/// The `--add-dir` values in `argv`, canonicalized for comparison against
/// fixture paths (TempDirs live behind the /var → /private/var symlink).
fn granted_dirs(argv: &str) -> Vec<std::path::PathBuf> {
    support::add_dir_values(argv)
        .into_iter()
        .map(|p| std::path::Path::new(p).canonicalize().unwrap())
        .collect()
}

/// A jj-enabled fixture: native (`.jj/` only) or colocated (`.git` beside
/// it), with GROVE_SKILL_DIR pointed inside like init_repo does. jj runs
/// with a test-local identity so no global config is required.
fn init_jj_repo(colocate: bool) -> TempDir {
    let tmp = TempDir::new().unwrap();
    std::env::set_var("GROVE_SKILL_DIR", tmp.path().join("global-skill"));
    run_jj(
        tmp.path(),
        if colocate {
            &["git", "init", "--colocate", "--quiet", "."]
        } else {
            &[
                "--config",
                "git.colocate=false",
                "git",
                "init",
                "--quiet",
                ".",
            ]
        },
    );
    tmp
}

fn run_jj(dir: &std::path::Path, args: &[&str]) {
    let mut full = vec![
        "--config",
        "user.name=Test",
        "--config",
        "user.email=t@example.com",
    ];
    full.extend_from_slice(args);
    let out = Command::new("jj")
        .current_dir(dir)
        .args(&full)
        .output()
        .unwrap_or_else(|e| panic!("running jj {args:?}: {e} (is jj installed?)"));
    assert!(
        out.status.success(),
        "jj {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn retire_on_codex_grants_the_gitdir_via_add_dir() {
    // codex-gitdir-grant applies to *every* codex launch, not just the loop's:
    // a `grove retire` session commits too, and would hit the same read-only
    // gitdir carve-out.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();

    let argv = codex_retire_argv(repo.path());
    assert_eq!(
        granted_dirs(&argv),
        vec![repo.path().join(".git").canonicalize().unwrap()],
        "the retire session's grant is the checkout's own `.git` (argv: {argv:?})"
    );
}

#[test]
fn retire_on_codex_in_jj_native_tree_grants_the_jj_store() {
    // A jj-native tree has no `.git` for `git rev-parse` to find — the grant
    // derivation must go through jj instead of erroring the launch outright.
    // The granted store is the main workspace's `.jj`: redundant here (it
    // sits under the sandbox cwd, and codex carves out only `.git`, verified
    // by probe), but load-bearing from a secondary workspace, and grants are
    // additive so the uniform rule costs nothing.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_jj_repo(false);
    std::env::set_current_dir(repo.path()).unwrap();

    let argv = codex_retire_argv(repo.path());
    assert_eq!(
        granted_dirs(&argv),
        vec![repo.path().join(".jj").canonicalize().unwrap()],
        "a jj-native launch grants the `.jj` store and nothing else (argv: {argv:?})"
    );
}

#[test]
fn retire_on_codex_in_colocated_tree_grants_jj_and_git_stores() {
    // Colocated: jj's git backend writes commit objects and exported refs
    // into `.git`, which the sandbox carves out of the cwd root (probe:
    // `jj describe` fails in-sandbox without the grant, succeeds with it).
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_jj_repo(true);
    std::env::set_current_dir(repo.path()).unwrap();

    let argv = codex_retire_argv(repo.path());
    assert_eq!(
        granted_dirs(&argv),
        vec![
            repo.path().join(".jj").canonicalize().unwrap(),
            repo.path().join(".git").canonicalize().unwrap(),
        ],
        "a colocated launch grants both stores (argv: {argv:?})"
    );
}

#[test]
fn retire_on_codex_in_secondary_jj_workspace_grants_the_main_workspace_store() {
    // A secondary workspace's own `.jj/` holds only the working copy; every
    // op lands in the *main* workspace's `.jj/repo`, outside the sandbox cwd
    // entirely (probe: `jj describe` fails there without the grant). The
    // grant must therefore name the main workspace's `.jj`, not the local one.
    let _g = CWD_LOCK.lock().unwrap();
    let tmp = TempDir::new().unwrap();
    std::env::set_var("GROVE_SKILL_DIR", tmp.path().join("global-skill"));
    let main = tmp.path().join("main");
    fs::create_dir_all(&main).unwrap();
    run_jj(
        &main,
        &[
            "--config",
            "git.colocate=false",
            "git",
            "init",
            "--quiet",
            ".",
        ],
    );
    let ws = tmp.path().join("ws2");
    run_jj(
        &main,
        &["workspace", "add", "--quiet", ws.to_str().unwrap()],
    );
    std::env::set_current_dir(&ws).unwrap();

    let argv = codex_retire_argv(tmp.path());
    assert_eq!(
        granted_dirs(&argv),
        vec![main.join(".jj").canonicalize().unwrap()],
        "a secondary-workspace launch grants the main workspace's `.jj` (argv: {argv:?})"
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
