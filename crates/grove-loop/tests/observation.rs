use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::path::Path;

use grove_loop::{try_observe, TreeObservation};

fn fixture() -> tempfile::TempDir {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    fs::create_dir_all(root.join("01-k1")).unwrap();
    fs::write(root.join("_BRIEF.md"), "root").unwrap();
    fs::write(root.join("01-k1/_branch.md"), "branch").unwrap();
    fs::write(root.join("01-k1/01-impl--work-k2.md"), "leaf").unwrap();
    work
}

fn ready(work: &Path, candidates: &[Option<u32>]) -> grove_loop::CapturedTree {
    match try_observe(work, candidates).unwrap() {
        TreeObservation::Ready(tree) => tree,
        _ => panic!("expected a captured tree"),
    }
}

#[test]
fn captures_selected_bytes_and_fallback_from_one_validated_tree() {
    let work = fixture();
    for (candidates, key, bytes) in [
        (vec![Some(2)], Some(2), "leaf"),
        (vec![Some(99), Some(1)], Some(1), "branch"),
        (vec![Some(99)], None, "root"),
        (vec![None, Some(2)], None, "root"),
    ] {
        let captured = ready(work.path(), &candidates);
        assert_eq!(captured.selected, key);
        assert_eq!(captured.content.as_ref().unwrap(), bytes.as_bytes());
        assert_eq!(captured.root(), work.path().join(".grove"));
        assert_eq!(captured.snapshot().walk().count(), 4);
        assert!(captured.lifetime.at(work.path()).unwrap());
    }
    assert!(!work.path().join(".jj").exists());
}

#[test]
fn captures_release_the_tree_lock_and_pin_distinct_root_lifetimes() {
    let work = fixture();
    let first = ready(work.path(), &[Some(2)]);
    let second = ready(work.path(), &[Some(2)]);
    assert!(first.lifetime.same(&second.lifetime).unwrap());
    let directory = File::open(work.path()).unwrap();
    // A retained capture must not block a real writer on an independent fd.
    assert_eq!(
        unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
        0
    );
    assert!(matches!(
        try_observe(work.path(), &[]).unwrap(),
        TreeObservation::Busy
    ));
    drop(directory);
    fs::rename(work.path().join(".grove"), work.path().join("old")).unwrap();
    fs::create_dir(work.path().join(".grove")).unwrap();
    fs::write(work.path().join(".grove/_BRIEF.md"), "replacement").unwrap();
    let replacement = ready(work.path(), &[Some(2)]);
    assert!(!first.lifetime.at(work.path()).unwrap());
    assert!(!first.lifetime.same(&replacement.lifetime).unwrap());
    assert_eq!(first.content.unwrap(), b"leaf");
    assert_eq!(replacement.content.unwrap(), b"replacement");
}

#[test]
fn vacant_and_invalid_trees_are_distinct_and_capture_creates_nothing() {
    let work = tempfile::tempdir().unwrap();
    assert!(matches!(
        try_observe(work.path(), &[]).unwrap(),
        TreeObservation::Vacant
    ));
    assert_eq!(fs::read_dir(work.path()).unwrap().count(), 0);
    let work = fixture();
    fs::write(
        work.path().join(".grove/02-DONE-impl--duplicate-k2.md"),
        "duplicate",
    )
    .unwrap();
    let error = try_observe(work.path(), &[]).err().unwrap().to_string();
    assert!(error.contains("duplicate"), "{error}");
}
