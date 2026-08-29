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

use anyhow::Result;
use keyed_launch::Token;
use std::path::PathBuf;

/// What a finished session tells the self-driving loop to do next. The agent
/// picks this when it signals; the loop driver reads it back from the signal
/// file (self-driving-loop). The third case — *no* signal at all — is the
/// *absence* of a [`Disposition`], represented by [`interpret`] returning
/// `None`, so the loop can tell a clean finish from an abnormal exit. That case
/// is only ever **reached** when the session process itself ends (a crash, or a
/// human `/exit`/Ctrl-C): an agent that simply forgets to signal does not get
/// there at all — see [`interpret`].
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

/// Interpret the token a finished session left in its completion channel.
///
/// **This is the whole of grove's stake in the channel's content.** The runner
/// carries the token opaquely — allocating the path, watching for it, reading
/// the bytes back — and hands it here without ever having decided what it
/// means; the meaning is one match, in one place, on grove's side of the seam.
///
/// `None` = no token at all, and the driver only ever observes that when the
/// session *process* ended without signalling: a human `/exit`/Ctrl-C, or a
/// crash → the loop stops.
///
/// An agent that finishes its work and forgets the verb is **not** that case,
/// and reading it as one is what made this failure mode hard to see. The
/// configured templates launch *interactive* harnesses (no `-p`, no `exec`), so
/// finishing a turn returns the session to its prompt and it never exits: the
/// runner's supervision sits on a channel that will never appear and a child
/// that will never exit, and the loop **stalls** rather than stopping. Nothing
/// downstream of here can distinguish that from a session still working — which
/// is why the reminder to signal is delivered at the moment of decision, on
/// `leaf-retire`/`leaf-prune`'s stderr.
///
/// `Some(Done)` = a clean whole-grove finish;
/// `Some(Relaunch)` = relaunch the next task (also the backward-compatible
/// reading of any present-but-unrecognised content, e.g. a stale binary's
/// legacy `"complete"`).
pub fn interpret(token: Option<&Token>) -> Option<Disposition> {
    let token = token?;
    if token.as_str().trim() == Disposition::DONE_TOKEN {
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
/// driver's job now — it is watching for this very channel — not this
/// process's, so there is nothing else to do here.
///
/// Written through `keyed_launch::signal`, the child-side half of the channel
/// the driver allocated: this process holds only the path it was handed, and
/// the file's framing belongs to whoever reads it back.
pub fn signal_complete(opts: &CompleteOpts) -> Result<()> {
    match &opts.signal_file {
        Some(sig) => {
            keyed_launch::signal(sig, opts.disposition.token())?;
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
