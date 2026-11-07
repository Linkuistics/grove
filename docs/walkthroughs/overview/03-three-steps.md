# Lifecycle startup
<!-- book-page id="three-steps" slice="one-call" order="3" -->
[Previous: The surface](02-the-surface.md) | [Contents](README.md) | [Next: Proving a negative](04-proving-a-negative.md)

<a id="one-call"></a>
## Dispatch, then prepare the lifecycle

The CLI first parses the command; `execute` then
dispatches standalone invocation and its hidden log viewer
before obtaining the current directory. For
`view`, it calls the viewer with the supplied path or that directory and
returns. The remaining
statements belong to the bare lifecycle. Tree viewing needs no workspace and
no Ravel policy.

For bare `grove`, the workspace is resolved once and shared with the lease and
loop. Ravel is then resolved, and the loop is called. Startup installs no
skills. The loop returns its reason
for stopping; the final match turns that
reason into the process's exit behavior.

<a id="the-block"></a>
## The dispatch block in five fragments

The composite assembles the function's contract and implementation in source
order. The documentation separates ordinary errors from a signal that took the
driver away; the body returns from standalone, display and observation operations
before lifecycle setup.

<!-- fragment «surface-resolve-lease-run» owner="one-call" source="crates/grove/src/cli.rs" lines="67-125" parent="source-command-surface" -->
<!-- insert «run-seam-doc» -->
<!-- insert «run-signal-doc» -->
<!-- insert «run-errors-doc» -->
<!-- insert «run-three-steps» -->
<!-- insert «run-call-and-endings» -->
<!-- /fragment -->

<a id="the-entry-point"></a>
## The entry point

`main.rs` declares the CLI, dispatch-lookup, standalone and display modules and calls
the CLI’s `run`. Its module comment
explains why the binaries and library are separate packages: the application
entry point should call public seams rather than compile private modules into
itself. The CLI reports errors once and returns an `ExitCode`; main propagates that
status without adding a second message.

<!-- fragment «entry-point-three-steps» owner="one-call" source="crates/grove/src/main.rs" lines="1-16" parent="source-entry-point" -->
<!-- insert «entry-point-module-doc» -->
<!-- insert «entry-point-module-and-main» -->
<!-- /fragment -->

The module documentation states which actor owns each path. The loop and viewer
sit behind public library entry points; standalone artifact handling remains a
binary adapter. Main propagates the selected exit status.

<!-- fragment «entry-point-module-doc» owner="one-call" source="crates/grove/src/main.rs" lines="1-7" parent="entry-point-three-steps" -->
````rust
//! The human's binary: lifecycle, standalone work and observation.
//!
//! The CLI dispatches `run` and `view` before lifecycle setup. Bare `grove`
//! resolves the working tree and takes the one-driver lease before calling
//! [`grove_loop::run`]. The loop and viewer use public library entry points;
//! standalone artifact handling stays here. Main propagates exit status
//! (`docs/specs/module-decomposition.md`, decision 9).
````
<!-- /fragment -->

The module declaration and main function implement that boundary: one call
returns the CLI result directly to the process runtime.

<!-- fragment «entry-point-module-and-main» owner="one-call" source="crates/grove/src/main.rs" lines="8-16" parent="entry-point-three-steps" -->
````rust

mod cli;
mod dispatch;
mod run_display;
mod standalone;

fn main() -> std::process::ExitCode {
    cli::run()
}
````
<!-- /fragment -->

<a id="one-resolution"></a>
## One workspace identity

The loop and lease need the same resolved workspace. Resolving independently
would leave two answers that could differ after a path change. `Workspace::resolve`
therefore runs once and passes its result to both consumers. This contract is
about the lifecycle; the viewer's absolute observation path is not a workspace
resolution.

<!-- fragment «run-seam-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="67-75" parent="surface-resolve-lease-run" -->
````rust

/// Dispatch a standalone invocation or the viewer before the lifecycle.
///
/// The workspace is resolved **here**, once, and handed to both the lease and
/// the loop. That is the shape `loop-crate-driver-k22` gave the seam: the lease
/// used to resolve a path itself and hold the answer, so a caller that also
/// needed a workspace had two derivations of one fact and no way to see that
/// they agreed (`docs/adr/one-live-driver-per-working-tree.md`).
///
````
<!-- /fragment -->

<a id="the-three-steps"></a>
## The setup statements

| Statement | Input and result |
|---|---|
| `Cli::parse` | Shell arguments become an optional command; help, version and usage refusals exit here |
| `standalone::run` | Execute one kind and return before workspace setup |
| `run_display::watch` | Display a parent-selected log and status file |
| `current_dir` | Capture the caller's working directory, or return its I/O error |
| `grove_tui::run` | A view request observes a path and returns before the following steps |
| `Workspace::resolve` | Find the enclosing jj workspace for bare invocation |
| `DriverLease::acquire` | Refuse a competing lifecycle driver, otherwise hold the lease |
| `dispatch::ravel::locate` | Resolve/version-check independent Ravel once from absolute PATH entries |

Missing Ravel on PATH refuses with its independent installation remedy before
scaffolding or launch. Resolution never happens on a view request. A non-jj
directory can be observed, even though
bare invocation there is refused. Both outcomes follow from the early return.

<!-- fragment «run-three-steps» owner="one-call" source="crates/grove/src/cli.rs" lines="91-104" parent="surface-resolve-lease-run" -->
````rust
fn execute(cli: Cli) -> anyhow::Result<()> {
    if let Some(Command::Run(args)) = cli.command {
        return crate::standalone::run(args);
    }
    if let Some(Command::RunLog { log, status }) = cli.command {
        return crate::run_display::watch(&log, &status);
    }
    let cwd = std::env::current_dir()?;
    if let Some(Command::View { worktree }) = cli.command {
        return grove_tui::run(&worktree.unwrap_or(cwd));
    }
    let workspace = Workspace::resolve(&cwd)?;
    let lease = DriverLease::acquire(&workspace)?;
    let ravel = crate::dispatch::locate()?;
````
<!-- /fragment -->

<a id="codex-provisioning"></a>
## No skill installation before launch

The lease is followed directly by Ravel resolution. No statement between them
reads or writes a harness's skill directory. Homebrew installs each skill
source, and Ravel delivers it to the session, as the decision record
`docs/adr/homebrew-installs-skills-and-ravel-delivers-them.md` states. Ravel's
native routes still serve a native installation that already exists.

The prompt names the bare skill, and the session loads it. If the session
cannot find or read that skill, the prompt tells it to report the problem and
run the launch's exact force-stop command. The leaf stays live, and the loop
stops without relaunching it.

Grove 22.8 and earlier called a private Codex provisioner at this point. A
build script embedded every plugin file, and startup published links in
`~/.agents/skills` through snapshots under `~/.agents/.grove`. That module and
its build script are gone. Startup now leaves their output unchanged, because
old drivers and lazy readers can still resolve it. The crate's
`tests/native_skill_delivery.rs` compares those entries before and after
startup.

<a id="the-signal-path"></a>
## Preserve an interrupted driver’s exit status

A driver killed by SIGTERM or SIGHUP must not report a clean finish. The loop
cleans up its child, drops the lease as it returns, and reports
`LoopOutcome::Interrupted(signal)`. The CLI then calls `reraise`. Its parent
then observes termination by that signal.

This is the lifecycle's contract. An error of the loop keeps the signal too. The
CLI reports the error, and then it dies of the signal. The viewer's current
Ctrl-c handling is an ordinary quit through its own terminal lifetime. Full
signal hardening is a later viewer increment.

<!-- fragment «run-signal-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="76-86" parent="surface-resolve-lease-run" -->
````rust
/// **A driver that was killed does not exit 0, and an error does not hide it.**
/// The loop returns *why* it stopped, and one of the reasons is that this process
/// was sent SIGTERM or SIGHUP mid-grove. An error of the loop can carry that
/// signal too. The driver then reports the error and dies of the signal.
/// An explicit force-stop returns failure; ordinary stops and
/// completion exit cleanly. Interruption is the loop being taken away, and the only
/// way to say so through a wait status is to die of the same signal after the
/// cleanup — the lease is dropped by the `run` above, and the session was
/// already reaped by the runner. Whoever started `grove` — a systemd unit, a
/// `timeout(1)`, a shell `wait` — then reads `128 + N` instead of success.
///
````
<!-- /fragment -->

The call comes first. The CLI reports an error from the loop that carries a
signal, and then it re-raises that signal. An error with no signal returns to
`run`, which exits 1.

The final match maps lifecycle completion and ordinary stops to clean return,
explicit force-stop to failure, and interruption to signal re-raising.
The standalone runner, log viewer and tree viewer have already returned before
this match can run.

<!-- fragment «run-call-and-endings» owner="one-call" source="crates/grove/src/cli.rs" lines="105-125" parent="surface-resolve-lease-run" -->
````rust
    let outcome = match grove_loop::run(&workspace, lease, &ravel) {
        Ok(outcome) => outcome,
        // An error that stops the loop is reported first. A signal that Grove
        // caught then ends the driver, as it does without an error.
        Err(error) => match error.interrupted() {
            Some(signal) => {
                report(&error);
                grove_loop::reraise(signal)
            }
            None => return Err(error.into()),
        },
    };
    match outcome {
        LoopOutcome::Finished | LoopOutcome::Stopped => Ok(()),
        LoopOutcome::ForceStopped => {
            anyhow::bail!("Grove force-stop requested; work did not complete")
        }
        LoopOutcome::Interrupted(signal) => grove_loop::reraise(signal),
    }
}

````
<!-- /fragment -->

<a id="worked-run"></a>
## Worked example: two commands from one directory

From `/work/atlas/src`, with `/work/atlas` a jj workspace:

```text
grove
  parse: no subcommand
  current_dir: /work/atlas/src
  resolve: workspace /work/atlas
  lease: claim the lifecycle driver for /work/atlas
  dispatch: resolve/version-check independent Ravel on PATH
  run: the loop launches each selected session and reads its completion signal
```

The same directory gives a different observation path:

```text
grove view
  parse: View with no explicit path
  current_dir: /work/atlas/src
  view: observe /work/atlas/src/.grove
  return: no workspace resolution, lease or dispatch lookup
```

If that `.grove` is absent, view displays a Missing state and accepts `r` to retry.
An explicit `grove view /work/atlas` observes the parent's tree without changing
the command's no-upward-search rule.

| Lifecycle outcome | Exit behaviour |
|---|---|
| `Finished` | `Ok(())`, process exits 0 |
| `Stopped` | `Ok(())`, process exits 0. A later bare invocation resumes |
| `Interrupted(15)` | Re-raise SIGTERM. Shell convention reports 143 |
| `Interrupted(1)` | Re-raise SIGHUP. Shell convention reports 129 |
| An error that carries a signal | Print the error, then re-raise that signal |

A session that ends without signalling stops the loop. That differs from a
signal sent to the driver itself. The returned outcome preserves that
distinction.

<a id="what-run-refuses"></a>
## Errors return through main

Workspace, lease and missing/incompatible-Ravel refusals stop the
lifecycle before any session launch. Runtime failures can stop it later. A launch
that the owner's policy refuses is not one of them, because the loop stops on it
and returns cleanly.

A viewer can return a non-TTY refusal or a terminal setup/input/draw error. Its terminal owner restores the modes before that error
reaches the reporting boundary. `execute` returns the error to `run`. `run`
writes the error and returns exit 1 to main. An error of the loop that carries a
signal is the exception. `execute` writes it and dies of the signal.

<!-- fragment «run-errors-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="87-90" parent="surface-resolve-lease-run" -->
````rust
/// # Errors
///
/// A working tree that is not a jj workspace, a lease another driver holds, or
/// anything the loop refuses, or a viewer terminal setup/input/draw failure.
````
<!-- /fragment -->

<a id="one-iteration"></a>
## The lifecycle behind the call

One lifecycle iteration reads the tree and performs any required lifecycle
transition. It selects a leaf and prepares the session epoch and the launch
directory. Then it runs `ravel start-session` for that leaf and holds the public
stdout result.

After the reap and the epoch invalidation, the iteration reads the end in this
order. Interruption of the driver comes first, including a signal that arrives
after the reap. A successful status with exactly `grove-force-stop` comes next,
and it stops the loop without completion. A recorded teardown then finishes the
loop. On success, exactly `grove-continue` starts the next pick. Any other end
stops the loop, including an unsuccessful status.

Ravel owns the completion, the escalation and the cleanup of the managed process.
Grove takes a successful Ravel status as the only evidence of cleanup, and it
probes no process group. Grove supplies the instruction and gives the result its
meaning. The loop walkthrough at `docs/walkthroughs/grove-loop/README.md`
describes these details. Its public `run` is the boundary of this chapter. No
viewer action enters that loop.

[Previous: The surface](02-the-surface.md) | [Contents](README.md) | [Next: Proving a negative](04-proving-a-negative.md)
