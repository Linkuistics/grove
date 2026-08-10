// The self-driving loop — grove's runtime (self-driving-loop).
//
// Bare `grove` drives the *whole loop*, not one task: it launches a fresh
// foreground session per grove task and relaunches with fresh context each time
// the agent fires the completion signal (`grove-llm complete`). Any other exit —
// human `/exit`/Ctrl-C, or a crash — stops the loop, resumable later by
// re-running `grove` from the same working tree (restart ≡ continuation, the
// loop body holds zero state and re-derives position from the tree).
//
// The driver spawns the configured command directly — no shell, no PID-export
// trick; the spawned `Child` already carries its own pid — and watches it while
// it runs: poll `try_wait` alongside the completion-signal file, and once the
// file appears, apply grace → SIGTERM → kill-grace → SIGKILL to the child itself
// (driver-side watcher — self-driving-loop). The driver is the session's own
// parent process, outside whatever sandbox the session runs under, so it can
// always signal its child — unlike the in-agent self-kill this replaces, which
// codex's Seatbelt sandbox silently denied.
//
// The driver is deliberately tiny — a plain shell `while` loop could stand in
// (constraint 6, walk-away-able). Nothing below infers anything about the
// session: the selected leaf's filename kind indexes one complete-config entry,
// and that entry's argv is the launch in full.
//
//     # after owning the workspace lease, clean abandoned signal-<128-bit> paths
//     while :; do
//       v=$(grove-llm --version | awk '{print $NF}')     # version-skew guard
//       [ "$v" = "<own compiled-in version>" ] || exit 1
//       grove_recover_or_migrate_tree                    # driver-only transition
//       # One in-process selection: the leaf's stable handle *and* its kind.
//       read -r handle kind <<<"$(grove_select_or_materialize_finish)"
//       # The kind indexes the config; there is no default, family, or fallback.
//       argv=$(kdl_lookup "$HOME/.config/grove/config.kdl" "$kind")
//       # Draw a fresh OS-random 128-bit suffix in the workspace control dir;
//       # retry occupied names without touching their contents.
//       sig="$workspace_control/signal-<fresh-128-bit-suffix>"
//       GROVE_SIGNAL_FILE="$sig" $argv &                 # ${prompt} carries $handle
//       pid=$!
//       # poll $pid (try_wait) and "$sig" every ~500ms; on signal appearing:
//       # sleep 2, kill -TERM $pid, sleep 5, kill -KILL $pid
//       wait "$pid"
//       stty sane 2>/dev/null
//       disposition=$(read_signal "$sig")
//       rm -f "$sig"                  # only this launch's accepted channel
//       [ -n "$disposition" ] || break # no completion signal → stop
//     done

use crate::complete::{self, Disposition};
use crate::driver_lease::DriverLease;
use crate::session_config::{ExpansionContext, SessionConfig};
use anyhow::{Context, Result};
use std::ffi::OsString;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus};
use std::time::{Duration, Instant};

/// Why the loop stopped — the loop's terminal disposition, made first-class so
/// a clean whole-grove finish is distinguishable from an abnormal stop (rather
/// than both looking like "the loop just ended").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopOutcome {
    /// The grove finished cleanly: a session signalled `complete --done`.
    Finished,
    /// A non-signalled exit stopped the loop (human `/exit`/Ctrl-C, or a
    /// crash), or a pre-launch guard declined to start the next session (a
    /// version-skewed `grove-llm` — driver-version-skew-k11); resumable by
    /// re-running `grove` from the same working tree.
    Stopped,
}

/// Entry point for the bare config-driven lifecycle.
pub fn run_configured(
    repo_path: &Path,
    worktree: &Path,
    name: &str,
    driver_lease: DriverLease,
) -> Result<()> {
    ignore_interrupts();
    install_termination_handler();
    run_configured_loop_with_lease(repo_path, worktree, name, &driver_lease).map(|_| ())
}

fn run_configured_loop_with_lease(
    repo_path: &Path,
    worktree: &Path,
    name: &str,
    driver_lease: &DriverLease,
) -> Result<LoopOutcome> {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .context("$HOME is not set; cannot locate ~/.config/grove/config.kdl")?;
    let config_path = SessionConfig::path(&home);
    let repo_name = repo_path
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "repo".to_string());
    let session_name = format!("{repo_name}: {name} grove");

    loop {
        driver_lease
            .revalidate()
            .context("revalidating driver lease before loop transition")?;
        let _grove_llm = checked_grove_llm()?;
        let _pre_transition_config = SessionConfig::load(&home)?;

        crate::tree_lifecycle::transition_driver_to_current(worktree)?;
        let selection = match crate::tree_read::select(&worktree.join(".grove"))? {
            Some(selection) => selection,
            None => crate::tree_lifecycle::materialize_finish(worktree)?,
        };

        let config = SessionConfig::load(&home)?;
        let prompt = mandate_prompt(&selection.handle)?;
        let argv = config.expand(
            selection.kind.label(),
            &ExpansionContext {
                prompt: &prompt,
                session_name: &session_name,
                worktree,
                repository: repo_path,
            },
        )?;

        driver_lease
            .revalidate()
            .context("revalidating driver lease before foreground launch")?;
        let signal_channel = driver_lease
            .allocate_signal_channel()
            .context("allocating a fresh foreground-session signal channel")?;
        let ended = launch_configured_session(
            &argv,
            &selection,
            &config_path,
            worktree,
            signal_channel.path(),
            driver_lease,
        );
        let (ended, signal) = complete_post_reap_epoch_handoff(
            ended,
            || driver_lease.invalidate_session_epoch(),
            |ended| {
                reset_terminal();
                (ended, complete::read_signal(signal_channel.path()))
            },
        )?;

        if let Err(error) = driver_lease.remove_signal_channel(signal_channel) {
            eprintln!(
                "grove: warning: could not remove the interpreted foreground-session signal channel; preserving the session outcome: {error:#}"
            );
        }

        if ended.end == SessionEnd::Interrupted {
            eprintln!("grove: interrupted — stopping the loop.");
            return Ok(LoopOutcome::Stopped);
        }

        match signal {
            Some(Disposition::Relaunch) => continue,
            Some(Disposition::Done) => {
                eprintln!("grove: grove finished — loop complete.");
                return Ok(LoopOutcome::Finished);
            }
            None => {
                eprintln!(
                    "grove: session ended without a completion signal — status {}, elapsed {:.3}s; loop stopped.",
                    ended.status,
                    ended.elapsed.as_secs_f64()
                );
                if !ended.status.success() {
                    eprintln!(
                        "       configured session kind `{}` failed via {:?} from {}.",
                        selection.kind.label(),
                        argv[0],
                        config_path.display()
                    );
                }
                return Ok(LoopOutcome::Stopped);
            }
        }
    }
}

fn mandate_prompt(handle: &str) -> Result<String> {
    let launcher = crate::provision::continue_prompt()?;
    Ok(format!(
        "{launcher}\n\nGrove mandate: resolve and execute `{handle}`. This selected handle is authoritative; do not call `grove-llm pick` in this session.\n"
    ))
}

/// Launch one fresh foreground session owning the real TTY, then watch it while
/// it runs (see [`wait_with_watcher_result`]). Spawned directly — no shell, no
/// PID-export trick; the `Child` already carries its own pid, and the driver
/// signals it directly once the completion file appears.
///
/// The argv is taken whole from the expanded configuration. Nothing is appended,
/// injected, or reordered here: no session-name argument, no model flag, no
/// sandbox grant. A target that needs any of those spells them out in its own
/// command template, where the configuration owner can see them.
///
/// Prints one diagnostic line naming the kind, the executable and the selected
/// handle. That line is the only durable record of what each session in a loop
/// was working on, so it names the **stable handle** rather than a path, which
/// moves under `leaf-insert`.
fn launch_configured_session(
    argv: &[OsString],
    selection: &crate::tree_read::SelectedLeaf,
    config_path: &Path,
    worktree: &Path,
    signal_file: &Path,
    driver_lease: &DriverLease,
) -> Result<WatchedSession> {
    let (executable, arguments) = argv
        .split_first()
        .context("validated Grove command expanded to an empty argv")?;
    eprintln!(
        "grove: launching {} with configured {:?} — {}",
        selection.kind.label(),
        executable,
        selection.handle
    );
    let mut command = Command::new(executable);
    command.args(arguments).current_dir(worktree);
    crate::launch::scrub_loop_control_env(&mut command);
    command.env("GROVE_SIGNAL_FILE", signal_file);

    driver_lease
        .activate_session_epoch(signal_file)
        .context("activating the foreground session epoch before spawn")?;
    let child = command.spawn().with_context(|| {
        format!(
            "launching configured session kind `{}` via {:?} from {}",
            selection.kind.label(),
            executable,
            config_path.display()
        )
    })?;
    wait_with_watcher_result(child, signal_file, (DEFAULT_GRACE, DEFAULT_KILL_GRACE))
}

fn complete_post_reap_epoch_handoff<E, T>(
    ended: Result<E>,
    invalidate: impl FnOnce() -> Result<()>,
    continue_after_invalidation: impl FnOnce(E) -> T,
) -> Result<T> {
    const INVALIDATION_CONTEXT: &str =
        "post-reap session epoch invalidation blocked; completion signal left unconsumed";

    match (ended, invalidate()) {
        (Ok(ended), Ok(())) => Ok(continue_after_invalidation(ended)),
        (Err(launch_error), Ok(())) => Err(launch_error),
        (Ok(_), Err(invalidation_error)) => Err(invalidation_error.context(INVALIDATION_CONTEXT)),
        (Err(launch_error), Err(invalidation_error)) => Err(invalidation_error.context(format!(
            "{INVALIDATION_CONTEXT}; foreground session also failed: {launch_error:#}"
        ))),
    }
}

/// The watcher's state machine: idle until the completion signal file
/// appears, then timed toward SIGTERM and finally SIGKILL.
enum Watch {
    Running,
    Signalled(Instant),
    Terminated(Instant),
}

/// How a watched session ended: normally (whatever it left in the signal file
/// decides the rest), or because the *driver* was signalled — a case the signal
/// file cannot express, since an interrupt normally leaves none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SessionEnd {
    Exited,
    Interrupted,
}

struct WatchedSession {
    end: SessionEnd,
    status: ExitStatus,
    elapsed: Duration,
}

/// How often the watcher checks the child's liveness and the signal file.
const POLL_INTERVAL: Duration = Duration::from_millis(500);

/// How long after the completion signal file appears before the watcher sends
/// SIGTERM (lets the agent's `complete` tool call return, and the agent's turn
/// end, before its session dies).
///
/// A built-in constant, not a knob. It is passed into [`wait_with_watcher_result`]
/// rather than read there so the suite can drive the escalation on test
/// timescales through that module-local parameter — an internal seam, never
/// process configuration.
const DEFAULT_GRACE: Duration = Duration::from_secs(2);
/// How long after SIGTERM before the watcher escalates to SIGKILL. Same
/// built-in-constant rule as [`DEFAULT_GRACE`].
const DEFAULT_KILL_GRACE: Duration = Duration::from_secs(5);

/// Watch a spawned session while it runs: poll for the completion signal file
/// alongside the child's own exit (`try_wait`), and once the file appears, apply
/// grace → SIGTERM → kill-grace → SIGKILL — the out-of-band kill an interactive
/// session cannot perform on itself (self-driving-loop). This is the *driver's*
/// job, not the agent's: the driver is the session's own parent process, outside
/// whatever sandbox the session runs under, so it can always signal its child —
/// codex's Seatbelt sandbox, for one, denies a same-sandbox process from
/// signalling its own session, which is why the previous in-agent self-kill
/// silently failed there.
///
/// A caught SIGTERM/SIGHUP also lands here: the handler only flips
/// [`TERMINATED`], and this poll loop is what acts on it — forwarding the
/// signal to the child and letting the existing escalation reap it. That
/// ordering is deliberate: the handler performs only an async-signal-safe
/// atomic store, while the watcher signals and reaps the child on a normal
/// stack.
fn wait_with_watcher_result(
    mut child: Child,
    signal_file: &Path,
    (grace, kill_grace): (Duration, Duration),
) -> Result<WatchedSession> {
    let started = Instant::now();
    let mut watch = Watch::Running;
    let mut interrupted = false;
    loop {
        if let Some(status) = child.try_wait().context("waiting on the session")? {
            // A completion kill makes the session exit non-zero (or via
            // signal); that is the normal exit path, not an error. The signal
            // file — not the exit status — decides relaunch.
            return Ok(WatchedSession {
                end: if interrupted {
                    SessionEnd::Interrupted
                } else {
                    SessionEnd::Exited
                },
                status,
                elapsed: started.elapsed(),
            });
        }
        // A signalled driver forwards the signal to its child and hands over to
        // the same escalation the completion path uses, so a session that
        // ignores SIGTERM still gets SIGKILL'd rather than orphaned onto the
        // TTY. Latched, because the driver must still return `Interrupted` on
        // the iteration where the child finally exits.
        if !interrupted && TERMINATED.load(std::sync::atomic::Ordering::Relaxed) {
            interrupted = true;
            unsafe { libc::kill(child.id() as i32, libc::SIGTERM) };
            watch = Watch::Terminated(Instant::now());
        }
        watch = match watch {
            Watch::Running if signal_file.exists() => Watch::Signalled(Instant::now()),
            Watch::Signalled(at) if at.elapsed() >= grace => {
                // SAFETY: kill(2) on a pid this process itself spawned. A
                // failure (e.g. ESRCH, the child already exited) is ignored,
                // same as the shell `kill ... 2>/dev/null` this replaces —
                // the next `try_wait` above will catch that exit anyway.
                unsafe { libc::kill(child.id() as i32, libc::SIGTERM) };
                Watch::Terminated(Instant::now())
            }
            Watch::Terminated(at) if at.elapsed() >= kill_grace => {
                unsafe { libc::kill(child.id() as i32, libc::SIGKILL) };
                let status = child.wait().context("reaping the killed session")?;
                return Ok(WatchedSession {
                    end: if interrupted {
                        SessionEnd::Interrupted
                    } else {
                        SessionEnd::Exited
                    },
                    status,
                    elapsed: started.elapsed(),
                });
            }
            other => other,
        };
        std::thread::sleep(POLL_INTERVAL);
    }
}

/// Resolve the exact agent-side binary used by a configured session and reject
/// missing, malformed, or skewed versions before configuration or tree access.
///
/// The sibling of the running executable wins over `PATH` (`grove` and
/// `grove-llm` install together), and there is no override: a variable that
/// re-pointed the agent's own CLI would be launch policy by another name.
fn checked_grove_llm() -> Result<OsString> {
    let binary = if let Ok(executable) = std::env::current_exe() {
        match executable.parent().map(|parent| parent.join("grove-llm")) {
            Some(sibling) if sibling.is_file() => sibling.into_os_string(),
            _ => OsString::from("grove-llm"),
        }
    } else {
        OsString::from("grove-llm")
    };
    let display = binary.to_string_lossy();
    let output = Command::new(&binary)
        .arg("--version")
        .output()
        .with_context(|| format!("could not run `{display} --version`"))?;
    if !output.status.success() {
        anyhow::bail!("`{display} --version` failed ({})", output.status);
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let version = parse_checked_version(&stdout).with_context(|| {
        format!(
            "unrecognised `{display} --version` output {:?}",
            stdout.trim()
        )
    })?;
    if version != DRIVER_VERSION {
        anyhow::bail!(
            "grove/grove-llm version skew: driver is {DRIVER_VERSION}, `{display}` is {version}"
        );
    }
    Ok(binary)
}

fn parse_checked_version(stdout: &str) -> Option<&str> {
    let mut lines = stdout.lines();
    let mut words = lines.next()?.split_whitespace();
    if words.next()? != "grove-llm" {
        return None;
    }
    let version = words.next()?;
    if words.next().is_some()
        || !version.starts_with(|character: char| character.is_ascii_digit())
        || lines.any(|line| !line.trim().is_empty())
    {
        return None;
    }
    Some(version)
}

/// The driver's own compiled-in version — what this process's text segment
/// was built as, however the `grove` on disk has moved since.
///
/// The version-skew guard (driver-version-skew-k11) compares it against the
/// `grove-llm` the agent would invoke, **per session** rather than per driver
/// start: a long-running driver keeps executing the text segment it started
/// with, while `brew upgrade` replaces the binaries on disk under it, and a
/// mid-loop upgrade is exactly the case a start-time check misses. A skewed
/// pair splits the signal protocol's two halves — observed as every session
/// hanging at its completion signal with nothing ever relaunching.
const DRIVER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Reset the terminal after a (possibly SIGTERM'd) TUI: restore cooked mode,
/// leave the alternate screen, show the cursor. No-op when stdin isn't a TTY
/// (headless / test runs).
fn reset_terminal() {
    if unsafe { libc::isatty(libc::STDIN_FILENO) } != 1 {
        return;
    }
    let _ = Command::new("stty").arg("sane").status();
    print!("\x1b[?1049l\x1b[?25h\x1b[0m");
    let _ = std::io::stdout().flush();
}

/// Ignore SIGINT in the driver so a terminal Ctrl-C (delivered to the whole
/// foreground process group) does not kill the loop; the child session
/// installs its own handler and still responds. The driver must survive the
/// interrupt to reach the relaunch-vs-stop decision.
fn ignore_interrupts() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }
}

/// Set by [`on_terminate`], read by the watcher's poll loop. A `bool` store is
/// the only work done inside the handler because it is async-signal-safe; the
/// watcher handles child termination on a normal stack one poll tick later.
static TERMINATED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

extern "C" fn on_terminate(_signum: libc::c_int) {
    TERMINATED.store(true, std::sync::atomic::Ordering::Relaxed);
}

/// Catch SIGTERM and SIGHUP so the driver can forward termination to its child
/// and reap it through the ordinary watcher escalation. SIGINT is deliberately
/// absent because [`ignore_interrupts`] keeps Ctrl-C directed at the foreground
/// session.
fn install_termination_handler() {
    let handler = on_terminate as extern "C" fn(libc::c_int) as usize;
    unsafe {
        libc::signal(libc::SIGTERM, handler as libc::sighandler_t);
        libc::signal(libc::SIGHUP, handler as libc::sighandler_t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::process::Stdio;

    /// The escalation seam: [`wait_with_watcher_result`] takes its two graces as
    /// a parameter, so the suite drives the whole state machine on test
    /// timescales without any process configuration. Production passes
    /// [`DEFAULT_GRACE`]/[`DEFAULT_KILL_GRACE`] and nothing else can.
    const TEST_GRACES: (Duration, Duration) =
        (Duration::from_millis(50), Duration::from_millis(50));

    fn spawn_sh(script: &str) -> Child {
        Command::new("sh")
            .arg("-c")
            .arg(script)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawning the fixture child")
    }

    /// The signal file is watched for *existence*, so a fixture only has to
    /// create it. Named per test to keep concurrent tests independent.
    fn signal_path(dir: &tempfile::TempDir, name: &str) -> PathBuf {
        dir.path().join(name)
    }

    // No signal file, ever: the watcher must not touch the child at all. This is
    // the case that separates "the driver ends sessions" from "the driver ends
    // sessions that asked to be ended" — a human still working in a session that
    // has not signalled must be left alone however long it runs.
    #[test]
    fn an_unsignalled_session_runs_to_its_own_exit_untouched() {
        let dir = tempfile::tempdir().unwrap();
        let signal = signal_path(&dir, "never-written");
        let child = spawn_sh("sleep 0.3; exit 7");

        let ended = wait_with_watcher_result(child, &signal, TEST_GRACES).unwrap();

        assert_eq!(ended.end, SessionEnd::Exited);
        assert_eq!(ended.status.code(), Some(7), "the child chose its own exit");
        assert_eq!(ended.status.signal(), None, "nothing signalled the child");
        assert!(!signal.exists());
    }

    // The completion path: the agent writes the signal file and then keeps
    // running (an interactive TUI cannot end itself), so the *driver* ends it
    // after the grace.
    #[test]
    fn a_signalled_session_that_keeps_running_is_terminated_after_the_grace() {
        let dir = tempfile::tempdir().unwrap();
        let signal = signal_path(&dir, "relaunch");
        let child = spawn_sh(&format!(": > '{}'; sleep 30", signal.display()));

        let ended = wait_with_watcher_result(child, &signal, TEST_GRACES).unwrap();

        assert_eq!(ended.end, SessionEnd::Exited);
        assert_eq!(
            ended.status.signal(),
            Some(libc::SIGTERM),
            "a signalled session is ended with SIGTERM first, not SIGKILL"
        );
    }

    // Escalation. SIGTERM is a request; a session that ignores it would
    // otherwise hold the TTY forever and park the loop with no diagnostic, so
    // the watcher follows through.
    #[test]
    fn a_session_that_ignores_sigterm_is_killed_after_the_kill_grace() {
        let dir = tempfile::tempdir().unwrap();
        let signal = signal_path(&dir, "ignores-term");
        let child = spawn_sh(&format!(
            "trap '' TERM; : > '{}'; sleep 30",
            signal.display()
        ));

        let ended = wait_with_watcher_result(child, &signal, TEST_GRACES).unwrap();

        assert_eq!(
            ended.status.signal(),
            Some(libc::SIGKILL),
            "a session that ignores SIGTERM must still be reaped"
        );
    }

    // The grace exists so the agent's own `complete` call can return and its
    // turn can end before the session dies. A zero-grace kill would truncate
    // that, so the wait is asserted, not assumed.
    #[test]
    fn the_grace_elapses_before_the_first_signal_is_sent() {
        let dir = tempfile::tempdir().unwrap();
        let signal = signal_path(&dir, "graced");
        let child = spawn_sh(&format!(": > '{}'; sleep 30", signal.display()));

        let grace = Duration::from_millis(1200);
        let ended =
            wait_with_watcher_result(child, &signal, (grace, Duration::from_millis(50))).unwrap();

        assert!(
            ended.elapsed >= grace,
            "the watcher signalled after {:?}, before the {grace:?} grace",
            ended.elapsed
        );
        assert_eq!(ended.status.signal(), Some(libc::SIGTERM));
    }

    // A child that exits on its own between the signal and the grace is reaped
    // by `try_wait`, not by a kill — the common real case, since a session that
    // finishes its turn promptly after signalling just exits.
    #[test]
    fn a_signalled_session_that_exits_on_its_own_is_never_signalled() {
        let dir = tempfile::tempdir().unwrap();
        let signal = signal_path(&dir, "self-exit");
        let child = spawn_sh(&format!(": > '{}'; exit 0", signal.display()));

        let ended =
            wait_with_watcher_result(child, &signal, (Duration::from_secs(30), TEST_GRACES.1))
                .unwrap();

        assert_eq!(ended.end, SessionEnd::Exited);
        assert!(ended.status.success());
        assert_eq!(ended.status.signal(), None);
    }

    #[test]
    fn successive_launches_get_independent_workspace_signal_files() {
        let worktree = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(worktree.path().join(".git")).unwrap();
        let lease = DriverLease::acquire(worktree.path()).unwrap();

        let first = lease.allocate_signal_channel().unwrap();
        let second = lease.allocate_signal_channel().unwrap();
        let expected_control_dir = worktree.path().canonicalize().unwrap().join(".git/grove");

        assert_ne!(
            first.path(),
            second.path(),
            "each foreground launch needs an independent control channel"
        );
        assert_eq!(first.path().parent(), Some(expected_control_dir.as_path()));
        assert_eq!(second.path().parent(), Some(expected_control_dir.as_path()));
    }

    #[test]
    fn an_epoch_handoff_failure_preserves_the_launch_failure_that_preceded_it() {
        let launch: Result<SessionEnd> = Err(anyhow::anyhow!(
            "launching the session: executable was not found"
        ));
        let continuation_called = std::cell::Cell::new(false);

        let error = complete_post_reap_epoch_handoff(
            launch,
            || {
                Err(anyhow::anyhow!(
                    "timed out waiting for exclusive session epoch lock"
                ))
            },
            |_| continuation_called.set(true),
        )
        .unwrap_err();

        let message = format!("{error:#}");
        assert!(
            message.contains(
                "post-reap session epoch invalidation blocked; completion signal left unconsumed"
            ),
            "{message}"
        );
        assert!(message.contains("executable was not found"), "{message}");
        assert!(
            message.contains("timed out waiting for exclusive session epoch lock"),
            "{message}"
        );
        assert!(
            !continuation_called.get(),
            "signal interpretation must remain behind successful epoch invalidation"
        );
    }

    #[test]
    fn signal_interpretation_cannot_run_before_epoch_invalidation_succeeds() {
        let continuation_called = std::cell::Cell::new(false);

        let error = complete_post_reap_epoch_handoff(
            Ok(SessionEnd::Exited),
            || Err(anyhow::anyhow!("exclusive epoch handoff timed out")),
            |_| continuation_called.set(true),
        )
        .unwrap_err();

        assert!(
            error.to_string().contains(
                "post-reap session epoch invalidation blocked; completion signal left unconsumed"
            ),
            "{error:#}"
        );
        assert!(
            !continuation_called.get(),
            "signal interpretation must remain behind successful epoch invalidation"
        );
    }

    // The version-skew guard may only ever act on output that *is* a
    // `grove-llm --version` line. Anything else — an empty read, a shell's own
    // error text, another binary's version, a dev-build tag — must parse to
    // `None`, so the driver refuses on an unrecognised answer rather than
    // "comparing" free text.
    #[test]
    fn version_parsing_accepts_the_agent_cli_and_rejects_everything_else() {
        assert_eq!(parse_checked_version("grove-llm 13.0.0\n"), Some("13.0.0"));
        assert_eq!(
            parse_checked_version("grove-llm 13.0.0\n\n"),
            Some("13.0.0")
        );
        assert_eq!(parse_checked_version(""), None);
        assert_eq!(parse_checked_version("zsh: command not found\n"), None);
        assert_eq!(parse_checked_version("grove-llm dev-build\n"), None);
        assert_eq!(parse_checked_version("not-grove-llm 13.0.0\n"), None);
        assert_eq!(parse_checked_version("grove-llm 13.0.0 extra\n"), None);
        assert_eq!(parse_checked_version("grove-llm 13.0.0\ntrailing\n"), None);
    }
}
