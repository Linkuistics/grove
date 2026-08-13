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

## Decisions taken during grilling

The durable record is `docs/specs/mandate-delivered-methodology.md`, *A unit
narrows when a kind added later would not need it*. This log is the session's
own; it dies with `.grove/`.

**Measurement first, because nothing here was observable before the composer
landed.** An independent parser over `content/` was cross-checked against
`tests/goldens/composed-mandates.tsv` and reproduces its per-kind unit counts
exactly (53 / 54 / 58), so the byte figures rest on two agreeing readers. 140
units: 69 procedural (93,102 bytes, in no mandate), 71 triggering (58,901). Of
those, 51 are `kinds=*` totalling 45,032 bytes — the core every kind receives —
and 20 are narrowed, spreading 13,869 bytes. Per-kind mandates run 45.7 KiB
(`prototype`) to 49.2 KiB (`requirements`), against a 64 KiB alarm.

**D1 — nothing narrows beyond the endings.** The audit's whole target is the
44 KiB universal core, and the one unguarded candidate in it is worth 3.4% of a
mandate. Both candidates the ending split named were already spent: `skill-finish`'s
cycle body was narrowed *by* that split, and the review/chain units are largely
procedural already.

**D2 — the admissibility rule is recorded, recovered from the existing set
rather than invented.** *Does a kind added later need this unit?* A single-kind
scope answers no by construction — a new kind arrives with its own units and no
single-kind scope ever widens for it — so it narrows free. A set-shaped scope may
answer yes, so it narrows only behind a check derived from the closed kind set.

Two drafts were discarded getting there, and both failures are worth keeping.
The first keyed on machine-derivability and rejected `task-producer-*`, the most
obviously correct narrowings in the embed. The second keyed on the unit's
*subject* — "states one kind's own discipline" — and split the twenty 18/2; that
survived until the units were recounted one by one, when `task-review-kinds`,
`task-integrate-review-kinds` and `task-research-pair` turned out to state a
*family's* discipline across several kinds. Scope **shape** is the property that
carries the rule; subject is the property that reads as though it does.

**D5 — three existing set-shaped scopes are unguarded, and a leaf is cut for
them.** The honest split is 16 single-kind / 4 set-shaped, and only `skill-signal`
of the four is covered. `task-review-kinds`, `task-integrate-review-kinds` and
`task-research-pair` carry the mirror hazard with nothing on it. Each is exactly a
family partition, so each is derivable — `Kind::is_producer` (`src/leaf.rs`) is
the in-repo precedent, an exhaustive `match` kept deliberately in place of a
roster lookup so a new variant fails to build until classified. Adding a kind goes
red at that match and at the ending guard, and neither red points at these three
markers. Cut as `family-scope-guard`, inserted **ahead** of
`templated-mandate-k12` because the human placed that leaf last deliberately.

**D3 — one reopen condition, two demoted in writing.** Live: a unit shown wrong
for the kinds it reaches. Demoted with reasons, so a future reader meeting a
large mandate does not re-run this audit: *measured size* calls for
reclassification, not scope (narrowing can only redistribute, at a proportional
ceiling), and *a grown kind set* leaves per-session cost invariant while making
explicit lists less auditable — pointing away from narrowing.

**D4 — `skill-starting-a-new-grove` is recorded as the worked example, not as
pending work.** 1,622 bytes at `kinds=*`, genuinely reachable by one kind only
(the scaffolded first leaf's kind is a driver constant), and left universal
because the guard it would need couples a `content/` marker to Rust.

**No ADR.** The when-to-write test fails on reversibility — reversing this costs
a prose edit and nothing else, no state is stored under either answer. D2 also
refines reasoning `mandate-delivers-the-methodology` already records rather than
overturning it. **No new test seam**: this increment writes no code, and drift in
the narrowed set is already caught — a scope change moves rows in the composition
golden.
