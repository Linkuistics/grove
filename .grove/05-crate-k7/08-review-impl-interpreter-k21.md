# interpreter-k21

**Reviews:** interpreter-k10

## Goal

Attack the plan machinery before three mutation leaves and a CLI are built on it.
It is the second substrate: `Plan`, `Effect`, `Level`, `Decision`, `Refusal`, the
one interpreter and the one rollback are what `insert-k11`, `promote-k12`,
`rewrite-k13` and `cli-k16` all consume, and `ARCHITECTURE.md` is explicit that a
plan shape an operation cannot use is a *finding about the plan shape* rather
than a licence to add a second interpreter. So a defect here is a rework of four
leaves, not a patch.

The measured precedent is `seam-k17` and `reading-k19`: six findings and four,
each on a substrate, and `docs/formalism-findings.md` entry 005 says in as many
words that taking them after the dependent leaves would have been rework.

## Context

Beyond the brief chain, and beyond `interpreter-k10`'s own `Context` section:

- The artifact: `crates/ordinal-fs-tree/src/plan.rs`, `src/ops.rs`,
  `src/report.rs`, `src/fs/apply.rs`, the additions to `src/error.rs`,
  `src/fs/mod.rs`, `src/lib.rs` and `src/snapshot.rs`, the new `src/fixtures.rs`,
  and four test files — `src/plan/tests.rs`, `src/ops/tests.rs`,
  `src/fs/apply/tests.rs`, `tests/appending_on_disk.rs`.
- `docs/formalism-findings.md` entry 009, which states this leaf's own claims
  about what the model was worth and where it could not reach. Entries 003
  (sequential destination check, the ordering rule's real payoff) and 005 (*name
  the judgement to attack*; *quote the claim's predicate, not its name*) are the
  method.
- The two document sections this leaf amended: *How an operation runs* (a
  mutation consumes its handle) and *Refusals* (three refusals no model can
  pose).

## What to attack, named

Seven judgements, in the order of what they would cost if wrong. Each was made
deliberately and each is defensible, which is why a fresh reader is worth paying
for.

1. **A mutation consumes its guard.** `WriteGuard::append` takes `self`, so one
   guard is one mutation and the lock drops with it. The argument is that the
   alternatives are worse: a surviving guard plans the next operation from a
   stale snapshot, and refreshing the snapshot means a mutation that succeeded
   returning the error of the read that followed it — which *plan atomicity*
   forbids. Attack it from `cli-shape-k15`'s side: a CLI that must read, decide
   and then mutate under **one** lock, or must perform two operations under one,
   is the case that breaks this. Ask also whether `promote`-with-a-first-child
   and `insert` really need nothing more.
2. **`Effect` has two variants and the undo has two others.** The model's
   `Effect` has three, with `Remove` documented as undo-only; here that is a type
   split, so `inv_rollbackRemovesOnlyItsOwn` is structural. Attack the claim that
   nothing is lost: is there an operation whose plan legitimately wants a forward
   removal, and does the split make `insert`'s or `promote`'s plan harder to
   express than the model's does?
3. **`MoveTo` and `Level::Created` exist with no operation producing them.** They
   are exercised only by hand-built plans in this crate's own tests, and marked
   `#[cfg_attr(not(test), expect(dead_code))]` so the leaf that builds them for
   real must remove the marker. Attack the call both ways: speculative generality
   that should have waited for `insert`, or exactly the shared machinery a
   later-leaf rewrite would have had to redo.
4. **Occupancy compares `view()`s, not renderings.** Soundness rests entirely on
   the canonicity obligation plus *a foreign name can never occupy a
   destination*. Attack the second: construct a domain, or a tree, where a name
   the consumer disclaims — or one the snapshot never saw — sits at a filename a
   plan will compose, and see what the interpreter does with it. The rename path
   is where to look, because that is the one whose claim is not a syscall.
5. **The rename's destination claim is a look, then a call.** `create_new` and
   `create_dir` claim atomically; `rename(2)` replaces its destination silently
   and macOS has no portable no-replace rename, so the interpreter calls
   `symlink_metadata` first. Attack the window that leaves, and attack the
   reasoning that the only writer who could exploit it is one already ignoring
   the advisory lock. A mutation control found this path *entirely untested*
   before the test that now covers it — ask what else the same blind spot covers.
6. **Three refusals no model can pose.** `ContentForANode`, `KeysExhausted`,
   `OrdinalsExhausted`, now in `ARCHITECTURE.md`. Attack each: bytes for a node
   could have been made unrepresentable in the API rather than refused at run
   time; exhaustion could have been argued unreachable and left to panic, or the
   types widened. Attack the direction of each trade, and whether a refusal
   variant a consumer must match on is the right price.
7. **`append` is `append_many` of one.** The model has two planning functions
   that agree on a single-element run *by inspection*; the implementation has
   one. Attack the inspection: is there any tree on which `planAppend` and
   `planAppendMany(List(p))` differ, and does deriving one from the other change
   any refusal's identity or order?

## Done when

- Every judgement above has a verdict: sound, or a specific defect with the
  structure it admits.
- Anything found is a **finding**, not a fix — this session changes no code.
- The nine mutation controls are checked for the thing they cannot prove.
  Entry 009's own counterfactual is the sharpest question to put to them: *one
  property implemented by two mechanisms wants two controls* — the rename gap was
  found that way, so ask which other property here has more than one mechanism
  behind it and only one control in front of it.
- The twenty-eight tests that name a model claim are read against the claim's
  **predicate** rather than its name (005's counterfactual, and `seam-k17`'s two
  sharpest findings). The fourteen that say they have none are read for whether
  that is honest.
- An entry in `docs/formalism-findings.md`, including if the answer is *nothing
  found*.
- An `integrate-review-impl` leaf beside this one **only if** there are findings
  worth acting on. Cut it with `leaf-insert` against the first later sibling
  still holding live work, because an integration consumes `path:line`
  coordinates that any intervening leaf silently moves.

## Notes

**The producer spent none of its in-session review allowance.** Every claim in
the code comments and in entry 009 is the producer's own.

**Both model suites were re-run green as controls** — Alloy 20/20, Quint every
claim across all eight instances, every witness reached in a non-zero number of
traces — and no model was changed by this leaf. So a disagreement found here is
between the code and a model that still says what it said, which makes it a
finding of the sharpest kind this workstream collects.

**The two hand-built shift plans in `src/plan/tests.rs` are `insert`'s, not this
leaf's.** They are there because the fold-versus-snapshot decision is invisible
to every plan an `append` can build. Read them as a prediction about what
`insert-k11` will produce, and say so if the prediction is wrong — that is
cheaper to find now than after the leaf runs.

## Findings
