//! The interpreter, against real directories.
//!
//! These are unit tests rather than tests in `tests/`, and for the same reason
//! the snapshot's are: the seam that makes an effect fail on demand is
//! **internal**, and it must stay internal — a second public seam would
//! contradict `docs/adr/entry-name-is-the-only-seam.md`. Atomicity is not
//! observable without it. *After a mutation returns an error, either every
//! effect landed or none did* needs an error, and every effect an `append_many`
//! builds succeeds on a healthy filesystem.
//!
//! Plans are also built by hand here, which is the only way to reach the
//! interpreter's rename path before `insert` and `promote` exist: the rollback
//! is *shared*, so the half of it that puts a moved entry back has to be
//! exercised by this leaf or by nothing.
//!
//! Every test here names the model claim it discharges, or says it has none.

use std::fs;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use super::Faults;
use crate::fixtures::{lesson, module, overview, Sneaky};
use crate::ops::{self, NewEntry, Target};
use crate::plan::{Decision, Effect, Level, Plan};
use crate::reference::{Label, Parts, Status, SyllabusName};
use crate::{EntryName, Error, Key, Ordinal, Report};

fn draft(label: &str) -> Parts {
    Parts::lesson(
        Status::Draft,
        Label::new(label).expect("a well-formed label"),
    )
}

/// A tree on disk holding two lessons, each with its own bytes.
fn two_lessons() -> (TempDir, PathBuf) {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = temporary.path().join("syllabus");
    fs::create_dir(&root).expect("creating the tree root");
    fs::write(root.join("01-draft-first-i1.md"), "first").expect("a fixture");
    fs::write(root.join("02-draft-second-i2.md"), "second").expect("a fixture");
    (temporary, root)
}

/// Every path under a directory, relative and sorted — the whole state of a
/// small tree, so that *nothing changed* is one assertion rather than several.
fn listing(root: &Path) -> Vec<String> {
    fn walk(directory: &Path, prefix: &str, out: &mut Vec<String>) {
        let mut names: Vec<_> = fs::read_dir(directory)
            .expect("a readable directory")
            .map(|entry| entry.expect("a readable entry").path())
            .collect();
        names.sort();
        for path in names {
            let name = path.file_name().expect("a named entry").to_string_lossy();
            let relative = format!("{prefix}{name}");
            if path.is_dir() {
                out.push(format!("{relative}/"));
                walk(&path, &format!("{relative}/"), out);
            } else {
                let content = fs::read_to_string(&path).unwrap_or_default();
                out.push(format!("{relative} = {content}"));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, "", &mut out);
    out
}

/// Run a decision under the guard, with the failure seam armed.
fn run(
    root: &Path,
    decide: impl FnOnce(&crate::Snapshot<SyllabusName>) -> Decision<SyllabusName>,
    faults: Faults,
) -> Result<Report<SyllabusName>, Error<SyllabusName>> {
    let guard = crate::fs::write::<SyllabusName>(root).expect("a well-formed tree");
    let decision = decide(guard.snapshot());
    guard.run(decision, faults)
}

/// Discharges `inv_atomicity`: *after a mutation returns an error, either every
/// effect landed or none did* — and `inv_rollbackRemovesOnlyItsOwn` with it,
/// since the two entries that were already there are still there afterwards.
///
/// A **multi-effect** plan, deliberately: a single-effect `append` that fails
/// leaves nothing behind whatever the interpreter does, so it cannot tell an
/// implementation that unwinds from one that does not. That is the whole reason
/// `append_many` is in this leaf.
#[test]
fn a_failure_part_way_through_a_run_leaves_the_tree_as_it_was() {
    let (_temporary, root) = two_lessons();
    let before = listing(&root);

    let failed = run(
        &root,
        |snapshot| {
            ops::append_many(
                snapshot,
                Target::Root,
                vec![
                    NewEntry::new(draft("third"), b"third".to_vec()),
                    NewEntry::new(draft("fourth"), b"fourth".to_vec()),
                    NewEntry::new(draft("fifth"), b"fifth".to_vec()),
                ],
            )
        },
        Faults::at_effect(1),
    )
    .expect_err("the seam failed the second of three effects");

    assert!(
        matches!(failed, Error::Failed { .. }),
        "the run unwound cleanly, so this is the atomic failure and not the other one: {failed:?}"
    );
    assert_eq!(
        listing(&root),
        before,
        "the first effect had landed and was removed again"
    );
}

/// Discharges `inv_rollbackRemovesOnlyItsOwn` on the case that distinguishes an
/// unwind from a cleanup: the failing run's own creations go, and the entries
/// that were there before — including one whose *name* the run never mentioned —
/// keep their bytes. A rollback that removed by level rather than by what it
/// created would pass the test above and fail this one.
#[test]
fn a_rollback_removes_what_the_run_created_and_nothing_else() {
    let (_temporary, root) = two_lessons();

    run(
        &root,
        |snapshot| {
            ops::append_many(
                snapshot,
                Target::Root,
                vec![
                    NewEntry::empty(draft("third")),
                    NewEntry::empty(draft("fourth")),
                ],
            )
        },
        Faults::at_effect(1),
    )
    .expect_err("the seam failed the second effect");

    assert_eq!(
        listing(&root),
        [
            "01-draft-first-i1.md = first".to_string(),
            "02-draft-second-i2.md = second".to_string(),
        ]
    );
}

/// Discharges `wit_partialRollbackLeavesNeitherState`: *a rollback that did not
/// finish — the tree is then neither the state the operation found nor the one
/// it intended*. The model's `rollback_fails` instance is the only one that does
/// not claim key uniqueness at rest, because this is what breaks it.
///
/// The bound has to be **in the type**, not only in the prose: a consumer that
/// cannot tell this from a clean unwind has been promised something the library
/// does not do.
#[test]
fn a_rollback_that_itself_fails_says_so_and_says_what_to_do() {
    let (_temporary, root) = two_lessons();

    let failed = run(
        &root,
        |snapshot| {
            ops::append_many(
                snapshot,
                Target::Root,
                vec![
                    NewEntry::empty(draft("third")),
                    NewEntry::empty(draft("fourth")),
                ],
            )
        },
        // Fail the second effect, and then the first — and only — unwind step.
        Faults::at_effect_and_unwind(1, 0),
    )
    .expect_err("the seam failed an effect and then its undo");

    let Error::FailedPartiallyRolledBack { .. } = &failed else {
        panic!("a rollback that fails is not the same outcome as one that does not: {failed:?}");
    };
    assert!(
        failed.to_string().contains("neither the state"),
        "a consumer meeting this needs a next step: {failed}"
    );
    assert!(
        root.join("03-draft-third-i3.md").exists(),
        "and the tree really is in neither state: what landed is still there"
    );
}

/// Discharges `inv_interpreterNeverFindsADestinationTaken` by exhibiting the one
/// thing that can still reach it. The model says the interpreter's own exclusive
/// create never fires, because the algebra folded the plan through the snapshot
/// and already knows what every effect will meet — **under the lock**. The lock
/// is advisory, so a writer that does not take it can occupy a destination
/// between the snapshot and the apply, and that is what this test is: the plan
/// is built, the destination is taken behind its back, and the interpreter
/// refuses to write over it.
///
/// Without a test the check reads as dead code to whoever next tidies up.
#[test]
fn an_uncooperative_neighbour_cannot_be_written_over() {
    let (_temporary, root) = two_lessons();
    let guard = crate::fs::write::<SyllabusName>(&root).expect("a well-formed tree");
    let decision = ops::append(
        guard.snapshot(),
        Target::Root,
        NewEntry::new(draft("third"), b"mine".to_vec()),
    );

    // A neighbour that never took the lock, arriving between the snapshot and
    // the apply.
    fs::write(root.join("03-draft-third-i3.md"), "not yours").expect("the neighbour writes");

    let failed = guard
        .run(decision, Faults::none())
        .expect_err("the destination was taken");
    let Error::Failed { source, .. } = &failed else {
        panic!("the run unwinds cleanly, so this is the atomic failure: {failed:?}");
    };
    assert_eq!(source.kind(), std::io::ErrorKind::AlreadyExists);
    assert_eq!(
        fs::read_to_string(root.join("03-draft-third-i3.md")).expect("still there"),
        "not yours",
        "an exclusive create claims a destination; it never replaces one"
    );
}

/// The same claim on the effect that cannot claim its destination with a
/// syscall. `rename(2)` **replaces** whatever is at its destination, silently,
/// so the interpreter has to look first — unfollowed, because a symbolic link
/// occupies a name whatever it points at. This test is here because a mutation
/// control found nothing testing it: removing the look left every other test in
/// this crate green while a rename would have destroyed a neighbour's file.
///
/// Discharges `inv_interpreterNeverFindsADestinationTaken`'s other half, and
/// the *Refusals* row *every mutation is refused when its destination is
/// occupied by anything at all*.
#[test]
fn a_rename_looks_before_it_leaps() {
    let (_temporary, root) = two_lessons();
    let guard = crate::fs::write::<SyllabusName>(&root).expect("a well-formed tree");
    let first = guard
        .snapshot()
        .by_key(Key::new(1))
        .expect("the first lesson")
        .index();
    let decision = Decision::Proceed(Plan::of(vec![Effect::MoveTo {
        entry: first,
        to: Level::Root,
        name: lesson(3, 1, Status::Draft, "first"),
    }]));

    // A neighbour that never took the lock, occupying the destination between
    // the snapshot and the apply.
    fs::write(root.join("03-draft-first-i1.md"), "not yours").expect("the neighbour writes");

    let failed = guard
        .run(decision, Faults::none())
        .expect_err("the destination was taken");
    let Error::Failed { source, .. } = &failed else {
        panic!("nothing had landed, so the unwind is empty and this is atomic: {failed:?}");
    };
    assert_eq!(source.kind(), std::io::ErrorKind::AlreadyExists);
    assert_eq!(
        fs::read_to_string(root.join("03-draft-first-i1.md")).expect("still there"),
        "not yours",
        "a rename that replaced this would have destroyed it, and `rename(2)` does"
    );
    assert!(
        root.join("01-draft-first-i1.md").exists(),
        "and the entry that was to be moved is where it was"
    );
}

/// Discharges no model claim about *this* plan — no operation builds it yet —
/// but it is `planPromote`'s shape exactly: create the node carrying the leaf's
/// own ordinal and key, then move the leaf into it as the distinguished child.
/// It is here because the rollback is **shared**, so the interpreter's rename
/// path and [`Level::Created`] have to be exercised by this leaf or by nothing
/// until `promote` lands.
#[test]
fn a_plan_can_create_a_level_and_move_an_entry_into_it() {
    let (_temporary, root) = two_lessons();
    let report = run(
        &root,
        |snapshot| {
            let first = snapshot
                .by_key(Key::new(1))
                .expect("the first lesson")
                .index();
            Decision::Proceed(Plan::of(vec![
                Effect::Create {
                    at: Level::Root,
                    name: SyllabusName::compose(
                        Ordinal::new(1),
                        Key::new(1),
                        Parts::module(Label::new("first").expect("a label")),
                    ),
                    content: Vec::new(),
                },
                Effect::MoveTo {
                    entry: first,
                    to: Level::Created(0),
                    name: overview(),
                },
            ]))
        },
        Faults::none(),
    )
    .expect("both effects land");

    assert_eq!(
        listing(&root),
        [
            "01-first-i1/".to_string(),
            "01-first-i1/OVERVIEW.md = first".to_string(),
            "02-draft-second-i2.md = second".to_string(),
        ],
        "the leaf's bytes moved verbatim into the node's distinguished child"
    );
    assert_eq!(report.created().len(), 1);
    assert_eq!(report.renamed().len(), 1);
    assert_eq!(report.renamed()[0].from, root.join("01-draft-first-i1.md"));
}

/// Discharges `inv_atomicity` on the half an `append` can never reach: an
/// unwind that has to put a **moved** entry back rather than remove something it
/// created. Same plan as above with a third effect, failed — and the leaf is
/// back at its own path with its own bytes.
#[test]
fn unwinding_a_move_puts_the_entry_back_where_it_was() {
    let (_temporary, root) = two_lessons();
    let before = listing(&root);

    let failed = run(
        &root,
        |snapshot| {
            let first = snapshot
                .by_key(Key::new(1))
                .expect("the first lesson")
                .index();
            Decision::Proceed(Plan::of(vec![
                Effect::Create {
                    at: Level::Root,
                    name: SyllabusName::compose(
                        Ordinal::new(1),
                        Key::new(1),
                        Parts::module(Label::new("first").expect("a label")),
                    ),
                    content: Vec::new(),
                },
                Effect::MoveTo {
                    entry: first,
                    to: Level::Created(0),
                    name: overview(),
                },
                Effect::Create {
                    at: Level::Created(0),
                    name: lesson(1, 3, Status::Draft, "a-first-child"),
                    content: b"child".to_vec(),
                },
            ]))
        },
        Faults::at_effect(2),
    )
    .expect_err("the seam failed the third effect");

    assert!(
        matches!(failed, Error::Failed { .. }),
        "the move and the create both unwound, so this is the atomic failure and \
         not the other one: {failed:?}"
    );
    assert_eq!(
        listing(&root),
        before,
        "the move was undone before the create that preceded it"
    );
}

/// Discharges no model claim. The report is what crosses the surface in a
/// plan's place, and a caller's first question of an `append` is *what key did
/// it get* — which is on the name, and which the caller could not have known.
#[test]
fn the_report_carries_the_names_and_the_paths_of_what_landed() {
    let (_temporary, root) = two_lessons();
    let report = run(
        &root,
        |snapshot| {
            ops::append(
                snapshot,
                Target::Root,
                NewEntry::new(draft("third"), b"third".to_vec()),
            )
        },
        Faults::none(),
    )
    .expect("a clean run");

    assert_eq!(report.created().len(), 1);
    assert_eq!(report.created()[0].name.to_string(), "03-draft-third-i3.md");
    assert_eq!(
        report.created()[0].path,
        root.join("03-draft-third-i3.md"),
        "in the caller's own spelling of the root, because nothing canonicalises"
    );
    assert!(report.renamed().is_empty());
}

// ===========================================================================
// The five controls `interpreter-k21` found missing. Each stands in front of a
// mechanism that was implementing a property a second mechanism already had a
// control for — entry 010's counterfactual, applied at the boundary.
// ===========================================================================

/// Discharges `wit_rewriteToSameParts`: *a rewrite whose new parts equal the old
/// is a rename onto itself, and it must **succeed***.
///
/// The algebra had this and the interpreter did not. `src/plan/tests.rs`'s
/// `an_entry_does_not_occupy_its_own_destination` proves the plan applicable and
/// stops there; applying it met `claim_vacant`, which saw the mover itself and
/// returned `AlreadyExists`. One property, two mechanisms, a control in front of
/// the first only — so this is the same plan, run to the end.
///
/// The whole plan matters, not only that it returns `Ok`: nothing must be
/// undone, because a no-op registers no undo, and the entry's bytes must be
/// exactly where they were.
#[test]
fn a_move_onto_an_entrys_own_path_is_the_no_op_the_model_requires() {
    let (_temporary, root) = two_lessons();
    let before = listing(&root);

    let report = run(
        &root,
        |snapshot| {
            let first = snapshot
                .by_key(Key::new(1))
                .expect("the first lesson")
                .index();
            Decision::Proceed(Plan::of(vec![Effect::MoveTo {
                entry: first,
                to: Level::Root,
                // The name it already carries: `rewrite` to the parts it has.
                name: lesson(1, 1, Status::Draft, "first"),
            }]))
        },
        Faults::none(),
    )
    .expect("the no-op rewrite must succeed");

    assert_eq!(listing(&root), before, "a no-op changes nothing");
    assert_eq!(report.renamed().len(), 1, "and it is still reported");
    assert_eq!(report.renamed()[0].from, report.renamed()[0].to);
}

/// Discharges `inv_interpreterNeverFindsADestinationTaken` on the case the fix
/// above must not reach — which is what makes the exclusion *narrow* rather than
/// a hole. A move onto a destination that is genuinely something else is still
/// refused, and what is there is still there; `rename(2)` would have replaced it
/// silently.
#[test]
fn the_no_op_exclusion_does_not_reach_a_different_occupied_destination() {
    let (_temporary, root) = two_lessons();
    let guard = crate::fs::write::<SyllabusName>(&root).expect("a well-formed tree");
    let first = guard
        .snapshot()
        .by_key(Key::new(1))
        .expect("the first lesson")
        .index();
    // Ordinal 2 is the *second* lesson's place, and it is occupied.
    let decision = Decision::Proceed(Plan::of(vec![Effect::MoveTo {
        entry: first,
        to: Level::Root,
        name: lesson(2, 2, Status::Draft, "second"),
    }]));

    let failed = guard
        .run(decision, Faults::none())
        .expect_err("the destination is another entry, not this one");
    let Error::Failed { source, .. } = &failed else {
        panic!("nothing had landed, so the unwind is empty and this is atomic: {failed:?}");
    };
    assert_eq!(source.kind(), std::io::ErrorKind::AlreadyExists);
    assert_eq!(
        fs::read_to_string(root.join("02-draft-second-i2.md")).expect("still there"),
        "second"
    );
}

/// Discharges `inv_atomicity` on the interval no whole-effect failure can reach:
/// **after** a leaf's destination is claimed exclusively and **before** its bytes
/// are written.
///
/// Atomicity has two mechanisms here — the order effects run in, and the order
/// the undo is registered in relative to the write — and `Faults::at_effect`
/// only ever fires before a create, so it controls the first and not the second.
/// A real short write or a full disk lands in this interval, and
/// `Error::Failed`'s own words are *every effect this operation had applied was
/// undone*, which includes the partial file.
///
/// The mutation this catches: move `self.undo.push(…)` below `write_all`. Every
/// other test in this crate stays green, because their writes all succeed.
#[test]
fn a_failure_between_claiming_a_leaf_and_writing_it_removes_the_partial_file() {
    let (_temporary, root) = two_lessons();
    let before = listing(&root);

    let failed = run(
        &root,
        |snapshot| {
            ops::append(
                snapshot,
                Target::Root,
                NewEntry::new(draft("third"), b"third".to_vec()),
            )
        },
        Faults::at_content(0),
    )
    .expect_err("the seam failed the write, not the create");

    assert!(
        matches!(failed, Error::Failed { .. }),
        "the file the create claimed was removed, so this is the atomic failure: {failed:?}"
    );
    assert!(
        !root.join("03-draft-third-i3.md").exists(),
        "the destination was claimed before the failure and must not survive it"
    );
    assert_eq!(listing(&root), before);
}

/// Discharges `inv_interpreterNeverFindsADestinationTaken` on the **third**
/// mechanism that claims a destination.
///
/// Entry 009 counted two — `create_new`/`create_dir` together, and the rename's
/// look — but they are separate branches with separate syscalls, and only the
/// file one had a control. `create_dir` refuses an occupied destination;
/// `create_dir_all` would accept a neighbour's directory as this run's own
/// creation, and then *register an undo that removes it*.
///
/// So the plan has a second effect that fails. Under the correct code the first
/// effect refuses and the fault at effect 1 never fires; under the mutation the
/// first effect "succeeds", the second fails, and the unwind deletes a directory
/// this run never created — which is exactly what
/// `inv_rollbackRemovesOnlyItsOwn` forbids. The neighbour's directory is left
/// empty deliberately: a non-empty one would survive `remove_dir` by accident
/// and the control would prove nothing.
#[test]
fn an_uncooperative_neighbour_cannot_have_its_directory_claimed_or_removed() {
    let (_temporary, root) = two_lessons();
    let guard = crate::fs::write::<SyllabusName>(&root).expect("a well-formed tree");
    let decision = Decision::Proceed(Plan::of(vec![
        Effect::Create {
            at: Level::Root,
            name: module(3, 3, "topology"),
            content: Vec::new(),
        },
        Effect::Create {
            at: Level::Root,
            name: lesson(4, 4, Status::Draft, "fourth"),
            content: b"fourth".to_vec(),
        },
    ]));

    // A neighbour that never took the lock, taking the node's destination
    // between the snapshot and the apply.
    let taken = root.join("03-topology-i3");
    fs::create_dir(&taken).expect("the neighbour writes");

    let failed = guard
        .run(decision, Faults::at_effect(1))
        .expect_err("the node's destination was taken");
    let Error::Failed { source, .. } = &failed else {
        panic!("nothing had landed, so the unwind is empty and this is atomic: {failed:?}");
    };
    assert_eq!(source.kind(), std::io::ErrorKind::AlreadyExists);
    assert!(
        taken.is_dir(),
        "a `create_dir` claims a destination; it never adopts one, and no unwind \
         may remove what this run did not create"
    );
    assert!(
        !root.join("04-draft-fourth-i4.md").exists(),
        "and the effect after it never ran"
    );
}

/// Discharges no model claim — the report is unmodelled, as bytes are. The
/// promise `Report::paths` makes is *in the order the effects landed*, and two
/// species-sorted buckets cannot keep it: this is `planPromote`-with-a-first-child's
/// exact shape, create, move, create, whose middle effect no bucket ordering
/// reproduces.
///
/// `created()` and `renamed()` keep their own orders, which is where the
/// highest-first shift rule stays observable.
#[test]
fn the_reports_paths_are_in_the_order_the_effects_landed() {
    let (_temporary, root) = two_lessons();
    let report = run(
        &root,
        |snapshot| {
            let first = snapshot
                .by_key(Key::new(1))
                .expect("the first lesson")
                .index();
            Decision::Proceed(Plan::of(vec![
                Effect::Create {
                    at: Level::Root,
                    name: module(1, 1, "first"),
                    content: Vec::new(),
                },
                Effect::MoveTo {
                    entry: first,
                    to: Level::Created(0),
                    name: overview(),
                },
                Effect::Create {
                    at: Level::Created(0),
                    name: lesson(1, 3, Status::Draft, "a-first-child"),
                    content: b"child".to_vec(),
                },
            ]))
        },
        Faults::none(),
    )
    .expect("all three effects land");

    let node = root.join("01-first-i1");
    assert_eq!(
        report.paths().collect::<Vec<_>>(),
        vec![
            node.as_path(),
            node.join("OVERVIEW.md").as_path(),
            node.join("01-draft-a-first-child-i3.md").as_path(),
        ],
        "the move is between the two creations, which no pair of buckets can say"
    );
    assert_eq!(
        report.created().len(),
        2,
        "and the buckets are still buckets"
    );
    assert_eq!(report.renamed().len(), 1);
}

/// The seventh obligation, at the boundary a hand-built plan reaches: a name
/// that renders as more than one path component is refused **before any effect
/// runs**, so nothing is created, nothing is moved, nothing is reported and
/// there is nothing to roll back.
///
/// Discharges no model claim, and cannot: `operations.qnt` and `structure.als`
/// both hold no strings by design, so neither can pose a rendering at all. This
/// is prose's to own, and the prose is `ARCHITECTURE.md`'s seventh obligation.
///
/// [`Sneaky`] satisfies the trait everywhere the algebra looks — its `view` is
/// the reference domain's, so occupancy sees a perfectly canonical name — and
/// renders every composed name with a leading `../`.
#[test]
fn a_plan_naming_a_path_outside_the_tree_is_refused_before_anything_moves() {
    let (_temporary, root) = two_lessons();
    let before = listing(&root);
    let outside = root.join("..").join("03-draft-escaped-i1.md");

    let guard = crate::fs::write::<Sneaky>(&root).expect("Sneaky reads a tree honestly");
    let first = guard
        .snapshot()
        .by_key(Key::new(1))
        .expect("the first lesson")
        .index();
    let decision = Decision::Proceed(Plan::of(vec![Effect::MoveTo {
        entry: first,
        to: Level::Root,
        name: Sneaky::compose(Ordinal::new(3), Key::new(1), draft("escaped")),
    }]));

    let failed = guard
        .run(decision, Faults::none())
        .expect_err("a name that is not one filename cannot be placed");
    let Error::NameIsNotOneComponent { rendered, .. } = &failed else {
        panic!("this is not an I/O failure and not a refusal of the algebra's: {failed:?}");
    };
    assert_eq!(rendered, "../03-draft-escaped-i1.md");
    assert!(
        failed.to_string().contains("one filename"),
        "a consumer meeting this needs to know what is wrong with the name: {failed}"
    );
    assert_eq!(listing(&root), before, "and the tree is untouched");
    assert!(
        !outside.exists(),
        "nothing was placed outside the tree either"
    );
}

/// Discharges `inv_atomicity` on the promotion path: *after a mutation returns
/// an error, either every effect landed or none did*. The node is created, the
/// move fails, the node is removed, and the tree is exactly the two lessons it
/// was.
///
/// The plan is a real `promote` rather than one written by hand, because what is
/// under test is the interaction between *that* plan's shape and the shared
/// rollback: a create whose undo is a `remove_dir`, which succeeds only because
/// the directory is still empty.
#[test]
fn a_promotion_whose_move_fails_leaves_the_tree_as_it_was() {
    let (_temporary, root) = two_lessons();
    let before = listing(&root);

    let failed = run(
        &root,
        |snapshot| {
            ops::promote(
                snapshot,
                Key::new(1),
                Parts::module(Label::new("first").expect("a label")),
                None,
            )
        },
        // Effect 1 is the move of the leaf into the node effect 0 just created.
        Faults::at_effect(1),
    )
    .expect_err("the seam failed the move");

    let Error::Failed { .. } = &failed else {
        panic!("a rollback that succeeds is `Failed`, not something else: {failed:?}");
    };
    assert_eq!(
        listing(&root),
        before,
        "the node was removed and the leaf never moved"
    );
}

/// Discharges `wit_partialRollbackLeavesADuplicateKey` and
/// `wit_partialRollbackLeavesADuplicateOrdinal` — the model's `rollback_fails`
/// instance, which is the only one that does not claim key uniqueness at rest,
/// **because this operation is what breaks it**.
///
/// This is the single path by which this library creates a duplicate key in a
/// tree it was handed. Everywhere else a duplicate key is a defect it inherits.
/// The promotion's one undo is *remove the node just created*, so an unwind that
/// fails there leaves the leaf and the node both in place, sharing an ordinal and
/// a key, with the node holding nothing.
///
/// A library that can leave a tree in that state and does not say how to get out
/// of it has told the consumer nothing useful — so the second half of this test
/// is the **error text**, checked clause by clause against what is actually on
/// disk. Each clause of the advice is asserted here as a fact about the tree, and
/// the fact is asserted as a clause of the message: a recovery instruction is
/// only worth printing if it describes the state it will be read in.
#[test]
fn a_promotion_whose_rollback_fails_leaves_a_duplicate_key_and_says_how_to_resolve_it() {
    let (_temporary, root) = two_lessons();

    let failed = run(
        &root,
        |snapshot| {
            ops::promote(
                snapshot,
                Key::new(1),
                Parts::module(Label::new("first").expect("a label")),
                None,
            )
        },
        // Fail the move, and then the only unwind step there is: removing the
        // node the create had just made.
        Faults::at_effect_and_unwind(1, 0),
    )
    .expect_err("the seam failed the move and then its undo");

    let Error::FailedPartiallyRolledBack { .. } = &failed else {
        panic!("a rollback that fails is not the same outcome as one that does not: {failed:?}");
    };

    // The damage, as the tree actually holds it: a node and a leaf at the same
    // ordinal carrying the same key, and the node holding no distinguished
    // child.
    assert_eq!(
        listing(&root),
        [
            "01-draft-first-i1.md = first".to_string(),
            "01-first-i1/".to_string(),
            "02-draft-second-i2.md = second".to_string(),
        ],
        "both halves of the promotion are on disk, at ordinal 1 and key 1"
    );

    // And the advice, clause by clause against exactly that.
    let said = failed.to_string();
    assert!(
        said.contains("neither the state"),
        "a consumer meeting this needs to know the tree is in neither state: {said}"
    );
    assert!(
        said.contains("node and a leaf share an ordinal and a key")
            && said.contains("no distinguished child"),
        "the advice has to describe the state it will be read in: {said}"
    );
    assert!(
        said.contains("interrupted promotion") && said.contains("removing either half"),
        "and it has to say what to do, mechanically: {said}"
    );

    // The recovery really is mechanical: removing either half resolves it, and
    // the tree then reads cleanly again. Removing the node is the half that
    // restores what the operation found.
    fs::remove_dir(root.join("01-first-i1")).expect("removing the empty node");
    let tree = crate::fs::read::<SyllabusName>(&root).expect("a tree that reads again");
    assert_eq!(
        tree.walk().map(|e| e.name().to_string()).collect::<Vec<_>>(),
        ["01-draft-first-i1.md", "02-draft-second-i2.md"]
    );
}
