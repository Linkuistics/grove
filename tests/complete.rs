// Tests for the `grove-llm complete` verb core (src/complete.rs): the in-loop
// completion signal — write the relaunch flag + fork the detached delayed
// killer (030 D4 option (b)).

use grove::complete::{self, CompleteOpts};
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
    );
    assert_eq!(o.pid, Some(42));
    assert_eq!(
        o.signal_file.as_deref(),
        Some(std::path::Path::new("/tmp/relaunch.signal"))
    );
    assert_eq!(o.grace, 0.5);
    assert_eq!(o.kill_grace, 0.7);
}

#[test]
fn signal_complete_writes_the_relaunch_signal_file() {
    let tmp = TempDir::new().unwrap();
    let sig = tmp.path().join("relaunch.signal");
    // No pid → no kill; we only assert the signal file is created so the loop
    // driver knows to relaunch.
    let opts = CompleteOpts {
        pid: None,
        signal_file: Some(sig.clone()),
        grace: 0.1,
        kill_grace: 0.1,
    };

    complete::signal_complete(&opts).unwrap();

    assert!(
        sig.exists(),
        "completion must create the signal file so the loop relaunches"
    );
}

#[test]
fn signal_complete_kills_the_target_pid_out_of_band() {
    // Stand-in for the `claude` session: a process that would otherwise run for
    // 30s. The detached killer must terminate it within the grace window.
    let mut child = Command::new("sleep").arg("30").spawn().unwrap();
    let pid = child.id() as i32;

    let tmp = TempDir::new().unwrap();
    let opts = CompleteOpts {
        pid: Some(pid),
        signal_file: Some(tmp.path().join("relaunch.signal")),
        grace: 0.3,
        kill_grace: 0.5,
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
