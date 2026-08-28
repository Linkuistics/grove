# doubt-driven-development-k95

## Goal

Land this campaign's findings *about the review loop itself* into
`plugins/linkuistics/skills/doubt-driven-development/`. The headline is one
sentence the skill does not currently contain, and it is the most actionable
thing the whole harvest produced.

The skill already controls bias in one direction — *never pass the CLAIM to the
reviewer*. This is the mirror image, on the reconcile side:

> **Never let the reviewer author the reconciler's contract.** A reader graded
> against a finding list it did not write can reject a finding; a reader whose
> charter *is* the finding list cannot.

## Context

- [`docs/review-yield.md`](../docs/review-yield.md) — the measurement, and the
  authority for everything here. §*A 100% survival rate is what a broken
  instrument reads* is the defect; §*The in-session channel, and the five leaves
  that declined it* is the second half; §*Why the curve cannot be read* is the
  counting caution.
- [`docs/candidate-lessons.md`](../docs/candidate-lessons.md) §6 — the relay
  finding, and how much of the campaign's most-quoted anecdote survived.
- [`docs/results-of-formal-methods-trial.md`](../docs/results-of-formal-methods-trial.md)
  §3B — the proposed landing sites, as a proposal rather than a specification.
- `content/references/decompose.md` and `content/references/integrate-review.md`
  — where `methodology-changes-k91` already landed this rule **for Grove** (rule
  C). The skill must **align with and cite** that, not restate it: under
  [`docs/adr/grove-binds-without-the-plugin.md`](../docs/adr/grove-binds-without-the-plugin.md)
  the rule is Grove-owned, because its absence changes what a session *writes*.
- The skill's own *Composition with Grove* section, which is already
  Grove-aware and is the place that has to stay consistent with `content/`.

## Done when

- **The reviewer/contract rule has landed**, sited where a session meets it —
  §4 RECONCILE and the *Red flags* list — with the asymmetry as its evidence: 45
  findings raised at tree level and **0 rejected**, against **4 of 24 (17%)**
  rejected in the channel where the reader owed the finding nothing.
- **The rule states its own residue**, as `content/`'s version does: the finding
  list is still the reviewer's, so the repair moves the hazard one level down
  rather than removing it. A repair that oversells itself is the failure this
  skill is about.
- **When-to-use is sharpened**: spend a reviewer when it is a **different
  instrument**, not a second opinion — 3 for 3 on that framing, with the three
  outcomes named. And **declining is a real answer**, carrying the four evidenced
  reasons rather than a generic "when NOT to use".
- **The *doubt theater* signal carries its counting cautions.** That signal is
  count-based, and this campaign measured why a count misleads: findings per
  review fell 6.2 → 3.5 and **that was a selection effect of a rising review
  rate, not decay**; and without a severity scale fixed *before* the reviewer
  starts, counting findings measures enumeration style — 1.0 to 3.3
  sub-corrections per finding.
- **The relay caveat qualifies the diverse-lens advice**: three readers who each
  found what the previous two missed were **not independent**; each wrote its
  doubt into the next one's charter. A chain of three *unaimed* readers is a
  different experiment, and this workstream did not run it.
- **Consistency with `content/` is checked, not assumed** — the skill's
  *Composition with Grove* section and Grove's rule C say compatible things, and
  the citation states what binds in the skill's absence.

## Notes

**Scope discipline.** This is a small, sharp edit to one skill. It is not a
rewrite of the review-chain policy, and it is not the model-suite cluster — that
is `model-led-development-k94`. If it starts growing into either, stop and cut.

**The evidence's own limit travels with it.** `review-yield.md` says the
structural reading cannot be separated, on nine chains, from *reviewers who were
simply right every time*. That is precisely why the repair is the cheap
structural one rather than a claim about anybody's accuracy — and the landed rule
should say so rather than overstating what nine chains can show.
