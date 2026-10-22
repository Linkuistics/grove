# The loop
<!-- book-page id="the-loop" slice="four-things-a-runner-cannot-choose" order="19" -->
[Previous: The guaranteed core](18-the-core.md) | [Contents](README.md) | [Next: What could not move](20-what-could-not-move.md)

<a id="four-things-a-runner-cannot-choose"></a>
## The rule: Ravel owns the harness's end

Grove owns the task tree and the loop decision. Ravel owns one harness run.
It selects the command and spawns the harness as its own job. It watches the
exit channel, ends the group and returns the opaque published argument.
Grove's runner supervises Ravel as a channel-free job. This separation lets
another caller use Ravel without implementing a harness watcher.

For a selected `impl` leaf, `drive` composes the prompt, allocates a launch
directory, and captures Ravel's public stdout in a held unlinked file. The
prompt supplies the exact selected executable's continue and force-stop
command. Ravel reaps its managed group and returns that result separately from
managed output. Grove restores the terminal and invalidates its session epoch
before checking teardown and interpreting the result.

<a id="restart-is-continuation"></a>
## Restart is continuation, and a boundary is not a step

Each iteration transitions the tree and picks its next live leaf anew. No
cursor survives a stop, so rerunning Grove continues from the tree's filenames.
The caller acquires the workspace lease before entering `run`.

<a id="what-the-instruments-see"></a>
## Both blind spots, on one root

The module header and inline tests are source fragments even though rustdoc
cannot inspect those regions. The book validator reconstructs their bytes as
well as the production functions. Process behavior is separate evidence: the
loop-driver PTY suite runs the real Grove binary, Ravel front and worker.

<a id="the-harness-widened"></a>
## The harness this chapter had to widen, and the direction the error runs

The fake harness can send Ravel's public result, exit by itself, or leave a
raw-mode orphan after Ravel is killed. The PTY cases observe terminal recovery
before `reset_terminal` changes the saved modes. A background driver is launched
beneath a shell that retains the foreground. Creator lifecycle cases use an explicit caller-owned policy and retain
retirement, reordering, cascade, missing-provenance refusal and declaration
resume behavior without a Ravel store or embedded Grove adapter.

<a id="the-block-declared"></a>
## The block, declared

The composite below connects the loop's source fragments in production order.
The definitions follow the reader's path from caller data through launch to
interpretation. The root still includes the inline tests.

<!-- fragment «loop-driver» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="1-918" parent="source-loop-driver" -->
<!-- insert «loop-header» -->
<!-- insert «loop-imports» -->
<!-- insert «loop-control-env» -->
<!-- insert «loop-scrub-list» -->
<!-- insert «loop-scrub-helper» -->
<!-- insert «loop-outcome» -->
<!-- insert «loop-run» -->
<!-- insert «loop-drive-open» -->
<!-- insert «loop-drive-interrupt» -->
<!-- insert «loop-drive-selection» -->
<!-- insert «loop-drive-invocation» -->
<!-- insert «loop-drive-launch» -->
<!-- insert «loop-drive-discard» -->
<!-- insert «loop-drive-survivor» -->
<!-- insert «loop-drive-endings» -->
<!-- insert «loop-session-prompt» -->
<!-- insert «loop-launch-contract» -->
<!-- insert «loop-launch-spawn» -->
<!-- insert «loop-handoff» -->
<!-- insert «loop-escalation» -->
<!-- insert «loop-reset-terminal» -->
<!-- insert «loop-ignore-interrupts» -->
<!-- insert «loop-picked» -->
<!-- insert «loop-tests-open» -->
<!-- insert «loop-test-handoff-preserves» -->
<!-- insert «loop-test-ordering» -->
<!-- /fragment -->

<a id="the-header"></a>
## The header, and the ownership boundary

The header assigns each effect to its owner. The driver receives the workspace,
the lease and the Ravel path. It derives the selection and allocates the control
area. A harness that exits by itself leaves its leaf live.

<!-- fragment «loop-header» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="1-12" parent="loop-driver" -->
````rust
// The self-driving loop: select a leaf, compose its prompt, run ravel,
// invalidate the reaped session's epoch, then interpret the launch.
//
// Ravel alone watches the harness's exit channel and ends its group. Grove
// launches ravel as a channel-free job; its runner forwards cancellation,
// reaps ravel and restores the terminal, including after ravel's death
// left an orphaned harness holding the foreground. The driver then resets it.
//
// Restart is continuation: each iteration derives its position from the tree.
// The caller owns the workspace lease; this module owns the launch directory,
// prompt, argv and ordered reading of the ending and teardown record.

````
<!-- /fragment -->

<a id="what-it-imports"></a>
## The loop's vocabulary and runner types

Tree vocabulary belongs to Grove. `Launch`, `Ended`, `End` and `Group`
belong to its runner. `Argv` belongs to `session_command`, and `launch_session`
converts it for the runner. The loop reads a held stdout result separately from
`LaunchDirectory`. This keeps JSON interpretation apart from the loop's selection
and terminal reset.

<!-- fragment «loop-imports» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="13-27" parent="loop-driver" -->
````rust
use crate::driver_lease::DriverLease;
use crate::session_command::Argv;
use crate::session_result::SessionDisposition;
use crate::{Handle, Kind, Reading, Selection, Sought, TreeLifetime};
use anyhow::{ensure, Context, Result};
use jj_workspace::Workspace;
use keyed_launch::{End, Ended, Escalation, Group, Launch};
use std::ffi::OsStr;
#[cfg(test)]
use std::ffi::OsString;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

````
<!-- /fragment -->

<a id="the-third-choice"></a>
## The third choice: what a child may not inherit

`LOOP_CONTROL_ENV` names inherited authority that must be removed from every
spawn. The current Ravel launch receives only its own `GROVE_LAUNCH_DIR`
after scrubbing. Ravel grants its fresh exit channel to the harness. The
Ravel job authority and `GROVE_LAUNCH_DIR` are scrubbed in nested launches.
The list also removes Ravel's inherited ending context, the retired standalone
channel, old dispatch run/store identity and retired PID handles. It also
scrubs `GROVE_PROVIDER_EVIDENCE`: only the current owner policy wrapper may
provide fresh evidence to the managed process. This guard
guards both active Ravel callers, including the version probe. The helper gives
`stty` the same scrub list so resetting the terminal carries no completion
authority. The real lifecycle fixture
`lifecycle_scrubs_enclosing_ravel_and_retired_controls_before_selection` grants
each reserved control at process entry and allowed controls through policy:
a direct dirty probe observes every injected name, while the Grove launch removes them and completes.

<!-- fragment «loop-control-env» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="28-77" parent="loop-driver" -->
````rust
/// The loop driver's **launch-scoped environment** (self-driving-loop) — the
/// variables a descendant could act on, and the exact set every spawn below
/// hands to `keyed_launch` as its scrub list.
///
/// `GROVE_SIGNAL_FILE` and `GROVE_RUN_SIGNAL_FILE` are retired completion
/// authority, retained for installed v22 sessions. `GROVE_LAUNCH_DIR` identifies
/// the admitted session. Dispatch and Ravel exit contexts, and dispatch's old
/// run/store context, are removed before the current launch is granted.
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
/// This loop's own spawns are two — the session and `stty sane`
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
const LOOP_CONTROL_ENV: [&str; 10] = [
    "GROVE_SIGNAL_FILE",
    "GROVE_RUN_SIGNAL_FILE",
    "GROVE_LAUNCH_DIR",
    "HARNESS_DISPATCH_EXIT_FILE",
    "HARNESS_DISPATCH_RUN_ID",
    "HARNESS_DISPATCH_STATE_DIR",
    "RAVEL_EXIT_FILE",
    "GROVE_PROVIDER_EVIDENCE",
    "GROVE_HARNESS_PID",
    "GROVE_CLAUDE_PID",
];

````
<!-- /fragment -->

<a id="the-second-choice"></a>
## The launch identity and Ravel's ephemeral context

Grove publishes launch identity rather than an exit channel. The directory is
bound into the session epoch before the spawn. So the epoch admits only the tree
verbs of that launch. Ravel's `--exit-dir` chooses where its own fresh channel
can be written. The public stdout descriptor returns the opaque result. Neither
reaches policy.

<!-- fragment «loop-scrub-list» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="78-82" parent="loop-driver" -->
````rust
/// [`LOOP_CONTROL_ENV`] as the runner takes it.
fn scrub_list() -> [&'static OsStr; LOOP_CONTROL_ENV.len()] {
    LOOP_CONTROL_ENV.map(OsStr::new)
}

````
<!-- /fragment -->

`scrub_loop_control_env` applies the same names to the terminal-reset command. A helper spawn receives no launch authority.

<!-- fragment «loop-scrub-helper» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="83-93" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="how-a-loop-ends"></a>
## Three ways a loop ends, and why the third is not the second

`LoopOutcome` distinguishes a recorded finish, a resumable stop, and the driver's
own interruption. It keeps the signal number. So the human-facing binary can
re-raise the signal, and its parent can distinguish cancellation from success.

<!-- fragment «loop-outcome» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="94-125" parent="loop-driver" -->
````rust
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
    /// [`keyed_launch::reraise`] and the parent sees `128 + N` instead.
    Interrupted(i32),
}

````
<!-- /fragment -->

<a id="run"></a>
## `run`: the three things a loop cannot derive

`run` takes the workspace, lease by value and once-resolved installed Ravel path.
It ignores driver SIGINT between sessions and delegates to `drive`. Taking the
lease by value releases ownership when the loop returns. A policy refusal is a
stopped launch, while failure to spawn Ravel returns an error.

<!-- fragment «loop-run» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="126-163" parent="loop-driver" -->
````rust
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
pub fn run(
    workspace: &Workspace,
    mut lease: DriverLease,
    ravel: &Path,
) -> Result<LoopOutcome, crate::Error> {
    ignore_interrupts();
    Ok(drive(workspace, &mut lease, ravel)?)
}

````
<!-- /fragment -->

<a id="the-loop-body"></a>
## The loop body: launch, invalidate, interpret

`drive` revalidates the lease, transitions the tree, and obtains one selection.
It then composes the prompt and allocates a fresh launch directory. The runner
forwards driver TERM/HUP to Ravel and waits up to ten seconds before killing
Ravel's group. The budget allows Ravel to cancel and reap its managed group.
It promises no wall-clock bound under suspension or slow I/O.

After the reap, terminal reset happens even on an error path. Epoch invalidation
must then succeed. Only after that does the driver interpret a launch record.
The driver removes the directory only after reading. If the removal fails, the
driver reports a warning and keeps the outcome. The driver checks for
interruption again after the reset and the invalidation, because the runner's
ending cannot include signals that arrived after the reap.

First match wins: driver interruption, successful exact force-stop, teardown,
surviving group or failed front status, then exact continuation or stop. A
surviving Ravel group prevents relaunch. A Ravel that died of a signal can have
left its harness alive. So the stop names that possibility. It does not treat
missing output as completion.

<!-- fragment «loop-drive-open» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="164-173" parent="loop-driver" -->
````rust
fn drive(
    workspace: &Workspace,
    driver_lease: &mut DriverLease,
    ravel: &Path,
) -> Result<LoopOutcome> {
    // Taken from the lease rather than recomputed: ravel runs here, which is
    // the location the prompt assumes and the one a policy reads as its `cwd`.
    let worktree_path = driver_lease.worktree_root().to_path_buf();
    let worktree = worktree_path.as_path();

````
<!-- /fragment -->

The top-of-loop interrupt check consumes cancellation that arrived between launches, before Grove mutates the tree or launches another session.

<!-- fragment «loop-drive-interrupt» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="174-188" parent="loop-driver" -->
````rust
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
````
<!-- /fragment -->

Lease validation precedes transition and selection. The selected lifetime and kind remain the values that the launch will use.

<!-- fragment «loop-drive-selection» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="189-197" parent="loop-driver" -->
````rust

        // No kind is checked here, or anywhere before the launch: whether the
        // owner's policy routes one is asked when its leaf launches.
        crate::driver::transition_to_current(worktree)?;
        let selected = match picked(worktree)? {
            Sought::Match(selection) => selection,
            Sought::Nothing => picked_after_finish(worktree)?,
        };
        let selection = selected.selection.clone();
````
<!-- /fragment -->

<a id="no-kind-is-asked"></a>
## No kind is asked about before its leaf launches

The tree can hold an unrouted kind. Grove asks the owner's policy only at launch;
Ravel's refusal and remedy reach the terminal and the selected leaf remains
live. No route table or owner configuration is read by this module.

<a id="the-selection"></a>
## What `Sought::Nothing` means here

`Sought::Nothing` causes the driver to materialize a finish leaf and select again
under a fresh guard. The retained `TreeLifetime` pins the selected root's
identity without retaining its tree lock across a session.

<!-- fragment «loop-drive-invocation» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="198-208" parent="loop-driver" -->
````rust

        let ending = crate::session_command::end_session(ravel)?;
        let force_stop = crate::session_command::force_stop(ravel)?;
        let prompt = session_prompt(
            &selection.handle,
            &selection.kind,
            workspace,
            &ending,
            &force_stop,
        );

````
<!-- /fragment -->

The launch directory is allocated after lease validation. Its path becomes both session identity and the directory Ravel uses for its run mechanics.

<!-- fragment «loop-drive-launch» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="209-238" parent="loop-driver" -->
````rust
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
            |ended: Ended| {
                let reading = launch_dir.read();
                (ended, reading)
            },
        )?;

````
<!-- /fragment -->

After runner recovery, reset happens unconditionally. Epoch invalidation gates
the reading closure. Directory removal preserves the interpreted outcome even if
cleanup fails. The driver then calls `interpret_launch`. Its first matching
branch either returns a terminal outcome or returns `None` to relaunch.

Interruption wins over every disposition. Successful exact force-stop wins over
teardown, and teardown otherwise wins over a surviving Ravel group. The driver
keeps the branch body at the runner-result seam. So a test can exercise the
survivor precedence without an unkillable process fixture.

<!-- fragment «loop-drive-discard» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="239-286" parent="loop-driver" -->
````rust
        if let Err(error) = launch_dir.discard() {
            eprintln!(
                "grove: warning: could not remove the interpreted foreground-session launch directory; preserving the session outcome: {error}"
            );
        }

        // TERM/HUP during terminal reset or invalidation arrived after the
        // runner froze Ended. Honour that latch before accepting a disposition.
        if let Some(signal) = keyed_launch::take_interrupt() {
            eprintln!("grove: interrupted by signal {signal} after reap — stopping the loop.");
            return Ok(LoopOutcome::Interrupted(signal));
        }

        if let Some(outcome) = interpret_launch(&ended, &reading, &selection, || capture.read()) {
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
    read_result: impl FnOnce() -> std::result::Result<SessionDisposition, crate::Error>,
) -> Option<LoopOutcome> {
    if let End::Interrupted { signal } = ended.end {
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

    // Teardown otherwise retains its original authority over status, results
    // and surviving groups. Driver interruption and explicit force-stop win.
    if reading.teardown {
        eprintln!("grove: grove finished — loop complete.");
        return Some(LoopOutcome::Finished);
    }

````
<!-- /fragment -->

A remaining Ravel group is reported with the selected handle. A relaunch beside a survivor would let two processes keep changing the tree.

<!-- fragment «loop-drive-survivor» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="287-299" parent="loop-driver" -->
````rust
    // Before the ending is acted on, and whatever it says: a member of
    // ravel's group that survived the runner's kills may still hold the
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

````
<!-- /fragment -->

The final branch relaunches only on the supported exact `grove-continue` result. Unsupported, missing or unreadable results stop, with Ravel death identified as a possible surviving harness.

<!-- fragment «loop-drive-endings» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="300-326" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="session-prompt"></a>
## `session_prompt`: a pointer, not the methodology

The helper renders the selected handle, kind, workspace and published version
through the prompt module. The same selection supplies Ravel's native kind,
task path and handle, so the mandate and policy inputs cannot name different
leaves. The signalling contract now names Ravel's end-session command and Grove's
separate teardown record and exact caller-owned result.

<!-- fragment «loop-session-prompt» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="327-368" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="dispatch-run"></a>
## Shared public command construction

The loop calls `session_command::lifecycle` with the same selection that
composed its prompt and the once-selected executable. Kind, prompt, writable
exit directory and task data are public inputs. It passes no private report flag.
The shared module, explained below, also constructs standalone confinement
inputs and quotes the exact ending command for either caller.

<a id="launch"></a>
## `launch_session`: a channel-free supervisor job

`launch_session` activates the selected root and launch identity before the
spawn. `supervise_launch` carries the witness across the spawn and reap
events of the runner. The runner receives `channel: None`, the scrub list and
only the launch-directory grant. Ravel then makes the harness its own foreground
job.

<!-- fragment «loop-launch-contract» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="369-398" parent="loop-driver" -->
````rust
/// Launch ravel as a foreground, channel-free job. Ravel selects and
/// supervises the harness as a separate job and owns its exit signal. The runner
/// reports ravel's start/reap and restores the handed-over terminal modes.
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
    stdout: std::fs::File,
) -> Result<Ended> {
    let selection = &selected.selection;
    eprintln!(
        "grove: launching {} through Ravel — {}",
        selection.kind.label(),
        selection.handle
    );

````
<!-- /fragment -->

The epoch already names this launch when the runner spawns Ravel. Witness events surround the job, and a channel-free launch forwards only caller cancellation.
The runner still takes its own `Argv`, so the call converts the value of
`session_command` first. The runner also emits its own `LaunchEvent`, so the
call maps each event to the type of `ravel_job` before the lease sees it. The
migration to [the shared function](#ravel-job) removes both conversions.

<!-- fragment «loop-launch-spawn» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="399-435" parent="loop-driver" -->
````rust
    driver_lease
        .prepare_launch(selected.lifetime, selection, launch_dir)
        .context("activating the foreground session epoch before spawn")?;

    // The runner takes its own values until the next leaf deletes this bridge.
    let keyed_argv = keyed_launch::Argv::new(argv.program().to_owned(), argv.args().to_vec());
    driver_lease
        .supervise_launch(|observer| {
            let mut bridge = |event: keyed_launch::LaunchEvent| {
                observer(match event {
                    keyed_launch::LaunchEvent::Started => crate::ravel_job::LaunchEvent::Started,
                    keyed_launch::LaunchEvent::Reaped => crate::ravel_job::LaunchEvent::Reaped,
                });
            };
            keyed_launch::run_observed_with_stdout(
                Launch {
                    argv: &keyed_argv,
                    channel: None,
                    scrub: &scrub_list(),
                    grant: &[(OsStr::new("GROVE_LAUNCH_DIR"), launch_dir.as_os_str())],
                    transparent: None,
                    cwd: Some(worktree),
                    escalation: ESCALATION,
                },
                stdout,
                &mut bridge,
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

````
<!-- /fragment -->

<a id="the-handoff"></a>
## The handoff: the one ordering the loop cannot get wrong

The handoff invalidates the epoch even when launch returned an error. Only a
successful invalidation invokes the reading closure. If both fail, the error
retains the launch failure beside the invalidation failure; nothing consumes the
ending beside an admitted orphan still holding the old epoch.

<!-- fragment «loop-handoff» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="436-453" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="the-fourth-choice"></a>
## The cancellation bound

The zero channel grace has no escalation trigger because this launch carries
no channel. The ten-second kill-grace bounds forwarded cancellation of Ravel,
covering managed shutdown and group confirmation before the public result.
Grove does not choose the harness's exit-signal graces.

<!-- fragment «loop-escalation» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="454-461" parent="loop-driver" -->
````rust
/// No channel grace: Grove ends no harness. Its own TERM/HUP is forwarded
/// to ravel, whose cancellation must finish before this 10-second kill-grace.
/// Ravel budgets managed-process cleanup before its public result returns.
const ESCALATION: Escalation = Escalation {
    grace: Duration::ZERO,
    kill_grace: Duration::from_secs(10),
};

````
<!-- /fragment -->

<a id="reset-terminal"></a>
## `reset_terminal`: a guard that is a hang, not a tidiness

The runner restores the attributes saved when Grove handed over the terminal.
After Ravel's signal death it can reclaim from the orphaned harness's group,
not just Ravel's. `reset_terminal` then runs `stty sane`, leaves the alternate
screen and shows the cursor. It runs only on tty stdin while the driver owns the
foreground. Otherwise an `stty` child could stop on SIGTTOU and hang the driver.

<!-- fragment «loop-reset-terminal» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="462-493" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="ignore-interrupts"></a>
## `ignore_interrupts`: one signal, and the reason it is only one

The driver's SIGINT ignore covers gaps between launches. The runner resets the
child's terminal-signal dispositions, so Ravel and the harness receive a typed
Ctrl-C. TERM and HUP belong to runner cancellation, and the runner forwards them
to the job.

<!-- fragment «loop-ignore-interrupts» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="494-524" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="picked"></a>
## `picked`: retain the selected root without retaining its lock

Selection opens the root identity before acquiring the read guard and checks
it while the guard is held. `picked_after_finish` selects afresh after
materialization. A concurrent root replacement cannot lend an old selection
the new root's identity.

<!-- fragment «loop-picked» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="525-564" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="the-test-block"></a>
## The inline tests: root identity and handoff ordering

The inline tests cover root pinning, finish materialisation, admission before
the spawn, the launch reading table and the error precedence of the invalidation
gate. The reading-table cases combine teardown with TERM and HUP, and with a
surviving Ravel group. Swapping interruption and teardown makes both that test
and the real PTY interruption case fail. Moving the group guard above teardown
makes the runner-result test fail. These tests use the real Ravel front for the
launch case. The process suite supplies the PTY ending, terminal, refusal,
stale-session and cancellation cases.

<!-- fragment «loop-tests-open» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="565-859" parent="loop-driver" -->
````rust
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
    fn launch_reading_precedence_over_interrupts_and_surviving_groups() {
        use std::os::unix::process::ExitStatusExt;
        let selection = Selection {
            path: "01-impl--work-k1.md".into(),
            handle: Handle::parse("work-k1").unwrap(),
            kind: Kind::new("impl").unwrap(),
        };
        for (end, group, teardown, supported_result, want) in [
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
                status: std::process::ExitStatus::from_raw(0),
                elapsed: Duration::ZERO,
                signalled: false,
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
                "{end:?}, {group:?}, teardown={teardown}, supported_result={supported_result}"
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
        for group in [Group::Gone, Group::Present { pgid: 123 }] {
            for teardown in [false, true] {
                for (end, status, want, reads) in [
                    (End::Exited, 0, Some(LoopOutcome::ForceStopped), 1),
                    (
                        End::Exited,
                        256,
                        Some(if teardown {
                            LoopOutcome::Finished
                        } else {
                            LoopOutcome::Stopped
                        }),
                        0,
                    ),
                    (
                        End::Interrupted {
                            signal: libc::SIGTERM,
                        },
                        0,
                        Some(LoopOutcome::Interrupted(libc::SIGTERM)),
                        0,
                    ),
                    (
                        End::Interrupted {
                            signal: libc::SIGHUP,
                        },
                        256,
                        Some(LoopOutcome::Interrupted(libc::SIGHUP)),
                        0,
                    ),
                ] {
                    let ended = Ended {
                        end,
                        group,
                        status: std::process::ExitStatus::from_raw(status),
                        elapsed: Duration::ZERO,
                        signalled: false,
                    };
                    let reading = crate::launch_directory::LaunchReading { teardown };
                    let read_count = Cell::new(0);
                    let got = interpret_launch(&ended, &reading, &selection, || {
                        read_count.set(read_count.get() + 1);
                        Ok(SessionDisposition::ForceStop)
                    });
                    assert_eq!(
                        got, want,
                        "{end:?}, {status}, {group:?}, teardown={teardown}"
                    );
                    assert_eq!(read_count.get(), reads);
                }
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

    /// The two `complete_post_reap_epoch_handoff` cases below drive that
    /// ordering with a stand-in for the launch result, because what the
    /// ordering is *about* is which of the two failures survives — not what a
    /// session left behind. The launch and escalation themselves are the
    /// runner's, and `crates/keyed-launch/tests/launch.rs` drives them end to
    /// end against a fake child.
````
<!-- /fragment -->

This case supplies simultaneous launch and invalidation failures. It checks that both causes remain visible and interpretation never runs.

<!-- fragment «loop-test-handoff-preserves» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="860-895" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

This case isolates the invalidation gate. Even a successful reap cannot cause the reading closure to run before the driver holds exclusive epoch admission.

<!-- fragment «loop-test-ordering» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="896-918" parent="loop-driver" -->
````rust
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
````
<!-- /fragment -->

<a id="six-calls-into-the-lease"></a>
## Calls into the lease, and what each one is for

The lease revalidations bound transitions and launch. Preparation activates
session admission, supervision emits the witnessed start/reap pair, and
invalidation closes admission before interpretation. None retains a tree guard
while the harness performs its own mutations.

<a id="what-holds-the-four-choices"></a>
## The executable seams

The executable seams hold distinct obligations. The job tests of the runner
hold job control and recovery. The suite of Ravel holds harness escalation and
opaque result transport. The PTY suite of Grove holds the composition of those
mechanisms and the reading table of the loop. An exit status alone cannot
replace the supported public result.

<a id="the-zeros"></a>
## The ownership boundaries

Grove allocates no harness channel and reads no owner policy or run record.
These are ownership boundaries visible in the launch and reading functions,
not conclusions drawn from missing grep matches. It retains the ten-second
cancellation bound because Ravel remains its child.

<a id="what-could-not-move-here"></a>
## What could not move

The loop's choices stay here: which leaf runs, what prompt it receives, where
its launch identity lives, and which ending continues the tree. The runner has
no task vocabulary. Ravel has no Grove vocabulary. A teardown remains
Grove's deliberate record, interpreted before Ravel's opaque returned result.

<a id="launch-control-area"></a>
## The launch directory carries identity, ending and teardown

`LaunchDirectory` creates an owner-only directory with a fresh random suffix.
Its reading checks only for a regular teardown marker after reap and epoch
invalidation. Symlinks and directories are not teardown markers. Legacy JSON
files are ignored: public result bytes live in the separate held capture,
which never reopens a launch-directory pathname.

`record_teardown` refuses while `.grove/` exists, names `finish-commit`, and
creates its record exclusively. Replacement-driver cleanup reads no abandoned
contents. The real finish seam exercises record refusal, finish-commit,
idempotent recording and both the exit signal and the harness's own exit.

<!-- fragment «launch-control-directory» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/launch_directory.rs" lines="1-134" parent="source-launch-directory" -->
````rust
//! Grove's launch-scoped control area: identity, interpretation and removal.

use std::ffi::OsStr;
use std::fs::{self, DirBuilder, OpenOptions};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

pub(crate) struct LaunchDirectory(PathBuf);

pub(crate) struct LaunchReading {
    pub(crate) teardown: bool,
}

pub(crate) fn is_launch_name(name: Option<&OsStr>) -> bool {
    name.and_then(OsStr::to_str)
        .and_then(|name| name.strip_prefix("launch-"))
        .is_some_and(|suffix| {
            suffix.len() == 32
                && suffix
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
}

impl LaunchDirectory {
    pub(crate) fn allocate(control: &Path) -> Result<Self> {
        use std::io::Read;
        for _ in 0..8 {
            let mut bytes = [0u8; 16];
            fs::File::open("/dev/urandom")?.read_exact(&mut bytes)?;
            let suffix: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
            let path = control.join(format!("launch-{suffix}"));
            match DirBuilder::new().mode(0o700).create(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error).context("allocating launch directory"),
            }
        }
        bail!("could not allocate launch directory after 8 occupied draws")
    }

    pub(crate) fn path(&self) -> &Path {
        &self.0
    }

    /// Teardown is interpreted only after Ravel's reap and epoch invalidation.
    /// The public result is held separately, never reopened through this path.
    pub(crate) fn read(&self) -> LaunchReading {
        LaunchReading {
            teardown: fs::symlink_metadata(self.0.join("teardown"))
                .is_ok_and(|metadata| metadata.file_type().is_file()),
        }
    }

    pub(crate) fn discard(&self) -> Result<()> {
        fs::remove_dir_all(&self.0).context("removing interpreted launch directory")
    }

    /// The replacement owns the lease and has invalidated the predecessor's
    /// epoch. Contents carry no meaning across drivers and are never read.
    pub(crate) fn discard_abandoned(control: &Path) -> Result<()> {
        for entry in fs::read_dir(control)? {
            let entry = entry?;
            if is_launch_name(Some(&entry.file_name())) {
                let kind = entry.file_type()?;
                if kind.is_dir() {
                    fs::remove_dir_all(entry.path())?;
                } else if kind.is_symlink() {
                    fs::remove_file(entry.path())?;
                }
            }
        }
        Ok(())
    }
}

pub(crate) fn record_teardown(
    worktree: &Path,
    launch_dir: Option<&Path>,
) -> Result<crate::verbs::Recorded> {
    let Some(launch_dir) = launch_dir else {
        return Ok(crate::verbs::Recorded::NoLoop);
    };
    // A dangling symlink still occupies .grove. Never turn a metadata failure
    // into proof that the finish-commit precondition has been met.
    match fs::symlink_metadata(worktree.join(".grove")) {
        Ok(_) => bail!(".grove/ still exists; run `grove-llm finish-commit <finish-handle>` before `record-teardown`"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {},
        Err(error) => return Err(error).context("checking grove teardown"),
    }
    let path = launch_dir.join("teardown");
    // Exclusive creation is atomic and never truncates an existing record.
    // https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&path)
    {
        Ok(_) => {}
        Err(error)
            if error.kind() == std::io::ErrorKind::AlreadyExists
                && fs::symlink_metadata(&path)?.file_type().is_file() => {}
        Err(error) => return Err(error).context("recording launch teardown"),
    }
    Ok(crate::verbs::Recorded::Wrote(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teardown_requires_a_regular_marker_and_ignores_legacy_reports() {
        let control = tempfile::tempdir().unwrap();
        let launch = LaunchDirectory::allocate(control.path()).unwrap();
        assert!(!launch.read().teardown);
        fs::write(launch.path().join("ending.json"), "grove-continue").unwrap();
        assert!(!launch.read().teardown);
        let other = control.path().join("other-teardown");
        fs::write(&other, "").unwrap();
        let marker = launch.path().join("teardown");
        std::os::unix::fs::symlink(other, &marker).unwrap();
        assert!(!launch.read().teardown);
        fs::remove_file(&marker).unwrap();
        fs::create_dir(&marker).unwrap();
        assert!(!launch.read().teardown);
        fs::remove_dir(&marker).unwrap();
        fs::write(marker, "").unwrap();
        assert!(launch.read().teardown);
    }
}
````
<!-- /fragment -->

<a id="held-session-result"></a>
## The shared result capture for the Ravel migration

`SessionResult` is the active shared transport for both actual callers. It
owns allocation, stdout duplication and bounded exact-byte interpretation.
Both callers run their authority and supervision checks before calling its reader.

The supervisor writes its public stdout result into an unlinked regular file.
The reader keeps the original descriptor rather than opening a pathname after
a managed process could replace it. Allocation sets mode 0600 and checks that
the file has no links, because the Unix fallback of tempfile can ignore an unlink
failure. Ravel and policy retain the stdout authority granted to the supervisor.
Unlinking does not authenticate their bytes.

The writer's cloned descriptor shares an offset with the reader. After
the caller reaps the supervisor and every writer closes, the reader rewinds
and reads at most seventeen bytes. That is the longest accepted literal,
`grove-force-stop`, plus one byte that rejects any tail. The shorter
`grove-continue` is also exact. Empty, padded, binary, transcript-contaminated
and unsupported values refuse.

The typed disposition carries only Grove's meaning. It cannot establish
supervisor success, group cleanup or epoch invalidation. Lifecycle interprets
force-stop before teardown. Standalone reports force-stop as failure before
validating or publishing any outputs. Both callers consume the typed reader
directly, with no continuation-only adapter.

<!-- fragment «held-session-result» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/session_result.rs" lines="1-105" parent="source-session-result" -->
````rust
//! Caller-owned stdout transport for Ravel's opaque Grove results.

use std::fs::File;
use std::io::{Read, Seek};
use std::os::unix::fs::{MetadataExt, PermissionsExt};

use anyhow::Context;

use crate::Error;

/// The opaque result Grove accepts for ordinary continuation.
pub const CONTINUE_RESULT: &str = "grove-continue";

/// The opaque result Grove accepts for an explicit stop without completion.
pub const FORCE_STOP_RESULT: &str = "grove-force-stop";

/// Grove's interpretation of a supported opaque result; Ravel assigns none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionDisposition {
    Continue,
    ForceStop,
}

/// A held, unlinked regular file for Ravel's public stdout result.
///
/// Pass [`Self::stdout`] to keyed-launch's stdout-capturing entry point. Call
/// [`Self::read`] only after the supervisor has been reaped and all
/// writers have closed. Lifecycle callers must also invalidate the session
/// epoch first. Teardown precedence, cancellation, exit status and survivor
/// checks belong to the caller; accepted bytes alone attest none of them.
///
/// The result has no pathname for a managed process to substitute. Policy and
/// Ravel still have the stdout authority deliberately given to the supervisor.
pub struct SessionResult(File);

impl SessionResult {
    /// Allocate a fresh result capture.
    ///
    /// # Errors
    /// Failure to allocate or verify an unlinked regular file. No launch should
    /// start if capture allocation fails.
    pub fn allocate() -> Result<Self, Error> {
        // tempfile 3.27's Unix fallback ignores unlink failures; verify the held
        // file before granting stdout rather than assuming it has no pathname.
        // https://github.com/Stebalien/tempfile/blob/v3.27.0/src/file/imp/unix.rs
        let file = tempfile::tempfile().context("allocating session result capture")?;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))
            .context("restricting session result capture permissions")?;
        let metadata = file.metadata().context("checking session result capture")?;
        if !metadata.is_file() || metadata.nlink() != 0 {
            return Err(Error::msg(
                "session result capture must be an unlinked regular file; no session was launched",
            ));
        }
        Ok(Self(file))
    }

    /// An owned descriptor to pass to the stdout-capturing runner.
    ///
    /// It shares this capture's file offset. The file starts empty; the reader
    /// rewinds after supervision. This descriptor must carry only Ravel stdout,
    /// never managed output, stderr or a transcript.
    ///
    /// # Errors
    /// Failure to duplicate the file descriptor.
    pub fn stdout(&self) -> Result<File, Error> {
        Ok(self
            .0
            .try_clone()
            .context("duplicating session result stdout")?)
    }

    /// Consume the capture and accept either exact Grove result, without trimming.
    ///
    /// Reads at most the longest literal's length plus one byte; a prefix with
    /// any tail refuses. Never reopens a path or parses a private ending report.
    ///
    /// # Errors
    /// Missing, unsupported or unreadable results.
    pub fn read(mut self) -> Result<SessionDisposition, Error> {
        self.0
            .rewind()
            .context("rewinding session result capture")?;
        let bound = CONTINUE_RESULT.len().max(FORCE_STOP_RESULT.len()) + 1;
        let mut bytes = Vec::with_capacity(bound);
        self.0
            .take(bound as u64)
            .read_to_end(&mut bytes)
            .context("reading session result capture")?;
        // Ravel's public CLI returns the exact opaque argument without a newline.
        // https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-cli.md
        if bytes.is_empty() {
            return Err(Error::msg(format!(
                "Ravel returned no session result; complete with `ravel end-session {CONTINUE_RESULT}`"
            )));
        }
        match bytes.as_slice() {
            bytes if bytes == CONTINUE_RESULT.as_bytes() => Ok(SessionDisposition::Continue),
            bytes if bytes == FORCE_STOP_RESULT.as_bytes() => Ok(SessionDisposition::ForceStop),
            _ => Err(Error::msg(format!(
                "Ravel returned an unsupported session result; expected exactly {CONTINUE_RESULT} or {FORCE_STOP_RESULT}"
            ))),
        }
    }
}
````
<!-- /fragment -->

The evidence lives outside this book's source corpus.
`tests/session_result.rs` exercises the exact value, missing rewind, rejected
tails and the bounded sparse-file read. `tests/session_result_installed.rs`
uses installed Ravel 1.0.0 and its compiled worker from an independent prefix,
with both keyed-launch stdout APIs and fake shell agents.

Session stdout and stderr noise remain separate from the exact result. Empty and
unsupported values refuse, and a natural exit without a result fails. The
fixture isolates PATH to the copied pair and scrubs old and new enclosing-session
authority. `task ravel:result RAVEL=/absolute/path/to/ravel` runs these checks
without a model launch or host runtime on the managed PATH.

<a id="public-session-commands"></a>
## Shared public commands for the Ravel migration

`session_command` takes the canonical executable the caller's resolver already
selected. It performs no executable lookup or second canonicalisation, so its
argv and ending instructions name the same selected installation. Both actual
callers use this shared public boundary.

The module owns `Argv`, the value that both builders return. It holds a program
and its arguments, and `words` lists them with the program first. It has no
shell and no second reading of any word. The runner has its own `Argv` for now,
so each caller converts the value once, at its one call site.

`lifecycle` sends the selection's kind, task path and handle together with the
literal prompt and writable exit directory. `confined` sends the standalone
kind and prompt with `--confine` and canonical runtime-file grants, without task
data. Each value joins its flag in one argv word, so a leading dash cannot become
an option. These are public Ravel inputs. The call sends no private ending-report
flag.

Runtime-file paths resolve before the runner changes to the staged cwd. Ravel's
public confinement contract admits its own real front, so Grove adds no worker
or policy-directory grant. The caller still owns supervision and result capture.
A correctly built command alone proves neither successful cleanup nor a result.

`end_session` quotes the selected UTF-8 executable as one shell word and passes
Grove's exact `grove-continue` result. Quotes, spaces and newlines in the path
remain literal. A non-UTF-8 path refuses before it can become prompt text.

<!-- fragment «public-session-commands» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/session_command.rs" lines="1-143" parent="source-session-command" -->
````rust
//! Grove's public Ravel invocation and exact ending command.
//!
//! The caller supplies the executable already selected and canonicalized by
//! Grove's PATH/version resolver. These helpers never resolve that path again.
//! Both callers share these public launch and ending commands.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::{
    session_result::{CONTINUE_RESULT, FORCE_STOP_RESULT},
    Error, Selection,
};

/// A program and its arguments, in order, ready to spawn.
///
/// There is no shell and no second reading of any word. Each word is spawned as
/// it is given.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Argv {
    program: OsString,
    args: Vec<OsString>,
}

impl Argv {
    /// A program and arguments that the caller built, each string one whole word.
    #[must_use]
    pub fn new(program: OsString, args: Vec<OsString>) -> Self {
        Self { program, args }
    }

    #[must_use]
    pub fn program(&self) -> &OsStr {
        &self.program
    }

    #[must_use]
    pub fn args(&self) -> &[OsString] {
        &self.args
    }

    /// The whole command as one word list, program first. A consumer that builds
    /// a `Command` and a diagnostic both want this shape.
    #[must_use]
    pub fn words(&self) -> Vec<OsString> {
        let mut words = Vec::with_capacity(self.args.len() + 1);
        words.push(self.program.clone());
        words.extend(self.args.iter().cloned());
        words
    }
}

/// The lifecycle request for the same selection that composed the prompt.
///
/// Only public selection inputs and the caller's writable exit directory are
/// sent. Ravel's stdout, captured separately by the caller, is the result.
#[must_use]
pub fn lifecycle(ravel: &Path, task: &Selection, prompt: &str, exit_dir: &Path) -> Argv {
    let mut args = start(task.kind.label(), prompt, exit_dir);
    args.push(value("--task-file=", task.path.as_os_str()));
    args.push(value("--task-id=", task.handle.to_string().as_ref()));
    Argv::new(ravel.into(), args)
}

/// A standalone request whose managed process is confined to the staged cwd.
///
/// Runtime files are canonicalized now, before the runner changes cwd. Ravel
/// admits the real front itself; no policy directory or worker grant is added.
/// Owner routing settings, bounds and environment grants remain owner inputs.
///
/// # Errors
/// A runtime-file path cannot be resolved in the caller's current directory.
pub fn confined(
    ravel: &Path,
    kind: &str,
    prompt: &str,
    exit_dir: &Path,
    runtime_read: &[PathBuf],
) -> Result<Argv, Error> {
    // https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/specs/session-execution.md
    let mut args = start(kind, prompt, exit_dir);
    args.push("--confine".into());
    for path in runtime_read {
        let canonical = path.canonicalize().with_context(|| {
            format!(
                "resolving runtime file {} before staging cwd",
                path.display()
            )
        })?;
        args.push(value("--runtime-read=", canonical.as_os_str()));
    }
    Ok(Argv::new(ravel.into(), args))
}

/// The prompt's exact ending command, using the already selected executable.
///
/// Quote its UTF-8 path as one literal shell word, including embedded quotes
/// and newlines. The opaque result is Grove's, never a Ravel session identifier.
///
/// # Errors
/// A path that cannot be represented in a UTF-8 prompt. Select a UTF-8 Ravel
/// installation path before launching a session.
pub fn end_session(ravel: &Path) -> Result<String, Error> {
    end_with_result(ravel, CONTINUE_RESULT)
}

/// The exact command to stop without acknowledging completion.
///
/// # Errors
/// A non-UTF-8 executable path, as for [`end_session`].
pub fn force_stop(ravel: &Path) -> Result<String, Error> {
    end_with_result(ravel, FORCE_STOP_RESULT)
}

fn end_with_result(ravel: &Path, result: &str) -> Result<String, Error> {
    let path = ravel.to_str().context(
        "the ending command requires a UTF-8 Ravel path; select a UTF-8 installation on PATH",
    )?;
    Ok(format!(
        "'{}' end-session {result}",
        path.replace('\'', "'\\''")
    ))
}

fn start(kind: &str, prompt: &str, exit_dir: &Path) -> Vec<OsString> {
    // Ravel 1.0.0's documented flags return only the opaque result on stdout.
    // https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-cli.md
    vec![
        "start-session".into(),
        value("--kind=", kind.as_ref()),
        value("--prompt=", prompt.as_ref()),
        value("--exit-dir=", exit_dir.as_os_str()),
    ]
}

// Join each value to its flag so a leading dash never becomes another option.
fn value(flag: &str, argument: &OsStr) -> OsString {
    let mut word = OsString::from(flag);
    word.push(argument);
    word
}
````
<!-- /fragment -->

The evidence is `tests/session_command.rs`. Its shell test executes a path with
quotes, newlines and shell punctuation on an unusable PATH. Installed Ravel
fixtures observe the lifecycle inputs through the actual policy. They launch
through [`ravel_job::launch`](#ravel-job), the lifecycle case in the foreground
shape and the confined case in the detached shape. They execute the
quoted ending from a separate prefix. They keep managed output separate from the
result.

The lifecycle's lookup PATH contains an executable decoy after selection. So
launching or ending through a bare name fails the same real-front check.

The confined case first observes a denied credential read. It then grants the
same relative path before staging. It verifies that the path can be read while
the policy stays unreadable. `task ravel:commands RAVEL=/absolute/path/to/ravel`
runs the shell and installed checks. It needs no model and no host runtime on the
managed PATH.

<a id="ravel-job"></a>
## One function launches Ravel

`ravel_job::launch` spawns `ravel` as an ordinary child, as a job-control shell
spawns a job. The lifecycle and `grove run` will share it. It adds no completion
channel, no escalation and no group probe. Ravel keeps job control of the managed
process and does not change.

No production caller uses the function yet. The loop and `grove run` still launch
through the runner. Each converts its `Argv` at one call site. The function takes
two shapes. The foreground shape serves the lifecycle, and the detached shape
serves `grove run`.

<!-- fragment «ravel-job» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/ravel_job.rs" lines="1-757" parent="source-ravel-job" -->
<!-- insert «ravel-job-header» -->
<!-- insert «ravel-job-scrub» -->
<!-- insert «ravel-job-vocabulary» -->
<!-- insert «ravel-job-launch» -->
<!-- insert «ravel-job-watch» -->
<!-- insert «ravel-job-latch» -->
<!-- insert «ravel-job-terminal» -->
<!-- insert «ravel-job-lease» -->
<!-- insert «ravel-job-tests» -->
<!-- /fragment -->

<a id="the-ravel-job-header"></a>
## The header, and what the function leaves out

The header names the one function and the arrangement behind it. Grove keeps no
runner of its own. Ravel supervises its managed process and ends its group. So
this module sends no KILL and probes no process group. A successful Ravel status
with an exact result is the only evidence of cleanup.

The poll interval is a constant and not a setting. It bounds how late a caught
signal reaches `ravel`. Ravel budgets its own cancellation in seconds, so a
caller that tuned the interval would tune latency that it cannot observe.

<!-- fragment «ravel-job-header» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/ravel_job.rs" lines="1-32" parent="ravel-job" -->
````rust
//! Starting `ravel` as an ordinary child, as a job-control shell starts a job.
//!
//! One function, [`launch`], serves both callers. The lifecycle uses the
//! foreground [`Shape`]. `grove run` uses the detached one. The caller contract
//! states the procedures: `docs/specs/harness-selection-and-execution.md`, in
//! the section *Jobs and authority*.
//!
//! Grove keeps no runner of its own. Ravel supervises its managed process and
//! ends its group. So this module sends no KILL and probes no process group.
//! A successful Ravel status with an exact result is the only evidence of
//! cleanup. Ravel states its side of the arrangement in chapter 9 of its book:
//! <https://github.com/Linkuistics/Ravel/blob/v1.1.0/docs/book/src/09-jobs-signals-and-the-terminal.md>

use std::fmt;
use std::fs::File;
use std::io::Write as _;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::process::CommandExt as _;
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use crate::session_command::Argv;

/// How long the function waits between two checks of `ravel`.
///
/// This is not a knob. The interval only bounds how late a caught signal
/// reaches `ravel`, and Ravel budgets its own cancellation in seconds.
const POLL_INTERVAL: Duration = Duration::from_millis(500);

````
<!-- /fragment -->

<a id="the-ravel-job-scrub"></a>
## One list of ten names, and one helper

An environment is inherited. A child that Grove merely declines to give a control
variable still receives whatever Grove's own environment carried. That includes
the launch directory of another session, and the launch directory is authority
to end that session. So every spawn removes the ten names of `CONTROL_ENV`.

`scrub` is the helper for that removal. The job calls it. The version probe and
the terminal reset will call it when they move here. Until they move, the loop
and `grove run` keep their own copies of the list.

<!-- fragment «ravel-job-scrub» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/ravel_job.rs" lines="33-62" parent="ravel-job" -->
````rust
/// The loop control variables. Every spawn removes them.
///
/// An environment is inherited, so a child that Grove merely declines to give
/// these still receives whatever Grove's own environment carried. That
/// includes a live launch directory of another session, which is authority to
/// end that session. `GROVE_SIGNAL_FILE` and `GROVE_RUN_SIGNAL_FILE` are
/// retired completion authority, kept for installed v22 sessions.
pub const CONTROL_ENV: [&str; 10] = [
    "GROVE_SIGNAL_FILE",
    "GROVE_RUN_SIGNAL_FILE",
    "GROVE_LAUNCH_DIR",
    "HARNESS_DISPATCH_EXIT_FILE",
    "HARNESS_DISPATCH_RUN_ID",
    "HARNESS_DISPATCH_STATE_DIR",
    "RAVEL_EXIT_FILE",
    "GROVE_PROVIDER_EVIDENCE",
    "GROVE_HARNESS_PID",
    "GROVE_CLAUDE_PID",
];

/// Remove every [`CONTROL_ENV`] name from `command`'s environment.
///
/// Each spawn of Grove calls this: the job, the version probe and the terminal
/// reset. A rule with one argued exception is a rule that a later spawn breaks.
pub fn scrub(command: &mut Command) {
    for name in CONTROL_ENV {
        command.env_remove(name);
    }
}

````
<!-- /fragment -->

<a id="the-ravel-job-vocabulary"></a>
## The shape, the notices, the end and the error

`Shape` sets the terminal, the streams and the one variable that Grove grants.
The foreground shape carries the launch directory. The detached shape carries the
transcript file. `LaunchEvent` has the two notices that the driver lease needs:
`Started` after a successful spawn and `Reaped` after a successful wait.

`JobEnd` is the end of a job. It has the wait status, the interruption and the
elapsed time. It has no group field, because Grove checks no group.

`JobError` carries the interruption. A signal that Grove caught is the driver's,
even when the spawn that follows it fails. An error that dropped the signal
would turn a Ctrl-C into an ordinary error, and the driver would carry on.
`Failure` is the same error before the latch is taken into it.

<!-- fragment «ravel-job-vocabulary» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/ravel_job.rs" lines="63-149" parent="ravel-job" -->
````rust
/// Which kind of job `ravel` is. The shape fixes the terminal, the streams and
/// the one variable that Grove grants.
pub enum Shape<'a> {
    /// `ravel` leads a new process group in Grove's session. When Grove holds
    /// the foreground of its terminal, the group of `ravel` takes it. Standard
    /// input and standard error are inherited. The job receives
    /// `GROVE_LAUNCH_DIR` with `launch_dir` after the scrub.
    Foreground { launch_dir: &'a Path },
    /// `ravel` leads a new POSIX session with no terminal. Standard input is
    /// end of file, and standard error goes to `transcript`. Every descriptor
    /// above standard error closes at the exec. The job receives no variable
    /// that Grove grants.
    Detached { transcript: File },
}

/// Evidence about one job, independent of any result that `ravel` returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchEvent {
    /// The spawn succeeded. `ravel` may already have exited.
    Started,
    /// A wait confirmed the reap. It comes before the terminal recovery.
    Reaped,
}

/// How a job ended.
#[derive(Debug)]
pub struct JobEnd {
    /// The wait status of `ravel`. Only a successful status counts as evidence
    /// of cleanup.
    pub status: ExitStatus,
    /// The first signal that Grove caught during the job, or the signal that
    /// the latch still held at the reap. A later signal belongs to the caller.
    pub interrupted: Option<i32>,
    /// The time from the spawn to the reap.
    pub elapsed: Duration,
}

/// A job that failed to start or to be waited on.
///
/// **The error carries the interruption.** A signal that Grove caught is the
/// driver's, even when the spawn that follows it fails. Dropping it would turn
/// a Ctrl-C into an ordinary error, and the driver would carry on.
pub struct JobError {
    message: String,
    interrupted: Option<i32>,
}

impl JobError {
    /// The signal that Grove caught, if one reached the process before or
    /// during the job.
    #[must_use]
    pub fn interrupted(&self) -> Option<i32> {
        self.interrupted
    }
}

impl fmt::Display for JobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// The message, not a struct dump: a human reads this in an `anyhow` chain.
impl fmt::Debug for JobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for JobError {}

/// An error before the latch is taken into it.
struct Failure {
    message: String,
    /// A signal that Grove already sent to `ravel` before the failure.
    sent: Option<i32>,
}

impl Failure {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            sent: None,
        }
    }
}

````
<!-- /fragment -->

<a id="the-ravel-job-launch"></a>
## `launch`: scrub, grant, hand over, spawn

`launch` reads the entry state once and then does its work in `start_and_watch`.
The error path takes the latch into every failure. This holds for a failed spawn
and for the checks that run before it.

The order of the environment is load-bearing. The scrub list names
`GROVE_LAUNCH_DIR`. So the scrub comes first and the foreground grant comes after
it. Only the foreground shape grants the variable.

The foreground job leads a new process group in Grove's session. The child hands
itself the terminal before the exec, and the parent repeats the handover after
the spawn. The detached job closes every descriptor above standard error at the
exec and leads a new session. Both jobs set the seven job-control signals to
their defaults between the fork and the exec. The function never clears the latch
before the spawn.

<!-- fragment «ravel-job-launch» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/ravel_job.rs" lines="150-301" parent="ravel-job" -->
````rust
/// Start `ravel` with `command` in `cwd` and wait until it exits.
///
/// Standard output goes to `stdout`, a regular file that the caller holds. Its
/// position and contents stay the caller's. The function never truncates it,
/// rewinds it or reads it.
///
/// `observer` receives [`LaunchEvent::Started`] once after a successful spawn
/// and [`LaunchEvent::Reaped`] once after a successful wait. A failed spawn
/// sends neither notice. A failed wait sends no `Reaped`. The observer must
/// return promptly and must not panic.
///
/// **The environment is Grove's minus [`CONTROL_ENV`].** Only [`Shape::Foreground`]
/// then adds `GROVE_LAUNCH_DIR`.
///
/// **The latch catches INT, TERM and HUP from the first call on.** A signal
/// that Grove ignores stays ignored. The function never clears the latch before
/// the spawn. So a signal that arrives after the last check of the caller and
/// before the spawn goes to the new `ravel` at the first poll. At each poll the
/// function passes the foreground on, sends each latched signal to `ravel`
/// alone, and checks for an exit without blocking. It never signals a process
/// group. It waits until `ravel` exits, however long that takes.
///
/// # Errors
/// A command that cannot be spawned, a file that is not regular, or a wait that
/// fails. Each error carries the interruption: see [`JobError`].
pub fn launch(
    command: &Argv,
    cwd: &Path,
    stdout: File,
    observer: &mut dyn FnMut(LaunchEvent),
    shape: Shape<'_>,
) -> Result<JobEnd, JobError> {
    let sigchld_ignored = *ENTRY.get_or_init(enter);
    start_and_watch(command, cwd, stdout, observer, shape, sigchld_ignored).map_err(|failure| {
        JobError {
            message: failure.message,
            interrupted: freeze(failure.sent),
        }
    })
}

fn start_and_watch(
    command: &Argv,
    cwd: &Path,
    stdout: File,
    observer: &mut dyn FnMut(LaunchEvent),
    shape: Shape<'_>,
    sigchld_ignored: bool,
) -> Result<JobEnd, Failure> {
    ensure_regular(&stdout, "stdout")?;
    let detached = matches!(shape, Shape::Detached { .. });
    let mut job = Command::new(command.program());
    job.args(command.args()).current_dir(cwd).stdout(stdout);
    // Scrub first and grant second. The order is load-bearing: the scrub list
    // names `GROVE_LAUNCH_DIR`, and a grant before it would remove the path
    // that this job is meant to receive.
    scrub(&mut job);
    let mut lease = match shape {
        Shape::Foreground { launch_dir } => {
            job.env("GROVE_LAUNCH_DIR", launch_dir);
            Some(Lease {
                terminal: Terminal::open(),
                child: 0,
                held: false,
                saved: None,
            })
        }
        Shape::Detached { transcript } => {
            ensure_regular(&transcript, "transcript")?;
            job.stdin(Stdio::null()).stderr(transcript);
            None
        }
    };
    // Hand the terminal over from inside the child as well as from the parent
    // below. The handover of the parent waits on `spawn`, and nothing orders the
    // first read of the child after it. A read from a background group is what
    // SIGTTIN stops. Holding the foreground now is also when the modes are
    // saved, before a child can set the terminal raw.
    let handover = lease.as_mut().and_then(Lease::hold_before_spawn);
    let descriptor_limit = if detached { descriptor_limit()? } else { 3 };
    if !detached {
        // Through `std`, which runs it before the `pre_exec` callback and
        // reports a failure as a failed spawn.
        job.process_group(0);
    }
    // SAFETY: the closure runs between `fork` and `exec`, so it may call only
    // async-signal-safe functions. `fcntl`, `setsid`, `signal`, `getpid` and
    // `tcsetpgrp` (an `ioctl`) are all on the list of POSIX. The closure
    // allocates nothing, takes no lock and touches no state of the Rust runtime.
    unsafe {
        job.pre_exec(move || {
            if detached {
                // Close on exec, not here: the error pipe of `std` must survive
                // until the exec succeeds, and it is close-on-exec too.
                for fd in 3..descriptor_limit {
                    if libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) == -1 {
                        let error = std::io::Error::last_os_error();
                        if error.raw_os_error() != Some(libc::EBADF) {
                            return Err(error);
                        }
                    }
                }
                // `setsid` creates the session and the group. It must come
                // before the process becomes a group leader.
                if libc::setsid() == -1 {
                    return Err(std::io::Error::last_os_error());
                }
            }
            // Until the `tcsetpgrp` below returns, this process is a background
            // group that touches the terminal. That is what SIGTTOU is raised
            // for. The loop that follows puts the default back.
            libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            if let Some(fd) = handover {
                libc::tcsetpgrp(fd, libc::getpid());
            }
            for signal in DEFAULT_DISPOSITION_IN_CHILD {
                libc::signal(signal, libc::SIG_DFL);
            }
            // Grove repaired an inherited ignored SIGCHLD for itself alone. The
            // child gets the disposition that it would have inherited.
            if sigchld_ignored {
                libc::signal(libc::SIGCHLD, libc::SIG_IGN);
            }
            Ok(())
        });
    }
    let mut child = job.spawn().map_err(|error| {
        Failure::new(format!(
            "cannot spawn {:?}: {error}; check that the program exists and is executable",
            command.program()
        ))
    })?;
    observer(LaunchEvent::Started);
    let pid = child_pid(&child);
    if !detached {
        // The parent's half of the same `setpgid`. It is insurance and not a
        // race: `spawn` returns only after the exec, so the call is measured to
        // fail with EACCES. That ordering is undocumented, so the call stays.
        // SAFETY: `setpgid(2)` naming a child of this process.
        unsafe { libc::setpgid(pid, pid) };
    }
    if let Some(lease) = lease.as_mut() {
        lease.child = pid;
    }
    watch(&mut child, lease.as_mut(), observer)
}

fn child_pid(child: &Child) -> libc::pid_t {
    // The cast cannot truncate: `std` took this value from a `pid_t`.
    child.id() as libc::pid_t
}

````
<!-- /fragment -->

<a id="the-ravel-job-watch"></a>
## The poll: forward, check, reap

At each poll the function passes the foreground on and sends a latched signal to
`ravel` alone. It then checks for an exit without blocking. `try_wait` reports an
exit and never a stop, so a stopped `ravel` is not a finished job.

At the reap the function takes the signal that the latch still holds into the
report and then sends `Reaped`. Terminal recovery comes after the notice. A later
signal belongs to the caller.

A failed wait sends no `Reaped` and no KILL. The error names the process, and
the signal that Grove already sent stays with the driver. The descriptor scan for
the detached shape sits here too. It includes descriptors that a caller opened
before it lowered its soft limit.

<!-- fragment «ravel-job-watch» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/ravel_job.rs" lines="302-409" parent="ravel-job" -->
````rust
/// Watch `ravel` to its exit.
fn watch(
    child: &mut Child,
    mut lease: Option<&mut Lease>,
    observer: &mut dyn FnMut(LaunchEvent),
) -> Result<JobEnd, Failure> {
    let pid = child_pid(child);
    let started = Instant::now();
    let mut interrupted = None;
    loop {
        if let Some(lease) = lease.as_deref_mut() {
            lease.lend();
        }
        if let Some(signal) = take_interrupt() {
            interrupted.get_or_insert(signal);
            // SAFETY: `kill(2)` on a child of this process that is not reaped.
            // An unreaped child keeps its ID, so the signal cannot reach
            // another process. A failure is ignored: ESRCH means that the exit
            // came between the poll and the signal, and the check below sees it.
            unsafe { libc::kill(pid, signal) };
        }
        // `try_wait` reports an exit and never a stop. A stopped child is not
        // an exit.
        match child.try_wait() {
            Ok(Some(status)) => {
                let elapsed = started.elapsed();
                // Freeze the job at the reap, before the observer or the
                // terminal recovery can delay it. Later signals are the caller's.
                let interrupted = freeze(interrupted);
                observer(LaunchEvent::Reaped);
                if let Some(lease) = lease {
                    lease.reclaim(Some(status));
                }
                return Ok(JobEnd {
                    status,
                    interrupted,
                    elapsed,
                });
            }
            Ok(None) => std::thread::sleep(POLL_INTERVAL),
            Err(error) => {
                // The status is unknown. Grove sends no KILL: Ravel alone gives
                // the cleanup assurance. The terminal comes back by the narrow
                // rule, and the error names the process.
                if let Some(lease) = lease {
                    lease.reclaim(None);
                }
                return Err(Failure {
                    message: format!(
                        "cannot wait on ravel (process {pid}): {error}; it was not killed, \
                         so check for a running process"
                    ),
                    sent: interrupted,
                });
            }
        }
    }
}

fn ensure_regular(file: &File, name: &str) -> Result<(), Failure> {
    let metadata = file
        .metadata()
        .map_err(|error| Failure::new(format!("cannot inspect the job {name}: {error}")))?;
    if metadata.is_file() {
        Ok(())
    } else {
        Err(Failure::new(format!(
            "the job {name} must be a regular file"
        )))
    }
}

/// Include existing descriptors even if the caller lowered its soft limit after
/// opening them. The range then covers the descriptors that `std` opens while it
/// prepares the child. A detached job must not start while another thread
/// changes the descriptor limit.
fn descriptor_limit() -> Result<RawFd, Failure> {
    // SAFETY: `getrlimit` writes the initialised structure only.
    let mut limit: libc::rlimit = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit) } != 0 {
        return Err(Failure::new(format!(
            "cannot inspect the descriptor limit: {}",
            std::io::Error::last_os_error()
        )));
    }
    let mut maximum = i32::try_from(limit.rlim_cur).map_err(|_| {
        Failure::new("cannot isolate an unbounded descriptor table; set a finite open-file limit")
    })?;
    let directory = if cfg!(target_os = "linux") {
        "/proc/self/fd"
    } else {
        "/dev/fd"
    };
    for entry in std::fs::read_dir(directory)
        .map_err(|error| Failure::new(format!("cannot inspect inherited descriptors: {error}")))?
    {
        let entry = entry.map_err(|error| Failure::new(error.to_string()))?;
        if let Some(fd) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse::<i32>().ok())
        {
            maximum = maximum.max(fd.saturating_add(1));
        }
    }
    Ok(maximum)
}

````
<!-- /fragment -->

<a id="the-ravel-job-latch"></a>
## The latch, the handlers and `reraise`

The latch holds a signal number, because the ending is reported onward. A driver
that re-raises TERM for a HUP has told its parent the wrong thing. The first call
of `launch` installs the handlers for INT, TERM and HUP. A signal that Grove
ignores stays ignored. The driver ignores SIGINT, so a typed Ctrl-C never ends
the driver. `grove run` keeps the default, so a Ctrl-C cancels it.

The same first call repairs an inherited ignored SIGCHLD. An ignored SIGCHLD
makes the kernel reap children unwatched, and then nothing is left to wait for.
Grove restores the default for itself and never installs a handler. The child
gets the ignore back across the exec.

A job reports the first signal that Grove sent to `ravel`. At the reap and at a
failure, `freeze` also drains the latch. It drains the latch even when Grove
already sent a signal. So a later signal does not stay in the latch for the
caller to mistake for one that came after the job.

`take_interrupt` lets a looping driver collect a signal between jobs. `reraise`
dies of the signal, so that the parent of Grove sees `128 + N` and not a normal
exit. This module owns that call because this module installed the handler.

<!-- fragment «ravel-job-latch» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/ravel_job.rs" lines="410-543" parent="ravel-job" -->
````rust
/// The signal that the handler last received, or `0`.
///
/// The state is process-global because a signal disposition is process-global.
/// The latch holds the number and not a flag, because the ending is reported
/// onward. A driver that re-raises TERM for a HUP has told its parent the wrong
/// thing.
///
/// **The function never clears the latch before the spawn.** A signal that
/// arrived while no child existed belongs to the driver, and [`take_interrupt`]
/// is where the driver collects it. A signal that arrives after the last check of
/// the driver and before the spawn goes to the new `ravel` at the first poll.
static INTERRUPTED_BY: AtomicI32 = AtomicI32::new(0);

/// Whether Grove inherited SIGCHLD ignored, read at the first call.
///
/// The first call also installs the handlers. The state is read once because
/// the repair below changes the disposition, and every later read would see the
/// repair.
static ENTRY: OnceLock<bool> = OnceLock::new();

/// Which signal, if any, reached this process since the last check. The call
/// clears the latch.
///
/// A looping driver calls this at the top of each iteration and after each reap.
/// It answers `None` before the first call of [`launch`], because no handler
/// exists yet and the signal took its default action.
#[must_use]
pub fn take_interrupt() -> Option<i32> {
    match INTERRUPTED_BY.swap(0, Ordering::Relaxed) {
        0 => None,
        signal => Some(signal),
    }
}

/// The interruption of a job: the first signal that Grove sent to `ravel`, or
/// else the one that the latch holds. The call drains the latch either way.
///
/// **The drain does not depend on the first signal.** `Option::or_else` would
/// skip it once a signal had been sent. A later signal would then stay in the
/// latch, and the caller would take it for one that came after the job.
fn freeze(sent: Option<i32>) -> Option<i32> {
    let latched = take_interrupt();
    sent.or(latched)
}

/// A single store is the only work here that is async-signal-safe. The signal
/// reaches `ravel` one poll later, on the ordinary stack of [`launch`].
extern "C" fn on_signal(signal: libc::c_int) {
    INTERRUPTED_BY.store(signal, Ordering::Relaxed);
}

/// Catch INT, TERM and HUP, then repair an inherited ignored SIGCHLD. Answer
/// whether SIGCHLD was ignored.
///
/// **A disposition that Grove ignores stays ignored.** Ignoring is a statement
/// that Grove made. The driver ignores SIGINT so that a typed Ctrl-C never ends
/// it. A handler over that ignore would turn the signal into a cancellation.
///
/// **An ignored SIGCHLD makes the kernel reap children unwatched.** Then
/// nothing is left for the wait to observe. Grove restores the default for
/// itself, with no flags and never a handler: a handler would turn the exit of
/// each child into EINTR on whatever Grove is doing.
fn enter() -> bool {
    // Through the function pointer, as rustc requires for the cast.
    let handler = on_signal as extern "C" fn(libc::c_int) as usize;
    for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
        if disposition(signal) != libc::SIG_IGN {
            // SAFETY: `signal(2)` with a handler that performs one relaxed
            // atomic store.
            unsafe { libc::signal(signal, handler as libc::sighandler_t) };
        }
    }
    let ignored = disposition(libc::SIGCHLD) == libc::SIG_IGN;
    if ignored {
        // SAFETY: `sigaction(2)` installing the default with an empty mask and
        // no flags, which clears SA_NOCLDWAIT with it.
        unsafe {
            let mut default: libc::sigaction = std::mem::zeroed();
            default.sa_sigaction = libc::SIG_DFL;
            libc::sigemptyset(&mut default.sa_mask);
            libc::sigaction(libc::SIGCHLD, &default, std::ptr::null_mut());
        }
    }
    ignored
}

/// The current disposition of `signal`, read without changing it.
fn disposition(signal: libc::c_int) -> libc::sighandler_t {
    // SAFETY: `sigaction(2)` with a null new action only reads the current one
    // into an initialised structure.
    unsafe {
        let mut current: libc::sigaction = std::mem::zeroed();
        libc::sigaction(signal, std::ptr::null(), &mut current);
        current.sa_sigaction
    }
}

/// Die of the signal that ended this process, so that its parent sees `128 + N`
/// in the wait status.
///
/// A process that catches TERM, tidies up and exits 0 tells its parent that it
/// finished its work. An exit code cannot say that a signal ended the process.
/// Only a wait status can, and the only way to produce one is to die of the
/// signal. So the default disposition returns, the signal is unblocked in case
/// the handler left it masked, and the signal is raised.
///
/// **This module owns the call because this module installed the handler.** What
/// stays with the caller is whether to re-raise at all. That is a statement about
/// the exit status of the caller.
pub fn reraise(signal: i32) -> ! {
    // A death by signal loses buffered output as `process::exit` does, and the
    // last diagnostic before a termination is the one that a reader wants most.
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();

    // SAFETY: restoring the default disposition of a signal that this module set
    // a handler for, unblocking that one signal and raising it against this
    // process. The set is initialised by `sigemptyset` before use.
    unsafe {
        libc::signal(signal, libc::SIG_DFL);
        let mut unblock: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut unblock);
        libc::sigaddset(&mut unblock, signal);
        libc::sigprocmask(libc::SIG_UNBLOCK, &unblock, std::ptr::null_mut());
        libc::raise(signal);
    }

    // Unreachable for a signal whose default action terminates, which is every
    // signal that interrupts a driver. A caller that passes SIGCHLD or SIGURG
    // gets the conventional code and not a fall-through to code that expected
    // this call to diverge.
    std::process::exit(128 + signal)
}

````
<!-- /fragment -->

<a id="the-ravel-job-terminal"></a>
## Job-control defaults and the terminal

Only an ignored disposition survives `execve`. The driver ignores SIGINT for its
own reasons. Without the list of defaults, `ravel` would inherit the ignore and
pass it to the harness. Nothing in that session could then be interrupted, and
nothing could say why.

`Terminal` opens `/dev/tty`. A process with no controlling terminal cannot open
it and gets no job control. That is the whole gate, and no caller can configure
it wrongly. `tcsetpgrp` from a background group raises SIGTTOU, so every call
ignores SIGTTOU and then restores the previous disposition.

<!-- fragment «ravel-job-terminal» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/ravel_job.rs" lines="544-652" parent="ravel-job" -->
````rust
/// The signals that the child gets at their **default** disposition.
///
/// **Only an ignored disposition survives `execve`.** POSIX resets a caught
/// handler across an exec and leaves an ignore in place. The driver ignores
/// SIGINT for its own reasons, and without this list `ravel` would inherit the
/// ignore. Ravel passes its entry signal state to the harness, so the ignore
/// would reach the harness and every login shell or `ssh` hop in between. An
/// interactive session under them could not be interrupted, and nothing in it
/// could say why.
const DEFAULT_DISPOSITION_IN_CHILD: [libc::c_int; 7] = [
    libc::SIGINT,
    libc::SIGQUIT,
    libc::SIGTERM,
    libc::SIGHUP,
    libc::SIGTSTP,
    libc::SIGTTIN,
    libc::SIGTTOU,
];

/// The controlling terminal of Grove, open for as long as a job needs to hand it
/// over and take it back.
struct Terminal(OwnedFd);

impl Terminal {
    /// `/dev/tty` and not standard input, and the difference is the gate.
    ///
    /// `/dev/tty` is the controlling terminal by definition. A process whose
    /// standard input was redirected still hands over the right device. A
    /// process with no controlling terminal, such as a test runner, a CI job or a
    /// daemon, cannot open it and gets no job control. There is no flag to set
    /// and nothing for a caller to configure wrongly.
    fn open() -> Option<Self> {
        // SAFETY: `open(2)` on a constant NUL-terminated path. The descriptor is
        // owned from here on and closed by `OwnedFd`.
        let fd = unsafe {
            libc::open(
                c"/dev/tty".as_ptr(),
                libc::O_RDWR | libc::O_NOCTTY | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return None;
        }
        // SAFETY: a fresh descriptor that this process just opened and has not
        // handed to anything else.
        Some(Self(unsafe { OwnedFd::from_raw_fd(fd) }))
    }

    fn fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }

    /// The process group that owns the terminal now, or `-1`.
    fn foreground(&self) -> libc::pid_t {
        // SAFETY: `tcgetpgrp(3)` on a descriptor that this structure owns.
        unsafe { libc::tcgetpgrp(self.fd()) }
    }

    /// Make `pgid` the foreground group of the terminal.
    ///
    /// **`tcsetpgrp` from a group that is not the foreground group raises
    /// SIGTTOU at the caller**, and the default action stops it. A process that
    /// reclaims the terminal from its child would stop itself in the act. Ignore
    /// SIGTTOU across the call and restore the previous disposition after it.
    /// That is the standard dance of job control, and it is why this is a method
    /// and not a bare call at three sites.
    fn hand_to(&self, pgid: libc::pid_t) {
        // SAFETY: `signal(2)` and `tcsetpgrp(3)` on a descriptor that this
        // structure owns. The previous disposition returns before this function
        // does.
        unsafe {
            let previous = libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            libc::tcsetpgrp(self.fd(), pgid);
            libc::signal(libc::SIGTTOU, previous);
        }
    }

    /// The attributes of the terminal now, or `None` when they cannot be read.
    fn attributes(&self) -> Option<libc::termios> {
        // SAFETY: `tcgetattr(3)` fills an initialised structure from a
        // descriptor that this structure owns.
        unsafe {
            let mut attributes: libc::termios = std::mem::zeroed();
            (libc::tcgetattr(self.fd(), &mut attributes) == 0).then_some(attributes)
        }
    }

    /// Put back attributes that were saved earlier: cooked mode and echo, for a
    /// child that set the terminal raw and ended before it could undo that.
    ///
    /// SIGTTOU is ignored across the call, for the reason in [`Terminal::hand_to`].
    fn restore(&self, attributes: &libc::termios) {
        // SAFETY: `signal(2)` and `tcsetattr(3)` on a descriptor that this
        // structure owns, with attributes that `tcgetattr` produced. The previous
        // disposition returns before this function does.
        unsafe {
            let previous = libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            libc::tcsetattr(self.fd(), libc::TCSANOW, attributes);
            libc::signal(libc::SIGTTOU, previous);
        }
    }
}

/// The process group of this process.
fn own_group() -> libc::pid_t {
    // SAFETY: `getpgrp(2)` takes no argument and cannot fail.
    unsafe { libc::getpgrp() }
}

````
<!-- /fragment -->

<a id="the-ravel-job-lease"></a>
## The lease: when the terminal comes back

A `Lease` records whether the job ever held the foreground. Having held it is the
gate for the way back. A process that never held the foreground lent nothing, so
it takes nothing and restores nothing.

Three rules decide from whom `reclaim` takes the terminal. After an ordinary
exit it takes the terminal only from the group of `ravel`. After a death by
signal it takes the terminal from any group except its own and the session
leader's. After a failed wait it behaves as after an ordinary exit.

The attributes return whenever Grove ends in the foreground. So a raw-mode
harness that was killed still leaves cooked mode behind.

<!-- fragment «ravel-job-lease» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/ravel_job.rs" lines="653-740" parent="ravel-job" -->
````rust
/// The share of the terminal that one foreground job holds: when it was held,
/// what it looked like then, and how it returns.
///
/// **Having held the foreground is the gate for everything on the way back.**
/// A process in the foreground lends the terminal to its child and owes the
/// human a terminal back. A process that never held it lent nothing. It takes
/// nothing and restores nothing, because anything else would steal the terminal
/// from the job that holds it.
struct Lease {
    terminal: Option<Terminal>,
    child: libc::pid_t,
    /// Whether the job has held the foreground at any moment so far.
    held: bool,
    /// The attributes at the moment the job first held the foreground, when they
    /// could be read.
    saved: Option<libc::termios>,
}

impl Lease {
    /// Before the spawn: whether Grove holds the foreground now. If it does,
    /// answer the descriptor that the child hands itself the terminal through.
    fn hold_before_spawn(&mut self) -> Option<RawFd> {
        let terminal = self.terminal.as_ref()?;
        if terminal.foreground() != own_group() {
            return None;
        }
        self.held = true;
        self.saved = terminal.attributes();
        Some(terminal.fd())
    }

    /// Called at each poll. The check repeats each time, so a driver that started
    /// in the background and later came forward (`grove &`, then `fg`) hands the
    /// terminal to the job that runs under it. It saves the modes only then.
    fn lend(&mut self) {
        let Some(terminal) = &self.terminal else {
            return;
        };
        if terminal.foreground() != own_group() {
            return;
        }
        if !self.held {
            self.held = true;
            self.saved = terminal.attributes();
        }
        terminal.hand_to(self.child);
    }

    /// Take the terminal back after the reap, and restore it.
    ///
    /// **Three rules decide from whom.**
    ///
    /// - After an ordinary exit, take it only from the group of `ravel`. That
    ///   group is gone by now, but the terminal still names it until another
    ///   group receives it. A group that took the terminal on purpose, such as a
    ///   job-control shell nested in this session, keeps it.
    /// - After a death by signal, take it from any group except this one and
    ///   the session leader's. A supervisor that died can leave the group of its
    ///   own child holding the terminal, an orphan in the foreground that reads
    ///   the input of the human. Only this rule takes the terminal back.
    /// - After a failed wait the status is unknown. Take it only from the group
    ///   of `ravel`.
    ///
    /// The attributes return whenever this group ends in the foreground, so a
    /// raw-mode harness that was killed still leaves cooked mode behind.
    fn reclaim(&self, status: Option<ExitStatus>) {
        use std::os::unix::process::ExitStatusExt as _;
        let Some(terminal) = self.terminal.as_ref().filter(|_| self.held) else {
            return;
        };
        let holder = terminal.foreground();
        let own = own_group();
        let take = if status.and_then(|status| status.signal()).is_some() {
            // SAFETY: `getsid(2)` on this process, which cannot fail.
            let leader = unsafe { libc::getsid(0) };
            holder > 0 && holder != own && holder != leader
        } else {
            holder == self.child
        };
        if take {
            terminal.hand_to(own);
        }
        if let Some(saved) = self.saved.as_ref().filter(|_| terminal.foreground() == own) {
            terminal.restore(saved);
        }
    }
}

````
<!-- /fragment -->

<a id="ravel-job-evidence"></a>
## What checks the function

`tests/ravel_job.rs` checks what a caller relies on, with no terminal. It checks
the order of the notices and the environment grant of each shape. It checks that
a failed spawn sends no notice and that the error carries a caught signal. It
uses `/usr/bin/env` and `/usr/bin/true` only for the environment and the notices.

Job control has no stand-in test. `tests/session_command.rs` and
`tests/session_result_installed.rs` run the installed Ravel through the function,
one case for each shape. `task ravel:commands` and `task ravel:result` run them
with `RAVEL` set. Neither case has a terminal, so neither checks the handover.

`tests/ravel_job_terminal.rs` checks the handover. It runs the foreground shape
on a pseudo-terminal of its own. The harness reports which group holds the
terminal while it runs. The case fails when the function does not hand over the
terminal. After the reap, the case checks that the terminal is back with the
group of Grove. `task ravel:job` runs it with `RAVEL` set.

The loop-driver cases under a pseudo-terminal stay the acceptance of the
foreground job after the lifecycle moves here.

No caller can time the window between the last check of the latch and the reap.
So one unit test drives `freeze` with a signal stored in the latch. The test
fails when `freeze` skips the drain.

<!-- fragment «ravel-job-tests» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/ravel_job.rs" lines="741-757" parent="ravel-job" -->
````rust
#[cfg(test)]
mod tests {
    use super::*;

    // A drain that skips when a signal was already sent leaves the HUP in the
    // latch for the caller.
    #[test]
    fn the_freeze_drains_the_latch_when_the_job_already_has_a_signal() {
        INTERRUPTED_BY.store(libc::SIGHUP, Ordering::Relaxed);
        assert_eq!(freeze(Some(libc::SIGTERM)), Some(libc::SIGTERM));
        assert_eq!(take_interrupt(), None);

        INTERRUPTED_BY.store(libc::SIGHUP, Ordering::Relaxed);
        assert_eq!(freeze(None), Some(libc::SIGHUP));
        assert_eq!(take_interrupt(), None);
    }
}
````
<!-- /fragment -->

[Previous: The guaranteed core](18-the-core.md) | [Contents](README.md) | [Next: What could not move](20-what-could-not-move.md)
