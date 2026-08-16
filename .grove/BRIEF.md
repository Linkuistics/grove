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

Not yet cut. One `design` leaf is live; it owes a spec, and the decomposition
into increments follows from it.

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

- **Two paths can disagree.** A guaranteed core plus a skill is the two-path
  state the superseded ADR rejected outright. At a couple of KiB the drift risk
  is small but real, and the design owes a *rule* for what earns a place in the
  core — not a list, which goes stale.
- **Trigger strength is a first-class question**, not a detail: the frontmatter
  `description:`, how forcefully the launcher words the instruction, and how
  `SKILL.md` opens are what decide whether the skill is read this time.
- **The shared namespace question reopens.** `CONTEXT-MAP.md` recorded the
  `grove` entry in the personal skill directory as going away; it stays, so
  precedence and double-provisioning against the `linkuistics` symlinks is an
  open question again rather than a removed one.
