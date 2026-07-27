// Integration test for the self-driving loop (src/loop_driver.rs).
//
// Drives the *real* loop against a **fake `claude`** (a shell script wired in
// via the `GROVE_HARNESS_BIN` seam) so the mechanism is proven end-to-end with
// no real TUI: no PID handle is exported to the child (the driver kills its
// own child instead — driver-side watcher), the driver ends a session that
// signalled and then hung, relaunch is gated on the completion signal, and the
// start→continue prompt switch happens once `.grove/` exists.

mod support;

use grove::harness;
use grove::loop_driver::{self, LoopOutcome};
use grove::provision::STAMP_FILE;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use support::EnvGuard;
use tempfile::TempDir;

// The loop launch reads several process-global env vars (the harness-bin
// override + the fake's bookkeeping handles); serialize so cargo's parallel
// runner doesn't cross test wires. A prior test's panic mid-mutation poisons
// this lock; `support::lock_env` tolerates that (`EnvGuard`'s `Drop` already
// restored the env before the panic unwound past it — see B1/T7).
static ENV_LOCK: Mutex<()> = Mutex::new(());

// This build's own `grove-llm`, pinned (via the `GROVE_LLM_BIN` seam) in every
// test that reaches the loop: the driver's per-session version-skew guard
// (driver-version-skew-k11) compares its compiled-in version against the
// `grove-llm` the agent would invoke, and without the pin that is whatever the
// *machine's* PATH happens to carry — a mismatched installed release would
// stop the loop before the session under test ever launched.
const OWN_GROVE_LLM: &str = env!("CARGO_BIN_EXE_grove-llm");

// Model selection is **required** (model-per-task-kind, *A kind with no model
// is a configuration error*): a picked leaf whose kind resolves no model var
// fails the launch instead of inheriting the harness's own default. So every
// fixture here has to configure the kinds its loop actually launches — for most
// of them that is `requirements` alone, since the start path is requirements by
// construction. That is **scaffolding, not intent**: these tests are about
// kills, graces, prompts and signals, and the value below is deliberately
// meaningless so a reader does not go looking for significance in it. The tests
// that *are* about model selection name their own models.
const SCAFFOLD_MODEL: &str = "scaffold-model";

fn write_exec(path: &std::path::Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut perms = fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).unwrap();
}

/// Create `dir` as a real git repo — the shape `grove-llm kind` needs to
/// resolve a grove root at all.
///
/// Also scaffolding, and for the same reason: the kind peek used to be skipped
/// entirely when no routing env was set, so a fixture whose loop reached the
/// *continue* path never ran `grove-llm kind` and never needed to be a repo.
/// The peek is unconditional now, and outside a repo it fails — which is a
/// degraded peek, which refuses to launch.
fn init_worktree(dir: &std::path::Path) {
    fs::create_dir_all(dir).unwrap();
    assert!(
        std::process::Command::new("git")
            .args(["init", "-q"])
            .current_dir(dir)
            .status()
            .unwrap()
            .success(),
        "git init failed"
    );
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
    init_worktree(&worktree);

    let counter = repo_path.join("counter");
    let log = repo_path.join("log");

    // Fake claude: log <iter>\t<harness-pid-handle>\t<claude-pid-handle>\t<prompt>;
    // create `.grove/` after the first iteration so the loop switches
    // start→continue; fire the completion signal for the first two
    // iterations, then stop. Logs whether `GROVE_HARNESS_PID`/`GROVE_CLAUDE_PID`
    // are set at all (driver-side-kill-k2): the driver no longer exports
    // either — the agent never needs its own PID, since the driver kills its
    // own child directly.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
for a in "$@"; do prompt="$a"; done
printf '%s\t%s\t%s\t%s\n' "$n" "${GROVE_HARNESS_PID:-unset}" "${GROVE_CLAUDE_PID:-unset}" "$prompt" >> "$GROVE_TEST_LOG"
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        // Scaffolding: only the first (start ⇒ requirements) iteration needs a
        // model at all — the two continue iterations peek an *empty* `.grove/`,
        // which is the no-live-leaf exemption.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

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

    // Neither PID handle is exported any more: the driver kills its own
    // child directly, so the agent never needs its own PID (driver-side-kill-k2).
    for row in &rows {
        assert_eq!(
            row[1], "unset",
            "GROVE_HARNESS_PID must no longer be exported to the harness (row: {row:?})"
        );
        assert_eq!(
            row[2], "unset",
            "GROVE_CLAUDE_PID must no longer be exported to the harness (row: {row:?})"
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        // Scaffolding: the single iteration is the start path ⇒ requirements.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

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

// Signal-file identity (signal-file-identity-k6): two `grove do` loops with
// the *same grove name* but *different worktrees* must not interfere, even
// running truly concurrently. Pre-fix, `signal_file_path` derived the path
// from `name` alone (`$TMPDIR/grove-loop-<name>.signal`), so two worktrees
// that happen to share a basename (generic names like "bugs"/"plan"/"docs"
// are the norm) collided on one file.
//
// Runs two `run_loop`s on real OS threads at the same time, both named
// "samegrove": an "attacker" loop whose fake harness signals `done`
// immediately then hangs (the driver must still kill *its own* child — a
// sanity check this test also covers), and a "victim" loop whose fake
// harness never touches its own signal file and just outlives the
// attacker's entire kill sequence before exiting cleanly on its own.
// Pre-fix, the attacker's `done` write would land in the file the victim's
// watcher was *also* polling (same name ⇒ same path), killing the victim
// early and reporting a phantom clean finish from content it never wrote.
//
// Both loops share one fake-harness script (env vars like GROVE_HARNESS_BIN
// are process-global, so two literal scripts can't be threaded through two
// concurrent in-process loops) that branches on a marker file in its own
// `$PWD` — safe because `current_dir` is set per spawned `Command`, unlike
// env vars, so it genuinely differs between the two loops' children.
#[test]
fn concurrent_loops_with_the_same_grove_name_in_different_worktrees_do_not_interfere() {
    let _g = support::lock_env(&ENV_LOCK);

    let repo_a = TempDir::new().unwrap();
    let worktree_a = repo_a.path().join("wt");
    fs::create_dir_all(&worktree_a).unwrap();
    fs::write(worktree_a.join("ROLE_ATTACKER"), "").unwrap();

    let repo_b = TempDir::new().unwrap();
    let worktree_b = repo_b.path().join("wt");
    fs::create_dir_all(&worktree_b).unwrap();

    let skill_dir = repo_a.path().join("skill");
    fs::create_dir_all(skill_dir.join("prompts")).unwrap();
    fs::write(skill_dir.join("prompts/start.md"), "START PROMPT").unwrap();

    let fake = repo_a.path().join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
if [ -f "$PWD/ROLE_ATTACKER" ]; then
  printf 'done\n' > "$GROVE_SIGNAL_FILE"
  exec sleep 30
else
  sleep 1.5
  exit 0
fi
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_KILL_GRACE", "0.2")
        .set("GROVE_KILL_GRACE_KILL", "0.2")
        // Scaffolding: both loops run one start-path (requirements) session.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

    let repo_a_path = repo_a.path().to_path_buf();
    let repo_b_path = repo_b.path().to_path_buf();
    let started = Instant::now();
    let attacker = std::thread::spawn(move || {
        loop_driver::run_loop(harness, &repo_a_path, &worktree_a, "samegrove")
    });
    let victim = std::thread::spawn(move || {
        loop_driver::run_loop(harness, &repo_b_path, &worktree_b, "samegrove")
    });

    let attacker_result = attacker.join().unwrap().unwrap();
    let victim_result = victim.join().unwrap().unwrap();
    let elapsed = started.elapsed();

    assert_eq!(
        attacker_result,
        LoopOutcome::Finished,
        "sanity check: the attacker's own `done` signal still ends its own loop"
    );
    assert_eq!(
        victim_result,
        LoopOutcome::Stopped,
        "the victim's session ended without ever signalling anything of its \
         own — a foreign `done` from the other worktree's loop must not be \
         mistaken for its own completion signal"
    );
    assert!(
        elapsed >= Duration::from_millis(1200),
        "the victim must run its full ~1.5s session, not be cut short by the \
         attacker's early SIGTERM (elapsed: {elapsed:?})"
    );
    assert!(
        elapsed < Duration::from_secs(10),
        "sanity bound: neither loop should hang (elapsed: {elapsed:?})"
    );
}

// Driver-side kill (driver-side-kill-k2): the loop driver, not the agent,
// ends the harness session once the completion signal fires. These three
// tests stand in for a `claude` session whose turn is still wrapping up when
// `complete` writes the signal file — a fake that signals then keeps running
// (a 30s `sleep`) rather than exiting on its own, so the *only* way `run_loop`
// returns quickly is if the watcher actually killed it. Small
// `GROVE_KILL_GRACE`/`GROVE_KILL_GRACE_KILL` values keep the tests fast
// without a new test seam (BRIEF.md Notes).

#[test]
fn driver_kills_a_hung_session_that_signalled_done() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let worktree = repo_path.join(".grove-worktrees/killgrove");
    fs::create_dir_all(&worktree).unwrap();

    // `exec sleep 30`, not a plain `sleep 30`: exec replaces the shell's own
    // process image (same pid), so the pid the driver signals *is* the
    // sleeping process — a plain `sleep 30` would run as the shell's child,
    // leaving it orphaned (and still asleep for the full 30s) once the driver
    // kills the shell around it.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
printf 'done\n' > "$GROVE_SIGNAL_FILE"
exec sleep 30
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_KILL_GRACE", "0.2")
        .set("GROVE_KILL_GRACE_KILL", "0.3")
        // Scaffolding: one start-path (requirements) session.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

    let started = Instant::now();
    let result = loop_driver::run_loop(harness, repo_path, &worktree, "killgrove");
    let elapsed = started.elapsed();

    assert_eq!(
        result.unwrap(),
        LoopOutcome::Finished,
        "a `done` signal must still end the loop cleanly even though the \
         session hung instead of exiting on its own"
    );
    assert!(
        elapsed < Duration::from_secs(10),
        "the driver must kill the hung session promptly, not wait out its \
         30s sleep (elapsed: {elapsed:?})"
    );
}

#[test]
fn driver_kills_a_hung_session_that_signalled_relaunch() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    // A repo, because iteration 2 takes the continue path and its kind peek is
    // no longer skippable (see `init_worktree`).
    let worktree = repo_path.join(".grove-worktrees/killgrove2");
    init_worktree(&worktree);

    let counter = repo_path.join("counter");

    // Iteration 1 signals a relaunch then hangs (must be killed by the
    // driver); iteration 2 materialises `.grove/` and exits immediately with
    // no signal of its own, so the loop stops there rather than looping on
    // the same hang forever. `exec sleep 30` (not a plain `sleep 30`) so the
    // hang takes over the shell's own pid rather than orphaning a grandchild
    // once the driver kills it (see the sibling test above).
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
mkdir -p "$PWD/.grove"
if [ "$n" -eq 1 ]; then
  : > "$GROVE_SIGNAL_FILE"
  exec sleep 30
fi
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_KILL_GRACE", "0.2")
        .set("GROVE_KILL_GRACE_KILL", "0.3")
        // Scaffolding: iteration 1 is the start path ⇒ requirements; iteration 2
        // peeks an empty `.grove/` and needs nothing.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

    let started = Instant::now();
    let result = loop_driver::run_loop(harness, repo_path, &worktree, "killgrove2");
    let elapsed = started.elapsed();

    assert_eq!(
        result.unwrap(),
        LoopOutcome::Stopped,
        "the first (hung) session's relaunch signal must still be honoured \
         once the driver kills it; the second session then stops the loop \
         with no signal of its own"
    );
    assert!(
        elapsed < Duration::from_secs(10),
        "the driver must kill the hung first session promptly, not wait out \
         its 30s sleep (elapsed: {elapsed:?})"
    );
    assert_eq!(
        fs::read_to_string(&counter).unwrap().trim(),
        "2",
        "both iterations must have run — the kill did not just abort the loop"
    );
}

// watcher-test-hardening-k7, mutant 1: dropping the `signal_file.exists()`
// guard makes the watcher SIGTERM *every* session `grace` after launch,
// signalled or not — every fixture above either exits immediately or
// signals-then-hangs, so none proves the driver leaves an un-signalled
// session alone. (`concurrent_loops_with_the_same_grove_name_...` above
// happens to also fail under this mutant, as a side effect of proving
// signal-file identity, but only by ~180ms out of its 1.2s margin — too
// tight to trust as this property's real coverage.) `exec sleep 1.5`, not a
// plain one, for the same reason as the sibling kill tests: same pid, no
// orphaned grandchild.
#[test]
fn driver_leaves_an_unsignalled_session_alone() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let worktree = repo_path.join(".grove-worktrees/killgrove4");
    fs::create_dir_all(&worktree).unwrap();

    // Never touches `$GROVE_SIGNAL_FILE`; just runs ~1.5s then exits on its
    // own. Small graces so a wrongly-early kill (guard dropped ⇒ SIGTERM at
    // ~grace, SIGKILL at ~grace+kill_grace) lands in well under a second —
    // unambiguously short of the full 1.5s.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
exec sleep 1.5
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_KILL_GRACE", "0.2")
        .set("GROVE_KILL_GRACE_KILL", "0.2")
        // Scaffolding: one start-path (requirements) session.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

    let started = Instant::now();
    let result = loop_driver::run_loop(harness, repo_path, &worktree, "killgrove4");
    let elapsed = started.elapsed();

    assert_eq!(
        result.unwrap(),
        LoopOutcome::Stopped,
        "a session that never signals must stop the loop, not relaunch"
    );
    assert!(
        elapsed >= Duration::from_millis(1400),
        "the watcher must leave an un-signalled session alone — a dropped \
         `signal_file.exists()` guard would SIGTERM it ~0.2-0.7s after \
         launch, far short of its own ~1.5s natural completion \
         (elapsed: {elapsed:?})"
    );
    assert!(
        elapsed < Duration::from_secs(10),
        "sanity bound: the session should not hang (elapsed: {elapsed:?})"
    );
}

#[test]
fn driver_escalates_to_sigkill_when_the_session_ignores_sigterm() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let worktree = repo_path.join(".grove-worktrees/killgrove3");
    fs::create_dir_all(&worktree).unwrap();

    let term_marker = repo_path.join("term-received");

    // watcher-test-hardening-k7, mutant 2: traps SIGTERM and *records*
    // receipt rather than only ignoring it — a bare `trap '' TERM` fixture
    // passes whether or not TERM is ever actually sent (the escalation to
    // SIGKILL happens either way), so it cannot distinguish real
    // TERM-then-KILL from a driver that sends KILL from the start
    // (BRIEF.md Notes: "Keep TERM-before-KILL"). This fixture still never
    // exits on its own — forcing the SIGKILL escalation the test name
    // promises — but the marker file only appears if a real, catchable
    // SIGTERM landed first. No `exec sleep 30` here: a caught disposition
    // does not survive `exec` (POSIX preserves only SIG_IGN across it), so
    // the trap must run in the script's own shell process — which is why
    // this sleeps in short increments rather than one blocking `sleep 30`:
    // a signal delivered to a shell blocked on one long foreground child is
    // not guaranteed to be handled until that child exits, but a signal
    // between/within short sleeps is.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        &format!(
            r#"#!/bin/sh
trap 'printf term > "{marker}"' TERM
printf 'done\n' > "$GROVE_SIGNAL_FILE"
i=0
while [ "$i" -lt 300 ]; do
  sleep 0.1
  i=$((i + 1))
done
"#,
            marker = term_marker.display()
        ),
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_KILL_GRACE", "0.2")
        .set("GROVE_KILL_GRACE_KILL", "0.3")
        // Scaffolding: one start-path (requirements) session.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

    let started = Instant::now();
    let result = loop_driver::run_loop(harness, repo_path, &worktree, "killgrove3");
    let elapsed = started.elapsed();

    assert_eq!(
        result.unwrap(),
        LoopOutcome::Finished,
        "SIGKILL must end a session that ignores SIGTERM"
    );
    assert!(
        elapsed < Duration::from_secs(10),
        "the driver must escalate to SIGKILL rather than waiting out the \
         30s loop (elapsed: {elapsed:?})"
    );
    assert!(
        term_marker.exists(),
        "the session must have received a real, catchable SIGTERM before \
         the SIGKILL landed — a driver that sent SIGKILL from the start \
         (skipping TERM) would kill it just as fast, with the trap never \
         firing (watcher-test-hardening-k7)"
    );
}

// watcher-test-hardening-k7, mutant 3: dropping the grace guard fires the
// kill the instant the signal file appears — every timing assertion in this
// file so far is an upper bound (`elapsed < 10s`); none pins a *lower*
// bound, so a regression to grace≈0 is invisible. The grace exists so
// `complete`'s Bash-tool call can return and the agent's own turn can end
// before its session dies (BRIEF.md); this proves the watcher actually
// waits for it. `GROVE_KILL_GRACE=3.0` — well above `POLL_INTERVAL` (500ms)
// — so a dropped-guard instant kill (bounded to ~2 poll intervals,
// independent of the configured grace) and a correctly-honoured 3s grace
// land unambiguously on opposite sides of the threshold below.
#[test]
fn driver_waits_the_grace_before_sending_sigterm() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let worktree = repo_path.join(".grove-worktrees/killgrove5");
    fs::create_dir_all(&worktree).unwrap();

    // Signals `done` immediately then hangs, same as the sibling kill tests
    // — `exec sleep 30` so the pid the driver signals is the sleeping
    // process itself.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
printf 'done\n' > "$GROVE_SIGNAL_FILE"
exec sleep 30
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_KILL_GRACE", "3.0")
        .set("GROVE_KILL_GRACE_KILL", "0.3")
        // Scaffolding: one start-path (requirements) session.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

    let started = Instant::now();
    let result = loop_driver::run_loop(harness, repo_path, &worktree, "killgrove5");
    let elapsed = started.elapsed();

    assert_eq!(
        result.unwrap(),
        LoopOutcome::Finished,
        "a `done` signal must still end the loop cleanly"
    );
    assert!(
        elapsed >= Duration::from_millis(2500),
        "the watcher must honour the configured grace before its first \
         SIGTERM — a dropped grace guard would kill within ~2 poll \
         intervals of the signal file appearing, independent of the \
         configured grace (elapsed: {elapsed:?})"
    );
    assert!(
        elapsed < Duration::from_secs(10),
        "sanity bound: the driver must not hang (elapsed: {elapsed:?})"
    );
}

// Model selection (model-per-task-kind): the driver launches each session on a
// model chosen by the picked leaf's **kind**. The start path is `requirements`
// by construction (fresh-grove-start-contract); the continue path peeks the next
// live leaf's kind via the real `grove-llm kind` binary (wired in via the
// `GROVE_LLM_BIN` seam, run against a real git worktree so `kind` resolves the
// grove root). Asserts the exact `--model` per iteration, across three of the
// seventeen kinds — requirements (start), then two continue kinds, one of them a
// *hyphenated* one (`impl`, then `review-impl`) — proving the scheme is a real
// per-kind lookup and that the label → env-suffix mapping survives a hyphen.
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
  printf '# a-k1\n\n**Kind:** impl\n' > "$PWD/.grove/01-a-k1.md"
fi
if [ "$n" -eq 2 ]; then
  printf '# a-k1\n\n**Kind:** review-impl\n' > "$PWD/.grove/01-a-k1.md"
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
    // — ambient `GROVE_REVIEW_IMPL_HARNESS`/`GROVE_PI_REVIEW_IMPL_MODEL` would silently
    // reroute iteration 3 (B1).
    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_REQUIREMENTS_MODEL", "opus")
        .set("GROVE_IMPL_MODEL", "sonnet")
        .set("GROVE_REVIEW_IMPL_MODEL", "haiku");

    let result = loop_driver::run_loop(harness, worktree, worktree, "modelgrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        rows.len(),
        3,
        "loop should run three times then stop (log: {log:?})"
    );

    // Iteration 1 — start path ⇒ requirements ⇒ GROVE_REQUIREMENTS_MODEL.
    assert!(
        rows[0].contains("--model opus"),
        "start (requirements) session must launch on the requirements model (argv: {:?})",
        rows[0]
    );
    assert!(
        !rows[0].contains("sonnet") && !rows[0].contains("haiku"),
        "start session must use only the requirements model (argv: {:?})",
        rows[0]
    );
    // Iteration 2 — continue path ⇒ impl leaf ⇒ GROVE_IMPL_MODEL.
    assert!(
        rows[1].contains("--model sonnet"),
        "continue (impl) session must launch on the impl model (argv: {:?})",
        rows[1]
    );
    // Iteration 3 — continue path ⇒ review-impl leaf ⇒ GROVE_REVIEW_IMPL_MODEL.
    // The discriminating case: a hyphenated label has to reach the underscored
    // env suffix, which a label used verbatim as a var name would not.
    assert!(
        rows[2].contains("--model haiku"),
        "continue (review-impl) session must launch on the review-impl model \
         (argv: {:?})",
        rows[2]
    );
}

// Degrade-on-read must be **loud** (task-kind-taxonomy). An unrecognised
// `**Kind:**` line — a typo, a hand-edited file, or a tree written by a newer
// grove — is treated as `impl`, which in a typical config is the *cheapest*
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
        .env("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .env("GROVE_SKILL_DIR", &skill_dir)
        .env("GROVE_TEST_LOG", &log)
        .env("GROVE_IMPL_MODEL", "sonnet")
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
        "an unrecognised kind degrades to `impl` and still launches — a typo must \
         never jam the unattended loop (argv: {argv:?})"
    );
}

// The load-bearing rule, **inverted** (required-model-vars-k18): a picked leaf
// whose kind resolves no model var makes the launch fail, where it previously
// launched with no `--model` and let the session inherit the user's own
// default. The old rule never clobbered an existing default, which is true and
// beside the point — falling through is still grove deciding which model runs a
// `review-impl` leaf, just invisibly, and it leaves *partial* configuration
// indistinguishable from complete configuration (model-per-task-kind).
//
// Two halves, both necessary. Nothing may launch — a session running on a model
// chosen by omission is the state the requirement exists to make
// unrepresentable. And the refusal must carry its own fix: the kind, plus every
// var that would satisfy it. Driven over a **`review-impl`** leaf because it is
// a kind *with a family*, so all four lattice keys exist for it and the error's
// completeness is observable; a standalone kind would prove only half of it.
#[test]
fn a_kind_with_no_model_var_fails_loudly_instead_of_launching() {
    let _g = support::lock_env(&ENV_LOCK);
    let worktree_dir = TempDir::new().unwrap();
    let worktree = worktree_dir.path();
    init_worktree(worktree);

    fs::create_dir_all(worktree.join(".grove")).unwrap();
    fs::write(worktree.join(".grove/BRIEF.md"), "# g — brief\n").unwrap();
    fs::write(
        worktree.join(".grove/01-a-k1.md"),
        "# a-k1\n\n**Kind:** review-impl\n",
    )
    .unwrap();

    let skill_dir = worktree.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let log = worktree.join("log");
    let fake = worktree.join("fake-claude.sh");
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_LOG", &log);

    let err = loop_driver::run_loop(harness, worktree, worktree, "nomodelgrove")
        .expect_err("a kind resolving no model var must fail the launch")
        .to_string();

    assert!(
        err.contains("review-impl"),
        "the error must name the kind that resolved nothing — it is what the \
         operator has to go configure (err: {err})"
    );
    for key in [
        "GROVE_CLAUDE_REVIEW_IMPL_MODEL",
        "GROVE_CLAUDE_REVIEW_MODEL",
        "GROVE_REVIEW_IMPL_MODEL",
        "GROVE_REVIEW_MODEL",
    ] {
        assert!(
            err.contains(key),
            "the error must list every var that would satisfy the requirement — \
             exact-kind and family, harness-scoped and unscoped — or it sends \
             the operator hunting; missing {key} (err: {err})"
        );
    }
    assert!(
        !log.exists(),
        "no session may launch: running on a model grove chose by omission is \
         exactly what the requirement makes unrepresentable"
    );
}

// T6, carried across the inversion: an *empty-string* model var must still
// behave exactly like an unset one — a blank `GROVE_REQUIREMENTS_MODEL=` (e.g.
// from a shell template that never filled in a value) must never reach the
// harness as a literal empty `--model`. What changed is only the consequence:
// "treated as unset" now means the loud refusal above rather than a bare launch.
//
// The start path, so this covers the standalone-kind shape the sibling test
// cannot: `requirements` has no family, so its error lists two keys, not four.
// It also pins the fresh-grove config contract from the other side — this is
// the var a brand-new grove cannot start without.
#[test]
fn an_empty_string_model_var_fails_loudly_like_an_unset_one() {
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_REQUIREMENTS_MODEL", "");

    let err = loop_driver::run_loop(harness, repo_path, &worktree, "loopgrove")
        .expect_err("an empty-string model var must fail exactly as an unset one does")
        .to_string();

    assert!(
        err.contains("requirements")
            && err.contains("GROVE_REQUIREMENTS_MODEL")
            && err.contains("GROVE_CLAUDE_REQUIREMENTS_MODEL"),
        "the error must name the kind and both keys that would satisfy a \
         standalone (family-less) kind (err: {err})"
    );
    assert!(
        !log.exists(),
        "an empty-string model var must never be passed through as a literal \
         empty --model — and now must not launch at all"
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
  printf '# a-k1\n\n**Kind:** impl\n' > "$PWD/.grove/01-a-k1.md"
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("codex").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        // Scaffolding for run 1 (start ⇒ requirements); run 2 is the impl leaf
        // this test is actually about.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL)
        .set("GROVE_IMPL_MODEL", "sol-high");

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

// The pi harness declaration (pi-session-naming-k13). pi 0.80.10 *does* have a
// launch-time session-name flag (`--name, -n <name>` — "Set session display
// name"), but the registry recorded it as having none, so every pi grove
// session ran unnamed while claude's were pre-named: unidentifiable in pi's
// session picker, and the skill's "suggest /rename once per session" fallback
// fired forever. Drives the real loop over a fake pi and asserts the launched
// argv carries `-n <session-name>` — options before the positional prompt,
// matching pi's `pi [options] [@files...] [messages...]` usage.
#[test]
fn pi_launches_with_its_session_name_flag() {
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

    // Fake pi: log the full argv. The first (start) run materialises a `.grove/`
    // holding one live **work** leaf, so the second run takes the continue path;
    // signal only on the first, so the loop stops after two.
    let fake = worktree.join("fake-pi.sh");
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
  printf '# a-k1\n\n**Kind:** impl\n' > "$PWD/.grove/01-a-k1.md"
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("pi").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        // Scaffolding for run 1 (start ⇒ requirements).
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL)
        .set("GROVE_IMPL_MODEL", "moonshot/k3");

    let result = loop_driver::run_loop(harness, worktree, worktree, "pigrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        rows.len(),
        2,
        "loop should run twice then stop (log: {log:?})"
    );

    // Every launch is pre-named: `-n <repo-basename>: <name> grove`.
    let session_name = format!(
        "{}: pigrove grove",
        worktree.file_name().unwrap().to_string_lossy()
    );
    let name_flag = format!("-n {session_name}");
    for row in &rows {
        assert!(
            row.contains(&name_flag),
            "pi must be pre-named at launch via -n (argv: {row:?})"
        );
    }

    // pi's usage is `pi [options] [@files...] [messages...]`: options strictly
    // before the positional prompt, name flag then model flag (the
    // launch_session order).
    let row = rows[1];
    let name_at = row.find(&name_flag).unwrap();
    let model_at = row
        .find("--model moonshot/k3")
        .expect("pi honours model-per-task-kind via --model");
    let prompt_at = row
        .find("CONTINUE PROMPT")
        .expect("continue prompt must be the positional argument");
    assert!(
        name_at < model_at && model_at < prompt_at,
        "options must precede the positional prompt: -n, then --model, then \
         the prompt (argv: {row:?})"
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

    // Fake harness: run 1 (start/requirements) materialises an impl leaf + signal;
    // run 2 (continue/impl) stops.
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
  printf '# a-k1\n\n**Kind:** impl\n' > "$PWD/.grove/01-a-k1.md"
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    // clear_grove_env first (this repo dogfoods per-kind model envs — BRIEF.md
    // Notes — so a session driving this very test suite may already have
    // GROVE_REQUIREMENTS_MODEL etc. set), then layer the scenario's own vars:
    // a scoped override for the launching harness + a base var it must beat,
    // and an override for a *different* harness that must be ignored.
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_CLAUDE_IMPL_MODEL", "kimi-k3")
        .set("GROVE_IMPL_MODEL", "sonnet")
        .set("GROVE_REQUIREMENTS_MODEL", "requirements-base")
        .set("GROVE_PI_REQUIREMENTS_MODEL", "must-not-leak");

    let result = loop_driver::run_loop(harness, worktree, worktree, "envgrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(rows.len(), 2, "loop should run twice (log: {log:?})");

    // Start/requirements: the base requirements var applies and the *pi*-scoped
    // one does not. This is a structural guarantee, not a live-risk assertion —
    // `model_keys` only ever interpolates *this* harness's own name into the
    // env-vars it reads (src/loop_driver.rs), so a pi-scoped var has no code
    // path that could consult it; kept as documentation of that intent,
    // alongside the genuinely discriminating precedence check below. It could
    // no longer be written as "no --model at all": that state is now a hard
    // error, not a quiet launch (required-model-vars-k18).
    assert!(
        rows[0].contains("--model requirements-base") && !rows[0].contains("must-not-leak"),
        "another harness's scoped var must not select a model (argv: {:?})",
        rows[0]
    );
    // Continue/work: the claude-scoped var beats the base var — the
    // discriminating assertion (precedence, not cross-harness isolation).
    assert!(
        rows[1].contains("--model kimi-k3") && !rows[1].contains("sonnet"),
        "GROVE_CLAUDE_IMPL_MODEL must beat GROVE_IMPL_MODEL (argv: {:?})",
        rows[1]
    );
}

// Per-kind harness routing: GROVE_REVIEW_IMPL_HARNESS=pi must launch review leaves
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

    // Fake codex: tags rows "codex"; run 1 (start/requirements) materialises a
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
  printf '# a-k1\n\n**Kind:** review-impl\n' > "$PWD/.grove/01-a-k1.md"
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_REVIEW_IMPL_HARNESS", "pi")
        .set("GROVE_CODEX_REQUIREMENTS_MODEL", "sol-xhigh")
        .set("GROVE_PI_REVIEW_IMPL_MODEL", "kimi-code/k3");

    let result = loop_driver::run_loop(harness, worktree, worktree, "reroutegrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<Vec<&str>> = log
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.splitn(3, '\t').collect())
        .collect();
    assert_eq!(rows.len(), 2, "loop should run twice (log: {log:?})");

    // Bootstrap (requirements) leaf: the stamped harness (codex) with its
    // scoped profile, reading codex's own start prompt.
    assert_eq!(
        rows[0][0], "codex",
        "the bootstrap leaf stays on the stamped harness"
    );
    assert!(
        rows[0][1].contains("--profile sol-xhigh"),
        "codex requirements launches on its scoped profile (argv: {:?})",
        rows[0][1]
    );
    assert_eq!(
        rows[0][2], "CODEX START PROMPT",
        "the bootstrap session must read codex's own start prompt"
    );

    // Review leaf: rerouted to pi, with pi's scoped model — the launch flag
    // template must be the *post-override* harness's (--model, not --profile)
    // — and pi's own continue prompt, not codex's (B7).
    assert_eq!(
        rows[1][0], "pi",
        "review must reroute to GROVE_REVIEW_IMPL_HARNESS"
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
// the harness a per-kind override reroutes to. Only the harness-scoped var may
// supply a model once a reroute has happened.
//
// What that means changed with required-model-vars-k18: the rerouted leaf used
// to launch with no `--model` at all, and now fails loudly, because "resolves
// nothing" is a configuration error wherever it happens. The test discriminates
// exactly as sharply either way — a base var that *did* leak would launch pi on
// `sol-high` instead of erroring — and the refusal is the louder of the two.
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
  printf '# a-k1\n\n**Kind:** review-impl\n' > "$PWD/.grove/01-a-k1.md"
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_REVIEW_IMPL_HARNESS", "pi")
        // Scaffolding: run 1 is the start path ⇒ requirements, which is not
        // rerouted, so the unscoped var reaches the stamped codex.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL)
        // The base var — a codex profile name, meaningless to pi — with no
        // GROVE_PI_REVIEW_IMPL_MODEL set to beat it.
        .set("GROVE_REVIEW_IMPL_MODEL", "sol-high");

    let err = loop_driver::run_loop(harness, worktree, worktree, "basemodelgrove")
        .expect_err("a reroute that resolves no harness-scoped model must fail")
        .to_string();

    assert!(
        err.contains("GROVE_PI_REVIEW_IMPL_MODEL"),
        "the refusal must point at the *pi*-scoped keys, the only ones that can \
         satisfy a rerouted launch (err: {err})"
    );
    assert!(
        err.contains("rerouted"),
        "…and must say why the base var the operator can plainly see set was \
         declined rather than used (err: {err})"
    );

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<Vec<&str>> = log
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.splitn(2, '\t').collect())
        .collect();
    assert_eq!(
        rows.len(),
        1,
        "only the stamped bootstrap session may have run — the rerouted review \
         leaf must never launch (log: {log:?})"
    );
    assert!(
        !log.contains("sol-high"),
        "the base GROVE_REVIEW_IMPL_MODEL (a codex profile name) must not reach \
         pi across a reroute (log: {log:?})"
    );
}

// ── The family axis (family-fallback-k14) ────────────────────────────────
//
// One variable states a policy covering all five kinds of a family:
// `GROVE_REVIEW_HARNESS` / `GROVE_REVIEW_MODEL` govern every `review-*` leaf,
// and the exact-kind var beats them. Driven through the whole-`grove do` seam
// the spec names (docs/specs/task-kind-taxonomy.md, *Test seams*): the real
// driver, a fake binary per vendor, assertions on the recorded argv.

/// Drive the real loop over exactly one leaf of `kind`, with `vars` layered on
/// a scrubbed environment. Run 1 takes the start path (requirements by
/// construction — fresh-grove-start-contract), materialises the leaf and
/// signals; run 2 takes the continue path over that leaf and stops without
/// signalling. Returns the loop's own outcome plus one `(harness, argv)` row
/// per launch, so a case can assert *which* harness ran the leaf, *what* model
/// flag it carried, **and** — since required-model-vars-k18 — whether it was
/// allowed to launch at all.
///
/// All three harnesses are wired to their own fake binary through the
/// per-harness `GROVE_HARNESS_BIN_<NAME>` seam, so a reroute is *observed* in
/// the row's first field rather than inferred from the flags. The caller holds
/// `ENV_LOCK`; the guard this sets up lives only for the call, which is why
/// each case reads its rows before configuring the next one.
///
/// `GROVE_REQUIREMENTS_MODEL` is set here as scaffolding, before `vars`, so
/// every case gets past run 1 without restating it: requirements is never the
/// kind under test in this section, and a case that wants a different one can
/// still override it.
fn drive_one_leaf(
    stamped: &str,
    kind: &str,
    vars: &[(&str, &str)],
) -> (Result<LoopOutcome, String>, Vec<(String, String)>) {
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

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_TEST_KIND", kind)
        // The leaf the fake materialises carries a `**Harness:**` line only when
        // a case asks for one, by setting this in `vars` — so every existing
        // case keeps writing the undeclared leaf it always wrote. Explicitly
        // cleared rather than merely unset: `clear_grove_env` sweeps the routing
        // surface, not the fixture's own handles, and a value left behind by the
        // previous case in this binary would silently reroute the next one.
        .remove("GROVE_TEST_LEAF_HARNESS");

    for name in ["claude", "codex", "pi"] {
        let fake = worktree.join(format!("fake-{name}.sh"));
        write_exec(
            &fake,
            &format!(
                r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
printf '{name}\t%s\n' "$*" >> "$GROVE_TEST_LOG"
if [ "$n" -eq 1 ]; then
  mkdir -p "$PWD/.grove"
  printf '# g — brief\n' > "$PWD/.grove/BRIEF.md"
  printf '# a-k1\n\n**Kind:** %s\n' "$GROVE_TEST_KIND" > "$PWD/.grove/01-a-k1.md"
  if [ -n "$GROVE_TEST_LEAF_HARNESS" ]; then
    printf '**Harness:** %s\n' "$GROVE_TEST_LEAF_HARNESS" >> "$PWD/.grove/01-a-k1.md"
  fi
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#
            ),
        );
        env.set(&format!("GROVE_HARNESS_BIN_{}", name.to_uppercase()), &fake);
    }

    env.set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);
    for (key, value) in vars {
        env.set(key, value);
    }

    let result = loop_driver::run_loop(
        harness::by_name(stamped).unwrap(),
        worktree,
        worktree,
        "familygrove",
    )
    .map_err(|e| e.to_string());

    let log = fs::read_to_string(&log).unwrap_or_default();
    let rows: Vec<(String, String)> = log
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (h, argv) = l.split_once('\t').expect("the fake logs harness\\targv");
            (h.to_string(), argv.to_string())
        })
        .collect();
    (result, rows)
}

/// [`drive_one_leaf`] for the cases that must launch: asserts the loop ran both
/// sessions and stopped normally, and hands back the two rows.
fn loop_over_one_leaf(stamped: &str, kind: &str, vars: &[(&str, &str)]) -> Vec<(String, String)> {
    let (result, rows) = drive_one_leaf(stamped, kind, vars);
    let outcome = result.unwrap_or_else(|e| panic!("the loop must not fail here: {e}"));
    assert_eq!(outcome, LoopOutcome::Stopped);
    assert_eq!(rows.len(), 2, "loop should run twice (rows: {rows:?})");
    rows
}

/// [`drive_one_leaf`] for the cases that must **not** launch: asserts the loop
/// refused, and hands back the refusal text alongside whatever did run before
/// it — so a case can prove both that the error is diagnostic and that no
/// session slipped through.
fn refusal_over_one_leaf(
    stamped: &str,
    kind: &str,
    vars: &[(&str, &str)],
) -> (String, Vec<(String, String)>) {
    let (result, rows) = drive_one_leaf(stamped, kind, vars);
    let err = result.expect_err("this configuration must refuse to launch");
    (err, rows)
}

// The claim the family axis exists to make good on: *one* line covers all five
// kinds of a family. Without it the same policy would be written five times and
// hand-kept in sync, and the seventeen-kind set would not pay for itself
// (model-per-task-kind). All five, because "covers the family" is exactly the
// property a per-kind implementation would satisfy for four of them.
#[test]
fn one_family_model_var_covers_every_kind_in_the_family() {
    let _g = support::lock_env(&ENV_LOCK);
    for kind in [
        "review-requirements",
        "review-design",
        "review-planning",
        "review-prototype",
        "review-impl",
    ] {
        let rows = loop_over_one_leaf("claude", kind, &[("GROVE_REVIEW_MODEL", "sonnet")]);
        assert!(
            rows[1].1.contains("--model sonnet"),
            "GROVE_REVIEW_MODEL must cover {kind} (argv: {:?})",
            rows[1].1
        );
    }
}

// "Specific beats general" on the kind axis, at both scopes and on both vars.
// The family var is the fallback, never the winner, whenever the exact kind is
// configured alongside it — which has to hold *within* each scope, not only
// between the unscoped pair, or the lattice's key 2 would swallow key 1.
#[test]
fn an_exact_kind_var_beats_its_family_var_on_both_axes() {
    let _g = support::lock_env(&ENV_LOCK);

    // Keys 1 vs 2 — both harness-scoped.
    let rows = loop_over_one_leaf(
        "claude",
        "review-impl",
        &[
            ("GROVE_CLAUDE_REVIEW_IMPL_MODEL", "opus"),
            ("GROVE_CLAUDE_REVIEW_MODEL", "sonnet"),
        ],
    );
    assert!(
        rows[1].1.contains("--model opus") && !rows[1].1.contains("sonnet"),
        "GROVE_CLAUDE_REVIEW_IMPL_MODEL must beat GROVE_CLAUDE_REVIEW_MODEL \
         (argv: {:?})",
        rows[1].1
    );

    // Keys 3 vs 4 — both unscoped.
    let rows = loop_over_one_leaf(
        "claude",
        "review-impl",
        &[
            ("GROVE_REVIEW_IMPL_MODEL", "opus"),
            ("GROVE_REVIEW_MODEL", "sonnet"),
        ],
    );
    assert!(
        rows[1].1.contains("--model opus") && !rows[1].1.contains("sonnet"),
        "GROVE_REVIEW_IMPL_MODEL must beat GROVE_REVIEW_MODEL (argv: {:?})",
        rows[1].1
    );

    // The harness axis. The pi-scoped model is scaffolding: a rerouted launch
    // consults only harness-scoped keys, and one that resolves none now
    // refuses before it can be observed on either harness.
    let rows = loop_over_one_leaf(
        "claude",
        "review-impl",
        &[
            ("GROVE_REVIEW_IMPL_HARNESS", "pi"),
            ("GROVE_REVIEW_HARNESS", "codex"),
            ("GROVE_PI_REVIEW_IMPL_MODEL", SCAFFOLD_MODEL),
            ("GROVE_CODEX_REVIEW_MODEL", SCAFFOLD_MODEL),
        ],
    );
    assert_eq!(
        rows[1].0, "pi",
        "GROVE_REVIEW_IMPL_HARNESS must beat GROVE_REVIEW_HARNESS"
    );
}

// The user's actual configuration (this node's brief, *Notes*): the whole
// policy layer is two lines — a family harness var and the matching
// harness-scoped family model var — and everything else falls through to the
// stamp. Also the only end-to-end exercise of lattice key 2
// (`GROVE_<HARNESS>_<FAMILY>_MODEL`), which is the one key that has to survive
// a reroute.
#[test]
fn the_two_line_review_policy_routes_a_review_leaf_by_family_alone() {
    let _g = support::lock_env(&ENV_LOCK);
    let rows = loop_over_one_leaf(
        "claude",
        "review-design",
        &[
            ("GROVE_REVIEW_HARNESS", "codex"),
            ("GROVE_CODEX_REVIEW_MODEL", "sol-high"),
        ],
    );
    assert_eq!(
        rows[0].0, "claude",
        "requirements has no family and stays on the stamped harness"
    );
    assert_eq!(rows[1].0, "codex", "the review leaf routes by its family");
    assert!(
        rows[1].1.contains("--profile sol-high"),
        "a harness-scoped family model var must survive the reroute, under the \
         *launching* harness's flag template (argv: {:?})",
        rows[1].1
    );
}

// The two family labels overlap as strings — `integrate-review-impl` contains
// `review` — so longest match wins. The second half of this test is the one
// that fails under naive substring matching: with *only* the review family
// configured, an integration leaf must resolve nothing at all, on either axis.
#[test]
fn integrate_review_resolves_to_its_own_family_never_to_review() {
    let _g = support::lock_env(&ENV_LOCK);

    let rows = loop_over_one_leaf(
        "claude",
        "integrate-review-impl",
        &[
            ("GROVE_REVIEW_MODEL", "reviewer-model"),
            ("GROVE_INTEGRATE_REVIEW_MODEL", "integrator-model"),
        ],
    );
    assert!(
        rows[1].1.contains("--model integrator-model") && !rows[1].1.contains("reviewer-model"),
        "an integration step must take its own family's model (argv: {:?})",
        rows[1].1
    );

    // The harness half. `GROVE_INTEGRATE_REVIEW_MODEL` is scaffolding — the
    // leaf has to resolve *some* model to launch at all now — chosen so the
    // model assertion still discriminates: if the review family captured this
    // leaf on either axis, it would run on pi and/or `reviewer-model`.
    let rows = loop_over_one_leaf(
        "claude",
        "integrate-review-impl",
        &[
            ("GROVE_REVIEW_MODEL", "reviewer-model"),
            ("GROVE_REVIEW_HARNESS", "pi"),
            ("GROVE_INTEGRATE_REVIEW_MODEL", "integrator-model"),
        ],
    );
    assert_eq!(
        rows[1].0, "claude",
        "the review family must not capture an integration leaf — it belongs to \
         integrate-review, which is unconfigured on the harness axis here, so \
         the leaf stays on the stamped harness"
    );
    assert!(
        !rows[1].1.contains("reviewer-model"),
        "…and takes nothing from the review family's model var either \
         (argv: {:?})",
        rows[1].1
    );
}

// Harness-major, and the case that distinguishes it from kind-major: a
// harness-scoped *family* var (lattice key 2) beats an unscoped *exact-kind*
// var (key 3). Kind-major ordering would invert this and hand the launch a
// value written with some other harness in mind — the precise failure the
// harness axis exists to prevent (model-per-task-kind).
#[test]
fn a_harness_scoped_family_var_beats_an_unscoped_exact_kind_var() {
    let _g = support::lock_env(&ENV_LOCK);
    let rows = loop_over_one_leaf(
        "claude",
        "review-impl",
        &[
            ("GROVE_CLAUDE_REVIEW_MODEL", "scoped-family"),
            ("GROVE_REVIEW_IMPL_MODEL", "unscoped-kind"),
        ],
    );
    assert!(
        rows[1].1.contains("--model scoped-family") && !rows[1].1.contains("unscoped-kind"),
        "the harness axis outranks the kind axis (argv: {:?})",
        rows[1].1
    );
}

// The reroute rule and the family fallback run along different axes, and the
// family fallback must compose with the reroute rule rather than open a hole
// in it: falling back `review-impl` → `review` is fine, but the *unscoped*
// family var is still a value written for some other harness (here a codex
// profile name, garbage to pi) and must not follow the kind across a reroute.
//
// With no `GROVE_PI_*` key to supply one instead, that leaves the rerouted leaf
// resolving nothing — which is a refusal now rather than a bare launch
// (required-model-vars-k18). The property under test is unchanged and the
// discrimination is the same: a family var that *did* cross would launch pi on
// a codex profile name instead of erroring.
#[test]
fn an_unscoped_family_model_var_does_not_survive_a_reroute() {
    let _g = support::lock_env(&ENV_LOCK);
    let (err, rows) = refusal_over_one_leaf(
        "codex",
        "review-impl",
        &[
            ("GROVE_REVIEW_IMPL_HARNESS", "pi"),
            ("GROVE_REVIEW_MODEL", "sol-high"),
        ],
    );
    assert!(
        err.contains("GROVE_PI_REVIEW_IMPL_MODEL") && err.contains("GROVE_PI_REVIEW_MODEL"),
        "the refusal must name the two pi-scoped keys — the whole lattice a \
         rerouted launch has left (err: {err})"
    );
    assert_eq!(
        rows.len(),
        1,
        "only the stamped bootstrap session ran; the rerouted review leaf never \
         launched (rows: {rows:?})"
    );
    assert!(
        !rows.iter().any(|(_, argv)| argv.contains("sol-high")),
        "the unscoped family var must not cross the reroute (rows: {rows:?})"
    );
}

// A family var is the var a user is most likely to set exactly once and never
// look at again, so a typo in one must fail at the very next launch — the same
// contract `an_off_kind_harness_override_typo_is_caught_immediately` pins for
// kind vars. The start path resolves straight to Planning, which has no family
// and never consults this var at all.
#[test]
fn a_family_harness_override_typo_is_caught_immediately() {
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

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_INTEGRATE_REVIEW_HARNESS", "lemur");

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "famtypogrove");

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("GROVE_INTEGRATE_REVIEW_HARNESS") && err.contains("lemur"),
        "a typo in a family override must fail at the very next launch (err: {err})"
    );
}

// Pre-flight resolves every harness a launch might need. A family var names a
// harness exactly as a kind var does, so it must be pre-flighted the same way
// — otherwise `GROVE_REVIEW_HARNESS=codex` with no codex installed sails
// through and only dies once the first review leaf is finally picked, which is
// the whole failure harness-spawn-preflight-k8 exists to close.
#[test]
fn preflight_check_catches_a_missing_family_override_binary() {
    let _g = support::lock_env(&ENV_LOCK);
    let tmp = TempDir::new().unwrap();

    let fake_claude = tmp.path().join("fake-claude.sh");
    write_exec(&fake_claude, "#!/bin/sh\nexit 0\n");
    let missing_pi = tmp.path().join("no-such-pi");

    let stamped = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake_claude)
        .set("GROVE_HARNESS_BIN_PI", &missing_pi)
        .set("GROVE_REVIEW_HARNESS", "pi");

    let err = loop_driver::preflight_check(stamped).unwrap_err();
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
        msg.contains("review-*"),
        "diagnostic must say the var covers a whole family, not one kind — a
         reader who set one line should not be sent hunting for a per-kind var \
         they never wrote (got: {msg:?})"
    );
}

// ── The per-leaf axis (leaf-harness-k15) ─────────────────────────────────
//
// A leaf may name its own harness on a `**Harness:**` line, and that beats every
// policy var and the grove's own stamp. It exists for the **vendor pair** — two
// `research` leaves differing only by vendor — which is the one shape a
// kind→harness *function* cannot express, so `research` is the kind driven
// throughout. Same seam as the family axis above: the real driver, a fake binary
// per vendor, assertions on the recorded argv.

// The claim itself. `research` has no policy var set anywhere here, so the only
// thing that can move this leaf off the stamp is the line on the leaf.
#[test]
fn a_leaf_declared_harness_launches_there_whatever_the_stamp() {
    let _g = support::lock_env(&ENV_LOCK);
    let rows = loop_over_one_leaf(
        "claude",
        "research",
        &[
            ("GROVE_TEST_LEAF_HARNESS", "codex"),
            ("GROVE_CODEX_RESEARCH_MODEL", "sol-high"),
        ],
    );
    assert_eq!(
        rows[0].0, "claude",
        "the requirements start path has no leaf to declare anything and stays on the stamp"
    );
    assert_eq!(rows[1].0, "codex", "the leaf's own declaration must win");
    assert!(
        rows[1].1.contains("--profile sol-high"),
        "the leaf names the seat; the env names who sits in it — the model still \
         comes from the (harness, kind) pair (argv: {:?})",
        rows[1].1
    );
}

// Precedence: leaf beats kind beats family beats stamp. The discriminating
// fixture sets the *kind* var — the most specific thing the env can say about
// this leaf — and still loses, which is the whole point of the axis: the pair's
// second survey goes elsewhere *because its sibling does not*, and no policy
// keyed on `research` can say that about one of two identical-kind leaves.
#[test]
fn a_leaf_declaration_beats_the_per_kind_policy() {
    let _g = support::lock_env(&ENV_LOCK);
    let rows = loop_over_one_leaf(
        "claude",
        "research",
        &[
            ("GROVE_RESEARCH_HARNESS", "pi"),
            ("GROVE_TEST_LEAF_HARNESS", "codex"),
            ("GROVE_CODEX_RESEARCH_MODEL", "sol-high"),
            ("GROVE_PI_RESEARCH_MODEL", SCAFFOLD_MODEL),
        ],
    );
    assert_eq!(
        rows[1].0, "codex",
        "a leaf declaration must outrank GROVE_<KIND>_HARNESS"
    );
}

// `rerouted` is computed against the **stamp**, exactly as it is for the env
// axis — so a leaf-declared reroute gets no unscoped model var and no global
// binary override. Without this the pair's codex leaf could launch on a value
// written for claude, which is the failure the reroute rule exists to prevent.
#[test]
fn a_leaf_declared_reroute_consults_no_unscoped_model_var() {
    let _g = support::lock_env(&ENV_LOCK);
    let (err, rows) = refusal_over_one_leaf(
        "claude",
        "research",
        &[
            ("GROVE_TEST_LEAF_HARNESS", "codex"),
            ("GROVE_RESEARCH_MODEL", "opus"),
        ],
    );
    assert!(
        err.contains("GROVE_CODEX_RESEARCH_MODEL"),
        "the refusal must name the harness-scoped key, which is the whole \
         lattice a rerouted launch has left (err: {err})"
    );
    assert_eq!(
        rows.len(),
        1,
        "only the stamped bootstrap session ran (rows: {rows:?})"
    );
    assert!(
        !rows.iter().any(|(_, argv)| argv.contains("opus")),
        "the unscoped var must not cross the leaf-declared reroute (rows: {rows:?})"
    );
}

// The other side of that rule: declaring the harness the grove is already
// stamped to is **not** a reroute, so the unscoped var still applies. Otherwise
// a leaf could be made unlaunchable by writing down the harness it was already
// going to run on.
#[test]
fn declaring_the_stamped_harness_is_not_a_reroute() {
    let _g = support::lock_env(&ENV_LOCK);
    let rows = loop_over_one_leaf(
        "claude",
        "research",
        &[
            ("GROVE_TEST_LEAF_HARNESS", "claude"),
            ("GROVE_RESEARCH_MODEL", "opus"),
        ],
    );
    assert_eq!(rows[1].0, "claude");
    assert!(
        rows[1].1.contains("--model opus"),
        "an unscoped var must still apply when nothing was rerouted (argv: {:?})",
        rows[1].1
    );
}

// Refuse, do not degrade. A wrong *harness* is not a wrong label: degrading
// would run the leaf on a vendor the tree explicitly said not to. The read side
// refuses (tests/kind.rs proves the message), which surfaces here as a peek the
// driver cannot resolve — and the driver must then stop rather than fall back to
// the stamp, which is the exact fallback the declaration forbade.
#[test]
fn an_unrecognised_leaf_harness_refuses_to_launch() {
    let _g = support::lock_env(&ENV_LOCK);
    let (err, rows) = refusal_over_one_leaf(
        "claude",
        "research",
        &[
            ("GROVE_TEST_LEAF_HARNESS", "codx"),
            ("GROVE_RESEARCH_MODEL", SCAFFOLD_MODEL),
        ],
    );
    assert!(
        err.contains("could not be resolved") && err.contains("declares for itself"),
        "the refusal must point at the leaf, since the operator's mistake is on \
         it and not in the environment (err: {err})"
    );
    assert_eq!(
        rows.len(),
        1,
        "the research leaf must not launch on the stamp (rows: {rows:?})"
    );
}

// Pre-flight deliberately does not walk the tree for declarations — it cannot,
// since the tree grows while the loop runs — so the not-installed case is caught
// at launch, and must be caught *by name* rather than as a raw spawn failure.
// Same instruction as the pre-flight refusals: which harness, which binary.
#[test]
fn a_leaf_declared_harness_that_is_not_installed_refuses_by_name() {
    let _g = support::lock_env(&ENV_LOCK);
    let (err, rows) = refusal_over_one_leaf(
        "claude",
        "research",
        &[
            ("GROVE_TEST_LEAF_HARNESS", "pi"),
            ("GROVE_HARNESS_BIN_PI", "/nonexistent/pi-binary"),
            ("GROVE_PI_RESEARCH_MODEL", SCAFFOLD_MODEL),
        ],
    );
    assert!(
        err.contains("pi") && err.contains("/nonexistent/pi-binary"),
        "the refusal must name the harness and the binary it looked for (err: {err})"
    );
    assert!(
        err.contains("not on PATH"),
        "…and say what is actually wrong, rather than reporting it as a \
         mis-declared harness (err: {err})"
    );
    assert_eq!(
        rows.len(),
        1,
        "nothing may launch in its place (rows: {rows:?})"
    );
}

// An unknown harness name in a family var fails loudly at pre-flight too, not
// only once a leaf of that family is picked.
#[test]
fn preflight_check_rejects_an_unknown_family_harness_name() {
    let _g = support::lock_env(&ENV_LOCK);
    let tmp = TempDir::new().unwrap();
    let fake_claude = tmp.path().join("fake-claude.sh");
    write_exec(&fake_claude, "#!/bin/sh\nexit 0\n");

    let stamped = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake_claude)
        .set("GROVE_REVIEW_HARNESS", "lemur");

    let err = loop_driver::preflight_check(stamped).unwrap_err();
    assert!(
        err.to_string().contains("unknown harness"),
        "an unknown family override name must fail loudly (got: {err})"
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
  printf '# a-k1\n\n**Kind:** review-impl\n' > "$PWD/.grove/01-a-k1.md"
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_REVIEW_IMPL_HARNESS", "pi")
        // Scaffolding: run 1 is requirements on the stamped codex, run 2 is the
        // review leaf rerouted to pi — each needs a model to launch at all,
        // and this test is about *which binary* runs, not which model.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL)
        .set("GROVE_PI_REVIEW_IMPL_MODEL", SCAFFOLD_MODEL);

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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_HARNESS_BIN", "")
        // Scaffolding: one start-path (requirements) session.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

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

/// A grove whose continue-path kind peek is guaranteed to degrade: a real tree
/// with a live leaf, driven with `GROVE_LLM_BIN` pointing at a binary that does
/// not exist, so the spawn itself fails (the `Err(e)` arm) rather than any
/// parse or exit-code path. `vars` layers the scenario's own configuration.
fn degraded_peek_error(vars: &[(&str, &str)]) -> String {
    let worktree_dir = TempDir::new().unwrap();
    let worktree = worktree_dir.path();
    init_worktree(worktree);
    fs::create_dir_all(worktree.join(".grove")).unwrap();
    fs::write(worktree.join(".grove/BRIEF.md"), "# g — brief\n").unwrap();
    fs::write(
        worktree.join(".grove/01-a-k1.md"),
        "# a-k1\n\n**Kind:** review-impl\n",
    )
    .unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_LLM_BIN", worktree.join("no-such-grove-llm"));
    for (key, value) in vars {
        env.set(key, value);
    }

    loop_driver::run_loop(
        harness::by_name("codex").unwrap(),
        worktree,
        worktree,
        "degradedgrove",
    )
    .expect_err("a degraded kind peek must refuse to launch")
    .to_string()
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
    let err = degraded_peek_error(&[
        ("GROVE_REVIEW_IMPL_HARNESS", "pi"),
        ("GROVE_PI_REVIEW_IMPL_MODEL", SCAFFOLD_MODEL),
        ("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL),
    ]);
    assert!(
        err.contains("could not be resolved") && err.contains("stamped harness"),
        "a degraded kind peek with an active harness override must fail \
         loudly rather than silently launching on the stamped harness \
         (err: {err})"
    );
}

// The half of the degrade rule that required-model-vars-k18 inverted: a
// degraded peek used to bail *only* when a harness override was configured, and
// otherwise launched on the stamped harness with no model — on the argument
// that "model selection is a nicety, a misroute is not". That asymmetry died
// with the requirement: a model is not a nicety any more, so an undeterminable
// kind can no longer be routed by guessing on *either* axis
// (model-per-task-kind).
//
// The discriminating fixture is a config with **nothing routed at all** — no
// harness override, no model var — which is precisely the case the old rule
// let through, and which under the old *gate* would not even have reached the
// peek.
#[test]
fn degraded_kind_peek_bails_even_with_nothing_configured() {
    let _g = support::lock_env(&ENV_LOCK);
    let err = degraded_peek_error(&[]);
    assert!(
        err.contains("could not be resolved"),
        "a degraded peek must refuse whether or not anything is configured — \
         every routing axis reads the leaf the peek could not read (err: {err})"
    );
    assert!(
        err.contains("model-per-task-kind"),
        "…and must say which rule it is enforcing, since with nothing set the \
         operator has no configuration to look at for a clue (err: {err})"
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
    // GROVE_REVIEW_IMPL_HARNESS — yet the typo there must still fail loudly right
    // away, not once a review leaf happens to be picked.
    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_REVIEW_IMPL_HARNESS", "lemur");

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "offkindgrove");

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("GROVE_REVIEW_IMPL_HARNESS") && err.contains("lemur"),
        "a typo in an off-kind override must fail at the very next launch, \
         not only once a review leaf is picked (err: {err})"
    );
}

// An unknown override value must fail loudly at launch — a typo'd harness
// name that silently fell back to the stamped harness would run reviews on
// the wrong (and possibly self-reviewing) model for a whole trial. The start
// path takes a shortcut straight to `Kind::Requirements` (fresh-grove-start-
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

    // Start path ⇒ kind is Requirements by construction; route it to a typo.
    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_REQUIREMENTS_HARNESS", "lemur");

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "typogrove");

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("GROVE_REQUIREMENTS_HARNESS") && err.contains("lemur"),
        "the error must name the variable and the bad value (err: {err})"
    );
    assert!(
        err.contains("claude") && err.contains("codex") && err.contains("pi"),
        "the error must list the known harnesses (err: {err})"
    );
}

// T3: the continue path's kind peek must honour the same
// unknown-override-fails-loudly contract as the start path above — that path
// short-circuits to `Kind::Requirements` and never calls `resolve_kind`
// (src/loop_driver.rs:279-281), so it cannot exercise `GROVE_REVIEW_IMPL_HARNESS`
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
        "# a-k1\n\n**Kind:** review-impl\n",
    )
    .unwrap();

    let skill_dir = worktree.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_REVIEW_IMPL_HARNESS", "lemur");

    let result = loop_driver::run_loop(harness, worktree, worktree, "typogrove2");

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("GROVE_REVIEW_IMPL_HARNESS") && err.contains("lemur"),
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_REQUIREMENTS_HARNESS", "")
        // Scaffolding: the leaf still has to resolve a model to launch.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

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
        "empty GROVE_REQUIREMENTS_HARNESS must stay on the stamped harness (log: {log:?})"
    );
}

// harness-spawn-preflight-k8: `do_grove`'s pre-flight used to validate only
// the stamped harness's binary, so `GROVE_REVIEW_IMPL_HARNESS=pi` against a
// codex-stamped grove with no `pi` installed sailed through pre-flight, ran
// for however long, and only died the moment a review leaf was finally
// picked. `preflight_check` must catch that up front — resolved through the
// same `GROVE_HARNESS_BIN_<NAME>` seam `harness_bin` uses for the real
// launch, here pointed at a path that plain does not exist (the leaf's own
// Notes ask for exactly this).
#[test]
fn preflight_check_catches_a_missing_per_kind_override_binary() {
    let _g = support::lock_env(&ENV_LOCK);
    let tmp = TempDir::new().unwrap();

    let fake_claude = tmp.path().join("fake-claude.sh");
    write_exec(&fake_claude, "#!/bin/sh\nexit 0\n");
    let missing_pi = tmp.path().join("no-such-pi");

    let stamped = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake_claude)
        .set("GROVE_HARNESS_BIN_PI", &missing_pi)
        .set("GROVE_REVIEW_IMPL_HARNESS", "pi");

    let err = loop_driver::preflight_check(stamped).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("GROVE_REVIEW_IMPL_HARNESS"),
        "diagnostic must name the override var (got: {msg:?})"
    );
    assert!(
        msg.contains(&missing_pi.display().to_string()),
        "diagnostic must name the missing binary (got: {msg:?})"
    );
}

// The stamped harness is still checked exactly as before — a missing
// per-kind override is an addition to pre-flight, not a replacement of its
// original job.
#[test]
fn preflight_check_still_catches_a_missing_stamped_binary() {
    let _g = support::lock_env(&ENV_LOCK);
    let tmp = TempDir::new().unwrap();
    let missing_claude = tmp.path().join("no-such-claude");

    let stamped = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &missing_claude);

    let err = loop_driver::preflight_check(stamped).unwrap_err();
    assert!(
        err.to_string()
            .contains(&missing_claude.display().to_string()),
        "diagnostic must name the missing stamped binary (got: {err})"
    );
}

// A configured override that resolves fine must not block pre-flight — only
// a genuinely missing binary should.
#[test]
fn preflight_check_passes_when_every_configured_harness_resolves() {
    let _g = support::lock_env(&ENV_LOCK);
    let tmp = TempDir::new().unwrap();

    let fake_claude = tmp.path().join("fake-claude.sh");
    write_exec(&fake_claude, "#!/bin/sh\nexit 0\n");
    let fake_pi = tmp.path().join("fake-pi.sh");
    write_exec(&fake_pi, "#!/bin/sh\nexit 0\n");

    let stamped = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake_claude)
        .set("GROVE_HARNESS_BIN_PI", &fake_pi)
        .set("GROVE_REVIEW_IMPL_HARNESS", "pi");

    loop_driver::preflight_check(stamped).unwrap();
}

// An unknown harness name in a per-kind override must fail loudly at
// pre-flight too, not just once `resolve_launch` hits it mid-loop — same
// typo-safety contract, just moved earlier.
#[test]
fn preflight_check_rejects_an_unknown_harness_override_name() {
    let _g = support::lock_env(&ENV_LOCK);
    let tmp = TempDir::new().unwrap();
    let fake_claude = tmp.path().join("fake-claude.sh");
    write_exec(&fake_claude, "#!/bin/sh\nexit 0\n");

    let stamped = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake_claude)
        .set("GROVE_PLANNING_HARNESS", "lemur");

    let err = loop_driver::preflight_check(stamped).unwrap_err();
    assert!(
        err.to_string().contains("unknown harness"),
        "an unknown per-kind override name must fail loudly (got: {err})"
    );
}

// The version-skew guard (driver-version-skew-k11). A long-running driver
// keeps executing the text segment it started with — `brew upgrade` replaces
// (or deletes) the binary on disk without touching it — while the agent's
// `grove-llm` is resolved through PATH afresh at every invocation. That skew
// silently splits the signal protocol's two halves: observed twice as a
// pre-watcher driver paired with a watcher-era `grove-llm`, every session
// hanging at its completion signal and nothing ever relaunching. The driver
// must notice the disagreement and stop *before* launching a session on the
// skewed pair.
#[test]
fn a_version_skewed_grove_llm_stops_the_loop_before_any_session() {
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
printf 'ran\n' >> "$GROVE_TEST_LOG"
exit 0
"#,
    );
    // A `grove-llm` whose version can never match this build's own.
    let skewed = repo_path.join("skewed-grove-llm.sh");
    write_exec(&skewed, "#!/bin/sh\necho 'grove-llm 99.0.0'\n");

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", &skewed)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_LOG", &log);

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "skewgrove");

    assert_eq!(
        result.unwrap(),
        LoopOutcome::Stopped,
        "a confirmed version skew stops the loop — resumable, not an error \
         (restart ≡ continuation)"
    );
    assert!(
        !log.exists(),
        "no session may launch on a skewed driver/grove-llm pair — the hang \
         this guards against happens *inside* such a session"
    );
}

// Constraint 5 (grove guides, it does not gate): only a successfully read,
// definitely different version may stop the loop. A `grove-llm` whose version
// cannot be read at all — missing binary, failing `--version`, unparseable
// output — must warn and carry on, never jam the unattended loop.
#[test]
fn an_unreadable_grove_llm_version_never_jams_the_loop() {
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
printf 'ran\n' >> "$GROVE_TEST_LOG"
exit 0
"#,
    );
    // `--version` fails outright: the check has nothing to compare.
    let broken = repo_path.join("broken-grove-llm.sh");
    write_exec(&broken, "#!/bin/sh\nexit 1\n");

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", &broken)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_LOG", &log)
        // Scaffolding: the start path resolves to requirements without consulting
        // `grove-llm` at all, so the session still launches — which is the
        // point of the test — and still needs a model to do so.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "unreadablegrove");

    assert_eq!(
        result.unwrap(),
        LoopOutcome::Stopped,
        "an unreadable version must leave the loop's own behaviour untouched \
         (one un-signalled session, then the normal stop)"
    );
    assert_eq!(
        fs::read_to_string(&log).unwrap(),
        "ran\n",
        "the session must still launch — an unreadable version degrades the \
         check, it never gates the loop"
    );
}

// The stop must say so *plainly* (the leaf's Done-when): both versions and
// the restart instruction, on the operator's own stderr. Drives the real
// `grove do` binary — the only way to observe what the operator actually
// sees — with the skew injected through the same `GROVE_LLM_BIN` seam.
#[test]
fn a_version_skew_stop_names_both_versions_and_how_to_restart() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    assert!(
        std::process::Command::new("git")
            .args(["init", "-q"])
            .current_dir(repo_path)
            .status()
            .unwrap()
            .success(),
        "git init failed"
    );
    // `.claude/` so the harness is detected; no `.grove/` — the guard fires
    // before any session, start path or continue path alike.
    fs::create_dir_all(repo_path.join(".claude")).unwrap();

    // Stamped skill dir, as in the sibling `grove do` subprocess test above:
    // provisioning re-extracts the embedded prompts, which is fine — this
    // test only asserts on stderr and on the harness never running.
    let skill_dir = repo_path.join("global-skill");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join(STAMP_FILE), "stale-hash").unwrap();

    let log = repo_path.join("log");
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
printf 'ran\n' >> "$GROVE_TEST_LOG"
exit 0
"#,
    );
    let skewed = repo_path.join("skewed-grove-llm.sh");
    write_exec(&skewed, "#!/bin/sh\necho 'grove-llm 99.0.0'\n");

    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_grove"));
    cmd.args(["do"]).current_dir(repo_path);
    for name in support::grove_env_names() {
        cmd.env_remove(name);
    }
    let out = cmd
        .env("GROVE_HARNESS_BIN", &fake)
        .env("GROVE_LLM_BIN", &skewed)
        .env("GROVE_SKILL_DIR", &skill_dir)
        .env("GROVE_TEST_LOG", &log)
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let own_version = env!("CARGO_PKG_VERSION");
    assert!(
        stderr.contains(own_version) && stderr.contains("99.0.0"),
        "the operator must see BOTH versions — the driver's own ({own_version}) \
         and the skewed grove-llm's (99.0.0) — or the stop is undiagnosable \
         (stderr: {stderr:?})"
    );
    assert!(
        stderr.contains("grove do"),
        "the stop must carry its own recovery: re-run `grove do` \
         (stderr: {stderr:?})"
    );
    assert!(
        !log.exists(),
        "the harness must never have launched (stderr: {stderr:?})"
    );
}

// codex-gitdir-grant: codex's `workspace-write` sandbox carves the repository
// gitdir out read-only, so `git commit` — and with it grove's mandatory
// Commit and Retire steps — fails inside a codex session. Every codex launch
// must grant the gitdir back via `--add-dir <git-common-dir>`. In a plain
// checkout `git rev-parse --git-common-dir` prints the *relative* `.git`, so
// this shape also proves the value is absolutized against the worktree
// rather than passed through raw.
#[test]
fn codex_launch_grants_the_gitdir_via_add_dir_in_a_plain_repo() {
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

    let log = worktree.join("log");
    let fake = worktree.join("fake-codex.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
printf '%s\n' "$*" >> "$GROVE_TEST_LOG"
exit 0
"#,
    );

    let harness = harness::by_name("codex").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_LOG", &log)
        // Scaffolding: one start-path (requirements) session.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

    let result = loop_driver::run_loop(harness, worktree, worktree, "gitdirgrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let argv = fs::read_to_string(&log).unwrap();
    let granted = support::add_dir_value(&argv)
        .unwrap_or_else(|| panic!("a codex launch must carry --add-dir <gitdir> (argv: {argv:?})"));
    let granted = std::path::Path::new(granted);
    assert!(
        granted.is_absolute(),
        "the granted gitdir must be absolutized, never the raw relative \
         `.git` git prints in a plain checkout (argv: {argv:?})"
    );
    assert_eq!(
        granted.canonicalize().unwrap(),
        worktree.join(".git").canonicalize().unwrap(),
        "a plain checkout's grant is its own `.git` dir (argv: {argv:?})"
    );
}

// The other repo shape (codex-gitdir-grant): a linked worktree's own gitdir
// (`<main>/.git/worktrees/<name>`) lives outside the workspace entirely, and
// is a subpath of the common dir — one grant of the absolutized
// `--git-common-dir` covers it. The derived path must be the MAIN repo's
// `.git`, not anything under the linked worktree itself.
#[test]
fn codex_launch_from_a_linked_worktree_grants_the_main_repos_gitdir() {
    let _g = support::lock_env(&ENV_LOCK);
    let tmp = TempDir::new().unwrap();
    let main = tmp.path().join("main");
    fs::create_dir_all(&main).unwrap();
    let git = |dir: &std::path::Path, args: &[&str]| {
        assert!(
            std::process::Command::new("git")
                .args(args)
                .current_dir(dir)
                .status()
                .unwrap()
                .success(),
            "git {args:?} failed"
        );
    };
    git(&main, &["init", "-q", "-b", "main"]);
    git(&main, &["config", "user.email", "t@example.com"]);
    git(&main, &["config", "user.name", "t"]);
    git(&main, &["commit", "-q", "--allow-empty", "-m", "init"]);
    git(&main, &["worktree", "add", "-q", "../wt", "-b", "feature"]);
    let worktree = tmp.path().join("wt");

    let skill_dir = worktree.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let log = worktree.join("log");
    let fake = worktree.join("fake-codex.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
printf '%s\n' "$*" >> "$GROVE_TEST_LOG"
exit 0
"#,
    );

    let harness = harness::by_name("codex").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_LOG", &log)
        // Scaffolding: one start-path (requirements) session.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

    let result = loop_driver::run_loop(harness, &worktree, &worktree, "linkedgrove");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let argv = fs::read_to_string(&log).unwrap();
    let granted = support::add_dir_value(&argv)
        .unwrap_or_else(|| panic!("a codex launch must carry --add-dir <gitdir> (argv: {argv:?})"));
    assert_eq!(
        std::path::Path::new(granted).canonicalize().unwrap(),
        main.join(".git").canonicalize().unwrap(),
        "a linked worktree's grant is the MAIN repo's common gitdir — the \
         worktree's own gitdir is a subpath of it (argv: {argv:?})"
    );
}

// The grant is codex-only: claude and pi launches must stay byte-identical
// to before. Run both against a real git repo — a gitdir genuinely exists
// that a harness-blind implementation would wrongly grant.
#[test]
fn claude_and_pi_launches_carry_no_add_dir() {
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

    let log = worktree.join("log");
    let fake = worktree.join("fake-harness.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
printf '%s\n' "$*" >> "$GROVE_TEST_LOG"
exit 0
"#,
    );

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_LOG", &log)
        // Scaffolding: one start-path (requirements) session per harness. The
        // unscoped var covers both, since neither launch is rerouted.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

    for name in ["claude", "pi"] {
        let harness = harness::by_name(name).unwrap();
        let result = loop_driver::run_loop(harness, worktree, worktree, "noaddgrove");
        assert_eq!(result.unwrap(), LoopOutcome::Stopped);
    }

    let argv = fs::read_to_string(&log).unwrap();
    let rows: Vec<&str> = argv.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(rows.len(), 2, "one launch per harness (log: {argv:?})");
    for row in &rows {
        assert!(
            !row.contains("--add-dir"),
            "the gitdir grant is codex-only — claude/pi launches must be \
             byte-identical to before (argv: {row:?})"
        );
    }
}

// herdr pane-state reporting (herdr-optional-ui, report-plumbing-k8). The unit
// tests in `src/herdr.rs` cover the state table and the transport separately;
// what neither can prove is the **wiring** — that the driver reaches the right
// report site at the right moment. So these drive the real loop against a fake
// herdr: a `UnixListener` this test owns, addressed through the same
// `HERDR_SOCKET_PATH`/`HERDR_PANE_ID` variables herdr itself puts in a pane.
//
// `support::grove_env_names` scrubs those three vars for every *other* test in
// this file, precisely so a `cargo test` run cannot report into the developer's
// own pane; these two set them back deliberately.

// The happy path, end to end: two tasks then a finish. Every launch reports
// `working`; a relaunch reports nothing of its own; `complete --done` reports
// `idle` and *then* releases, in that order.
#[test]
fn a_finishing_loop_reports_working_per_task_then_idle_and_releases() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    // A repo: task 2 takes the continue path, whose kind peek is no longer
    // skippable (see `init_worktree`).
    let worktree = repo_path.join("wt");
    init_worktree(&worktree);

    let sock = repo_path.join("herdr.sock");
    let herdr = support::fake_herdr(&sock);

    let counter = repo_path.join("counter");

    // Task 1 relaunches; task 2 runs the finish cycle (`done`).
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
mkdir -p "$PWD/.grove"
if [ "$n" -eq 1 ]; then
  : > "$GROVE_SIGNAL_FILE"
else
  printf 'done\n' > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_COUNTER", &counter)
        // Scaffolding: task 1 is the start path ⇒ requirements; task 2 peeks an
        // empty `.grove/` (the finish-cycle iteration, exempt by construction).
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL)
        .set("HERDR_ENV", "1")
        .set("HERDR_SOCKET_PATH", &sock)
        .set("HERDR_PANE_ID", "wQ:p1");

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "herdrgrove");
    assert_eq!(result.unwrap(), LoopOutcome::Finished);

    drop(env);
    let lines = herdr.lock().unwrap().clone();

    assert_eq!(
        support::reported(&lines),
        vec![
            ("pane.report_agent".into(), "working".into()),
            ("pane.report_agent".into(), "working".into()),
            ("pane.report_agent".into(), "idle".into()),
            ("pane.release_agent".into(), String::new()),
        ],
        "one `working` per launch (a relaunch adds nothing of its own), then \
         idle-then-release at the finish — release last, so a failed release \
         leaves the pane reading done rather than pinned at working \
         (lines: {lines:?})"
    );
}

// The headline case (root BRIEF.md): a loop that stops without a completion
// signal — `/exit`, Ctrl-C, or a crash — must report **`blocked`** and must
// **keep** its authority. Releasing here would hand the pane back to screen
// detection, which reads a parked grove as `idle`, which herdr surfaces as its
// derived `done`: the exact "stalled overnight, shows as finished" complaint
// this leaf exists to fix.
#[test]
fn a_loop_that_stops_without_a_signal_reports_blocked_and_holds_the_pane() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let worktree = repo_path.join("wt");
    fs::create_dir_all(&worktree).unwrap();

    let sock = repo_path.join("herdr.sock");
    let herdr = support::fake_herdr(&sock);

    // Never signals — stands in for `/exit`, a double Ctrl-C, or a crash.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        // Scaffolding: one start-path (requirements) session.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL)
        .set("HERDR_ENV", "1")
        .set("HERDR_SOCKET_PATH", &sock)
        .set("HERDR_PANE_ID", "wQ:p1");

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "herdrgrove");
    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    drop(env);
    let lines = herdr.lock().unwrap().clone();

    assert_eq!(
        support::reported(&lines),
        vec![
            ("pane.report_agent".into(), "working".into()),
            ("pane.report_agent".into(), "blocked".into()),
        ],
        "a parked loop reads `blocked`, and nothing releases it: the grove has \
         live leaves and genuinely needs a human (lines: {lines:?})"
    );
}

// herdr-optional-ui's load-bearing negative, at the loop level: with no herdr
// in the environment the driver must not so much as look for a socket, and the
// loop must behave exactly as it did before this feature existed.
#[test]
fn a_loop_with_no_herdr_in_the_environment_reports_nothing() {
    let _g = support::lock_env(&ENV_LOCK);
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let worktree = repo_path.join("wt");
    fs::create_dir_all(&worktree).unwrap();

    // A listener is bound, but the pane vars are scrubbed — so a driver that
    // reported regardless of the environment would still be caught here.
    let sock = repo_path.join("herdr.sock");
    let herdr = support::fake_herdr(&sock);

    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
printf 'done\n' > "$GROVE_SIGNAL_FILE"
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    let mut env = EnvGuard::new();
    // `clear_grove_env` scrubs the HERDR_* trio too — the point of the test.
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        // Scaffolding: one start-path (requirements) session.
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "herdrgrove");
    assert_eq!(
        result.unwrap(),
        LoopOutcome::Finished,
        "the loop is unaffected by herdr's absence"
    );

    drop(env);
    let lines = herdr.lock().unwrap().clone();

    assert!(
        support::reported(&lines).is_empty(),
        "with no HERDR_* pane environment grove must report nothing at all \
         (lines: {lines:?})"
    );
}

// Release-on-exit, the one genuinely new mechanism here: herdr never expires a
// hook authority, so a driver killed without releasing leaves the pane pinned
// at whatever grove last reported. Only `loop_driver::run` installs the
// SIGTERM/SIGHUP handler (`run_loop`, which every test above calls, deliberately
// does not — it must stay free of process-global signal changes), so this is
// the one case that has to drive the **real `grove do` binary** as a subprocess
// and signal it for real.
//
// Also the only test that exercises handler→poll-loop→release end to end: the
// handler itself may only flip an atomic (a socket round trip is not
// async-signal-safe), so the release happening at all depends on the watcher's
// poll loop noticing the flag and acting on it.
#[test]
fn a_sigtermed_driver_releases_the_pane_before_exiting() {
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

    // `.claude/` so the harness is detected; a `.grove/` so the loop takes the
    // continue path without needing to bootstrap.
    fs::create_dir_all(repo_path.join(".claude")).unwrap();
    fs::create_dir_all(repo_path.join(".grove")).unwrap();
    fs::write(repo_path.join(".grove/BRIEF.md"), "# g — brief\n").unwrap();
    fs::write(
        repo_path.join(".grove/01-a-k1.md"),
        "# a-k1\n\n**Kind:** impl\n",
    )
    .unwrap();
    git(&["add", "-A"]);
    git(&["commit", "-qm", "tree"]);

    // Live provisioning, as in the sibling subprocess test: stamp the dir so the
    // foreign-dir guard treats it as grove's own.
    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();
    fs::write(skill_dir.join(STAMP_FILE), "stale-hash").unwrap();

    let sock = repo_path.join("herdr.sock");
    let herdr = support::fake_herdr(&sock);

    // Never signals, never exits on its own — stands in for a session sitting
    // mid-task when the driver is killed from outside. `exec` so the pid the
    // driver signals is the sleeping process itself.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
exec sleep 60
"#,
    );

    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_grove"));
    cmd.arg("do").current_dir(repo_path);
    for name in support::grove_env_names() {
        cmd.env_remove(name);
    }
    let mut child = cmd
        .env("GROVE_HARNESS_BIN", &fake)
        .env("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .env("GROVE_SKILL_DIR", &skill_dir)
        // Scaffolding: the committed tree's live leaf is `impl`, and the
        // continue path now requires it to resolve a model.
        .env("GROVE_IMPL_MODEL", SCAFFOLD_MODEL)
        .env("GROVE_KILL_GRACE", "0.2")
        .env("GROVE_KILL_GRACE_KILL", "0.3")
        .env("HERDR_ENV", "1")
        .env("HERDR_SOCKET_PATH", &sock)
        .env("HERDR_PANE_ID", "wQ:p1")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    // Wait for the launch report, so the SIGTERM lands mid-session rather than
    // racing the driver's own startup.
    let deadline = Instant::now() + Duration::from_secs(20);
    while herdr.lock().unwrap().is_empty() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(50));
    }
    assert_eq!(
        support::reported(&herdr.lock().unwrap()),
        vec![("pane.report_agent".into(), "working".into())],
        "the driver must have reported `working` before being signalled"
    );

    unsafe { libc::kill(child.id() as i32, libc::SIGTERM) };

    let exited = Instant::now() + Duration::from_secs(20);
    loop {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        assert!(
            Instant::now() < exited,
            "a SIGTERM'd driver must stop, not hang"
        );
        std::thread::sleep(Duration::from_millis(50));
    }

    let lines = herdr.lock().unwrap().clone();
    assert_eq!(
        support::reported(&lines),
        vec![
            ("pane.report_agent".into(), "working".into()),
            ("pane.release_agent".into(), String::new()),
        ],
        "a torn-down driver hands the pane back and says nothing else — herdr \
         never expires an authority, so skipping this pins the pane at \
         `working` forever (lines: {lines:?})"
    );
}

// The turn-boundary hooks' *injection* (herdr-turn-boundary-hooks). The verb
// they call is driven end to end in `tests/report_turn.rs` and the payload's
// exact bytes are pinned in `src/launch.rs`; what only the real loop can prove
// is that the flag reaches the argv of the harness that actually launches, and
// — just as load-bearing — that it reaches nothing else.

/// Run one start-path iteration of the real loop against an argv-logging fake
/// harness, and return what it was launched with. `herdr` picks whether the
/// pane environment is present; the socket path deliberately points at nothing,
/// since these tests assert on the argv and a refused socket is a fast no-op.
fn launched_argv(harness_name: &str, herdr: bool) -> String {
    let worktree_dir = TempDir::new().unwrap();
    let worktree = worktree_dir.path();
    // A real repo, because a codex launch resolves the gitdir it grants back
    // (codex-gitdir-grant) before it ever gets as far as the turn hooks.
    init_worktree(worktree);

    let skill_dir = worktree.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();

    let log = worktree.join("log");
    let fake = worktree.join("fake-harness.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
printf '%s\n' "$*" >> "$GROVE_TEST_LOG"
exit 0
"#,
    );

    let mut env = EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake)
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_TEST_LOG", &log)
        .set("GROVE_REQUIREMENTS_MODEL", SCAFFOLD_MODEL);
    if herdr {
        env.set("HERDR_ENV", "1")
            .set("HERDR_SOCKET_PATH", worktree.join("nowhere.sock"))
            .set("HERDR_PANE_ID", "wQ:p1");
    }

    let result = loop_driver::run_loop(
        harness::by_name(harness_name).unwrap(),
        worktree,
        worktree,
        "hookgrove",
    );
    assert_eq!(result.unwrap(), LoopOutcome::Stopped);
    fs::read_to_string(&log).unwrap()
}

// Both boundaries, or the surface is worse than useless: `Stop` alone would
// report `blocked` when the agent asks and then never take it back down once
// the human answers.
#[test]
fn a_claude_launch_under_herdr_carries_both_turn_hooks() {
    let _g = support::lock_env(&ENV_LOCK);
    let argv = launched_argv("claude", true);
    assert!(
        argv.contains("--settings"),
        "a claude launch under herdr must inject the turn hooks (argv: {argv:?})"
    );
    for boundary in ["report-turn start", "report-turn end"] {
        assert!(
            argv.contains(boundary),
            "the injected settings must wire {boundary:?} (argv: {argv:?})"
        );
    }
}

// herdr-optional-ui's load-bearing negative, at its strongest: with no herdr in
// the pane environment there is no hook to fire, nothing to spawn, and no new
// surface to go wrong — the launch is byte-identical to a grove that never had
// turn hooks.
#[test]
fn a_claude_launch_outside_herdr_carries_no_turn_hooks() {
    let _g = support::lock_env(&ENV_LOCK);
    let argv = launched_argv("claude", false);
    assert!(
        !argv.contains("--settings") && !argv.contains("report-turn"),
        "absent herdr, the hooks must not be injected at all (argv: {argv:?})"
    );
}

// codex has no turn-end hook event, and pi has herdr's own full-lifecycle
// extension already reporting on the same events. Injecting a claude-shaped
// `--settings` into either would at best be ignored and at worst refuse the
// launch outright.
#[test]
fn codex_and_pi_launches_never_carry_turn_hooks() {
    let _g = support::lock_env(&ENV_LOCK);
    for harness in ["codex", "pi"] {
        let argv = launched_argv(harness, true);
        assert!(
            !argv.contains("--settings"),
            "the turn hooks are claude-shaped and claude-only ({harness} argv: {argv:?})"
        );
    }
}
