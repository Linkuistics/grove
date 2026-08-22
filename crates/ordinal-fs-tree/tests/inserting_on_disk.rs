//! `insert` through the public surface: a real directory, an exclusive lock, a
//! real `rename(2)` per shifted sibling, and a report.
//!
//! This is where the **assumed** half of *subtree preservation under shift*
//! becomes observable. `operations.qnt` makes it true by construction — entries
//! reference their parent by a stable id — and `docs/formalism-findings.md`
//! entry 003's first miss warns that a model satisfying an invariant by
//! construction looks exactly like one that verified it. No model reaches below
//! the interpreter, so the only thing that can hold the library to *a shifted
//! node is one directory rename, with nothing inside it touched* is a directory
//! with something inside it.
//!
//! Every test here names the model claim it discharges, or says it has none.

use std::fs;
use std::path::{Path, PathBuf};

use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusName};
use ordinal_fs_tree::{Error, Key, NewEntry, Ordinal, Refusal, Target};
use tempfile::TempDir;

fn file(at: &Path, name: &str, content: &str) {
    fs::write(at.join(name), content).expect("writing a fixture file");
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

/// `ARCHITECTURE.md`'s own example tree, on disk, with every leaf carrying bytes
/// that name it — so a shift that disturbed one would be visible rather than
/// merely absent.
fn documents_tree() -> (TempDir, PathBuf) {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = dir(temporary.path(), "syllabus");
    file(&root, "OVERVIEW.md", "the syllabus");
    file(&root, "01-published-orientation-i1.md", "orientation");
    let algebra = dir(&root, "02-linear-algebra-i2");
    file(&algebra, "OVERVIEW.md", "linear algebra");
    file(&algebra, "01-published-vectors-i5.md", "vectors");
    file(&algebra, "02-draft-matrices-i6.md", "matrices");
    file(&root, "03-draft-assessment-i9.md", "assessment");
    (temporary, root)
}

fn walk(root: &Path) -> Vec<String> {
    let tree = ordinal_fs_tree::fs::read::<SyllabusName>(root).expect("a well-formed tree");
    tree.walk().map(|e| e.name().to_string()).collect()
}

fn refusal(error: Error<SyllabusName>) -> Refusal {
    match error {
        Error::Refused(refusal) => refusal,
        other => panic!("expected a refusal, got {other}"),
    }
}

/// Discharges `inv_insertOnlyShifts` end to end: the occupant and every later
/// sibling move up by one, the new entry lands on the vacated ordinal with the
/// tree's fresh key and its bytes, and everything before the target is where it
/// was.
#[test]
fn an_inserted_leaf_shifts_the_occupant_and_every_later_sibling() {
    let (_temporary, root) = documents_tree();
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .insert(
            Target::Root,
            Ordinal::new(2),
            NewEntry::new(draft("interlude"), b"between\n".to_vec()),
        )
        .expect("a clean run");

    assert_eq!(
        walk(&root),
        [
            "OVERVIEW.md",
            "01-published-orientation-i1.md",
            "02-draft-interlude-i10.md",
            "03-linear-algebra-i2",
            "OVERVIEW.md",
            "01-published-vectors-i5.md",
            "02-draft-matrices-i6.md",
            "04-draft-assessment-i9.md",
        ]
    );
    let created = &report.created()[0];
    assert_eq!(created.path, root.join("02-draft-interlude-i10.md"));
    assert_eq!(
        fs::read_to_string(&created.path).expect("the new leaf"),
        "between\n"
    );
}

/// Discharges the assumed half of *subtree preservation under shift*, which no
/// model reaches: **a shifted node is one directory rename**. The module moves
/// from ordinal 2 to 3, and every name, every key and every byte inside it is
/// unchanged — its children keep ordinals 1 and 2, because an ordinal is
/// per-level and says nothing about the path to it.
///
/// A test that rebuilt the subtree, or walked into it to rename its children,
/// would have misread the design; this is what would fail if one ever did.
#[test]
fn a_shifted_node_carries_its_whole_subtree_untouched() {
    let (_temporary, root) = documents_tree();
    ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .insert(
            Target::Root,
            Ordinal::new(2),
            NewEntry::empty(draft("interlude")),
        )
        .expect("a clean run");

    let moved = root.join("03-linear-algebra-i2");
    assert!(moved.is_dir(), "the node moved as a directory");
    let mut inside: Vec<String> = fs::read_dir(&moved)
        .expect("the moved node")
        .map(|entry| {
            entry
                .expect("a directory entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    inside.sort();
    assert_eq!(
        inside,
        [
            "01-published-vectors-i5.md",
            "02-draft-matrices-i6.md",
            "OVERVIEW.md",
        ]
    );
    assert_eq!(
        fs::read_to_string(moved.join("02-draft-matrices-i6.md")).expect("a child"),
        "matrices",
        "nothing inside a shifted node is touched — not a name, not a key, not a byte"
    );
    assert!(
        !root.join("02-linear-algebra-i2").exists(),
        "and it is a rename, not a copy"
    );
}

/// Discharges `shiftIds` under `HIGHEST_FIRST` at the surface a consumer reads:
/// `renamed()` is the renames in the order they ran, which is descending, and
/// that is where the ordering rule is observable from outside the crate.
#[test]
fn the_report_shows_the_renames_highest_ordinal_first() {
    let (_temporary, root) = documents_tree();
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .insert(
            Target::Root,
            Ordinal::new(1),
            NewEntry::empty(draft("preface")),
        )
        .expect("a clean run");

    assert_eq!(
        report
            .renamed()
            .iter()
            .map(|renamed| renamed.name.to_string())
            .collect::<Vec<_>>(),
        [
            "04-draft-assessment-i9.md",
            "03-linear-algebra-i2",
            "02-published-orientation-i1.md",
        ]
    );
    assert_eq!(
        report.renamed()[0].from,
        root.join("03-draft-assessment-i9.md"),
        "and each says where it came from"
    );
}

/// Discharges `Report::paths()`'s own contract, which `interpreter-k22` settled
/// and `insert` is the first operation to make observable: **the plan's own
/// landing order**, which for a mixed plan is neither species' order. Shifts,
/// then the create — a creation-first reading would report the new entry before
/// the shifts that made room for it.
#[test]
fn the_reports_paths_are_in_the_order_the_effects_landed() {
    let (_temporary, root) = documents_tree();
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .insert(
            Target::Root,
            Ordinal::new(2),
            NewEntry::empty(draft("interlude")),
        )
        .expect("a clean run");

    assert_eq!(
        report.paths().collect::<Vec<_>>(),
        [
            root.join("04-draft-assessment-i9.md").as_path(),
            root.join("03-linear-algebra-i2").as_path(),
            root.join("02-draft-interlude-i10.md").as_path(),
        ]
    );
}

/// Discharges `inv_insertOnlyShifts` in a level that is not the root: the target
/// is a **node**, named by key, and the ordinals shifted are that node's own.
/// An ordinal is per-level, which is what makes insertion cheap.
#[test]
fn an_insert_into_a_node_shifts_that_nodes_children() {
    let (_temporary, root) = documents_tree();
    ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .insert(
            Target::Key(Key::new(2)),
            Ordinal::new(1),
            NewEntry::new(draft("notation"), b"notation\n".to_vec()),
        )
        .expect("a clean run");

    assert_eq!(
        walk(&root),
        [
            "OVERVIEW.md",
            "01-published-orientation-i1.md",
            "02-linear-algebra-i2",
            "OVERVIEW.md",
            "01-draft-notation-i10.md",
            "02-published-vectors-i5.md",
            "03-draft-matrices-i6.md",
            "03-draft-assessment-i9.md",
        ],
        "the module's children shifted; the root's did not"
    );
}

/// Discharges no model claim of its own — the model holds no filesystem — but it
/// is what *the species follows from the parts* means for this operation: an
/// inserted module is a **directory**, and the library was never told which to
/// make.
#[test]
fn an_inserted_node_is_a_directory() {
    let (_temporary, root) = documents_tree();
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .insert(
            Target::Root,
            Ordinal::new(3),
            NewEntry::empty(topic("topology")),
        )
        .expect("a clean run");

    assert!(
        report.created()[0].path.is_dir(),
        "a module is a node, and a node is a directory"
    );
    assert_eq!(report.created()[0].name.to_string(), "03-topology-i10");
}

/// Discharges `wit_insertPastTheEnd` at the surface: the refusal reaches the
/// consumer as [`Error::Refused`], the tree is untouched, and the message sends
/// them to `append`.
#[test]
fn inserting_past_the_last_sibling_is_refused_and_changes_nothing() {
    let (_temporary, root) = documents_tree();
    let before = walk(&root);
    let error = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .insert(Target::Root, Ordinal::new(9), NewEntry::empty(draft("far")))
        .expect_err("past the end");

    assert_eq!(
        refusal(error),
        Refusal::NoOccupantAtOrdinal {
            ordinal: Ordinal::new(9),
            occupied: Some((Ordinal::FIRST, Ordinal::new(3))),
        }
    );
    assert_eq!(walk(&root), before, "a refusal changes nothing");
}

/// Discharges `wit_insertIntoAGap`: the same refusal on a hand-edited level,
/// with the advice the other half cannot give. This is the tree a `mv` leaves —
/// which is a thing this design invites, since a directory listing *is* the data
/// structure.
#[test]
fn inserting_into_a_hand_edited_gap_is_refused_with_its_own_advice() {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = dir(temporary.path(), "syllabus");
    file(&root, "01-draft-first-i1.md", "first");
    // What `mv 02-draft-third-i2.md 05-draft-third-i2.md` leaves: ordinals 2, 3
    // and 4 are a gap no operation fills.
    file(&root, "05-draft-third-i2.md", "third");

    let error = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .insert(
            Target::Root,
            Ordinal::new(3),
            NewEntry::empty(draft("wedge")),
        )
        .expect_err("into a gap");

    let refused = refusal(error);
    assert_eq!(
        refused,
        Refusal::NoOccupantAtOrdinal {
            ordinal: Ordinal::new(3),
            occupied: Some((Ordinal::FIRST, Ordinal::new(5))),
        }
    );
    let said = refused.to_string();
    assert!(
        said.contains("gap") && said.contains("by hand"),
        "no operation fills a gap, and the message has to say so: {said}"
    );
    assert!(
        said.contains("something below it"),
        "ordinal 1 is occupied here, so the lower neighbour the message names \
         really is there: {said}"
    );
}

/// **No model claim.** `wit_insertIntoAGap`'s `a.at < maxOrdIn` is true of a
/// hole below every occupant as well as of one between two, so the leading-hole
/// message is the library's own distinction and this is its public-surface
/// control.
///
/// The tree is what `mv 01-… 05-…` leaves: one lesson, at ordinal 5. Asking for
/// ordinal 1 reaches the hole branch with nothing underneath the request, and
/// the refusal must not claim otherwise.
#[test]
fn inserting_below_the_first_occupied_ordinal_is_refused_without_a_lower_neighbour() {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = dir(temporary.path(), "syllabus");
    file(&root, "05-draft-only-i1.md", "only");
    let before = walk(&root);

    let error = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .insert(
            Target::Root,
            Ordinal::FIRST,
            NewEntry::empty(draft("underneath")),
        )
        .expect_err("below the first occupant");

    let refused = refusal(error);
    assert_eq!(
        refused,
        Refusal::NoOccupantAtOrdinal {
            ordinal: Ordinal::FIRST,
            occupied: Some((Ordinal::new(5), Ordinal::new(5))),
        }
    );
    let said = refused.to_string();
    assert!(
        !said.contains("something below it"),
        "nothing occupies an ordinal below 1 here: {said}"
    );
    assert!(
        said.contains("by hand") && !said.contains("`append`'s job"),
        "`append` would take ordinal 6, so the advice is the hole's: {said}"
    );
    assert_eq!(walk(&root), before, "a refusal changes nothing");
}

/// Discharges `wit_refusedTargetNotNode` for `insert`: a leaf is a regular file
/// and holds nothing, so it is not a level to insert into.
#[test]
fn inserting_into_a_leaf_is_refused() {
    let (_temporary, root) = documents_tree();
    let error = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .insert(
            Target::Key(Key::new(1)),
            Ordinal::FIRST,
            NewEntry::empty(draft("inside-a-file")),
        )
        .expect_err("a leaf holds nothing");

    assert!(matches!(refusal(error), Refusal::TargetNotNode { .. }));
}
