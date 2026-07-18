// The `grove-llm complete` verb — the in-loop completion signal (self-driving-loop).
//
// The agent runs this as its **last step** of a task (after commit + retire).
// It is the "external exit" an interactive `claude` cannot perform on itself:
// finishing a turn does not make `claude` quit, so the loop driver needs an
// out-of-band kill triggered on the agent's command.
//
// Realisation = the *self-spawned delayed killer* (030 D4 option (b)): the verb
// (1) writes the signal file so the loop driver knows to relaunch, then (2)
// forks a fully-detached helper that waits a short grace and kills the `claude`
// session (SIGTERM, then SIGKILL), and (3) returns immediately. The grace lets
// this very Bash-tool call return cleanly before its own session dies.

use anyhow::{Context, Result};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Default seconds before SIGTERM (lets the agent's `complete` Bash-tool call
/// return, and the agent's turn end, before its session dies).
const DEFAULT_GRACE: f64 = 2.0;
/// Default seconds after SIGTERM before escalating to SIGKILL.
const DEFAULT_KILL_GRACE: f64 = 5.0;

/// What a finished session tells the self-driving loop to do next. The agent
/// picks this when it signals; the loop driver reads it back from the signal
/// file (self-driving-loop). The third case — *no* signal at all (human `/exit`/Ctrl-C
/// or a crash) — is the *absence* of a [`Disposition`], represented by
/// [`read_signal`] returning `None`, so the loop can tell a clean finish from
/// an abnormal exit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    /// Relaunch with fresh context for the next task (the default — today's
    /// behaviour, fired after every per-task commit + retire).
    Relaunch,
    /// The whole grove is finished — stop the loop cleanly (the Finish cycle's
    /// last teardown action, `grove-llm complete --done`).
    Done,
}

impl Disposition {
    /// The sentinel written to / read from the signal file. Only `Done` needs a
    /// distinguished token; any other (or unrecognised, e.g. a stale binary's
    /// legacy `"complete"`) content reads back as `Relaunch`, the safe default.
    const DONE_TOKEN: &'static str = "done";

    fn token(self) -> &'static str {
        match self {
            Disposition::Relaunch => "relaunch",
            Disposition::Done => Self::DONE_TOKEN,
        }
    }
}

/// Read the disposition a finished session left in its signal file. `None` =
/// no signal file (the session exited without signalling: human `/exit`/Ctrl-C
/// or a crash → the loop stops). `Some(Done)` = a clean whole-grove finish;
/// `Some(Relaunch)` = relaunch the next task (also the backward-compatible
/// reading of any present-but-unrecognised content).
pub fn read_signal(path: &Path) -> Option<Disposition> {
    let content = std::fs::read_to_string(path).ok()?;
    if content.trim() == Disposition::DONE_TOKEN {
        Some(Disposition::Done)
    } else {
        Some(Disposition::Relaunch)
    }
}

#[derive(Debug, Clone)]
pub struct CompleteOpts {
    /// PID of the harness session to kill. Defaults to $GROVE_HARNESS_PID,
    /// exported by the loop driver (and inherited by the agent's Bash tool).
    pub pid: Option<i32>,
    /// Relaunch-signal file the loop driver polls after `claude` exits.
    /// Defaults to `$GROVE_SIGNAL_FILE`.
    pub signal_file: Option<PathBuf>,
    /// Seconds before the killer sends SIGTERM.
    pub grace: f64,
    /// Seconds after SIGTERM before the killer escalates to SIGKILL.
    pub kill_grace: f64,
    /// What the loop should do once this session dies: relaunch the next task
    /// (default) or finish the whole grove (`--done`).
    pub disposition: Disposition,
}

/// Resolve options from explicit flags, falling back to the loop driver's
/// environment handles and then to built-in defaults. `disposition` comes
/// straight from the verb (no env fallback): the default verb relaunches, the
/// `--done` flag finishes.
pub fn resolve_opts(
    pid: Option<i32>,
    signal_file: Option<PathBuf>,
    grace: Option<f64>,
    kill_grace: Option<f64>,
    disposition: Disposition,
) -> CompleteOpts {
    CompleteOpts {
        pid: pid
            .or_else(|| env_parse("GROVE_HARNESS_PID"))
            // One release of fallback for the pre-rename handle.
            .or_else(|| env_parse("GROVE_CLAUDE_PID"))
            // A non-positive PID must never reach `kill`: 0 or a negative
            // value signals a process group or every process the caller can
            // signal, not one specific session (B10).
            .filter(|p| *p > 0),
        signal_file: signal_file
            .or_else(|| std::env::var_os("GROVE_SIGNAL_FILE").map(PathBuf::from)),
        grace: grace
            .or_else(|| env_parse("GROVE_KILL_GRACE"))
            .unwrap_or(DEFAULT_GRACE),
        kill_grace: kill_grace
            .or_else(|| env_parse("GROVE_KILL_GRACE_KILL"))
            .unwrap_or(DEFAULT_KILL_GRACE),
        disposition,
    }
}

fn env_parse<T: std::str::FromStr>(key: &str) -> Option<T> {
    std::env::var(key).ok().and_then(|s| s.trim().parse().ok())
}

/// Write the disposition signal and fork the detached delayed killer. Returns
/// immediately — it never blocks on the kill. The out-of-band kill is the same
/// for both dispositions (the agent still cannot quit its own session); only
/// the token written — which the loop driver reads back — differs.
pub fn signal_complete(opts: &CompleteOpts) -> Result<()> {
    if let Some(sig) = &opts.signal_file {
        if let Some(parent) = sig.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating signal-file dir {}", parent.display()))?;
        }
        std::fs::write(sig, format!("{}\n", opts.disposition.token()))
            .with_context(|| format!("writing signal file {}", sig.display()))?;
    } else {
        eprintln!(
            "grove complete: no GROVE_SIGNAL_FILE — the loop driver won't see the completion signal"
        );
    }

    match opts.pid {
        Some(pid) => {
            spawn_delayed_killer(pid, opts.grace, opts.kill_grace)?;
            let tail = match opts.disposition {
                Disposition::Relaunch => "the loop will start the next task",
                Disposition::Done => "the grove is finished — the loop will stop",
            };
            eprintln!(
                "grove complete: this session ends in ~{}s; {tail}.",
                opts.grace
            );
        }
        None => {
            eprintln!(
                "grove complete: no GROVE_HARNESS_PID — not running under the loop driver; \
                 exit this session manually."
            );
        }
    }
    Ok(())
}

/// Fork a fully-detached helper (own session via `setsid`, stdio to /dev/null)
/// that waits `grace`, sends SIGTERM, waits `kill_grace`, then sends SIGKILL.
/// Detached so it outlives both this `complete` process and the `claude`
/// session it kills.
fn spawn_delayed_killer(pid: i32, grace: f64, kill_grace: f64) -> Result<()> {
    let script = format!(
        "sleep {grace}; kill -TERM {pid} 2>/dev/null; sleep {kill_grace}; kill -KILL {pid} 2>/dev/null"
    );
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(script)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    // SAFETY: setsid() is async-signal-safe; the forked child is single-threaded
    // between fork and exec.
    unsafe {
        cmd.pre_exec(|| {
            libc::setsid();
            Ok(())
        });
    }
    cmd.spawn()
        .context("spawning the detached delayed killer")?;
    Ok(())
}
