# skill-delivered-methodology-k3

**Reviews:** skill-delivered-methodology-k2

## Goal

Adversarially review `docs/specs/skill-delivered-methodology.md`. The artifact is
load-bearing — the whole rewrite of `content/`, the deletion of the mandate
machinery, and the ADR rework all build on it for months — and it settles a
reversal whose two failure modes were both *measured*, so a wrong call here is
paid for on a real Grove run.

Read the producer's commit (named by handle `skill-delivered-methodology-k2`)
against the current source. `plan-k1` and the root `BRIEF.md` carry the decisions
this spec consumes; `docs/specs/mandate-delivered-methodology.md` is the design it
replaces and is still on disk.

## Context

The design's central bet is that **trigger strength** makes a provisioned skill
un-skippable where a mild launcher clause did not. Nothing else in the design
answers the first observed failure, and the bet cannot be settled by reading — so
the highest-value review is on the *arguments*, not on the prose.

## Done when

Findings recorded, ranked, with `path:line` citations. Six specific doubts the
producing session holds about its own work, plus whatever the review finds
independently:

1. **The wording is untested and the design says so.** The house authoring
   guidance prescribes micro-testing behaviour-shaping wording against a no-skill
   control before shipping it, and this design declines both that and a
   machine-checked receipt, leaving the human-watched run as the only instrument.
   Is a cheap pre-landing test genuinely unavailable, or was it waved off? This is
   the single highest-value line of attack: if a wording test is affordable, the
   design's one unfalsifiable claim becomes falsifiable before the increments run.

2. **The fact/rule split at the too-late test's boundary may license smuggling.**
   The *do not pick again* row admits the handle's authority as a **fact** while
   leaving the rule in the skill. Does that generalise, or can any rule be
   restated as a fact and walked into the core? If it can, the test has a hole
   exactly where erosion would enter, and it needs a closing clause.

3. **The 4 KiB alarm rests on a load instruction nobody has written.** The
   arithmetic assumes ~1.2 KiB for an instruction that must carry an imperative,
   an ordering clause enumerating alternatives, absolute paths for three
   harnesses, a five-row table, a not-a-summary clause, and an acknowledgement
   instruction. Draft it. If it lands near 2 KiB the alarm is tight rather than
   generous, and the honest fix is a different number, not a thinner instruction.

4. **The derived reference-file set is uneven and may be wrong at the thin end.**
   `references/design.md` inherits two small units (~600 bytes) while
   `references/requirements.md` inherits six plus `grilling.md`. Is a 600-byte
   reference file worth the pointer, or does the derivation produce files too thin
   to justify the hop the core spends naming them?

5. **The old spec's retention may be over-argued.** The design keeps
   `mandate-delivered-methodology.md` on the grounds that ~10 live source sites
   cite its sections. Test that: could those citations point at
   `docs/ARCHITECTURE.md`'s embedded-methodology section instead, letting the set
   hold one spec rather than two describing opposite designs? The producing
   session did not check whether the architecture doc already carries the same
   claims.

6. **Two requirement limbs are softer than their SHALL suggests.** *The loop fits
   on one page* and *SKILL.md states no procedure* are not mechanically checkable
   as written. Either bound them the way the design bounds its other prose limbs —
   by naming where the automated boundary stops — or replace them with something
   the suite can hold.

## Notes

This is an **inspection-only** session: no test, build, lint, or format commands;
no edits to the spec or to `content/`. Findings only. If they are worth acting on,
cut `integrate-review-design` with the same bare stem — and note that
`04-planning-skill-delivered-methodology-k4` is live and sits after this leaf, so
the integration wants `leaf-insert` at that entry rather than `leaf-add`, or
planning will consume an unintegrated spec.
