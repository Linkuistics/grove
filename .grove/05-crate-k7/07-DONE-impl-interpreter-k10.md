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

## Decisions (running log)

**Both model suites re-run before anything was written, and both are green** —
Alloy 20/20, Quint every claim across all eight instances with every witness
reached in a non-zero number of traces. No model was changed by this leaf, so
every disagreement found later is between the code and a model that still says
what it said.

**The plan holds two effect variants and the interpreter holds two undos.**
`operations.qnt`'s `Effect` has three, with a comment saying `Remove` never
appears in a forward plan. In Rust that comment is a type: `Effect` is
`Create | MoveTo`, and the interpreter's own `Undo` is `Remove | Restore`,
constructible only from an effect the run just applied. `inv_rollbackRemovesOnlyItsOwn`
— *rollback removes only entries the run itself created* — becomes structural.
The same move on `Decision`: two variants, no third, which is the totality the
model gets from `decide`'s return type.

**`MoveTo` and `Level::Created` are implemented though no operation builds them
yet.** They are the model's own plan shape (`planPromote` builds exactly the
create-then-move-into-it pair), and the rollback that puts a moved entry back is
*shared* machinery — leaving it out would hand `insert-k11` the job of writing
unwind semantics this leaf exists to own. They are exercised by hand-built plans
in this crate's own tests and marked `#[cfg_attr(not(test), expect(dead_code))]`,
so the leaf that finally builds one is forced to remove the marker rather than
inheriting a stale allowance.

**The destination check folds the plan through the snapshot, and there is a test
that fails if that is ever rewritten.** Entry 003's second finding, and the
reason it needed a test no `append` can motivate: an append composes names
carrying a key no entry holds, so its destinations are free under either reading.
The discriminating case is a shift on two siblings sharing a key *and* its parts,
which is `insert`'s plan on a hand-corrupted tree — so that plan is built by hand
in `src/plan/tests.rs`, in both orders, now.

**A mutation consumes its guard.** One guard, one mutation, and the lock drops
with it. A guard that outlived its mutation would plan the next operation from a
snapshot describing a tree that no longer exists — a second `append` would
compute the same fresh key and collide — and refreshing the snapshot instead
would let a mutation that succeeded return the error of the read that followed
it, which is precisely the shape *plan atomicity* promises not to have. So an
invariant that says nothing about handles decided the handle. Recorded in
`ARCHITECTURE.md` under *How an operation runs*.

**Three refusals neither model can pose, all three now in the document.**
`ContentForANode`, because a directory has nowhere to hold bytes and discarding
them silently is not an option; `KeysExhausted` and `OrdinalsExhausted`, because
an integer in either model is unbounded and a key is 32 bits — a hand-written
name carrying `u32::MAX` makes `max + 1` impossible, and wrapping would re-issue
a key that is still referenced. `ARCHITECTURE.md` is the specification of record
for what the models do not reach, so that is where they went.

**The interpreter claims destinations by two mechanisms, not one.**
`create_new` and `create_dir` refuse an occupied destination in a single
syscall. `rename(2)` **replaces** its destination silently and macOS has no
portable no-replace rename, so a `MoveTo` looks first with `symlink_metadata` —
unfollowed, so a symbolic link occupies a name whatever it points at — and an
occupancy that cannot be determined is a refusal rather than an assumption. The
window that leaves is against a writer who ignored the advisory lock, which is
already outside what the library defends.

**Nine mutation controls, and the ninth found a hole.** Each patched, run and
reverted by script. Eight fired exactly the tests they should; deleting the
rename's pre-rename look fired **nothing**, so the whole rename claim was
untested — `a_rename_looks_before_it_leaps` was written for it and the control
re-run against it. That is entry 009's counterfactual: a property implemented by
two mechanisms wants two controls.

**Twenty-eight of forty-two tests name a model claim; fourteen say they have
none.** The reading layer's ratio was five of twenty-three. The difference is
that this leaf's subject is the model's subject, and that measurement is the
leaf's main H3 evidence rather than a by-product.
