// The self-driving loop — grove's runtime (self-driving-loop).
//
// Bare `grove` drives the *whole loop*, not one task: it launches a fresh
// foreground session per grove task and relaunches with fresh context each time
// the agent fires the completion signal (`grove-llm complete`). Any other exit —
// human `/exit`/Ctrl-C, or a crash — stops the loop, resumable later by
// re-running `grove` from the same working tree (restart ≡ continuation, the
// loop body holds zero state and re-derives position from the tree).
//
// The configured command is spawned directly — no shell, no PID-export trick —
// and watched while it runs: poll it alongside the completion-signal file, and
// once the file appears, apply grace → SIGTERM → kill-grace → SIGKILL to the
// child itself (driver-side watcher — self-driving-loop). The driver is the
// session's own parent process, outside whatever sandbox the session runs
// under, so it can always signal its child — unlike the in-agent self-kill this
// replaces, which codex's Seatbelt sandbox silently denied.
//
// **All of that is `crates/keyed-launch`'s, not this module's.** What stays here
// is the four things a loop has to choose and a runner cannot: which directory
// the channel is allocated in, which variable publishes it, which variables are
// scrubbed, and how long the two graces are. The shell sketch below is still the
// whole loop, because a boundary is not a step.
//
// The driver is deliberately tiny — a plain shell `while` loop could stand in
// (constraint 6, walk-away-able). Nothing below infers anything about the
// session: the selected leaf's filename kind indexes one complete-config entry,
// and that entry's argv is the launch in full.
//
//     # after owning the workspace lease, clean abandoned signal-<128-bit> paths
//     while :; do
//       grove_recover_or_migrate_tree                    # driver-only transition
//       # One in-process selection: the leaf's stable handle *and* its kind.
//       read -r handle kind <<<"$(grove_select_or_materialize_finish)"
//       # The kind indexes the config; there is no default, family, or fallback.
//       argv=$(kdl_lookup "$HOME/.config/grove/config.kdl" "$kind")
//       # Draw a fresh OS-random 128-bit suffix in the workspace control dir;
//       # retry occupied names without touching their contents.
//       sig="$control_dir/signal-<fresh-128-bit-suffix>"
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

use crate::driver_lease::DriverLease;
use crate::session_config::{DeltaRoots, ExpansionContext, SessionConfig};
use anyhow::{Context, Result};
use grove_loop::{interpret, Disposition, Reading, Selection, Sought};
use grove_loop::{Handle, Kind};
use jj_workspace::Workspace;
use keyed_launch::{Argv, Channel, End, Ended, Escalation, Launch};
use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// Drive the config-defined lifecycle from the current working tree. This is
/// the sole path reached by the human-facing bare command: acquire the
/// workspace lease, and run one configured foreground session per selected task
/// until the agent stops signalling.
///
/// Nothing here inspects the working tree for a harness, and nothing chooses a
/// binary: the configured argv is the whole of launch policy. Nothing here
/// delivers the methodology either — since `delete-provisioning-k19` the
/// methodology is a plugin a human installs, so the driver's first act is the
/// lease rather than a sweep over three personal skill directories.
pub fn bare_grove() -> Result<()> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    let driver_lease = DriverLease::acquire(&cwd)?;
    let worktree = driver_lease.worktree_root().to_path_buf();
    let repository = driver_lease.main_repo().to_path_buf();
    let name = worktree_name(&worktree);

    run_configured(&repository, &worktree, &name, driver_lease)
}

/// The grove name is the worktree directory's basename (user-owned-worktrees).
fn worktree_name(worktree: &Path) -> String {
    worktree
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string())
}

/// The loop driver's **launch-scoped environment** (self-driving-loop) — the
/// variables a descendant could act on, and the exact set every spawn below
/// hands to `keyed_launch` as its scrub list.
///
/// `GROVE_SIGNAL_FILE` is the completion channel: the runner watches that path
/// while its child runs and applies grace → SIGTERM → kill-grace → SIGKILL the
/// moment the file *appears*. Whoever holds the variable can therefore end the
/// session, and the environment is inherited by every descendant — so the
/// authority is ambient unless each spawn scopes it deliberately.
/// `GROVE_HARNESS_PID` / `GROVE_CLAUDE_PID` are the retired pre-watcher handles
/// (driver-side-kill), kept here because a stale, unrelated PID leaking into a
/// nested grove is the same class of mistake one notch quieter — the value is
/// something a reader could still *act on*. That is the bar for membership.
///
/// **Any spawn that is not the configured session itself must scrub this whole
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
/// Grove's own spawns are exactly two — the configured session and `stty sane`
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
const LOOP_CONTROL_ENV: [&str; 3] = ["GROVE_SIGNAL_FILE", "GROVE_HARNESS_PID", "GROVE_CLAUDE_PID"];

/// The variable the completion channel's path is published under — the name
/// this build and `grove-llm complete` have agreed on. It is the runner's
/// `channel_var`, and it is the first entry of [`LOOP_CONTROL_ENV`] because
/// granting it is exactly the exception scrubbing exists to carve out.
const CHANNEL_VAR: &str = "GROVE_SIGNAL_FILE";

/// [`LOOP_CONTROL_ENV`] as the runner takes it.
fn scrub_list() -> [&'static OsStr; LOOP_CONTROL_ENV.len()] {
    LOOP_CONTROL_ENV.map(OsStr::new)
}

/// Deliberately one helper rather than an `env_remove` per site: the list is the
/// interesting part, and a second site open-coding it is how the first one came
/// to be missed. The configured session goes through
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
    /// The grove finished cleanly: a session signalled `complete --done`.
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
}

/// Entry point for the bare config-driven lifecycle.
pub fn run_configured(
    repo_path: &Path,
    worktree: &Path,
    name: &str,
    driver_lease: DriverLease,
) -> Result<()> {
    ignore_interrupts();
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
    // The two roots the configuration delta is searched at, taken from the
    // resolution that already happened rather than recomputed here: `repo_path`
    // is the seam's `main_repo` answer and the very value `${repo}` expands to,
    // so the search order cannot drift from the template it selects
    // (`docs/adr/untracked-configuration-delta.md`).
    let delta_roots = DeltaRoots {
        worktree,
        repository: repo_path,
    };
    let repo_name = repo_path
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "repo".to_string());
    let session_name = format!("{repo_name}: {name} grove");

    loop {
        // A SIGTERM or SIGHUP that arrived while no session was running has no
        // launch to be reported against, so the runner discards it rather than
        // spending it on the next child. Collecting it here is what keeps the
        // driver from going on mutating the tree and taking commits after its
        // terminal has gone.
        if keyed_launch::take_interrupt() {
            eprintln!("grove: interrupted between sessions — stopping the loop.");
            return Ok(LoopOutcome::Stopped);
        }
        driver_lease
            .revalidate()
            .context("revalidating driver lease before loop transition")?;
        let pre_transition_config = SessionConfig::load(&home, &delta_roots)?;

        grove_loop::driver::transition_to_current(worktree)?;
        let selection = match picked(worktree)? {
            Sought::Match(selection) => selection,
            Sought::Nothing => {
                // The finish sentinel is a leaf grove writes itself, so the
                // just-in-time presence rule binds it exactly as it binds
                // `leaf-add` — before the write, not at the launch that follows
                // (`docs/adr/complete-session-configuration.md`). Asked against
                // the pre-transition load, which is the document as it stood
                // before anything was mutated.
                pre_transition_config
                    .require(Kind::finish().label())
                    .context("materializing the driver-owned finish leaf")?;
                grove_loop::driver::materialize_finish(worktree)?
            }
        };

        let config = SessionConfig::load(&home, &delta_roots)?;
        // The file this kind actually resolved from — the personal file, or the
        // delta that overrode it. Every diagnostic below names *that*, because
        // naming the personal file for a delta-supplied kind points a reader at
        // a file which never held the failing template.
        let resolved_source = config
            .source(selection.kind.label())
            .unwrap_or(config_path.as_path())
            .to_path_buf();
        let prompt = session_prompt(&selection.handle, &selection.kind, worktree)?;
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
        let channel = Channel::allocate(driver_lease.control_dir())
            .context("allocating a fresh foreground-session signal channel")?;
        let ended = launch_configured_session(
            &argv,
            &selection,
            &resolved_source,
            worktree,
            &channel,
            driver_lease,
        );
        // Unconditionally, and before the invalidation gate below: the session
        // may have left the terminal in raw mode and on the alternate screen,
        // and an error path that returns without restoring it hands the human
        // an unusable shell to read the error in. Restoring is not
        // interpretation, so it is not what the gate is protecting.
        reset_terminal();
        let (ended, signal) = complete_post_reap_epoch_handoff(
            ended,
            || driver_lease.invalidate_session_epoch(),
            |ended: Ended| {
                let signal = interpret(ended.token.as_ref());
                (ended, signal)
            },
        )?;

        if let Err(error) = channel.discard() {
            eprintln!(
                "grove: warning: could not remove the interpreted foreground-session signal channel; preserving the session outcome: {error}"
            );
        }

        if ended.end == End::Interrupted {
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
                        argv.program(),
                        resolved_source.display()
                    );
                }
                return Ok(LoopOutcome::Stopped);
            }
        }
    }
}

/// The whole `${prompt}`: the guaranteed core, composed for the launched kind.
///
/// **The driver hands a session a pointer, not the methodology.** What earns a
/// place, and why the facts below arrive as bare values with no normative tail,
/// is [`crate::prompt`]'s to state and this function's to supply: the selected
/// leaf's stable handle, the resolved workspace the prompt states the version
/// control from, and grove's own published release version.
///
/// The kind is passed in rather than re-read: it is the same value that indexed
/// the configuration entry, taken from the one guarded selection, so the prompt
/// and the command a session receives cannot disagree about what kind it is.
///
/// **The version is `CARGO_PKG_VERSION`, read here rather than in `prompt`**, so
/// composition takes a value like every other runtime fact and the module that
/// composes text does not also decide what build it is part of. It is the same
/// value `grove --version` renders — clap derives that from this constant — which
/// is what makes the flag a fallback for the published fact rather than a second
/// source of it (`docs/specs/module-decomposition.md`, decision 10).
///
/// Resolving the workspace is the one thing here that can fail, and it is
/// unreachable in a driver that got this far: the lease it holds lives *in* the
/// workspace's own `.jj/`, so a marker was already found. Spent as an error
/// rather than a panic — the prompt has no second case to express, and a driver
/// with nothing to say here must not launch a session that then has to guess.
fn session_prompt(handle: &Handle, kind: &Kind, worktree: &Path) -> Result<String> {
    // Taken from the resolution rather than assumed. It *is* `worktree` — the
    // walk starts at the path itself and the lease root is the marker's own
    // directory — but reading the resolved root cannot drift if either end
    // moves.
    let workspace = Workspace::resolve(worktree)
        .context("the session prompt cannot state the version control")?;
    Ok(crate::prompt::compose(&crate::prompt::Mandate {
        handle,
        kind,
        workspace: &workspace,
        version: crate::VERSION,
    }))
}

/// Launch one fresh foreground session owning the real TTY, and hand it to
/// `keyed_launch::run`, which spawns it directly — no shell — and supervises it
/// until it ends.
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
///
/// A spawn failure names `resolved_source` — the file this kind's template was
/// actually read from, personal or delta — rather than the personal path
/// unconditionally, which would name a file that never held the failing
/// template (`docs/adr/untracked-configuration-delta.md`). The runner's own
/// message names the program and says to check that it is executable; grove
/// adds the two things only grove knows, the kind and the file that supplied
/// it.
///
/// The epoch is activated **before** the spawn and never after: a child that is
/// already running under an inactive epoch would have its own `grove-llm` verbs
/// refused.
fn launch_configured_session(
    argv: &Argv,
    selection: &grove_loop::Selection,
    resolved_source: &Path,
    worktree: &Path,
    channel: &Channel,
    driver_lease: &DriverLease,
) -> Result<Ended> {
    eprintln!(
        "grove: launching {} with configured {:?} — {}",
        selection.kind.label(),
        argv.program(),
        selection.handle
    );

    driver_lease
        .activate_session_epoch(channel.path())
        .context("activating the foreground session epoch before spawn")?;

    keyed_launch::run(Launch {
        argv,
        channel,
        channel_var: CHANNEL_VAR,
        scrub: &scrub_list(),
        cwd: Some(worktree),
        escalation: ESCALATION,
    })
    .with_context(|| {
        format!(
            "launching configured session kind `{}` via {:?} from {}",
            selection.kind.label(),
            argv.program(),
            resolved_source.display()
        )
    })
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

/// The kill escalation the runner applies once the completion channel appears.
///
/// Built-in constants, not knobs. Two seconds lets the agent's `complete` tool
/// call return and its turn end before its session dies; five more is time for
/// an orderly SIGTERM shutdown before SIGKILL. Why an escalation is needed at
/// all — an interactive session is never reaped on its own, and cannot be
/// trusted to end itself under every sandbox — is `keyed_launch::Escalation`'s
/// to state, and it states it.
const ESCALATION: Escalation = Escalation {
    grace: Duration::from_secs(2),
    kill_grace: Duration::from_secs(5),
};

/// Reset the terminal after a (possibly SIGTERM'd) TUI: restore cooked mode,
/// leave the alternate screen, show the cursor. No-op when stdin isn't a TTY
/// (headless / test runs).
fn reset_terminal() {
    if unsafe { libc::isatty(libc::STDIN_FILENO) } != 1 {
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

/// Ignore SIGINT in the driver so a terminal Ctrl-C (delivered to the whole
/// foreground process group) does not kill the loop; the child session
/// installs its own handler and still responds. The driver must survive the
/// interrupt to reach the relaunch-vs-stop decision.
///
/// SIGINT and SIGINT alone, because it is the one disposition that is a
/// *policy* rather than a mechanism: what a loop does about the human's Ctrl-C
/// is the loop's business. SIGTERM and SIGHUP belong to the runner, which
/// catches them itself so it can forward one to its child and reap it rather
/// than orphan it onto the terminal — and reports that as
/// [`keyed_launch::End::Interrupted`].
fn ignore_interrupts() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }
}

/// The driver's own `pick`, over the worktree it is driving.
///
/// The transition above has already brought the worktree to a grove, so the
/// vacant arm is unreachable in practice — but it is an arm of
/// [`grove_loop::read`], and answering it as *no live leaves* is the same thing
/// the transition would have made true a moment earlier.
fn picked(worktree: &Path) -> anyhow::Result<Sought<Selection>> {
    match grove_loop::read(worktree)? {
        Reading::Tree(tree) => Ok(grove_loop::verbs::pick(&tree)?),
        Reading::Vacant => Ok(Sought::Nothing),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            Ok("a session that ended"),
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
}
