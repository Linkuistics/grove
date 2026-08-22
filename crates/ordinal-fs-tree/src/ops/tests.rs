//! `append`, `append_many` and `insert`, as the algebra decides them: no
//! filesystem, no directory, and every answer a pure function of a snapshot
//! built by hand.
//!
//! Every test here names the model claim it discharges, or says it has none.

use super::{append, append_many, insert, NewEntry, Target};
use crate::fixtures::{documents_tree, empty_tree, lesson, module, overview};
use crate::plan::{Decision, Effect, Level, Plan, Refusal};
use crate::reference::{Label, Parts, Status, SyllabusName};
use crate::snapshot::{Builder, Snapshot};
use crate::{EntryNameExt, Key, Ordinal, Species};

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
