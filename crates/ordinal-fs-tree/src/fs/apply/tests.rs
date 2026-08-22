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
use crate::fixtures::{lesson, overview};
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

    assert!(matches!(failed, Error::Failed { .. }));
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
