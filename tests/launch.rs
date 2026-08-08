mod support;

use grove::cli::{RetireArgs, StartArgs};
use grove::driver_lease::DriverLease;
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

// This build's own `grove-llm`, pinned through the `GROVE_LLM_BIN` seam: the
// kind peek `--no-launch` now runs (no-launch-config-check-k20) spawns it, and
// unpinned that is whatever the machine's PATH happens to carry.
const OWN_GROVE_LLM: &str = env!("CARGO_BIN_EXE_grove-llm");

// Scaffolding, not intent (mirroring tests/loop_driver.rs): model selection is
// required, so a dry run that resolves a kind has to find a model var for it.
// These tests are about provisioning, migration and stamping — the value is
// deliberately meaningless.
const SCAFFOLD_MODEL: &str = "scaffold-model";

fn write_exec(path: &std::path::Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut perms = fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).unwrap();
}

/// The env a `--no-launch` dry run needs to reach its own subject matter, now
/// that the flag runs both config checks before reporting: a harness binary that
/// exists (pre-flight is a PATH lookup), this build's `grove-llm` for the kind
/// peek, and a model var for each kind these fixtures actually resolve —
/// `requirements` on the start path (no `.grove/`), `impl` on the continue path.
///
/// `clear_grove_env` first, and load-bearingly: this repo dogfoods the routing
/// vars, and this suite may itself be running inside a rerouted session, so an
/// ambient `GROVE_IMPL_HARNESS` would reroute a dry run that never asked for one.
fn dry_run_env(repo: &std::path::Path) -> support::EnvGuard {
    let fake = repo.join("fake-harness.sh");
    write_exec(&fake, "#!/bin/sh\nexit 0\n");
    let mut env = support::EnvGuard::new();
    env.clear_grove_env()
        .remove("GROVE_HARNESS_BIN_CLAUDE")
        .remove("GROVE_HARNESS_BIN_CODEX")
        .remove("GROVE_HARNESS_BIN_PI")
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL)
        .set("GROVE_IMPL_MODEL", SCAFFOLD_MODEL);
    env
}

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    // `do_grove` provisions the global skill on launch
    // (self-extension-core-and-methodology); `load_prompt` reads its launcher
    // prompts from that same global dir, never a repo-local mirror.
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

fn git_head(worktree: &std::path::Path) -> Vec<u8> {
    let output = Command::new("git")
        .args(["-C", worktree.to_str().unwrap(), "rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(output.status.success());
    output.stdout
}

#[test]
fn do_reports_readiness_in_no_launch_mode() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let _env = dry_run_env(repo.path());

    // `do` never creates or touches git topology (user-owned-worktrees) — cwd's
    // toplevel *is* the worktree, and the grove name is its basename.
    launch::do_grove(&StartArgs {
        harness: None,
        no_launch: true,
    })
    .unwrap();
}

#[test]
fn no_launch_reports_readiness_while_a_driver_owns_the_worktree() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let _env = dry_run_env(repo.path());
    let _driver_lease = DriverLease::acquire(repo.path()).unwrap();

    launch::do_grove(&StartArgs {
        harness: None,
        no_launch: true,
    })
    .expect("a read-only readiness check must remain available beside a live driver");
}

#[test]
fn no_launch_does_not_migrate_a_live_drivers_legacy_tree() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let _env = dry_run_env(repo.path());
    fs::create_dir_all(repo.path().join(".grove")).unwrap();
    fs::write(repo.path().join(".grove/BRIEF.md"), "# test — brief\n").unwrap();
    fs::write(
        repo.path().join(".grove/1-[1]-test.md"),
        "# test-k1\n\n**Kind:** impl\n",
    )
    .unwrap();
    let head_before = git_head(repo.path());
    let _driver_lease = DriverLease::acquire(repo.path()).unwrap();

    launch::do_grove(&StartArgs {
        harness: None,
        no_launch: true,
    })
    .expect_err("a read-only check must report a pending migration without performing it");

    assert!(repo.path().join(".grove/1-[1]-test.md").exists());
    assert!(!repo.path().join(".grove/01-impl-test-k1.md").exists());
    assert!(!repo.path().join(".grove/FORMAT").exists());
    assert_eq!(git_head(repo.path()), head_before);
}

#[test]
fn do_runs_from_a_linked_worktree() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();

    // Precondition is "any working tree" (user-owned-worktrees): a linked
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
    let _env = dry_run_env(repo.path());

    launch::do_grove(&StartArgs {
        harness: None,
        no_launch: true,
    })
    .unwrap();
}

#[test]
fn do_provisions_the_global_skill_on_launch() {
    // `grove do` extracts the binary-embedded methodology to the global personal
    // skill dir (self-extension-core-and-methodology), so the launched session's
    // skill matches the binary.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();

    let _env = dry_run_env(repo.path());

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
    let _env = dry_run_env(repo.path());

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
fn do_stops_after_the_bounded_layout_migration_until_session_kind_migration() {
    // This increment retains the pre-existing layout adapter as bounded legacy
    // input for session-kind-migration-k27, but current readers deliberately
    // refuse its unwitnessed output. The later migration task owns deriving
    // filename kinds, writing FORMAT, and resuming the launch.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let _env = dry_run_env(repo.path());

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

    let error = launch::do_grove(&StartArgs {
        harness: None,
        no_launch: false,
    })
    .expect_err("an unwitnessed legacy tree must not be launched as current");
    assert!(
        error.to_string().contains("launch could not be resolved"),
        "legacy output must stop before launch: {error}"
    );

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
    assert!(
        !worktree.join(".grove/FORMAT").exists(),
        "the bounded layout adapter must not claim session-kind migration"
    );
}

#[test]
fn no_launch_does_not_stamp_the_grove() {
    // branch-review-k14 B3: `--no-launch` is documented as "report readiness,
    // don't exec" — a dry run. It must not durably rebind the grove, even with
    // an explicit `--harness` (the one case that would otherwise always stamp).
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let _env = dry_run_env(repo.path());
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
fn retire_never_stamps_the_grove() {
    // The other half of the same rule, and the one the help text got wrong for
    // as long as `RetireArgs::harness`'s doc comment was a byte-identical copy
    // of `StartArgs`' (retire-harness-stamp-claim-k23): a lasting binding is
    // written by the action that asks for one, so `grove do` writes the stamp
    // and `grove retire --harness` resolves a harness for its single by-hand
    // session and leaves the binding alone (model-per-task-kind).
    //
    // A **real** launch, deliberately, not the `--no-launch` dry run the
    // sibling above uses. `do`'s `maybe_stamp` sits *below* its no-launch
    // return, so a retire that grew one would put it in the same place — and a
    // dry-run assertion would pass on exactly the regression this exists to
    // catch. `exec_harness` has no bin seam, so the fake carries the harness's
    // own name and the launch really spawns it.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let _env = retire_launch_deps(repo.path(), "claude");
    let name = repo
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    // Explicit `--harness`: the trigger that always stamps on `grove do`, so
    // the absence below is this verb's own behaviour rather than `maybe_stamp`
    // declining a single-harness repo it was never asked about.
    launch::retire(&RetireArgs {
        path: "01-x-k1".into(),
        harness: Some("claude".into()),
        no_launch: false,
    })
    .unwrap();

    assert!(
        !grove::harness_stamp::path(repo.path(), &name).exists(),
        "retiring one node by hand must not rebind every later `grove do`"
    );
}

/// Plant a minimal current task tree with one leaf of `kind`, live or retired.
/// Committed so readiness observes the same stable witnessed tree as a launch.
fn plant_leaf(worktree: &std::path::Path, kind: &str, retired: bool) {
    let grove_dir = worktree.join(".grove");
    fs::create_dir_all(&grove_dir).unwrap();
    fs::write(grove_dir.join("BRIEF.md"), "# g — brief\n").unwrap();
    fs::write(grove_dir.join("FORMAT"), "session-kinds-v1\n").unwrap();
    let name = if retired {
        format!("01-DONE-{kind}-a-k1.md")
    } else {
        format!("01-{kind}-a-k1.md")
    };
    fs::write(grove_dir.join(name), "# a-k1\n").unwrap();
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
            "plant tree",
            "--no-verify",
        ])
        .status()
        .unwrap();
}

#[test]
fn no_launch_fails_when_the_next_leafs_kind_has_no_model_var() {
    // no-launch-config-check-k20, the measured case: with a required model var
    // unset, `--no-launch` used to print `ready` and exit 0 while the very next
    // real `grove do` died on that same var. A dry run that reports readiness
    // has to fail on everything a launch fails on, or "ready" is the one thing
    // it is not.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let mut env = dry_run_env(repo.path());
    env.remove("GROVE_IMPL_MODEL");
    plant_leaf(repo.path(), "impl", false);

    let err = launch::do_grove(&StartArgs {
        harness: None,
        no_launch: true,
    })
    .expect_err("a dry run must not report `ready` for a launch that would fail");

    // The same words the real launch would use — the dry run resolves through
    // the same code path, so the operator is told to set the same variables.
    let msg = err.to_string();
    for var in ["GROVE_CLAUDE_IMPL_MODEL", "GROVE_IMPL_MODEL"] {
        assert!(
            msg.contains(var),
            "the readiness failure must name {var} (got: {msg:?})"
        );
    }
    assert!(
        msg.contains("impl"),
        "the readiness failure must name the kind that went unconfigured (got: {msg:?})"
    );
}

#[test]
fn no_launch_still_succeeds_when_the_grove_has_no_live_leaves() {
    // The finish-cycle iteration is the required-model rule's first exemption
    // (model-per-task-kind): no live leaf means no task to require a model
    // *for*. A grove ready to finish must therefore still report ready, with
    // every model var unset — otherwise the new checks would make the last
    // `grove do` of a grove's life unrunnable.
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let mut env = dry_run_env(repo.path());
    env.remove("GROVE_IMPL_MODEL")
        .remove("GROVE_REQUIREMENTS_MODEL");
    plant_leaf(repo.path(), "impl", true);

    launch::do_grove(&StartArgs {
        harness: None,
        no_launch: true,
    })
    .expect("a grove with no live leaves has no kind to require a model for");
}

#[test]
fn the_readiness_report_names_the_next_leaf_its_kind_and_the_resolved_model() {
    // The report is what makes `--no-launch` an inspection tool rather than
    // merely non-committal: an operator checking a routing change wants to see
    // *which* leaf resolved to *which* model, not a bare "ready".
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let mut env = dry_run_env(repo.path());
    env.set("GROVE_IMPL_MODEL", "sonnet");
    plant_leaf(repo.path(), "impl", false);

    let claude = grove::harness::by_name("claude").unwrap();
    let line = grove::loop_driver::readiness(claude, repo.path())
        .unwrap()
        .to_string();

    assert!(
        line.contains("01-impl-a-k1.md") && line.contains("impl") && line.contains("sonnet"),
        "readiness must name the leaf, its kind and the model (got: {line:?})"
    );

    // A brand-new grove has no leaf to name — and says so, rather than
    // rendering the same "no leaf" as a finished one.
    let fresh = init_repo();
    std::env::set_current_dir(fresh.path()).unwrap();
    let line = grove::loop_driver::readiness(claude, fresh.path())
        .unwrap()
        .to_string();
    assert!(
        line.contains("bootstraps") && line.contains("requirements"),
        "a grove with no `.grove/` yet reports the bootstrap session (got: {line:?})"
    );
}

#[test]
fn readiness_retains_one_structured_peek_even_if_the_tree_changes_after_it() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    plant_leaf(repo.path(), "impl", false);
    let routed_path = repo
        .path()
        .join(".grove/01-impl-a-k1.md")
        .canonicalize()
        .unwrap();
    let calls = repo.path().join("peek-calls");
    let fake_llm = repo.path().join("fake-grove-llm.sh");
    write_exec(
        &fake_llm,
        r#"#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
printf 'peek\n' >> "$GROVE_TEST_PEEK_CALLS"
printf '{"path":"%s","handle":"a-k1","kind":"impl","harness":null,"review":null}\n' "$GROVE_TEST_ROUTE_PATH"
mv "$PWD/.grove/01-impl-a-k1.md" "$PWD/.grove/02-impl-a-k1.md"
printf '# earlier-k9\n' > "$PWD/.grove/01-design-earlier-k9.md"
"#,
    );
    let mut env = dry_run_env(repo.path());
    env.set("GROVE_LLM_BIN", &fake_llm)
        .set("GROVE_TEST_PEEK_CALLS", &calls)
        .set("GROVE_TEST_ROUTE_PATH", &routed_path)
        .set("GROVE_IMPL_MODEL", "sonnet");

    let line =
        grove::loop_driver::readiness(grove::harness::by_name("claude").unwrap(), repo.path())
            .unwrap()
            .to_string();

    assert!(
        line.contains("01-impl-a-k1.md") && !line.contains("01-design-earlier-k9.md"),
        "readiness must render the retained forecast, not pick again: {line:?}"
    );
    assert_eq!(fs::read_to_string(calls).unwrap(), "peek\n");
}

#[test]
fn malformed_or_handle_free_structured_peeks_refuse_readiness() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    plant_leaf(repo.path(), "impl", false);
    let path = repo
        .path()
        .join(".grove/01-impl-a-k1.md")
        .canonicalize()
        .unwrap();
    let fake_llm = repo.path().join("fake-grove-llm.sh");
    write_exec(
        &fake_llm,
        r#"#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
printf '%s\n' "$GROVE_TEST_PEEK_PAYLOAD"
"#,
    );
    let mut env = dry_run_env(repo.path());
    env.set("GROVE_LLM_BIN", &fake_llm);

    let handle_free = format!(
        "{{\"path\":\"{}\",\"kind\":\"impl\",\"harness\":null}}",
        grove::json::escape(&path.display().to_string())
    );
    for payload in ["not-json", handle_free.as_str()] {
        env.set("GROVE_TEST_PEEK_PAYLOAD", payload);
        let error =
            grove::loop_driver::readiness(grove::harness::by_name("claude").unwrap(), repo.path())
                .err()
                .expect("an uncheckable routing peek must refuse before launch")
                .to_string();
        assert!(
            error.contains("launch could not be resolved"),
            "the refusal must use the routing no-guess contract: {error}"
        );
    }
}

#[test]
fn a_failed_provision_never_leaves_a_permanent_stamp() {
    // branch-review-k14 B4: the stamp must only be written once every step that
    // could bail (provisioning, PATH resolution) has already succeeded —
    // otherwise a failed bind is permanent, with no verb to clear it.
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
        "a failed provision must not leave a permanent bad binding \
         (branch-review-k14 B4)"
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
        .set("GROVE_REVIEW_IMPL_HARNESS", "pi");

    let result = launch::do_grove(&StartArgs {
        harness: None,
        no_launch: false,
    });

    let err = result.expect_err("a missing rerouted harness binary must fail pre-flight");
    let msg = err.to_string();
    assert!(
        msg.contains("GROVE_REVIEW_IMPL_HARNESS"),
        "diagnostic must name the override var (got: {msg:?})"
    );
    assert!(
        msg.contains(&missing_pi.display().to_string()),
        "diagnostic must name the missing binary (got: {msg:?})"
    );

    assert!(
        !grove::harness_stamp::path(repo.path(), &name).exists(),
        "a pre-flight failure must not leave a permanent stamp \
         (branch-review-k14 B4)"
    );

    // …and the dry run reports the same failure (no-launch-config-check-k20):
    // pre-flight now runs *above* the no-launch return, so `--no-launch` can no
    // longer print `ready` for an environment whose next real launch would die
    // on an uninstalled rerouted harness.
    let err = launch::do_grove(&StartArgs {
        harness: None,
        no_launch: true,
    })
    .expect_err("a dry run must fail pre-flight wherever a real launch would");
    assert!(
        err.to_string().contains("GROVE_REVIEW_IMPL_HARNESS"),
        "the dry run's diagnostic must name the override var too (got: {err})"
    );
}

/// Plant a fake argv-logging `codex` on PATH plus the retire prompt under
/// `skill_root` (the dir init helpers point GROVE_SKILL_DIR into), run a
/// codex `retire` from the current cwd, and return the launch's logged argv
/// line. `exec_harness` has no bin seam — it execs `harness.exec_bin` through
/// PATH — which is why the fake goes there. Callers hold CWD_LOCK.
///
/// The fake carries [`support::CODEX_SANDBOX_PROBE_REPLY`], because a codex
/// retire is pre-flighted exactly as a codex loop launch is: it commits, so it
/// dies on the same read-only refusal and wants the same diagnostic. Asserting
/// the log holds **one** line is therefore load-bearing rather than pedantic —
/// it is what pins the probe as a probe, so these `--add-dir` assertions can
/// never be quietly summing two invocations.
fn codex_retire_argv(skill_root: &std::path::Path) -> String {
    let bindir = skill_root.join("bin");
    fs::create_dir_all(&bindir).unwrap();
    let log = skill_root.join("log");
    let fake = bindir.join("codex");
    fs::write(
        &fake,
        format!(
            "#!/bin/sh\n{probe}printf '%s\\n' \"$*\" >> \"{log}\"\nexit 0\n",
            probe = support::CODEX_SANDBOX_PROBE_REPLY,
            log = log.display()
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

    let log = fs::read_to_string(&log).unwrap();
    let mut lines = log.lines();
    let argv = lines
        .next()
        .unwrap_or_else(|| panic!("the fake codex logged nothing (log: {log:?})"))
        .to_string();
    assert_eq!(
        lines.next(),
        None,
        "the sandbox pre-flight must not reach the argv log — it is a probe, \
         not a launch (log: {log:?})"
    );
    argv
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

/// Everything a `grove retire` **launch** depends on, planted so a `--no-launch`
/// dry run reaches its own subject matter — the retire dry run now resolves the
/// launch it declines to perform (retire-no-launch-help-k21), so its
/// preconditions are the launch's.
///
/// Both are retire-specific and neither is covered by [`dry_run_env`].
/// `grove retire` **never provisions**, so its prompt has to be planted by hand
/// in the global skill dir `init_repo` pointed `GROVE_SKILL_DIR` at (the same
/// reason `codex_retire_argv` plants one). And `exec_harness` has no bin seam —
/// it execs `harness.exec_bin` through PATH, which `GROVE_HARNESS_BIN` does not
/// redirect — so the fake must carry the harness's own name.
fn retire_launch_deps(skill_root: &std::path::Path, harness_bin: &str) -> support::EnvGuard {
    let prompts = skill_root.join("global-skill/prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("retire.md"), "RETIRE {{NODE_PATH}}").unwrap();

    let bindir = skill_root.join("bin");
    fs::create_dir_all(&bindir).unwrap();
    write_exec(&bindir.join(harness_bin), "#!/bin/sh\nexit 0\n");

    let mut env = support::EnvGuard::new();
    env.set(
        "PATH",
        format!(
            "{}:{}",
            bindir.display(),
            std::env::var("PATH").unwrap_or_default()
        ),
    );
    env
}

#[test]
fn retire_resolves_a_bare_node_path_in_worktree() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    let _env = retire_launch_deps(repo.path(), "claude");

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

#[test]
fn the_retire_dry_run_fails_when_the_prompt_it_would_load_is_not_provisioned() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    // Deliberately *not* `retire_launch_deps`: the bin is planted, the prompt is
    // not. `grove retire` never provisions, so this is the state of any machine
    // whose `grove do` has not yet run for this harness — the one launch
    // dependency a user cannot see, and the one the dry run used to sit on top
    // of while reporting that it would exec.
    let bindir = repo.path().join("bin");
    fs::create_dir_all(&bindir).unwrap();
    write_exec(&bindir.join("claude"), "#!/bin/sh\nexit 0\n");
    let mut env = support::EnvGuard::new();
    env.set(
        "PATH",
        format!(
            "{}:{}",
            bindir.display(),
            std::env::var("PATH").unwrap_or_default()
        ),
    );

    let err = launch::retire(&RetireArgs {
        path: "03-some-leaf-k3".into(),
        harness: None,
        no_launch: true,
    })
    .expect_err("a dry run must fail on the prompt a real launch would fail on");
    assert!(
        err.to_string().contains("retire.md"),
        "the diagnostic must name the prompt it could not read (got: {err})"
    );
}

#[test]
fn the_retire_dry_run_fails_when_the_binary_it_would_exec_is_absent() {
    let _g = CWD_LOCK.lock().unwrap();
    let repo = init_repo();
    std::env::set_current_dir(repo.path()).unwrap();
    // The prompt resolves; only the exec's own precondition is missing. Checking
    // it is the one place the dry run cannot share the launch's code path — the
    // launch finds out by trying — so it is checked explicitly, and against
    // `harness.exec_bin` rather than the loop's overridable bin name.
    let prompts = repo.path().join("global-skill/prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("retire.md"), "RETIRE {{NODE_PATH}}").unwrap();

    // A PATH holding `git` **and nothing else**, rather than an empty one:
    // `repo::toplevel` shells out to git before any of this is reached, so an
    // empty PATH fails one step too early and would pass on a build where the
    // harness check had been deleted. Linking the real git in is what makes
    // "claude is absent" a fact about this PATH instead of a fact about the
    // machine the suite happens to run on.
    let bindir = repo.path().join("git-only-bin");
    fs::create_dir_all(&bindir).unwrap();
    let git = String::from_utf8(
        Command::new("sh")
            .args(["-c", "command -v git"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    std::os::unix::fs::symlink(git.trim(), bindir.join("git")).unwrap();
    let mut env = support::EnvGuard::new();
    env.set("PATH", &bindir);

    let err = launch::retire(&RetireArgs {
        path: "03-some-leaf-k3".into(),
        harness: None,
        no_launch: true,
    })
    .expect_err("`would exec claude` is a claim, and an absent claude falsifies it");
    assert!(
        err.to_string().contains("claude is not on PATH"),
        "the diagnostic must name the binary it would have exec'd (got: {err})"
    );
}

/// Harvest every token in `help` that **offers itself as a tree-entry name** —
/// a word whose leading digit run is followed by `-`, which under
/// task-tree-scheme's grammar can only be a position prefix. Path segments are
/// split, so a nested example is checked segment by segment, and the
/// punctuation a doc comment puts around an example (backticks, parens, commas)
/// is trimmed. The digits-then-dash shape is what keeps an incidental number in
/// the prose (a version, a count) out of the candidate set.
#[test]
fn retire_is_not_exposed_by_the_bare_human_cli() {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_grove"))
        .args(["retire", "--help"])
        .output()
        .unwrap();
    assert!(
        !out.status.success(),
        "the config-driven cutover must reject the retired human verb: {out:?}"
    );
}
