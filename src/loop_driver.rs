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
//       grove_reverify_skill_stamps                      # restore a clobbered dir
//       # Build pairing is *reported*, never gated: the driver's PATH is only a
//       # proxy for an opaque configured command's (one-build-owns-a-session).
//       # Resolved from the session's cwd, so a relative PATH entry names what
//       # the session would run rather than what the driver's cwd would.
//       llm=$(cd "$worktree" && command -v grove-llm)
//       [ "$(cd "$worktree" && "$llm" --content-hash)" = "<own identity>" ] || echo ...
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
use crate::leaf::Kind;
use crate::session_config::{DeltaRoots, ExpansionContext, SessionConfig};
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
    /// crash); resumable by re-running `grove` from the same working tree.
    ///
    /// Build pairing is deliberately *not* among the reasons: the driver reports
    /// a mismatched, unidentifiable or missing `grove-llm` and launches anyway
    /// (`docs/adr/one-build-owns-a-session.md`).
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
    // The two roots the configuration delta is searched at, taken from the
    // resolution that already happened rather than recomputed here: `repo_path`
    // is `repo::main_repo_of`'s answer and the very value `${repo}` expands to,
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
        driver_lease
            .revalidate()
            .context("revalidating driver lease before loop transition")?;
        // The one artifact Grove owns is repaired; the two things it can only
        // predict — where the methodology landed, and which CLI a session will
        // resolve — are reported. All three run before configuration validation
        // and any tree mutation, so their lines land ahead of any mutation
        // output.
        crate::provision::reverify_installed()?;
        crate::provision::report_absent_skill_destination();
        report_build_pairing(worktree);
        let _pre_transition_config = SessionConfig::load(&home, &delta_roots)?;

        crate::tree_lifecycle::transition_to_current(worktree)?;
        let selection = match crate::task_tree::select(&worktree.join(".grove"))? {
            Some(selection) => selection,
            None => crate::tree_lifecycle::materialize_finish(worktree)?,
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
        let prompt = session_prompt(&selection.handle, selection.kind, worktree)?;
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
            &resolved_source,
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
/// place, and why the two facts below arrive as bare values with no normative
/// tail, is [`crate::prompt`]'s to state and this function's to supply: the
/// selected leaf's stable handle, the stated version control, and the
/// directories provisioning actually wrote on this same iteration
/// (`docs/adr/skill-delivers-the-methodology.md`).
///
/// The kind is passed in rather than re-read: it is the same value that indexed
/// the configuration entry, taken from the one guarded selection, so the prompt
/// and the command a session receives cannot disagree about what kind it is.
///
/// A missing reference file fails the launch rather than degrading it. The suite
/// already asserted every mapped path against the embed, so an error here is a
/// bug — and the answer to it is still not to spawn a session pointed at a file
/// that is not there.
fn session_prompt(handle: &str, kind: Kind, worktree: &Path) -> Result<String> {
    crate::prompt::compose(
        kind,
        handle,
        &stated_vcs(worktree)?,
        &crate::provision::installed_skill_dirs(),
    )
}

/// The **value** that states this working tree's VCS to the session, so no
/// session ever detects it (`docs/ARCHITECTURE.md#symmetric-vcs-rule`).
///
/// The fact is the driver's: [`crate::repo::require_jj_workspace`] is the named
/// authority every tree-mutation verb already passes through, and it resolved
/// before this session existed. Only the session re-derived it, and re-derived
/// it badly — a harness banner computed from `.git` alone reads a jj workspace
/// as no repository at all, and detection carried as skill instructions is
/// skippable, so a session that never loaded them commits with Git in a jj tree
/// and bypasses the operation log.
///
/// **Two elements, and the third one left**: identity and the resolved root, and
/// no *do not probe for it*. That clause is a normative consequence of a value,
/// and the closed fact test hands every such consequence to the skill —
/// `content/SKILL.md`'s `skill-stated-vcs-is-definitive` states it, and stating
/// it here again would be the second source the core exists to avoid
/// (`docs/adr/skill-delivers-the-methodology.md`). This is that closure's one
/// real cost: a rule the prompt used to carry now depends on the skill being
/// read, like every other rule.
///
/// Still deliberately **not** the
/// commit-boundary commands — those live in the methodology's Commit step, and a
/// copy here would drift across the build boundary
/// (`docs/ARCHITECTURE.md#the-boundary-is-a-build-not-a-commit`).
fn stated_vcs(worktree: &Path) -> Result<String> {
    // Unreachable in a driver that got this far: the lease it holds lives *in*
    // the workspace's own `.jj/`, so a marker was already found. Spent as an
    // error rather than a panic — the prompt has no second case to express, and
    // a driver with nothing to say here must not launch a session that then has
    // to guess.
    //
    // Taken from the resolution rather than assumed. It *is* `worktree` — the
    // probe starts its walk at the path itself and the lease root is the
    // marker's own directory — but reading the resolved root cannot drift if
    // either end moves.
    let workspace_root = crate::repo::require_jj_workspace(worktree)
        .context("the session prompt cannot state the version control")?;
    Ok(format!(
        "this working tree is jj-enabled (jj workspace root: `{}`)",
        workspace_root.display()
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
///
/// A spawn failure names `resolved_source` — the file this kind's template was
/// actually read from, personal or delta — rather than the personal path
/// unconditionally, which would name a file that never held the failing
/// template (`docs/adr/untracked-configuration-delta.md`).
fn launch_configured_session(
    argv: &[OsString],
    selection: &crate::task_tree::SelectedLeaf,
    resolved_source: &Path,
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
            resolved_source.display()
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
///
/// Those are the loop's only three exits — the child exits on its own, the
/// signal file appears, or the driver itself is signalled — and a session that
/// finishes its work and simply forgets `grove-llm complete` reaches none of
/// them: an interactive harness returns to its prompt instead of exiting, so
/// the loop **stalls** rather than stopping (see [`crate::complete::read_signal`]
/// for why that was hard to see). The fix shipped for it is at the instruction
/// layer, deliberately: `leaf-retire`/`leaf-prune` name the remaining steps on
/// stderr, and the Signal step composes last in every mandate.
///
/// **The escalation, weighed and set aside.** Give this watcher a *second*
/// completion observable — something it can see without the agent's
/// cooperation — so a forgotten verb costs nothing. It was not taken, because
/// the contract that the agent signals is kept on purpose and no cheap second
/// observable is free of the same ambiguity: nothing here distinguishes a
/// forgotten verb from a session still working. Reopen it only on evidence that
/// sessions still skip the verb *after* a build carrying both instruction-layer
/// fixes — which is evidence no meta-grove can produce for itself, since
/// `content/` and `grove-llm`'s output are fixed at build and install time.
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

/// The agent-side CLI name a session resolves. Not a path: resolution through
/// `PATH` is the whole point of the check below.
const AGENT_CLI: &str = "grove-llm";

/// What the driver could learn about the `grove-llm` a session would run.
enum Pairing {
    /// No `PATH` entry holds an executable `grove-llm`.
    Missing,
    /// One resolved, but it could not name its methodology — too old to answer
    /// `--content-hash`, or answering unparseably.
    Unidentifiable { path: PathBuf, why: String },
    /// One resolved and named a methodology other than this build's.
    Mismatched { path: PathBuf, identity: String },
    /// One resolved and named this build's methodology.
    Paired,
}

/// Report — never gate on — the build pairing a session would get: resolve
/// `grove-llm` the way a session inheriting this environment resolves it, ask it
/// for its methodology identity, and compare with the driver's own.
///
/// **It reports because it is a proxy.** The driver never invokes `grove-llm`,
/// and a configured command may be a wrapper, a login shell, an `ssh` hop or a
/// container that re-derives `PATH` — a supported, deliberately opaque shape in
/// which the driver's environment is simply not the one that matters. So the
/// probe can disagree while the session is correct, and the two errors do not
/// cost the same: a missed mismatch misleads one session, while a false refusal
/// launches nothing at all on a machine that may be configured correctly
/// (`docs/adr/one-build-owns-a-session.md`).
///
/// It deliberately does **not** prefer the sibling of the running executable.
/// That sibling agrees with the driver by construction — `cargo run` builds both
/// side by side — which is exactly what made the motivating case invisible while
/// the session went on resolving the *installed* CLI.
///
/// Per iteration rather than per driver start: a long-running driver keeps
/// executing the text segment it started with while `brew upgrade` replaces the
/// binaries on disk under it, and a mid-loop upgrade is the case a start-time
/// check misses.
///
/// `session_cwd` is the directory the configured session is spawned in — the
/// worktree root, never the driver's own cwd. See [`resolve_in`] for why the
/// difference is load-bearing.
fn report_build_pairing(session_cwd: &Path) {
    let own = crate::methodology::identity();
    // One requirement, not one command. `cargo install --path .` makes the
    // checkout resolve first only where `~/.cargo/bin` outranks every other
    // prefix holding a `grove-llm`; where a package-manager prefix wins, that
    // install is already done and still is not what a session reaches.
    const REQUIREMENT: &str =
        "       the build being driven must be the one a session's PATH resolves first.";
    match resolve_agent_cli_pairing(own, session_cwd) {
        Pairing::Paired => {}
        Pairing::Missing => {
            eprintln!("grove: no `{AGENT_CLI}` on this driver's PATH, so a session inheriting this environment would find none (this build's methodology is {own});");
            eprintln!("{REQUIREMENT}");
        }
        Pairing::Unidentifiable { path, why } => {
            eprintln!("grove: {} could not name its methodology ({why}), so its pairing with this build ({own}) is unknown;", path.display());
            eprintln!("{REQUIREMENT}");
        }
        Pairing::Mismatched { path, identity } => {
            eprintln!(
                "grove: build pairing mismatch — this driver's methodology is {own}, but {} carries {identity};",
                path.display()
            );
            eprintln!("{REQUIREMENT}");
        }
    }
}

fn resolve_agent_cli_pairing(own: &str, session_cwd: &Path) -> Pairing {
    let Some(path) = resolve_on_path(AGENT_CLI, session_cwd) else {
        return Pairing::Missing;
    };
    let mut command = Command::new(&path);
    command.arg("--content-hash");
    // Probed from the cwd the session will have, so a relative `PATH` entry is
    // run as the session would run it and not as the driver's own cwd would.
    command.current_dir(session_cwd);
    // Every spawn that is not the configured session scrubs the loop's
    // launch-scoped environment: this repository is a meta-grove, so the driver
    // itself may be running inside a live session whose kill channel it must not
    // hand down (guard-loop-signal-k37).
    crate::launch::scrub_loop_control_env(&mut command);
    let output = match command.output() {
        Ok(output) => output,
        Err(error) => {
            return Pairing::Unidentifiable {
                path,
                why: format!("could not run it: {error}"),
            }
        }
    };
    if !output.status.success() {
        return Pairing::Unidentifiable {
            path,
            why: format!(
                "`--content-hash` failed ({}) — it may predate the flag",
                output.status
            ),
        };
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let Some(identity) = parse_methodology_identity(&stdout) else {
        return Pairing::Unidentifiable {
            path,
            why: format!("unrecognised `--content-hash` output {:?}", stdout.trim()),
        };
    };
    if identity == own {
        Pairing::Paired
    } else {
        Pairing::Mismatched {
            path,
            identity: identity.to_string(),
        }
    }
}

/// The first executable named `name` on this process's `PATH` — the way a
/// session inheriting this environment would find it, from the cwd that session
/// is given.
fn resolve_on_path(name: &str, session_cwd: &Path) -> Option<PathBuf> {
    resolve_in(&std::env::var_os("PATH")?, name, session_cwd)
}

/// The rule itself, over a `PATH`-shaped value given as an argument.
///
/// Split from the environment read above for the reason `driver_lease`'s own
/// admission split states: a test that set the process `PATH` to exercise this
/// would be writing a global that every parallel sibling test's `Command::new`
/// reads at the same moment. That is not hypothetical here — it failed
/// `an_unsignalled_session_runs_to_its_own_exit_untouched`, which spawns `sh`,
/// the first time this was written as one function.
///
/// An empty entry means the current directory, as every POSIX shell reads it, so
/// it is left to `join` rather than skipped — and **whose** current directory is
/// the reason `session_cwd` is a parameter rather than the process's own cwd.
/// Bare `grove` is deliberately accepted from any directory inside the working
/// tree and keeps that cwd while it resolves the root (`launch::bare_grove`),
/// but the configured session is spawned with the worktree root as its cwd
/// (`launch_configured_session`). Resolving an empty or relative entry against
/// the driver's cwd would therefore inspect one `grove-llm` while the session
/// ran another: from `<worktree>/subdir`, `PATH=:/usr/bin` would probe
/// `<worktree>/subdir/grove-llm` and report on a binary no session can reach.
/// `Path::join` already gives the whole rule — an absolute entry replaces the
/// base, an empty or relative one extends it.
fn resolve_in(search: &std::ffi::OsStr, name: &str, session_cwd: &Path) -> Option<PathBuf> {
    std::env::split_paths(search)
        .map(|directory| session_cwd.join(directory).join(name))
        .find(|candidate| is_executable_file(candidate))
}

fn is_executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .is_ok_and(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
}

/// A lone lowercase-hex SHA-256 on its own line, and nothing else. Anything
/// looser would let a shell's error text or another binary's chatter be
/// "compared" as an identity.
fn parse_methodology_identity(stdout: &str) -> Option<&str> {
    let mut lines = stdout.lines();
    let identity = lines.next()?.trim();
    if lines.any(|line| !line.trim().is_empty()) {
        return None;
    }
    let hex = identity.len() == 64
        && identity
            .chars()
            .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character));
    hex.then_some(identity)
}

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
    use std::os::unix::fs::PermissionsExt;
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
        std::fs::create_dir_all(worktree.path().join(".jj")).unwrap();
        let lease = DriverLease::acquire(worktree.path()).unwrap();

        let first = lease.allocate_signal_channel().unwrap();
        let second = lease.allocate_signal_channel().unwrap();
        let expected_control_dir = worktree.path().canonicalize().unwrap().join(".jj/grove");

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

    /// The identity a binary reports is *compared*, so what counts as one has to
    /// be exactly a lowercase-hex SHA-256 and nothing else. Everything rejected
    /// here would otherwise be reported as a **mismatch** — a wrong claim about
    /// a correctly paired machine — where the honest answer is
    /// *unidentifiable*: an empty read, a shell's own error text, a version line
    /// from a binary predating the flag, a truncated or uppercased digest.
    #[test]
    fn identity_parsing_accepts_a_lone_digest_and_rejects_everything_else() {
        let digest = "a".repeat(64);
        assert_eq!(parse_methodology_identity(&digest), Some(digest.as_str()));
        assert_eq!(
            parse_methodology_identity(&format!("{digest}\n\n")),
            Some(digest.as_str())
        );
        assert_eq!(parse_methodology_identity(""), None);
        assert_eq!(parse_methodology_identity("zsh: command not found\n"), None);
        assert_eq!(parse_methodology_identity("grove-llm 17.0.0\n"), None);
        assert_eq!(parse_methodology_identity(&"a".repeat(63)), None);
        assert_eq!(parse_methodology_identity(&"A".repeat(64)), None);
        assert_eq!(parse_methodology_identity(&"g".repeat(64)), None);
        assert_eq!(
            parse_methodology_identity(&format!("{digest}\ntrailing\n")),
            None
        );
    }

    /// The resolution rule the whole check turns on. It is `PATH` order and only
    /// `PATH` order — never the sibling of the running executable, which agrees
    /// with the driver by construction and so hides the one case worth seeing.
    #[test]
    fn path_resolution_takes_the_first_executable_and_skips_the_rest() {
        let fixture = tempfile::tempdir().unwrap();
        let empty = fixture.path().join("empty");
        let non_executable = fixture.path().join("non-executable");
        let winner = fixture.path().join("winner");
        let loser = fixture.path().join("loser");
        for directory in [&empty, &non_executable, &winner, &loser] {
            std::fs::create_dir_all(directory).unwrap();
        }
        // A same-named *non-executable* file must not win: `PATH` resolution is
        // about what can be run, and a stray data file would otherwise mask the
        // real binary behind it.
        std::fs::write(non_executable.join(AGENT_CLI), "not a program").unwrap();
        for directory in [&winner, &loser] {
            let path = directory.join(AGENT_CLI);
            std::fs::write(&path, "#!/bin/sh\nexit 0\n").unwrap();
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        }

        let search = std::env::join_paths([&empty, &non_executable, &winner, &loser]).unwrap();

        assert_eq!(
            resolve_in(&search, AGENT_CLI, fixture.path()),
            Some(winner.join(AGENT_CLI))
        );
    }

    #[test]
    fn path_resolution_reports_nothing_when_no_entry_holds_the_agent_cli() {
        let fixture = tempfile::tempdir().unwrap();
        let search = std::env::join_paths([fixture.path()]).unwrap();

        assert_eq!(resolve_in(&search, AGENT_CLI, fixture.path()), None);
    }

    /// A relative or empty `PATH` entry is resolved against the cwd the
    /// *session* is spawned with, not the driver's. The driver may be run from
    /// any directory inside the working tree while the session always starts at
    /// the root, so resolving here against the driver's cwd would probe a binary
    /// no session can reach — and could execute an unrelated repository-local
    /// helper while doing it.
    #[test]
    fn relative_and_empty_path_entries_resolve_against_the_sessions_cwd() {
        let fixture = tempfile::tempdir().unwrap();
        let session_cwd = fixture.path().join("worktree");
        let nested = session_cwd.join("nested");
        std::fs::create_dir_all(&nested).unwrap();
        let executable = |path: &Path| {
            std::fs::write(path, "#!/bin/sh\nexit 0\n").unwrap();
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
        };
        executable(&session_cwd.join(AGENT_CLI));
        executable(&nested.join(AGENT_CLI));

        // An empty entry: the session's cwd itself, never the driver's.
        let empty_first = std::ffi::OsString::from(format!(":{}", fixture.path().display()));
        assert_eq!(
            resolve_in(&empty_first, AGENT_CLI, &session_cwd),
            Some(session_cwd.join(AGENT_CLI))
        );

        // A relative entry: extended from the session's cwd.
        let relative = std::ffi::OsString::from("nested");
        assert_eq!(
            resolve_in(&relative, AGENT_CLI, &session_cwd),
            Some(session_cwd.join("nested").join(AGENT_CLI))
        );

        // An absolute entry is unaffected — `join` replaces rather than extends.
        let absolute = std::env::join_paths([&nested]).unwrap();
        assert_eq!(
            resolve_in(&absolute, AGENT_CLI, &session_cwd),
            Some(nested.join(AGENT_CLI))
        );
    }
}
