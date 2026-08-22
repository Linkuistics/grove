# insert-k23

**Reviews:** insert-k11

## Goal

Read `insert-k11`'s artifact adversarially and produce findings. The artifact is
`ops::insert`, `Refusal::NoOccupantAtOrdinal`, `WriteGuard::insert`, the fifteen
algebra tests appended to `src/ops/tests.rs` and the nine in
`tests/inserting_on_disk.rs`, plus one corrected bullet in `ARCHITECTURE.md`'s
*Refusals* and entry 012 of `docs/formalism-findings.md`.

## Context

Beyond the brief chain and the *Read first* list in `05-crate-k7/BRIEF.md`:

- `insert-k11`'s own task file, whose `## Decisions (running log)` states five
  calls this session made. Each is a place to push.
- `operations.qnt`'s `planInsert`, `shiftIds`, `inv_insertOnlyShifts`,
  `inv_ordinalsDistinctThroughout`, and the four insert witnesses —
  `wit_insertPastTheEnd`, `wit_insertIntoAGap`,
  `wit_shiftTransientlyDuplicatesAnOrdinal`, `wit_shiftOrderRefusesTheInsert`.
  `./run-quint.sh` was green and unchanged by this leaf.
- `docs/formalism-findings.md` entry 012, including its two recorded misses.

## Done when

Findings exist, or the session concludes there are none and retires without
cutting an integrate leaf.

## Notes

Four specific doubts this session could not settle on itself, written down so the
reviewer does not have to find them first. They are a floor, not a ceiling.

**The refusal now carries state, and state can be wrong.**
`Refusal::NoOccupantAtOrdinal { ordinal, greatest }` exists so one refusal can
give two pieces of advice, and `greatest` is computed by a second pass over the
level after the first pass has already collected the shifted siblings. Is the
value it carries always the level's true greatest ordinal — on a level holding
only a distinguished child, on one whose ordinals are not dense, on one a hand
edit left with two entries at the greatest ordinal? And is `ordinal <= greatest`
the right test for *gap*, or does it mis-classify an ordinal below the level's
smallest, where nothing is occupied below it either and *by hand* may be worse
advice than *append*?

**The shift order rests on `positioned()` being ascending, and that is a
transitive assumption.** `insert` collects the siblings from `Container::positioned`
and calls `Vec::reverse`, which is the model's `reverseI(asc)` only if
`positioned()` yields ascending ordinals. That comes from `sort_level` in
`src/snapshot.rs` — prose from the architecture document, checked by no model,
with two tie-breaks that only fire on a hand-edited level. Does the reverse
survive a level carrying a duplicate ordinal? `operations.qnt`'s `shiftIds`
folds `idsAtOrdinal` per ordinal and reverses the whole list; is the Rust the
same list, or only the same on trees without duplicates?

**The intermediate-state test may be weaker than it reads.**
`every_intermediate_state_of_a_shift_has_distinct_ordinals` replays the plan's
landings over the level, and `the_same_shifts_run_lowest_first_pass_through_a_duplicate_ordinal`
replays them reversed as its control. Both use the same hand-written fold
(`ordinals_after_each_step`), which is a second implementation of the
interpreter's semantics living in a test file. If that fold is wrong, both tests
are wrong together and the control does not detect it. Entry 005's finding —
a test that names a claim and checks a weaker property — is the shape to look
for.

**Two things are asserted and not checked.** *A shifted node is one directory
rename* is held by exactly one integration test reading one directory's contents
after one insert; is that enough to catch a plan that reached inside a subtree in
some other shape? And entry 012's second miss says the create's last position is
arranged in the model rather than claimed by it, defended only by mutation (b) —
is there a test that pins it directly, and should there be?

## Findings

### Medium — the gap refusal invents an occupied lower neighbour

`crates/ordinal-fs-tree/src/plan.rs:382` classifies every unoccupied ordinal at
or below the level's greatest ordinal as a gap, then lines 383–384 tell the
caller that "something below it and something above it are occupied." The
predicate does not establish the first half. A hand-edited level containing only
ordinal 5, followed by `insert(..., Ordinal::FIRST, ...)`, reaches this branch
even though no lower ordinal is occupied; `Ordinal::new(0)` makes the statement
impossible on its face. The refusal's actionable conclusion is still right —
`append` will not occupy the requested ordinal and only a hand edit can — but its
state report is false.

Both controls arrange an interior hole with ordinal 1 below the requested
ordinal and ordinal 5 above it (`src/ops/tests.rs:715` and
`tests/inserting_on_disk.rs:302`), then assert only that the message contains
`gap` and `by hand`. Neither distinguishes the implemented `ordinal <= greatest`
predicate from the stronger prose it emits. Cover a leading hole explicitly and
either make the message state only what `greatest` proves or carry enough state
to distinguish an interior gap from one below the first occupied ordinal.

## Doubt verdicts

1. **Refusal state: mixed.** The second pass is over the same immutable,
   deterministically ordered level, so it returns the true greatest ordinal for
   distinguished-only, gapped and duplicate-ordinal levels. `None` correctly
   identifies a level with no positioned child. The greatest alone cannot
   justify the lower-neighbour claim in the finding above.
2. **Shift order: sound.** `sort_level` orders positioned children by ordinal,
   key and rendered name; `Container::positioned` preserves that order and
   reversing the collected suffix therefore leaves ordinals descending even
   when a hand edit duplicated one. Reversing also reverses the two prose
   tie-breaks within an equal ordinal, but neither the operation contract nor
   `operations.qnt` specifies an order within `idsAtOrdinal`, and every sibling
   is still shifted exactly once.
3. **Intermediate-state projection: sound in combination.** For this operation
   there is one create and every move stays in the target level, so
   `ordinals_after_each_step` is the interpreter's exact ordinal projection.
   `an_inserts_plan_names_no_descendant` separately pins every effect to that
   level, and the exact-effect tests pin the one-create shape; the helper is not
   silently compensating for a differently shaped plan.
4. **Subtree and create order: sound in combination.** The algebra test forbids
   descendant effects, `Effect::MoveTo` is applied by one `fs::rename`, and the
   on-disk test observes the directory and its children after that rename. The
   create-last property is asserted directly by
   `the_shift_runs_highest_ordinal_first` and again at the public surface by
   `Report::paths`; mutation (b) is supporting evidence rather than the only
   defence.

Codebase-memory had no project for this jj workspace. Indexing through both the
CLI and MCP worker failed because active-daemon coordination could not be
verified in the sandbox, and the coverage call then required unavailable
approval. This review therefore uses producer commit `294262c1`, complete direct
reads of every cited source/test file, and the exact Quint predicates; no
negative conclusion relies on an empty graph result.
