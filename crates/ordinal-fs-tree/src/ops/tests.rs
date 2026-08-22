//! `append` and `append_many`, as the algebra decides them: no filesystem, no
//! directory, and every answer a pure function of a snapshot built by hand.
//!
//! Every test here names the model claim it discharges, or says it has none.

use super::{append, append_many, NewEntry, Target};
use crate::fixtures::{documents_tree, empty_tree, lesson, module};
use crate::plan::{Decision, Effect, Level, Refusal};
use crate::reference::{Label, Parts, Status, SyllabusName};
use crate::snapshot::{Builder, Snapshot};
use crate::{EntryNameExt, Key, Species};

fn draft(label: &str) -> Parts {
    Parts::lesson(
        Status::Draft,
        Label::new(label).expect("a well-formed label"),
    )
}

fn topic(label: &str) -> Parts {
    Parts::module(Label::new(label).expect("a well-formed label"))
}

/// The names a decision's plan would create, with the level each lands in.
fn creations(decision: Decision<SyllabusName>) -> Vec<(Level, String, Vec<u8>)> {
    match decision {
        Decision::Refuse(refusal) => panic!("expected a plan, got {refusal:?}"),
        Decision::Proceed(plan) => plan
            .effects()
            .iter()
            .map(|effect| match effect {
                Effect::Create { at, name, content } => (*at, name.to_string(), content.clone()),
                Effect::MoveTo { .. } => panic!("an append plan renames nothing"),
            })
            .collect(),
    }
}

fn refusal(decision: Decision<SyllabusName>) -> Refusal {
    match decision {
        Decision::Refuse(refusal) => refusal,
        Decision::Proceed(_) => panic!("expected a refusal, got a plan"),
    }
}

/// Discharges `inv_appendOnlyAdds` on the emptiest tree there is: *the new entry
/// sits in the target level at `maxOrdIn(before, d) + 1` with key
/// `freshKey(before)*, and nothing else changed. A level holding nothing has a
/// greatest ordinal of zero, so the first append lands on [`Ordinal::FIRST`],
/// and a tree holding no keys makes the first key 1.
#[test]
fn the_first_append_into_an_empty_tree_is_ordinal_one_key_one() {
    let decision = append(
        &empty_tree(),
        Target::Root,
        NewEntry::new(draft("orientation"), b"hello".to_vec()),
    );
    assert_eq!(
        creations(decision),
        [(
            Level::Root,
            "01-draft-orientation-i1.md".to_string(),
            b"hello".to_vec()
        )]
    );
}

/// Discharges `inv_appendOnlyAdds`'s ordinal half: `maxOrdIn(before, d) + 1`,
/// where `d` is the *target level* — the document's tree has three positioned
/// children at the root, so the next is 4.
///
/// And its key half, which is **not** the level's business: `freshKey` is
/// `max(key over the whole tree) + 1`, and the greatest key here (9) is at the
/// root while the deepest entries carry 5 and 6.
#[test]
fn an_append_takes_the_levels_next_ordinal_and_the_trees_next_key() {
    let decision = append(
        &documents_tree(),
        Target::Root,
        NewEntry::empty(draft("assessment-two")),
    );
    assert_eq!(
        creations(decision),
        [(
            Level::Root,
            "04-draft-assessment-two-i10.md".to_string(),
            Vec::new()
        )]
    );
}

/// Discharges the same claim on the case that tells a whole-tree maximum from a
/// per-level one: the greatest key in the tree is two levels down, and an append
/// at the **root** must still step past it. A per-level `max + 1` would allocate
/// 10 here and re-issue a key another entry already holds.
#[test]
fn the_fresh_key_is_the_whole_trees_maximum_and_not_the_levels() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(1, 1, Status::Published, "orientation"));
    let algebra = builder
        .add(root, module(2, 9, "linear-algebra"))
        .expect("a module is a node");
    builder.add(algebra, lesson(1, 41, Status::Draft, "vectors"));
    let snapshot = builder.finish();

    let decision = append(&snapshot, Target::Root, NewEntry::empty(draft("next")));
    assert_eq!(
        creations(decision),
        [(Level::Root, "03-draft-next-i42.md".to_string(), Vec::new())]
    );
}

/// Discharges `RefusedTargetNotNode`'s complement: a node named by key is a
/// level, and an append into it counts *its* children rather than the root's.
#[test]
fn an_append_into_a_node_counts_that_nodes_children() {
    let snapshot = documents_tree();
    let algebra = snapshot
        .by_key(Key::new(2))
        .expect("the document's module")
        .index();
    let decision = append(
        &snapshot,
        Target::Key(Key::new(2)),
        NewEntry::empty(draft("eigenvalues")),
    );
    assert_eq!(
        creations(decision),
        [(
            Level::Entry(algebra),
            "03-draft-eigenvalues-i10.md".to_string(),
            Vec::new()
        )]
    );
}

/// Discharges `wit_appendManySucceeded` and the *Operations* table's own words:
/// *several children at consecutive ordinals with consecutive keys, planned from
/// one snapshot and applied as a unit*. One snapshot answers all three, which is
/// what makes them contiguous — a loop of `append`s would re-read between them.
#[test]
fn a_run_takes_consecutive_ordinals_and_consecutive_keys() {
    let decision = append_many(
        &documents_tree(),
        Target::Root,
        vec![
            NewEntry::empty(draft("one")),
            NewEntry::new(draft("two"), b"body".to_vec()),
            NewEntry::empty(topic("three")),
        ],
    );
    assert_eq!(
        creations(decision),
        [
            (Level::Root, "04-draft-one-i10.md".to_string(), Vec::new()),
            (
                Level::Root,
                "05-draft-two-i11.md".to_string(),
                b"body".to_vec()
            ),
            (Level::Root, "06-three-i12".to_string(), Vec::new()),
        ]
    );
}

/// Discharges no model claim. The species of each created name follows from its
/// parts and from nothing else — the third entry above is a module, so its name
/// is a node's, which is what tells the interpreter to make a directory.
#[test]
fn the_species_of_each_new_name_follows_from_its_parts() {
    let decision = append_many(
        &empty_tree(),
        Target::Root,
        vec![
            NewEntry::empty(draft("a-lesson")),
            NewEntry::empty(topic("a-module")),
        ],
    );
    let Decision::Proceed(plan) = decision else {
        panic!("expected a plan");
    };
    let species: Vec<Species> = plan
        .effects()
        .iter()
        .map(|effect| match effect {
            Effect::Create { name, .. } => name.species(),
            Effect::MoveTo { .. } => panic!("an append plan renames nothing"),
        })
        .collect();
    assert_eq!(species, [Species::Leaf, Species::Node]);
}

/// Discharges `inv_denseAtRest`'s deliberate absence from the `hand_edited`
/// instance, which is the model's way of saying density is *preserved and never
/// established*: an append into a gapped level takes `max + 1` and leaves the
/// gap exactly where it was. Counting the children instead would fill it — and
/// collide, on the tree where the gap is not at the end.
#[test]
fn an_append_into_a_gapped_level_keeps_the_gap() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(1, 1, Status::Draft, "first"));
    builder.add(root, lesson(4, 2, Status::Draft, "fourth"));
    let snapshot = builder.finish();

    let decision = append(&snapshot, Target::Root, NewEntry::empty(draft("next")));
    assert_eq!(
        creations(decision),
        [(Level::Root, "05-draft-next-i3.md".to_string(), Vec::new())]
    );
}

/// Discharges `wit_refusedTargetMissing`: *a key naming no entry is refused*.
#[test]
fn a_key_naming_no_entry_is_refused() {
    let refused = refusal(append(
        &documents_tree(),
        Target::Key(Key::new(99)),
        NewEntry::empty(draft("nowhere")),
    ));
    assert_eq!(refused, Refusal::TargetMissing { key: Key::new(99) });
    assert!(
        refused
            .to_string()
            .contains("no entry in this tree has key 99"),
        "a refusal says what to do about it: {refused}"
    );
}

/// Discharges `wit_refusedTargetNotNode`: *`append`, `append_many` and `insert`
/// require their target to be a node. A designated leaf is refused: a leaf is a
/// regular file and holds nothing.*
#[test]
fn a_leaf_target_is_refused() {
    let refused = refusal(append(
        &documents_tree(),
        Target::Key(Key::new(1)),
        NewEntry::empty(draft("inside-a-file")),
    ));
    assert_eq!(
        refused,
        Refusal::TargetNotNode {
            key: Key::new(1),
            species: Species::Leaf,
        }
    );
}

/// Discharges no model claim, and cannot: a distinguished child carries no key,
/// so `by_key` cannot answer with one and the refusal above is the only one a
/// caller can reach. The test is here because *the target is the node's own
/// content* is the case a reader expects to find refused, and the honest answer
/// is that it is unnameable.
#[test]
fn the_distinguished_child_cannot_be_named_as_a_target() {
    let snapshot = documents_tree();
    assert!(
        snapshot
            .walk()
            .filter(|entry| entry.species() == Species::Distinguished)
            .all(|entry| entry.key().is_none()),
        "a distinguished child has no key to name it by"
    );
}

/// Discharges no model claim, and **cannot**: content is outside both models by
/// design, so this refusal is the library's own — the same position
/// `Error::NonUtf8Name` is in. A directory has nowhere to put bytes, and
/// discarding them silently is the alternative.
#[test]
fn bytes_for_a_node_are_refused_rather_than_discarded() {
    assert_eq!(
        refusal(append(
            &empty_tree(),
            Target::Root,
            NewEntry::new(topic("a-module"), b"where would these go?".to_vec()),
        )),
        Refusal::ContentForANode
    );
}

/// Discharges no model claim, for a reason worth stating: an integer in either
/// model is unbounded, so neither can pose exhaustion at all. A [`Key`] is a
/// `u32`, and a hand-edited name carrying the maximum makes `max + 1`
/// impossible. Wrapping instead would re-issue a key other entries still
/// reference, which is the one thing the whole no-removal rule exists to
/// prevent.
#[test]
fn a_tree_whose_greatest_key_is_the_greatest_key_refuses_rather_than_wrapping() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(1, u32::MAX, Status::Draft, "the-last-one"));
    let snapshot = builder.finish();

    assert_eq!(
        refusal(append(
            &snapshot,
            Target::Root,
            NewEntry::empty(draft("next"))
        )),
        Refusal::KeysExhausted
    );
}

/// The same, for the ordinal — which is the level's and not the tree's, so a
/// level can be exhausted while the tree is not.
#[test]
fn a_level_whose_greatest_ordinal_is_the_greatest_ordinal_is_refused() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(u32::MAX, 1, Status::Draft, "the-last-place"));
    let snapshot = builder.finish();

    assert_eq!(
        refusal(append(
            &snapshot,
            Target::Root,
            NewEntry::empty(draft("next"))
        )),
        Refusal::OrdinalsExhausted
    );
}

/// Discharges no model claim. An empty run is a plan of no effects: it proceeds,
/// and the interpreter applies nothing. Stated as a test because the alternative
/// — a special case somewhere — is what usually happens to it.
#[test]
fn a_run_of_nothing_is_a_plan_of_nothing() {
    let decision = append_many(&documents_tree(), Target::Root, Vec::new());
    assert!(creations(decision).is_empty());
}

/// Discharges `inv_appendOnlyAdds`'s *and nothing else changed* half at the
/// level the algebra can be held to: the plan **names** nothing but the entry it
/// creates. A plan is a value, so this is checkable by reading it — which is
/// precisely what `ARCHITECTURE.md` says the two rejected shapes could not
/// offer.
#[test]
fn an_append_plan_names_nothing_but_the_entry_it_creates() {
    let snapshot: Snapshot<SyllabusName> = documents_tree();
    let decision = append(&snapshot, Target::Root, NewEntry::empty(draft("next")));
    let Decision::Proceed(plan) = decision else {
        panic!("expected a plan");
    };
    assert_eq!(plan.effects().len(), 1);
}
