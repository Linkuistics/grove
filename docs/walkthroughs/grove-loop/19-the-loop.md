# The loop
<!-- book-page id="the-loop" slice="four-things-a-runner-cannot-choose" order="19" -->
[Previous: The guaranteed core](18-the-core.md) | [Contents](README.md) | [Next: What could not move](20-what-could-not-move.md)

<a id="four-things-a-runner-cannot-choose"></a>
## The rule: dispatch owns the harness's end

Grove owns the task tree and the loop decision. Dispatch owns one harness run:
it selects the command, spawns the harness as its own job, watches its exit
channel, ends its group, and reports the ending. Grove's runner supervises
dispatch as a channel-free job. This separation lets another caller use dispatch
without implementing a harness watcher.

For a selected `impl` leaf, `drive` composes the prompt, allocates a launch
directory, and runs dispatch in the working-tree root. Dispatch allocates its
exit channel in that directory and hands the terminal to the harness. The
session's last `harness-dispatch exit` creates the channel file; dispatch reaps
the harness and writes `ending.json`. Grove's runner reaps dispatch and restores
the terminal, and Grove invalidates the session epoch before reading either the
ending or the teardown record.

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
loop-driver PTY suite runs the real Grove binary, dispatch front and worker.

<a id="the-harness-widened"></a>
## The harness this chapter had to widen, and the direction the error runs

The fake harness can send dispatch's exit signal, exit by itself, or leave a
raw-mode orphan after dispatch is killed. The PTY cases observe terminal recovery
before `reset_terminal` changes the saved modes. A background driver is launched
beneath a shell that retains the foreground. Creator cases follow the same
retirement procedure as a session and end through the real dispatch exit verb.

<a id="the-block-declared"></a>
## The block, declared

The composite below connects the loop's source fragments in production order.
The definitions follow the reader's path from caller data through launch to
interpretation. The root still includes the inline tests.

<!-- fragment «loop-driver» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="1-723" parent="source-loop-driver" -->
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
<!-- insert «loop-dispatch-run» -->
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
lease and dispatch path; it derives the selection and allocates the control
area. A harness that exits by itself leaves its leaf live.

<!-- fragment «loop-header» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="1-12" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="what-it-imports"></a>
## The loop's vocabulary and runner types

Tree vocabulary belongs to Grove; `Argv`, `Launch`, `Ended`, `End` and `Group`
belong to its runner. An ending report is read by `LaunchDirectory`, keeping
JSON interpretation apart from the loop's selection and terminal reset.

<!-- fragment «loop-imports» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="13-23" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="the-third-choice"></a>
## The third choice: what a child may not inherit

`LOOP_CONTROL_ENV` names inherited authority that must be removed from every
spawn. The current dispatch launch receives only its own `GROVE_LAUNCH_DIR`
after scrubbing. Dispatch grants its fresh exit channel to the harness. The
dispatch job authority and `GROVE_LAUNCH_DIR` are scrubbed in nested launches;
the new driver neither grants nor watches it. The helper gives `stty` the same
scrub list so resetting the terminal carries no completion authority.

<!-- fragment «loop-control-env» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="24-68" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="the-second-choice"></a>
## The launch identity and dispatch's channel

Grove publishes launch identity rather than an exit channel. The directory is
bound into the session epoch before spawning, so only that launch's tree verbs
are admitted. Dispatch's `--exit-dir` chooses where its own fresh channel can be
written; `--ending-file` chooses where it reports the run. Neither reaches policy.

<!-- fragment «loop-scrub-list» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="69-73" parent="loop-driver" -->
````rust
/// [`LOOP_CONTROL_ENV`] as the runner takes it.
fn scrub_list() -> [&'static OsStr; LOOP_CONTROL_ENV.len()] {
    LOOP_CONTROL_ENV.map(OsStr::new)
}

````
<!-- /fragment -->

`scrub_loop_control_env` applies the same names to the terminal-reset command. A helper spawn receives no launch authority.

<!-- fragment «loop-scrub-helper» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="74-84" parent="loop-driver" -->
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
own interruption. The signal number is retained so the human-facing binary can
re-raise it and its parent can distinguish cancellation from success.

<!-- fragment «loop-outcome» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="85-113" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="run"></a>
## `run`: the three things a loop cannot derive

`run` takes the workspace, the lease by value, and the sibling dispatch path.
It ignores driver SIGINT between sessions and delegates to `drive`. Taking the
lease by value releases ownership when the loop returns. A policy refusal is a
stopped launch, while failure to spawn dispatch returns an error.

<!-- fragment «loop-run» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="114-151" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="the-loop-body"></a>
## The loop body: launch, invalidate, interpret

`drive` revalidates the lease, transitions the tree, and obtains one selection.
It then composes the prompt and allocates a fresh launch directory. The runner
forwards driver TERM/HUP to dispatch and waits up to ten seconds before killing
dispatch's group. Dispatch has time to cancel, reap its harness and append the
ending within that budget.

After the reap, terminal reset happens even on an error path. Epoch invalidation
must then succeed before any launch record is interpreted. The directory is
removed only after reading; removal failure reports a warning without changing
the outcome. First match wins: driver interruption, teardown record, exit-signal
ending, then stop. A surviving dispatch group prevents relaunch. A dispatch
that died of a signal can have left its harness alive, so the stop identifies
that possibility instead of treating missing output as completion.

<!-- fragment «loop-drive-open» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="152-161" parent="loop-driver" -->
````rust
fn drive(
    workspace: &Workspace,
    driver_lease: &mut DriverLease,
    dispatch: &Path,
) -> Result<LoopOutcome> {
    // Taken from the lease rather than recomputed: dispatch runs here, which is
    // the location the prompt assumes and the one a policy reads as its `cwd`.
    let worktree_path = driver_lease.worktree_root().to_path_buf();
    let worktree = worktree_path.as_path();

````
<!-- /fragment -->

The top-of-loop interrupt check consumes cancellation that arrived between launches, before Grove mutates the tree or starts another session.

<!-- fragment «loop-drive-interrupt» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="162-176" parent="loop-driver" -->
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

<!-- fragment «loop-drive-selection» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="177-185" parent="loop-driver" -->
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
dispatch's refusal and remedy reach the terminal and the selected leaf remains
live. No route table or owner configuration is read by this module.

<a id="the-selection"></a>
## What `Sought::Nothing` means here

`Sought::Nothing` causes the driver to materialize a finish leaf and select again
under a fresh guard. The retained `TreeLifetime` pins the selected root's
identity without retaining its tree lock across a session.

<!-- fragment «loop-drive-invocation» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="186-188" parent="loop-driver" -->
````rust

        let prompt = session_prompt(&selection.handle, &selection.kind, workspace);

````
<!-- /fragment -->

The launch directory is allocated after lease validation. Its path becomes both session identity and the directory dispatch uses for its run mechanics.

<!-- fragment «loop-drive-launch» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="189-210" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

After runner recovery, reset happens unconditionally. Epoch invalidation gates the reading closure, and directory removal preserves the interpreted outcome even if cleanup fails.

<!-- fragment «loop-drive-discard» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="211-228" parent="loop-driver" -->
````rust
        if let Err(error) = launch_dir.discard() {
            eprintln!(
                "grove: warning: could not remove the interpreted foreground-session launch directory; preserving the session outcome: {error}"
            );
        }

        if let End::Interrupted { signal } = ended.end {
            eprintln!("grove: interrupted by signal {signal} — stopping the loop.");
            return Ok(LoopOutcome::Interrupted(signal));
        }

        // Teardown is this launch's explicit disposition, irrespective of the
        // harness's ending. The driver interrupt above remains authoritative.
        if reading.teardown {
            eprintln!("grove: grove finished — loop complete.");
            return Ok(LoopOutcome::Finished);
        }

````
<!-- /fragment -->

A remaining dispatch group is reported with the selected handle. A relaunch beside a survivor would let two processes keep changing the tree.

<!-- fragment «loop-drive-survivor» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="229-241" parent="loop-driver" -->
````rust
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
            return Ok(LoopOutcome::Stopped);
        }

````
<!-- /fragment -->

The final branch relaunches only on the supported exit-signal ending. Other reports or missing reports stop, with dispatch death identified as a possible surviving harness.

<!-- fragment «loop-drive-endings» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="242-265" parent="loop-driver" -->
````rust
        if reading.exit_signal {
            continue;
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
        return Ok(LoopOutcome::Stopped);
    }
}

````
<!-- /fragment -->

<a id="session-prompt"></a>
## `session_prompt`: a pointer, not the methodology

The helper renders the selected handle, kind, workspace and published version
through the prompt module. The same selection supplies dispatch's native kind,
task path and handle, so the mandate and policy inputs cannot name different
leaves. The signalling contract now names dispatch's exit verb and Grove's
separate teardown record.

<!-- fragment «loop-session-prompt» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="266-299" parent="loop-driver" -->
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

````
<!-- /fragment -->

<a id="dispatch-run"></a>
## `dispatch_run`: everything the policy may select from

`dispatch_run` builds six whole flag/value words after `run`: kind, task file,
task ID, prompt, exit directory and ending file. Joined flag values preserve
paths and prompts beginning with dashes. The first four are selection data; the
last two are run mechanics. The working-tree root is supplied as cwd at launch.

<!-- fragment «loop-dispatch-run» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="300-332" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="launch"></a>
## `launch_session`: a channel-free supervisor job

`launch_session` activates the selected root and launch identity before the
spawn. `supervise_launch` carries the witness across runner start and reap events.
The runner receives `channel: None`, the scrub list and only the launch-directory
grant. Dispatch then makes the harness its own foreground job.

<!-- fragment «loop-launch-contract» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="333-361" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

The epoch already names this launch when the runner spawns dispatch. Witness events surround the job, and a channel-free launch forwards only caller cancellation.

<!-- fragment «loop-launch-spawn» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="362-389" parent="loop-driver" -->
````rust
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

````
<!-- /fragment -->

<a id="the-handoff"></a>
## The handoff: the one ordering the loop cannot get wrong

The handoff invalidates the epoch even when launch returned an error. Only a
successful invalidation invokes the reading closure. If both fail, the error
retains the launch failure beside the invalidation failure; nothing consumes the
ending beside an admitted orphan still holding the old epoch.

<!-- fragment «loop-handoff» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="390-407" parent="loop-driver" -->
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
no channel. The ten-second kill-grace bounds forwarded cancellation of dispatch,
covering its harness shutdown, group confirmation and record-store lock wait.
Grove does not choose the harness's exit-signal graces.

<!-- fragment «loop-escalation» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="408-416" parent="loop-driver" -->
````rust
/// No channel grace: Grove ends no harness. Its own TERM/HUP is forwarded
/// to dispatch, whose cancellation must finish before this 10-second kill-grace.
/// Dispatch budgets 5 seconds for the harness, 1 for group confirmation and
/// 2 for its record-store lock (decision 7).
const ESCALATION: Escalation = Escalation {
    grace: Duration::ZERO,
    kill_grace: Duration::from_secs(10),
};

````
<!-- /fragment -->

<a id="reset-terminal"></a>
## `reset_terminal`: a guard that is a hang, not a tidiness

The runner restores the attributes saved when Grove handed over the terminal.
After dispatch's signal death it can reclaim from the orphaned harness's group,
not just dispatch's. `reset_terminal` then runs `stty sane`, leaves the alternate
screen and shows the cursor. It runs only on tty stdin while the driver owns the
foreground; otherwise an `stty` child could stop on SIGTTOU and hang the driver.

<!-- fragment «loop-reset-terminal» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="417-448" parent="loop-driver" -->
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
child's terminal-signal dispositions so dispatch and the harness receive typed
Ctrl-C. TERM/HUP belong to runner cancellation and are forwarded to the job.

<!-- fragment «loop-ignore-interrupts» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="449-479" parent="loop-driver" -->
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

<!-- fragment «loop-picked» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="480-519" parent="loop-driver" -->
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

The inline tests cover root pinning, finish materialization, admission before
spawn and the invalidation gate's error precedence. They exercise the real
dispatch front for the launch case. The process suite supplies the PTY ending,
terminal, refusal, stale-session and cancellation cases.

<!-- fragment «loop-tests-open» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="520-664" parent="loop-driver" -->
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
````
<!-- /fragment -->

This case supplies simultaneous launch and invalidation failures. It checks that both causes remain visible and interpretation never runs.

<!-- fragment «loop-test-handoff-preserves» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="665-700" parent="loop-driver" -->
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

This case isolates the invalidation gate. Even a successful reap cannot cause the reading closure to run before exclusive epoch admission is obtained.

<!-- fragment «loop-test-ordering» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/loop_driver.rs" lines="701-723" parent="loop-driver" -->
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

The executable seams hold distinct obligations: the runner's job tests hold
job control and recovery; dispatch's suite holds harness escalation and ending
reports; Grove's PTY suite holds composition of those mechanisms and the loop's
reading table. An exit status alone cannot replace the ending report.

<a id="the-zeros"></a>
## The ownership boundaries

Grove allocates no harness channel and reads no owner policy or run record.
These are ownership boundaries visible in the launch and reading functions,
not conclusions drawn from missing grep matches. It retains the ten-second
cancellation bound because dispatch remains its child.

<a id="what-could-not-move-here"></a>
## What could not move

The loop's choices stay here: which leaf runs, what prompt it receives, where
its launch identity lives, and which ending continues the tree. The runner has
no task vocabulary. Dispatch has no Grove vocabulary. A teardown remains
Grove's deliberate record rather than payload in dispatch's pure exit signal.

<a id="launch-control-area"></a>
## The launch directory carries identity, ending and teardown

`LaunchDirectory` creates an owner-only directory with a fresh random suffix.
Its reading checks for a regular teardown record and a bounded, supported
JSON report from dispatch. Only an observed `exit_signal` relaunches; a missing,
unreadable, malformed or unsupported report stops. The report is opened without
following symlinks and nonblocking, then checked to be regular, so a FIFO cannot
hang interpretation. The unit cases cover report shape and a symlink control.

`record_teardown` refuses while `.grove/` exists, names `finish-commit`, and
creates its record exclusively. Replacement-driver cleanup reads no abandoned
contents. The real finish seam exercises record refusal, finish-commit,
idempotent recording and both the exit signal and the harness's own exit.

<!-- fragment «launch-control-directory» owner="four-things-a-runner-cannot-choose" source="crates/grove-loop/src/launch_directory.rs" lines="1-193" parent="source-launch-directory" -->
````rust
//! Grove's launch-scoped control area: identity, interpretation and removal.

use std::ffi::OsStr;
use std::fs::{self, DirBuilder, OpenOptions};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use std::io::Read;

pub(crate) struct LaunchDirectory(PathBuf);

pub(crate) struct LaunchReading {
    pub(crate) teardown: bool,
    pub(crate) exit_signal: bool,
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

    /// Interpretation happens only after dispatch's reap and epoch invalidation.
    /// Missing, unreadable, malformed or unsupported reports never relaunch.
    pub(crate) fn read(&self) -> LaunchReading {
        let exit_signal = (|| {
            let mut bytes = Vec::new();
            let file = OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
                .open(self.0.join("ending.json"))
                .ok()?;
            if !file.metadata().ok()?.is_file() {
                return None;
            }
            file.take(1024 * 1024 + 1).read_to_end(&mut bytes).ok()?;
            if bytes.len() > 1024 * 1024 {
                return None;
            }
            let report: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
            Some(
                report["schemaVersion"] == 1
                    && report["source"] == "harness-dispatch"
                    && report["measurements"]["ending"]["state"] == "observed"
                    && report["measurements"]["ending"]["value"] == "exit_signal",
            )
        })()
        .unwrap_or(false);
        LaunchReading {
            teardown: fs::symlink_metadata(self.0.join("teardown"))
                .is_ok_and(|metadata| metadata.file_type().is_file()),
            exit_signal,
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
    fn only_a_supported_observed_exit_signal_relaunches() {
        let control = tempfile::tempdir().unwrap();
        let launch = LaunchDirectory::allocate(control.path()).unwrap();
        let report = launch.path().join("ending.json");
        assert!(!launch.read().exit_signal);
        for (document, want) in [
            (
                r#"{"schemaVersion":1,"source":"harness-dispatch","measurements":{"ending":{"state":"observed","value":"exit_signal"}}}"#,
                true,
            ),
            (
                r#"{"schemaVersion":1,"source":"harness-dispatch","measurements":{"ending":{"state":"observed","value":"harness_exit"}}}"#,
                false,
            ),
            (
                r#"{"schemaVersion":1,"source":"harness-dispatch","measurements":{"ending":{"state":"observed","value":"cancelled"}}}"#,
                false,
            ),
            (
                r#"{"schemaVersion":2,"source":"harness-dispatch","measurements":{"ending":{"state":"observed","value":"exit_signal"}}}"#,
                false,
            ),
            (
                r#"{"schemaVersion":1,"source":"other","measurements":{"ending":{"state":"observed","value":"exit_signal"}}}"#,
                false,
            ),
            (
                r#"{"schemaVersion":1,"source":"harness-dispatch","measurements":{"ending":{"state":"unknown","value":"exit_signal"}}}"#,
                false,
            ),
            ("{}", false),
            ("broken", false),
        ] {
            fs::write(&report, document).unwrap();
            assert_eq!(launch.read().exit_signal, want, "{document}");
        }
        fs::remove_file(&report).unwrap();
        fs::create_dir(&report).unwrap();
        assert!(!launch.read().exit_signal);
    }

    #[test]
    fn an_ending_symlink_does_not_relaunch() {
        let control = tempfile::tempdir().unwrap();
        let launch = LaunchDirectory::allocate(control.path()).unwrap();
        let other = control.path().join("other-ending");
        fs::write(&other, r#"{"schemaVersion":1,"source":"harness-dispatch","measurements":{"ending":{"state":"observed","value":"exit_signal"}}}"#).unwrap();
        std::os::unix::fs::symlink(other, launch.path().join("ending.json")).unwrap();
        assert!(!launch.read().exit_signal);
    }
}
````
<!-- /fragment -->

[Previous: The guaranteed core](18-the-core.md) | [Contents](README.md) | [Next: What could not move](20-what-could-not-move.md)
