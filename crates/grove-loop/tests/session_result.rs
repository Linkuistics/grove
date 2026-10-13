//! The shared caller result contract, independently of either launch consumer.
use std::io::{Seek, Write};
use std::os::unix::fs::MetadataExt;

use grove_loop::session_result::{SessionDisposition, SessionResult};

#[test]
fn reads_both_exact_dispositions_without_granting_force_stop_completion() {
    for (bytes, want) in [
        (b"grove-continue".as_slice(), SessionDisposition::Continue),
        (
            b"grove-force-stop".as_slice(),
            SessionDisposition::ForceStop,
        ),
    ] {
        let result = SessionResult::allocate().unwrap();
        result.stdout().unwrap().write_all(bytes).unwrap();
        assert_eq!(result.read().unwrap(), want);
    }
}

#[test]
fn reads_the_exact_result_after_the_writer_advanced_the_shared_offset() {
    let result = SessionResult::allocate().unwrap();
    let mut stdout = result.stdout().unwrap();
    stdout.write_all(b"grove-").unwrap();
    stdout.write_all(b"continue").unwrap();
    drop(stdout);
    assert_eq!(result.read().unwrap(), SessionDisposition::Continue);
}

#[test]
fn missing_and_unsupported_bytes_never_mean_continue() {
    for bytes in [
        b"".as_slice(),
        b"continue",
        b"grove-continue\n",
        b" grove-continue",
        b"grove-continue\0",
        b"grove-force-stop\n",
        b"grove-force-stop\0",
        b" grove-force-stop",
        b"grove-force-sto",
        b"grove-force-stopgrove-continue",
        b"session-noisegrove-continue",
        b"\xff",
        br#"{"result":"grove-continue"}"#,
    ] {
        let result = SessionResult::allocate().unwrap();
        result.stdout().unwrap().write_all(bytes).unwrap();
        assert!(result.read().is_err(), "accepted {bytes:?}");
    }
}

#[test]
fn oversized_result_reads_only_the_literal_and_one_rejecting_byte() {
    let result = SessionResult::allocate().unwrap();
    let mut stdout = result.stdout().unwrap();
    stdout.write_all(b"grove-continue").unwrap();
    // A sparse tail would be expensive to read without the caller's bound.
    stdout.set_len(1024 * 1024).unwrap();
    assert!(result.read().is_err());
    // Clones share the offset: this sees how far the actual reader consumed.
    assert_eq!(stdout.stream_position().unwrap(), 17);
}

#[test]
fn capture_is_a_private_unlinked_regular_file() {
    let result = SessionResult::allocate().unwrap();
    let metadata = result.stdout().unwrap().metadata().unwrap();
    assert!(metadata.is_file());
    assert_eq!(metadata.nlink(), 0);
    assert_eq!(metadata.mode() & 0o777, 0o600);
}
