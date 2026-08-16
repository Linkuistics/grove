# grove.move-prompts-back-to-skills — brief

## Goal

Retire mandate-delivered methodology and return Grove's `content/` to the
harness's own skill mechanism — restructured for progressive disclosure, not
merely re-plumbed — while keeping a **short guaranteed core** in `${prompt}` so
the reversal does not walk back into the failure the mandate was built to fix.

## Done when

- A session receives a `${prompt}` of a couple of KiB — a forceful pointer to the
  provisioned skill, the kind's reference file, the two runtime facts, and the
  conditions that must never be missed — instead of ~49 KiB of sliced prose.
- `content/` is a progressive-disclosure skill: a short `SKILL.md` of conditions,
  `references/` carrying the procedures.
- Skill provisioning is the delivery path again, restored on the `launch.rs` /
  `loop_driver.rs` seam it never left.
- The mandate machinery is gone: composer, marker grammar, fence-state parser,
  build gate, `grove-llm methodology`.
- `mandate-delivers-the-methodology` and `mandate-delivered-methodology` are
  **reworked in place** — the ADR/spec sets are current-state, so no superseding
  record is appended.
- A real Grove run, with a human watching, shows sessions both **ending** and
  **reading the skill**. Both are the bar; one without the other is a swap.

## Decomposition

The spec is written — [`skill-delivered-methodology`](../docs/specs/skill-delivered-methodology.md)
— reviewed, and cut into increments. In order:

1. `wording-micro-test-k6` — the two-arm micro-test, first because the spec makes
   it an ordering constraint: run after the rewrite lands, it is a post-mortem.
2. `corpus-rewrite-k7` — the rewrite of `content/`, as a node of five children.
3. `absent-skill-destination-k8` — the driver's one new report.
4. `guaranteed-core-k9` — the cutover: the `prompt` seam, and both record reworks.
5. `mandate-machinery-k10` — the deletion, and the old spec with it.
6. `delivery-acceptance-k11` — the human-watched run, both limbs.

**The one ordering constraint the brief inherited is satisfied by construction,
not by sequencing.** `${prompt}` must not shrink before `SKILL.md` is short — and
under the live machinery those are the *same edit*, because the mandate is the
union of triggering units and `SKILL.md` is destined to be the union of triggering
conditions. So there is no increment that does one without the other, and the
spec's open question — one increment or two with a flag — resolves to **neither,
and no flag**. `07-corpus-rewrite-k7/BRIEF.md` carries the working.

## Pointers

- ADRs a session here must read:
  [`mandate-delivers-the-methodology`](../docs/adr/mandate-delivers-the-methodology.md)
  (superseded, awaiting rework),
  [`one-build-owns-a-session`](../docs/adr/one-build-owns-a-session.md)
  (its pairing story changes back — provisioning survives, so the shared mutable
  directory and the stamp survive with it).
- Specs: [`mandate-delivered-methodology`](../docs/specs/mandate-delivered-methodology.md)
  — 1,640 lines describing machinery this grove deletes. Read for *why* each
  device exists before deleting it; several of its arguments survive their
  mechanism.
- Glossary terms in play, all needing rework as the mechanism changes:
  Global skill provisioning, Methodology unit, Mandate slice, Triggering unit /
  procedural unit, Methodology identity, Build pairing (see `CONTEXT.md`).
- The unit classification is **scaffolding for the rewrite, not an obstacle to
  it**: 140 markers already record which prose is `if` and which is `then`, which
  is exactly the split a progressive-disclosure skill needs. Use it, then delete
  it.

## Notes

**The central tension, which every leaf here inherits.** Two failures are
*observed*, not theorised, and they pull opposite ways:

| deliver via | failure |
|---|---|
| `${prompt}` | the wall degrades behaviour — sessions finish and fail to signal, stalling the loop |
| provisioned skill | sessions demonstrably did not read it |

A design that answers one is a swap. The short guaranteed core exists to answer
both: `${prompt}` is the one channel a session cannot skip, so the conditions
that must never be missed ride it, and everything else moves off it.

**The superseded design's own words carry the reversal.** Its rejection of
*"point at locations instead of slicing"* names a live reopen condition —
*"Reopen if `content/` is ever restructured so that every rule is separately
addressable"* — which the 140 markers satisfied. Its rejection of behavioural
verification names the check to trust — *"the next real Grove run after the
change lands, with a human watching"* — which has now run and failed. What the
reversal genuinely **overturns** is the same clause's *"never as a replacement
for triggering conditions"*, and the rework owes that an argument.

## On the horizon

The three questions this brief foresaw are settled in the spec — the core's rule
is the **too-late test**, drift is answered by one source rather than by size,
trigger strength has its own section, and the shared namespace turns out to
collide with nothing. What is left over from them:

- **Trigger strength is the one claim no reading settles.** The design's answer to
  the *observed* prior failure is wording, and wording is judged by a real run.
  The grove's `Done when` is the instrument, and both limbs are required.

  **`corpus-rewrite-k7` closed with the trade-off already visible in a number,
  and promoted it here rather than resolving it.** That node derived a bound of
  *~200 lines, roughly 8 KiB*; `content/SKILL.md` landed at 399 body lines and
  ~25 KiB (277 lines / ~19.7 KiB once unit markers and blanks — both of which die
  at `mandate-machinery-k10` — are excluded). The derivation assumed **one line
  per condition**; the rewrite wrote each as a short paragraph carrying its
  reason, which is the thing that makes a condition fire. The spec's *checkable*
  ceiling is 500 body lines and is met with room, so nothing is in breach — the
  brief's ~200 was arithmetic, not a requirement. But it means the corpus has
  already spent 2.5× its estimate buying trigger strength, and
  `delivery-acceptance-k11` is judging whether that bought anything. If the run's
  *reading* limb passes, the estimate was simply wrong and the spec's number
  governs; if it fails, condition wording is the first thing to change and the
  size is evidence about which direction to change it in.
- **`leaf-prune`'s HITL rule is guarded by prose alone.** The spec names it as a
  live gap and deliberately does not widen the core to cover it; a guard in the
  verb is the shape that would, and it is a confirmation-boundary question rather
  than a delivery one.
- **A launch target that cannot read a provisioned skill now gets nothing**, and
  the driver cannot detect it. Named and accepted, not solved.
