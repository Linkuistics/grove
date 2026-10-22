// The self-driving loop: select a leaf, compose its prompt, run ravel,
// invalidate the reaped session's epoch, then interpret the launch.
//
// Ravel alone watches the harness's exit channel and ends its group. Grove
// starts ravel as an ordinary foreground job through `ravel_job::launch`. That
// function forwards a caught signal to ravel, reaps it and passes the terminal
// back, including after ravel's death left an orphaned harness holding the
// foreground. The driver then resets it.
//
// Restart is continuation: each iteration derives its position from the tree.
// The caller owns the workspace lease; this module owns the launch directory,
// prompt, argv and ordered reading of the ending and teardown record.

use crate::driver_lease::DriverLease;
use crate::ravel_job::{self, JobEnd, Shape};
use crate::session_command::Argv;
use crate::session_result::SessionDisposition;
use crate::{Handle, Kind, Reading, Selection, Sought, TreeLifetime};
use anyhow::{ensure, Context, Result};
use jj_workspace::Workspace;
#[cfg(test)]
use std::ffi::OsString;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Why the loop stopped — the loop's terminal disposition, made first-class so
/// a clean whole-grove finish is distinguishable from an abnormal stop (rather
/// than both looking like "the loop just ended").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopOutcome {
    /// The grove finished cleanly: this launch recorded its teardown.
    Finished,
    /// An explicit `grove-force-stop` stopped the loop without completion.
    /// The CLI reports failure; remaining work is resumable by rerunning Grove.
    ForceStopped,
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
    /// [`ravel_job::reraise`] and the parent sees `128 + N` instead.
    Interrupted(i32),
}

/// **The whole loop**: `exists? → create or find next → run → finalise`, one
/// foreground session per selected task, until a session stops signalling
/// (`docs/specs/module-decomposition.md`, decision 9).
///
/// The three arguments are the three things a loop cannot derive for itself and
/// is therefore handed: the **workspace** it drives (resolved by its caller,
/// which had to resolve one to take the lease), the **lease** proving it is the
/// only driver in that working tree, and the path of the **Ravel**
/// executable every session is launched through, selected and version-checked
/// by its caller from absolute PATH entries.
///
/// Nothing here inspects the working tree for a harness, nothing chooses a
/// binary and nothing reads a configuration: the owner's ravel policy is the
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
/// store refuses, or a Ravel that could not be spawned. A launch
/// the owner's policy refuses is not an error: it is a session that ended
/// without a supported Grove result, and the loop stops on it.
///
/// An error keeps a signal that Grove caught: [`crate::Error::interrupted`]
/// names it, and the caller ends on it after reporting the error.
pub fn run(
    workspace: &Workspace,
    mut lease: DriverLease,
    ravel: &Path,
) -> Result<LoopOutcome, crate::Error> {
    ignore_interrupts();
    drive(workspace, &mut lease, ravel).map_err(|error| {
        // The latch is read after the failure, so a signal that no step consumed
        // still ends the driver. The signal an error carries came first.
        let signal = caught_signal(&error).or_else(ravel_job::take_interrupt);
        crate::Error::from(error).interrupted_by(signal)
    })
}

/// A signal that Grove caught and that an error of the loop still carries.
///
/// The failed spawn or wait carries it in [`ravel_job::JobError`]. The handoff
/// below attaches it as [`Caught`] when a later error would otherwise drop it.
fn caught_signal(error: &anyhow::Error) -> Option<i32> {
    error
        .downcast_ref::<Caught>()
        .map(|caught| caught.0)
        .or_else(|| {
            error
                .downcast_ref::<ravel_job::JobError>()
                .and_then(ravel_job::JobError::interrupted)
        })
}

/// The marker that keeps a caught signal on an error of the loop, as a type
/// and not as text.
#[derive(Debug)]
struct Caught(i32);

impl std::fmt::Display for Caught {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "signal {} reached the driver", self.0)
    }
}

fn drive(
    workspace: &Workspace,
    driver_lease: &mut DriverLease,
    ravel: &Path,
) -> Result<LoopOutcome> {
    // Taken from the lease rather than recomputed: ravel runs here, which is
    // the location the prompt assumes and the one a policy reads as its `cwd`.
    let worktree_path = driver_lease.worktree_root().to_path_buf();
    let worktree = worktree_path.as_path();

    loop {
        // A SIGTERM or SIGHUP that arrived while no session was running has no
        // launch to be reported against, so the latch holds it rather than
        // spending it on the next child. Collecting it here is what keeps the
        // driver from going on mutating the tree and taking commits after its
        // terminal has gone.
        if let Some(signal) = ravel_job::take_interrupt() {
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

        let ending = crate::session_command::end_session(ravel)?;
        let force_stop = crate::session_command::force_stop(ravel)?;
        let prompt = session_prompt(
            &selection.handle,
            &selection.kind,
            workspace,
            &ending,
            &force_stop,
        );

        driver_lease
            .revalidate()
            .context("revalidating driver lease before foreground launch")?;
        let launch_dir =
            crate::launch_directory::LaunchDirectory::allocate(driver_lease.control_dir())?;
        let capture = crate::session_result::SessionResult::allocate()?;
        let argv = crate::session_command::lifecycle(ravel, &selection, &prompt, launch_dir.path());
        let ended = launch_session(
            &argv,
            selected,
            worktree,
            launch_dir.path(),
            driver_lease,
            capture.stdout()?,
        );
        // Unconditionally, and before the invalidation gate below: the session
        // may have left the terminal in raw mode and on the alternate screen,
        // and an error path that returns without restoring it hands the human
        // an unusable shell to read the error in. Restoring is not
        // interpretation, so it is not what the gate is protecting.
        reset_terminal();
        let (ended, reading) = complete_post_reap_epoch_handoff(
            ended,
            || driver_lease.invalidate_session_epoch(),
            |ended: JobEnd| {
                let reading = launch_dir.read();
                (ended, reading)
            },
        )?;

        if let Err(error) = launch_dir.discard() {
            eprintln!(
                "grove: warning: could not remove the interpreted foreground-session launch directory; preserving the session outcome: {error}"
            );
        }

        // TERM/HUP during terminal reset or invalidation arrived after the
        // function froze the end. Honour that latch before accepting a disposition.
        if let Some(signal) = ravel_job::take_interrupt() {
            eprintln!("grove: interrupted by signal {signal} after reap — stopping the loop.");
            return Ok(LoopOutcome::Interrupted(signal));
        }

        if let Some(outcome) = interpret_launch(&ended, &reading, &selection, || capture.read()) {
            return Ok(outcome);
        }
    }
}

/// First-match interpretation after reap and epoch invalidation. Kept at the
/// seam of the job's end so that every precedence is exercised without a
/// process.
fn interpret_launch(
    ended: &JobEnd,
    reading: &crate::launch_directory::LaunchReading,
    selection: &Selection,
    read_result: impl FnOnce() -> std::result::Result<SessionDisposition, crate::Error>,
) -> Option<LoopOutcome> {
    if let Some(signal) = ended.interrupted {
        eprintln!("grove: interrupted by signal {signal} — stopping the loop.");
        return Some(LoopOutcome::Interrupted(signal));
    }

    // Only a successful supervisor's exact result can override teardown. Read
    // once, after reap and invalidation; failed supervisors grant no authority.
    let result = ended.status.success().then(read_result);
    if matches!(result, Some(Ok(SessionDisposition::ForceStop))) {
        eprintln!("grove: force-stop requested — loop stopped without completion; rerun `grove` to resume remaining work.");
        return Some(LoopOutcome::ForceStopped);
    }

    // Teardown otherwise retains its original authority over status and
    // results. Driver interruption and explicit force-stop win.
    if reading.teardown {
        eprintln!("grove: grove finished — loop complete.");
        return Some(LoopOutcome::Finished);
    }

    if let Some(result) = result {
        match result {
            Ok(SessionDisposition::Continue) => return None,
            Ok(SessionDisposition::ForceStop) => return Some(LoopOutcome::ForceStopped),
            Err(error) => eprintln!("grove: unsupported or missing session result: {error}"),
        }
    }
    eprintln!(
        "grove: session ended without a supported Grove result — status {}, elapsed {:.3}s; loop stopped.",
        ended.status, ended.elapsed.as_secs_f64()
    );
    use std::os::unix::process::ExitStatusExt as _;
    if ended.status.signal().is_some() {
        eprintln!("       Ravel died of a signal; its harness may have survived. Check for a leftover process before rerunning `grove`.");
    }
    if !ended.status.success() {
        eprintln!(
            "       session kind `{}` for `{}` failed; if Ravel refused the \
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
/// Ravel receives as `--kind`, taken from the one guarded
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
fn session_prompt(
    handle: &Handle,
    kind: &Kind,
    workspace: &Workspace,
    ending_command: &str,
    force_stop_command: &str,
) -> String {
    crate::prompt::compose(&crate::prompt::Mandate {
        handle,
        kind,
        workspace,
        version: crate::VERSION,
        ending_command,
        force_stop_command,
    })
}

/// Launch ravel as a foreground job through [`ravel_job::launch`]. Ravel selects
/// and supervises the harness as a separate job and owns its exit signal. The
/// function reports ravel's start and reap and restores the handed-over terminal
/// modes. `GROVE_LAUNCH_DIR` goes to this session only.
///
/// Prints one diagnostic line naming the kind and the selected handle. That
/// line is the only durable record of what each session in a loop was working
/// on, so it names the **stable handle** rather than a path, which moves under
/// `leaf-insert`.
///
/// A spawn failure names the program, which the message of the function says to
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
    stdout: std::fs::File,
) -> Result<JobEnd> {
    let selection = &selected.selection;
    eprintln!(
        "grove: launching {} through Ravel — {}",
        selection.kind.label(),
        selection.handle
    );

    driver_lease
        .prepare_launch(selected.lifetime, selection, launch_dir)
        .context("activating the foreground session epoch before spawn")?;

    driver_lease
        .supervise_launch(|observer| {
            ravel_job::launch(
                argv,
                worktree,
                stdout,
                observer,
                Shape::Foreground { launch_dir },
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

/// **Every outcome keeps a caught signal.** The launch error carries its own, and
/// a failed invalidation after an interrupted job would drop the end that held
/// it, so the signal is attached as [`Caught`] before the end goes.
fn complete_post_reap_epoch_handoff<T>(
    ended: Result<JobEnd>,
    invalidate: impl FnOnce() -> Result<()>,
    continue_after_invalidation: impl FnOnce(JobEnd) -> T,
) -> Result<T> {
    const INVALIDATION_CONTEXT: &str =
        "post-reap session epoch invalidation blocked; launch ending left unconsumed";

    match (ended, invalidate()) {
        (Ok(ended), Ok(())) => Ok(continue_after_invalidation(ended)),
        (Err(launch_error), Ok(())) => Err(launch_error),
        (Ok(ended), Err(invalidation_error)) => {
            let error = invalidation_error.context(INVALIDATION_CONTEXT);
            Err(match ended.interrupted {
                Some(signal) => error.context(Caught(signal)),
                None => error,
            })
        }
        (Err(launch_error), Err(invalidation_error)) => {
            let signal = caught_signal(&launch_error);
            let error = invalidation_error.context(format!(
                "{INVALIDATION_CONTEXT}; foreground session also failed: {launch_error:#}"
            ));
            Err(match signal {
                Some(signal) => error.context(Caught(signal)),
                None => error,
            })
        }
    }
}

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
/// case, because `ravel_job::launch` hands the terminal to the session and takes
/// it back before returning; what this covers is the case where it never had it, which
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
    ravel_job::scrub(&mut stty);
    let _ = stty.status();
    print!("\x1b[?1049l\x1b[?25h\x1b[0m");
    let _ = std::io::stdout().flush();
}

/// Ignore SIGINT in the driver so a terminal Ctrl-C does not kill the loop. The
/// driver must survive the interrupt to reach the relaunch-vs-stop decision.
///
/// **This covers the gaps between sessions, and only those.** While a session
/// is running it is the terminal's foreground process group in its own right —
/// `ravel_job::launch` puts it there and hands it the terminal — so a typed Ctrl-C is
/// delivered to the session and never reaches the driver at all. This ignore is
/// what holds in the moments the driver is transitioning the tree and selecting
/// a leaf with no child in front of it.
///
/// **It does not leak into the session.** An ignored disposition is the one
/// kind that survives `execve`, which is exactly why this used to reach the
/// session, everything it spawned, and every wrapper in front of it — a login
/// shell that inherits an ignored SIGINT keeps ignoring it and
/// passes it on, so Ctrl-C did nothing at all and nothing in the session could
/// say why. `ravel_job::launch` now resets the child's dispositions to their
/// defaults across the spawn, so this stays the driver's own policy rather than
/// the subtree's.
///
/// SIGINT and SIGINT alone, because it is the one disposition that is a
/// *policy* rather than a mechanism: what a loop does about the human's Ctrl-C
/// is the loop's business. SIGTERM and SIGHUP belong to `ravel_job::launch`,
/// which catches them itself so it can forward one to `ravel` and reap it rather
/// than orphan it onto the terminal — and reports that, with the signal, as
/// [`JobEnd::interrupted`].
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
    use std::time::Duration;

    fn selected_root_fixture() -> tempfile::TempDir {
        let temp = tempfile::tempdir().unwrap();
        std::fs::create_dir(temp.path().join(".jj")).unwrap();
        std::fs::create_dir(temp.path().join(".grove")).unwrap();
        std::fs::write(temp.path().join(".grove/_BRIEF.md"), "brief").unwrap();
        std::fs::write(temp.path().join(".grove/01-impl--work-k1.md"), "task").unwrap();
        temp
    }

    /// The loop's own `harness-ravel run` for `task`, under a HOME whose
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
        let run = crate::session_command::lifecycle(
            &support::installed_ravel(),
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
    fn launch_reading_precedence_over_interrupts() {
        use std::os::unix::process::ExitStatusExt;
        let selection = Selection {
            path: "01-impl--work-k1.md".into(),
            handle: Handle::parse("work-k1").unwrap(),
            kind: Kind::new("impl").unwrap(),
        };
        for (interrupted, teardown, supported_result, want) in [
            (
                Some(libc::SIGTERM),
                true,
                true,
                Some(LoopOutcome::Interrupted(libc::SIGTERM)),
            ),
            (
                Some(libc::SIGHUP),
                true,
                true,
                Some(LoopOutcome::Interrupted(libc::SIGHUP)),
            ),
            (None, true, true, Some(LoopOutcome::Finished)),
            (None, true, false, Some(LoopOutcome::Finished)),
            (None, false, true, None),
            (None, false, false, Some(LoopOutcome::Stopped)),
        ] {
            let ended = JobEnd {
                status: std::process::ExitStatus::from_raw(0),
                interrupted,
                elapsed: Duration::ZERO,
            };
            let reading = crate::launch_directory::LaunchReading { teardown };
            assert_eq!(
                interpret_launch(&ended, &reading, &selection, || {
                    if supported_result {
                        Ok(SessionDisposition::Continue)
                    } else {
                        Err(anyhow::anyhow!("missing result").into())
                    }
                }),
                want,
                "{interrupted:?}, teardown={teardown}, supported_result={supported_result}"
            );
        }
    }

    #[test]
    fn force_stop_precedence_requires_a_successful_uninterrupted_supervisor() {
        use std::cell::Cell;
        use std::os::unix::process::ExitStatusExt;
        let selection = Selection {
            path: "01-impl--work-k1.md".into(),
            handle: Handle::parse("work-k1").unwrap(),
            kind: Kind::new("impl").unwrap(),
        };
        for teardown in [false, true] {
            for (interrupted, status, want, reads) in [
                (None, 0, Some(LoopOutcome::ForceStopped), 1),
                (
                    None,
                    256,
                    Some(if teardown {
                        LoopOutcome::Finished
                    } else {
                        LoopOutcome::Stopped
                    }),
                    0,
                ),
                (
                    Some(libc::SIGTERM),
                    0,
                    Some(LoopOutcome::Interrupted(libc::SIGTERM)),
                    0,
                ),
                (
                    Some(libc::SIGHUP),
                    256,
                    Some(LoopOutcome::Interrupted(libc::SIGHUP)),
                    0,
                ),
            ] {
                let ended = JobEnd {
                    status: std::process::ExitStatus::from_raw(status),
                    interrupted,
                    elapsed: Duration::ZERO,
                };
                let reading = crate::launch_directory::LaunchReading { teardown };
                let read_count = Cell::new(0);
                let got = interpret_launch(&ended, &reading, &selection, || {
                    read_count.set(read_count.get() + 1);
                    Ok(SessionDisposition::ForceStop)
                });
                assert_eq!(got, want, "{interrupted:?}, {status}, teardown={teardown}");
                assert_eq!(read_count.get(), reads);
            }
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
            let capture = crate::session_result::SessionResult::allocate().unwrap();
            let result = launch_session(
                &argv,
                selection,
                work,
                launch_dir.path(),
                &mut lease,
                capture.stdout().unwrap(),
            );
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

    /// The `complete_post_reap_epoch_handoff` cases below drive that ordering
    /// with a result value that Grove owns, because what the ordering is *about*
    /// is which failure survives and whether the caught signal survives with it,
    /// not what a session left behind. The launch itself is `ravel_job::launch`'s,
    /// and `tests/ravel_job.rs` drives it against a fake child.
    fn job_end(interrupted: Option<i32>) -> JobEnd {
        use std::os::unix::process::ExitStatusExt;
        JobEnd {
            status: std::process::ExitStatus::from_raw(0),
            interrupted,
            elapsed: Duration::ZERO,
        }
    }

    /// A launch error as `launch_session` returns it: a `JobError` under the
    /// context that names the session kind.
    fn launch_error(interrupted: Option<i32>) -> anyhow::Error {
        anyhow::Error::new(ravel_job::JobError::for_test(
            "executable was not found",
            interrupted,
        ))
        .context("launching session kind `impl`")
    }

    #[test]
    fn an_epoch_handoff_failure_preserves_the_launch_failure_that_preceded_it() {
        let continuation_called = std::cell::Cell::new(false);

        let error = complete_post_reap_epoch_handoff(
            Err(launch_error(None)),
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
        assert_eq!(caught_signal(&error), None);
        assert!(
            !continuation_called.get(),
            "signal interpretation must remain behind successful epoch invalidation"
        );
    }

    #[test]
    fn signal_interpretation_cannot_run_before_epoch_invalidation_succeeds() {
        let continuation_called = std::cell::Cell::new(false);

        let error = complete_post_reap_epoch_handoff(
            Ok(job_end(None)),
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
        assert_eq!(caught_signal(&error), None);
        assert!(
            !continuation_called.get(),
            "signal interpretation must remain behind successful epoch invalidation"
        );
    }

    #[test]
    fn a_launch_error_keeps_the_signal_it_carries() {
        for signal in [libc::SIGTERM, libc::SIGHUP] {
            let error = complete_post_reap_epoch_handoff(
                Err(launch_error(Some(signal))),
                || Ok(()),
                |_| (),
            )
            .unwrap_err();
            assert!(format!("{error:#}").contains("executable was not found"));
            assert_eq!(caught_signal(&error), Some(signal));
        }
    }

    #[test]
    fn a_failed_invalidation_after_an_interrupted_job_keeps_the_signal() {
        for signal in [libc::SIGTERM, libc::SIGHUP] {
            let error = complete_post_reap_epoch_handoff(
                Ok(job_end(Some(signal))),
                || Err(anyhow::anyhow!("exclusive epoch handoff timed out")),
                |_| (),
            )
            .unwrap_err();
            assert!(format!("{error:#}").contains("exclusive epoch handoff timed out"));
            assert_eq!(caught_signal(&error), Some(signal));
        }
    }

    #[test]
    fn a_launch_error_and_a_failed_invalidation_keep_the_launch_signal_as_a_type() {
        let error = complete_post_reap_epoch_handoff(
            Err(launch_error(Some(libc::SIGHUP))),
            || Err(anyhow::anyhow!("exclusive epoch handoff timed out")),
            |_| (),
        )
        .unwrap_err();
        let message = format!("{error:#}");
        assert!(message.contains("executable was not found"), "{message}");
        assert!(
            message.contains("exclusive epoch handoff timed out"),
            "{message}"
        );
        assert_eq!(caught_signal(&error), Some(libc::SIGHUP));
    }
}
