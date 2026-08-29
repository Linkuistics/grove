//! `append` and `append_many` through the public surface: a real directory, an
//! exclusive lock, and a report.
//!
//! What is *not* here is atomicity, and deliberately: the seam that makes an
//! effect fail is internal — a second public seam would contradict
//! `docs/adr/entry-name-is-the-only-seam.md` — so those tests are unit tests,
//! beside the interpreter in `src/fs/apply/tests.rs`. What is here is everything
//! a consumer can reach: the entries that appear, the bytes in them, the report
//! that comes back, and the refusals.
//!
//! Every test here names the model claim it discharges, or says it has none.

use std::fs;
use std::path::{Path, PathBuf};

use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusName};
use ordinal_fs_tree::{Error, Key, NewEntry, Refusal, Sought, Species, Target};
use tempfile::TempDir;

fn file(at: &Path, name: &str) {
    fs::write(at.join(name), "").expect("writing a fixture file");
}

fn dir(at: &Path, name: &str) -> PathBuf {
    let path = at.join(name);
    fs::create_dir(&path).expect("creating a fixture directory");
    path
}

fn draft(label: &str) -> Parts {
    Parts::lesson(
        Status::Draft,
        Label::new(label).expect("a well-formed label"),
    )
}

fn topic(label: &str) -> Parts {
    Parts::module(Label::new(label).expect("a well-formed label"))
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

fn walk(root: &Path) -> Vec<String> {
    let tree = ordinal_fs_tree::fs::read::<SyllabusName>(root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy");
    tree.walk().map(|e| e.name().to_string()).collect()
}

/// Discharges `inv_appendOnlyAdds` end to end: the entry lands at the level's
/// next ordinal with the tree's next key, its bytes are written verbatim, and
/// the tree reads back with everything else where it was.
#[test]
fn an_appended_leaf_appears_with_its_bytes() {
    let (_temporary, root) = documents_tree();
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .append(
            Target::Root,
            NewEntry::new(draft("conclusion"), b"# Conclusion\n".to_vec()),
        )
        .expect("a clean run");

    let created = &report.created()[0];
    assert_eq!(created.name.to_string(), "04-draft-conclusion-i10.md");
    assert_eq!(created.path, root.join("04-draft-conclusion-i10.md"));
    assert_eq!(
        fs::read_to_string(&created.path).expect("the new leaf"),
        "# Conclusion\n"
    );
    assert_eq!(
        walk(&root),
        [
            "OVERVIEW.md",
            "01-published-orientation-i1.md",
            "02-linear-algebra-i2",
            "OVERVIEW.md",
            "01-published-vectors-i5.md",
            "02-draft-matrices-i6.md",
            "03-draft-assessment-i9.md",
            "04-draft-conclusion-i10.md",
        ]
    );
}

/// Discharges no model claim of its own — the model holds no filesystem — but it
/// is what *the species follows from the parts* means once the interpreter has
/// it: parts that make a module make a **directory**, and the library was never
/// told which to make.
#[test]
fn an_appended_node_is_a_directory() {
    let (_temporary, root) = documents_tree();
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .append(Target::Root, NewEntry::empty(topic("topology")))
        .expect("a clean run");

    let created = &report.created()[0];
    assert_eq!(created.name.to_string(), "04-topology-i10");
    assert!(
        created.path.is_dir(),
        "a module is a node, and a node is a directory"
    );
    assert_eq!(
        ordinal_fs_tree::fs::read::<SyllabusName>(&root)
            .expect("a well-formed tree")
            .expect_tree("a tree, not a vacancy")
            .by_key(Key::new(10))
            .map(|entry| entry.species()),
        Sought::Match(Species::Node)
    );
}

/// Discharges `RefusedTargetNotNode`'s complement on disk: a node named by key
/// is a level, and the entry lands inside it at *its* next ordinal — 3, not the
/// root's 4 — while the key still steps past every key in the tree.
#[test]
fn an_append_into_a_node_lands_inside_it() {
    let (_temporary, root) = documents_tree();
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .append(
            Target::Key(Key::new(2)),
            NewEntry::new(draft("eigenvalues"), b"lambda".to_vec()),
        )
        .expect("a clean run");

    assert_eq!(
        report.created()[0].path,
        root.join("02-linear-algebra-i2")
            .join("03-draft-eigenvalues-i10.md")
    );
}

/// Discharges `wit_appendManySucceeded`: *several children at consecutive
/// ordinals with consecutive keys, planned from one snapshot and applied as a
/// unit*.
#[test]
fn a_run_of_appends_lands_whole_and_consecutive() {
    let (_temporary, root) = documents_tree();
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .append_many(
            Target::Root,
            vec![
                NewEntry::empty(draft("one")),
                NewEntry::empty(draft("two")),
                NewEntry::empty(draft("three")),
            ],
        )
        .expect("a clean run");

    let names: Vec<String> = report
        .created()
        .iter()
        .map(|created| created.name.to_string())
        .collect();
    assert_eq!(
        names,
        [
            "04-draft-one-i10.md",
            "05-draft-two-i11.md",
            "06-draft-three-i12.md"
        ]
    );
    assert!(report.paths().all(Path::exists));
}

/// Discharges no model claim. An empty run succeeds and changes nothing —
/// stated on disk because *changes nothing* is a claim about a directory.
#[test]
fn a_run_of_nothing_changes_nothing() {
    let (_temporary, root) = documents_tree();
    let before = walk(&root);
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .append_many(Target::Root, Vec::new())
        .expect("a plan of no effects");
    assert!(report.created().is_empty());
    assert_eq!(walk(&root), before);
}

/// Discharges `wit_refusedTargetNotNode`: *a leaf is a regular file and holds
/// nothing*. The assertion is on the refusal **and** on its advice, because
/// detection alone produces errors that are useless to whoever hits them.
#[test]
fn appending_into_a_leaf_is_refused_with_advice() {
    let (_temporary, root) = documents_tree();
    let refused = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .append(Target::Key(Key::new(1)), NewEntry::empty(draft("inside")))
        .expect_err("a lesson is a leaf");

    let Error::Refused(Refusal::TargetNotNode { key, species }) = &refused else {
        panic!("wrong refusal: {refused:?}");
    };
    assert_eq!((*key, *species), (Key::new(1), Species::Leaf));
    assert!(
        refused.to_string().contains("promote it first"),
        "a refusal says what to do about it: {refused}"
    );
    assert_eq!(walk(&root).len(), 7, "and nothing was created");
}

/// Discharges `wit_refusedTargetMissing`: *a key naming no entry is refused*.
#[test]
fn appending_into_a_key_that_names_nothing_is_refused() {
    let (_temporary, root) = documents_tree();
    let refused = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .append(Target::Key(Key::new(99)), NewEntry::empty(draft("nowhere")))
        .expect_err("no entry has key 99");
    assert!(matches!(
        refused,
        Error::Refused(Refusal::TargetMissing { .. })
    ));
}

/// Discharges no model claim, and cannot: content is unmodelled by design, so
/// this refusal is the library's own. A directory has nowhere to hold bytes, and
/// the alternative to refusing is discarding them silently.
#[test]
fn bytes_for_a_node_are_refused() {
    let (_temporary, root) = documents_tree();
    let refused = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .append(
            Target::Root,
            NewEntry::new(topic("topology"), b"where would this go?".to_vec()),
        )
        .expect_err("a directory holds no bytes");
    assert!(matches!(refused, Error::Refused(Refusal::ContentForANode)));
}

/// Discharges `wit_haltedUnparseable` on a **mutation**: snapshot scope is the
/// whole tree, so a name the consumer recognises and cannot parse — two levels
/// away from anything this append would touch — freezes the tree. The halt
/// happens when the guard is taken, before a plan exists at all, which is why
/// this test never reaches `append`.
#[test]
fn a_broken_name_anywhere_freezes_every_mutation() {
    let (_temporary, root) = documents_tree();
    let broken = dir(&root.join("02-linear-algebra-i2"), "5-topology-i7");
    file(&broken, "01-draft-surfaces-i8.md");

    let Err(Error::Malformed { path, .. }) = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
    else {
        panic!("a mutation cannot proceed past a name the consumer cannot parse");
    };
    assert_eq!(path, broken);
}

/// Discharges no model claim. The paths a report carries are built from the
/// caller's own spelling of the root, because nothing in this crate
/// canonicalises anything — on macOS `/var` and `/private/var` name one inode,
/// so canonicalising would make the mere presence of a lock rewrite every path
/// the library returns.
#[test]
fn the_reported_paths_keep_the_callers_spelling() {
    let (_temporary, root) = documents_tree();
    let roundabout = root.join("02-linear-algebra-i2").join("..");
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&roundabout)
        .expect("the same tree")
        .expect_tree("a tree, not a vacancy")
        .append(Target::Root, NewEntry::empty(draft("conclusion")))
        .expect("a clean run");

    assert_eq!(
        report.created()[0].path,
        roundabout.join("04-draft-conclusion-i10.md")
    );
    assert!(root.join("04-draft-conclusion-i10.md").exists());
}
