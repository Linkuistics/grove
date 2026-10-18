# publication-k18

**Integrates:** publication-k17

## Goal

Triage the publication review and apply the findings that the source and the
existing contract support. Prepare the skill for the finish cycle.

## Context

- Read the findings from `publication-k17`'s commit. Its report is
  `docs/technical-communication-publication-review.md`.
- The contract remains the spec `docs/specs/technical-communication.md`,
  section *Restatement discipline*, and
  `docs/adr/technical-communication-carries-no-standard-text.md`.
- The source is `~/Downloads/ASD-STE100_ISSUE9.pdf`. Read it as page images
  only. Never extract its text or put it in repository files or `.grove/`.

## Done when

- Every finding from `publication-k17` has an evidence-based classification
  under the integration procedure.
- Accepted findings are repaired and verified. Rejected findings and accepted
  trade-offs have a recorded reason.
- The twelve-file review surface, its coverage references and affected rule
  indexes remain coherent after any edits.

## Notes

The report's coordinates refer to producer commit `30fdcaa70734`. The review
changed no implementation file. The earlier producer's remaining repository
check and historical task-log question are recorded in `publication-k16`.

## Decisions (running log)

Source pages 124, 91, 87, 95, 77 and 107 were read as page images only, in a
scratchpad render outside the repository.

- **F1, a real issue: accepted and repaired.** The means example in
  `practices.md` made the source's paired transformation with software nouns,
  and the ambiguity example led into it with the same attachment skeleton. I
  rebuilt `with-one-meaning` around a worker restart that loads a saved queue.
  The means paragraph is now prose that points to `verb-for-action`, with no
  before/after pair, because `verbs.md` already carries that idea.
- **F2, a real issue: accepted and repaired.** The first correct note in
  `procedures.md` kept the source's settling-time scenario. It now states a
  background fact about an export's row order. The second note stays, as the
  review advised.
- **F3, a real issue under the contract's "a list in the standard's order"
  clause: accepted and repaired.** Each of `procedures.md`, `descriptions.md`,
  `sentences.md` and `punctuation.md` now teaches in its own order, in the
  summary table and the body together. The order runs from the decision a
  writer makes first to the finishing check, or from the unit to the whole.
  `word-count` stays last in `punctuation.md` and `dangling-conjunction` stays
  after `vertical-list`, because each depends on its predecessor. The `SKILL.md`
  rule index lists the four files' identifiers in the new order. The numeric
  `## Coverage` tables stay in ascending rule order, which the spec permits.
  A line-multiset comparison against the producer's commit showed the reorder
  moved lines and changed none. The other files' orders are untouched, since
  the review judged them clean.
- **The three trade-offs.** Necessary grammar, numeric coverage and generic
  identifiers are accepted as the review's verdicts stand. No ADR or spec
  changes: the findings apply the existing contract.
- **Verification.** `bash scripts/check.sh` passes 19 of 20. The failure is the
  Bun pin (1.4.3 installed, 1.4.2 pinned), the same one the producer recorded.
  It does not touch these files.
- **Open for the human, carried from `publication-k16`.** The finished leaves'
  logs in `.grove/` name the standard because they record the human's decisions.
  The producer left them, and this session did too.
