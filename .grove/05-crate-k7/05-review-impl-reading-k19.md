# reading-k19

**Reviews:** reading-k9

## Goal

Attack the reading layer before four mutation leaves are built on it. It is the
substrate: `Snapshot`, `Builder`, `Entry`, `Container`, the two guards and the
error type are what `interpreter-k10`, `insert-k11`, `promote-k12` and
`rewrite-k13` all consume, so a defect in its shape costs a subtree rather than a
patch. That is not a forecast — `seam-k17` is the measured precedent, recorded in
`docs/formalism-findings.md` entry 005: six findings, two of which changed the
public surface, and the entry says in as many words that taking them after the
dependent leaves would have been a rework instead of a fix.

## Context

Beyond the brief chain, and beyond `reading-k9`'s own `Context` section, which
lists the document sections and models this artifact answers to:

- The artifact: `crates/ordinal-fs-tree/src/snapshot.rs`, `src/error.rs`,
  `src/fs/` (`mod.rs`, `lock.rs`, `read.rs`), the two new test files, and the
  amended `tests/algebra_has_no_filesystem.rs`.
- `reading-k9`'s `## Decisions (running log)` — eight decisions, each with its
  reasoning. Those are the target: a stated reason is what an adversarial read
  can be *against*.
- `docs/formalism-findings.md` entry 006, and 005 before it. 005's third
  counterfactual is this leaf's method: **name the judgement to attack**, do not
  ask for a review.

## What to attack, named

Five judgements, in the order of what they would cost if wrong. Each was made
deliberately and each is defensible, which is exactly why a fresh reader is worth
paying for — the producer cannot be surprised by its own reasoning.

1. **The public shape the mutation leaves inherit.** `Snapshot` is an arena of
   names with `Entry`/`Container` handles borrowing it, guards `Deref` to it, and
   `Builder`/`Place` are public so a test can build a tree without a directory.
   Ask what `insert` and `promote` will actually need — a level's positioned
   children in ordinal order, an entry's parent, a fresh key from the whole-tree
   maximum — and whether this shape supplies it without a widening. A borrow that
   has to be dropped before a mutation can start is the specific smell.
2. **The walk-order tie-break.** No model checks it. On a level a hand edit left
   carrying a duplicate ordinal, the order is (ordinal, key, rendered name), and
   `by_key` answers the first of it. Attack the claim that this is *total* and
   the claim that it is *the right* total order; and attack the reasoning that
   the alternative — leaving it to listing order — is a real hazard rather than
   an imagined one.
3. **Halting on a filename that is not UTF-8.** A new refusal, in neither model
   and, until this leaf, in no document. The argument is the trichotomy's own: a
   name that cannot be read cannot be disclaimed. The cost is that genuinely
   foreign junk freezes the tree. Attack the direction of that trade, and attack
   the alternative — treating it as `Foreign`, which is what the library's own
   `Verdict` would do if it could see the name at all.
4. **The guard's new carve-out.** `tests/algebra_has_no_filesystem.rs` now
   exempts the `mod fs;` declaration shape. The claim is that this is narrower
   than it looks and that refusing re-exports makes the guard *stronger*. Attack
   the exemption's parser for shapes it wrongly admits, and attack the claim that
   no crate-root alias can now launder a filesystem item into the algebra.
5. **The lock, which no test can fully reach.** Blocking-only with no `try`
   variant, taken on the lexical parent, released by dropping a `File`, retried on
   `EINTR`. Two claims worth doubt: that a lexical parent plus kernel resolution
   really does give two spellings of one tree the same lock, and that a
   `libc` dependency was the right answer to `flock` not being in `std`.

## Done when

- Every judgement above has a verdict: sound, or a specific defect with the
  structure it admits.
- Anything found is a **finding**, not a fix — this session changes no code.
- The mutation controls are checked for the thing they cannot prove: each was
  written by the same session as the test it validates, so ask which of them
  would still fire against a *differently wrong* implementation.
- An entry in `docs/formalism-findings.md`, including if the answer is *nothing
  found* — the log records misses with the same care as hits, and a review that
  finds nothing is H2 evidence.
- An `integrate-review-impl` leaf beside this one **only if** there are findings
  worth acting on. Cut it with `leaf-insert` against the first later sibling
  still holding live work, because an integration consumes `path:line`
  coordinates that any intervening leaf silently moves.

## Notes

**The producer spent none of its in-session review allowance**, so nothing here
has been read adversarially yet. Every claim in the code comments and in the
decisions log is the producer's own.

**Read the tests as an artifact, not as evidence.** `seam-k17`'s sharpest two
findings were tests that cited a model claim and checked something weaker, and
005's counterfactual is to compare the assertion against the claim's *predicate*
rather than its name. Eighteen of this leaf's twenty-three tests say they
discharge no claim at all — check that each of those is honestly claimless rather
than a claim nobody looked for.
