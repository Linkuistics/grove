//! `promote` through the public surface: a real directory, an exclusive lock, a
//! `create_dir` and a real `rename(2)`, and a report.
//!
//! This is the operation with the most that can go wrong, and the only one by
//! which the library can damage a tree it was handed. Two things are observable
//! here and nowhere else. The first is that the leaf's **bytes** move — content
//! is unmodelled in both models by design, so no claim reaches it and the only
//! instrument that can is a file with something in it. The second is that a
//! promotion is a *directory* appearing where a regular file was, which no
//! algebra test can see.
//!
//! Every test here names the model claim it discharges, or says it has none.

use core::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusError, SyllabusName};
use ordinal_fs_tree::{
    EntryName, EntryNameExt, Error, Found, Key, NameView, NewEntry, Ordinal, PositionedSpecies,
    Refusal, Triple, Verdict,
};
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
/// that name it — so a promotion that lost or rewrote one would be visible
/// rather than merely absent.
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

/// Discharges `inv_promoteKeepsIdentity` end to end, and the half of it no model
/// reaches: **the leaf's bytes move verbatim**.
///
/// The lesson at ordinal 1, key 1 becomes a module at ordinal 1, key 1 — the
/// same ordinal and the same key, so a consumer holding the key still resolves
/// it — and the file that held "orientation" is now the node's `OVERVIEW.md`,
/// holding "orientation". Nothing read those bytes: the file moved.
#[test]
fn a_promoted_leaf_becomes_a_node_holding_its_own_content() {
    let (_temporary, root) = documents_tree();
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .promote(Key::new(1), topic("orientation"), None)
        .expect("a clean run");

    assert_eq!(
        walk(&root),
        [
            "OVERVIEW.md",
            "01-orientation-i1",
            "OVERVIEW.md",
            "02-linear-algebra-i2",
            "OVERVIEW.md",
            "01-published-vectors-i5.md",
            "02-draft-matrices-i6.md",
            "03-draft-assessment-i9.md",
        ],
        "the leaf's ordinal and key are the node's, so nothing else moved"
    );
    let node = root.join("01-orientation-i1");
    assert!(
        node.is_dir(),
        "a promotion makes a directory where a file was"
    );
    assert_eq!(
        fs::read_to_string(node.join("OVERVIEW.md")).expect("the distinguished child"),
        "orientation",
        "the leaf's bytes moved verbatim, and the library never read them"
    );
    assert!(
        !root.join("01-published-orientation-i1.md").exists(),
        "and the leaf's own path is gone: a consumer holding a path is stale, \
         which is the whole reason the key exists"
    );
    assert_eq!(report.created().len(), 1);
    assert_eq!(report.renamed().len(), 1);
    assert_eq!(
        report.renamed()[0].to,
        node.join("OVERVIEW.md"),
        "and the report says where the content went"
    );
}

/// Discharges `wit_promoteWithChild` end to end, together with
/// `Report::paths()`'s contract: **the plan's own landing order**, which for a
/// promotion with a first child is create, move, create — a sequence no pair of
/// species-sorted buckets can reconstruct.
#[test]
fn a_promotion_with_a_first_child_lands_all_three_in_the_plans_order() {
    let (_temporary, root) = documents_tree();
    let report = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .promote(
            Key::new(1),
            topic("orientation"),
            Some(NewEntry::new(draft("welcome"), b"welcome\n".to_vec())),
        )
        .expect("a clean run");

    let node = root.join("01-orientation-i1");
    assert_eq!(
        report.paths().collect::<Vec<_>>(),
        [
            node.as_path(),
            node.join("OVERVIEW.md").as_path(),
            node.join("01-draft-welcome-i10.md").as_path(),
        ],
        "create, move, create — and `created()` alone would report the child \
         before the content it sits beside"
    );
    assert_eq!(
        fs::read_to_string(node.join("01-draft-welcome-i10.md")).expect("the first child"),
        "welcome\n"
    );
    assert_eq!(
        walk(&root)[1..4],
        [
            "01-orientation-i1",
            "OVERVIEW.md",
            "01-draft-welcome-i10.md"
        ],
        "the distinguished child comes first within the level, then the children \
         by ordinal"
    );
}

/// Discharges `inv_promoteKeepsIdentity`'s *nothing else moved* clause on disk,
/// in a level that is not the root: promoting a lesson inside a module leaves
/// every sibling, every key and every byte in that module alone.
///
/// A promotion is not an insert. Nothing shifts, because the node takes the
/// ordinal the leaf vacates in the same breath.
#[test]
fn a_promotion_shifts_nothing() {
    let (_temporary, root) = documents_tree();
    ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .promote(Key::new(5), topic("vectors"), None)
        .expect("a clean run");

    assert_eq!(
        walk(&root),
        [
            "OVERVIEW.md",
            "01-published-orientation-i1.md",
            "02-linear-algebra-i2",
            "OVERVIEW.md",
            "01-vectors-i5",
            "OVERVIEW.md",
            "02-draft-matrices-i6.md",
            "03-draft-assessment-i9.md",
        ]
    );
    let algebra = root.join("02-linear-algebra-i2");
    assert_eq!(
        fs::read_to_string(algebra.join("02-draft-matrices-i6.md")).expect("the sibling"),
        "matrices",
        "the sibling at the next ordinal is untouched — not its name, not its \
         key, not a byte"
    );
    assert_eq!(
        fs::read_to_string(algebra.join("OVERVIEW.md")).expect("the module's own content"),
        "linear algebra",
        "and the level's own distinguished child is not the one that moved"
    );
}

/// Discharges `wit_refusedPromoteNotLeaf` at the surface: a node is already a
/// node, the refusal reaches the consumer as [`Error::Refused`], and the tree is
/// untouched.
#[test]
fn promoting_a_node_is_refused_and_changes_nothing() {
    let (_temporary, root) = documents_tree();
    let before = walk(&root);
    let error = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .promote(Key::new(2), topic("linear-algebra"), None)
        .expect_err("a node is already a node");

    assert!(matches!(refusal(error), Refusal::PromoteNotLeaf { .. }));
    assert_eq!(walk(&root), before, "a refusal changes nothing");
}

/// Discharges `wit_refusedPromotePartsNotNode` at the surface, and with it the
/// reason the parts come from the caller at all: the library cannot make a
/// `Parts` that describes this entry as a node, so it checks the one it is
/// handed.
#[test]
fn promoting_with_parts_that_make_a_leaf_is_refused_and_changes_nothing() {
    let (_temporary, root) = documents_tree();
    let before = walk(&root);
    let error = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .promote(Key::new(1), draft("orientation"), None)
        .expect_err("a leaf's parts do not name a directory")
        .to_string();

    assert!(
        error.contains("make a leaf, not a node"),
        "a refusal says what to do about it: {error}"
    );
    assert_eq!(walk(&root), before, "a refusal changes nothing");
}

/// Discharges `wit_refusedPromoteNoDistinguished` at the surface — the whole
/// content of the model's `no_distinguished` instance, in a real domain.
///
/// [`Contentless`] disclaims `OVERVIEW.md` rather than merely declining to name
/// it, so the tree it reads is the same directory with one fewer entry in it.
/// The refusal is the domain's shape and not this call's: nothing this consumer
/// can pass would promote anything.
#[test]
fn promoting_in_a_domain_with_no_distinguished_child_is_refused_and_changes_nothing() {
    let (_temporary, root) = documents_tree();
    let before = walk(&root);
    let error = ordinal_fs_tree::fs::write::<Contentless>(&root)
        .expect("a well-formed tree")
        .promote(Key::new(1), topic("orientation"), None)
        .expect_err("the leaf's content would have nowhere to go");

    let Error::Refused(Refusal::PromoteNoDistinguished { key }) = &error else {
        panic!("expected the domain-shaped refusal, got {error:?}");
    };
    assert_eq!(*key, Key::new(1));
    assert!(
        error.to_string().contains("nowhere to go"),
        "a refusal says why, and what to do: {error}"
    );
    assert_eq!(walk(&root), before, "a refusal changes nothing");
}

/// **No model claim, and none possible**: content is unmodelled in both models
/// by design. A promotion creates an entry when it is given a first child, so it
/// inherits the refusal that belongs to *every* operation that creates one.
#[test]
fn bytes_for_a_first_child_that_makes_a_node_are_refused() {
    let (_temporary, root) = documents_tree();
    let before = walk(&root);
    let error = ordinal_fs_tree::fs::write::<SyllabusName>(&root)
        .expect("a well-formed tree")
        .promote(
            Key::new(1),
            topic("orientation"),
            Some(NewEntry::new(topic("nested"), b"bytes".to_vec())),
        )
        .expect_err("a directory has nowhere to hold bytes");

    assert_eq!(refusal(error), Refusal::ContentForANode);
    assert_eq!(walk(&root), before, "a refusal changes nothing");
}

/// A domain with **no distinguished child**: `operations.qnt`'s
/// `no_distinguished` instance, as a real `EntryName`.
///
/// It disclaims `OVERVIEW.md`. A domain answering `None` here while still
/// parsing some name as `Distinguished` would have a distinguished child the
/// library cannot name — the obligation *`distinguished()` names the only entry
/// of its species*, read backwards — so the honest domain has none at all.
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
// The domain whose `Parts` equality is coarser than its species
//
// `promote-k25`'s finding, as a domain rather than as an argument. Everything
// below exists to put one question to the public surface: when a lawful `Eq`
// makes a leaf's parts and its promoted node's parts compare **equal**, does
// the promotion still happen?
//
// The node deliberately reuses the leaf's ordinal and key — that is what a
// promotion *is* — so parts are the only thing left to tell the two names
// apart, and this domain has given that up. What remains is the species, and
// the library compares it: see `EntryNameExt::same_name`.
// ---------------------------------------------------------------------------

/// The reference domain's parts, compared by **label alone**.
///
/// A lawful equivalence relation — reflexive, symmetric, transitive — and
/// coarser than the rendering: a lesson and a module sharing a label compare
/// equal here while rendering as `01-draft-vectors-i1.md` and `01-vectors-i1`.
/// The trait asks only for `Clone + Eq`, so nothing in the seam forbids it, and
/// `positioned_species` still answers honestly for each value.
#[derive(Clone, Debug)]
struct LabelOnly(Parts);

impl PartialEq for LabelOnly {
    fn eq(&self, other: &Self) -> bool {
        self.0.label() == other.0.label()
    }
}

impl Eq for LabelOnly {}

/// A domain that renders exactly as the reference domain does and compares its
/// parts more coarsely than it renders them.
#[derive(Clone)]
enum Blind {
    Positioned {
        ordinal: Ordinal,
        key: Key,
        parts: LabelOnly,
    },
    Overview,
}

impl Blind {
    /// The reference name this one renders as. The two grammars are the same
    /// grammar — only the equality differs — so `Display` and `parse` are the
    /// reference domain's, and the adversarial half is `LabelOnly` alone.
    fn reference(&self) -> SyllabusName {
        match self {
            Self::Overview => SyllabusName::Overview,
            Self::Positioned {
                ordinal,
                key,
                parts,
            } => SyllabusName::compose(*ordinal, *key, parts.0.clone()),
        }
    }
}

impl fmt::Display for Blind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.reference(), f)
    }
}

impl EntryName for Blind {
    type Parts = LabelOnly;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        match SyllabusName::parse(name, found) {
            Verdict::Entry(inner) => Verdict::Entry(match inner.view() {
                NameView::Distinguished => Self::Overview,
                NameView::Positioned(triple) => Self::Positioned {
                    ordinal: triple.ordinal,
                    key: triple.key,
                    parts: LabelOnly(triple.parts.clone()),
                },
            }),
            Verdict::Foreign => Verdict::Foreign,
            Verdict::Malformed(e) => Verdict::Malformed(e),
            Verdict::Reserved(e) => Verdict::Reserved(e),
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self::Positioned {
            ordinal,
            key,
            parts,
        }
    }

    fn distinguished() -> Option<Self> {
        Some(Self::Overview)
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        match self {
            Self::Overview => NameView::Distinguished,
            Self::Positioned {
                ordinal,
                key,
                parts,
            } => NameView::Positioned(Triple {
                ordinal: *ordinal,
                key: *key,
                parts,
            }),
        }
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        parts.0.species()
    }
}

/// **No model claim, and neither model can pose one**: `structure.als` compares
/// `Parts` atoms and `operations.qnt` compares ints, so in both of them equal
/// parts are the *same* parts and this domain does not exist.
///
/// This is the mutation control for the test below, and it is a control in both
/// directions. It proves the domain is genuinely adversarial — the two names
/// have equal [`NameView`]s, so a `same_name` that compared views alone would
/// call the promotion's destination occupied — and it proves that what
/// separates them is the species and nothing else. Revert
/// `EntryNameExt::same_name` to a view comparison and this test fails on its
/// last assertion while the promotion below fails with `DestinationOccupied`.
#[test]
fn a_coarser_parts_equality_makes_two_species_share_one_view() {
    let leaf = Blind::compose(
        Ordinal::new(1),
        Key::new(1),
        LabelOnly(draft("orientation")),
    );
    let node = Blind::compose(
        Ordinal::new(1),
        Key::new(1),
        LabelOnly(topic("orientation")),
    );

    assert_eq!(
        leaf.view(),
        node.view(),
        "the ordinal and the key are the promotion's own, and this domain's \
         parts equality ignores the rest"
    );
    assert_ne!(
        leaf.to_string(),
        node.to_string(),
        "and yet they are two different filenames, which is what makes an \
         occupancy decided on the view alone wrong"
    );
    assert_eq!(
        Blind::positioned_species(&LabelOnly(draft("orientation"))),
        PositionedSpecies::Leaf
    );
    assert_eq!(
        Blind::positioned_species(&LabelOnly(topic("orientation"))),
        PositionedSpecies::Node
    );
    assert!(
        !leaf.same_name(&node),
        "so they are not the same name: identity includes the species"
    );
}

/// **No model claim**, for the reason the control above gives. This is
/// `promote-k25`'s finding at the public on-disk surface: the domain conforms,
/// and the promotion its contract promises goes through.
#[test]
fn a_domain_whose_parts_equality_ignores_the_species_conforms_and_can_promote() {
    let report = ordinal_fs_tree::conformance::check::<Blind>(
        &[
            ("OVERVIEW.md", Found::File),
            ("01-published-orientation-i1.md", Found::File),
            ("02-linear-algebra-i2", Found::Dir),
            ("README.md", Found::File),
        ],
        &[
            (
                Ordinal::new(1),
                Key::new(1),
                LabelOnly(draft("orientation")),
            ),
            (
                Ordinal::new(1),
                Key::new(1),
                LabelOnly(topic("orientation")),
            ),
        ],
    );
    report.assert_conforming();

    let (_temporary, root) = documents_tree();
    ordinal_fs_tree::fs::write::<Blind>(&root)
        .expect("a well-formed tree")
        .promote(Key::new(1), LabelOnly(topic("orientation")), None)
        .expect("a lawful domain does not lose its promotion to its own equality");

    let node = root.join("01-orientation-i1");
    assert!(
        node.is_dir(),
        "a promotion makes a directory where a file was"
    );
    assert_eq!(
        fs::read_to_string(node.join("OVERVIEW.md")).expect("the distinguished child"),
        "orientation",
        "and the leaf's bytes moved verbatim, as in any other domain"
    );
}
