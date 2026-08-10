//! What survives of the legacy launcher's test surface: the **routing readiness
//! report** (`loop_driver::readiness`).
//!
//! The `grove do` / `grove migrate` / `grove retire` verbs, `--harness`,
//! `--no-launch` and the harness stamp are gone — implementation and tests
//! alike. `readiness` outlived its only caller (`--no-launch`) because it is
//! routing machinery rather than command surface, so it is covered here until
//! the routing contraction removes it.

mod support;

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
// readiness report's kind peek spawns it, and unpinned that is whatever the
// machine's PATH happens to carry.
const OWN_GROVE_LLM: &str = env!("CARGO_BIN_EXE_grove-llm");

// Scaffolding, not intent (mirroring tests/loop_driver.rs): model selection is
// required, so a readiness report that resolves a kind has to find a model var
// for it. These tests are about what the report *names*; the value is
// deliberately meaningless.
const SCAFFOLD_MODEL: &str = "scaffold-model";

fn write_exec(path: &std::path::Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut perms = fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).unwrap();
}

/// The env a readiness report needs to reach its own subject matter: a harness
/// binary that exists, this build's `grove-llm` for the kind peek, and a model
/// var for each kind these fixtures actually resolve.
///
/// `clear_grove_env` first, and load-bearingly: this repo dogfoods the routing
/// vars, and this suite may itself be running inside a rerouted session, so an
/// ambient `GROVE_IMPL_HARNESS` would reroute a report that never asked for one.
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
    // Point the global skill dir at a throwaway dir inside the repo so the
    // suite never touches the real ~/.claude/skills/grove. Safe under
    // CWD_LOCK (all callers serialize).
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
fn the_readiness_report_names_the_next_leaf_its_kind_and_the_resolved_model() {
    // The report is what makes readiness an inspection value rather than a bare
    // "ready": a caller checking a routing change wants to see *which* leaf
    // resolved to *which* model.
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

// The removed human verbs stay removed. `retire` is the last one a user is
// likely to reach for by muscle memory, and unlike `do`/`migrate` it never had a
// `--help` sweep of its own.
#[test]
fn retire_is_not_exposed_by_the_bare_human_cli() {
    let out = Command::new(env!("CARGO_BIN_EXE_grove"))
        .args(["retire", "--help"])
        .output()
        .unwrap();
    assert!(
        !out.status.success(),
        "the config-driven cutover must reject the retired human verb: {out:?}"
    );
}
