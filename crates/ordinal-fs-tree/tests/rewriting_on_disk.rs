//! `rewrite` through the public surface: a real directory, an exclusive lock,
//! one real `rename(2)`, and a report.
//!
//! The smallest mutation there is, and three things are observable here that no
//! algebra test can see. The first is that the entry's **bytes** are untouched —
//! content is unmodelled in both models by design, so no claim reaches it and
//! the only instrument that can is a file with something in it. The second is
//! that a rewritten **node** carries its whole subtree: `operations.qnt`'s
//! handoff block names that an *assumption* rather than a checked property,
//! because entries reference their parent by a stable id there, so a directory
//! is the only instrument for it. The third is the no-op — a rename onto the
//! entry's own path, which the algebra declines to refuse and the interpreter
//! declines to perform.
//!
//! Every test here names the model claim it discharges, or says it has none.

use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusName};
use ordinal_fs_tree::{Error, Key, PositionedSpecies, Refusal};
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

fn published(label: &str) -> Parts {
    Parts::lesson(
        Status::Published,
        Label::new(label).expect("a well-formed label"),
    )
}

fn topic(label: &str) -> Parts {
    Parts::module(Label::new(label).expect("a well-formed label"))
}

/// `ARCHITECTURE.md`'s own example tree, on disk, with every leaf carrying bytes
/// that name it.
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
    let tree = ordinal_fs_tree::fs::read::<SyllabusName>(root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy");
    tree.walk().map(|e| e.name().to_string()).collect()
}

fn refusal(error: Error<SyllabusName>) -> Refusal {
    match error {
        Error::Refused(refusal) => refusal,
        other => panic!("expected a refusal, got {other}"),
    }
}

/// The whole tree of names, in walk order, unchanged by the operations that
/// refuse.
fn documents_walk() -> Vec<&'static str> {
    vec![
        "OVERVIEW.md",
        "01-published-orientation-i1.md",
        "02-linear-algebra-i2",
        "OVERVIEW.md",
        "01-published-vectors-i5.md",
        "02-draft-matrices-i6.md",
        "03-draft-assessment-i9.md",
    ]
}

/// Discharges `inv_rewriteKeepsPlace` end to end, and the half no model reaches:
/// **the file is the same file**.
///
/// The draft assessment at ordinal 3, key 9 becomes published. It keeps its
/// ordinal, its key and its place in walk order, and only the attribute token in
/// its filename moved. The inode is the instrument for the rest: a rename does
/// not copy, so an unchanged inode says the library did not read the bytes and
/// write them back — which content equality alone would not, since bytes written
/// back identically compare equal.
#[test]
fn a_rewritten_lesson_keeps_its_place_and_stays_the_same_file() {
    let (_temporary, root) = documents_tree();
    let was = fs::metadata(root.join("03-draft-assessment-i9.md"))
        .expect("the draft assessment")
        .ino();

    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .rewrite(Key::new(9), published("assessment"))
        .expect("a clean run");

    assert_eq!(
        walk(&root),
        [
            "OVERVIEW.md",
            "01-published-orientation-i1.md",
            "02-linear-algebra-i2",
            "OVERVIEW.md",
            "01-published-vectors-i5.md",
            "02-draft-matrices-i6.md",
            "03-published-assessment-i9.md",
        ],
        "the ordinal and the key are the ones it had, so nothing else moved and \
         it did not change place"
    );
    let now = root.join("03-published-assessment-i9.md");
    assert_eq!(
        fs::read_to_string(&now).expect("the rewritten lesson"),
        "assessment",
        "the bytes are what they were"
    );
    assert_eq!(
        fs::metadata(&now).expect("the rewritten lesson").ino(),
        was,
        "and it is the same file: one rename, and nothing read or rewrote it"
    );
    assert!(
        !root.join("03-draft-assessment-i9.md").exists(),
        "a consumer holding a path is stale, which is the whole reason the key \
         exists"
    );

    assert!(report.created().is_empty(), "a rewrite creates nothing");
    assert_eq!(report.renamed().len(), 1);
    assert_eq!(
        report.renamed()[0].from,
        root.join("03-draft-assessment-i9.md")
    );
    assert_eq!(report.renamed()[0].to, now);
    assert_eq!(
        report.paths().collect::<Vec<_>>(),
        [now.as_path()],
        "one effect, so the plan's landing order is one path"
    );
}

/// **No model claim, and none possible.** `operations.qnt`'s handoff block names
/// this an *assumption*: an entry there references its parent by a stable id, so
/// a directory rename carrying its subtree is true by construction and checked
/// nowhere. A real directory is the only instrument.
///
/// It matters more for `rewrite` than for `insert`'s shift, which has the same
/// property and its own test: a shift is `compose` with a new ordinal and
/// therefore cannot disturb anything, while a rewrite is handed **new parts** by
/// a caller — so the thing being renamed is the one whose contents a reader
/// might expect to be rebuilt.
#[test]
fn a_rewritten_node_carries_its_whole_subtree_untouched() {
    let (_temporary, root) = documents_tree();
    let was = fs::metadata(root.join("02-linear-algebra-i2/01-published-vectors-i5.md"))
        .expect("a lesson inside the module")
        .ino();

    ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .rewrite(Key::new(2), topic("linear-algebra-revised"))
        .expect("a clean run");

    let module = root.join("02-linear-algebra-revised-i2");
    assert!(
        module.is_dir(),
        "still a directory: the species did not change"
    );
    assert_eq!(
        walk(&root),
        [
            "OVERVIEW.md",
            "01-published-orientation-i1.md",
            "02-linear-algebra-revised-i2",
            "OVERVIEW.md",
            "01-published-vectors-i5.md",
            "02-draft-matrices-i6.md",
            "03-draft-assessment-i9.md",
        ],
        "and every child is where it was, at the ordinals and keys it had"
    );
    assert_eq!(
        fs::read_to_string(module.join("01-published-vectors-i5.md")).expect("the lesson"),
        "vectors"
    );
    assert_eq!(
        fs::metadata(module.join("01-published-vectors-i5.md"))
            .expect("the lesson")
            .ino(),
        was,
        "one rename of the directory, and nothing inside it was touched at all"
    );
}

/// Discharges `wit_rewriteToSameParts` end to end — the half the algebra cannot
/// show.
///
/// Two mechanisms buy this one property and each can fail alone. The algebra
/// excludes the object being moved from occupancy, so the plan is not refused as
/// a collision with itself; the interpreter then short-circuits a rename whose
/// destination is its source, because `rename(2)` on one path is defined to
/// change nothing and an `Undo::Restore` for it would rename onto its own
/// occupied path and turn a clean rollback into `FailedPartiallyRolledBack`.
///
/// It is still **reported**: the operation did place this name, and a consumer
/// reading `renamed()` to learn where an entry lives needs the answer whether or
/// not the filesystem was touched. `from` and `to` being one path is how the
/// report says the tree did not move.
#[test]
fn rewriting_to_the_parts_an_entry_already_carries_succeeds_and_changes_nothing() {
    let (_temporary, root) = documents_tree();
    let path = root.join("03-draft-assessment-i9.md");
    let was = fs::metadata(&path).expect("the draft assessment").ino();

    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .rewrite(Key::new(9), draft("assessment"))
        .expect("a no-op is a clean run, not a collision with itself");

    assert_eq!(walk(&root), documents_walk(), "nothing moved");
    assert_eq!(
        fs::metadata(&path).expect("the draft assessment").ino(),
        was,
        "and nothing was replaced either"
    );
    assert_eq!(report.renamed().len(), 1, "it is still reported");
    assert_eq!(
        report.renamed()[0].from,
        report.renamed()[0].to,
        "and the report says the rename went nowhere by naming one path twice"
    );
}

/// Discharges `wit_refusedRewriteSpeciesChange` through the public surface, in
/// the direction a consumer is likeliest to try: turning a lesson into a module.
///
/// A refusal changes nothing, which is the half worth asserting on disk — the
/// algebra decides before the interpreter runs, so there is no partial state to
/// roll back. The advice names `promote`, because that operation exists and it
/// moves the leaf's content into the new node rather than discarding it.
#[test]
fn rewriting_a_lesson_into_a_module_is_refused_and_changes_nothing() {
    let (_temporary, root) = documents_tree();

    let error = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .rewrite(Key::new(9), topic("assessment"))
        .expect_err("a file cannot be renamed into a directory");
    let refused = refusal(error);

    assert_eq!(
        refused,
        Refusal::RewriteSpeciesChange {
            key: Key::new(9),
            species: PositionedSpecies::Leaf,
        }
    );
    assert!(
        refused.to_string().contains("`promote`"),
        "a refusal says what to do, and this direction has somewhere to go: \
         {refused}"
    );
    assert_eq!(walk(&root), documents_walk(), "and nothing was touched");
}

/// Discharges `wit_refusedRewriteSpeciesChange` in the other direction, which is
/// the same model outcome and a **different message**.
///
/// There is no operation that turns a node into a leaf: its children would have
/// nowhere to go, and `docs/adr/entries-are-never-removed.md` is why they cannot
/// simply be dropped. So the advice cannot name a remedy, and offering `promote`
/// here would be advice that fails when taken.
#[test]
fn rewriting_a_module_into_a_lesson_is_refused_with_no_remedy_to_offer() {
    let (_temporary, root) = documents_tree();

    let error = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .rewrite(Key::new(2), draft("linear-algebra"))
        .expect_err("a directory cannot be renamed into a file");
    let refused = refusal(error);

    assert_eq!(
        refused,
        Refusal::RewriteSpeciesChange {
            key: Key::new(2),
            species: PositionedSpecies::Node,
        }
    );
    assert!(
        refused.to_string().contains("nowhere to go"),
        "so it says why instead of offering a remedy: {refused}"
    );
    assert_eq!(walk(&root), documents_walk(), "and nothing was touched");
}

/// Discharges `wit_refusedTargetMissing` on this operation. The key is the one
/// handle the design promises survives, so naming one that has never existed is
/// the caller's error and is reported as such.
#[test]
fn rewriting_a_key_that_names_nothing_is_refused_and_changes_nothing() {
    let (_temporary, root) = documents_tree();

    let error = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .expect_tree("a tree, not a vacancy")
        .rewrite(Key::new(404), draft("nothing"))
        .expect_err("no entry carries that key");

    assert_eq!(
        refusal(error),
        Refusal::TargetMissing { key: Key::new(404) }
    );
    assert_eq!(walk(&root), documents_walk());
}
