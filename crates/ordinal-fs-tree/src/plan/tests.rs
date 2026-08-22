//! The plan's own check: is this plan applicable, folded through the snapshot
//! it was built from?
//!
//! Plans are built here by hand rather than by an operation, which is the only
//! way to reach the interesting cases: `append` composes names carrying keys no
//! entry in the tree has, so nothing it builds can ever meet an occupied
//! destination. Every plan below is one a later leaf's operation will build for
//! real — the shift pair is exactly what `insert` produces — and building them
//! now is what keeps this leaf's machinery from being accepted on the strength
//! of a case that cannot fail.
//!
//! Every test here names the model claim it discharges, or says it has none.

use super::{Decision, Effect, Level, Plan, Refusal};
use crate::fixtures::{documents_tree, lesson, module};
use crate::reference::{Parts, Status, SyllabusName};
use crate::snapshot::{Builder, Snapshot};
use crate::{EntryName, Key, Ordinal};

/// Two siblings sharing a key **and** its parts at adjacent ordinals: what
/// `cp 01-foo-i5.md 02-foo-i5.md` produces.
///
/// `operations.qnt`'s `handEditClone`, and its `corrupted` instance's whole
/// subject. It is the only tree shape in which a sibling shift can collide,
/// which is what makes the shift order load-bearing — and, below, the only
/// shape in which the difference between folding a plan and checking it against
/// the snapshot is observable at all.
fn cloned_sibling_tree() -> Snapshot<SyllabusName> {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, lesson(1, 5, Status::Draft, "foo"));
    builder.add(root, lesson(2, 5, Status::Draft, "foo"));
    builder.finish()
}

/// The arena index of the entry at an ordinal in the top level.
fn at_ordinal(snapshot: &Snapshot<SyllabusName>, ordinal: u32) -> usize {
    snapshot
        .root()
        .positioned()
        .find(|entry| entry.ordinal() == Some(Ordinal::new(ordinal)))
        .expect("the fixture has an entry there")
        .index()
}

fn refusal<N: EntryName>(decision: Decision<N>) -> Refusal {
    match decision {
        Decision::Refuse(refusal) => refusal,
        Decision::Proceed(plan) => {
            panic!(
                "expected a refusal, got a plan of {} effects",
                plan.effects().len()
            )
        }
    }
}

fn proceeds<N: EntryName>(decision: Decision<N>) -> Plan<N> {
    match decision {
        Decision::Proceed(plan) => plan,
        Decision::Refuse(refusal) => panic!("expected a plan, got {refusal:?}"),
    }
}

/// Discharges `operations.qnt`'s `planIsApplicable` — *the algebra folds the
/// plan through the snapshot, so it meets each destination in the state the
/// interpreter will meet it* — and it is the test that fails if that is ever
/// rewritten as *check every destination against the snapshot*.
///
/// The plan is the shift half of an `insert` at ordinal 1, run highest-first, on
/// the one tree where the difference shows. Its second effect renames the entry
/// at ordinal 1 onto the name the entry at ordinal 2 **had**, which the first
/// effect has just vacated. Checked against the snapshot that name is occupied
/// and this correct plan is refused; folded, it is free.
///
/// `docs/formalism-findings.md` entry 003 records that the architecture document
/// had not made this decision until the model forced it, and that getting it
/// wrong makes the highest-first ordering rule *vacuous* — both orders are then
/// refused in exactly the same cases — while nothing fails to tell you.
#[test]
fn a_plan_is_folded_through_the_snapshot_and_not_checked_against_it() {
    let snapshot = cloned_sibling_tree();
    let (first, second) = (at_ordinal(&snapshot, 1), at_ordinal(&snapshot, 2));
    let plan = Plan::of(vec![
        // Highest-first: vacate each destination before it is needed.
        Effect::MoveTo {
            entry: second,
            to: Level::Root,
            name: lesson(3, 5, Status::Draft, "foo"),
        },
        Effect::MoveTo {
            entry: first,
            to: Level::Root,
            name: lesson(2, 5, Status::Draft, "foo"),
        },
    ]);
    assert_eq!(proceeds(plan.guarded(&snapshot)).effects().len(), 2);
}

/// Discharges `operations.qnt`'s `wit_shiftOrderRefusesTheInsert`, which is
/// reachable **only** under lowest-first and only on a tree carrying two
/// siblings that share a key and its parts. The same two renames as the test
/// above, in the other order: the first of them wants the name the second still
/// holds.
///
/// The pair is what makes either test mean anything. A fold that never refuses
/// anything would pass the test above on its own.
#[test]
fn the_same_two_renames_the_other_way_round_are_refused() {
    let snapshot = cloned_sibling_tree();
    let (first, second) = (at_ordinal(&snapshot, 1), at_ordinal(&snapshot, 2));
    let plan = Plan::of(vec![
        Effect::MoveTo {
            entry: first,
            to: Level::Root,
            name: lesson(2, 5, Status::Draft, "foo"),
        },
        Effect::MoveTo {
            entry: second,
            to: Level::Root,
            name: lesson(3, 5, Status::Draft, "foo"),
        },
    ]);
    assert_eq!(
        refusal(plan.guarded(&snapshot)),
        Refusal::DestinationOccupied {
            ordinal: Some(Ordinal::new(2)),
            key: Some(Key::new(5)),
        }
    );
}

/// Discharges `RefusedDestinationOccupied` against the snapshot itself: a plan
/// whose destination is taken by an entry that was already there and is not
/// going anywhere.
#[test]
fn a_destination_an_existing_entry_holds_is_refused() {
    let snapshot = documents_tree();
    let plan = Plan::of(vec![Effect::Create {
        at: Level::Root,
        name: lesson(1, 1, Status::Published, "orientation"),
        content: Vec::new(),
    }]);
    assert!(matches!(
        refusal(plan.guarded(&snapshot)),
        Refusal::DestinationOccupied { .. }
    ));
}

/// Discharges the same claim on the case only a *plan* can produce: nothing in
/// the tree is in the way, and the plan collides with itself. This is what the
/// interpreter's own exclusive create would otherwise be left to catch — and
/// catching it here is what makes that check unreachable in ordinary use.
#[test]
fn a_plan_that_collides_with_itself_is_refused() {
    let snapshot = documents_tree();
    let twice = || Effect::Create {
        at: Level::Root,
        name: lesson(4, 10, Status::Draft, "duplicated"),
        content: Vec::new(),
    };
    assert!(matches!(
        refusal(Plan::of(vec![twice(), twice()]).guarded(&snapshot)),
        Refusal::DestinationOccupied { .. }
    ));
}

/// Discharges no model claim of its own — it is the Rust reading of the model's
/// `Create({ id: nodeId, … })` followed by `MoveTo({ parent: nodeId, … })`,
/// which is the shape `planPromote` builds. A level this plan creates holds
/// nothing, so a name placed in it can only be blocked by this same plan.
#[test]
fn a_level_the_plan_creates_starts_empty() {
    let snapshot = documents_tree();
    let plan = Plan::of(vec![
        Effect::Create {
            at: Level::Root,
            name: module(4, 10, "topology"),
            content: Vec::new(),
        },
        // The name `01-published-orientation-i1.md` is taken at the root and
        // free here: the level is new.
        Effect::Create {
            at: Level::Created(0),
            name: lesson(1, 1, Status::Published, "orientation"),
            content: Vec::new(),
        },
    ]);
    assert_eq!(proceeds(plan.guarded(&snapshot)).effects().len(), 2);
}

/// The other half of that, and it discharges no model claim of its own for the
/// same reason: a created level is not a free-for-all, and two effects placing
/// one name in it collide like anywhere else.
#[test]
fn a_level_the_plan_creates_still_refuses_a_collision() {
    let snapshot = documents_tree();
    let leaf = || Effect::Create {
        at: Level::Created(0),
        name: lesson(1, 11, Status::Draft, "same"),
        content: Vec::new(),
    };
    let plan = Plan::of(vec![
        Effect::Create {
            at: Level::Root,
            name: module(4, 10, "topology"),
            content: Vec::new(),
        },
        leaf(),
        leaf(),
    ]);
    assert!(matches!(
        refusal(plan.guarded(&snapshot)),
        Refusal::DestinationOccupied { .. }
    ));
}

/// Discharges `operations.qnt`'s `occupied`, whose comment is the reason:
/// *excluding the mover is not decoration — a rewrite whose new parts equal the
/// old is a rename onto itself, and without the exclusion the library would
/// refuse its own no-op.* `wit_rewriteToSameParts` is the witness that it must
/// succeed.
#[test]
fn an_entry_does_not_occupy_its_own_destination() {
    let snapshot = documents_tree();
    let orientation = at_ordinal(&snapshot, 1);
    let plan = Plan::of(vec![Effect::MoveTo {
        entry: orientation,
        to: Level::Root,
        name: lesson(1, 1, Status::Published, "orientation"),
    }]);
    assert_eq!(proceeds(plan.guarded(&snapshot)).effects().len(), 1);
}

/// Discharges no model claim — names are values in both models, and this is
/// about the Rust encoding of one. Occupancy compares **views** and never
/// renderings, because the library holds no strings; the two are the same
/// question only because the grammar is canonical, which is the obligation
/// `structure.als`'s `witness_two_filenames_name_one_entry` exists to picture.
/// This is the control that the comparison is by value: a name equal in every
/// field to one already there collides, though it is a different `SyllabusName`
/// value built a different way.
#[test]
fn occupancy_compares_names_and_not_the_objects_holding_them() {
    let snapshot = documents_tree();
    let rebuilt = SyllabusName::Positioned {
        ordinal: Ordinal::new(1),
        key: Key::new(1),
        parts: Parts::lesson(
            Status::Published,
            crate::reference::Label::new("orientation").expect("a label"),
        ),
    };
    let plan = Plan::of(vec![Effect::Create {
        at: Level::Root,
        name: rebuilt,
        content: Vec::new(),
    }]);
    assert!(matches!(
        refusal(plan.guarded(&snapshot)),
        Refusal::DestinationOccupied { .. }
    ));
}

/// Discharges no model claim. A plan of no effects is applicable, which is what
/// makes an `append_many` of nothing a success that changes nothing rather than
/// a special case anywhere.
#[test]
fn an_empty_plan_proceeds() {
    let snapshot = documents_tree();
    assert!(
        proceeds(Plan::<SyllabusName>::of(Vec::new()).guarded(&snapshot))
            .effects()
            .is_empty()
    );
}
