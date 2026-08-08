// Tests for the `grove-llm complete` verb core (src/complete.rs): writing the
// loop-disposition signal file. The out-of-band kill itself is the loop
// driver's job now (src/loop_driver.rs, driver-side-kill-k2) — `complete`
// only ever writes a file; see tests/loop_driver.rs for the kill half.

use grove::complete::{self, CompleteOpts, Disposition};
use tempfile::TempDir;

#[test]
fn resolve_opts_passes_explicit_values_through() {
    let o = complete::resolve_opts(Some("/tmp/relaunch.signal".into()), Disposition::Relaunch);
    assert_eq!(
        o.signal_file.as_deref(),
        Some(std::path::Path::new("/tmp/relaunch.signal"))
    );
    assert_eq!(o.disposition, Disposition::Relaunch);
}

#[test]
fn resolve_opts_carries_the_done_disposition() {
    let o = complete::resolve_opts(None, Disposition::Done);
    assert_eq!(
        o.disposition,
        Disposition::Done,
        "`complete --done` must reach the signal as a finish disposition"
    );
}

#[test]
fn an_empty_signal_environment_is_no_loop_context() {
    assert_eq!(std::env::var_os("GROVE_SIGNAL_FILE"), Some("".into()));

    let opts = complete::resolve_opts(None, Disposition::Relaunch);

    assert!(
        opts.signal_file.is_none(),
        "the meta-grove's empty environment guard became a real signal path"
    );
}

#[test]
fn relaunch_signal_is_read_back_as_relaunch() {
    let tmp = TempDir::new().unwrap();
    let sig = tmp.path().join("loop.signal");
    let opts = CompleteOpts {
        signal_file: Some(sig.clone()),
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
        signal_file: Some(sig.clone()),
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
