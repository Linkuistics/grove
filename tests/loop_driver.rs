// Integration tests for the self-driving loop (src/loop_driver.rs).
//
// Every test here drives the **real bare `grove` process** against an isolated
// `$HOME` carrying a complete `config.kdl` and a fake configured command. That
// is the only way in: there is no harness to detect, no binary override to
// point somewhere else, and no in-process entry point that skips the driver's
// signal handlers. What the process seam buys is exactly what these tests are
// about — the driver's own stderr, its session-epoch bookkeeping, its response
// to being signalled, and its ownership of one foreground child.
//
// The watcher's grace → SIGTERM → kill-grace → SIGKILL escalation is *not*
// tested here. Its two durations are built-in constants passed into
// `wait_with_watcher_result`, so the escalation is driven on test timescales
// through that module-local parameter, in `src/loop_driver.rs`'s own unit tests.
// Reaching it from out here would need a process-configuration knob, which is
// the thing this leaf removed.

mod support;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

// A few tests below still mutate process-global state (cwd-independent, but
// they read the ambient environment to build a scrubbed child env); serialize
// them so cargo's parallel runner cannot cross wires.
static ENV_LOCK: Mutex<()> = Mutex::new(());

/// This build's own agent CLI. The driver resolves `grove-llm` as the sibling of
/// its own executable, which `CARGO_BIN_EXE_grove` satisfies — so nothing here
/// has to point it anywhere. Fixtures that need to *call* the CLI themselves
/// bake this path into their script.
const OWN_GROVE_LLM: &str = env!("CARGO_BIN_EXE_grove-llm");

const SESSION_KINDS: &[&str] = &[
    "requirements",
    "review-requirements",
    "integrate-review-requirements",
    "design",
    "review-design",
    "integrate-review-design",
    "planning",
    "review-planning",
    "integrate-review-planning",
    "prototype",
    "review-prototype",
    "integrate-review-prototype",
    "impl",
    "review-impl",
    "integrate-review-impl",
    "research-a",
    "research-b",
    "combine-research",
    "finish",
];

fn init_worktree(dir: &Path) {
    fs::create_dir_all(dir).unwrap();
    assert!(
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(dir)
            .status()
            .unwrap()
            .success(),
        "git init failed"
    );
}

fn git(dir: &Path, args: &[&str]) {
    assert!(
        Command::new("git")
            .args(args)
            .current_dir(dir)
            .status()
            .unwrap()
            .success(),
        "git {args:?} failed"
    );
}

fn write_exec(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut perms = fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).unwrap();
}

fn shell_quote(path: &Path) -> String {
    let value = path.to_str().unwrap();
    assert!(!value.contains('\''), "test fixture path contains a quote");
    format!("'{value}'")
}

/// A complete personal config: one command template for every session kind, so
/// configuration validation passes whichever leaf the tree happens to select.
fn write_complete_config(home: &Path, command: &Path) {
    fs::create_dir_all(home.join(".codex")).unwrap();
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    let template = format!("{} '${{prompt}}'", shell_quote(command));
    let document = SESSION_KINDS
        .iter()
        .map(|kind| format!("{kind} {template:?}\n"))
        .collect::<String>();
    fs::write(config_dir.join("config.kdl"), document).unwrap();
}

/// Plant a minimal current-format tree with one live leaf.
fn plant_tree(worktree: &Path, leaf: &str) {
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# g — brief\n").unwrap();
    fs::write(grove.join(leaf), "# planted\n").unwrap();
}

/// The bare driver, with the whole legacy launch-policy environment scrubbed
/// out of the child. A `Command` inherits this process's ambient environment,
/// and this repo dogfoods Grove, so scrubbing is what makes the fixture the
/// only input.
fn grove_driver(worktree: &Path, home: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_grove"));
    command.current_dir(worktree);
    for name in support::grove_env_names() {
        command.env_remove(name);
    }
    command.env("HOME", home);
    command
}

fn run_driver(worktree: &Path, home: &Path) -> Output {
    grove_driver(worktree, home).output().unwrap()
}

fn wait_for(path: &Path, what: &str) {
    let deadline = Instant::now() + Duration::from_secs(20);
    while !path.exists() {
        assert!(Instant::now() < deadline, "timed out waiting for {what}");
        thread::sleep(Duration::from_millis(25));
    }
}

// The session epoch is what admits an agent's `grove-llm` calls, so its window
// has to be exactly the child's lifetime: active before the spawn (or the very
// first call the session makes is refused) and inactive after the reap (or a
// dead session's signal path stays admissible). Observed from *inside* the
// session, which is the only vantage point that can tell "active before spawn"
// from "active shortly after".
#[test]
fn the_driver_activates_immediately_before_spawn_and_invalidates_after_reap() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl-subject-k1.md");

    let observed_epoch = fixture.path().join("observed-epoch");
    let observed_signal = fixture.path().join("observed-signal");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\ncp \"$PWD/.git/grove/session.epoch\" {epoch}\nprintf '%s\\n' \"$GROVE_SIGNAL_FILE\" > {signal}\nexit 0\n",
            epoch = shell_quote(&observed_epoch),
            signal = shell_quote(&observed_signal),
        ),
    );
    write_complete_config(&home, &configured);

    let output = run_driver(&worktree, &home);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let during_launch = fs::read_to_string(&observed_epoch).unwrap();
    let signal_path = fs::read_to_string(&observed_signal).unwrap();
    assert!(
        during_launch.starts_with("state=active\n"),
        "{during_launch:?}"
    );
    let signal_hex = signal_path
        .trim_end()
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join("");
    assert!(
        during_launch.contains(&format!("signal-path-hex={signal_hex}\n")),
        "the live epoch must name this launch's own channel: {during_launch:?}"
    );
    let after_reap = fs::read_to_string(worktree.join(".git/grove/session.epoch")).unwrap();
    assert!(after_reap.starts_with("state=inactive\n"), "{after_reap:?}");
    assert!(!after_reap.contains("signal-path-hex="), "{after_reap:?}");
}

// Every `grove-llm` mutator takes the **exclusive** tree-access lock on the
// working-tree root, and the driver takes that same lock to select. If the
// driver held its guard across the launch window, the first `leaf-add` any
// session ran would block until the session holding it exited — the loop would
// deadlock on its own first task, and every grove would stall at Retire.
//
// A read cannot detect that: `pick` takes a *shared* lock, so a driver still
// holding one would admit it. Only a session-side **mutation** proves release,
// which is why this drives the two real grow/retire verbs rather than writing
// the files directly the way the mandate fixtures do.
//
// Two iterations, because release is only half the claim: the mutations must
// also be what the *next* pick sees. The session retires its own leaf and adds
// the successor, and the loop must then select that successor by name.
#[test]
fn a_session_mutates_the_tree_through_grove_llm_without_deadlocking_the_driver() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl-subject-k1.md");

    let mandates = fixture.path().join("mandates");
    let verbs = fixture.path().join("verb-output");
    let first_run = fixture.path().join("first-run");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\n\
             printf '%s\\n' \"$1\" >> {mandates}\n\
             if [ -e {first_run} ]; then exit 0; fi\n\
             : > {first_run}\n\
             {grove_llm} leaf-add . follow-up --kind design >> {verbs} 2>&1 || exit 91\n\
             {grove_llm} leaf-retire .grove/01-impl-subject-k1.md >> {verbs} 2>&1 || exit 92\n\
             printf 'relaunch\\n' > \"$GROVE_SIGNAL_FILE\"\n\
             exit 0\n",
            mandates = shell_quote(&mandates),
            verbs = shell_quote(&verbs),
            first_run = shell_quote(&first_run),
            grove_llm = shell_quote(Path::new(OWN_GROVE_LLM)),
        ),
    );
    write_complete_config(&home, &configured);

    // Bounded rather than `run_driver`, because the failure this test exists to
    // catch is a *hang*: a blocking wait would turn it into a stuck suite
    // instead of a named assertion.
    let mut child = grove_driver(&worktree, &home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!(
                "the driver never returned: a session-side `grove-llm` mutation blocked on a \
                 tree-access guard the driver still held across its launch window"
            );
        }
        thread::sleep(Duration::from_millis(50));
    };
    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(status.success(), "{stderr}");
    let verb_output = fs::read_to_string(&verbs).unwrap_or_default();
    assert!(
        worktree.join(".grove/01-DONE-impl-subject-k1.md").is_file(),
        "the session's own `leaf-retire` must have completed: {verb_output}"
    );
    assert!(
        worktree.join(".grove/02-design-follow-up-k2.md").is_file(),
        "the session's own `leaf-add` must have completed: {verb_output}"
    );
    // The lock blocks rather than failing fast, so "it completed" alone would
    // also hold for a guard released late. `acquire_worktree` announces every
    // wait, and a launch-window session must never see one.
    assert!(
        !verb_output.contains("waiting for active Grove tree operation"),
        "the driver must release its tree-access guard *before* launching, not \
         while the session waits on it: {verb_output}"
    );

    // The mandate is a multi-line prompt, so the handles are read out of it
    // rather than counted by line.
    let mandates = fs::read_to_string(&mandates).unwrap();
    let handles: Vec<&str> = mandates
        .match_indices("resolve and execute `")
        .map(|(at, marker)| {
            let rest = &mandates[at + marker.len()..];
            &rest[..rest.find('`').expect("unterminated mandate handle")]
        })
        .collect();
    assert_eq!(
        handles,
        ["subject-k1", "follow-up-k2"],
        "the next pick must see the tree the previous session mutated: {mandates:?}"
    );
}

// A `done` signal — the finish cycle's last teardown action — must end the loop
// exactly once, cleanly, and must not be confused with either a relaunch or the
// no-signal stop. The abandoned-channel housekeeping a replacement driver does
// on acquiring the lease rides along here, because its failure modes have to
// stay advisory: a channel it cannot clean is reported and stepped over, never
// a reason not to launch.
#[test]
fn a_done_signal_finishes_the_loop_once_and_housekeeping_stays_advisory() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl-subject-k1.md");

    let control_dir = worktree.join(".git/grove");
    fs::create_dir_all(&control_dir).unwrap();
    let abandoned_signal = control_dir.join("signal-00000000000000000000000000000000");
    let blocked_signal = control_dir.join("signal-11111111111111111111111111111111");
    let unrelated_control = control_dir.join("signal-not-a-grove-channel");
    fs::write(&abandoned_signal, "old completion\n").unwrap();
    fs::create_dir(&blocked_signal).unwrap();
    fs::write(&unrelated_control, "keep me\n").unwrap();

    let log = fixture.path().join("log");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\nprintf 'ran\\n' >> {log}\nprintf 'done\\n' > \"$GROVE_SIGNAL_FILE\"\nexit 0\n",
            log = shell_quote(&log),
        ),
    );
    write_complete_config(&home, &configured);

    let output = run_driver(&worktree, &home);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "abandoned-channel housekeeping must not prevent launch: {stderr}"
    );
    assert!(
        stderr.contains("could not clean every signal channel abandoned by a previous driver"),
        "an abandoned-channel cleanup failure must remain visible: {stderr}"
    );
    assert!(
        stderr.contains("grove finished — loop complete"),
        "a `done` signal must end the loop with a clean finish: {stderr}"
    );

    let log = fs::read_to_string(&log).unwrap();
    assert_eq!(
        log.lines().filter(|line| !line.is_empty()).count(),
        1,
        "the loop must run exactly once then finish — no relaunch (log: {log:?})"
    );
    assert!(
        !abandoned_signal.exists(),
        "a replacement driver must clean abandoned signal channels after acquiring the lease"
    );
    assert!(
        blocked_signal.is_dir(),
        "an unremovable abandoned channel must remain inert while the loop continues"
    );
    assert_eq!(
        fs::read_to_string(unrelated_control).unwrap(),
        "keep me\n",
        "cleanup must ignore names outside Grove's exact channel grammar"
    );
}

// Outcome precedence: the signal the session actually wrote decides the loop,
// and a later failure to tidy that channel away cannot retroactively change it.
// The session's own disposition is the authoritative fact; channel removal is
// housekeeping, and housekeeping reports rather than overrules.
#[test]
fn a_signal_removal_failure_does_not_override_a_done_disposition() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl-subject-k1.md");
    let control_dir = worktree.join(".git/grove");

    let signal_log = fixture.path().join("signal-log");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\nprintf 'done\\n' > \"$GROVE_SIGNAL_FILE\"\nprintf '%s\\n' \"$GROVE_SIGNAL_FILE\" > {log}\nchmod 0500 \"$(dirname \"$GROVE_SIGNAL_FILE\")\"\nexit 0\n",
            log = shell_quote(&signal_log),
        ),
    );
    write_complete_config(&home, &configured);

    let output = run_driver(&worktree, &home);

    // Restore before asserting, so a failing assertion cannot leave the fixture
    // undeletable.
    let mut permissions = fs::metadata(&control_dir).unwrap().permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(&control_dir, permissions).unwrap();
    let signal_path = PathBuf::from(fs::read_to_string(signal_log).unwrap().trim());
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "channel housekeeping must not make a clean finish fail: {stderr}"
    );
    assert!(
        stderr.contains("could not remove the interpreted foreground-session signal channel"),
        "the removal failure must remain visible: {stderr}"
    );
    assert!(
        stderr.contains("grove finished — loop complete"),
        "the interpreted done disposition must remain authoritative: {stderr}"
    );
    assert!(
        signal_path.exists(),
        "the fixture must force the removal failure whose outcome precedence this test exercises"
    );
}

// Signal-file identity (signal-file-identity-k6): two loops with the *same
// grove name* in *different worktrees* must not interfere, even running truly
// concurrently. The pre-fix path was derived from the name alone
// (`$TMPDIR/grove-loop-<name>.signal`), so two worktrees sharing a basename —
// generic names like "bugs"/"plan"/"docs" are the norm — collided on one file.
//
// The "attacker" signals `done` immediately then hangs; the "victim" never
// signals and outlives the attacker's whole kill sequence before exiting on its
// own. Pre-fix, the attacker's `done` would land in the file the victim's
// watcher was also polling, killing the victim early and reporting a phantom
// clean finish from content it never wrote.
#[test]
fn concurrent_loops_with_the_same_grove_name_in_different_worktrees_do_not_interfere() {
    let fixture = TempDir::new().unwrap();

    let setup = |role: &str, body: &str| {
        let home = fixture.path().join(format!("{role}-home"));
        // Same *basename* in both trees — that is the whole point.
        let worktree = fixture.path().join(role).join("samegrove");
        init_worktree(&worktree);
        plant_tree(&worktree, "01-impl-subject-k1.md");
        let configured = fixture.path().join(format!("{role}-command.sh"));
        write_exec(&configured, body);
        write_complete_config(&home, &configured);
        (home, worktree)
    };

    let (attacker_home, attacker_tree) = setup(
        "attacker",
        "#!/bin/sh\nprintf 'done\\n' > \"$GROVE_SIGNAL_FILE\"\nexec sleep 30\n",
    );
    let (victim_home, victim_tree) = setup("victim", "#!/bin/sh\nsleep 1.5\nexit 0\n");

    let started = Instant::now();
    let attacker = grove_driver(&attacker_tree, &attacker_home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let victim = grove_driver(&victim_tree, &victim_home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let attacker_out = attacker.wait_with_output().unwrap();
    let victim_out = victim.wait_with_output().unwrap();
    let elapsed = started.elapsed();

    let attacker_stderr = String::from_utf8_lossy(&attacker_out.stderr);
    let victim_stderr = String::from_utf8_lossy(&victim_out.stderr);
    assert!(
        attacker_stderr.contains("grove finished — loop complete"),
        "sanity check: the attacker's own `done` signal still ends its own loop: {attacker_stderr}"
    );
    assert!(
        victim_stderr.contains("without a completion signal"),
        "the victim's session ended without ever signalling anything of its own — a \
         foreign `done` from the other worktree's loop must not be mistaken for its \
         own completion signal: {victim_stderr}"
    );
    assert!(
        elapsed >= Duration::from_millis(1200),
        "the victim must run its full ~1.5s session, not be cut short by the \
         attacker's kill sequence (elapsed: {elapsed:?})"
    );
}

// The real binary installs the SIGTERM/SIGHUP handler that forwards termination
// to a live child and reaps it through the ordinary watcher escalation. A
// driver that merely died would orphan an interactive session onto the TTY.
#[test]
fn a_sigtermed_driver_stops_and_reaps_its_child() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl-subject-k1.md");

    // Never signals, never exits on its own — a session sitting mid-task when
    // the driver is killed from outside. `exec` so the pid the driver signals
    // is the sleeping process itself.
    let launched = fixture.path().join("launched");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\n: > {launched}\nexec sleep 60\n",
            launched = shell_quote(&launched),
        ),
    );
    write_complete_config(&home, &configured);

    let mut child = grove_driver(&worktree, &home)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    // Wait for the child marker so SIGTERM lands mid-session rather than racing
    // the driver's own startup.
    wait_for(&launched, "the configured session to launch");

    unsafe { libc::kill(child.id() as i32, libc::SIGTERM) };

    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "a SIGTERM'd driver must stop, not hang"
        );
        thread::sleep(Duration::from_millis(50));
    }
}

// A leaf whose filename kind this binary does not know is a tree the driver
// cannot index into the configuration, so it refuses before launching anything
// rather than guessing a kind. The refusal names the offending file, because
// the fix is an edit to that filename.
#[test]
fn an_unrecognised_filename_kind_refuses_to_launch() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    git(&worktree, &["config", "user.email", "t@example.com"]);
    git(&worktree, &["config", "user.name", "t"]);
    plant_tree(&worktree, "01-reserch-a-k1.md");
    git(&worktree, &["add", "-A"]);
    git(
        &worktree,
        &["commit", "-qm", "tree with an unrecognised kind"],
    );

    let log = fixture.path().join("log");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\nprintf 'ran\\n' >> {log}\nexit 0\n",
            log = shell_quote(&log)
        ),
    );
    write_complete_config(&home, &configured);

    let output = run_driver(&worktree, &home);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success(), "unexpected success: {stderr}");
    assert!(
        stderr.contains("01-reserch-a-k1.md"),
        "the refusal must name the offending filename: {stderr}"
    );
    assert!(
        !log.exists(),
        "nothing may be launched for a leaf whose kind cannot be resolved"
    );
}

// An orphaned agent command that was admitted under the live epoch, then blocked
// on the tree lock, holds shared epoch admission after its foreground parent is
// gone. The driver must not park the loop waiting for it: its post-reap
// invalidation is bounded, and it stops with a visible diagnostic while leaving
// the completion signal deliberately unconsumed — a relaunch on a tree some
// orphan is still mutating is the outcome the bound exists to prevent.
//
// Slow by construction (~40s): the fixed invalidation timeout is the subject.
#[test]
fn an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal() {
    let _lock = support::lock_env(&ENV_LOCK);
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl-subject-k1.md");

    let launch_ready = fixture.path().join("launch-ready");
    let tree_locked = fixture.path().join("tree-locked");
    let orphan_started = fixture.path().join("orphan-started");
    let epoch_held = fixture.path().join("epoch-held");
    let orphan_done = fixture.path().join("orphan-done");
    let orphan_stderr = fixture.path().join("orphan-stderr");
    let term_received = fixture.path().join("term-received");
    let session_pid = fixture.path().join("session-pid");
    let observed_signal = fixture.path().join("observed-signal");
    let launch_count = fixture.path().join("launch-count");

    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            r#"#!/bin/sh
n=$(cat {count} 2>/dev/null || echo 0)
n=$((n + 1))
printf '%s\n' "$n" > {count}
printf '%s\n' "$GROVE_SIGNAL_FILE" > {observed}
printf '%s\n' "$$" > {pid}
: > {ready}
while [ ! -e {locked} ]; do sleep 0.01; done
(
  {llm} pick >/dev/null 2>{orphan_err}
  : > {orphan_done}
) >/dev/null 2>{orphan_err} &
: > {orphan_started}
while [ ! -e {held} ]; do sleep 0.01; done
: > "$GROVE_SIGNAL_FILE"
trap ': > {term}' TERM
while :; do sleep 0.1; done
"#,
            count = shell_quote(&launch_count),
            observed = shell_quote(&observed_signal),
            pid = shell_quote(&session_pid),
            ready = shell_quote(&launch_ready),
            locked = shell_quote(&tree_locked),
            llm = shell_quote(Path::new(OWN_GROVE_LLM)),
            orphan_err = shell_quote(&orphan_stderr),
            orphan_done = shell_quote(&orphan_done),
            orphan_started = shell_quote(&orphan_started),
            held = shell_quote(&epoch_held),
            term = shell_quote(&term_received),
        ),
    );
    write_complete_config(&home, &configured);

    let (release_tx, release_rx) = mpsc::channel();
    let lock_worktree = worktree.clone();
    let lock_launch_ready = launch_ready.clone();
    let lock_tree_locked = tree_locked.clone();
    let lock_orphan_started = orphan_started.clone();
    let lock_epoch_held = epoch_held.clone();
    let lock_thread = thread::spawn(move || {
        use std::os::fd::AsRawFd;
        let deadline = Instant::now() + Duration::from_secs(20);
        while !lock_launch_ready.exists() {
            assert!(
                Instant::now() < deadline,
                "configured session did not start"
            );
            thread::sleep(Duration::from_millis(10));
        }

        let tree_guard = fs::File::open(&lock_worktree).unwrap();
        assert_eq!(
            unsafe { libc::flock(tree_guard.as_raw_fd(), libc::LOCK_EX) },
            0,
            "locking the worktree failed"
        );
        fs::write(&lock_tree_locked, "").unwrap();

        while !lock_orphan_started.exists() {
            assert!(
                Instant::now() < deadline,
                "orphaned tree command did not start"
            );
            thread::sleep(Duration::from_millis(10));
        }
        let epoch_path = lock_worktree.join(".git/grove/session.epoch");
        loop {
            assert!(
                Instant::now() < deadline,
                "orphaned tree command never acquired shared epoch admission"
            );
            let probe = fs::File::open(&epoch_path).unwrap();
            let result = unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
            if result != 0 {
                let error = std::io::Error::last_os_error();
                assert!(
                    matches!(
                        error.raw_os_error(),
                        Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN
                    ),
                    "probing shared epoch admission failed unexpectedly: {error}"
                );
                break;
            }
            assert_eq!(
                unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_UN) },
                0,
                "releasing the epoch probe failed"
            );
            thread::sleep(Duration::from_millis(10));
        }
        fs::write(&lock_epoch_held, "").unwrap();
        release_rx.recv().unwrap();
        drop(tree_guard);
    });

    let started = Instant::now();
    let mut child = grove_driver(&worktree, &home)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let setup_deadline = Instant::now() + Duration::from_secs(25);
    while !epoch_held.exists() && Instant::now() < setup_deadline {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    if !epoch_held.exists() {
        if child.try_wait().unwrap().is_none() {
            child.kill().unwrap();
        }
        let output = child.wait_with_output().unwrap();
        let _ = release_tx.send(());
        let _ = lock_thread.join();
        panic!(
            "orphan contention fixture did not reach shared epoch admission: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let contention_observed = Instant::now();
    // The built-in grace (2s) plus kill-grace (5s) precede the fixed 30s
    // invalidation timeout, so the process-level bound is the timeout plus that
    // escalation plus slack — not the timeout alone.
    let stop_deadline = contention_observed + Duration::from_secs(60);
    let stopped_within_bound = loop {
        if child.try_wait().unwrap().is_some() {
            break true;
        }
        if Instant::now() >= stop_deadline {
            child.kill().unwrap();
            break false;
        }
        thread::sleep(Duration::from_millis(25));
    };
    let output = child.wait_with_output().unwrap();
    let total_elapsed = started.elapsed();
    let contention_elapsed = contention_observed.elapsed();
    let orphan_outlived_killed_parent = !orphan_done.exists();
    let signal_path = fs::read_to_string(&observed_signal)
        .ok()
        .map(|path| PathBuf::from(path.trim()));
    let signal_was_left_unconsumed = signal_path.as_ref().is_some_and(|path| path.exists());

    let _ = release_tx.send(());
    lock_thread.join().unwrap();
    let orphan_deadline = Instant::now() + Duration::from_secs(10);
    while !orphan_done.exists() {
        assert!(
            Instant::now() < orphan_deadline,
            "orphaned tree command did not finish after the tree lock was released: {}",
            fs::read_to_string(&orphan_stderr).unwrap_or_default()
        );
        thread::sleep(Duration::from_millis(10));
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stopped_within_bound,
        "driver exceeded its watchdog after contention began: {stderr}"
    );
    assert!(
        !output.status.success(),
        "epoch handoff must fail: {stderr}"
    );
    assert!(
        stderr.contains(
            "post-reap session epoch invalidation blocked; completion signal left unconsumed"
        ),
        "unexpected error: {stderr}"
    );
    assert!(
        stderr.contains("timed out after 30s waiting for exclusive session epoch lock"),
        "unexpected error: {stderr}"
    );
    let contention_report = "waiting for exclusive session epoch lock for post-reap invalidation";
    assert_eq!(
        stderr
            .lines()
            .filter(|line| *line == contention_report)
            .count(),
        1,
        "real contention must be reported exactly once: {stderr}"
    );
    assert!(
        total_elapsed >= Duration::from_secs(30),
        "the fixed timeout fired too early: {total_elapsed:?}"
    );
    assert!(
        contention_elapsed < Duration::from_secs(60),
        "the fixed 30s timeout exceeded its process-level bound: {contention_elapsed:?}"
    );
    assert!(
        term_received.exists(),
        "the foreground parent must receive SIGTERM before forced death"
    );
    assert!(
        orphan_outlived_killed_parent,
        "the admitted background command must still be blocked after its foreground parent died"
    );
    let session_pid: i32 = fs::read_to_string(session_pid)
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    assert_eq!(unsafe { libc::kill(session_pid, 0) }, -1);
    assert_eq!(
        std::io::Error::last_os_error().raw_os_error(),
        Some(libc::ESRCH),
        "the TERM-ignoring foreground parent must have been SIGKILLed"
    );
    assert!(signal_was_left_unconsumed);
    assert_eq!(fs::read_to_string(&launch_count).unwrap().trim(), "1");
}
