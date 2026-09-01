# pilot-measure-k26 — brief

## Goal

Run the five editorial stages that follow the draft over the `jj-workspace` book
— one stage, one session, one commit each — and then produce the measurement
report the preregistration demands, saying per stage whether it paid for itself.

## Done when

- Each of developmental edit, technical edit, copy edit, art and proof has been
  run and has its **own commit**, so that what it changed is recoverable from a
  single diff.
- Final validation over the book is green after every stage, not only at the end.
- A report is committed under `docs/evaluations/`, evaluated against
  `pilot-preregistration-k24`, stating per stage: what it changed, what the judged
  outcome says about it, and which of keep / merge / drop the decision rule
  selects.
- A stage the evidence says to drop is reported as such. A result that says two
  stages beat six is a successful outcome of this node.

## Decomposition

Six leaves: one per remaining stage, in the order the pipeline runs them, then the
report.

1. `developmental-edit-k40`
2. `technical-edit-k41`
3. `copy-edit-k42`
4. `art-k43`
5. `proof-k44`
6. `measurement-report-k45`

**This was one leaf, and `walkthroughs-k9`'s F2 is why it is six.** As cut, a
single leaf required five stage commits plus a committed report from one session,
which contradicts grove's one-task-one-focused-commit boundary and
`walkthroughs-k3` decision 1's claim that the subtree's leaves each fit one
session. The old leaf's escape hatch — decompose *if* it proves too large — named
its own proof in advance, and its "one child per stage" left the report with no
owner. The sequence was known, so it is cut rather than deferred.

One commit per stage is not bookkeeping: it *is* the attribution rule's
instrument. The diff between two commits is the record of what a stage changed,
and a session that folds two stages into one commit has destroyed the evidence
the whole pilot exists to gather.

## Pointers

- The preregistration is `pilot-preregistration-k24`'s committed document under
  `docs/evaluations/`. The judged outcome, the alternative to beat, the
  attribution rule and the decision rule are fixed there and are **not reopenable
  in this node**. A part that turns out to be unworkable is reported as a protocol
  breach, not quietly replaced.
- The draft is `jj-workspace-book-k25`'s **commit range**, not one commit, and it
  is the unedited baseline every later stage is measured against. The leaf
  decomposed into one child per slice of the book's own sequence — seven commits,
  `orientation-k55` through `what-jj-owns-k61` — which the preregistration's
  *Part 3* admits as the draft's one exception: the stage is every commit from the
  first drafting commit to the last inclusive, its baseline is the commit before
  the first of them, and the draft is not scored, so a range costs the measurement
  nothing. **Diff against the range, never against its last commit** — child 7 is
  the synthesis chapter alone and is a seventh of the baseline.
  `docs/evaluations/editorial-pipeline-pilot/stages/0-draft.md` is authoritative
  for the whole of it: `## Provenance` lists all seven change ids with the
  book-directory digest after each, and `## Baseline` carries the final page
  inventory, the final digest
  `0e9d3f1e7d356c33509327dfe4df584ffa6ca531e531049098cdcf6b923aca14`, and the
  green final validation. That record also carries `## Findings not fixed` — seven
  defects the draft found and did not fix, six of them with leaves under
  `crate-books-k14` — so a later stage does not re-report them as its own.
- The stage list is decision 11 of `plan-k1`: draft, developmental edit,
  technical edit, copy edit, art, proof. The alternative they must beat is draft
  plus proof alone.
- `docs/evaluations/writing-code-walkthroughs/` is the in-house precedent for
  reporting a shortfall honestly instead of completing it.

## Notes

**Art runs with what Markdown gives it** — no asset machinery and no validator
concept of a figure. `figure-contract-k18` decides afterwards, from this node's
report, whether that machinery is earned.

**The corpus is frozen.** No session here edits `crates/jj-workspace/`. A defect
found while editing becomes its own leaf under the root brief's cross-book rule.

**Finish every edit, then measure.** An instrument adjusted mid-reading has not
read anything. The stage leaves are the edits; the report leaf is the reading, and
it is a separate session for exactly that reason.
