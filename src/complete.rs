// The `grove-llm complete` verb — the in-loop completion signal (self-driving-loop).
//
// The agent runs this as its **last step** of a task (after commit + retire).
// It is the "external exit" an interactive `claude` cannot perform on itself:
// finishing a turn does not make `claude` quit, so the loop needs an
// out-of-band kill triggered on the agent's command.
//
// Realisation: this verb only writes the disposition into the signal file.
// The out-of-band kill itself is the loop driver's job (src/loop_driver.rs),
// which watches for this file and applies grace → SIGTERM → kill-grace →
// SIGKILL to the harness session it spawned (driver-side watcher). That split
// exists because an in-agent self-kill cannot be trusted under every harness
// sandbox: codex's Seatbelt denies a same-sandbox process signalling its own
// session (`(allow signal (target same-sandbox))`), so the previous
// self-spawned delayed killer silently failed under codex. The driver is the
// harness's own parent process, outside any sandbox the harness runs under,
// so it can always signal its child.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

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
    /// Relaunch-signal file the loop driver watches for while its harness
    /// child runs. Defaults to `$GROVE_SIGNAL_FILE`.
    pub signal_file: Option<PathBuf>,
    /// What the loop should do once this session ends: relaunch the next
    /// task (default) or finish the whole grove (`--done`).
    pub disposition: Disposition,
}

/// Resolve options from an explicit flag, falling back to the loop driver's
/// environment handle. `disposition` comes straight from the verb (no env
/// fallback): the default verb relaunches, the `--done` flag finishes.
pub fn resolve_opts(signal_file: Option<PathBuf>, disposition: Disposition) -> CompleteOpts {
    CompleteOpts {
        signal_file: signal_file.or_else(|| {
            std::env::var_os("GROVE_SIGNAL_FILE")
                .filter(|value| !value.is_empty())
                .map(PathBuf::from)
        }),
        disposition,
    }
}

/// Write the disposition signal and return. Ending this session is the loop
/// driver's job now — it is watching for this very file — not this
/// process's, so there is nothing else to do here.
pub fn signal_complete(opts: &CompleteOpts) -> Result<()> {
    match &opts.signal_file {
        Some(sig) => {
            if let Some(parent) = sig.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("creating signal-file dir {}", parent.display()))?;
            }
            std::fs::write(sig, format!("{}\n", opts.disposition.token()))
                .with_context(|| format!("writing signal file {}", sig.display()))?;
            let tail = match opts.disposition {
                Disposition::Relaunch => "the loop will start the next task",
                Disposition::Done => "the grove is finished — the loop will stop",
            };
            eprintln!("grove complete: signalled; {tail}.");
        }
        None => {
            eprintln!(
                "grove complete: no GROVE_SIGNAL_FILE — not running under the loop driver; \
                 exit this session manually."
            );
        }
    }
    Ok(())
}
