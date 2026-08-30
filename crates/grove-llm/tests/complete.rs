// Tests for the `grove-llm complete` verb (`grove_loop::verbs::complete`):
// writing the loop-disposition token into the completion channel the driver
// allocated, and reading one back. The channel, the kill and the escalation are the runner's
// (`crates/keyed-launch`) — `complete` only ever writes a token and interprets
// one.
//
// The round trips below go through a real `Channel` rather than a path this
// file invents, because that is the only pairing that proves anything: the
// framing `signal` writes and the framing `read` strips are the runner's, and a
// test that wrote and parsed its own file would agree with itself while
// disagreeing with the driver.

use grove_loop::verbs::{self, Signalled};
use grove_loop::{interpret, Disposition};
use keyed_launch::Channel;
use tempfile::TempDir;

#[test]
fn an_explicit_channel_passes_through() {
    assert_eq!(
        verbs::signal_channel(Some(std::path::Path::new("/tmp/relaunch.signal"))).as_deref(),
        Some(std::path::Path::new("/tmp/relaunch.signal"))
    );
}

#[test]
fn an_empty_signal_environment_is_no_loop_context() {
    assert_eq!(std::env::var_os("GROVE_SIGNAL_FILE"), Some("".into()));

    assert!(
        verbs::signal_channel(None).is_none(),
        "the meta-grove's empty environment guard became a real signal path"
    );
}

#[test]
fn no_channel_at_all_is_answered_rather_than_refused() {
    assert_eq!(
        verbs::complete(None, false).unwrap(),
        Signalled::NoLoop,
        "outside a loop the verb is a no-op that says so"
    );
}

/// Signal into a freshly allocated channel and read the disposition back the
/// way the driver does.
fn round_trip(tmp: &TempDir, disposition: Disposition) -> (Channel, Option<Disposition>) {
    let channel = Channel::allocate(tmp.path()).unwrap();

    let signalled =
        verbs::complete(Some(channel.path()), disposition == Disposition::Done).unwrap();
    assert_eq!(signalled, Signalled::Wrote(channel.path().to_path_buf()));

    let read = interpret(channel.read().as_ref());
    (channel, read)
}

#[test]
fn relaunch_signal_is_read_back_as_relaunch() {
    let tmp = TempDir::new().unwrap();

    let (channel, read) = round_trip(&tmp, Disposition::Relaunch);

    assert!(
        channel.path().exists(),
        "completion must create the channel file"
    );
    assert_eq!(
        read,
        Some(Disposition::Relaunch),
        "the default completion must signal a relaunch"
    );
}

#[test]
fn done_signal_is_read_back_as_done() {
    let tmp = TempDir::new().unwrap();

    let (_channel, read) = round_trip(&tmp, Disposition::Done);

    assert_eq!(
        read,
        Some(Disposition::Done),
        "`complete --done` must signal a clean whole-grove finish"
    );
}

#[test]
fn an_absent_token_is_none() {
    let tmp = TempDir::new().unwrap();
    let channel = Channel::allocate(tmp.path()).unwrap();

    assert_eq!(
        interpret(channel.read().as_ref()),
        None,
        "no token → the loop stops (human exit / crash), it does not relaunch"
    );
}

#[test]
fn unrecognised_signal_content_is_treated_as_relaunch() {
    // Backward compatibility: a stale binary wrote the legacy "complete" token.
    // Anything present-but-not-"done" must still relaunch (the safe default),
    // never be mistaken for a clean finish.
    let tmp = TempDir::new().unwrap();
    let channel = Channel::allocate(tmp.path()).unwrap();
    std::fs::write(channel.path(), "complete\n").unwrap();

    assert_eq!(
        interpret(channel.read().as_ref()),
        Some(Disposition::Relaunch)
    );
}
