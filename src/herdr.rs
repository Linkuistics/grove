// Optional pane-state reporting to herdr (herdr-optional-ui).
//
// The one thing `.grove/` cannot record is *semantic state at a moment in
// time* — whether the loop is working, or has stopped and is waiting for a
// human. That is knowledge only the running driver holds, so it is the only
// thing grove pushes to herdr; everything richer (the tree, the live leaf) is
// the plugin's job, read straight off disk. Keeping the boundary exactly there
// is what stops the integration becoming a dependency.
//
// **Never load-bearing.** Every entry point here is a no-op when the pane
// environment is absent, when the socket refuses, when it wedges, or when the
// herdr on the other end is stock and silently drops the report. Nothing in
// this module can fail a launch or stall the loop; nothing returns a `Result`,
// because there is no caller-visible failure to handle.
//
// ## Transport
//
// Newline-delimited JSON over the unix socket at `$HERDR_SOCKET_PATH`, the
// same protocol herdr's own integrations speak (`src/integration/assets/kilo/
// herdr-agent-state.js` is the readable reference):
//
//     {"id":"…","method":"pane.report_agent","params":{"pane_id":"wQ:p1",
//      "source":"grove","agent":"grove","state":"working"}}\n
//
// Spoken directly rather than shelled out through `herdr pane report-agent`,
// for three reasons: it depends only on the variables herdr itself put in the
// pane environment (never on `herdr` being on `PATH`), the attempt is bounded
// by socket options rather than a spawn-poll-kill dance around a subprocess,
// and a test can bind a real `UnixListener` and assert the exact bytes.
//
// ## What grove sends, and what it deliberately does not
//
// - **No `session_ref`/`agent_session_id`, ever.** The authority patch grove
//   carries in its herdr fork keys entirely off `session_ref.is_some()`: a
//   report that makes no session-identity claim neither conflicts with, nor
//   clears, the identity owner. Grow a session ref here and every report
//   starts conflicting with the harness's own herdr integration again and is
//   dropped — and the harness's session-resume gets destroyed on the way past.
// - **No `seq`.** `accept_hook_report` takes a `seq: None` report only while no
//   seq was ever recorded for that source, so it is all-or-nothing per source.
//   A per-process counter is the wrong shape for a restartable driver: it would
//   restart at 1 after a Ctrl-C + `grove do` resume and be rejected as stale.
// - **`source` and `agent` are both the constant `grove`.** Stable because
//   release matches on the `(source, agent)` pair — a source that varied
//   between report and release would leave authority behind. `agent` is
//   honest (a `grove do` pane is a loop over a *sequence* of sessions, and the
//   harness may vary per leaf) and, as a label that parses to no *known* herdr
//   agent, it also bypasses the gate that drops a report naming a different
//   known agent than the one herdr detected.

use crate::complete::Disposition;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::sync::mpsc;
use std::time::Duration;

/// The `(source, agent)` pair grove reports under. Stable by contract: release
/// matches on the pair, so a drift between report and release would orphan
/// grove's authority on the pane.
const SOURCE: &str = "grove";
const AGENT: &str = "grove";

/// Hard bound on one report, covering *all* of connect, write and read. Chosen
/// to match what herdr's own integrations allow themselves (kilo uses a 500ms
/// socket timeout): long enough that a healthy socket always completes, short
/// enough to be invisible at a session boundary.
const BUDGET: Duration = Duration::from_millis(500);

/// The semantic states grove reports. Deliberately not herdr's full vocabulary:
/// `unknown` is what herdr already infers on its own, and **`done` is not a
/// reportable state at all** — herdr derives it as `idle && !seen`, which is
/// exactly why the pre-patch world showed a grove stalled on a question as
/// "done": with nothing reporting `blocked`, "finished" and "waiting on you"
/// both landed on `idle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// A session is running.
    Working,
    /// The loop has stopped and cannot continue without a human.
    Blocked,
    /// The grove is finished.
    Idle,
}

impl State {
    /// herdr's `PaneAgentState` is serialised snake_case.
    fn token(self) -> &'static str {
        match self {
            State::Working => "working",
            State::Blocked => "blocked",
            State::Idle => "idle",
        }
    }
}

/// Why the loop stopped — every terminal path the driver can reach, named so
/// the state policy below is one exhaustive table rather than a report call
/// scattered per branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stop {
    /// A session ended; the payload is what it left in the signal file
    /// (`None` = no signal at all: `/exit`, Ctrl-C, or a crash).
    Signal(Option<Disposition>),
    /// The version-skew guard declined to start the next session.
    VersionSkew,
    /// The driver caught SIGTERM/SIGHUP.
    Interrupted,
    /// The loop itself errored out.
    Failed,
}

/// What a [`Stop`] means for the pane: a state to report, and whether to hand
/// the pane back to herdr's own screen detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plan {
    pub report: Option<State>,
    pub release: bool,
}

/// The whole state policy, in one table.
///
/// Two asymmetries in here are deliberate and are the substance of this module;
/// both are cases where "release on every catchable exit" is the wrong rule.
///
/// **A no-signal stop reports `blocked` and keeps authority.** Releasing there
/// would hand the pane straight back to screen detection, which reads a parked
/// grove as `idle` — and therefore as herdr's derived `done`. That is the
/// headline complaint this whole grove exists to fix, so releasing would undo
/// the fix a few milliseconds after making it. `blocked` is not a stale
/// leftover: the grove genuinely has live leaves and genuinely needs a human to
/// re-run `grove do`, and it self-heals, because the next session reports
/// `working` over the top of it (later reports land now that the owner gate no
/// longer vetoes them).
///
/// **An interrupt releases and reports nothing.** SIGTERM/SIGHUP means grove is
/// being torn down by something outside it and no longer knows anything about
/// the pane, so the honest move is to say nothing and hand it back — the
/// opposite of the no-signal case, where grove knows exactly one useful thing.
pub fn plan_for(stop: Stop) -> Plan {
    match stop {
        // The next launch reports `working` again a moment later; re-reporting
        // it here would be one redundant round trip per task.
        Stop::Signal(Some(Disposition::Relaunch)) => Plan {
            report: None,
            release: false,
        },
        // The finish cycle ran. Report `idle` *then* release, in that order: if
        // the release is the call that fails, the pane is left reading
        // idle/done rather than pinned at a stale `working`. Releasing at all
        // is what stops grove's label shadowing every future agent in this pane.
        Stop::Signal(Some(Disposition::Done)) => Plan {
            report: Some(State::Idle),
            release: true,
        },
        // `/exit`, Ctrl-C, or a crash. See the note above on why this holds
        // authority instead of releasing it.
        Stop::Signal(None) => Plan {
            report: Some(State::Blocked),
            release: false,
        },
        // Both need a human as squarely as a no-signal stop does: the loop is
        // parked with live leaves left.
        Stop::VersionSkew | Stop::Failed => Plan {
            report: Some(State::Blocked),
            release: false,
        },
        Stop::Interrupted => Plan {
            report: None,
            release: true,
        },
    }
}

/// An **intra-session moment**, reported by the harness itself through a hook
/// grove injects at launch — the half of the pane's state the loop driver
/// structurally cannot see.
///
/// The driver is the harness's *parent*, so its whole vocabulary is "a session
/// started" / "a session ended". A session that stalls **mid-session** on a
/// question — the overnight case this whole grove exists to fix — never reaches
/// either, and reads `working` forever. Only the harness knows a turn ended.
///
/// Two of the four are turn *boundaries*; the other two are the pair that closes
/// the gap **inside** one turn, where a permission prompt stalls an unattended
/// loop exactly as badly as a question does. A mid-turn stall needs a *paired*
/// restore, which is why it is a pair and not one more boundary: nothing else
/// would ever take the `blocked` back down before the turn ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    /// The human submitted a prompt: whatever the pane said, the agent is going
    /// again.
    Started,
    /// The agent stopped responding. Whether that means "asking" or "finished
    /// the task" is not something the harness knows — see [`plan_for_turn`].
    Ended,
    /// A dialog is on screen waiting for a human, **mid-turn** — a permission
    /// prompt, or an MCP elicitation. claude only raises these once the human
    /// has been silent for six seconds, so this already means *unattended*, not
    /// merely *asked*.
    Waiting,
    /// A tool call finished, so the agent is demonstrably running again. The
    /// restore half of [`Turn::Waiting`], and the only event claude fires
    /// between a granted permission and the end of the turn.
    ToolUsed,
}

/// The turn table — the state policy for a boundary only the harness can see,
/// refining (never replacing) [`plan_for`]'s session-level one.
///
/// **The discriminator is the signal file, and nothing else.** A grove turn ends
/// exactly two ways: the model ran `grove-llm complete` — in which case the
/// disposition is sitting in `$GROVE_SIGNAL_FILE` and the driver is already
/// winding this session down — or it did not, in which case the model stopped
/// mid-task and a human is needed. That needs **no new model contract**:
/// `complete` is already mandatory as the last action of every task, so the
/// disposition is already being deposited. A "declare your intent" verb would
/// only be a second thing to forget.
///
/// This is also precisely why herdr cannot fix this upstream and grove can:
/// herdr sees a turn end and genuinely cannot tell "done" from "asking". grove
/// knows, because grove is the thing that relaunches.
///
/// Three entries are worth their own sentence:
///
/// **A signalled turn end reports `working`, not nothing.** The driver's
/// equivalent row stays silent because the next launch re-reports `working` a
/// moment later; here it is not redundant, because a *earlier* turn in this same
/// session may have left the pane `blocked` (the agent asked, the human
/// answered, the task then finished). Re-reporting is one round trip per task
/// and heals that.
///
/// **A `done` signal silences every row that fires without a human.** The driver
/// is about to report `idle` and release; any report in that window is one in
/// the wrong direction at a grove that has just finished, for the two seconds
/// until the completion kill lands. [`Turn::Started`] is the exception, because
/// a prompt submitted after `complete --done` is a human actually typing.
///
/// **A mid-turn stall does not consult the *session*'s state, only the signal.**
/// [`Turn::Waiting`] reports `blocked` whatever else is true: a dialog waiting
/// on a human is not a thing grove has an opinion to weigh, and unlike a turn
/// end there is nothing ambiguous to discriminate.
pub fn plan_for_turn(turn: Turn, signal: Option<Disposition>) -> Plan {
    let report = match (turn, signal) {
        // A prompt was submitted: the human is done and the agent is running.
        (Turn::Started, _) => Some(State::Working),
        // The grove finished: leave the last word to the driver's idle+release.
        (Turn::Ended | Turn::Waiting | Turn::ToolUsed, Some(Disposition::Done)) => None,
        // The headline case. No signal at a turn end means the model stopped
        // without completing the task — it asked something, or gave up — and
        // the loop cannot proceed until a human answers.
        (Turn::Ended, None) => Some(State::Blocked),
        (Turn::Ended, Some(Disposition::Relaunch)) => Some(State::Working),
        // The mid-turn pair (herdr-mid-turn-blockers): a dialog nobody has
        // touched for six seconds, and the tool call that proves the agent got
        // past it.
        (Turn::Waiting, _) => Some(State::Blocked),
        (Turn::ToolUsed, _) => Some(State::Working),
    };
    // Never, on any row: release is the *driver's* call (herdr-optional-ui's
    // table). A session that released mid-loop would hand the pane back to
    // screen detection while the loop was still running.
    Plan {
        report,
        release: false,
    }
}

/// Enact a [`Plan`]. Report before release — see [`plan_for`].
pub fn apply(plan: Plan) {
    if let Some(state) = plan.report {
        report(state);
    }
    if plan.release {
        release();
    }
}

/// Report a turn boundary from **inside** a harness session — the body of
/// `grove-llm report-turn`, run by the hooks grove injects at launch
/// (`launch::append_turn_hooks`).
///
/// Reads the one fact the policy turns on out of the environment the driver
/// exported: `$GROVE_SIGNAL_FILE`. Its **absence** and an absent *file at* that
/// path are deliberately different things. No variable at all means this session
/// is not a loop task — a `grove retire` launch, or a hand-run harness that
/// somehow carries the hooks — and grove has no opinion about that pane; herdr
/// never expires an authority, so reporting one would pin a pane nobody is
/// driving. A variable with no file behind it is the headline case: the driver
/// clears the signal at the top of every iteration, so within a session the file
/// exists only if *this* session's agent wrote it.
pub fn report_turn(turn: Turn) {
    let Some(signal_file) = std::env::var_os("GROVE_SIGNAL_FILE") else {
        return;
    };
    let signal = crate::complete::read_signal(std::path::Path::new(&signal_file));
    apply(plan_for_turn(turn, signal));
}

/// Report one state. The launch site's `working` is the only report that is not
/// a [`Stop`], so it calls this directly.
pub fn report(state: State) {
    send("pane.report_agent", Some(state));
}

/// Give the pane back to herdr's screen detection. Necessary because herdr
/// never expires a hook authority: there is no TTL, and its
/// clear-on-process-exit path only fires for a label that parses to a *known*
/// agent, which `grove` deliberately does not. So grove's last report is what
/// the pane shows until something releases it.
pub fn release() {
    send("pane.release_agent", None);
}

/// The pane environment herdr places in every pane it spawns, or `None` when
/// grove is not running under herdr — the gate that makes this whole module
/// vanish. All three variables are checked, matching every herdr integration
/// asset; `HERDR_ENV` alone is what herdr's own code treats as "inside herdr".
/// Whether this process is running inside a herdr pane at all — the same gate
/// [`pane`] applies, exposed for the *launch* site, which must decide whether to
/// inject the turn-boundary hooks in the first place.
///
/// Gating the injection (and not merely the reporting) is what makes
/// "absent herdr, the hooks are inert" true in the strongest sense: outside a
/// herdr pane the launch argv is byte-identical to what it was before turn
/// hooks existed, so there is no hook to fire, nothing to spawn, and no new
/// surface to go wrong. Safe to read once at launch: the pane environment comes
/// from the pane grove is already running in and cannot appear mid-loop.
pub fn in_pane() -> bool {
    pane().is_some()
}

fn pane() -> Option<(String, String)> {
    if std::env::var("HERDR_ENV").ok()? != "1" {
        return None;
    }
    let socket = std::env::var("HERDR_SOCKET_PATH")
        .ok()
        .filter(|s| !s.is_empty())?;
    let pane_id = std::env::var("HERDR_PANE_ID")
        .ok()
        .filter(|s| !s.is_empty())?;
    Some((socket, pane_id))
}

fn send(method: &str, state: Option<State>) {
    let Some((socket, pane_id)) = pane() else {
        return;
    };
    dispatch(socket, request_line(method, &pane_id, state));
}

/// Build one request line. Split out as a pure function so the wire shape is
/// asserted in a test rather than trusted.
fn request_line(method: &str, pane_id: &str, state: Option<State>) -> String {
    // grove carries no JSON serialiser, and does not need one for four string
    // fields with a fixed vocabulary. Only `pane_id` comes from outside grove
    // (it is herdr's own opaque handle, e.g. `wQ:p1`), so it is the only field
    // escaped — but it *is* escaped, rather than assumed well-formed.
    let mut params = format!(
        r#"{{"pane_id":"{}","source":"{SOURCE}","agent":"{AGENT}""#,
        crate::json::escape(pane_id)
    );
    if let Some(state) = state {
        params.push_str(&format!(r#","state":"{}""#, state.token()));
    }
    params.push('}');
    // The `id` is required by herdr's envelope but grove never reads the
    // response, so it only has to be present and stable-ish.
    format!(r#"{{"id":"grove:{method}","method":"{method}","params":{params}}}"#)
}

/// Send one line with a hard wall-clock bound, whatever the socket does.
///
/// The write happens on a throwaway thread that this function waits on for at
/// most [`BUDGET`], rather than inline. Socket read/write timeouts alone are
/// not enough: `UnixStream::connect` has no timeout in `std`, and a listener
/// whose backlog is full blocks the connecting side — so an inline attempt has
/// one unbounded step in it, and this is the driver, which must never stall.
/// Waiting via a channel gives the bound without a non-blocking-connect dance;
/// in the pathological case the thread is abandoned still blocked in `connect`,
/// which costs one thread in a process that is about to continue or exit.
///
/// The wait is what makes release-on-exit real: a fire-and-forget thread would
/// let the driver exit before the release ever reached the socket.
fn dispatch(socket: String, line: String) {
    let (tx, rx) = mpsc::channel();
    // `Builder::spawn` rather than `thread::spawn`: at OS thread exhaustion the
    // latter panics, and nothing in this module may take the driver down.
    let spawned = std::thread::Builder::new()
        .name("grove-herdr-report".into())
        .spawn(move || {
            let _ = tx.send(write_line(&socket, &line));
        });
    if spawned.is_ok() {
        let _ = rx.recv_timeout(BUDGET);
    }
}

/// One connect-write-read cycle. The response is read but discarded: herdr
/// answers `{"result":{"type":"ok"}}` whether it accepted the report or
/// silently dropped it (a stock, unpatched herdr always drops it), so the reply
/// carries nothing grove could act on. It is read only so the server is never
/// left writing into a socket that closed under it.
fn write_line(socket: &str, line: &str) -> std::io::Result<()> {
    let stream = UnixStream::connect(socket)?;
    stream.set_write_timeout(Some(BUDGET))?;
    stream.set_read_timeout(Some(BUDGET))?;
    let mut stream = stream;
    stream.write_all(line.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    let mut discard = String::new();
    let _ = BufReader::new(&stream).read_line(&mut discard);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;
    use std::os::unix::net::UnixListener;

    // The state policy is the substance of this module, so it is pinned as a
    // whole table — including the two entries that are *not* "release on every
    // catchable exit", which is the rule this table deliberately departs from.
    #[test]
    fn every_stop_maps_to_its_intended_pane_state() {
        assert_eq!(
            plan_for(Stop::Signal(Some(Disposition::Relaunch))),
            Plan {
                report: None,
                release: false
            },
            "a relaunch says nothing — the next launch re-reports working"
        );
        assert_eq!(
            plan_for(Stop::Signal(Some(Disposition::Done))),
            Plan {
                report: Some(State::Idle),
                release: true
            },
            "a finished grove reports idle, then hands the pane back"
        );
        for stop in [Stop::Signal(None), Stop::VersionSkew, Stop::Failed] {
            assert_eq!(
                plan_for(stop),
                Plan {
                    report: Some(State::Blocked),
                    release: false
                },
                "{stop:?} parks the loop with live leaves left: blocked, and \
                 authority held so screen detection cannot report it as done"
            );
        }
        assert_eq!(
            plan_for(Stop::Interrupted),
            Plan {
                report: None,
                release: true
            },
            "a torn-down driver knows nothing about the pane: say nothing, release"
        );
    }

    // The turn table, pinned whole beside the stop table it refines. The
    // discriminator is the signal file and nothing else: a turn that ended
    // *without* one is a model that stopped mid-task and needs a human.
    #[test]
    fn every_turn_boundary_maps_to_its_intended_pane_state() {
        assert_eq!(
            plan_for_turn(Turn::Started, None),
            Plan {
                report: Some(State::Working),
                release: false
            },
            "a human answered: the agent is going again, whatever the pane said"
        );
        assert_eq!(
            plan_for_turn(Turn::Ended, None),
            Plan {
                report: Some(State::Blocked),
                release: false
            },
            "no signal at a turn end = stopped mid-task with a question: blocked"
        );
        assert_eq!(
            plan_for_turn(Turn::Ended, Some(Disposition::Relaunch)),
            Plan {
                report: Some(State::Working),
                release: false
            },
            "the task signalled: the driver is about to relaunch, so the pane is \
             busy — and this heals any `blocked` an earlier turn end left behind"
        );
        assert_eq!(
            plan_for_turn(Turn::Ended, Some(Disposition::Done)),
            Plan {
                report: None,
                release: false
            },
            "the grove finished: say nothing and let the driver's idle-then-release \
             be the last word, rather than flapping `working` at a finished grove"
        );
    }

    // The mid-turn pair (herdr-mid-turn-blockers). A permission prompt stalls an
    // unattended loop exactly as badly as a question does, and grove's own
    // authority took away the screen detection that used to catch it by
    // accident — so this closes a gap grove itself opened.
    #[test]
    fn a_mid_turn_dialog_blocks_and_the_next_tool_call_takes_it_back_down() {
        for signal in [None, Some(Disposition::Relaunch)] {
            assert_eq!(
                plan_for_turn(Turn::Waiting, signal).report,
                Some(State::Blocked),
                "{signal:?}: a dialog claude has been waiting six seconds on is a \
                 human-needed stall, whatever else the session is doing"
            );
            assert_eq!(
                plan_for_turn(Turn::ToolUsed, signal).report,
                Some(State::Working),
                "{signal:?}: a finished tool call is proof the agent got past the \
                 dialog — without this the pane stays blocked until the turn ends"
            );
        }
    }

    // The restore is the whole reason this is a *pair* rather than one more
    // boundary: `Waiting` alone would pin a pane at `blocked` from the first
    // permission prompt of a session until its turn finally ended.
    #[test]
    fn the_mid_turn_pair_is_symmetric() {
        assert_eq!(
            plan_for_turn(Turn::ToolUsed, None).report,
            Some(State::Working),
            "the restore must undo the block, or the block is worse than no report"
        );
        assert_ne!(
            plan_for_turn(Turn::Waiting, None).report,
            plan_for_turn(Turn::ToolUsed, None).report,
            "a pair that reported the same thing would be one event, not two"
        );
    }

    // A grove that has finished belongs to the driver's idle-then-release, and
    // that holds for every row a *machine* can fire — not just the turn end the
    // rule was first written for. `Started` is deliberately not in here: a
    // prompt submitted after `complete --done` is a human actually typing.
    #[test]
    fn a_done_signal_silences_every_report_no_human_asked_for() {
        for turn in [Turn::Ended, Turn::Waiting, Turn::ToolUsed] {
            assert_eq!(
                plan_for_turn(turn, Some(Disposition::Done)).report,
                None,
                "{turn:?}: the driver is two seconds from reporting idle and \
                 releasing; anything here is a report in the wrong direction"
            );
        }
    }

    // The whole point of the discriminator, guarded on its own: every *other*
    // turn end in a grove session is a completed task, and a table that reported
    // `blocked` for those would flap the pane at the end of every single task —
    // which is indistinguishable from the stall it exists to surface.
    #[test]
    fn a_signalled_turn_end_never_reports_blocked() {
        for signal in [Disposition::Relaunch, Disposition::Done] {
            assert_ne!(
                plan_for_turn(Turn::Ended, Some(signal)).report,
                Some(State::Blocked),
                "{signal:?} means the task finished on purpose, not that a human is needed"
            );
        }
    }

    // A turn hook fires inside the harness session, which never releases:
    // release is the driver's call alone (herdr-optional-ui's table), and a
    // session that released mid-loop would hand the pane to screen detection
    // while the loop was still running.
    #[test]
    fn no_turn_boundary_ever_releases_the_pane() {
        for turn in [Turn::Started, Turn::Ended, Turn::Waiting, Turn::ToolUsed] {
            for signal in [None, Some(Disposition::Relaunch), Some(Disposition::Done)] {
                assert!(
                    !plan_for_turn(turn, signal).release,
                    "{turn:?}/{signal:?}: release belongs to the driver, not a turn hook"
                );
            }
        }
    }

    // A no-signal stop is the one case where releasing would undo this leaf's
    // whole point, so it gets its own named guard: a future edit that
    // "tidies" the table into releasing everywhere fails here with the reason.
    #[test]
    fn a_parked_loop_never_releases_its_blocked_report() {
        let plan = plan_for(Stop::Signal(None));
        assert!(
            !plan.release,
            "releasing here returns the pane to screen detection, which reads a \
             parked grove as idle — i.e. herdr's derived `done`, the exact bug \
             this reporter exists to fix"
        );
    }

    #[test]
    fn a_report_carries_state_and_a_release_does_not() {
        assert_eq!(
            request_line("pane.report_agent", "wQ:p1", Some(State::Working)),
            r#"{"id":"grove:pane.report_agent","method":"pane.report_agent","params":{"pane_id":"wQ:p1","source":"grove","agent":"grove","state":"working"}}"#
        );
        assert_eq!(
            request_line("pane.release_agent", "wQ:p1", None),
            r#"{"id":"grove:pane.release_agent","method":"pane.release_agent","params":{"pane_id":"wQ:p1","source":"grove","agent":"grove"}}"#
        );
    }

    // Neither field may ever appear: `agent_session_id` would make the report
    // conflict with the harness's session owner again (the patch keys off its
    // absence), and a `seq` would be rejected as stale after any driver
    // restart. Both are absences, so they are asserted as absences.
    #[test]
    fn a_report_claims_no_session_and_carries_no_seq() {
        let line = request_line("pane.report_agent", "wQ:p1", Some(State::Blocked));
        assert!(!line.contains("session"), "{line}");
        assert!(!line.contains("seq"), "{line}");
    }

    #[test]
    fn a_pane_id_is_escaped_rather_than_trusted() {
        let line = request_line("pane.report_agent", "a\"b\\c", Some(State::Idle));
        assert!(line.contains(r#""pane_id":"a\"b\\c""#), "{line}");
    }

    // The transport, against a real listener — no herdr, no `herdr` binary, no
    // environment. Proves the bytes arrive newline-framed and that the driver
    // is not left waiting on a server that never answers.
    #[test]
    fn the_exact_line_reaches_a_listening_socket() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("herdr.sock");
        let listener = UnixListener::bind(&path).unwrap();
        let accepted = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut line = String::new();
            BufReader::new(stream).read_line(&mut line).unwrap();
            line
        });

        let line = request_line("pane.report_agent", "wQ:p1", Some(State::Working));
        dispatch(path.to_string_lossy().into_owned(), line.clone());

        assert_eq!(
            accepted.join().unwrap(),
            format!("{line}\n"),
            "herdr frames requests as one JSON object per line"
        );
    }

    // herdr-optional-ui, the load-bearing negative: a socket that is not there
    // must cost the driver nothing. Bounded well under BUDGET because
    // connecting to a missing path fails immediately (ECONNREFUSED/ENOENT)
    // rather than waiting out the timeout.
    #[test]
    fn a_missing_socket_is_a_silent_fast_no_op() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("not-a-socket");
        let started = std::time::Instant::now();
        dispatch(
            path.to_string_lossy().into_owned(),
            request_line("pane.report_agent", "wQ:p1", Some(State::Working)),
        );
        assert!(
            started.elapsed() < BUDGET,
            "a refused socket must not spend the budget, let alone block"
        );
    }

    // A server that accepts and then says nothing is the wedge that matters:
    // the driver must walk away on BUDGET rather than wait for a reply that is
    // never coming.
    #[test]
    fn a_silent_server_costs_the_driver_only_the_budget() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("herdr.sock");
        let listener = UnixListener::bind(&path).unwrap();
        let held = std::thread::spawn(move || {
            let _stream = listener.accept().unwrap();
            std::thread::sleep(Duration::from_secs(30));
        });

        let started = std::time::Instant::now();
        dispatch(
            path.to_string_lossy().into_owned(),
            request_line("pane.report_agent", "wQ:p1", Some(State::Working)),
        );
        let elapsed = started.elapsed();

        assert!(
            elapsed < BUDGET * 4,
            "a wedged socket must be abandoned near BUDGET, not waited out (took {elapsed:?})"
        );
        drop(held);
    }
}
