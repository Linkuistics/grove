// Integration test for the self-driving loop (src/loop_driver.rs).
//
// Drives the *real* loop against a **fake `claude`** (a shell script wired in
// via the `GROVE_HARNESS_BIN` seam) so the mechanism is proven end-to-end with
// no real TUI: the PID handle reaches the child, relaunch is gated on the
// completion signal, and the start→continue prompt switch happens once `.grove/`
// exists.

mod support;

use grove::harness;
use grove::loop_driver::{self, LoopOutcome};
use grove::provision::STAMP_FILE;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::sync::Mutex;
use support::EnvGuard;
use tempfile::TempDir;

// The loop launch reads several process-global env vars (the harness-bin
// override + the fake's bookkeeping handles); serialize so cargo's parallel
// runner doesn't cross test wires. A prior test's panic mid-mutation poisons
// this lock; `support::lock_env` tolerates that (`EnvGuard`'s `Drop` already
// restored the env before the panic unwound past it — see B1/T7).
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn write_exec(path: &std::path::Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut perms = fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).unwrap();
}

#[test]
fn loop_relaunches_on_signal_and_stops_without_one() {
    let _g = support::lock_env(&ENV_LOCK);
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

    // Fake claude: log <iter>\t<own-pid>\t<inherited-handle>\t<co-exported-legacy-handle>\t<prompt>;
    // create `.grove/` after the first iteration so the loop switches
    // start→continue; fire the completion signal for the first two
    // iterations, then stop. Logs both `GROVE_HARNESS_PID` and the legacy
    // `GROVE_CLAUDE_PID` co-export (T6): a content/agent that still reads the
    // old name for one release must see the same handle.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
for a in "$@"; do prompt="$a"; done
printf '%s\t%s\t%s\t%s\t%s\n' "$n" "$$" "$GROVE_HARNESS_PID" "$GROVE_CLAUDE_PID" "$prompt" >> "$GROVE_TEST_LOG"
mkdir -p "$PWD/.grove"
if [ "$n" -lt 3 ]; then
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log);

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "loopgrove");

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

    // The exec-PID trick: the handle the child sees equals the child's own
    // pid, on both the new name and the co-exported legacy name.
    for row in &rows {
        assert_eq!(
            row[1], row[2],
            "GROVE_HARNESS_PID must equal the session's own pid (row: {row:?})"
        );
        assert_eq!(
            row[1], row[3],
            "GROVE_CLAUDE_PID must be co-exported with the same handle, for one \
             release of backward compatibility (row: {row:?})"
        );
    }

    // start→continue switch: first iteration has no `.grove/` (start), the rest
    // do (continue).
    assert_eq!(
        rows[0][4], "START PROMPT",
        "first iteration bootstraps via start"
    );
    assert_eq!(rows[1][4], "CONTINUE PROMPT", "second iteration continues");
    assert_eq!(rows[2][4], "CONTINUE PROMPT", "third iteration continues");
}

#[test]
fn loop_finishes_clean_on_a_done_signal() {
    let _g = support::lock_env(&ENV_LOCK);
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

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log);

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "loopgrove");

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
// grove root). Asserts the exact `--model` per iteration, across three of the
// five kinds — planning (start), then two non-binary continue kinds (work,
// review) — proving the scheme is a real five-way lookup, not just a binary.
#[test]
fn loop_selects_model_by_kind() {
    let _g = support::lock_env(&ENV_LOCK);
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
    // materialise a `.grove/` with one live leaf kinded **work**, so the second
    // run takes the continue path and `grove-llm kind` resolves it to `work`;
    // that run then rewrites the same leaf's kind to **review** in place, so the
    // third run's peek resolves to `review`. Fire the completion signal on the
    // first two runs only, so the loop stops after three.
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
if [ "$n" -eq 2 ]; then
  printf '# a-k1\n\n**Kind:** review\n' > "$PWD/.grove/01-a-k1.md"
fi
if [ "$n" -lt 3 ]; then
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    // clear_grove_env is load-bearing here, not just hygiene: this repo
    // dogfoods per-kind model + harness routing envs (BRIEF.md Notes), and
    // this very test suite may be running *inside* a rerouted review session
    // — ambient `GROVE_REVIEW_HARNESS`/`GROVE_PI_REVIEW_MODEL` would silently
    // reroute iteration 3 (B1).
    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"))
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_PLANNING_MODEL", "opus")
        .set("GROVE_WORK_MODEL", "sonnet")
        .set("GROVE_REVIEW_MODEL", "haiku");

    let result = loop_driver::run_loop(harness, worktree, worktree, "modelgrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        rows.len(),
        3,
        "loop should run three times then stop (log: {log:?})"
    );

    // Iteration 1 — start path ⇒ planning ⇒ GROVE_PLANNING_MODEL.
    assert!(
        rows[0].contains("--model opus"),
        "start (planning) session must launch on the planning model (argv: {:?})",
        rows[0]
    );
    assert!(
        !rows[0].contains("sonnet") && !rows[0].contains("haiku"),
        "start session must use only the planning model (argv: {:?})",
        rows[0]
    );
    // Iteration 2 — continue path ⇒ work leaf ⇒ GROVE_WORK_MODEL.
    assert!(
        rows[1].contains("--model sonnet"),
        "continue (work) session must launch on the work model (argv: {:?})",
        rows[1]
    );
    // Iteration 3 — continue path ⇒ review leaf ⇒ GROVE_REVIEW_MODEL, proving a
    // non-binary kind resolves correctly, not just planning/work.
    assert!(
        rows[2].contains("--model haiku"),
        "continue (review) session must launch on the review model (argv: {:?})",
        rows[2]
    );
}

// Degrade-on-read must be **loud** (task-kind-taxonomy). An unrecognised
// `**Kind:**` line — a typo, a hand-edited file, or a tree written by a newer
// grove — is treated as `work`, which in a typical config is the *cheapest*
// model: a silent downgrade. `grove-llm kind` warns on stderr but exits 0, so
// the warning rides the **success** path; a driver that captures the child's
// stderr swallows exactly the diagnostic that explains the downgrade.
//
// Runs the real `grove` binary as a subprocess (the only way to observe what the
// operator actually sees on stderr) and asserts both halves: the warning reaches
// them, *and* the leaf still launches — degrading, never jamming the loop.
#[test]
fn unrecognised_kind_warns_the_operator_and_still_launches() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let git = |args: &[&str]| {
        assert!(
            std::process::Command::new("git")
                .args(args)
                .current_dir(repo_path)
                .status()
                .unwrap()
                .success(),
            "git {args:?} failed"
        );
    };
    git(&["init", "-q", "-b", "main"]);
    git(&["config", "user.email", "t@example.com"]);
    git(&["config", "user.name", "t"]);

    // `.claude/` so the harness is detected; a committed `.grove/` whose live
    // leaf carries a kind this binary does not know, so the continue path's peek
    // hits the degrade branch.
    fs::create_dir_all(repo_path.join(".claude")).unwrap();
    fs::create_dir_all(repo_path.join(".grove")).unwrap();
    fs::write(repo_path.join(".grove/BRIEF.md"), "# g — brief\n").unwrap();
    fs::write(
        repo_path.join(".grove/01-a-k1.md"),
        "# a-k1\n\n**Kind:** reserch\n",
    )
    .unwrap();
    git(&["add", "-A"]);
    git(&["commit", "-qm", "tree with an unrecognised kind"]);

    // This test drives the real `grove do` binary, so provisioning is live
    // (unlike the in-process `run_loop` tests above, which never reach
    // `provision_all`). Stamp the dir so the foreign-dir guard
    // (`provision_target`) treats it as grove's own — a mismatched hash still
    // re-extracts the real embedded prompts, which is fine: this test only
    // asserts on stderr and the logged argv, never on prompt content.
    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();
    fs::write(skill_dir.join(STAMP_FILE), "stale-hash").unwrap();

    let log = repo_path.join("log");
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
printf '%s\n' "$*" >> "$GROVE_TEST_LOG"
exit 0
"#,
    );

    // A subprocess inherits this test process's *whole* ambient environment
    // unless each var is explicitly overridden or removed — `.env(...)` alone
    // does not isolate it. Scrub the full routing/model surface before layering
    // the scenario's own vars, or this repo's own dogfooded `~/.zshenv` (or a
    // session running these tests under itself) can steer the subprocess.
    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_grove"));
    cmd.args(["do"]).current_dir(repo_path);
    for name in support::grove_env_names() {
        cmd.env_remove(name);
    }
    let out = cmd
        .env("GROVE_HARNESS_BIN", &fake)
        .env("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"))
        .env("GROVE_SKILL_DIR", &skill_dir)
        .env("GROVE_TEST_LOG", &log)
        .env("GROVE_WORK_MODEL", "sonnet")
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        stderr.contains("unrecognised") && stderr.contains("reserch"),
        "the operator must SEE why the leaf was downgraded — `grove-llm kind`'s \
         degrade warning rides a zero exit, so capturing the child's stderr \
         swallows it (stderr: {stderr:?})"
    );

    let argv = fs::read_to_string(&log).unwrap_or_default();
    assert!(
        argv.contains("--model sonnet"),
        "an unrecognised kind degrades to `work` and still launches — a typo must \
         never jam the unattended loop (argv: {argv:?})"
    );
}

// The load-bearing rule: with neither model env var set, the driver passes no
// `--model` at all — byte-for-byte the pre-feature launch, so a user's own
// `ANTHROPIC_MODEL`/settings default is never clobbered.
#[test]
fn loop_omits_model_flag_when_env_unset() {
    let _g = support::lock_env(&ENV_LOCK);
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

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log);

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "loopgrove");

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

// T6: an *empty-string* model var must behave exactly like an unset one — a
// blank `GROVE_WORK_MODEL=` (e.g. from a shell template that didn't fill in a
// value) must never reach the harness as a literal empty `--model`.
#[test]
fn loop_omits_model_flag_when_env_is_empty_string() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let worktree = repo_path.join(".grove-worktrees/loopgrove");
    fs::create_dir_all(&worktree).unwrap();

    let log = repo_path.join("log");
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
printf '%s\n' "$*" >> "$GROVE_TEST_LOG"
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_PLANNING_MODEL", "");

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "loopgrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let argv = fs::read_to_string(&log).unwrap();
    assert!(
        !argv.contains("--model"),
        "an empty-string model var must be treated as unset, never passed \
         through as a literal empty --model (argv: {argv:?})"
    );
}

// The codex harness declaration (issue #1). Two independent defects, both
// observable only in the launched argv, which is what this drives:
//
//   * `name_args: &["--name"]` — codex has no such flag (checked against
//     codex-cli 0.144.1: zero `--name` matches in `--help`). Session names exist
//     in codex but are assigned *after* start, via `/rename`. A launch would die
//     in codex's argument parser before any session began.
//   * `model_args: &[]` — codex opted out of model-per-task-kind, but it does
//     accept `-m, --model <MODEL>`, so the opt-out cost it the feature for no
//     reason. It now participates via `--profile`, since profiles are the
//     only way to bind reasoning effort to the launch.
//
// Latent until now only because `select` runs in `SelectMode::Single` and no
// grove drives codex; it fires the first time anyone runs `grove do` in a repo
// with a `.codex/` directory.
#[test]
fn codex_launches_with_no_name_flag_and_a_model_flag() {
    let _g = support::lock_env(&ENV_LOCK);
    // A real git repo *is* the worktree, so the real `grove-llm kind` resolves the
    // leaf the second (continue) iteration peeks at.
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

    // Fake codex: log the full argv. The first (start) run materialises a `.grove/`
    // holding one live **work** leaf, so the second run takes the continue path and
    // `grove-llm kind` resolves it to `work`. Signal only on the first, so the loop
    // stops after two.
    let fake = worktree.join("fake-codex.sh");
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
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("codex").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"))
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_WORK_MODEL", "sol-high");

    let result = loop_driver::run_loop(harness, worktree, worktree, "codexgrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        rows.len(),
        2,
        "loop should run twice then stop (log: {log:?})"
    );

    // No launch-time session-name flag: codex would abort on an unknown `--name`.
    for row in &rows {
        assert!(
            !row.contains("--name"),
            "codex has no launch-time session-name flag (argv: {row:?})"
        );
    }
    // ...and the profile *is* selected: codex model-per-task-kind values name
    // profiles (`--profile`), which bind model + reasoning effort.
    assert!(
        rows[1].contains("--profile sol-high"),
        "codex must honour model-per-task-kind via --profile (argv: {:?})",
        rows[1]
    );
}

// Per-harness model envs: GROVE_<HARNESS>_<KIND>_MODEL beats GROVE_<KIND>_MODEL.
// One shared kind env can't serve two harnesses at once (a codex profile name is
// garbage to pi and vice versa), so each harness gets a scoped override.
#[test]
fn per_harness_model_env_beats_the_base_var() {
    let _g = support::lock_env(&ENV_LOCK);
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

    // Fake harness: run 1 (start/planning) materialises a work leaf + signal;
    // run 2 (continue/work) stops.
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
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    // clear_grove_env first (this repo dogfoods per-kind model envs — BRIEF.md
    // Notes — so a session driving this very test suite may already have
    // GROVE_PLANNING_MODEL etc. set), then layer the scenario's own vars:
    // a scoped override for the launching harness + a base var it must beat,
    // and an override for a *different* harness that must be ignored.
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"))
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_CLAUDE_WORK_MODEL", "kimi-k3")
        .set("GROVE_WORK_MODEL", "sonnet")
        .set("GROVE_PI_PLANNING_MODEL", "must-not-leak");

    let result = loop_driver::run_loop(harness, worktree, worktree, "envgrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(rows.len(), 2, "loop should run twice (log: {log:?})");

    // Start/planning: no CLAUDE planning override and no base planning var ⇒
    // no --model. This is a structural guarantee, not a live-risk assertion —
    // `model_for` only ever interpolates *this* harness's own name into the
    // env-var it reads (src/loop_driver.rs `model_for`), so a pi-scoped var
    // has no code path that could consult it; kept as documentation of that
    // intent, alongside the genuinely discriminating precedence check below.
    assert!(
        !rows[0].contains("--model"),
        "another harness's scoped var must not select a model (argv: {:?})",
        rows[0]
    );
    // Continue/work: the claude-scoped var beats the base var — the
    // discriminating assertion (precedence, not cross-harness isolation).
    assert!(
        rows[1].contains("--model kimi-k3") && !rows[1].contains("sonnet"),
        "GROVE_CLAUDE_WORK_MODEL must beat GROVE_WORK_MODEL (argv: {:?})",
        rows[1]
    );
}

// Per-kind harness routing: GROVE_REVIEW_HARNESS=pi must launch review leaves
// on pi even in a codex-stamped grove — the trial's "K3 reviews everywhere"
// invariant. Proven with two distinct fake binaries wired through the
// per-harness bin seam, so the argv log shows *which* harness ran each leaf.
//
// T4/B7: uses **real per-harness skill dirs under a scratch `$HOME`**, not a
// shared `GROVE_SKILL_DIR` override — that override "collapses the sweep to
// that single dir" (`provision::provision_all`'s doc) regardless of harness,
// which makes a shared-dir fixture structurally blind to B7 (`load_prompt`
// reading the *stamped* harness's prompt copy, not the post-reroute launch
// harness's): both harnesses would read the identical file either way. With
// distinct codex/pi prompt copies, this test also proves B7 directly — the
// rerouted review session must read *pi's* continue prompt, not codex's.
#[test]
fn review_leaf_reroutes_to_the_review_harness() {
    let _g = support::lock_env(&ENV_LOCK);
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

    // Real per-harness skill dirs under a scratch $HOME (T4) — see the
    // function doc above for why a shared GROVE_SKILL_DIR can't prove B7.
    let home = worktree.join("scratch-home");
    let codex_prompts = home.join(".codex/skills/grove/prompts");
    let pi_prompts = home.join(".pi/agent/skills/grove/prompts");
    fs::create_dir_all(&codex_prompts).unwrap();
    fs::create_dir_all(&pi_prompts).unwrap();
    fs::write(codex_prompts.join("start.md"), "CODEX START PROMPT").unwrap();
    fs::write(codex_prompts.join("continue.md"), "CODEX CONTINUE PROMPT").unwrap();
    fs::write(pi_prompts.join("continue.md"), "PI CONTINUE PROMPT").unwrap();

    let counter = worktree.join("counter");
    let log = worktree.join("log");

    // Fake codex: tags rows "codex"; run 1 (start/planning) materialises a
    // *review* leaf + signal, so run 2 is a review continue. Logs the prompt
    // it received (the last positional arg) so the test can tell which
    // harness's copy `load_prompt` actually read.
    let fake_codex = worktree.join("fake-codex.sh");
    write_exec(
        &fake_codex,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
for a in "$@"; do prompt="$a"; done
printf 'codex\t%s\t%s\n' "$*" "$prompt" >> "$GROVE_TEST_LOG"
if [ "$n" -eq 1 ]; then
  mkdir -p "$PWD/.grove"
  printf '# g — brief\n' > "$PWD/.grove/BRIEF.md"
  printf '# a-k1\n\n**Kind:** review\n' > "$PWD/.grove/01-a-k1.md"
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );
    // Fake pi: tags rows "pi"; never signals, so the loop stops after it.
    let fake_pi = worktree.join("fake-pi.sh");
    write_exec(
        &fake_pi,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
for a in "$@"; do prompt="$a"; done
printf 'pi\t%s\t%s\n' "$*" "$prompt" >> "$GROVE_TEST_LOG"
exit 0
"#,
    );

    let harness = harness::by_name("codex").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("HOME", &home)
        .set("GROVE_HARNESS_BIN_CODEX", &fake_codex)
        .set("GROVE_HARNESS_BIN_PI", &fake_pi)
        .set("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"))
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_REVIEW_HARNESS", "pi")
        .set("GROVE_CODEX_PLANNING_MODEL", "sol-xhigh")
        .set("GROVE_PI_REVIEW_MODEL", "kimi-code/k3");

    let result = loop_driver::run_loop(harness, worktree, worktree, "reroutegrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<Vec<&str>> = log
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.splitn(3, '\t').collect())
        .collect();
    assert_eq!(rows.len(), 2, "loop should run twice (log: {log:?})");

    // Planning leaf: the stamped harness (codex) with its scoped profile,
    // reading codex's own start prompt.
    assert_eq!(rows[0][0], "codex", "planning stays on the stamped harness");
    assert!(
        rows[0][1].contains("--profile sol-xhigh"),
        "codex planning launches on its scoped profile (argv: {:?})",
        rows[0][1]
    );
    assert_eq!(
        rows[0][2], "CODEX START PROMPT",
        "the planning session must read codex's own start prompt"
    );

    // Review leaf: rerouted to pi, with pi's scoped model — the launch flag
    // template must be the *post-override* harness's (--model, not --profile)
    // — and pi's own continue prompt, not codex's (B7).
    assert_eq!(
        rows[1][0], "pi",
        "review must reroute to GROVE_REVIEW_HARNESS"
    );
    assert!(
        rows[1][1].contains("--model kimi-code/k3"),
        "the rerouted review leaf resolves models against pi (argv: {:?})",
        rows[1][1]
    );
    assert_eq!(
        rows[1][2], "PI CONTINUE PROMPT",
        "the rerouted review session must read pi's own continue prompt, not \
         codex's (B7: load_prompt must read the launching harness's copy)"
    );
}

// B2: the harness-agnostic base var must not survive a reroute — a codex
// profile name (or any value meant for the *stamped* harness) is garbage on
// the harness a per-kind override reroutes to. Only the harness-scoped var
// may supply a model once a reroute has happened; with it unset, the
// rerouted leaf must launch with no `--model` at all, never the base var's
// value.
#[test]
fn base_model_var_does_not_survive_a_reroute() {
    let _g = support::lock_env(&ENV_LOCK);
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

    let fake_codex = worktree.join("fake-codex.sh");
    write_exec(
        &fake_codex,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
printf 'codex\t%s\n' "$*" >> "$GROVE_TEST_LOG"
if [ "$n" -eq 1 ]; then
  mkdir -p "$PWD/.grove"
  printf '# g — brief\n' > "$PWD/.grove/BRIEF.md"
  printf '# a-k1\n\n**Kind:** review\n' > "$PWD/.grove/01-a-k1.md"
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );
    let fake_pi = worktree.join("fake-pi.sh");
    write_exec(
        &fake_pi,
        r#"#!/bin/sh
printf 'pi\t%s\n' "$*" >> "$GROVE_TEST_LOG"
exit 0
"#,
    );

    let harness = harness::by_name("codex").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN_CODEX", &fake_codex)
        .set("GROVE_HARNESS_BIN_PI", &fake_pi)
        .set("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"))
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_REVIEW_HARNESS", "pi")
        // The base var — a codex profile name, meaningless to pi — with no
        // GROVE_PI_REVIEW_MODEL set to beat it.
        .set("GROVE_REVIEW_MODEL", "sol-high");

    let result = loop_driver::run_loop(harness, worktree, worktree, "basemodelgrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<Vec<&str>> = log
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.splitn(2, '\t').collect())
        .collect();
    assert_eq!(rows.len(), 2, "loop should run twice (log: {log:?})");

    assert_eq!(rows[1][0], "pi", "review must reroute to pi");
    assert!(
        !rows[1][1].contains("--model"),
        "the base GROVE_REVIEW_MODEL (a codex profile name) must not reach \
         pi across a reroute — no scoped GROVE_PI_REVIEW_MODEL means no \
         --model at all (argv: {:?})",
        rows[1][1]
    );
}

// B5: the legacy unscoped `GROVE_HARNESS_BIN` must not leak into a
// per-kind-rerouted launch — once one loop can launch two harnesses, a
// single global bin override is incoherent (it would exec the *stamped*
// harness's wrapper under the *rerouted* harness's flag template). Proven by
// putting a distinctly-named `pi` executable on PATH (the `exec_bin`
// fallback) and asserting the reroute reaches *that*, not the unscoped
// wrapper meant for the stamped harness.
#[test]
fn unscoped_harness_bin_does_not_leak_across_a_reroute() {
    let _g = support::lock_env(&ENV_LOCK);
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

    // The unscoped legacy wrapper: correct for the *stamped* harness (codex,
    // no GROVE_HARNESS_BIN_CODEX set), wrong for anything rerouted to.
    let wrapper = worktree.join("wrapper.sh");
    write_exec(
        &wrapper,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
printf 'wrapper\t%s\n' "$*" >> "$GROVE_TEST_LOG"
if [ "$n" -eq 1 ]; then
  mkdir -p "$PWD/.grove"
  printf '# g — brief\n' > "$PWD/.grove/BRIEF.md"
  printf '# a-k1\n\n**Kind:** review\n' > "$PWD/.grove/01-a-k1.md"
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    // The real fallback for pi: a dedicated executable literally named `pi`
    // (harness::exec_bin), reached only via PATH — proving `harness_bin` fell
    // through to `exec_bin`, not the unscoped wrapper.
    let bindir = worktree.join("bin");
    fs::create_dir_all(&bindir).unwrap();
    write_exec(
        &bindir.join("pi"),
        r#"#!/bin/sh
printf 'realpi\t%s\n' "$*" >> "$GROVE_TEST_LOG"
exit 0
"#,
    );
    let path = format!(
        "{}:{}",
        bindir.display(),
        std::env::var("PATH").unwrap_or_default()
    );

    let harness = harness::by_name("codex").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("PATH", &path)
        .set("GROVE_HARNESS_BIN", &wrapper)
        .set("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"))
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_REVIEW_HARNESS", "pi");

    let result = loop_driver::run_loop(harness, worktree, worktree, "binleakgrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<Vec<&str>> = log
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.splitn(2, '\t').collect())
        .collect();
    assert_eq!(rows.len(), 2, "loop should run twice (log: {log:?})");

    assert_eq!(
        rows[0][0], "wrapper",
        "the stamped harness (codex, no scoped bin) still uses the legacy \
         unscoped GROVE_HARNESS_BIN (argv: {:?})",
        rows[0]
    );
    assert_eq!(
        rows[1][0], "realpi",
        "the rerouted review leaf must not run the stamped harness's \
         unscoped wrapper — it must fall through to pi's own exec_bin \
         (argv: {:?})",
        rows[1]
    );
}

// B5: an empty-string `GROVE_HARNESS_BIN` must behave like unset — parallel
// to the model-var and kind-harness-override empty-string guards (env_model,
// harness_override) — not like a literal empty-string binary path, which
// would fail every launch (`harness_bin` was the only env seam in the file
// treating `""` as set).
#[test]
fn empty_string_harness_bin_is_treated_as_unset() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let worktree = repo_path.join("wt");
    fs::create_dir_all(&worktree).unwrap();

    // A fake `claude` (harness::exec_bin) on PATH, so an empty
    // GROVE_HARNESS_BIN falling through to exec_bin is observable without
    // depending on the real `claude` CLI being installed on this machine.
    let bindir = repo_path.join("bin");
    fs::create_dir_all(&bindir).unwrap();
    let log = repo_path.join("log");
    write_exec(
        &bindir.join("claude"),
        &format!(
            r#"#!/bin/sh
printf 'ran\n' >> "{log}"
exit 0
"#,
            log = log.display()
        ),
    );
    let path = format!(
        "{}:{}",
        bindir.display(),
        std::env::var("PATH").unwrap_or_default()
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("PATH", &path)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_HARNESS_BIN", "");

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "emptybingrove");

    assert_eq!(
        result.unwrap(),
        LoopOutcome::Stopped,
        "an empty GROVE_HARNESS_BIN must fall through to exec_bin, not be \
         attempted as a literal empty-string binary path"
    );
    assert_eq!(
        fs::read_to_string(&log).unwrap(),
        "ran\n",
        "the fallback exec_bin (`claude`) must actually have run"
    );
}

// B6: a degraded kind peek (grove-llm missing/failing/unparseable) must not
// silently cancel an active per-kind harness override by launching on the
// stamped harness — that is exactly the "K3 reviews everywhere" invariant a
// silent fallback here would defeat. `harness_override`'s own doc already
// makes this argument for an unknown *value*; this proves it also holds for
// a degraded *peek*.
#[test]
fn degraded_kind_peek_refuses_to_silently_cancel_a_harness_override() {
    let _g = support::lock_env(&ENV_LOCK);
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
    fs::create_dir_all(worktree.join(".grove")).unwrap();
    fs::write(worktree.join(".grove/BRIEF.md"), "# g — brief\n").unwrap();
    fs::write(
        worktree.join(".grove/01-a-k1.md"),
        "# a-k1\n\n**Kind:** review\n",
    )
    .unwrap();

    let harness = harness::by_name("codex").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        // A nonexistent grove-llm binary: the spawn itself fails ⇒ a
        // degraded peek (Err(e) arm), not a parse error.
        .set("GROVE_LLM_BIN", worktree.join("no-such-grove-llm"))
        .set("GROVE_REVIEW_HARNESS", "pi");

    let result = loop_driver::run_loop(harness, worktree, worktree, "degradedgrove");

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("harness override") && err.contains("stamped harness"),
        "a degraded kind peek with an active harness override must fail \
         loudly rather than silently launching on the stamped harness \
         (err: {err})"
    );
}

// Notes: `any_harness_override_env` already sweeps all five suffixes to
// decide whether routing applies at all; validating every
// `GROVE_<KIND>_HARNESS` value at that same point (not just the picked
// leaf's kind) means a typo in an *off-kind* var is caught immediately, not
// only once that kind's leaf is finally picked.
#[test]
fn an_off_kind_harness_override_typo_is_caught_immediately() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let worktree = repo_path.join("wt");
    fs::create_dir_all(&worktree).unwrap();

    let harness = harness::by_name("claude").unwrap();

    // The start path resolves straight to Planning, never touching
    // GROVE_REVIEW_HARNESS — yet the typo there must still fail loudly right
    // away, not once a review leaf happens to be picked.
    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_REVIEW_HARNESS", "lemur");

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "offkindgrove");

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("GROVE_REVIEW_HARNESS") && err.contains("lemur"),
        "a typo in an off-kind override must fail at the very next launch, \
         not only once a review leaf is picked (err: {err})"
    );
}

// An unknown override value must fail loudly at launch — a typo'd harness
// name that silently fell back to the stamped harness would run reviews on
// the wrong (and possibly self-reviewing) model for a whole trial. The start
// path takes a shortcut straight to `Kind::Planning` (fresh-grove-start-
// contract) without ever calling `resolve_kind`, so this alone cannot prove
// the *continue* path's peek honours the same contract — see the sibling
// test below for that (T3).
#[test]
fn unknown_review_harness_fails_loudly() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let worktree = repo_path.join("wt");
    fs::create_dir_all(&worktree).unwrap();

    let harness = harness::by_name("claude").unwrap();

    // Start path ⇒ kind is Planning by construction; route planning to a typo.
    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_PLANNING_HARNESS", "lemur");

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "typogrove");

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("GROVE_PLANNING_HARNESS") && err.contains("lemur"),
        "the error must name the variable and the bad value (err: {err})"
    );
    assert!(
        err.contains("claude") && err.contains("codex") && err.contains("pi"),
        "the error must list the known harnesses (err: {err})"
    );
}

// T3: the continue path's kind peek must honour the same
// unknown-override-fails-loudly contract as the start path above — that path
// short-circuits to `Kind::Planning` and never calls `resolve_kind`
// (src/loop_driver.rs:279-281), so it cannot exercise `GROVE_REVIEW_HARNESS`
// at all. This drives a real `.grove/` with a **review** leaf through the
// continue path (real `grove-llm kind`) so `resolve_kind` genuinely runs.
#[test]
fn unknown_review_harness_fails_loudly_on_the_continue_path() {
    let _g = support::lock_env(&ENV_LOCK);
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
    fs::create_dir_all(worktree.join(".grove")).unwrap();
    fs::write(worktree.join(".grove/BRIEF.md"), "# g — brief\n").unwrap();
    fs::write(
        worktree.join(".grove/01-a-k1.md"),
        "# a-k1\n\n**Kind:** review\n",
    )
    .unwrap();

    let skill_dir = worktree.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"))
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_REVIEW_HARNESS", "lemur");

    let result = loop_driver::run_loop(harness, worktree, worktree, "typogrove2");

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("GROVE_REVIEW_HARNESS") && err.contains("lemur"),
        "the error must name the variable and the bad value (err: {err})"
    );
    assert!(
        err.contains("claude") && err.contains("codex") && err.contains("pi"),
        "the error must list the known harnesses (err: {err})"
    );
}

// T6: an empty-string `GROVE_<KIND>_HARNESS` must be treated as unset (like
// the empty-string model var), not as a route to an empty-named harness —
// `harness_override` already guards this (`!name.is_empty()`); this proves it
// end-to-end rather than trusting the guard is reached.
#[test]
fn empty_string_kind_harness_override_is_treated_as_unset() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let worktree = repo_path.join("wt");
    fs::create_dir_all(&worktree).unwrap();

    let log = repo_path.join("log");
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
printf 'claude\t%s\n' "$*" >> "$GROVE_TEST_LOG"
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_PLANNING_HARNESS", "");

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "emptyharnessgrove");

    assert_eq!(
        result.unwrap(),
        LoopOutcome::Stopped,
        "an empty override must not error, and must not hang looking for an \
         empty-named harness"
    );

    let log = fs::read_to_string(&log).unwrap();
    assert!(
        log.starts_with("claude\t"),
        "empty GROVE_PLANNING_HARNESS must stay on the stamped harness (log: {log:?})"
    );
}
