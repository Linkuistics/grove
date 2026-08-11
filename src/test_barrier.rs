//! Publication rule for the crate's process-interruption barriers.
//!
//! The interruption seams (`finish_cleanup`'s `GROVE_TEST_FINISH_CLEANUP_*` and
//! `tree_promotion`'s `GROVE_TEST_PROMOTION_*`) park a subprocess at a named
//! step and hand the waiting test a payload naming what it stopped on. Those
//! variables are deliberately test-prefixed and are not user configuration; the
//! seams are inert unless a test sets them.
//!
//! Publishing that payload with `fs::write` is two observable steps —
//! `open(O_CREAT|O_TRUNC)` then `write` — and every waiter treats the barrier's
//! *existence* as its readiness signal. A waiter scheduled inside that window
//! sees the barrier, reads zero bytes, and acts on an empty payload; under
//! parallel-suite load that is exactly the flake this module exists to remove.
//!
//! So the payload goes to a staged sibling first and reaches the barrier's own
//! name by `rename(2)`, which POSIX makes atomic for other processes: the
//! barrier path resolves either to nothing or to the complete payload. This is
//! the same shape the production cleanup marker already publishes with, one
//! grain simpler because a test barrier needs no directory descriptor or
//! no-replace guarantee.

use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

/// Distinguishes concurrent publications inside one process. The seams
/// themselves are single-threaded, but in-crate unit tests drive them from
/// parallel test threads, so the staged name cannot be derived from the
/// barrier's name and the pid alone.
static STAGED_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Publish `payload` at `barrier` in one observable step.
pub(crate) fn publish_test_barrier(barrier: &Path, payload: &[u8]) -> io::Result<()> {
    let name = barrier.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("test barrier path {} has no file name", barrier.display()),
        )
    })?;
    let parent = barrier
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let sequence = STAGED_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let mut staged_name = OsString::from(".");
    staged_name.push(name);
    staged_name.push(format!(".{}.{sequence}.publishing", std::process::id()));
    let staged = parent.join(staged_name);

    fs::write(&staged, payload).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("writing staged test barrier {}: {error}", staged.display()),
        )
    })?;
    fs::rename(&staged, barrier).map_err(|error| {
        // A staged file left behind would be a stray entry in a directory the
        // test inspects, so drop it before reporting; the publication failure
        // is what the caller needs to see either way.
        let _ = fs::remove_file(&staged);
        io::Error::new(
            error.kind(),
            format!("publishing test barrier {}: {error}", barrier.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::publish_test_barrier;
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    use std::path::Path;
    use tempfile::TempDir;

    fn inode(path: &Path) -> u64 {
        fs::metadata(path).unwrap().ino()
    }

    #[test]
    fn publication_replaces_the_barrier_rather_than_writing_through_it() {
        let temporary = TempDir::new().unwrap();
        let barrier = temporary.path().join("barrier");
        let witness = temporary.path().join("witness");
        fs::write(&barrier, "stale\n").unwrap();
        fs::hard_link(&barrier, &witness).unwrap();
        let published = inode(&barrier);

        publish_test_barrier(&barrier, b"before-non-directory-unlink").unwrap();

        // The witness shares the inode the barrier had. An implementation that
        // truncates and writes through the live path would have emptied it and
        // then refilled it — the observable window a waiter reads as an empty
        // payload — and the witness would carry the new bytes.
        assert_eq!(fs::read_to_string(&witness).unwrap(), "stale\n");
        assert_eq!(
            fs::read(&barrier).unwrap(),
            b"before-non-directory-unlink".to_vec()
        );
        assert_ne!(inode(&barrier), published);
    }

    #[test]
    fn publication_leaves_no_staged_sibling_behind() {
        let temporary = TempDir::new().unwrap();
        let barrier = temporary.path().join("barrier");

        publish_test_barrier(&barrier, b"before-entry").unwrap();

        let entries: Vec<_> = fs::read_dir(temporary.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        assert_eq!(entries, vec![std::ffi::OsString::from("barrier")]);
    }

    #[test]
    fn repeated_publication_at_one_barrier_carries_the_latest_payload() {
        let temporary = TempDir::new().unwrap();
        let barrier = temporary.path().join("barrier");

        publish_test_barrier(&barrier, b"first-entry").unwrap();
        publish_test_barrier(&barrier, b"second-entry").unwrap();

        assert_eq!(fs::read(&barrier).unwrap(), b"second-entry".to_vec());
    }

    #[test]
    fn a_missing_barrier_directory_reports_the_barrier_path() {
        let temporary = TempDir::new().unwrap();
        let barrier = temporary.path().join("absent").join("barrier");

        let error = publish_test_barrier(&barrier, b"before-claim").unwrap_err();

        assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
        assert!(
            error.to_string().contains("barrier"),
            "{error}: the diagnostic must name the barrier it failed to stage"
        );
    }
}
