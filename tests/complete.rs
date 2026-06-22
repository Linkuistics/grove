// Tests for the `grove-llm complete` verb core (src/complete.rs): the in-loop
// completion signal — write the loop-disposition flag + fork the detached
// delayed killer (030 D4 option (b)). The disposition the agent chose
// (relaunch the next task, or finish the whole grove) is encoded in the signal
// file and read back by the loop driver.

use grove::complete::{self, CompleteOpts, Disposition};
use std::process::Command;
use std::time::{Duration, Instant};
use tempfile::TempDir;

#[test]
fn resolve_opts_passes_explicit_values_through() {
    let o = complete::resolve_opts(
        Some(42),
        Some("/tmp/relaunch.signal".into()),
        Some(0.5),
        Some(0.7),
        Disposition::Relaunch,
    );
    assert_eq!(o.pid, Some(42));
    assert_eq!(
        o.signal_file.as_deref(),
        Some(std::path::Path::new("/tmp/relaunch.signal"))
    );
    assert_eq!(o.grace, 0.5);
    assert_eq!(o.kill_grace, 0.7);
    assert_eq!(o.disposition, Disposition::Relaunch);
}

#[test]
fn resolve_opts_carries_the_done_disposition() {
    let o = complete::resolve_opts(None, None, None, None, Disposition::Done);
    assert_eq!(
        o.disposition,
        Disposition::Done,
        "`complete --done` must reach the signal as a finish disposition"
    );
}

#[test]
fn relaunch_signal_is_read_back_as_relaunch() {
    let tmp = TempDir::new().unwrap();
    let sig = tmp.path().join("loop.signal");
    // No pid → no kill; we only assert the signal the loop driver reads back.
    let opts = CompleteOpts {
        pid: None,
        signal_file: Some(sig.clone()),
        grace: 0.1,
        kill_grace: 0.1,
        disposition: Disposition::Relaunch,
    };

    complete::signal_complete(&opts).unwrap();

    assert!(sig.exists(), "completion must create the signal file");
    assert_eq!(
        complete::read_signal(&sig),
        Some(Disposition::Relaunch),
        "the default completion must signal a relaunch"
    );
}

#[test]
fn done_signal_is_read_back_as_done() {
    let tmp = TempDir::new().unwrap();
    let sig = tmp.path().join("loop.signal");
    let opts = CompleteOpts {
        pid: None,
        signal_file: Some(sig.clone()),
        grace: 0.1,
        kill_grace: 0.1,
        disposition: Disposition::Done,
    };

    complete::signal_complete(&opts).unwrap();

    assert_eq!(
        complete::read_signal(&sig),
        Some(Disposition::Done),
        "`complete --done` must signal a clean whole-grove finish"
    );
}

#[test]
fn read_signal_is_none_when_no_session_signalled() {
    let tmp = TempDir::new().unwrap();
    let missing = tmp.path().join("never-written.signal");
    assert_eq!(
        complete::read_signal(&missing),
        None,
        "no signal file → the loop stops (human exit / crash), it does not relaunch"
    );
}

#[test]
fn unrecognised_signal_content_is_treated_as_relaunch() {
    // Backward compatibility: a stale binary wrote the legacy "complete" token.
    // Anything present-but-not-"done" must still relaunch (the safe default),
    // never be mistaken for a clean finish.
    let tmp = TempDir::new().unwrap();
    let sig = tmp.path().join("loop.signal");
    std::fs::write(&sig, "complete\n").unwrap();
    assert_eq!(complete::read_signal(&sig), Some(Disposition::Relaunch));
}

#[test]
fn signal_complete_kills_the_target_pid_out_of_band() {
    // Stand-in for the `claude` session: a process that would otherwise run for
    // 30s. The detached killer must terminate it within the grace window. The
    // out-of-band kill is identical for both dispositions — only the loop's
    // relaunch-vs-finish decision differs.
    let mut child = Command::new("sleep").arg("30").spawn().unwrap();
    let pid = child.id() as i32;

    let tmp = TempDir::new().unwrap();
    let opts = CompleteOpts {
        pid: Some(pid),
        signal_file: Some(tmp.path().join("loop.signal")),
        grace: 0.3,
        kill_grace: 0.5,
        disposition: Disposition::Relaunch,
    };

    // Must return immediately — the kill happens out-of-band, after a grace.
    complete::signal_complete(&opts).unwrap();

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut died = false;
    while Instant::now() < deadline {
        if child.try_wait().unwrap().is_some() {
            died = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    if !died {
        let _ = child.kill();
    }
    let _ = child.wait();
    assert!(died, "the target pid must be killed by the detached killer");
}
