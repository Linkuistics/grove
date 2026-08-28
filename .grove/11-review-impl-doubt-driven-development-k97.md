# doubt-driven-development-k97

## Goal

Read `doubt-driven-development-k95`'s edit adversarially. **This is not the first
adversarial read** — the producer spent its in-session allowance, and that
reviewer returned substantive findings on six claims. **It is the first read of
the repairs**, and the repairs are where the new risk is: fixing an overstatement
by adding qualification writes *new* assertions, and those went in with nothing
fresh reading them.

**Reviews:** `doubt-driven-development-k95`. Read that commit's diff against the
current source.

## Context

- [`docs/review-yield.md`](../docs/review-yield.md) — **the authority for every
  number in the edit**, and the document the producer was found to have
  overstated twice. §*A 100% survival rate is what a broken instrument reads*,
  §*The in-session channel*, §*Why the curve cannot be read*, and the closing
  §*What this record does not establish*.
- [`docs/candidate-lessons.md`](../docs/candidate-lessons.md) §6 — the relay
  finding and both of its weakenings.
- [`docs/results-of-formal-methods-trial.md`](../docs/results-of-formal-methods-trial.md)
  §3B — the proposal, plus the landing note k95 appended to it. That note itself
  claims two of §3B's own sentences overstate their source; it is checkable and
  worth attacking.
- `content/references/decompose.md` and `content/references/integrate-review.md`
  — the Grove-owned rule the skill cites rather than restates, under
  [`grove-binds-without-the-plugin`](../docs/adr/grove-binds-without-the-plugin.md).
- [`plugins/linkuistics/skills/authoring-conventions/SKILL.md`](../plugins/linkuistics/skills/authoring-conventions/SKILL.md)
  — the house rules the additions have to meet, the no-op test especially.

## Done when

Each of these has been checked at the source rather than reasoned about, and
every finding is recorded whether or not it is actionable.

- **The repairs are graded, not the draft.** Four passages were written *after*
  the only adversarial read and have never been contested: the *Measurements*
  callout's two-instrument claim (derived at tree level, hand-counted in the
  in-session channel); the counting caution's *≈1.8σ / roughly 25 matched chains
  / unmatched subjects* paragraph; the *four late chains verified against the
  artifacts* counter-evidence sentence; and the relay's second weakening (*one
  node, selected for review a third time because it was the hardest*). Each is a
  claim about `review-yield.md` and `candidate-lessons.md` — open those and check
  the sentence, not the gist.
- **Every remaining figure reproduces**: 45 raised / all graded valid, 5 of 9
  chains, 4 of 24 (17%), 63 invariants, nine artifacts, three spends, five
  declines, four reasons, 6.2 and 3.5, 1.0 to 3.3. A figure the source qualifies
  in a way the skill drops is a finding even where the arithmetic is right.
- **The grading language is checked against the skill's own taxonomy.** The
  producer changed *rejected* to *graded something other than actionable*
  because the skill defines a *valid trade-off* as real-but-accepted. Check no
  surviving sentence still reads 4 of 24 as a rejection rate, and check the
  tree-level *0 rejected* contrast is stated in the same vocabulary.
- **The Grove citation is right in both directions.** It must align with and
  cite `references/decompose.md` / `references/integrate-review.md` rather than
  restate them, name what binds with the plugin absent, and the reverse claim —
  *outside a Grove mandate there is no leaf body to write, so nothing is lost* —
  must be true rather than convenient. Two live questions the producer decided
  and did not escalate: whether the plugin→`content/` citation owes a row
  anywhere (no registry covers this direction; `tests/plugin_fallback.rs` is
  scoped to the corpus), and whether RECONCILE's *what this does not fix*
  paragraph is a chartered residue or the duplication
  `corpus-rules-have-one-owner` exists to remove, given
  `content/references/decompose.md` states the same qualification.
- **The escalated-review path is coherent with the aiming rule.** The skill
  mandates writing a specific doubt into a new review leaf's body *and* says
  never to count an aimed reader as an independent sample. The producer
  reconciled those in the Composition section; check the reconciliation lands as
  behaviour a session can follow, and not only as an argument.
- **The additions earn their lines.** ~130 lines went into a model-invoked
  skill, and most of it is evidence narration for one workstream. Run
  `authoring-conventions`' no-op test sentence by sentence and name the ones
  that fail; the body is 407 lines against a ~500-line threshold, so the budget
  is a real constraint and not a formality.

## Notes

**A finding this review is specifically placed to catch.** The producer graded
its own reviewer's findings and folded in the ones it accepted — which is
exactly the shape §4 of the skill now warns about, one level up: this leaf's
body deliberately carries **the doubts, not the reviewer's finding list**, so
that rejecting one is not rejecting this leaf's charter. If that reads as
under-specified, that is the rule working.

**Latent, and cheap to check.** `tests/composition_guidance.rs`'s
`every_handoff_passage_retires_before_it_commits` extracts the passage between
two anchors in this skill and asserts *retire* precedes *commit* inside it. The
new Grove-citation paragraph sits immediately before the opening anchor and
contains the word *commit*; it passes only by position. Worth a note on whether
that boundary should be pinned differently.
