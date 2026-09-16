# Three steps
<!-- book-page id="three-steps" slice="one-call" order="3" -->
[Previous: The surface](02-the-surface.md) | [Contents](README.md) | [Next: Proving a negative](04-proving-a-negative.md)

<a id="one-call"></a>
## Dispatch, then resolve, lease and run

The CLI first parses the command and chooses the error format; `execute` then
obtains the current directory. For
`view`, it calls the viewer with the supplied path or that directory and
returns. `config show` likewise returns before lease acquisition, but resolves
a workspace and loads launch policy through SessionConfig. The remaining
statements belong to the bare lifecycle. Tree viewing needs no workspace or
configuration; configuration inspection needs no tree, lease or session epoch.

For bare `grove`, the workspace is resolved once and shared with the lease and
loop. The loop returns its reason for stopping; the final match turns that
reason into the process's exit behavior.

<a id="the-block"></a>
## The dispatch block in five fragments

The composite assembles the function's contract and implementation in source
order. The documentation separates ordinary errors from a signal that took the
driver away; the body adds the early observation branch before lifecycle setup.

<!-- fragment «surface-resolve-lease-run» owner="one-call" source="crates/grove/src/cli.rs" lines="107-148" parent="source-command-surface" -->
<!-- insert «run-seam-doc» -->
<!-- insert «run-signal-doc» -->
<!-- insert «run-errors-doc» -->
<!-- insert «run-three-steps» -->
<!-- insert «run-call-and-endings» -->
<!-- /fragment -->

<a id="the-entry-point"></a>
## The entry point

`main.rs` declares the CLI and configuration presentation modules and calls
the CLI’s `run`. Its module comment
explains why the binaries and library are separate packages: the application
entry point should call public seams rather than compile private modules into
itself. The CLI reports errors once and returns an `ExitCode`; main propagates that
status without adding a second message.

<!-- fragment «entry-point-three-steps» owner="one-call" source="crates/grove/src/main.rs" lines="1-16" parent="source-entry-point" -->
<!-- insert «entry-point-module-doc» -->
<!-- insert «entry-point-module-and-main» -->
<!-- /fragment -->

The module documentation owns the crate-boundary explanation. It states why
this entry point calls the public library instead of compiling private loop
modules into the binary.

<!-- fragment «entry-point-module-doc» owner="one-call" source="crates/grove/src/main.rs" lines="1-7" parent="entry-point-three-steps" -->
````rust
//! The human's binary: lifecycle, observation and inactive example delivery.
//!
//! The CLI dispatches `view` and `config` before lifecycle setup. Bare `grove`
//! resolves the working tree and takes the one-driver lease before calling
//! [`grove_loop::run`]. Both application lifetimes sit behind public library
//! entry points; sample delivery stays in the binary. Main propagates exit status
//! (`docs/specs/module-decomposition.md`, decision 9).
````
<!-- /fragment -->

The module declaration and main function implement that boundary: one call
returns the CLI result directly to the process runtime.

<!-- fragment «entry-point-module-and-main» owner="one-call" source="crates/grove/src/main.rs" lines="8-16" parent="entry-point-three-steps" -->
````rust

mod cli;
mod config;
mod config_json;
mod examples;

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

<!-- fragment «run-seam-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="107-115" parent="surface-resolve-lease-run" -->
````rust

/// Dispatch inspection, viewing or example delivery before the lifecycle.
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
| `Cli::try_parse_from` | Shell arguments become an optional observation command; help/version exit here |
| `current_dir` | Capture the caller's working directory, or return its I/O error |
| `grove_tui::run` | A view request observes a path and returns before the following steps |
| `config::show` | Validate and display configuration before lifecycle setup |
| `Workspace::resolve` | Find the enclosing jj workspace for bare invocation |
| `DriverLease::acquire` | Refuse a competing lifecycle driver, otherwise hold the lease |
| `TemplateSource::from_env` | Locate launch configuration for the loop to load |

An unset home directory can fail lifecycle configuration discovery. It is never
consulted for a view request. A non-jj directory can be observed, even though
bare invocation there is refused. Both outcomes follow from the early return.

`config examples` is the earliest branch: it resolves only HOME and returns
before current-directory lookup, workspace discovery or leasing. Missing or broken
active policy and stale ambient epochs therefore cannot change sample delivery.

<!-- fragment «run-three-steps» owner="one-call" source="crates/grove/src/cli.rs" lines="129-142" parent="surface-resolve-lease-run" -->
````rust
fn execute(cli: Cli) -> anyhow::Result<()> {
    if let Some(Command::Config(ConfigCommand::Examples)) = cli.command {
        return crate::examples::run();
    }
    let cwd = std::env::current_dir()?;
    if let Some(Command::View { worktree }) = cli.command {
        return grove_tui::run(&worktree.unwrap_or(cwd));
    }
    if let Some(Command::Config(ConfigCommand::Show { kind, json })) = cli.command {
        return crate::config::show(&cwd, kind.as_deref(), json);
    }
    let workspace = Workspace::resolve(&cwd)?;
    let lease = DriverLease::acquire(&workspace)?;
    let templates = TemplateSource::from_env()?;
````
<!-- /fragment -->

<a id="the-signal-path"></a>
## Preserve an interrupted driver’s exit status

A driver killed by SIGTERM or SIGHUP must not report a clean finish. The loop
cleans up its child, drops the lease as it returns, and reports
`LoopOutcome::Interrupted(signal)`. The CLI then calls `reraise` so its parent
observes termination by that signal. This is the lifecycle's contract; the
viewer's current Ctrl-c handling is an ordinary quit through its own terminal
lifetime, and full signal hardening is a later viewer increment.

<!-- fragment «run-signal-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="116-124" parent="surface-resolve-lease-run" -->
````rust
/// **A driver that was killed does not exit 0.** The loop returns *why* it
/// stopped, and one of the reasons is that this process was sent SIGTERM or
/// SIGHUP mid-grove. Every other reason is an outcome the loop was designed to
/// reach and exits cleanly; that one is the loop being taken away, and the only
/// way to say so through a wait status is to die of the same signal after the
/// cleanup — the lease is dropped by the `run` above, and the session was
/// already reaped by the runner. Whoever started `grove` — a systemd unit, a
/// `timeout(1)`, a shell `wait` — then reads `128 + N` instead of success.
///
````
<!-- /fragment -->

The final match maps the returned lifecycle outcome to clean return or signal
re-raising. The observation commands and sample installer have already returned before this match can run.

<!-- fragment «run-call-and-endings» owner="one-call" source="crates/grove/src/cli.rs" lines="143-148" parent="surface-resolve-lease-run" -->
````rust
    match grove_loop::run(&workspace, lease, &templates)? {
        LoopOutcome::Finished | LoopOutcome::Stopped => Ok(()),
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
  templates: locate personal launch policy
  run: the loop launches each selected session and reads its completion signal
```

The same directory gives a different observation path:

```text
grove view
  parse: View with no explicit path
  current_dir: /work/atlas/src
  view: observe /work/atlas/src/.grove
  return: no workspace resolution, lease or template source
```

If that `.grove` is absent, view shows a Missing state and accepts `r` to retry.
An explicit `grove view /work/atlas` observes the parent's tree without changing
the command's no-upward-search rule.

| Lifecycle outcome | Exit behavior |
|---|---|
| `Finished` | `Ok(())`, process exits 0 |
| `Stopped` | `Ok(())`, process exits 0; a later bare invocation resumes |
| `Interrupted(15)` | Re-raise SIGTERM; shell convention reports 143 |
| `Interrupted(1)` | Re-raise SIGHUP; shell convention reports 129 |

A session that ends without signalling stops the loop; it is different from a
signal sent to the driver itself. The returned outcome preserves that distinction.

<a id="what-run-refuses"></a>
## Errors return through main

Workspace and lease refusals stop the lifecycle before any session launch.
Configuration or runtime failures can stop it later. A viewer can return a
non-TTY refusal or a terminal setup/input/draw error; its terminal owner restores
modes before that error reaches the reporting boundary. `execute` returns the error
to `run`, which writes human or JSON diagnostics and returns exit 1 to main.

<!-- fragment «run-errors-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="125-128" parent="surface-resolve-lease-run" -->
````rust
/// # Errors
///
/// A working tree that is not a jj workspace, a lease another driver holds, or
/// anything the loop refuses, or a viewer terminal setup/input/draw failure.
````
<!-- /fragment -->

<a id="one-iteration"></a>
## The lifecycle behind the call

One lifecycle iteration reads the tree and launch configuration, performs any
required lifecycle transition, selects a leaf, prepares its session epoch and
completion channel, launches the configured command and waits for the result.
The completion signal decides whether to launch the next session or finish;
an ending without that signal stops the loop. Those details belong to the
loop walkthrough at `docs/walkthroughs/grove-loop/README.md`, whose public `run` is this
chapter's boundary. No viewer action enters that loop.

[Previous: The surface](02-the-surface.md) | [Contents](README.md) | [Next: Proving a negative](04-proving-a-negative.md)
