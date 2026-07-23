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
        .set("GROVE_KILL_GRACE_KILL", "0.2");

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
        .set("GROVE_KILL_GRACE_KILL", "0.3");

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

    let worktree = repo_path.join(".grove-worktrees/killgrove2");
    fs::create_dir_all(&worktree).unwrap();

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
        .set("GROVE_KILL_GRACE_KILL", "0.3");

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
        .set("GROVE_KILL_GRACE_KILL", "0.2");

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
        .set("GROVE_KILL_GRACE_KILL", "0.3");

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
        .set("GROVE_KILL_GRACE_KILL", "0.3");

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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .env("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
  printf '# a-k1\n\n**Kind:** work\n' > "$PWD/.grove/01-a-k1.md"
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
        .set("GROVE_WORK_MODEL", "moonshot/k3");

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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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
        .set("GROVE_LLM_BIN", OWN_GROVE_LLM)
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

// harness-spawn-preflight-k8: `do_grove`'s pre-flight used to validate only
// the stamped harness's binary, so `GROVE_REVIEW_HARNESS=pi` against a
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
        .set("GROVE_REVIEW_HARNESS", "pi");

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
        .set("GROVE_TEST_LOG", &log);

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
        .set("GROVE_TEST_LOG", &log);

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
        .set("GROVE_TEST_LOG", &log);

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
        .set("GROVE_TEST_LOG", &log);

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
