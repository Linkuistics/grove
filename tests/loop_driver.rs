// Integration test for the self-driving loop (src/loop_driver.rs).
//
// Drives the *real* loop against a **fake `claude`** (a shell script wired in
// via the `GROVE_HARNESS_BIN` seam) so the mechanism is proven end-to-end with
// no real TUI: the PID handle reaches the child, relaunch is gated on the
// completion signal, and the start→continue prompt switch happens once `.grove/`
// exists.

use grove::harness;
use grove::loop_driver::{self, LoopOutcome};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::sync::Mutex;
use tempfile::TempDir;

// The loop launch reads several process-global env vars (the harness-bin
// override + the fake's bookkeeping handles); serialize so cargo's parallel
// runner doesn't cross test wires.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn write_exec(path: &std::path::Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut perms = fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).unwrap();
}

#[test]
fn loop_relaunches_on_signal_and_stops_without_one() {
    let _g = ENV_LOCK.lock().unwrap();
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    // Prompts the loop's load_prompt must read — from the GLOBAL skill dir the
    // binary provisions (`$GROVE_SKILL_DIR`), NOT any repo-local mirror (the 9.3
    // repoint). Plant a stale mirror in the old `install_path` location to prove
    // the loop ignores it: a regression to the old read would log "STALE …".
    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let stale = repo_path.join(".claude/skills/grove/prompts");
    fs::create_dir_all(&stale).unwrap();
    fs::write(stale.join("start.md"), "STALE START").unwrap();
    fs::write(stale.join("continue.md"), "STALE CONTINUE").unwrap();

    let worktree = repo_path.join(".grove-worktrees/loopgrove");
    fs::create_dir_all(&worktree).unwrap();

    let counter = repo_path.join("counter");
    let log = repo_path.join("log");

    // Fake claude: log <iter>\t<own-pid>\t<inherited-handle>\t<prompt>; create
    // `.grove/` after the first iteration so the loop switches start→continue;
    // fire the completion signal for the first two iterations, then stop.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
for a in "$@"; do prompt="$a"; done
printf '%s\t%s\t%s\t%s\n' "$n" "$$" "$GROVE_CLAUDE_PID" "$prompt" >> "$GROVE_TEST_LOG"
mkdir -p "$PWD/.grove"
if [ "$n" -lt 3 ]; then
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    std::env::set_var("GROVE_HARNESS_BIN", &fake);
    std::env::set_var("GROVE_SKILL_DIR", &skill_dir);
    std::env::set_var("GROVE_TEST_COUNTER", &counter);
    std::env::set_var("GROVE_TEST_LOG", &log);

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "loopgrove");

    std::env::remove_var("GROVE_HARNESS_BIN");
    std::env::remove_var("GROVE_SKILL_DIR");
    std::env::remove_var("GROVE_TEST_COUNTER");
    std::env::remove_var("GROVE_TEST_LOG");

    assert_eq!(
        result.unwrap(),
        LoopOutcome::Stopped,
        "a non-signalled exit stops the loop (resume with `grove do`), not a clean finish"
    );

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<Vec<&str>> = log
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split('\t').collect())
        .collect();

    assert_eq!(
        rows.len(),
        3,
        "loop should run 3 times then stop (log: {log:?})"
    );

    // The exec-PID trick: the handle the child sees equals the child's own pid.
    for row in &rows {
        assert_eq!(
            row[1], row[2],
            "GROVE_CLAUDE_PID must equal the session's own pid (row: {row:?})"
        );
    }

    // start→continue switch: first iteration has no `.grove/` (start), the rest
    // do (continue).
    assert_eq!(
        rows[0][3], "START PROMPT",
        "first iteration bootstraps via start"
    );
    assert_eq!(rows[1][3], "CONTINUE PROMPT", "second iteration continues");
    assert_eq!(rows[2][3], "CONTINUE PROMPT", "third iteration continues");
}

#[test]
fn loop_finishes_clean_on_a_done_signal() {
    let _g = ENV_LOCK.lock().unwrap();
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let worktree = repo_path.join(".grove-worktrees/loopgrove");
    fs::create_dir_all(&worktree).unwrap();

    let counter = repo_path.join("counter");
    let log = repo_path.join("log");

    // Fake claude: fire the *done* signal (the finish cycle's last teardown
    // action) on the first iteration. The loop must run exactly once and stop
    // with a clean finish — not relaunch, and not the no-signal stop.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
printf '%s\n' "$n" >> "$GROVE_TEST_LOG"
printf 'done\n' > "$GROVE_SIGNAL_FILE"
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    std::env::set_var("GROVE_HARNESS_BIN", &fake);
    std::env::set_var("GROVE_SKILL_DIR", &skill_dir);
    std::env::set_var("GROVE_TEST_COUNTER", &counter);
    std::env::set_var("GROVE_TEST_LOG", &log);

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "loopgrove");

    std::env::remove_var("GROVE_HARNESS_BIN");
    std::env::remove_var("GROVE_SKILL_DIR");
    std::env::remove_var("GROVE_TEST_COUNTER");
    std::env::remove_var("GROVE_TEST_LOG");

    assert_eq!(
        result.unwrap(),
        LoopOutcome::Finished,
        "a `done` signal must end the loop with a clean finish"
    );

    let log = fs::read_to_string(&log).unwrap();
    let count = log.lines().filter(|l| !l.is_empty()).count();
    assert_eq!(
        count, 1,
        "the loop must run exactly once then finish — no relaunch (log: {log:?})"
    );
}

// Model selection (model-per-task-kind): the driver launches each session on a
// model chosen by the picked leaf's **kind**. The start path is planning by
// construction (fresh-grove-start-contract); the continue path peeks the next
// live leaf's kind via the real `grove-llm kind` binary (wired in via the
// `GROVE_LLM_BIN` seam, run against a real git worktree so `kind` resolves the
// grove root). Asserts the exact `--model` per iteration.
#[test]
fn loop_selects_model_by_kind() {
    let _g = ENV_LOCK.lock().unwrap();
    // A real git repo *is* the worktree, so the real `grove-llm kind` (which
    // finds the grove root via `git rev-parse --show-toplevel`) resolves `.grove/`.
    let worktree_dir = TempDir::new().unwrap();
    let worktree = worktree_dir.path();
    assert!(
        std::process::Command::new("git")
            .arg("init")
            .arg("-q")
            .current_dir(worktree)
            .status()
            .unwrap()
            .success(),
        "git init failed"
    );

    let skill_dir = worktree.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let counter = worktree.join("counter");
    let log = worktree.join("log");

    // Fake claude: log the full argv per iteration. On the first (start) run,
    // materialise a `.grove/` with one live **work** leaf so the second run
    // takes the continue path and `grove-llm kind` resolves it to `work`. Fire
    // the completion signal only on the first run, so the loop stops after two.
    let fake = worktree.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
printf '%s\n' "$*" >> "$GROVE_TEST_LOG"
if [ "$n" -eq 1 ]; then
  mkdir -p "$PWD/.grove"
  printf '# g — brief\n' > "$PWD/.grove/BRIEF.md"
  printf '# a-k1\n\n**Kind:** work\n' > "$PWD/.grove/01-a-k1.md"
fi
if [ "$n" -lt 2 ]; then
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    std::env::set_var("GROVE_HARNESS_BIN", &fake);
    std::env::set_var("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"));
    std::env::set_var("GROVE_SKILL_DIR", &skill_dir);
    std::env::set_var("GROVE_TEST_COUNTER", &counter);
    std::env::set_var("GROVE_TEST_LOG", &log);
    std::env::set_var("GROVE_PLANNING_MODEL", "opus");
    std::env::set_var("GROVE_WORK_MODEL", "sonnet");

    let result = loop_driver::run_loop(harness, worktree, worktree, "modelgrove");

    std::env::remove_var("GROVE_HARNESS_BIN");
    std::env::remove_var("GROVE_LLM_BIN");
    std::env::remove_var("GROVE_SKILL_DIR");
    std::env::remove_var("GROVE_TEST_COUNTER");
    std::env::remove_var("GROVE_TEST_LOG");
    std::env::remove_var("GROVE_PLANNING_MODEL");
    std::env::remove_var("GROVE_WORK_MODEL");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        rows.len(),
        2,
        "loop should run twice then stop (log: {log:?})"
    );

    // Iteration 1 — start path ⇒ planning ⇒ GROVE_PLANNING_MODEL.
    assert!(
        rows[0].contains("--model opus"),
        "start (planning) session must launch on the planning model (argv: {:?})",
        rows[0]
    );
    assert!(
        !rows[0].contains("sonnet"),
        "start session must not use the work model (argv: {:?})",
        rows[0]
    );
    // Iteration 2 — continue path ⇒ work leaf ⇒ GROVE_WORK_MODEL.
    assert!(
        rows[1].contains("--model sonnet"),
        "continue (work) session must launch on the work model (argv: {:?})",
        rows[1]
    );
}

// The load-bearing rule: with neither model env var set, the driver passes no
// `--model` at all — byte-for-byte the pre-feature launch, so a user's own
// `ANTHROPIC_MODEL`/settings default is never clobbered.
#[test]
fn loop_omits_model_flag_when_env_unset() {
    let _g = ENV_LOCK.lock().unwrap();
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let worktree = repo_path.join(".grove-worktrees/loopgrove");
    fs::create_dir_all(&worktree).unwrap();

    let counter = repo_path.join("counter");
    let log = repo_path.join("log");

    // Fake claude: log full argv; create `.grove/` on the first run so the
    // second run takes the continue path too (both paths must stay `--model`-free
    // when the env is unset); stop after two iterations.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
printf '%s\n' "$*" >> "$GROVE_TEST_LOG"
mkdir -p "$PWD/.grove"
if [ "$n" -lt 2 ]; then
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    // Guard against leakage from another test in the same process.
    std::env::remove_var("GROVE_PLANNING_MODEL");
    std::env::remove_var("GROVE_WORK_MODEL");
    std::env::set_var("GROVE_HARNESS_BIN", &fake);
    std::env::set_var("GROVE_SKILL_DIR", &skill_dir);
    std::env::set_var("GROVE_TEST_COUNTER", &counter);
    std::env::set_var("GROVE_TEST_LOG", &log);

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "loopgrove");

    std::env::remove_var("GROVE_HARNESS_BIN");
    std::env::remove_var("GROVE_SKILL_DIR");
    std::env::remove_var("GROVE_TEST_COUNTER");
    std::env::remove_var("GROVE_TEST_LOG");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        rows.len(),
        2,
        "loop should run twice then stop (log: {log:?})"
    );
    for row in &rows {
        assert!(
            !row.contains("--model"),
            "no env set ⇒ no --model flag (argv: {row:?})"
        );
    }
}
