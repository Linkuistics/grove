# unit-scope-audit-k4

## Goal

Grill whether the `kinds=*` default should be narrowed across the embed at all,
and if so what guards a narrowed scope. This is a **separate increment from the
composer**, surfaced by `specialised-ending-k2` and deliberately declined there.

The first question is whether to do it, not how. A grilling that concludes *leave
every other unit at `kinds=*`* is a good outcome and costs one session.

## Context

`specialised-ending-k2` narrowed exactly the units carrying a session ending, and
recorded why it went no further in
`docs/specs/mandate-delivered-methodology.md` — see the *Out of scope* entry *A
systematic audit of every unit's scope*, which states the trade and the reopen
condition. Read that before anything else; it is the input to this grilling.

The trade, in short. `kinds=*` is the safe default: a unit scoped to `*` reaches
every kind, so it cannot be *omitted* from one. Narrowing a unit to a list buys
tokens and costs a slice of the completeness invariant's protection, because a
kind added later is silently missing from every list that was not updated. The
ending specialisation paid that cost once, on one instruction, with a test
covering exactly the hazard it introduced (*Requirement: Every kind's mandate
states exactly one session ending*). An audit proposes to pay it across the whole
embed, and there is no general analogue of that test — the ending guard works
because "every kind must state an ending" is a universal claim over a closed set,
and "every kind must have the guidance it needs" is not checkable.

Concrete candidates observed while writing the spec, as evidence that the
question is real rather than as a work list:

- `skill-finish`'s cycle body — teardown steps, sentinel mechanics, the human
  gate — is read by one kind. Note that the ending split already moves most of
  this, so re-measure before treating it as a candidate.
- The `review-ownership` and chain/pair-cutting units are plausibly narrower than
  `*`, though several are read by producers deciding whether to cut a review.

## Done when

- A decision is recorded on whether to narrow anything beyond the endings, with
  the reason, in the spec (edited in place — the *Out of scope* entry either goes
  or gains the outcome).
- If the answer is yes: what guards a narrowed scope in general, decided *before*
  any unit is narrowed. A per-unit judgement with no mechanical guard is the
  shape the design has so far refused.
- If the answer is no: the spec says so, and the reopen condition is restated in
  terms of what would change the answer (measured mandate size, a kind set that
  grew, a unit shown to mislead the kinds it reaches).

## Notes

**Sequence this after the composer increment.** Nothing here can be measured
until mandates are actually composed per kind — the whole question turns on what
a mandate costs and what it contains, and today neither is observable. A
`planning` session cutting composer slices should `leaf-insert` them **ahead of
this leaf** rather than appending after it; this leaf sits at the root only
because that is where `leaf-add` puts it, and its position carries no claim to
run next.

**Do not treat the ending specialisation as a precedent for narrowing.** It
narrowed a unit whose correct scope was mechanically decidable, because the
driver resolves the kind. Most units are not like that.
