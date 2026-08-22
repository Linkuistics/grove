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

### High — a composed name can escape the locked tree

`crates/ordinal-fs-tree/src/name.rs:349` calls an `EntryName` a type wrapping a
filename, but the exhaustive six obligations beginning at line 353 never require
its `Display` to be one normal path component. The interpreter then joins that
unchecked rendering at `src/fs/apply.rs:98` for creates and line 146 for moves.
A domain can satisfy every stated obligation and every conformance check while
rendering a composed name as `../outside`, `child/../../outside`, or an absolute
path, provided its `parse` accepts that same spelling and returns the composed
view. `Path::join` then addresses outside the root (and an absolute rendering
replaces it outright), while the plan's value-level occupancy check sees only a
perfectly canonical `NameView`.

That admits creation, rename, rollback deletion and report paths outside the
tree whose containing directory is locked. It is both a confinement failure and
a violation of the library's central proposition that one directory tree is the
data structure. Neither model can expose it because both intentionally hold no
strings, and every test uses the reference domain's safe labels. Make
single-component rendering an enforced boundary property — not merely an
unstated domain convention — and cover `..`, separators and absolute renderings
with a domain that otherwise satisfies the six obligations.

### High — the interpreter refuses the no-op rewrite the model requires to succeed

The algebra deliberately excludes the mover from occupancy at
`crates/ordinal-fs-tree/src/plan.rs:116`–123, and
`src/plan/tests.rs:217` cites `wit_rewriteToSameParts`: rewriting to the existing
parts must succeed. The interpreter loses that exclusion at
`src/fs/apply.rs:144`–149. For the exact no-op plan, `from == path`, but
`claim_vacant(&path, ...)` sees the source entry itself and returns
`AlreadyExists` before `rename` runs.

The existing test stops after `Plan::guarded`; no test applies the plan it just
proved applicable. This is the counterfactual entry 009 asks for in its sharpest
form: one property is implemented once in the algebra and again at the
filesystem boundary, with a control only in front of the first mechanism. Apply
a same-path `MoveTo` end to end and preserve the no-replace check for genuinely
different destinations.

### Medium — `Report::paths` cannot return effect order as promised

`crates/ordinal-fs-tree/src/report.rs:42` stores creations and renames in two
separate vectors, then lines 84–90 promise every resulting path "in the order
the effects landed" while returning all creations followed by all renames.
Mixed plans make that promise false. `insert` is shifts (`MoveTo`) followed by a
`Create`, so its creation is reported before every shift; promotion with a first
child is `Create`, `MoveTo`, `Create`, which cannot be reconstructed from the
two buckets either.

The current append-only public tests cannot expose this and the hand-built
promotion test checks only the two per-species slices. Preserve an ordered event
record (or narrow the public ordering contract deliberately) and add mixed-plan
controls before the three dependent mutation leaves and the CLI consume it.

### Medium — the atomicity control cannot fail after a file has been created

`crates/ordinal-fs-tree/src/fs/apply.rs:127` correctly registers `Undo::Remove`
before `write_all` at line 134, because a short write or disk-full error may
leave a partial file behind. But `Faults::strike_effect` runs at line 99, before
the create, and every `inv_atomicity` test injects failure only between whole
effects (`src/fs/apply/tests.rs:84`–123). Moving the undo registration to after
`write_all` leaves those tests unchanged: their first write succeeds and their
next effect fails, so an undo still exists. A real error during `write_all`
would then return `Error::Failed` while leaving the partial file that variant
promises was removed.

The implementation is presently ordered correctly; the claimed predicate is
not protected. Add an internal failure point after exclusive creation and before
or during content writing, plus the mutation control that moves undo
registration past it. This is another distinct mechanism behind atomicity that
the model cannot cover because it has no filesystem or bytes.

### Medium — node creation is the third untested destination-claim mechanism

Entry 009 groups `create_new` and `create_dir` as one atomic-claim mechanism,
but the implementation has separate branches at
`crates/ordinal-fs-tree/src/fs/apply.rs:100`–139. The uncooperative-neighbour
test at `src/fs/apply/tests.rs:201` covers a leaf's `create_new`; the next test
covers the rename look. No test occupies a node destination after the snapshot.
Replacing `create_dir` with `create_dir_all` would leave the current suite green,
treat a neighbour's existing directory as this run's creation, and let a later
rollback remove that directory if it remained empty.

Count this as a third mechanism, not as a second instance of the file-create
control. Take a node destination behind the plan's back, require an
`AlreadyExists` failure, and prove the neighbour's directory survives both the
failed apply and any unwind.

## Judgement verdicts

1. **A mutation consumes its guard — sound.** A `WriteGuard` still supports
   read-then-choose-then-mutate under one lock; the operations named by the
   brief each need one mutation, while `append_many` and promotion-with-child
   are the required compound units. Keeping a stale snapshot or reporting a
   post-success refresh error is worse.
2. **Forward `Effect` and private `Undo` are split — sound.** No specified
   forward operation removes an entry, and every required mutation is
   expressible as create and/or move. The split makes the model's undo-only
   removal comment structural without narrowing `insert`, `promote` or
   `rewrite`.
3. **`MoveTo` and `Level::Created` land early — sound.** They are exactly the
   shared interpreter and rollback machinery `insert` and `promote` need, not a
   second abstraction. The dead-code expectations correctly force the later
   leaves to acknowledge first production use.
4. **Occupancy compares views — mixed.** Canonicity makes the comparison sound
   for one filename, and a foreign listing cannot share a producible spelling.
   The absent path-component boundary admits filesystem aliases and escape, and
   the mover exclusion is not carried into `claim_vacant`; both are findings
   above.
5. **Rename uses look-then-call — sound within the recorded boundary.** A writer
   arriving after the unfollowed look and before `rename` can still be replaced,
   but that is concurrent mutation during apply, explicitly outside the model
   and the advisory-lock contract. The existing control proves only occupancy
   before the look, which is the stated portable guarantee on macOS.
6. **The three unmodelled refusals — sound.** Opaque parts prevent the API from
   making leaf-versus-node content statically disjoint without widening the
   seam, and checked arithmetic is the correct response to finite public
   integer types. Refusal is preferable to silent byte loss, panic or key
   reissue.
7. **`append` delegates to `append_many` — sound.** For a singleton both model
   functions resolve the same target, choose `maxOrd + 1` and `freshKey`, and
   guard one create. The implementation preserves the same refusal order while
   removing duplicated arithmetic.

The count of twenty-eight model-naming tests and fourteen explicitly unmodelled
tests is accurate, and the fourteen disclaimers are honest. The two over-broad
evidence claims are the cross-layer no-op occupancy check and filesystem
atomicity during content write; the node destination claim is a third mechanism
left without its own control.

Codebase-memory had no project for this jj workspace. Two indexing attempts
failed because the worker could not verify active-daemon coordination; the only
existing grove graph predates `crates/ordinal-fs-tree` and returned no candidate
symbols. These findings therefore come from the producer commit `0bd965a5`, the
complete direct source/test reads cited above, and the exact Quint predicates.
No negative claim relies on an empty graph result.
