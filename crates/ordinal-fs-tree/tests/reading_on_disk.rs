//! The snapshot and the lock, against real directories.
//!
//! Every test here names the model claim it discharges, or says it has none.
//!
//! The pair this file exists for is [`a_foreign_directory_is_skipped_whole`] and
//! [`a_malformed_directory_halts_rather_than_vanishing`]. They are the same tree
//! twice, differing only in one filename, and the difference between *skipped*
//! and *halted* is the whole reason the parse trichotomy has three outcomes
//! rather than two: when the skipped name is a directory, an entire subtree
//! disappears from every traversal while the tree still reports itself healthy.

use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use ordinal_fs_tree::reference::SyllabusName;
use ordinal_fs_tree::{Entry, Error, Key, Sought, Species};
use tempfile::TempDir;

/// How long a lock test waits for something that should happen. Generous: it
/// bounds a *pass*, so a slow machine costs nothing.
const SOON: Duration = Duration::from_secs(10);

/// How long a lock test waits before concluding something is *not* happening.
/// This one bounds a *failure*, so it trades test time for confidence.
const A_WHILE: Duration = Duration::from_millis(300);

fn file(at: &Path, name: &str) {
    fs::write(at.join(name), "").expect("writing a fixture file");
}

fn dir(at: &Path, name: &str) -> PathBuf {
    let path = at.join(name);
    fs::create_dir(&path).expect("creating a fixture directory");
    path
}

/// `ARCHITECTURE.md`'s own example tree, on disk.
fn documents_tree() -> (TempDir, PathBuf) {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = dir(temporary.path(), "syllabus");
    file(&root, "OVERVIEW.md");
    file(&root, "01-published-orientation-i1.md");
    let algebra = dir(&root, "02-linear-algebra-i2");
    file(&algebra, "OVERVIEW.md");
    file(&algebra, "01-published-vectors-i5.md");
    file(&algebra, "02-draft-matrices-i6.md");
    file(&root, "03-draft-assessment-i9.md");
    (temporary, root)
}

fn read(root: &Path) -> Result<Vec<String>, Error<SyllabusName>> {
    let tree =
        ordinal_fs_tree::fs::read::<SyllabusName>(root)?.expect_tree("a tree, not a vacancy");
    Ok(tree.walk().map(|e| e.name().to_string()).collect())
}

/// Discharges no model claim. It is the end-to-end reading of the document's
/// own tree: the same names, the same order the pure test checks, arriving
/// through a real listing whose order the filesystem chose.
#[test]
fn the_documents_tree_reads_from_disk_in_walk_order() {
    let (_temporary, root) = documents_tree();
    assert_eq!(
        read(&root).expect("a well-formed tree"),
        [
            "OVERVIEW.md",
            "01-published-orientation-i1.md",
            "02-linear-algebra-i2",
            "OVERVIEW.md",
            "01-published-vectors-i5.md",
            "02-draft-matrices-i6.md",
            "03-draft-assessment-i9.md",
        ]
    );
}

/// Discharges no model claim, and there is none to discharge: `Sought` is a
/// type-level distinction neither model has. `operations.qnt` resolves a key
/// with `leastId`, which answers `-1` when nothing matched — an in-band
/// sentinel, and exactly the shape a word for *matched nothing* exists to
/// replace. A search adds no state transition, so nothing in either model moved
/// for it.
///
/// Both variants, from both searches, through the public interface — and the
/// door out to `Option`, which is the only way a search ever hands one back.
#[test]
fn a_search_answers_with_a_match_or_with_nothing() {
    let (_temporary, root) = documents_tree();
    let tree = ordinal_fs_tree::fs::read::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy");

    match tree.by_key(Key::new(6)) {
        Sought::Match(entry) => assert_eq!(entry.name().to_string(), "02-draft-matrices-i6.md"),
        Sought::Nothing => panic!("key 6 is in the document's tree"),
    }
    assert_eq!(
        tree.by_key(Key::new(99)),
        Sought::Nothing,
        "key 99 is not — and the tree is intact, so this is no refusal"
    );

    // A predicate this tree satisfies, and one it cannot: the deepest entry sits
    // at depth 2. Neither answer says anything is wrong with the tree.
    assert!(tree
        .seek(|entry| entry.species() == Species::Node)
        .is_match());
    assert!(tree.seek(|entry| entry.depth() > 2).is_nothing());

    // The door, both ways, and it round-trips.
    assert_eq!(
        tree.by_key(Key::new(1))
            .into_option()
            .map(|entry| entry.name().to_string())
            .as_deref(),
        Some("01-published-orientation-i1.md")
    );
    assert_eq!(
        Sought::from(tree.by_key(Key::new(99)).into_option()),
        Sought::Nothing
    );
    // The same door spelled the other way. The target type is written out
    // deliberately: `core`'s own `impl<T> From<T> for Option<T>` also applies
    // against an inferred `Option<_>`, so that spelling is ambiguous and this one
    // is not — the impl says so where a reader meets it.
    let out: Option<Entry<'_, SyllabusName>> = tree.by_key(Key::new(99)).into();
    assert!(out.is_none());
}

/// Discharges `NothingRecognisedIsSkipped`'s *Foreign* half — Alloy's claim is
/// that everything in a traversed directory either has a name or was
/// disclaimed, and a disclaimer is recursive. The file buried inside the foreign
/// directory would be `Malformed` if anything classified it; that reading
/// succeeds is what proves the walk never entered.
#[test]
fn a_foreign_directory_is_skipped_whole() {
    let (_temporary, root) = documents_tree();
    let archive = dir(&root, "archive");
    file(&archive, "5-draft-old-notes-i9.md");
    file(&archive, "OVERVIEW.md");

    let walked = read(&root).expect("a foreign directory is not this consumer's problem");
    assert!(
        !walked.iter().any(|name| name.contains("old-notes")),
        "nothing under a disclaimed directory is visited: {walked:?}"
    );
    assert_eq!(walked.len(), 7, "the document's tree, unchanged");
}

/// Discharges `structure.als`'s `Operable` and the invariant *no recognised name
/// is silently skipped*: a name this domain recognises and cannot parse halts,
/// and it halts **as a directory**, which is the case that makes the rule worth
/// having. The sibling of the previous test: one filename apart, and the whole
/// subtree's fate turns on it.
#[test]
fn a_malformed_directory_halts_rather_than_vanishing() {
    let (_temporary, root) = documents_tree();
    // This domain's own shape, spelled a way its grammar does not write. Not
    // foreign — foreign would take the subtree with it.
    let broken = dir(&root, "5-topology-i7");
    file(&broken, "01-draft-surfaces-i8.md");

    let Err(error) = read(&root) else {
        panic!("a name this domain recognises and cannot parse must halt");
    };
    let Error::Malformed { path, .. } = &error else {
        panic!("wrong refusal: {error:?}");
    };
    assert_eq!(path, &broken);
    assert!(
        error.to_string().contains("05-topology-i7"),
        "the refusal carries the consumer's recovery advice: {error}"
    );
}

/// Discharges `operations.qnt`'s `wit_haltedUnparseable` and the `unparseable`
/// instance's whole point: snapshot scope is the **whole tree**, so a broken
/// name in a far corner freezes everything rather than only its own level.
#[test]
fn a_malformed_name_anywhere_halts_the_whole_tree() {
    let (_temporary, root) = documents_tree();
    let algebra = root.join("02-linear-algebra-i2");
    let buried = dir(&algebra, "03-vector-spaces-i7");
    file(&buried, "01--i8.md");

    let Err(Error::Malformed { path, .. }) = read(&root) else {
        panic!("a broken name two levels down halts the whole tree");
    };
    assert_eq!(path, buried.join("01--i8.md"));
}

/// Discharges no model claim about the *name* — `Reserved` is a domain's own
/// affair — but does discharge the halt: `operations.qnt` merges `Malformed` and
/// `Reserved` into one `Broken` kind precisely because they differ only in the
/// advice they carry.
#[test]
fn a_reserved_name_halts_and_says_what_to_do() {
    let (_temporary, root) = documents_tree();
    file(&root, "PUBLISHING");

    let Err(error) = read(&root) else {
        panic!("a reserved name halts");
    };
    let Error::Reserved { path, .. } = &error else {
        panic!("wrong refusal: {error:?}");
    };
    assert_eq!(path, &root.join("PUBLISHING"));
    assert!(
        error.to_string().contains("delete"),
        "detection alone is useless to whoever hit it: {error}"
    );
}

/// Discharges `SpeciesAgreementHoldsWhenParsed`, whose predicate is that a name
/// declaring one species over an object of another is **`Malformed`** — not
/// merely *not an entry*. `seam-k17` found a test citing this claim and checking
/// the weaker property, so the assertion here is on the verdict itself, in both
/// directions.
#[test]
fn a_species_mismatch_is_malformed_in_both_directions() {
    // A name declaring a node, over a regular file.
    let temporary = TempDir::new().expect("a temporary directory");
    let root = dir(temporary.path(), "syllabus");
    file(&root, "01-linear-algebra-i1");
    let Err(Error::Malformed { path, .. }) = read(&root) else {
        panic!("a module's name over a file must be malformed");
    };
    assert_eq!(path, root.join("01-linear-algebra-i1"));

    // A name declaring a leaf, over a directory — the one that would hide a
    // subtree if it were merely skipped.
    let temporary = TempDir::new().expect("a temporary directory");
    let root = dir(temporary.path(), "syllabus");
    let posing = dir(&root, "01-draft-vectors-i1.md");
    file(&posing, "02-draft-hidden-i2.md");
    let Err(Error::Malformed { path, .. }) = read(&root) else {
        panic!("a lesson's name over a directory must be malformed");
    };
    assert_eq!(path, posing);
}

/// Discharges the obligation *`parse` refuses what `found` contradicts*, on the
/// case the architecture document names under *Refusals*: a symbolic link
/// wearing an entry's name is **malformed and not occupying**, because the
/// classification is made on what the listing reported, unfollowed, and the halt
/// therefore happens at the snapshot — before any destination is computed. That
/// is a snapshot-layer property even though it only pays off in the mutation
/// leaves.
#[test]
fn a_symbolic_link_wearing_an_entrys_name_is_malformed() {
    let (_temporary, root) = documents_tree();
    let link = root.join("04-draft-shortcut-i10.md");
    std::os::unix::fs::symlink(root.join("01-published-orientation-i1.md"), &link)
        .expect("creating a symbolic link");

    let Err(Error::Malformed { path, .. }) = read(&root) else {
        panic!("a link is neither a regular file nor a directory, whatever it points at");
    };
    assert_eq!(path, link);
}

/// Discharges no model claim, and **cannot**: both models hold no strings by
/// design, so a filename that is not a string at all is outside anything either
/// can state. `parse` takes a `&str`, so the library has no way to ask the
/// domain about such a name and no domain error to carry — it halts with its
/// own. Skipping it is what the whole trichotomy exists to prevent: one mangled
/// byte in a real name produces exactly this.
///
/// # This test does different work on different filesystems, and says which
///
/// APFS validates filenames as UTF-8 and refuses to create this one, so on a
/// stock macOS checkout the halting branch is **unreachable** and there is
/// nothing to observe. Rather than skip — a skipped test reports what a passing
/// one reports, which is this workstream's recurring instrument failure — the
/// test asserts whichever fact is true here: that the filesystem refused the
/// name, or that the library halted on it. Only the second can pass while the
/// library is wrong, and the first cannot pass at all if the filesystem starts
/// accepting such names.
#[test]
fn a_filename_that_is_not_utf8_halts() {
    let (_temporary, root) = documents_tree();
    let mangled = root.join(std::ffi::OsStr::from_bytes(b"04-published-caf\xe9-i10.md"));

    let Err(refused) = fs::write(&mangled, "") else {
        let Err(Error::NonUtf8Name { path }) = read(&root) else {
            panic!("a name that cannot be read cannot be disclaimed either");
        };
        assert_eq!(path, mangled);
        return;
    };
    // The filesystem under this checkout will not hold such a name, so the
    // branch above cannot be reached here. `Uncategorized` is what `std` maps
    // EILSEQ to; the raw code is the load-bearing half.
    assert_eq!(
        refused.raw_os_error(),
        Some(libc_eilseq()),
        "the only acceptable reason for not testing the halt is that the \
         filesystem refuses to store the name at all: {refused:?}"
    );
    assert!(
        !mangled.exists(),
        "and it really did refuse — nothing was created"
    );
}

/// `EILSEQ`, without a dependency in the dev graph to spell it: it is 92 on
/// Linux and 92 on Apple platforms, and the assertion above is the only place
/// this crate needs the value.
fn libc_eilseq() -> i32 {
    92
}

/// Discharges no model claim. The root brief's pointer: paths are deliberately
/// never canonicalised, so what a read verb reports is the caller's own
/// spelling. The `..` route is the same tree by a different name, and it reads
/// the same tree while reporting itself the way it was asked.
#[test]
fn the_root_path_comes_back_the_way_it_went_in() {
    let (_temporary, root) = documents_tree();
    let roundabout = root.join("02-linear-algebra-i2").join("..");

    let tree = ordinal_fs_tree::fs::read::<SyllabusName>(&roundabout)
        .expect("the same tree")
        .expect_tree("a tree, not a vacancy");
    assert_eq!(
        tree.root(),
        roundabout,
        "canonicalising here would make merely adding a lock rewrite every path"
    );
    assert_eq!(tree.walk().count(), 7, "and it is the same tree");
}

/// Discharges no model claim — the models do not reach the filesystem, and
/// concurrency is out of scope in both. `ARCHITECTURE.md` puts the lock on the
/// directory *containing* the root, and this is what that means in practice: a
/// second tree beside the first waits on the first. The containing directory is
/// chosen because it exists before the root is created and persists after it is
/// deleted, so a tree's creation and destruction fall under the same lock as
/// every ordinary operation — and nothing else can express that.
#[test]
fn the_lock_covers_the_directory_containing_the_root() {
    let temporary = TempDir::new().expect("a temporary directory");
    let alpha = dir(temporary.path(), "alpha");
    let beta = dir(temporary.path(), "beta");

    let held = ordinal_fs_tree::fs::write::<SyllabusName>(&alpha)
        .expect("an empty tree is a tree")
        .expect_tree("a tree, not a vacancy");
    let (arrived, waiting) = mpsc::channel();
    let reader = thread::spawn(move || {
        let tree = ordinal_fs_tree::fs::read::<SyllabusName>(&beta)
            .expect("an empty tree")
            .expect_tree("a tree, not a vacancy");
        arrived
            .send(tree.walk().count())
            .expect("the test is waiting");
    });

    assert!(
        waiting.recv_timeout(A_WHILE).is_err(),
        "a writer on one tree holds the lock covering its neighbour"
    );
    drop(held);
    assert_eq!(
        waiting
            .recv_timeout(SOON)
            .expect("the reader proceeds once the writer lets go"),
        0
    );
    reader.join().expect("the reader thread");
}

/// Discharges no model claim — the models do not reach the filesystem. This is
/// `reading-k19`'s High finding, as a test: the lock has to name the **tree**,
/// not a spelling of it. The suite already asserts that
/// `syllabus/02-linear-algebra-i2/..` reads the same seven entries as
/// `syllabus`, so a writer holding one must exclude a reader arriving by the
/// other. Under a lexical `Path::parent` it did not — that spelling locked the
/// module directory — and this is the assertion that fails when it comes back.
#[test]
fn a_roundabout_spelling_of_one_tree_waits_on_the_direct_one() {
    let (_temporary, root) = documents_tree();
    let roundabout = root.join("02-linear-algebra-i2").join("..");

    let held = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy");
    let (arrived, waiting) = mpsc::channel();
    let reader = thread::spawn(move || {
        let tree = ordinal_fs_tree::fs::read::<SyllabusName>(&roundabout)
            .expect("the same tree")
            .expect_tree("a tree, not a vacancy");
        arrived
            .send(tree.root().to_path_buf())
            .expect("the test is waiting");
    });

    assert!(
        waiting.recv_timeout(A_WHILE).is_err(),
        "two spellings of one tree must contend on one lock"
    );
    drop(held);
    let reported = waiting
        .recv_timeout(SOON)
        .expect("the reader proceeds once the writer lets go");
    assert_eq!(
        reported,
        root.join("02-linear-algebra-i2").join(".."),
        "and the lock converging must not cost the caller its own spelling"
    );
    reader.join().expect("the reader thread");
}

/// Discharges no model claim. The other shape of the same finding: a symbolic
/// link naming the root. `<root>/..` makes the kernel follow the link and then
/// step to the directory that really contains the tree, so the link and its
/// target reach one inode — where a lexical parent would have locked whatever
/// directory the link itself happens to sit in.
#[test]
fn a_symlinked_root_waits_on_its_target() {
    let (temporary, root) = documents_tree();
    // Deliberately in a *different* directory from the target, so a lexical
    // parent could not accidentally agree.
    let elsewhere = dir(temporary.path(), "elsewhere");
    let link = elsewhere.join("syllabus");
    std::os::unix::fs::symlink(&root, &link).expect("creating a symbolic link");

    let held = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy");
    let (arrived, waiting) = mpsc::channel();
    let reader = thread::spawn(move || {
        let tree = ordinal_fs_tree::fs::read::<SyllabusName>(&link)
            .expect("the same tree")
            .expect_tree("a tree, not a vacancy");
        arrived
            .send(tree.walk().count())
            .expect("the test is waiting");
    });

    assert!(
        waiting.recv_timeout(A_WHILE).is_err(),
        "a link and its target are one tree and take one lock"
    );
    drop(held);
    assert_eq!(
        waiting.recv_timeout(SOON).expect("the reader proceeds"),
        7,
        "and it really is the same tree"
    );
    reader.join().expect("the reader thread");
}

/// Discharges no model claim. A shared lock is shared: two readers hold it at
/// once. Written with a thread and a timeout rather than two guards in a row
/// because the failure mode is a *hang*, and a test that hangs reports nothing.
#[test]
fn two_readers_share_the_tree() {
    let (_temporary, root) = documents_tree();
    let held = ordinal_fs_tree::fs::read::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy");
    let elsewhere = root.clone();
    let (arrived, waiting) = mpsc::channel();
    let reader = thread::spawn(move || {
        let tree = ordinal_fs_tree::fs::read::<SyllabusName>(&elsewhere)
            .expect("a well-formed tree")
            .expect_tree("a tree, not a vacancy");
        arrived
            .send(tree.walk().count())
            .expect("the test is waiting");
    });

    assert_eq!(
        waiting
            .recv_timeout(SOON)
            .expect("a second reader must not wait on the first"),
        7
    );
    reader.join().expect("the reader thread");
    drop(held);
}

/// Discharges no model claim. The exclusive guard excludes, and — the half worth
/// testing separately — dropping it is the whole of releasing it. There is no
/// unlock call, so there is no unlock path to get wrong.
#[test]
fn a_writer_excludes_a_reader_until_it_is_dropped() {
    let (_temporary, root) = documents_tree();
    let held = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy");
    assert_eq!(
        held.by_key(Key::new(2)).map(|e| e.species()),
        Sought::Match(Species::Node),
        "the exclusive guard reads the tree exactly as the shared one does"
    );

    let elsewhere = root.clone();
    let (arrived, waiting) = mpsc::channel();
    let reader = thread::spawn(move || {
        let tree = ordinal_fs_tree::fs::read::<SyllabusName>(&elsewhere)
            .expect("a well-formed tree")
            .expect_tree("a tree, not a vacancy");
        arrived
            .send(tree.walk().count())
            .expect("the test is waiting");
    });

    assert!(
        waiting.recv_timeout(A_WHILE).is_err(),
        "a reader must wait for the writer"
    );
    drop(held);
    assert_eq!(
        waiting
            .recv_timeout(SOON)
            .expect("and proceed once it is gone"),
        7
    );
    reader.join().expect("the reader thread");
}

/// Discharges no model claim. A filesystem root has no containing directory, so
/// there is nothing to lock and the library says so rather than locking
/// somewhere else. Refused before anything is opened or read.
#[test]
fn a_root_with_no_containing_directory_is_refused() {
    let Err(Error::NoContainingDirectory { root }) =
        ordinal_fs_tree::fs::read::<SyllabusName>(Path::new("/"))
    else {
        panic!("a filesystem root cannot be a tree root");
    };
    assert_eq!(root, Path::new("/"));
}
