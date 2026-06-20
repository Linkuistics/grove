// The `grove-llm complete` verb — the in-loop completion signal (ADR-0032).
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
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Default seconds before SIGTERM (lets the agent's `complete` Bash-tool call
/// return, and the agent's turn end, before its session dies).
const DEFAULT_GRACE: f64 = 2.0;
/// Default seconds after SIGTERM before escalating to SIGKILL.
const DEFAULT_KILL_GRACE: f64 = 5.0;

#[derive(Debug, Clone)]
pub struct CompleteOpts {
    /// PID of the `claude` session to kill. Defaults to `$GROVE_CLAUDE_PID`,
    /// exported by the loop driver (and inherited by the agent's Bash tool).
    pub pid: Option<i32>,
    /// Relaunch-signal file the loop driver polls after `claude` exits.
    /// Defaults to `$GROVE_SIGNAL_FILE`.
    pub signal_file: Option<PathBuf>,
    /// Seconds before the killer sends SIGTERM.
    pub grace: f64,
    /// Seconds after SIGTERM before the killer escalates to SIGKILL.
    pub kill_grace: f64,
}

/// Resolve options from explicit flags, falling back to the loop driver's
/// environment handles and then to built-in defaults.
pub fn resolve_opts(
    pid: Option<i32>,
    signal_file: Option<PathBuf>,
    grace: Option<f64>,
    kill_grace: Option<f64>,
) -> CompleteOpts {
    CompleteOpts {
        pid: pid.or_else(|| env_parse("GROVE_CLAUDE_PID")),
        signal_file: signal_file
            .or_else(|| std::env::var_os("GROVE_SIGNAL_FILE").map(PathBuf::from)),
        grace: grace
            .or_else(|| env_parse("GROVE_KILL_GRACE"))
            .unwrap_or(DEFAULT_GRACE),
        kill_grace: kill_grace
            .or_else(|| env_parse("GROVE_KILL_GRACE_KILL"))
            .unwrap_or(DEFAULT_KILL_GRACE),
    }
}

fn env_parse<T: std::str::FromStr>(key: &str) -> Option<T> {
    std::env::var(key).ok().and_then(|s| s.trim().parse().ok())
}

/// Write the relaunch signal and fork the detached delayed killer. Returns
/// immediately — it never blocks on the kill.
pub fn signal_complete(opts: &CompleteOpts) -> Result<()> {
    if let Some(sig) = &opts.signal_file {
        if let Some(parent) = sig.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating signal-file dir {}", parent.display()))?;
        }
        std::fs::write(sig, "complete\n")
            .with_context(|| format!("writing signal file {}", sig.display()))?;
    } else {
        eprintln!(
            "grove complete: no GROVE_SIGNAL_FILE — the loop driver won't see a relaunch signal"
        );
    }

    match opts.pid {
        Some(pid) => {
            spawn_delayed_killer(pid, opts.grace, opts.kill_grace)?;
            eprintln!(
                "grove complete: task done — this session ends in ~{}s; the loop will start the next task.",
                opts.grace
            );
        }
        None => {
            eprintln!(
                "grove complete: no GROVE_CLAUDE_PID — not running under the loop driver; \
                 exit this session manually to continue."
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
