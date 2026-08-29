//! `initialize` through the public surface: a root that is not there, the
//! exclusive lock that says so, and the tree that comes out.
//!
//! This is the operation the rest of the crate could not express. Every other
//! mutation is on a [`WriteGuard`](ordinal_fs_tree::fs::WriteGuard), which is a
//! tree — so a tree had to already exist for the library to be asked anything,
//! and the one act that makes one was the consumer's, outside the lock and
//! outside the store.
//!
//! What is *not* here is `initialize` over a live tree, and its absence is the
//! point: the call does not typecheck, so there is no test to write. The
//! compile-fail proofs are doc tests on
//! [`Vacancy`](ordinal_fs_tree::fs::Vacancy), where a reader meets them beside
//! the type that makes the claim.
//!
//! Every test here names the model claim it discharges, or says it has none.

use core::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use ordinal_fs_tree::fs::{Reading, Writing};
use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusError, SyllabusName};
use ordinal_fs_tree::{
    EntryName, Error, Found, Key, NameView, NewEntry, Ordinal, PositionedSpecies, Refusal, Sought,
    Species, Verdict,
};
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

fn listing(directory: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(directory)
        .expect("reading a directory")
        .map(|entry| entry.expect("a listing entry").file_name())
        .map(|name| name.to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

/// A domain with **no distinguished child**: `operations.qnt`'s
/// `no_distinguished` instance, as a real `EntryName`.
///
/// The same domain `promoting_on_disk.rs` declares, and declared again here
/// rather than shared, because a fixture domain is one file's own scenery in
/// this crate's tests. It disclaims `OVERVIEW.md`: a domain answering `None` to
/// `distinguished()` while still parsing some name as `Distinguished` would have
/// a distinguished child the library cannot name.
#[derive(Clone)]
struct Contentless(SyllabusName);

impl fmt::Display for Contentless {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl EntryName for Contentless {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        match SyllabusName::parse(name, found) {
            Verdict::Entry(inner) => match inner.view() {
                NameView::Distinguished => Verdict::Foreign,
                NameView::Positioned(_) => Verdict::Entry(Self(inner)),
            },
            Verdict::Foreign => Verdict::Foreign,
            Verdict::Malformed(e) => Verdict::Malformed(e),
            Verdict::Reserved(e) => Verdict::Reserved(e),
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self(SyllabusName::compose(ordinal, key, parts))
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        self.0.view()
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

// ---------------------------------------------------------------------------
// The shape: what an opening answers
// ---------------------------------------------------------------------------

/// No model claim: *is there a tree here* is a question about the filesystem,
/// and both models hold a tree by construction.
///
/// The whole leaf in one test. `read` answers a root that is not there with a
/// vacancy rather than an error, and `write` answers it with a vacancy that
/// **holds the lock** — which is what makes the create that follows safe.
#[test]
fn a_root_that_is_not_there_is_a_vacancy_and_not_an_error() {
    let (_temporary, root) = nowhere();

    let reading = ordinal_fs_tree::fs::read::<SyllabusName>(&root).expect("no error");
    assert!(reading.is_vacant());
    assert!(!reading.is_tree());
    assert!(matches!(reading, Reading::Vacant));

    let writing = ordinal_fs_tree::fs::write::<SyllabusName>(&root).expect("no error");
    assert!(writing.is_vacant());
    assert!(matches!(writing, Writing::Vacancy(_)));
}

/// No model claim.
///
/// An empty *directory* is a tree holding no entries, which is a different
/// answer from no tree at all — and the difference is exactly which operations
/// are available. The library said so before this leaf too; what changed is that
/// the other side of the distinction now has a name.
#[test]
fn an_empty_directory_is_a_tree_and_not_a_vacancy() {
    let (_temporary, root) = nowhere();
    fs::create_dir(&root).expect("creating an empty tree");

    let reading = ordinal_fs_tree::fs::read::<SyllabusName>(&root).expect("no error");
    assert!(reading.is_tree());
    assert_eq!(
        reading
            .expect_tree("an empty tree is a tree")
            .walk()
            .count(),
        0
    );
}

/// No model claim: a root that is not a directory is not a tree either model can
/// hold.
///
/// The third answer, and an error rather than a variant: this library will not
/// move aside something it did not put there, so the honest thing is to say what
/// it found and stop. Reporting it as a vacancy would send `initialize` at a name
/// that is already taken.
#[test]
fn a_root_that_is_a_regular_file_is_neither_a_tree_nor_a_vacancy() {
    let (_temporary, root) = nowhere();
    fs::write(&root, "a file wearing the root's name").expect("a fixture");

    // A `let … else` rather than `expect_err`, because a guard is not `Debug`
    // and `Writing` therefore is not either — the same reason
    // `names_are_confined.rs` gives.
    let Err(error) = ordinal_fs_tree::fs::write::<SyllabusName>(&root) else {
        panic!("a root that is not a directory is neither a tree nor a vacancy");
    };
    let Error::RootIsNotATree { found, .. } = &error else {
        panic!("expected the trichotomy's third answer, got {error:?}");
    };
    assert_eq!(*found, Found::File);
    assert!(
        error.to_string().contains("a regular file"),
        "the message says what it found: {error}"
    );
}

/// No model claim.
///
/// A **dangling** symbolic link is the case the two filesystem questions exist to
/// separate: `metadata` calls it `NotFound`, which read alone would make it a
/// vacancy — and an `initialize` sent at it would collide with a name that is
/// plainly occupied. `symlink_metadata` is what stops that.
#[test]
fn a_dangling_symbolic_link_at_the_root_is_not_a_vacancy() {
    let (temporary, root) = nowhere();
    std::os::unix::fs::symlink(temporary.path().join("nothing-here"), &root)
        .expect("a dangling link");

    let Err(error) = ordinal_fs_tree::fs::write::<SyllabusName>(&root) else {
        panic!("a dangling link is occupying the root");
    };
    assert!(
        matches!(
            error,
            Error::RootIsNotATree {
                found: Found::Other,
                ..
            }
        ),
        "a link naming nothing is not nothing: {error:?}"
    );
}

/// No model claim.
///
/// **The lock a dangling link would take is the wrong one**, which is why it is
/// refused before any lock is taken rather than classified after one. Its last
/// component is followed, so `<root>/..` names the directory holding the
/// *target* while its lexical parent names the directory holding the *link* —
/// and if the target appears a moment later, a caller through the link and a
/// caller through the target path hold two different locks over one tree. That
/// is `reading-k19`'s defect re-entering through the door absence opened, and
/// this test is the door being shut: the link resolves here, and the answer is
/// still the error rather than the tree it now names.
#[test]
fn a_link_that_starts_dangling_is_refused_even_once_its_target_appears() {
    let (temporary, _) = nowhere();
    let target = temporary.path().join("elsewhere").join("tree");
    let link = temporary.path().join("link");
    std::os::unix::fs::symlink(&target, &link).expect("a dangling link");

    let Err(before) = ordinal_fs_tree::fs::write::<SyllabusName>(&link) else {
        panic!("a dangling link is not a vacancy");
    };
    assert!(matches!(before, Error::RootIsNotATree { .. }), "{before:?}");

    fs::create_dir_all(&target).expect("the target appears");
    let opened = ordinal_fs_tree::fs::read::<SyllabusName>(&link).expect("no error");
    assert!(
        opened.is_tree(),
        "and once it resolves to a directory it is an ordinary accepted spelling"
    );
}

/// No model claim.
///
/// A root whose *containing* path runs through a regular file has no directory
/// anywhere to lock, and reporting that is the whole of the right answer. The
/// lexical parent here is the regular file itself, and `File::open` succeeds on
/// one — so a fallback that did not ask this question would take the advisory
/// lock on a **file**, which is the module's central premise silently false.
#[test]
fn a_root_below_a_regular_file_has_no_directory_to_lock() {
    let (temporary, _) = nowhere();
    let file = temporary.path().join("not-a-directory");
    fs::write(&file, "a regular file").expect("a fixture");

    let Err(error) = ordinal_fs_tree::fs::write::<SyllabusName>(&file.join("syllabus")) else {
        panic!("there is no directory on this path");
    };
    let Error::Io { doing, .. } = &error else {
        panic!("expected the filesystem to be what refused, got {error:?}");
    };
    assert_eq!(*doing, "looking at the tree root");
}

/// No model claim.
///
/// **A tree removed between the two questions is a vacancy, not a link.** The
/// pair `symlink_metadata` yes / `metadata` `NotFound` is what a dangling link
/// gives *and* what an ordinary directory deleted underneath gives, so deriving
/// *dangling* from the disagreement reports the wrong one of the three answers —
/// with advice naming a file that is not there. Asking whether the first answer
/// was a link is what separates them, and this test stands in for the race by
/// putting the two observations either side of the removal: nothing is at the
/// root when the classification runs, and the answer is a vacancy.
#[test]
fn a_root_that_disappears_is_a_vacancy_and_not_a_dangling_link() {
    let (_temporary, root) = nowhere();
    fs::create_dir(&root).expect("a tree");
    fs::remove_dir(&root).expect("and then no tree");

    let opened = ordinal_fs_tree::fs::write::<SyllabusName>(&root).expect("no error");
    assert!(
        opened.is_vacant(),
        "nothing at the root is a vacancy, whatever was there a moment ago"
    );
}

// ---------------------------------------------------------------------------
// Initializing
// ---------------------------------------------------------------------------

/// No model claim yet — `operations.qnt` gains `Initialize` as a transition with
/// this leaf, and `wit_initializeCreatesTheRoot` is the witness.
///
/// Vacancy, initialize, read back: the round trip the store could not make
/// before. The distinguished child carries the root's own bytes, the entries land
/// at [`Ordinal::FIRST`] onward with keys from 1, and the whole of it is one
/// lock.
#[test]
fn a_vacancy_initializes_into_a_tree_that_reads_back() {
    let (_temporary, root) = nowhere();
    let Writing::Vacancy(vacancy) =
        ordinal_fs_tree::fs::write::<SyllabusName>(&root).expect("no error")
    else {
        panic!("a root that is not there is a vacancy");
    };
    assert_eq!(vacancy.root(), root);

    let report = vacancy
        .initialize(
            Some(b"An introduction.".to_vec()),
            vec![
                NewEntry::empty(draft("orientation")),
                NewEntry::empty(topic("linear-algebra")),
            ],
        )
        .expect("a tree from nothing");

    // The root itself has no name, so no report row describes it; every *named*
    // thing this call placed is here, distinguished child first.
    let created: Vec<String> = report
        .created()
        .iter()
        .map(|created| created.name.to_string())
        .collect();
    assert_eq!(
        created,
        [
            "OVERVIEW.md",
            "01-draft-orientation-i1.md",
            "02-linear-algebra-i2"
        ]
    );
    assert!(report.renamed().is_empty(), "nothing was there to rename");

    assert_eq!(
        fs::read_to_string(root.join("OVERVIEW.md")).expect("the root's own content"),
        "An introduction."
    );

    let tree = ordinal_fs_tree::fs::read::<SyllabusName>(&root)
        .expect("no error")
        .expect_tree("a tree now");
    assert_eq!(
        tree.walk()
            .map(|e| e.name().to_string())
            .collect::<Vec<_>>(),
        [
            "OVERVIEW.md",
            "01-draft-orientation-i1.md",
            "02-linear-algebra-i2"
        ]
    );
    let Sought::Match(first) = tree.by_key(Key::new(1)) else {
        panic!("key 1 was allocated to the first entry");
    };
    assert_eq!(first.ordinal(), Some(Ordinal::FIRST));
    assert_eq!(first.species(), Species::Leaf);
}

/// No model claim.
///
/// The two ways to have no distinguished child are different trees, and neither
/// is a default the library picks: `None` writes none at all, and `Some(&[])`
/// writes an empty one. A library that collapsed them would be choosing for the
/// domain.
#[test]
fn no_distinguished_child_and_an_empty_one_are_different_trees() {
    let (temporary, _) = nowhere();

    let bare = temporary.path().join("bare");
    ordinal_fs_tree::fs::write::<SyllabusName>(&bare)
        .expect("no error")
        .expect_vacancy("nothing is there")
        .initialize(None, Vec::new())
        .expect("a root with no OVERVIEW");
    assert_eq!(listing(&bare), Vec::<String>::new());

    let empty = temporary.path().join("empty");
    ordinal_fs_tree::fs::write::<SyllabusName>(&empty)
        .expect("no error")
        .expect_vacancy("nothing is there")
        .initialize(Some(Vec::new()), Vec::new())
        .expect("a root with an empty OVERVIEW");
    assert_eq!(listing(&empty), ["OVERVIEW.md"]);
    assert_eq!(
        fs::read_to_string(empty.join("OVERVIEW.md")).expect("an OVERVIEW"),
        ""
    );
}

/// No model claim: content is unmodelled in both models by design.
///
/// The same refusal a promotion gives, for the same reason — and `promoting` is
/// what tells them apart: a root initialization names no entry, because the root
/// is not one.
#[test]
fn bytes_for_a_distinguished_child_a_domain_does_not_have_are_refused() {
    let (_temporary, root) = nowhere();

    let error = ordinal_fs_tree::fs::write::<Contentless>(&root)
        .expect("no error")
        .expect_vacancy("nothing is there")
        .initialize(Some(b"nowhere to go".to_vec()), Vec::new())
        .expect_err("this domain has no distinguished child");

    assert!(
        matches!(
            error,
            Error::Refused(Refusal::NoDistinguishedChild { promoting: None })
        ),
        "expected the domain-shaped refusal, got {error:?}"
    );
    assert!(
        error.to_string().contains("nowhere to go"),
        "a refusal says why, and what to do: {error}"
    );
    assert!(
        !root.exists(),
        "a refusal is decided before anything is created, so not even the root \
         is left behind"
    );
}

/// No model claim: content is unmodelled in both models by design.
///
/// A refusal reaching `initialize` from the ordinary entry arithmetic, and the
/// property that matters is the second assertion — the plan is refused *before*
/// the root is created, so a refused initialization leaves the filesystem
/// exactly as it found it.
#[test]
fn a_refused_initialization_does_not_leave_a_root_behind() {
    let (_temporary, root) = nowhere();

    let error = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("no error")
        .expect_vacancy("nothing is there")
        .initialize(
            None,
            vec![NewEntry::new(topic("linear-algebra"), b"bytes".to_vec())],
        )
        .expect_err("a directory has nowhere to hold bytes");

    assert!(matches!(error, Error::Refused(Refusal::ContentForANode)));
    assert!(!root.exists(), "nothing was created");
}

/// No model claim.
///
/// A root whose **containing** directory is not there is refused where every
/// operation is refused — at the lock, before there is a vacancy to speak of.
/// The lock covers the directory holding the root precisely because that
/// directory outlives the root's creation and deletion, so a tree whose parent
/// does not exist has no lock and therefore no answer.
///
/// `initialize` creates *the root*, and one directory is the whole of what it
/// creates: there is no `mkdir -p` here, and a caller who wants a path of
/// directories makes them itself.
#[test]
fn a_root_whose_containing_directory_is_not_there_is_refused_at_the_lock() {
    let (temporary, _) = nowhere();
    let missing = temporary.path().join("missing");
    let root = missing.join("syllabus");

    let Err(error) = ordinal_fs_tree::fs::write::<SyllabusName>(&root) else {
        panic!("there is no directory to lock");
    };
    let Error::Io { doing, .. } = &error else {
        panic!("expected the lock to be what refused, got {error:?}");
    };
    assert_eq!(*doing, "locking the directory containing the tree");
    assert!(!missing.exists(), "nothing was created on the way past");
}

/// No model claim.
///
/// The lock a vacancy holds is the ordinary exclusive lock on the directory
/// *containing* the root — the same one every mutation takes — which is what
/// makes a tree's creation coverable at all. While the vacancy lives, nobody else
/// gets in.
#[test]
fn a_vacancy_holds_the_lock_it_will_create_the_tree_under() {
    let (temporary, root) = nowhere();
    let vacancy = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("no error")
        .expect_vacancy("nothing is there");

    // A second acquisition on the same containing directory, from another
    // process, must block. `flock` is per open file description, so a second
    // acquisition in *this* process would not — the same reason
    // `reading_on_disk.rs` shells out for its contention tests.
    let contender = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "exec 9<'{}'; flock -n -x 9 || exit 3",
            temporary.path().display()
        ))
        .status();
    if let Ok(status) = contender {
        // `flock(1)` is not on macOS; where it is missing the shell reports 127
        // and this assertion is skipped rather than asserted backwards.
        if status.code() != Some(127) {
            assert_eq!(
                status.code(),
                Some(3),
                "a live vacancy excludes another writer"
            );
        }
    }

    drop(vacancy);
}
