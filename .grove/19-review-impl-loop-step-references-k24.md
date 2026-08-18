# loop-step-references-k24

**Reviews:** loop-step-references-k11

## Goal

Adversarially read `loop-step-references-k11`'s rewrite of the seven loop-step
reference files. The producer rewrote `bootstrap`, `execute`, `decompose`,
`retire`, `commit`, `driver` and `grove` — most of what every future session
reads — so a rule silently dropped here is dropped for every session of every
kind until someone notices from behaviour.

That failure mode is not hypothetical in this grove: `skill-router-k20` found
`durable-artifact-set` landed in no owner at all, and `kind-references-k22` found
three rules whose `SKILL.md` trigger terminated at a file saying nothing about
them. Both were the same shape — a removal that outran its landing — and both
were found by a fresh context reading the corpus rather than the diff.

## Context

- `docs/specs/corpus-rule-ownership.md` — the inventory. The per-file tables for
  the seven files above are the checklist; every row must be stated by its owner.
- `docs/adr/corpus-rules-have-one-owner.md` and
  `docs/adr/restatement-declares-its-class.md` — the placement function and the
  three restatement classes.
- `tests/rule_ownership.rs` — the producer's own **S** sweep. Read it as a claim
  to be attacked, not as evidence: it is phrase-scoped by construction, so it
  cannot see a second statement written in different words.

## Done when

Each of these is checked and its verdict recorded, with `path:line` citations:

- **Nothing was deleted rather than rehomed.** Walk the inventory's rows for the
  seven files and confirm each is stated by its owner. `retire.md` gaining
  `finish-is-the-drivers-to-discover` while `references/finish.md` loses it is the
  one cross-file *move* in the commit — confirm the landing and the removal are
  both in it.
- **The `SKILL.md` edges still terminate.** Every one of the 26 trigger sentences
  names a file; confirm the file it names now states the rule the sentence is
  for. This is the exact defect `kind-references-k22` found, one leaf earlier, in
  the same kind of removal.
- **The relocations to `docs/` took argument and not rule.** `grove.md`'s spine
  and glossary sections moved to `docs/ARCHITECTURE.md`, which no session outside
  this repository can open. Confirm nothing normative went with them — in
  particular that constraint 4's *just-in-time, not few* clause and the glossary's
  inline obligation still bind from `content/`.
- **The shed material was command fact, not conduct.** `decompose.md` dropped the
  grow verbs' mechanics and `driver.md` dropped its dispatch and install prose.
  Confirm no *when-to-reach-for-it* judgement left with them.
- **The `S` sweep is not passing for the wrong reason.** `tests/rule_ownership.rs`
  excludes `content/driving.md` on the ground that no corpus file names it. Attack
  that: is the file genuinely off every loaded path, and does any row's phrase
  admit a differently worded duplicate the sweep cannot see? Three rows declare a
  transient `TASK-FORMAT.md` site — check whether more should.
- **The reconciled tests still assert what they claimed.**
  `tests/composition_guidance.rs` lost surfaces from five assertions as their
  rules acquired single owners. For each, confirm the claim is still made
  somewhere, and that the removal dropped a duplicate rather than a check.
- **`docs/specs/doubt-grove-review-mechanics.md` kept what is its own.** It now
  cites `execute.md` and `decompose.md` instead of restating the review budget,
  the placement rule and the no-exception decision. Confirm nothing that was only
  in that spec — the ownership predicate's rationale, the walk's three properties,
  the access seam, the handoff, the test seams — was lost in the trade.

## Notes

- Inspection only: read the committed diff, the corpus and the tests; run no
  build, test, lint or format command, and edit nothing.
- Findings only. If any are worth acting on, cut
  `integrate-review-impl` as this session's last act, placed by the rule in
  `references/decompose.md` — `corpus-split-k6` sits immediately after this leaf
  and rewrites the same corpus, so the integration must land before it.
- Cut ahead of `corpus-split-k6` rather than appended, for the reason
  `rule-ownership-k12` was: `k6` moves rationale out of `driving.md` and
  `TASK-FORMAT.md` on the assumption that these homes are correct.
