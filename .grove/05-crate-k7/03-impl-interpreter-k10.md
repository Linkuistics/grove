# interpreter-k10

## Goal

The machinery every mutation is built out of: the plan and effect types, the
algebra's `Decision`, the single interpreter that applies a plan and unwinds
what it applied, and the two operations that make all of it observable —
`append` and `append_many`.

## Context

Beyond the brief chain:

- `ARCHITECTURE.md` — *How an operation runs*, *The plan is checked against
  itself, in order*, *When rollback fails*, and the invariant *Plan atomicity*.
  Note the two rejected shapes it names (pure functions over name lists;
  read-transform-diff) — they are in the document because each is what a reader
  would otherwise reach for.
- `operations.qnt` — the `failures` instance, where atomicity and rollback are
  checked, and the invariant that the interpreter never finds a destination
  taken. That last one is **dead for every tree the library builds**: it fires
  only on a duplicated key or a wrecked rollback. Keep the check anyway — the
  lock is advisory and an uncooperative neighbour remains — and write the reason
  beside it, because without it the check reads as dead code to whoever next
  tidies up.
- `docs/formalism-findings.md` entry 003, on the sequential destination check:
  checking every destination against the *snapshot* is the obvious reading of "a
  pure function of the snapshot" and it is the wrong one. The algebra folds the
  plan through the snapshot so it meets each destination in the state the
  interpreter will meet it. Getting this wrong makes the next leaf's ordering
  rule vacuous, and nothing will tell you.

## Done when

- `Plan`, `Effect` and `Decision` exist; a `Decision` is a plan or a refusal and
  there is no third answer. Every operation is total.
- The interpreter applies effects in order, claims each destination with an
  exclusive create, and unwinds the effects it applied when a later one fails.
  It removes only entries the run itself created.
- An internal seam makes an effect fail on demand. It does not appear in the
  public API.
- `append` and `append_many` work: the next free ordinal, a fresh key of
  `max + 1` over the whole tree, consecutive ordinals and consecutive keys for a
  run, planned from one snapshot and applied as a unit.
- Atomicity is tested across a **multi-effect** plan with an injected failure —
  after the error, either every effect landed or none did. A single-effect
  `append` cannot show this, which is why `append_many` is in this leaf.
- The rollback promise is bounded in the code as it is in the document: reported
  errors, not process death, and not a rollback that itself fails.
- Each test names the model claim it discharges, or says it has none.
- An entry in `docs/formalism-findings.md`.

## Notes

**The interpreter is one rollback, shared.** That is the whole reason the plan is
a value rather than a set of hand-written procedures — five operations
hand-rolling their own unwind is five things that drift apart. If an operation
later needs a different rollback, that is a finding about the plan shape, not a
licence to add a second one.

**A key a failed operation created and then rolled back was never committed**, so
allocating it again is correct: the counter appears to go backwards and nothing
was reissued. The invariant is about **allocation, not creation**. Read it the
obvious way — no newly created object carries a key seen before — and it is
false, which the model found and the document now states.
