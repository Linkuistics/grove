// The self-driving loop: select a leaf, compose its prompt, run dispatch,
// invalidate the reaped session's epoch, then interpret the launch.
//
// Dispatch alone watches the harness's exit channel and ends its group. Grove
// launches dispatch as a channel-free job; its runner forwards cancellation,
// reaps dispatch and restores the terminal, including after dispatch's death
// left an orphaned harness holding the foreground. The driver then resets it.
//
// Restart is continuation: each iteration derives its position from the tree.
// The caller owns the workspace lease; this module owns the launch directory,
// prompt, argv and ordered reading of the ending and teardown record.

use crate::driver_lease::DriverLease;
use crate::{Handle, Kind, Reading, Selection, Sought, TreeLifetime};
use anyhow::{ensure, Context, Result};
use jj_workspace::Workspace;
use keyed_launch::{Argv, End, Ended, Escalation, Group, Launch};
use std::ffi::{OsStr, OsString};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

/// The loop driver's **launch-scoped environment** (self-driving-loop) — the
/// variables a descendant could act on, and the exact set every spawn below
/// hands to `keyed_launch` as its scrub list.
///
/// `GROVE_SIGNAL_FILE` is legacy completion authority, scrubbed while this
/// meta-grove still runs under installed v22. No current launch grants it.
/// `GROVE_LAUNCH_DIR` identifies the admitted session; `HARNESS_DISPATCH_EXIT_FILE`
/// ends a dispatch run. Both are scrubbed before the current launch is granted.
/// `GROVE_HARNESS_PID` / `GROVE_CLAUDE_PID` are the retired pre-watcher handles
/// (self-driving-loop), kept here because a stale, unrelated PID leaking into a
/// nested grove is the same class of mistake one notch quieter — the value is
/// something a reader could still *act on*. That is the bar for membership.
///
/// **Any spawn that is not the session itself must scrub this whole
/// list** (guard-loop-signal-k37), and so must the session's own, which then
/// receives the one path it owns. Scrubbing is the default and granting is the
/// exception; `keyed_launch::run` takes the list precisely so the grant cannot
/// happen without the scrub.
///
/// The failure this closes was not hypothetical. This repo is a meta-grove, so
/// its own suite runs as a *descendant* of a live session; a since-removed
/// pre-flight spawned a harness binary without scrubbing, the suite's fake
/// commands write `"$GROVE_SIGNAL_FILE"` unconditionally, and `cargo test`
/// killed the terminal it was typed into.
///
/// Grove's own spawns are exactly two — the session and `stty sane`
/// — and both scrub. (There were three: the build-pairing probe went with
/// provisioning at `delete-provisioning-k19`, since a driver that writes no
/// skill directory has no pairing to report.) The one
/// other family, the VCS probes and the teardown commit, went to
/// `jj-workspace`, which scrubs the *repository selectors* itself because
/// choosing the right repository is its guarantee to make. It deliberately does
/// not scrub this list: it has no consumer to speak for, and `jj` reads no
/// `GROVE_*` variable. That is narrower than *nothing downstream of it can act
/// on one* — `jj` execs a user-configured pager, editor and fsmonitor, which
/// inherit whatever `jj` inherited — and the seam's own record is where that
/// belongs rather than here.
const LOOP_CONTROL_ENV: [&str; 5] = [
    "GROVE_SIGNAL_FILE",
    "GROVE_LAUNCH_DIR",
    "HARNESS_DISPATCH_EXIT_FILE",
    "GROVE_HARNESS_PID",
    "GROVE_CLAUDE_PID",
];

/// [`LOOP_CONTROL_ENV`] as the runner takes it.
fn scrub_list() -> [&'static OsStr; LOOP_CONTROL_ENV.len()] {
    LOOP_CONTROL_ENV.map(OsStr::new)
}

/// Deliberately one helper rather than an `env_remove` per site: the list is the
/// interesting part, and a second site open-coding it is how the first one came
/// to be missed. The session goes through
/// [`keyed_launch::Launch::scrub`] instead, which is the same list by the same
/// rule.
pub(crate) fn scrub_loop_control_env(cmd: &mut Command) {
    for name in LOOP_CONTROL_ENV {
        cmd.env_remove(name);
    }
}

/// Why the loop stopped — the loop's terminal disposition, made first-class so
/// a clean whole-grove finish is distinguishable from an abnormal stop (rather
/// than both looking like "the loop just ended").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopOutcome {
    /// The grove finished cleanly: this launch recorded its teardown.
    Finished,
    /// A non-signalled exit stopped the loop (human `/exit`/Ctrl-C, or a
    /// crash); resumable by re-running `grove` from the same working tree.
    ///
    /// Nothing about the *delivery* of the methodology is among the reasons.
    /// The driver used to report a mismatched, unidentifiable or missing
    /// `grove-llm` here and launch anyway; `delete-provisioning-k19` deleted
    /// both that report and the skill directory it was about, so a session's
    /// environment is now entirely the human's to keep right.
    Stopped,
    /// **The driver itself** was sent SIGTERM or SIGHUP and stopped the loop
    /// because of it — carrying which one, because whoever started the driver
    /// has to be told.
    ///
    /// Distinct from `Stopped`, which is the loop reaching a decision it was
    /// designed to reach. This is the loop being taken away mid-grove: a
    /// systemd unit restarting, a `timeout(1)` firing, a terminal closing. A
    /// caller that mapped it to a clean exit would be telling its own parent
    /// that a grove finished, so `crates/grove` ends on
    /// [`keyed_launch::reraise`] and the parent sees `128 + N` instead.
    Interrupted(i32),
}

/// **The whole loop**: `exists? → create or find next → run → finalise`, one
/// foreground session per selected task, until a session stops signalling
/// (`docs/specs/module-decomposition.md`, decision 9).
///
/// The three arguments are the three things a loop cannot derive for itself and
/// is therefore handed: the **workspace** it drives (resolved by its caller,
/// which had to resolve one to take the lease), the **lease** proving it is the
/// only driver in that working tree, and the path of the **`harness-dispatch`**
/// executable every session is launched through, which its caller found beside
/// its own.
///
/// Nothing here inspects the working tree for a harness, nothing chooses a
/// binary and nothing reads a configuration: the owner's dispatch policy is the
/// whole of launch policy (`docs/specs/harness-selection-and-execution.md`,
/// *Grove integration*). Nothing here
/// delivers the methodology either — since `delete-provisioning-k19` the
/// methodology is a plugin a human installs, so the loop's first act is a
/// transition rather than a sweep over three personal skill directories.
///
/// **The lease is taken by value.** It is dropped when the loop returns, which
/// is the point at which the working tree stops being owned; a caller that kept
/// one could go on holding ownership after the loop that justified it ended.
///
/// # Errors
///
/// A lease that stops naming the descriptors this process owns, a tree the
/// store refuses, or a `harness-dispatch` that could not be spawned. A launch
/// the owner's policy refuses is not an error: it is a session that ended
/// without an exit-signal ending, and the loop stops on it.
pub fn run(
    workspace: &Workspace,
    mut lease: DriverLease,
    dispatch: &Path,
) -> Result<LoopOutcome, crate::Error> {
    ignore_interrupts();
    Ok(drive(workspace, &mut lease, dispatch)?)
}

fn drive(
    workspace: &Workspace,
    driver_lease: &mut DriverLease,
    dispatch: &Path,
) -> Result<LoopOutcome> {
    // Taken from the lease rather than recomputed: dispatch runs here, which is
    // the location the prompt assumes and the one a policy reads as its `cwd`.
    let worktree_path = driver_lease.worktree_root().to_path_buf();
    let worktree = worktree_path.as_path();

    loop {
        // A SIGTERM or SIGHUP that arrived while no session was running has no
        // launch to be reported against, so the runner discards it rather than
        // spending it on the next child. Collecting it here is what keeps the
        // driver from going on mutating the tree and taking commits after its
        // terminal has gone.
        if let Some(signal) = keyed_launch::take_interrupt() {
            eprintln!(
                "grove: interrupted by signal {signal} between sessions — stopping the loop."
            );
            return Ok(LoopOutcome::Interrupted(signal));
        }
        driver_lease
            .revalidate()
            .context("revalidating driver lease before loop transition")?;

        // No kind is checked here, or anywhere before the launch: whether the
        // owner's policy routes one is asked when its leaf launches.
        crate::driver::transition_to_current(worktree)?;
        let selected = match picked(worktree)? {
            Sought::Match(selection) => selection,
            Sought::Nothing => picked_after_finish(worktree)?,
        };
        let selection = selected.selection.clone();

        let prompt = session_prompt(&selection.handle, &selection.kind, workspace);

        driver_lease
            .revalidate()
            .context("revalidating driver lease before foreground launch")?;
        let launch_dir =
            crate::launch_directory::LaunchDirectory::allocate(driver_lease.control_dir())?;
        let argv = dispatch_run(dispatch, &selection, &prompt, launch_dir.path());
        let ended = launch_session(&argv, selected, worktree, launch_dir.path(), driver_lease);
        // Unconditionally, and before the invalidation gate below: the session
        // may have left the terminal in raw mode and on the alternate screen,
        // and an error path that returns without restoring it hands the human
        // an unusable shell to read the error in. Restoring is not
        // interpretation, so it is not what the gate is protecting.
        reset_terminal();
        let (ended, reading) = complete_post_reap_epoch_handoff(
            ended,
            || driver_lease.invalidate_session_epoch(),
            |ended: Ended| {
                let reading = launch_dir.read();
                (ended, reading)
            },
        )?;

        if let Err(error) = launch_dir.discard() {
            eprintln!(
                "grove: warning: could not remove the interpreted foreground-session launch directory; preserving the session outcome: {error}"
            );
        }

        if let Some(outcome) = interpret_launch(&ended, &reading, &selection) {
            return Ok(outcome);
        }
    }
}

/// First-match interpretation after reap and epoch invalidation. Kept at the
/// runner-result seam so precedence over a surviving group is deterministic
/// to exercise without manufacturing an unkillable OS process.
fn interpret_launch(
    ended: &Ended,
    reading: &crate::launch_directory::LaunchReading,
    selection: &Selection,
) -> Option<LoopOutcome> {
    if let End::Interrupted { signal } = ended.end {
        eprintln!("grove: interrupted by signal {signal} — stopping the loop.");
        return Some(LoopOutcome::Interrupted(signal));
    }

    // Teardown is this launch's explicit disposition, irrespective of the
    // harness's ending. The driver interrupt above remains authoritative.
    if reading.teardown {
        eprintln!("grove: grove finished — loop complete.");
        return Some(LoopOutcome::Finished);
    }

    // Before the ending is acted on, and whatever it says: a member of
    // dispatch's group that survived the runner's kills may still hold the
    // tree, so no relaunch may happen beside it.
    if let Group::Present { pgid } = ended.group {
        eprintln!(
            "grove: members of the session's process group {pgid} may have survived it — \
             status {}; loop stopped with `{}` still live. Check for a leftover process \
             before rerunning `grove`.",
            ended.status, selection.handle
        );
        return Some(LoopOutcome::Stopped);
    }

    if reading.exit_signal {
        return None;
    }
    eprintln!(
        "grove: session ended without an exit-signal ending — status {}, elapsed {:.3}s; loop stopped.",
        ended.status, ended.elapsed.as_secs_f64()
    );
    use std::os::unix::process::ExitStatusExt as _;
    if ended.status.signal().is_some() {
        eprintln!("       dispatch died of a signal; its harness may have survived. Check for a leftover process before rerunning `grove`.");
    }
    if !ended.status.success() {
        eprintln!(
            "       session kind `{}` for `{}` failed; if harness-dispatch refused the \
             launch, its diagnostic and remedy are above and the leaf is still live. \
             Either way, rerun `grove` to continue.",
            selection.kind.label(),
            selection.handle
        );
    }
    Some(LoopOutcome::Stopped)
}

/// The whole `${prompt}`: the guaranteed core, composed for the launched kind.
///
/// **The driver hands a session a pointer, not the methodology.** What earns a
/// place, and why the facts below arrive as bare values with no normative tail,
/// is [`crate::prompt`]'s to state and this function's to supply: the selected
/// leaf's stable handle, the resolved workspace the prompt states the version
/// control from, and grove's own published release version.
///
/// The kind is passed in rather than re-read: it is the same value
/// `harness-dispatch` receives as `--kind`, taken from the one guarded
/// selection, so the prompt and the policy that selects the command cannot
/// disagree about what kind it is.
///
/// **The version is `CARGO_PKG_VERSION`, read here rather than in `prompt`**, so
/// composition takes a value like every other runtime fact and the module that
/// composes text does not also decide what build it is part of. It is the same
/// value `grove --version` renders — clap derives that from this constant — which
/// is what makes the flag a fallback for the published fact rather than a second
/// source of it (`docs/specs/module-decomposition.md`, decision 10).
///
/// **Infallible**, and that is what taking the workspace as an argument bought.
/// This used to resolve one, which could fail in a driver that had already
/// proved a marker existed by leasing the `.jj/` beside it — an unreachable
/// error arm the loop still had to carry. The loop is handed the workspace it
/// is driving, so there is nothing left here to fail.
fn session_prompt(handle: &Handle, kind: &Kind, workspace: &Workspace) -> String {
    crate::prompt::compose(&crate::prompt::Mandate {
        handle,
        kind,
        workspace,
        version: crate::VERSION,
    })
}

/// The `harness-dispatch run` invocation for one selected leaf: everything the
/// owner's policy may select from, and nothing about how it selects
/// (`docs/specs/harness-selection-and-execution.md`, *A lifecycle session*).
///
/// The kind, the task file and the handle are read from the selection that
/// composed `prompt`, so they cannot describe a different leaf from the mandate.
/// No parameter is passed. The session's location is the `cwd` dispatch runs
/// in, and naming the session is the methodology's. No policy entry, bound,
/// grant or record directory is passed either: those are the owner's settings.
///
/// Each value is joined to its flag in one word, so a prompt or a path that
/// begins with a dash is still a value. A value dispatch cannot take, a path
/// that is not UTF-8 for one, is dispatch's to refuse.
fn dispatch_run(dispatch: &Path, task: &Selection, prompt: &str, launch_dir: &Path) -> Argv {
    let word = |flag: &str, value: &OsStr| {
        let mut word = OsString::from(flag);
        word.push(value);
        word
    };
    Argv::new(
        dispatch.into(),
        vec![
            "run".into(),
            word("--kind=", task.kind.label().as_ref()),
            word("--task-file=", task.path.as_os_str()),
            word("--task-id=", task.handle.to_string().as_ref()),
            word("--prompt=", prompt.as_ref()),
            word("--exit-dir=", launch_dir.as_os_str()),
            word("--ending-file=", launch_dir.join("ending.json").as_os_str()),
        ],
    )
}

/// Launch dispatch as a foreground, channel-free job. Dispatch selects and
/// supervises the harness as a separate job and owns its exit signal. The runner
/// reports dispatch's start/reap and restores the handed-over terminal modes.
///
/// Prints one diagnostic line naming the kind and the selected handle. That
/// line is the only durable record of what each session in a loop was working
/// on, so it names the **stable handle** rather than a path, which moves under
/// `leaf-insert`.
///
/// A spawn failure names the program, which the runner's own message says to
/// check is executable; grove adds the session kind.
///
/// The epoch is activated **before** the spawn and never after: a child that is
/// already running under an inactive epoch would have its own `grove-llm` verbs
/// refused.
fn launch_session(
    argv: &Argv,
    selected: SelectedTask,
    worktree: &Path,
    launch_dir: &Path,
    driver_lease: &mut DriverLease,
) -> Result<Ended> {
    let selection = &selected.selection;
    eprintln!(
        "grove: launching {} through harness-dispatch — {}",
        selection.kind.label(),
        selection.handle
    );

    driver_lease
        .prepare_launch(selected.lifetime, selection, launch_dir)
        .context("activating the foreground session epoch before spawn")?;

    driver_lease
        .supervise_launch(|observer| {
            keyed_launch::run_observed(
                Launch {
                    argv,
                    channel: None,
                    scrub: &scrub_list(),
                    grant: &[(OsStr::new("GROVE_LAUNCH_DIR"), launch_dir.as_os_str())],
                    transparent: None,
                    cwd: Some(worktree),
                    escalation: ESCALATION,
                },
                observer,
            )
        })
        .with_context(|| {
            format!(
                "launching session kind `{}` via {:?}",
                selection.kind.label(),
                argv.program()
            )
        })
}

fn complete_post_reap_epoch_handoff<E, T>(
    ended: Result<E>,
    invalidate: impl FnOnce() -> Result<()>,
    continue_after_invalidation: impl FnOnce(E) -> T,
) -> Result<T> {
    const INVALIDATION_CONTEXT: &str =
        "post-reap session epoch invalidation blocked; launch ending left unconsumed";

    match (ended, invalidate()) {
        (Ok(ended), Ok(())) => Ok(continue_after_invalidation(ended)),
        (Err(launch_error), Ok(())) => Err(launch_error),
        (Ok(_), Err(invalidation_error)) => Err(invalidation_error.context(INVALIDATION_CONTEXT)),
        (Err(launch_error), Err(invalidation_error)) => Err(invalidation_error.context(format!(
            "{INVALIDATION_CONTEXT}; foreground session also failed: {launch_error:#}"
        ))),
    }
}

/// No channel grace: Grove ends no harness. Its own TERM/HUP is forwarded
/// to dispatch, whose cancellation must finish before this 10-second kill-grace.
/// Dispatch budgets 5 seconds for the harness, 1 for group confirmation and
/// 2 for its record-store lock (decision 7).
const ESCALATION: Escalation = Escalation {
    grace: Duration::ZERO,
    kill_grace: Duration::from_secs(10),
};

/// Reset the terminal after a (possibly SIGTERM'd) TUI: restore cooked mode,
/// leave the alternate screen, show the cursor. No-op when stdin isn't a TTY
/// (headless / test runs), or when this driver is not the terminal's foreground
/// process group.
///
/// **The second guard is not tidiness, it is a hang.** `stty` sets terminal
/// attributes, and `tcsetattr` from a *background* process group raises SIGTTOU
/// at the caller whatever `TOSTOP` says — so a driver that is not the terminal's
/// owner would spawn an `stty` that stops on its first act, and wait on a
/// stopped child forever. A driver reaches here as the owner in the ordinary
/// case, because the runner hands the terminal to the session and takes it back
/// before returning; what this covers is the case where it never had it, which
/// is `grove &` from a shell that kept the foreground for itself.
fn reset_terminal() {
    if unsafe { libc::isatty(libc::STDIN_FILENO) } != 1 {
        return;
    }
    // SAFETY: `tcgetpgrp(3)` on stdin, proved a terminal above, and `getpgrp(2)`.
    if unsafe { libc::tcgetpgrp(libc::STDIN_FILENO) != libc::getpgrp() } {
        return;
    }
    let mut stty = Command::new("stty");
    stty.arg("sane");
    // `stty` reads no `GROVE_*` variable, so this grants it nothing it could
    // act on — and it is scrubbed anyway, because a rule with one argued
    // exception is a rule the next spawn has to re-argue.
    scrub_loop_control_env(&mut stty);
    let _ = stty.status();
    print!("\x1b[?1049l\x1b[?25h\x1b[0m");
    let _ = std::io::stdout().flush();
}

/// Ignore SIGINT in the driver so a terminal Ctrl-C does not kill the loop. The
/// driver must survive the interrupt to reach the relaunch-vs-stop decision.
///
/// **This covers the gaps between sessions, and only those.** While a session
/// is running it is the terminal's foreground process group in its own right —
/// the runner puts it there and hands it the terminal — so a typed Ctrl-C is
/// delivered to the session and never reaches the driver at all. This ignore is
/// what holds in the moments the driver is transitioning the tree and selecting
/// a leaf with no child in front of it.
///
/// **It does not leak into the session.** An ignored disposition is the one
/// kind that survives `execve`, which is exactly why this used to reach the
/// session, everything it spawned, and every wrapper in front of it — a login
/// shell that inherits an ignored SIGINT keeps ignoring it and
/// passes it on, so Ctrl-C did nothing at all and nothing in the session could
/// say why. The runner now resets the child's dispositions to their defaults
/// across the spawn (`keyed_launch::run`), so this stays the driver's own
/// policy rather than the subtree's.
///
/// SIGINT and SIGINT alone, because it is the one disposition that is a
/// *policy* rather than a mechanism: what a loop does about the human's Ctrl-C
/// is the loop's business. SIGTERM and SIGHUP belong to the runner, which
/// catches them itself so it can forward one to its child's process group and
/// reap it rather than orphan it onto the terminal — and reports that, with the
/// signal, as [`keyed_launch::End::Interrupted`].
fn ignore_interrupts() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }
}

/// The driver's own `pick`, over the worktree it is driving.
///
/// The transition above has already brought the worktree to a grove, so the
/// vacant arm is unreachable in practice — but it is an arm of [`crate::read`],
/// and answering it as *no live leaves* is the same thing
/// the transition would have made true a moment earlier.
fn picked(worktree: &Path) -> anyhow::Result<Sought<SelectedTask>> {
    // Open before snapshot acquisition and check while its tree guard is held.
    // A root replaced during the read must not lend its identity to old names.
    let lifetime = TreeLifetime::open(worktree)?;
    match crate::read(worktree)? {
        Reading::Tree(tree) => {
            let lifetime = lifetime.context("task tree changed during selection")?;
            ensure!(lifetime.at(worktree)?, "task tree changed during selection");
            Ok(crate::verbs::pick(&tree)?.map(|selection| SelectedTask {
                selection,
                lifetime,
            }))
        }
        Reading::Vacant => Ok(Sought::Nothing),
    }
}

/// Selection remains value data; only this driver's pending launch owns the pin.
#[derive(Debug)]
struct SelectedTask {
    selection: Selection,
    lifetime: TreeLifetime,
}

fn picked_after_finish(worktree: &Path) -> Result<SelectedTask> {
    crate::driver::materialize_finish(worktree)?;
    // Materialization releases its write guard. Select again under a fresh read
    // guard, so a concurrent edit supplies both the mandate and its pinned root.
    match picked(worktree)? {
        Sought::Match(selected) => Ok(selected),
        Sought::Nothing => anyhow::bail!("task tree changed after finish materialization"),
    }
}

// The repository's shared test helpers. Declared out here because a `#[path]`
// inside the inline module below would resolve against a directory that does
// not exist.
#[cfg(test)]
#[path = "../../../testing/support.rs"]
mod support;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task_grow::tests::descriptors_held_on;

    fn selected_root_fixture() -> tempfile::TempDir {
        let temp = tempfile::tempdir().unwrap();
        std::fs::create_dir(temp.path().join(".jj")).unwrap();
        std::fs::create_dir(temp.path().join(".grove")).unwrap();
        std::fs::write(temp.path().join(".grove/_BRIEF.md"), "brief").unwrap();
        std::fs::write(temp.path().join(".grove/01-impl--work-k1.md"), "task").unwrap();
        temp
    }

    /// The loop's own `harness-dispatch run` for `task`, under a HOME whose
    /// policy runs `harness` for every kind. The real front and its compiled
    /// worker select it. `env` sets that HOME on the front it becomes, because
    /// a test may not change its own process's environment.
    fn dispatched(home: &Path, harness: &Path, task: &Selection) -> Argv {
        std::fs::write(harness, "#!/bin/sh\ntouch launched\n").unwrap();
        let mut permissions = std::fs::metadata(harness).unwrap().permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
        std::fs::set_permissions(harness, permissions).unwrap();
        support::route_every_kind_to(home, harness);
        std::fs::create_dir_all(home.join("exit")).unwrap();
        let run = dispatch_run(
            &support::harness_dispatch(),
            task,
            "prompt",
            &home.join("exit"),
        );
        let mut home_word = OsString::from("HOME=");
        home_word.push(home);
        let mut words = vec![home_word];
        words.extend(run.words());
        Argv::new("/usr/bin/env".into(), words)
    }

    #[test]
    fn launch_reading_precedence_over_interrupts_and_surviving_groups() {
        use std::os::unix::process::ExitStatusExt;
        let selection = Selection {
            path: "01-impl--work-k1.md".into(),
            handle: Handle::parse("work-k1").unwrap(),
            kind: Kind::new("impl").unwrap(),
        };
        for (end, group, teardown, exit_signal, want) in [
            (
                End::Interrupted {
                    signal: libc::SIGTERM,
                },
                Group::Present { pgid: 123 },
                true,
                true,
                Some(LoopOutcome::Interrupted(libc::SIGTERM)),
            ),
            (
                End::Interrupted {
                    signal: libc::SIGHUP,
                },
                Group::Gone,
                true,
                true,
                Some(LoopOutcome::Interrupted(libc::SIGHUP)),
            ),
            (
                End::Exited,
                Group::Present { pgid: 123 },
                true,
                true,
                Some(LoopOutcome::Finished),
            ),
            (
                End::Exited,
                Group::Present { pgid: 123 },
                true,
                false,
                Some(LoopOutcome::Finished),
            ),
            (
                End::Exited,
                Group::Present { pgid: 123 },
                false,
                true,
                Some(LoopOutcome::Stopped),
            ),
            (End::Exited, Group::Gone, false, true, None),
            (
                End::Exited,
                Group::Gone,
                false,
                false,
                Some(LoopOutcome::Stopped),
            ),
        ] {
            let ended = Ended {
                end,
                group,
                status: std::process::ExitStatus::from_raw(7 << 8),
                elapsed: Duration::ZERO,
                signalled: false,
            };
            let reading = crate::launch_directory::LaunchReading {
                teardown,
                exit_signal,
            };
            assert_eq!(
                interpret_launch(&ended, &reading, &selection),
                want,
                "{end:?}, {group:?}, teardown={teardown}, exit_signal={exit_signal}"
            );
        }
    }

    #[test]
    fn selected_root_launches_only_while_its_directory_is_current() {
        for state in ["current", "removed", "replaced"] {
            let fixture = tempfile::tempdir().unwrap();
            let temp = selected_root_fixture();
            let work = temp.path();
            let workspace = Workspace::resolve(work).unwrap();
            let mut lease = DriverLease::acquire(&workspace).unwrap();
            let Sought::Match(selection) = picked(work).unwrap() else {
                panic!("fixture must select work-k1");
            };
            if state != "current" {
                std::fs::rename(work.join(".grove"), work.join("old")).unwrap();
            }
            if state == "replaced" {
                std::fs::create_dir(work.join(".grove")).unwrap();
                std::fs::write(work.join(".grove/_BRIEF.md"), "replacement").unwrap();
                std::fs::write(work.join(".grove/01-impl--work-k1.md"), "reused key").unwrap();
            }
            let argv = dispatched(
                &fixture.path().join("home"),
                &fixture.path().join("harness"),
                &selection.selection,
            );
            let launch_dir =
                crate::launch_directory::LaunchDirectory::allocate(lease.control_dir()).unwrap();
            let result = launch_session(&argv, selection, work, launch_dir.path(), &mut lease);
            assert_eq!(result.is_ok(), state == "current", "{state}: {result:?}");
            assert_eq!(work.join("launched").exists(), state == "current");
            let epoch = std::fs::read_to_string(lease.control_dir().join("session.epoch")).unwrap();
            assert_eq!(
                epoch.starts_with("state=active\n"),
                state == "current",
                "{epoch}"
            );
            if state == "current" {
                assert!(epoch.contains("observation-version=1\n"), "{epoch}");
                assert!(epoch.contains("observation-key=1\n"), "{epoch}");
                // Launch-time work-k1 and impl, encoded without record separators.
                assert!(epoch.contains("observation-handle-hex=776f726b2d6b31\n"));
                assert!(epoch.contains("observation-kind-hex=696d706c\n"));
            }
        }
    }

    #[test]
    fn selected_root_pin_releases_tree_guard_and_survives_path_replacement() {
        let temp = selected_root_fixture();
        let work = temp.path();
        // The control for the scan below: it must count a descriptor this test
        // is holding open, or a zero after selection says nothing.
        let sentinel = std::fs::File::open(work).unwrap();
        assert_eq!(descriptors_held_on(work), 1);
        drop(sentinel);
        let Sought::Match(selected) = picked(work).unwrap() else {
            panic!("fixture must select work-k1");
        };
        // Asked of this process's descriptors and not of the lock: a sibling
        // test's forked child keeps a released guard alive until it execs.
        // `descriptors_held_on` carries the measurement.
        assert_eq!(
            descriptors_held_on(work),
            0,
            "selection retained the containing-directory tree guard"
        );
        std::fs::rename(work.join(".grove"), work.join("old")).unwrap();
        std::fs::create_dir(work.join(".grove")).unwrap();
        let replacement = TreeLifetime::open(work).unwrap().unwrap();
        assert!(!selected.lifetime.same(&replacement).unwrap());
        assert!(!selected.lifetime.at(work).unwrap());
    }

    #[test]
    fn selected_root_finish_and_validation_use_the_shared_selector() {
        let temp = selected_root_fixture();
        let work = temp.path();
        std::fs::rename(
            work.join(".grove/01-impl--work-k1.md"),
            work.join(".grove/01-DONE-impl--work-k1.md"),
        )
        .unwrap();
        assert!(matches!(picked(work).unwrap(), Sought::Nothing));
        let finish = picked_after_finish(work).unwrap();
        assert_eq!(finish.selection.kind, Kind::finish());
        assert!(finish.lifetime.at(work).unwrap());
        // If ordinary work appears, materialization returns it instead of adding
        // another finish. The fresh guarded pick must preserve that behavior.
        std::fs::write(work.join(".grove/03-impl--other-k3.md"), "task").unwrap();
        let ordinary = picked_after_finish(work).unwrap();
        assert_eq!(ordinary.selection.handle.to_string(), "other-k3");
        std::fs::write(work.join(".grove/04-impl--duplicate-k3.md"), "task").unwrap();
        assert!(picked(work).is_err(), "duplicate keys bypassed validation");
    }

    /// The two `complete_post_reap_epoch_handoff` cases below drive that
    /// ordering with a stand-in for the launch result, because what the
    /// ordering is *about* is which of the two failures survives — not what a
    /// session left behind. The launch and escalation themselves are the
    /// runner's, and `crates/keyed-launch/tests/launch.rs` drives them end to
    /// end against a fake child.
    #[test]
    fn an_epoch_handoff_failure_preserves_the_launch_failure_that_preceded_it() {
        let launch: Result<&str> = Err(anyhow::anyhow!(
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
                "post-reap session epoch invalidation blocked; launch ending left unconsumed"
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
            Ok("a session that ended"),
            || Err(anyhow::anyhow!("exclusive epoch handoff timed out")),
            |_| continuation_called.set(true),
        )
        .unwrap_err();

        assert!(
            error.to_string().contains(
                "post-reap session epoch invalidation blocked; launch ending left unconsumed"
            ),
            "{error:#}"
        );
        assert!(
            !continuation_called.get(),
            "signal interpretation must remain behind successful epoch invalidation"
        );
    }
}
