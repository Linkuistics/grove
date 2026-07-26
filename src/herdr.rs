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

/// Enact a [`Plan`]. Report before release — see [`plan_for`].
pub fn apply(plan: Plan) {
    if let Some(state) = plan.report {
        report(state);
    }
    if plan.release {
        release();
    }
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
        escape(pane_id)
    );
    if let Some(state) = state {
        params.push_str(&format!(r#","state":"{}""#, state.token()));
    }
    params.push('}');
    // The `id` is required by herdr's envelope but grove never reads the
    // response, so it only has to be present and stable-ish.
    format!(r#"{{"id":"grove:{method}","method":"{method}","params":{params}}}"#)
}

fn escape(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for c in raw.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
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
