//! `delete` through the public surface: the root, everything beneath it, and an
//! honest account of what went.
//!
//! This is the operation the library said it did not have — and the record that
//! said so, [`entries-are-never-removed`], is about removing an **entry**, which
//! this is not. Removing an entry lowers the visible key maximum and the next
//! allocation re-issues a live key; deleting the root ends the tree, so there is
//! no next allocation to be wrong.
//!
//! What is *not* here is deleting a vacancy, and its absence is the point:
//! `delete` is on the write guard, a vacancy is not one, and the call does not
//! typecheck. The compile-fail proof is a doc test on
//! [`Vacancy`](ordinal_fs_tree::fs::Vacancy), beside the type that makes the
//! claim.
//!
//! Every test here names the model claim it discharges, or says it has none.
//!
//! [`entries-are-never-removed`]: https://github.com/Linkuistics/grove/blob/main/docs/adr/entries-are-never-removed.md

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use ordinal_fs_tree::fs::{Reading, WriteGuard, Writing};
use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusName};
use ordinal_fs_tree::{Error, Key, NewEntry, Removed, Target};
use tempfile::TempDir;

fn draft(label: &str) -> Parts {
    Parts::lesson(
        Status::Draft,
        Label::new(label).expect("a well-formed label"),
    )
}

fn topic(label: &str) -> Parts {
    Parts::module(Label::new(label).expect("a well-formed label"))
}

/// A directory that exists, holding a root path that does not.
fn nowhere() -> (TempDir, PathBuf) {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = temporary.path().join("syllabus");
    (temporary, root)
}

/// The tree every test below starts from, built through the library:
///
/// ```text
/// OVERVIEW.md
/// 01-draft-alpha-i1.md
/// 02-topic-i2/
///   01-draft-inner-i3.md
/// ```
fn built(root: &Path) {
    let Writing::Vacancy(vacancy) = opened_for_writing(root) else {
        unreachable!("the root is not there")
    };
    vacancy
        .initialize(
            Some(b"the course".to_vec()),
            vec![NewEntry::empty(draft("alpha"))],
        )
        .expect("initializing");
    tree(root)
        .append(Target::Root, NewEntry::empty(topic("topic")))
        .expect("appending a module");
    tree(root)
        .append(Target::Key(Key::new(2)), NewEntry::empty(draft("inner")))
        .expect("appending a lesson");
}

fn opened_for_writing(root: &Path) -> Writing<SyllabusName> {
    ordinal_fs_tree::fs::write::<SyllabusName>(root).expect("opening for writing")
}

fn opened_for_reading(root: &Path) -> Reading<SyllabusName> {
    ordinal_fs_tree::fs::read::<SyllabusName>(root).expect("opening for reading")
}

fn tree(root: &Path) -> WriteGuard<SyllabusName> {
    opened_for_writing(root).expect_tree("the tree is there")
}

/// Paths as strings relative to the root, so a failure reads as a tree.
fn beneath(root: &Path, removed: &Removed) -> Vec<String> {
    removed
        .entries
        .iter()
        .map(|path| {
            path.strip_prefix(root)
                .expect("every entry is beneath the root")
                .display()
                .to_string()
        })
        .collect()
}

/// Whether the mode bits actually bind for this process.
///
/// They do not for `root`, which is the one environment where a test built on a
/// refused write silently passes for the wrong reason. Asserted rather than
/// skipped, in the shape `docs/formalism-findings.md` entry 006 established for
/// the same class of problem.
fn writing_is_refused(directory: &Path) -> bool {
    let probe = directory.join(".probe");
    let refused = fs::File::create(&probe).is_err();
    if !refused {
        // Only reachable where the mode bits do not bind, and the caller returns
        // straight afterwards — but a fixture that leaves a file behind is a
        // fixture the next assertion has to know about.
        let _ = fs::remove_file(&probe);
    }
    refused
}

/// No model claim: neither model holds a root that ceases to exist.
///
/// The round trip. What was a tree is a **vacancy** afterwards — not an empty
/// tree, which is a different answer admitting different operations — and the
/// containing directory is untouched, which is what the lock was always on.
#[test]
fn delete_leaves_a_vacancy_where_the_tree_was() {
    let (temporary, root) = nowhere();
    built(&root);
    let removed = tree(&root).delete().expect("deleting");

    assert_eq!(removed.root, root);
    assert!(!root.exists(), "the root is gone");
    assert!(temporary.path().is_dir(), "its containing directory is not");
    assert!(
        opened_for_reading(&root).is_vacant(),
        "what is left is a vacancy and not an empty tree"
    );
}

/// No model claim.
///
/// **Children before the level that held them, and the root last.** The order is
/// the operation's own postcondition — a caller saying what it destroyed reads
/// this list — and it is reproducible because each level goes in the listing's
/// sorted order, which is the same order a snapshot is read in.
///
/// The root is not in `entries`: it is the level they were in, and it has no
/// name the domain ever parsed. `initialize` puts no report row on it for the
/// same reason.
#[test]
fn the_entries_go_children_first_and_the_root_goes_last() {
    let (_temporary, root) = nowhere();
    built(&root);
    let removed = tree(&root).delete().expect("deleting");

    assert_eq!(
        beneath(&root, &removed),
        vec![
            "01-draft-alpha-i1.md",
            "02-topic-i2/01-draft-inner-i3.md",
            "02-topic-i2",
            "OVERVIEW.md",
        ]
    );
    assert_eq!(removed.root, root);
}

/// No model claim: a foreign entry is one the domain declined to parse, and
/// neither model holds a name it cannot read.
///
/// **The report counts what the walk skips.** A snapshot holds only the entries
/// the domain named, and `Removed` is not built from one — a deletion acts on
/// the root, so a stray file goes with it, and a report that left it out would
/// undercount what was destroyed. This is the whole reason `Removed` carries
/// paths rather than a third bucket of `N`.
#[test]
fn a_foreign_entry_is_removed_and_reported() {
    let (_temporary, root) = nowhere();
    built(&root);
    fs::write(root.join("scratch.tmp"), b"not the domain's").expect("a foreign file");

    let removed = tree(&root).delete().expect("deleting");

    assert!(
        beneath(&root, &removed).contains(&"scratch.tmp".to_string()),
        "the foreign entry is in the report: {removed:?}"
    );
    assert!(!root.exists());
}

/// No model claim: neither model holds a symbolic link.
///
/// **The security property.** Descent is decided by the same unfollowed look a
/// snapshot is read through, so a link naming a directory *outside* the root is
/// unlinked as a link. Following it would take this operation out of the tree
/// entirely — out of the one directory the advisory lock covers — and destroy
/// something the caller never named.
#[test]
fn a_link_out_of_the_tree_is_unlinked_and_its_target_is_untouched() {
    let (temporary, root) = nowhere();
    built(&root);
    let outside = temporary.path().join("outside");
    fs::create_dir(&outside).expect("a directory outside the tree");
    fs::write(outside.join("precious"), b"not the tree's").expect("something in it");
    std::os::unix::fs::symlink(&outside, root.join("elsewhere")).expect("a link out of the tree");

    let removed = tree(&root).delete().expect("deleting");

    assert!(!root.exists(), "the tree is gone");
    assert!(outside.is_dir(), "and what the link named is not");
    assert!(outside.join("precious").is_file(), "nor what was in it");
    assert!(
        beneath(&root, &removed).contains(&"elsewhere".to_string()),
        "the link itself went, and is reported: {removed:?}"
    );
}

/// No model claim.
///
/// **A removal that stops partway says how far it got, and claims nothing else.**
/// `Error::Failed` promises the tree is as it was found and
/// `FailedPartiallyRolledBack` reports an unwind that failed; neither is true
/// here, because a removal has nothing to put back and never attempts to. What
/// the consumer gets instead is the paths that are gone.
#[test]
fn a_removal_that_stops_partway_reports_what_had_already_gone() {
    let (_temporary, root) = nowhere();
    built(&root);
    let level = root.join("02-topic-i2");
    // Readable and traversable, but not writable: the listing succeeds and the
    // unlink inside it does not.
    fs::set_permissions(&level, fs::Permissions::from_mode(0o500))
        .expect("making a level read-only");
    if !writing_is_refused(&level) {
        // The one environment where the mode bits do not bind. Stated rather
        // than skipped silently.
        assert_eq!(
            unsafe { libc::geteuid() },
            0,
            "a non-root process should have been refused a write into a 0o500 directory"
        );
        return;
    }

    let error = tree(&root).delete().expect_err("the removal is stopped");
    let Error::RemovalStopped {
        root: reported,
        path,
        removed,
        ..
    } = &error
    else {
        panic!("the removal's own error, and not another operation's: {error:?}")
    };
    assert_eq!(reported, &root);
    assert_eq!(path, &level.join("01-draft-inner-i3.md"));
    assert_eq!(
        removed,
        &vec![root.join("01-draft-alpha-i1.md")],
        "what went before the failure, in the order it went"
    );
    assert!(
        error.to_string().contains("neither the state"),
        "the message says the tree is in neither state: {error}"
    );
    assert!(root.is_dir(), "and the rest of the tree is still there");

    fs::set_permissions(&level, fs::Permissions::from_mode(0o700))
        .expect("giving the level back its permissions, so the fixture can be cleaned up");
}

/// No model claim.
///
/// The other side of the same variant: a removal that got **nowhere** left the
/// tree as it was found, and the message says that instead. One variant and two
/// messages, because the two want different next steps and the difference is
/// readable off the report.
#[test]
fn a_removal_that_gets_nowhere_says_the_tree_is_as_it_was_found() {
    let (_temporary, root) = nowhere();
    built(&root);
    // The guard is taken while the root is still listable, and the permissions
    // change under it — which is what makes the *first* listing fail rather than
    // the opening.
    let guard = tree(&root);
    fs::set_permissions(&root, fs::Permissions::from_mode(0o000))
        .expect("making the root unreadable");
    if !writing_is_refused(&root) {
        assert_eq!(
            unsafe { libc::geteuid() },
            0,
            "a non-root process should have been refused a read of a 0o000 directory"
        );
        return;
    }

    let error = guard.delete().expect_err("the removal is stopped");
    let Error::RemovalStopped { removed, .. } = &error else {
        panic!("the removal's own error: {error:?}")
    };
    assert!(removed.is_empty(), "nothing had gone: {removed:?}");
    assert!(
        error.to_string().contains("as it was found"),
        "the message says so: {error}"
    );

    fs::set_permissions(&root, fs::Permissions::from_mode(0o700))
        .expect("giving the root back its permissions, so the fixture can be cleaned up");

    // The report is not the claim — the tree is. An implementation that removed
    // half of it and reported an empty `removed` would satisfy every assertion
    // above, which is the iff direction this test exists to hold.
    assert_eq!(
        listing(&root),
        vec![
            "01-draft-alpha-i1.md".to_string(),
            "02-topic-i2".to_string(),
            "OVERVIEW.md".to_string(),
        ]
    );
}

/// The names in a directory, sorted, so a failure reads as a level.
fn listing(directory: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(directory)
        .expect("reading a directory")
        .map(|entry| entry.expect("a listing entry").file_name())
        .map(|name| name.to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

/// No model claim.
///
/// **A deletion is the one operation that acts on the root as an object, so it
/// is the one that cannot accept a spelling naming something else.** Every
/// other operation uses the root as a container and lets the kernel resolve its
/// last component — `reading_on_disk.rs` establishes a symbolic link naming a
/// directory as an accepted spelling, and asserts that two spellings of one tree
/// take one lock. Here the link and the directory are two objects and only one
/// of them is the tree, so guessing is refused.
///
/// The trailing-slash form is the same case wearing a disguise:
/// `symlink_metadata("link/")` **resolves** the link and answers *directory*, so
/// a check that did not rebuild the path from its components would let exactly
/// the spelling it exists to catch straight through.
#[test]
fn a_root_spelled_through_a_symbolic_link_is_refused_both_ways() {
    for spelling in ["elsewhere", "elsewhere/"] {
        let (temporary, root) = nowhere();
        built(&root);
        let link = temporary.path().join("elsewhere");
        std::os::unix::fs::symlink(&root, &link).expect("a link naming the tree");
        let spelled = temporary.path().join(spelling);

        let Err(error) = ordinal_fs_tree::fs::write::<SyllabusName>(&spelled)
            .expect("the opening accepts this spelling, as every other operation does")
            .expect_tree("the link names the tree")
            .delete()
        else {
            panic!("`{spelling}` must not delete what the link names")
        };
        assert!(
            matches!(error, Error::RootIsNotSpelledDirectly { .. }),
            "{error:?}"
        );
        assert!(root.is_dir(), "the tree is untouched");
        assert!(link.symlink_metadata().is_ok(), "and so is the link");
        assert_eq!(listing(&root).len(), 3, "nothing under it went either");
    }
}

/// No model claim.
///
/// **A spelling that descends into the tree and comes back out through `..` is
/// refused, because the removal would take away one of its own components.**
/// `syllabus/02-topic-i2/..` names the tree perfectly well — `reading_on_disk.rs`
/// tests exactly that spelling — until the walk removes `02-topic-i2`, after
/// which every remaining path built on it stops resolving. The tree would be
/// left half destroyed with no spelling able to finish it, which is why this is
/// refused up front rather than reported afterwards.
#[test]
fn a_root_spelled_back_out_through_a_component_of_itself_is_refused() {
    let (_temporary, root) = nowhere();
    built(&root);
    let spelled = root.join("02-topic-i2").join("..");

    let Err(error) = tree(&spelled).delete() else {
        panic!("a spelling the removal would invalidate must not be acted on")
    };
    assert!(
        matches!(error, Error::RootIsNotSpelledDirectly { .. }),
        "{error:?}"
    );
    assert_eq!(
        listing(&root),
        vec![
            "01-draft-alpha-i1.md".to_string(),
            "02-topic-i2".to_string(),
            "OVERVIEW.md".to_string(),
        ],
        "nothing was removed"
    );
}

/// No model claim.
///
/// **The `..` rule is deliberately coarser than the danger, and this is the
/// case that shows it.** The cancelled component here is the tree's own
/// *ancestor*, which no removal can reach, so acting on the spelling would in
/// fact be safe. Telling that apart from the dangerous case means resolving the
/// path to find out which components lie inside the tree, and this module never
/// resolves anything — so the whole class is refused, and the cost is one
/// message asking for a direct spelling.
#[test]
fn a_spelling_that_cancels_a_name_is_refused_even_where_it_would_be_harmless() {
    let (temporary, root) = nowhere();
    built(&root);
    let above = temporary
        .path()
        .file_name()
        .expect("the temporary directory has a name");
    // `<tmp>/../<tmp>/syllabus`: the `..` cancels the temporary directory, which
    // is above the tree and therefore nothing the removal could take away.
    let harmless = temporary.path().join("..").join(above).join("syllabus");

    let Err(error) = tree(&harmless).delete() else {
        panic!("the whole class is refused, harmless members included")
    };
    assert!(
        matches!(error, Error::RootIsNotSpelledDirectly { .. }),
        "{error:?}"
    );
    assert!(root.is_dir(), "nothing was removed");
}

/// No model claim.
///
/// **Deletion is not an escape from a tree the domain cannot read.** This
/// judges *reachability* and not the removal — no guard is produced, so nothing
/// in the removal runs, and that is the whole claim: every operation begins at
/// an opening, and an opening halts on a name the consumer recognises and cannot
/// parse, so `delete` is not reachable at all on such a tree. That is deliberate rather than incidental: a library that would destroy
/// a tree it was refused permission to understand is one whose halt means
/// nothing.
#[test]
fn a_tree_the_domain_cannot_read_cannot_be_deleted_either() {
    let (_temporary, root) = nowhere();
    built(&root);
    // This domain's own shape, spelled a way its grammar does not write: a
    // one-digit ordinal, which `parse` reports as `Malformed` carrying the
    // canonical spelling.
    fs::write(
        root.join("7-draft-omega-i7.md"),
        b"a name this domain cannot parse",
    )
    .expect("a malformed name");

    // `expect_err` is not available here: `Writing` is a guard and guards are
    // deliberately not `Debug`.
    let Err(error) = ordinal_fs_tree::fs::write::<SyllabusName>(&root) else {
        panic!("the opening halts before there is a guard to delete with")
    };
    assert!(
        matches!(error, Error::Malformed { .. }),
        "the domain's own halt, with the domain's own advice: {error:?}"
    );
    assert!(root.is_dir(), "and nothing was removed");
}

/// No model claim.
///
/// **The lock outlives the tree, which is why it is on the containing directory
/// and not on the root.** Deleting and initializing again is one root's whole
/// lifetime under one lock scope — and the second tree starts its keys at 1,
/// because the names *are* the counter and there are no names left.
#[test]
fn a_root_can_be_created_again_after_it_is_deleted() {
    let (_temporary, root) = nowhere();
    built(&root);
    tree(&root).delete().expect("deleting");

    let Writing::Vacancy(vacancy) = opened_for_writing(&root) else {
        unreachable!("what is left is a vacancy")
    };
    let report = vacancy
        .initialize(None, vec![NewEntry::empty(draft("again"))])
        .expect("initializing a second tree at the same root");

    assert_eq!(
        report
            .created()
            .iter()
            .map(|created| created.name.to_string())
            .collect::<Vec<_>>(),
        vec!["01-draft-again-i1.md"],
        "the counter is the names, and there are none left to count"
    );
}

/// No model claim.
///
/// An empty tree deletes to an empty report, and the root still goes. The
/// operation has no lower bound to special-case: `entries` is simply empty.
#[test]
fn an_empty_tree_deletes_to_an_empty_report() {
    let (_temporary, root) = nowhere();
    fs::create_dir(&root).expect("an empty root directory");

    let removed = tree(&root).delete().expect("deleting");

    assert!(removed.entries.is_empty(), "{removed:?}");
    assert_eq!(removed.root, root);
    assert!(matches!(opened_for_reading(&root), Reading::Vacant));
}
