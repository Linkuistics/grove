//! `append`, `append_many`, `insert`, `promote` and `rewrite`, as the algebra
//! decides them: no
//! filesystem, no directory, and every answer a pure function of a snapshot
//! built by hand.
//!
//! Every test here names the model claim it discharges, or says it has none.

use super::{append, append_many, insert, promote, rewrite, NewEntry, Target};
use crate::fixtures::{
    contentless_tree, documents_tree, empty_tree, lesson, module, overview, Contentless,
};
use crate::plan::{Decision, Effect, Level, Plan, Refusal};
use crate::reference::{Label, Parts, Status, SyllabusName};
use crate::snapshot::{Builder, Snapshot};
use crate::{EntryName, EntryNameExt, Key, Ordinal, PositionedSpecies, Sought, Species};

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
/// level can be exhausted while the tree is not. Discharges no model claim, and
/// for the same reason the one above does not: an integer in either model is
/// unbounded, so neither can pose exhaustion at all.
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

// ---------------------------------------------------------------------------
// `insert`
// ---------------------------------------------------------------------------

/// The ordinal an effect's name lands on, and which entry carries it there —
/// `None` for the entry the plan creates.
///
/// This is the plan read as a *value*, which is the whole reason the plan shape
/// was chosen over the two `ARCHITECTURE.md` rejects: the ordering rule is a
/// property of this list, not an accident of a loop's direction.
fn landings(plan: &Plan<SyllabusName>) -> Vec<(Option<usize>, u32)> {
    plan.effects()
        .iter()
        .map(|effect| match effect {
            Effect::Create { name, .. } => (
                None,
                name.triple()
                    .expect("a composed name is positioned")
                    .ordinal
                    .get(),
            ),
            Effect::MoveTo { entry, name, .. } => (
                Some(*entry),
                name.triple()
                    .expect("a composed name is positioned")
                    .ordinal
                    .get(),
            ),
        })
        .collect()
}

/// The ordinals one level holds after each landing in turn — the intermediate
/// states an apply passes through, which is what the shift order is about.
///
/// The interpreter applies one effect at a time and a process killed between two
/// of them leaves exactly one of these states on disk. A test cannot observe an
/// interruption; it can read every state an interruption could stop at, because
/// the plan is a value.
fn ordinals_after_each_step(
    level: &[(usize, u32)],
    landings: &[(Option<usize>, u32)],
) -> Vec<Vec<u32>> {
    let mut held: Vec<(Option<usize>, u32)> = level
        .iter()
        .map(|(index, ordinal)| (Some(*index), *ordinal))
        .collect();
    let mut states = Vec::with_capacity(landings.len());
    for (who, ordinal) in landings {
        match held.iter_mut().find(|(holder, _)| holder == who) {
            Some(slot) => slot.1 = *ordinal,
            None => held.push((*who, *ordinal)),
        }
        states.push(held.iter().map(|(_, ordinal)| *ordinal).collect());
    }
    states
}

fn distinct(ordinals: &[u32]) -> bool {
    let mut sorted = ordinals.to_vec();
    sorted.sort_unstable();
    sorted.windows(2).all(|pair| pair[0] != pair[1])
}

/// The top level of a snapshot as `(arena index, ordinal)`, in walk order.
fn top_level(snapshot: &Snapshot<SyllabusName>) -> Vec<(usize, u32)> {
    snapshot
        .root()
        .positioned()
        .map(|entry| {
            (
                entry.index(),
                entry
                    .ordinal()
                    .expect("a positioned entry has an ordinal")
                    .get(),
            )
        })
        .collect()
}

fn plan(decision: Decision<SyllabusName>) -> Plan<SyllabusName> {
    match decision {
        Decision::Proceed(plan) => plan,
        Decision::Refuse(refusal) => panic!("expected a plan, got {refusal:?}"),
    }
}

/// Discharges `inv_insertOnlyShifts`: *an `insert` changes the ordinals of
/// siblings at or after the target and nothing else*, at the level the algebra
/// can be held to — the plan is one rename per shifted sibling, each moving that
/// sibling's own key and parts to `ordinal + 1`, plus one create at the target
/// ordinal carrying the tree's fresh key.
///
/// The document's tree holds `01-published-orientation-i1.md`,
/// `02-linear-algebra-i2/` and `03-draft-assessment-i9.md` at the root, so an
/// insert at ordinal 2 shifts the module and the assessment and leaves the
/// orientation alone.
#[test]
fn an_insert_shifts_the_occupant_and_every_later_sibling() {
    let decision = insert(
        &documents_tree(),
        Target::Root,
        Ordinal::new(2),
        NewEntry::new(draft("interlude"), b"between".to_vec()),
    );
    let plan = plan(decision);
    let names: Vec<String> = plan
        .effects()
        .iter()
        .map(|effect| effect.name().to_string())
        .collect();
    assert_eq!(
        names,
        [
            // The shifts, highest ordinal first.
            "04-draft-assessment-i9.md",
            "03-linear-algebra-i2",
            // Then the new entry, at the ordinal that was vacated for it.
            "02-draft-interlude-i10.md",
        ]
    );
}

/// Discharges `shiftIds` under `HIGHEST_FIRST`, read off the plan: the renames
/// run in **descending** destination order, and the create comes last.
///
/// Stated separately from the test above because that one would still pass if
/// the two renames swapped and the names came out in some other arrangement —
/// this is the order itself, which is the property, and it is the thing a later
/// refactor is most likely to lose.
#[test]
fn the_shift_runs_highest_ordinal_first() {
    let snapshot = documents_tree();
    let plan = plan(insert(
        &snapshot,
        Target::Root,
        Ordinal::new(1),
        NewEntry::empty(draft("preface")),
    ));
    let moved: Vec<u32> = landings(&plan)
        .into_iter()
        .filter_map(|(who, ordinal)| who.map(|_| ordinal))
        .collect();
    assert_eq!(moved, [4, 3, 2], "the renames descend");
    assert_eq!(
        landings(&plan).last().expect("a plan with effects").0,
        None,
        "the create is last: every destination is vacated before it is needed"
    );
}

/// Discharges `inv_ordinalsDistinctThroughout` for `insert` — **the reason the
/// ordering rule exists**, and the only one that applies to every tree.
///
/// Not collision. A name embeds a tree-unique key, so two siblings never want
/// the same filename and no order collides on a well-formed tree; a test here
/// asserting *the other order collides* would be testing the architecture
/// document's corrected predecessor. `docs/formalism-findings.md` entry 003 is
/// where the model contradicted the document on exactly this.
///
/// What the order decides is what an **interruption** leaves. Highest-first
/// vacates each destination before it is needed, so every intermediate state has
/// distinct ordinals and a crash leaves a level that is merely gapped — which
/// this design admits everywhere. An interruption is not something a passing
/// test observes, so this reasons about the plan, which is a value, and reads
/// every state a crash could stop at off it. Those states are
/// `wit_shiftPartiallyApplied` — *a sibling shift that has landed some of its
/// renames and not the rest*, which the model reaches in `hand_edited` and
/// records as gapped and well-formed.
#[test]
fn every_intermediate_state_of_a_shift_has_distinct_ordinals() {
    let snapshot = documents_tree();
    let plan = plan(insert(
        &snapshot,
        Target::Root,
        Ordinal::new(1),
        NewEntry::empty(draft("preface")),
    ));
    let level = top_level(&snapshot);
    for state in ordinals_after_each_step(&level, &landings(&plan)) {
        assert!(
            distinct(&state),
            "an interrupted insert must leave a gapped level, never a duplicate \
             ordinal, and this state has one: {state:?}"
        );
    }
}

/// Discharges `wit_shiftTransientlyDuplicatesAnOrdinal`, which the model reaches
/// in the `lowest_first` instance **and nowhere else**. It is the control on the
/// test above: without it, an implementation that shifted in any order at all
/// would pass, because nothing would show that the property is contingent.
///
/// The same renames, replayed lowest-first: the first of them moves ordinal 1
/// onto 2 while ordinal 2 is still occupied, so the level transiently holds two
/// entries at ordinal 2 — a state this design does not admit, and one a process
/// killed at that moment would leave on disk.
#[test]
fn the_same_shifts_run_lowest_first_pass_through_a_duplicate_ordinal() {
    let snapshot = documents_tree();
    let plan = plan(insert(
        &snapshot,
        Target::Root,
        Ordinal::new(1),
        NewEntry::empty(draft("preface")),
    ));
    let mut reversed = landings(&plan);
    let create = reversed.pop().expect("the create is last");
    reversed.reverse();
    reversed.push(create);

    let level = top_level(&snapshot);
    assert!(
        ordinals_after_each_step(&level, &reversed)
            .iter()
            .any(|state| !distinct(state)),
        "lowest-first must pass through a duplicate ordinal, or the rule buys nothing"
    );
}

/// Discharges the **checkable half** of the invariant *Subtree preservation
/// under shift*: an `insert`'s plan names no descendant. One rename per shifted
/// sibling and one create, and nothing else.
///
/// The other half — that one directory rename carries a whole subtree — is a
/// property of `rename(2)`, below the boundary both models stop at, and is
/// **assumed**. `operations.qnt` makes it true by construction, since entries
/// reference their parent by a stable id, and `docs/formalism-findings.md` entry
/// 003's first miss is the warning that a model satisfying an invariant by
/// construction looks exactly like one that verified it. So this test is named
/// for the half it can hold and says which half it cannot.
#[test]
fn an_inserts_plan_names_no_descendant() {
    let snapshot = documents_tree();
    let plan = plan(insert(
        &snapshot,
        Target::Root,
        Ordinal::new(2),
        NewEntry::empty(draft("interlude")),
    ));
    for effect in plan.effects() {
        let level = match effect {
            Effect::Create { at, .. } | Effect::MoveTo { to: at, .. } => *at,
        };
        assert_eq!(level, Level::Root, "every effect acts in the target level");
        if let Effect::MoveTo { entry, .. } = effect {
            assert_eq!(
                snapshot.at(*entry).depth(),
                1,
                "a shifted entry is a child of the target level, never something \
                 inside one — a shifted node is one directory rename"
            );
        }
    }
    // Three: two shifts and one create. The module being shifted holds three
    // children of its own, and a plan that reached inside it would be longer.
    assert_eq!(plan.effects().len(), 3);
}

/// Discharges no model claim: the distinguished child carries no ordinal, so it
/// is not in `idsAtOrdinal` for any ordinal and the model cannot pose it moving.
/// The test is here because *the node's own content sits in the level being
/// shifted* is what a reader worries about, and the answer is that it never
/// participates in ordering.
#[test]
fn an_insert_leaves_the_distinguished_child_alone() {
    let snapshot = documents_tree();
    let plan = plan(insert(
        &snapshot,
        Target::Root,
        Ordinal::new(1),
        NewEntry::empty(draft("preface")),
    ));
    for effect in plan.effects() {
        if let Effect::MoveTo { entry, .. } = effect {
            assert_ne!(
                snapshot.at(*entry).species(),
                Species::Distinguished,
                "the distinguished child has no ordinal and is never shifted"
            );
        }
    }
}

/// Discharges `inv_insertOnlyShifts`'s key half, and `freshKey` with it: the new
/// entry takes `max(key over the whole tree) + 1`, while every shifted sibling
/// keeps the key it had. A shift is `compose(new_ordinal, key, parts)` — derived,
/// and therefore incapable of disturbing a key, a label or an attribute.
#[test]
fn a_shift_moves_the_ordinal_and_keeps_the_key_and_the_parts() {
    let snapshot = documents_tree();
    let plan = plan(insert(
        &snapshot,
        Target::Root,
        Ordinal::new(2),
        NewEntry::empty(draft("interlude")),
    ));
    for effect in plan.effects() {
        match effect {
            Effect::MoveTo { entry, name, .. } => {
                let was = snapshot.at(*entry);
                let before = was.triple().expect("a shifted entry is positioned");
                let after = name.triple().expect("a composed name is positioned");
                assert_eq!(after.key, before.key, "a shift keeps the key");
                assert_eq!(after.parts, before.parts, "a shift keeps the parts");
                assert_eq!(
                    after.ordinal.get(),
                    before.ordinal.get() + 1,
                    "and moves the ordinal by exactly one"
                );
                assert_eq!(
                    name.species(),
                    was.species(),
                    "the species follows from the parts, so a shift cannot rename \
                     a file into a directory"
                );
            }
            Effect::Create { name, .. } => assert_eq!(
                name.triple().expect("a composed name is positioned").key,
                Key::new(10),
                "the new entry takes the whole tree's next key"
            ),
        }
    }
}

/// Discharges `wit_insertPastTheEnd`: *inserting past the last sibling is
/// `append`'s job and is refused rather than quietly redirected — the two differ
/// in their effect on every later sibling, so guessing which was meant would be
/// guessing at intent.*
#[test]
fn inserting_past_the_last_sibling_is_refused_rather_than_redirected() {
    let refused = refusal(insert(
        &documents_tree(),
        Target::Root,
        Ordinal::new(4),
        NewEntry::empty(draft("too-far")),
    ));
    assert_eq!(
        refused,
        Refusal::NoOccupantAtOrdinal {
            ordinal: Ordinal::new(4),
            occupied: Some((Ordinal::FIRST, Ordinal::new(3))),
        }
    );
    assert!(
        refused.to_string().contains("`append`'s job"),
        "a refusal says what to do about it: {refused}"
    );
}

/// Discharges `wit_insertIntoAGap` — the **same** refusal on a case the
/// document's rationale does not cover, which the model is what surfaced
/// (`docs/formalism-findings.md` entry 003).
///
/// Density is preserved by every operation and established by none, so a level a
/// hand edit left gapped keeps its gap forever: `append` takes `max + 1` and
/// steps over it, and `insert` shifts an occupant rather than filling a hole. No
/// operation fills a gap, and a reader hitting this deserves to be told that
/// rather than left to work it out — so the message says *by hand*, and does not
/// send them to `append`, which would take ordinal 6.
#[test]
fn inserting_into_a_gap_is_refused_and_says_it_can_only_be_filled_by_hand() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(1, 1, Status::Draft, "first"));
    builder.add(root, lesson(5, 2, Status::Draft, "fifth"));
    let snapshot = builder.finish();

    let refused = refusal(insert(
        &snapshot,
        Target::Root,
        Ordinal::new(3),
        NewEntry::empty(draft("into-the-hole")),
    ));
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
        "the gap case has its own advice, and it is not `append`: {said}"
    );
    assert!(
        said.contains("something below it"),
        "an interior gap is the one hole that may claim a lower neighbour, \
         because the carried least proves one: {said}"
    );
}

/// **No model claim, and the model is why.** `wit_insertIntoAGap` discriminates
/// the gap from the past-the-end case with `a.at < maxOrdIn` alone, which is
/// true of an ordinal *below* every occupant as well as of one between two — so
/// a hole under the level's first occupied ordinal is inside the modelled
/// outcome and outside anything the model distinguishes. The distinction is the
/// message's, and this test is its own authority.
///
/// A level holding only ordinal 5, asked for [`Ordinal::FIRST`], has nothing
/// below the request. [`Ordinal::FIRST`] is not a floor the library enforces —
/// density is preserved and never established — so this is not an unreachable
/// arrangement, and the interior-gap sentence would be false on it.
#[test]
fn inserting_below_the_first_occupied_ordinal_claims_no_lower_occupant() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(5, 1, Status::Draft, "fifth"));
    let snapshot = builder.finish();

    let refused = refusal(insert(
        &snapshot,
        Target::Root,
        Ordinal::FIRST,
        NewEntry::empty(draft("underneath")),
    ));
    assert_eq!(
        refused,
        Refusal::NoOccupantAtOrdinal {
            ordinal: Ordinal::FIRST,
            occupied: Some((Ordinal::new(5), Ordinal::new(5))),
        },
        "one occupied ordinal is both ends of the span"
    );
    let said = refused.to_string();
    assert!(
        !said.contains("something below it"),
        "nothing sits below ordinal 1 here, so the refusal must not say so: {said}"
    );
    assert!(
        said.contains("by hand") && !said.contains("`append`'s job"),
        "the conclusion is still the hole's — `append` would take ordinal 6, \
         not this one: {said}"
    );
}

/// Discharges the same refusal on a level holding nothing at all, where every
/// ordinal is past the last sibling. `append` is the answer, and the message
/// gives it — this is the boundary between the two halves above, and the one a
/// `greatest` of `None` decides.
#[test]
fn inserting_into_an_empty_level_is_refused() {
    let mut builder = Builder::new();
    let root = builder.root();
    let empty = builder
        .add(root, module(1, 1, "empty"))
        .expect("a module is a node");
    builder.add(empty, overview());
    let snapshot = builder.finish();

    let refused = refusal(insert(
        &snapshot,
        Target::Key(Key::new(1)),
        Ordinal::FIRST,
        NewEntry::empty(draft("first")),
    ));
    assert_eq!(
        refused,
        Refusal::NoOccupantAtOrdinal {
            ordinal: Ordinal::FIRST,
            occupied: None,
        },
        "a level holding only its distinguished child holds no ordinal at all"
    );
    assert!(refused.to_string().contains("`append`'s job"));
}

/// Discharges `wit_refusedTargetMissing` and `wit_refusedTargetNotNode` for
/// `insert`: the same two refusals `append` has, because `insert` resolves its
/// target the same way. A leaf is a regular file and holds nothing.
#[test]
fn an_inserts_target_must_be_a_node_that_exists() {
    let snapshot = documents_tree();
    assert_eq!(
        refusal(insert(
            &snapshot,
            Target::Key(Key::new(99)),
            Ordinal::FIRST,
            NewEntry::empty(draft("nowhere")),
        )),
        Refusal::TargetMissing { key: Key::new(99) }
    );
    assert_eq!(
        refusal(insert(
            &snapshot,
            Target::Key(Key::new(1)),
            Ordinal::FIRST,
            NewEntry::empty(draft("inside-a-file")),
        )),
        Refusal::TargetNotNode {
            key: Key::new(1),
            species: Species::Leaf,
        }
    );
}

/// Discharges no model claim, and cannot: content is outside both models by
/// design. The refusal is the library's own, and it belongs to `insert` for the
/// same reason it belongs to `append` — a node is a directory and has nowhere to
/// hold bytes, so supplying some is a refusal rather than a silent discard.
#[test]
fn bytes_for_a_node_are_refused_by_insert_too() {
    assert_eq!(
        refusal(insert(
            &documents_tree(),
            Target::Root,
            Ordinal::new(1),
            NewEntry::new(topic("a-module"), b"where would these go?".to_vec()),
        )),
        Refusal::ContentForANode
    );
}

/// Discharges no model claim, for the reason the `append` exhaustion tests give:
/// an integer in either model is unbounded, so neither can pose exhaustion at
/// all. Here it is the **shift** that has nowhere to go — the greatest ordinal
/// in the level is the greatest an ordinal can be, so the first rename the plan
/// would build is impossible. Refused before any effect exists, rather than
/// wrapped onto an ordinal another entry holds.
#[test]
fn a_shift_that_would_overflow_the_ordinal_is_refused() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(u32::MAX, 1, Status::Draft, "the-last-place"));
    let snapshot = builder.finish();

    assert_eq!(
        refusal(insert(
            &snapshot,
            Target::Root,
            Ordinal::new(u32::MAX),
            NewEntry::empty(draft("before-the-last")),
        )),
        Refusal::OrdinalsExhausted
    );
}

/// The other exhaustion, and the other counter: the level has room to shift and
/// the **tree** has no fresh key. Discharges no model claim, for the same
/// reason.
#[test]
fn an_insert_with_no_fresh_key_is_refused() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(1, u32::MAX, Status::Draft, "the-last-key"));
    let snapshot = builder.finish();

    assert_eq!(
        refusal(insert(
            &snapshot,
            Target::Root,
            Ordinal::FIRST,
            NewEntry::empty(draft("next"))
        )),
        Refusal::KeysExhausted
    );
}

/// Discharges `wit_shiftOrderRefusesTheInsert`, which the model reaches **only**
/// under lowest-first — so under the order this library runs, the insert on that
/// same tree *succeeds*. That is the second payoff of the ordering rule, and the
/// one that needs a corrupted tree to see at all.
///
/// Two siblings sharing a key **and** its parts at adjacent ordinals, which
/// `cp 01-foo-i5.md 02-foo-i5.md` produces and the library never checks for. It
/// is the only tree shape in which a sibling shift can collide, and highest-first
/// does not: the pair is `src/plan/tests.rs`'s
/// `a_plan_is_folded_through_the_snapshot_and_not_checked_against_it`, here
/// reached through the operation that builds the plan for real rather than one
/// written by hand.
#[test]
fn an_insert_on_a_tree_with_a_cloned_sibling_still_proceeds() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(1, 5, Status::Draft, "foo"));
    builder.add(root, lesson(2, 5, Status::Draft, "foo"));
    let snapshot = builder.finish();

    let plan = plan(insert(
        &snapshot,
        Target::Root,
        Ordinal::FIRST,
        NewEntry::empty(draft("wedge")),
    ));
    assert_eq!(
        plan.effects()
            .iter()
            .map(|effect| effect.name().to_string())
            .collect::<Vec<_>>(),
        [
            "03-draft-foo-i5.md",
            "02-draft-foo-i5.md",
            "01-draft-wedge-i6.md",
        ]
    );
}

// ===========================================================================
// promote
// ===========================================================================

/// The names a plan places, in the plan's own order, with the level each lands
/// in and whether it is a create or a move.
///
/// A promotion is the first plan whose *shape* is the claim — create, move,
/// create — so this reads all three off the value rather than asserting the
/// names alone.
fn landings_by_kind(plan: &Plan<SyllabusName>) -> Vec<(&'static str, Level, String)> {
    plan.effects()
        .iter()
        .map(|effect| match effect {
            Effect::Create { at, name, .. } => ("create", *at, name.to_string()),
            Effect::MoveTo { to, name, .. } => ("move", *to, name.to_string()),
        })
        .collect()
}

/// Discharges `inv_promoteKeepsIdentity`, on the half the algebra decides: the
/// node carries the promoted leaf's **own** ordinal and its **own** key, and the
/// leaf itself is moved in as the distinguished child.
///
/// The document's lesson at ordinal 1, key 1 becomes the module at ordinal 1,
/// key 1 — *the entry that was a leaf is the node*, so every reference to it by
/// key still resolves.
#[test]
fn a_promotion_keeps_the_leafs_own_ordinal_and_key() {
    let snapshot = documents_tree();
    let leaf = snapshot
        .by_key(Key::new(1))
        .expect("the orientation lesson");
    let plan = plan(promote(&snapshot, Key::new(1), topic("orientation"), None));

    assert_eq!(
        landings_by_kind(&plan),
        [
            ("create", Level::Root, "01-orientation-i1".to_string()),
            ("move", Level::Created(0), "OVERVIEW.md".to_string()),
        ]
    );
    let Effect::MoveTo { entry, .. } = &plan.effects()[1] else {
        panic!("the second effect moves the leaf");
    };
    assert_eq!(
        *entry,
        leaf.index(),
        "and it is the leaf's own file that moves, which is how its bytes move \
         without the library ever reading them"
    );
}

/// Discharges `wit_promoteTransientlyDuplicatesAKey` and
/// `wit_promoteTransientlyDuplicatesAnOrdinal`, which the model **reaches**
/// rather than excludes — and which `inv_ordinalsDistinctThroughout` exempts by
/// name, in the one place it exempts anything.
///
/// A test cannot observe an interruption. What it can do — because the plan is a
/// value — is read the state a crash could stop at off the plan: after effect
/// one and before effect two, the node exists and the leaf has not moved, so
/// both are in the same level carrying the same ordinal and the same key. There
/// is no ordering that avoids it, which is the other half of this test: reversing
/// the two effects is not an alternative plan but a plan that cannot run, since
/// the level the move lands in is the one the create makes.
#[test]
fn a_promotion_passes_through_a_state_where_the_leaf_and_the_node_share_an_ordinal_and_a_key() {
    let snapshot = documents_tree();
    let leaf = snapshot
        .by_key(Key::new(1))
        .expect("the orientation lesson");
    let was = leaf.triple().expect("a positioned entry");
    let plan = plan(promote(&snapshot, Key::new(1), topic("orientation"), None));

    let Effect::Create { at, name, .. } = &plan.effects()[0] else {
        panic!("a promotion creates the node first");
    };
    let node = name.triple().expect("a composed name is positioned");
    assert_eq!(*at, Level::Root, "in the level the leaf sits in");
    assert_eq!(node.ordinal, was.ordinal);
    assert_eq!(node.key, was.key);
    assert_eq!(
        leaf.container().entry(),
        None,
        "so after effect one, and before effect two, this level holds two \
         entries at ordinal {} carrying key {} — the leaf and the node about to \
         hold it",
        was.ordinal,
        was.key
    );

    let Effect::MoveTo { to, .. } = &plan.effects()[1] else {
        panic!("a promotion moves the leaf second");
    };
    assert_eq!(
        *to,
        Level::Created(0),
        "and the order is forced: the move lands in the level the create makes, \
         so there is no plan with these two effects the other way round"
    );
}

/// Discharges `wit_promoteWithChild`: the optional first child lands **inside**
/// the new node, at [`Ordinal::FIRST`], in the same unit as the promotion — the
/// model's `compose(1, freshKey(f), …)` at `Level::Created(0)`.
#[test]
fn a_promotion_can_create_a_first_child_in_the_same_unit() {
    let snapshot = documents_tree();
    let plan = plan(promote(
        &snapshot,
        Key::new(1),
        topic("orientation"),
        Some(NewEntry::new(draft("welcome"), b"welcome\n".to_vec())),
    ));

    assert_eq!(
        landings_by_kind(&plan),
        [
            ("create", Level::Root, "01-orientation-i1".to_string()),
            ("move", Level::Created(0), "OVERVIEW.md".to_string()),
            (
                "create",
                Level::Created(0),
                "01-draft-welcome-i10.md".to_string()
            ),
        ]
    );
}

/// Discharges `inv_freshKeysAreFresh` on the case the model's own comment is
/// written about: *the property is about allocation, not creation*.
///
/// The node is a newly created object carrying key 1, which entry 1 already had
/// — read as *no newly created object carries a key seen before*, the claim is
/// simply false, and the model says so at length. Nothing was **allocated** for
/// it. The first child is what allocates, and it takes `freshKey` over the whole
/// tree, which is 10 and not 2: a promotion that had spent a key on the node
/// would give the child 11.
#[test]
fn a_promotion_allocates_a_key_for_its_child_and_none_for_the_node() {
    let snapshot = documents_tree();
    let plan = plan(promote(
        &snapshot,
        Key::new(1),
        topic("orientation"),
        Some(NewEntry::empty(draft("welcome"))),
    ));
    let keys: Vec<u32> = plan
        .effects()
        .iter()
        .filter_map(|effect| match effect {
            Effect::Create { name, .. } => Some(
                name.triple()
                    .expect("a composed name is positioned")
                    .key
                    .get(),
            ),
            Effect::MoveTo { .. } => None,
        })
        .collect();
    assert_eq!(keys, [1, 10]);
}

/// Discharges `inv_promoteKeepsIdentity`'s *nothing else moved* clause at the
/// level below the root: the node is created in the leaf's **own** container,
/// which is a node named by key, and no sibling is named by the plan at all.
///
/// A promotion is not an insert. Nothing shifts, because the node takes the
/// ordinal the leaf is vacating in the same breath.
#[test]
fn a_promotion_deeper_in_the_tree_names_only_the_leaf_and_its_own_level() {
    let snapshot = documents_tree();
    let algebra = snapshot.by_key(Key::new(2)).expect("the module").index();
    let plan = plan(promote(&snapshot, Key::new(6), topic("matrices"), None));

    assert_eq!(
        landings_by_kind(&plan),
        [
            (
                "create",
                Level::Entry(algebra),
                "02-matrices-i6".to_string()
            ),
            ("move", Level::Created(0), "OVERVIEW.md".to_string()),
        ],
        "the node lands in the module the lesson sat in, at the lesson's ordinal"
    );
}

/// Discharges `wit_refusedTargetMissing` for `promote`: an operation names its
/// target by key, and a key naming nothing is refused.
#[test]
fn promoting_a_key_that_names_nothing_is_refused() {
    assert_eq!(
        refusal(promote(
            &documents_tree(),
            Key::new(99),
            topic("nowhere"),
            None
        )),
        Refusal::TargetMissing { key: Key::new(99) }
    );
}

/// Discharges `wit_refusedPromoteNotLeaf`: a node is already a node.
///
/// The document says *a node is already a node, and a distinguished child has no
/// ordinal to carry across; both are refused* — and only the first half is
/// reachable, here and in the model alike, because a target is named by key and
/// a distinguished child has none. The refusal carries the species it actually
/// found, which is the only thing the predicate that selected it proves.
#[test]
fn promoting_a_node_is_refused() {
    let refused = refusal(promote(
        &documents_tree(),
        Key::new(2),
        topic("linear-algebra"),
        None,
    ));
    assert_eq!(
        refused,
        Refusal::PromoteNotLeaf {
            key: Key::new(2),
            species: Species::Node,
        }
    );
    assert!(
        refused.to_string().contains("already"),
        "a refusal says what to do about it: {refused}"
    );
}

/// **No model claim, and none possible.** The model's `resolve` is
/// `idsWithKey`, which filters on `isPositioned`, so the distinguished child is
/// unreachable there for exactly the reason it is unreachable here — and a
/// refusal no argument can reach has no witness to reach it.
///
/// This is the control on that reading: the document's example tree holds two
/// distinguished children and `by_key` answers with neither, whatever key it is
/// asked for. So the document's second clause describes a case that cannot
/// arise, and `Refusal::PromoteNotLeaf` says so where a reader will meet it.
#[test]
fn a_distinguished_child_cannot_be_named_by_key_at_all() {
    let snapshot = documents_tree();
    assert!(
        snapshot
            .walk()
            .any(|entry| entry.species() == Species::Distinguished),
        "the document's tree holds distinguished children"
    );
    for key in 0..12 {
        if let Sought::Match(found) = snapshot.by_key(Key::new(key)) {
            assert_ne!(
                found.species(),
                Species::Distinguished,
                "a distinguished child carries no key, so no key can name one"
            );
        }
    }
}

/// Discharges `wit_refusedNoDistinguishedChild`, and with it the whole content
/// of the `no_distinguished` instance: a domain whose `distinguished()` is
/// `None` cannot promote anything, because the leaf's content would have nowhere
/// to go.
///
/// Refused outright rather than guessed at. The alternatives are discarding the
/// bytes and inventing a name the domain never declared, and the library will do
/// neither.
#[test]
fn promoting_in_a_domain_with_no_distinguished_child_is_refused() {
    let snapshot = contentless_tree();
    let decision = promote(
        &snapshot,
        Key::new(1),
        Parts::module(Label::new("orientation").expect("a label")),
        None,
    );
    let Decision::Refuse(refused) = decision else {
        panic!("a domain with nowhere to put the content cannot promote");
    };
    assert_eq!(
        refused,
        Refusal::NoDistinguishedChild {
            promoting: Some(Key::new(1))
        }
    );
    assert!(
        refused.to_string().contains("nowhere to go"),
        "a refusal says why, and what to do: {refused}"
    );
}

/// Discharges `wit_refusedPromotePartsNotNode`: the parts come from the caller,
/// so the library checks what it was handed. Parts that make a leaf would name a
/// regular file, and a promotion has to name a directory.
#[test]
fn promoting_with_parts_that_make_a_leaf_is_refused() {
    let refused = refusal(promote(
        &documents_tree(),
        Key::new(1),
        draft("orientation"),
        None,
    ));
    assert_eq!(refused, Refusal::PromotePartsNotNode { key: Key::new(1) });
}

/// **No model claim** — the model has no notion of *first*, since a refusal is
/// an outcome and `planPromote` is a chain of `if`s. What it does have is the
/// chain's own order, and this transcribes it: a target that is both a node and
/// in a domain with no distinguished child reports **not a leaf**, because that
/// is the branch `planPromote` reaches first.
///
/// The order is observable, so it is transcribed rather than reinvented. It is
/// also the useful one: *this domain can never promote anything* is a better
/// answer than *that is a node* only when the call could otherwise have worked.
#[test]
fn the_refusals_are_reported_in_the_models_own_order() {
    let mut builder = Builder::new();
    let root = builder.root();
    let module = builder
        .add(
            root,
            Contentless::compose(
                Ordinal::FIRST,
                Key::new(1),
                Parts::module(Label::new("linear-algebra").expect("a label")),
            ),
        )
        .expect("a module is a node");
    let _ = module;
    let snapshot = builder.finish();

    let Decision::Refuse(refused) = promote(
        &snapshot,
        Key::new(1),
        Parts::module(Label::new("linear-algebra").expect("a label")),
        None,
    ) else {
        panic!("a node is not a leaf");
    };
    assert_eq!(
        refused,
        Refusal::PromoteNotLeaf {
            key: Key::new(1),
            species: Species::Node,
        },
        "both refusals are true here, and the first branch is the one reported"
    );
}

/// **No model claim, and none possible**: content is unmodelled in both models
/// by design. A node is a directory and has nowhere to hold bytes, and the
/// refusal belongs to *every operation that creates an entry* rather than to a
/// list of them — `docs/formalism-findings.md` entry 012 is where that list went
/// stale, and a promotion carrying a first child creates an entry.
#[test]
fn bytes_for_a_first_child_that_makes_a_node_are_refused() {
    assert_eq!(
        refusal(promote(
            &documents_tree(),
            Key::new(1),
            topic("orientation"),
            Some(NewEntry::new(topic("nested"), b"bytes".to_vec())),
        )),
        Refusal::ContentForANode
    );
}

/// **No model claim**: an integer in either model is unbounded, so neither can
/// pose exhaustion. The node allocates nothing, so a promotion *without* a first
/// child succeeds on this tree; the child is what needs `max + 1`, and there is
/// none.
#[test]
fn a_promotion_whose_child_has_no_fresh_key_is_refused_and_one_without_a_child_is_not() {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(1, u32::MAX, Status::Draft, "the-last-key"));
    let snapshot = builder.finish();

    assert_eq!(
        refusal(promote(
            &snapshot,
            Key::new(u32::MAX),
            topic("the-last-key"),
            Some(NewEntry::empty(draft("child"))),
        )),
        Refusal::KeysExhausted
    );
    let plan = plan(promote(
        &snapshot,
        Key::new(u32::MAX),
        topic("the-last-key"),
        None,
    ));
    assert_eq!(
        plan.effects().len(),
        2,
        "the node carries the leaf's own key, so it allocates nothing"
    );
}

// ---------------------------------------------------------------------------
// rewrite
//
// One effect, and every test here reads it off the plan. What makes the
// operation worth its own section is not its size but its two edges: the
// species that must not change, and the no-op that must not be refused.
// ---------------------------------------------------------------------------

fn published(label: &str) -> Parts {
    Parts::lesson(
        Status::Published,
        Label::new(label).expect("a well-formed label"),
    )
}

/// The one effect a rewrite plans, as `(moved entry, level, rendered name)`.
fn only_move(decision: Decision<SyllabusName>) -> (usize, Level, String) {
    let plan = plan(decision);
    assert_eq!(
        plan.effects().len(),
        1,
        "a rewrite is one rename and nothing else"
    );
    match &plan.effects()[0] {
        Effect::MoveTo { entry, to, name } => (*entry, *to, name.to_string()),
        Effect::Create { .. } => panic!("a rewrite creates nothing"),
    }
}

/// Discharges `inv_rewriteKeepsPlace` on the half the algebra decides: the new
/// name carries the entry's **own** ordinal and its **own** key, in the level it
/// already sits in, and only the parts moved.
///
/// The document's draft assessment at ordinal 3, key 9 becomes published. It
/// stays at ordinal 3 and key 9 — which is what makes this the general form of
/// *mark this entry*, and, with no way to remove an entry, how a domain retires
/// one.
#[test]
fn a_rewrite_keeps_the_ordinal_the_key_and_the_level() {
    let snapshot = documents_tree();
    let entry = snapshot.by_key(Key::new(9)).expect("the draft assessment");

    assert_eq!(
        only_move(rewrite(&snapshot, Key::new(9), published("assessment"))),
        (
            entry.index(),
            Level::Root,
            "03-published-assessment-i9.md".to_string()
        )
    );
}

/// Discharges `inv_rewriteKeepsPlace`'s other half — *nothing else changed* —
/// on a nested entry, where a level that is not the root can be got wrong.
///
/// The plan names one entry, and the level it names is the module the entry is
/// already in. A rewrite that landed its rename in the tree root would move the
/// entry between levels, which is the one thing this operation is not.
#[test]
fn a_rewrite_names_only_the_entry_it_changes_in_the_level_it_already_sits_in() {
    let snapshot = documents_tree();
    let matrices = snapshot.by_key(Key::new(6)).expect("the draft matrices");
    let module = snapshot
        .by_key(Key::new(2))
        .expect("the linear algebra module");

    assert_eq!(
        only_move(rewrite(&snapshot, Key::new(6), published("matrices"))),
        (
            matrices.index(),
            Level::Entry(module.index()),
            "02-published-matrices-i6.md".to_string()
        )
    );
}

/// Discharges `wit_rewriteToSameParts`: a rewrite to the parts an entry already
/// carries is a rename onto its own path and it must **succeed**.
///
/// This is what [`Effect::mover`] exists for — occupancy excludes the object
/// being moved, so the destination the plan computes is not found taken by the
/// very entry that is moving to it. Nothing in `rewrite` says so; it falls out
/// of the guard, which is why this test asserts a plan rather than a branch.
///
/// The interpreter carries the same exclusion across the boundary by
/// short-circuiting a same-path rename; `tests/rewriting_on_disk.rs` is where
/// that half is observable.
#[test]
fn rewriting_to_the_parts_an_entry_already_carries_is_not_refused() {
    let snapshot = documents_tree();
    let entry = snapshot.by_key(Key::new(9)).expect("the draft assessment");

    assert_eq!(
        only_move(rewrite(&snapshot, Key::new(9), draft("assessment"))),
        (entry.index(), Level::Root, entry.name().to_string(),),
        "the name it plans is the name it has, and the guard does not call that \
         a collision"
    );
}

/// Discharges `wit_refusedRewriteSpeciesChange`: parts implying a different
/// species are refused, because a regular file cannot be renamed into a
/// directory.
///
/// Both directions, in one test, because the refusal carries **one** species and
/// the two calls are what shows it is the entry's rather than the parts'. With
/// exactly two positioned species, *the entry is a leaf* already says *the parts
/// make a node*; carrying both would be two fields that can disagree.
#[test]
fn rewriting_across_the_species_is_refused_in_both_directions() {
    let snapshot = documents_tree();

    assert_eq!(
        refusal(rewrite(&snapshot, Key::new(9), topic("assessment"))),
        Refusal::RewriteSpeciesChange {
            key: Key::new(9),
            species: PositionedSpecies::Leaf,
        },
        "a lesson asked for a module's parts"
    );
    assert_eq!(
        refusal(rewrite(&snapshot, Key::new(2), draft("linear-algebra"))),
        Refusal::RewriteSpeciesChange {
            key: Key::new(2),
            species: PositionedSpecies::Node,
        },
        "and a module asked for a lesson's"
    );
}

/// **No model claim** — a refusal's message is outside both models. It is a
/// control on the advice rather than on the check: the two directions are not
/// symmetric, so one message would have to be wrong for one of them.
///
/// A leaf can become a node, by `promote`, which moves its content rather than
/// discarding it — so that is the advice. A node cannot become a leaf at all:
/// its children would have nowhere to go, and no operation removes one. Advice
/// that named `promote` in both directions would fail when taken in one of them,
/// which is `docs/formalism-findings.md` entry 013's habit applied to a message
/// that offers a remedy rather than an explanation.
#[test]
fn the_species_refusals_advice_differs_by_direction() {
    let snapshot = documents_tree();

    let leaf = refusal(rewrite(&snapshot, Key::new(9), topic("assessment"))).to_string();
    assert!(
        leaf.contains("`promote`"),
        "a leaf has somewhere to go, and the message says where: {leaf}"
    );

    let node = refusal(rewrite(&snapshot, Key::new(2), draft("linear-algebra"))).to_string();
    assert!(
        !node.contains("`promote`"),
        "a node has nowhere to go, and offering `promote` would be advice that \
         fails when taken: {node}"
    );
    assert!(
        node.contains("nowhere to go"),
        "so the message says why instead: {node}"
    );
}

/// Discharges `wit_refusedTargetMissing` on this operation: `resolve(f,
/// ByKey(k))` answering nothing is the model's first branch of `planRewrite`,
/// and it is reported before the species is looked at.
#[test]
fn rewriting_a_key_that_names_nothing_is_refused() {
    assert_eq!(
        refusal(rewrite(&documents_tree(), Key::new(404), draft("nothing"))),
        Refusal::TargetMissing { key: Key::new(404) }
    );
}

/// **No model claim, and none needed**: this is structural in both. A
/// distinguished child carries no key, `by_key` yields positioned entries only,
/// and the model's `idsWithKey` filters on `isPositioned` — so neither can be
/// handed one, and `rewrite` has no *not a positioned entry* refusal to reach.
///
/// The control is that the example tree holds two distinguished children and no
/// key names either. It is the same shape as `promote`'s, and it is here because
/// `rewrite` is the operation with no species precondition at all: a reader
/// could reasonably expect the distinguished child to arrive and be refused, and
/// what actually happens is that it never arrives.
#[test]
fn a_distinguished_child_cannot_be_rewritten_because_it_cannot_be_named() {
    let snapshot = documents_tree();
    let distinguished = snapshot
        .walk()
        .filter(|entry| entry.species() == Species::Distinguished)
        .count();
    assert_eq!(distinguished, 2, "the root's and the module's");

    for key in 0..=12 {
        if let Sought::Match(entry) = snapshot.by_key(Key::new(key)) {
            assert_ne!(
                entry.species(),
                Species::Distinguished,
                "no key answers with a distinguished child"
            );
        }
    }
}

/// **No model claim**: `Parts` is an `int` in the model, so *what changed* is
/// not a question it can pose. What it can be checked against is the seam — a
/// rewrite reads the parts through [`EntryName::positioned_species`] and through
/// nothing else, so parts differing in every other respect are placed verbatim.
///
/// The label moves here as well as the attribute, and the plan carries it
/// without comment. Anything that inspected the parts further would be the seam
/// leaking.
#[test]
fn a_rewrite_places_whatever_parts_it_is_handed_once_the_species_agrees() {
    let snapshot = documents_tree();

    assert_eq!(
        only_move(rewrite(&snapshot, Key::new(9), published("something-else"))).2,
        "03-published-something-else-i9.md",
        "the label is as opaque to the library as the attribute is"
    );
}
