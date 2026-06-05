//! Real-child half of the 050 assessment: spawn a deterministic command through
//! a real pty, drain its output via the reader-thread → mpsc pump, feed it into
//! the emulator, and assert the grid holds the expected text and the child exits.

use std::thread::sleep;
use std::time::{Duration, Instant};

use harness_pane::{PtySession, TerminalEmulator};

/// Pump the session into the emulator until `predicate` holds or `timeout`
/// elapses. Mirrors the host's per-tick `drain()` → `process()` step.
fn pump_until(
    pty: &PtySession,
    emu: &mut TerminalEmulator,
    timeout: Duration,
    mut predicate: impl FnMut(&TerminalEmulator) -> bool,
) -> bool {
    let deadline = Instant::now() + timeout;
    loop {
        emu.process(&pty.drain());
        if predicate(emu) {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        sleep(Duration::from_millis(20));
    }
}

#[test]
fn spawns_a_child_renders_its_output_and_it_exits() {
    let mut emu = TerminalEmulator::new(24, 80);
    let argv = vec![
        "sh".to_string(),
        "-c".to_string(),
        "printf 'HELLO-FROM-CHILD'".to_string(),
    ];
    let mut pty = PtySession::spawn(&argv, None, &[], 24, 80).expect("spawn child");

    let saw_text = pump_until(&pty, &mut emu, Duration::from_secs(5), |emu| {
        emu.screen().contents().contains("HELLO-FROM-CHILD")
    });
    assert!(saw_text, "expected child output on the grid; got:\n{}", emu.screen().contents());

    // The child should exit on its own; poll try_wait until it reports status.
    let deadline = Instant::now() + Duration::from_secs(5);
    let status = loop {
        if let Some(status) = pty.try_wait().expect("try_wait") {
            break status;
        }
        assert!(Instant::now() < deadline, "child did not exit in time");
        sleep(Duration::from_millis(20));
    };
    assert!(status.success(), "child exited unsuccessfully: {status:?}");
}

#[test]
fn write_input_reaches_the_child() {
    // `cat` echoes stdin back through the pty; feed it a line and read it back.
    let mut emu = TerminalEmulator::new(24, 80);
    let argv = vec!["cat".to_string()];
    let mut pty = PtySession::spawn(&argv, None, &[], 24, 80).expect("spawn cat");

    pty.write_input(b"ping\r").expect("write_input");

    let echoed = pump_until(&pty, &mut emu, Duration::from_secs(5), |emu| {
        emu.screen().contents().contains("ping")
    });
    assert!(echoed, "cat did not echo input; got:\n{}", emu.screen().contents());

    // Dropping the session kills `cat` and joins the reader thread (Drop impl).
    drop(pty);
}
