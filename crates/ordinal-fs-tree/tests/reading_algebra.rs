//! The five reading operations, on trees built by hand.
//!
//! No filesystem anywhere in this file: a snapshot is names and structure, and
//! `Builder` is how a test supplies both. That is the point of the algebra
//! boundary — walk order is checkable without a directory.
//!
//! Every test here names the model claim it discharges, or says it has none.
//! Most of them have none, and that is the honest reading rather than an
//! omission: `operations.qnt`'s handoff block records walk **order** as
//! unmodelled — it models reachability, and resolves `by_key` on a duplicate-key
//! tree by picking the least internal id. So the order these tests check comes
//! from `ARCHITECTURE.md`'s *Operations → Reading* table and from nowhere else,
//! and entry 003's warning is why it is spelled out: a property satisfied by
//! construction looks exactly like one that was verified.

use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusName};
use ordinal_fs_tree::{Builder, EntryName, Key, Ordinal, Snapshot, Species};

fn lesson(ordinal: u32, key: u32, status: Status, label: &str) -> SyllabusName {
    SyllabusName::compose(
        Ordinal::new(ordinal),
        Key::new(key),
        Parts::lesson(status, Label::new(label).expect("a well-formed label")),
    )
}

fn module(ordinal: u32, key: u32, label: &str) -> SyllabusName {
    SyllabusName::compose(
        Ordinal::new(ordinal),
        Key::new(key),
        Parts::module(Label::new(label).expect("a well-formed label")),
    )
}

fn overview() -> SyllabusName {
    SyllabusName::distinguished().expect("this domain has a distinguished child")
}

fn rendered(snapshot: &Snapshot<SyllabusName>) -> Vec<String> {
    snapshot.walk().map(|e| e.name().to_string()).collect()
}

/// `ARCHITECTURE.md`'s own tree, built in the order the document draws it.
fn documents_tree() -> Snapshot<SyllabusName> {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, overview());
    builder.add(root, lesson(1, 1, Status::Published, "orientation"));
    let algebra = builder
        .add(root, module(2, 2, "linear-algebra"))
        .expect("a module is a node");
    builder.add(algebra, overview());
    builder.add(algebra, lesson(1, 5, Status::Published, "vectors"));
    builder.add(algebra, lesson(2, 6, Status::Draft, "matrices"));
    builder.add(root, lesson(3, 9, Status::Draft, "assessment"));
    builder.finish()
}

const DOCUMENTS_WALK: &[&str] = &[
    "OVERVIEW.md",
    "01-published-orientation-i1.md",
    "02-linear-algebra-i2",
    "OVERVIEW.md",
    "01-published-vectors-i5.md",
    "02-draft-matrices-i6.md",
    "03-draft-assessment-i9.md",
];

/// Discharges no model claim — walk order is unmodelled. What it checks is the
/// *Reading* table's `walk` row, on the document's own example tree: depth-first
/// pre-order, the distinguished child first within a level, then children by
/// ordinal, and a node fully explored before its later siblings.
#[test]
fn walk_is_depth_first_with_the_distinguished_child_first() {
    assert_eq!(rendered(&documents_tree()), DOCUMENTS_WALK);
}

/// Discharges no model claim. The order a level is *built* in is arbitrary — a
/// directory listing arrives in whatever order the filesystem chose — so walk
/// order has to come from the names and never from insertion. Without this, two
/// machines holding byte-identical trees would disagree about *the first in walk
/// order*, and `by_key`'s tie-break below would be a different entry on each.
#[test]
fn walk_order_does_not_depend_on_the_order_names_arrived_in() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(3, 9, Status::Draft, "assessment"));
    let algebra = builder
        .add(root, module(2, 2, "linear-algebra"))
        .expect("a module is a node");
    builder.add(algebra, lesson(2, 6, Status::Draft, "matrices"));
    builder.add(algebra, overview());
    builder.add(algebra, lesson(1, 5, Status::Published, "vectors"));
    builder.add(root, lesson(1, 1, Status::Published, "orientation"));
    builder.add(root, overview());
    assert_eq!(rendered(&builder.finish()), DOCUMENTS_WALK);
}

/// Discharges no model claim, and cannot: the tree it builds is one the library
/// would never produce. `structure.als`'s `OrdinalsDistinct` and
/// `operations.qnt`'s `inv_ordinalsDistinctAtRest` both say a level has no
/// repeated ordinal — but every invariant in this design is a *preservation*
/// property, so a level hand-edited into carrying one is a tree the library must
/// still order deterministically. Key, then rendered name, and the second is
/// total because one directory cannot hold two entries of one name.
#[test]
fn a_duplicated_ordinal_is_ordered_by_key_and_then_by_name() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(1, 7, Status::Draft, "beta"));
    builder.add(root, lesson(1, 3, Status::Draft, "alpha"));
    builder.add(root, lesson(1, 3, Status::Published, "alpha"));
    assert_eq!(
        rendered(&builder.finish()),
        [
            // key 3 before key 7 …
            "01-draft-alpha-i3.md",
            // … and, at one key, by the name itself.
            "01-published-alpha-i3.md",
            "01-draft-beta-i7.md",
        ]
    );
}

/// Discharges the *Reading* table's `by_key` row on an ordinary tree, and
/// nothing more: `structure.als`'s `KeysUnique` is a precondition the library
/// never checks, so the interesting case is the next test.
#[test]
fn by_key_finds_the_entry_with_that_key() {
    let tree = documents_tree();
    let found = tree.by_key(Key::new(6)).expect("key 6 is in the tree");
    assert_eq!(found.name().to_string(), "02-draft-matrices-i6.md");
    assert_eq!(found.depth(), 2);
    assert!(tree.by_key(Key::new(99)).is_none(), "key 99 is not");
}

/// Discharges **no** model claim, and this is the one place where saying so
/// matters most. `operations.qnt` admits a duplicate-key tree
/// (`wit_duplicateKeysAdmitted`) and resolves the target by least internal id,
/// which is not walk order; `structure.als` states `KeysUnique` as a
/// precondition it does not enforce. So *the first in walk order* rests on the
/// document and on this test. The tree here would need a hand edit to exist, and
/// the caller has one to repair.
#[test]
fn by_key_on_a_duplicate_key_tree_answers_the_first_in_walk_order() {
    let mut builder = Builder::new();
    let root = builder.root();
    let first = builder
        .add(root, module(1, 4, "first"))
        .expect("a module is a node");
    // Deeper, and inside the *earlier* sibling: reached second, because a walk
    // descends in place.
    builder.add(first, lesson(1, 4, Status::Draft, "buried"));
    builder.add(root, lesson(2, 4, Status::Draft, "later"));

    let tree = builder.finish();
    let found = tree.by_key(Key::new(4)).expect("key 4 is in the tree, thrice");
    assert_eq!(found.name().to_string(), "01-first-i4");
    assert_eq!(
        rendered(&tree),
        ["01-first-i4", "01-draft-buried-i4.md", "02-draft-later-i4.md"],
        "the walk order this answer is the first of"
    );
}

/// Discharges no model claim. The *Reading* table says `find` short-circuits,
/// which is a statement about work not done and is invisible to any assertion
/// about the answer — so the predicate counts its own calls.
#[test]
fn find_short_circuits_at_the_first_match() {
    let tree = documents_tree();
    let mut seen = 0;
    let found = tree
        .find(|entry| {
            seen += 1;
            entry.species() == Species::Node
        })
        .expect("the tree holds a module");
    assert_eq!(found.name().to_string(), "02-linear-algebra-i2");
    assert_eq!(seen, 3, "the walk should have stopped at the third entry");
    assert_eq!(
        rendered(&tree).len(),
        7,
        "a full walk would have visited this many"
    );
}

/// Discharges no model claim. The *Reading* table's `ancestors` row, including
/// the sentence this test exists for: the chain ends at the tree root, which is
/// a node and **not** an entry, so its element type cannot be the entry type.
/// The last assertion is the one that would fail if `ancestors` ever returned
/// entries.
#[test]
fn ancestors_are_root_first_and_the_root_is_not_an_entry() {
    let tree = documents_tree();
    let matrices = tree.by_key(Key::new(6)).expect("key 6 is in the tree");

    let chain = matrices.ancestors();
    let names: Vec<String> = chain
        .iter()
        .map(|container| match container.entry() {
            None => "<root>".to_string(),
            Some(entry) => entry.name().to_string(),
        })
        .collect();
    assert_eq!(names, ["<root>", "02-linear-algebra-i2"]);

    let root = chain.first().expect("the chain is not empty");
    assert!(root.is_root());
    assert!(
        root.entry().is_none(),
        "the tree root has no ordinal, no key and no parts, so it is no entry"
    );

    // A child of the root has the root and nothing else above it.
    let orientation = tree.by_key(Key::new(1)).expect("key 1 is in the tree");
    let chain = orientation.ancestors();
    assert_eq!(chain.len(), 1);
    assert!(chain[0].is_root());
}

/// Discharges no model claim. The *Reading* table's `distinguished_chain` row:
/// root-first, and levels without one are skipped rather than represented.
#[test]
fn the_distinguished_chain_skips_levels_that_have_none() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, overview());
    let outer = builder
        .add(root, module(1, 1, "outer"))
        .expect("a module is a node");
    // `outer` has no OVERVIEW.md — the level to be skipped.
    let inner = builder
        .add(outer, module(1, 2, "inner"))
        .expect("a module is a node");
    builder.add(inner, overview());
    builder.add(inner, lesson(1, 3, Status::Draft, "deep"));

    let tree = builder.finish();
    let deep = tree.by_key(Key::new(3)).expect("key 3 is in the tree");
    let chain = deep.distinguished_chain();
    assert_eq!(chain.len(), 2, "the root's and `inner`'s, not `outer`'s");
    for entry in &chain {
        assert_eq!(entry.species(), Species::Distinguished);
    }
    assert!(
        chain[0].container().is_root(),
        "root-first: the first is the root's own content"
    );
    assert_eq!(
        chain[1]
            .container()
            .entry()
            .expect("the second belongs to a node")
            .name()
            .to_string(),
        "01-inner-i2"
    );
}

/// Discharges no model claim. A level is one ordered list, so a distinguished
/// child is reachable both as `Container::distinguished` and in walk order — the
/// shape that keeps a domain's second distinguished child visible rather than
/// dropped. `Container::positioned` is the same list without it.
#[test]
fn a_level_offers_its_distinguished_child_and_its_positioned_children() {
    let tree = documents_tree();
    let root = tree.root();
    assert_eq!(
        root.distinguished()
            .expect("the root has its own content")
            .name()
            .to_string(),
        "OVERVIEW.md"
    );
    let positioned: Vec<String> = root.positioned().map(|e| e.name().to_string()).collect();
    assert_eq!(
        positioned,
        [
            "01-published-orientation-i1.md",
            "02-linear-algebra-i2",
            "03-draft-assessment-i9.md"
        ]
    );
    assert_eq!(root.children().count(), 4, "the distinguished child too");
}

/// Discharges no model claim. An empty tree is a tree: `ARCHITECTURE.md` says a
/// node may hold zero or more children and that nothing requires one to be
/// populated, and the root is a node.
#[test]
fn an_empty_tree_walks_to_nothing() {
    let tree: Snapshot<SyllabusName> = Builder::new().finish();
    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
    assert_eq!(tree.walk().count(), 0);
    assert!(tree.by_key(Key::new(1)).is_none());
    assert!(tree.root().distinguished().is_none());
}
