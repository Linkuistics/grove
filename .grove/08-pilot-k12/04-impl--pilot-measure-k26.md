# pilot-measure-k26

## Goal

Run the remaining editorial stages over the drafted `jj-workspace` book, one
commit per stage, and produce the measurement report the preregistration
demands — including, per stage, whether it paid for itself.

## Context

- The judged outcome, the alternative to beat, the attribution rule and the
  decision rule are all fixed by `pilot-preregistration-k24` and are not
  reopenable here. If one of them turns out to be unworkable, say so explicitly
  in the report as a protocol breach rather than quietly substituting another —
  `docs/evaluations/writing-code-walkthroughs/` is the in-house precedent for
  reporting a shortfall honestly instead of completing it.
- The stages remaining after the draft: developmental edit, technical edit, copy
  edit, art, proof (decision 11 of `plan-k1`). The alternative they must beat is
  draft plus proof alone.
- One commit per stage is what makes the attribution mechanical: the diff between
  two commits *is* the record of what that stage changed.
- Art runs with what Markdown gives it — no asset machinery, no validator concept
  of a figure. That is deliberate; `figure-contract-k18` decides afterwards, from
  this report, whether the machinery is earned.

## Done when

- Each remaining stage has been run and has its own commit, so that what it
  changed is recoverable from the diff.
- Final validation over the book is green after every stage, not only at the end.
- A report is committed under `docs/evaluations/`, evaluated against the
  preregistration, stating per stage: what it changed, what the judged outcome
  says about it, and which of keep / merge / drop the decision rule selects.
- A stage the evidence says to drop is reported as such. A result that says two
  stages beat six is a successful outcome of this leaf.
- `bash scripts/check.sh` passes.

## Notes

**Finish every edit, then measure.** An instrument adjusted mid-reading has not
read anything; a rubric or a corpus rewritten under the run that is consuming it
makes the whole reading undefined rather than merely untidy. Restarting is
cheaper than a run you have to throw away.

**Confirm any re-run item by item, not by matching totals.** Two runs can agree
on a total while disagreeing about which item did what.

**This is the evidence `pipeline-kinds-k27` extracts from, and there is no human
between the two.** If the report leaves a stage's verdict genuinely undetermined,
say so in those words rather than rounding it to keep — an undetermined stage
that reads as kept is how six stages get shipped on the strength of nothing.

**If this proves bigger than one session, decompose it** — one child per stage,
doing only the first. Per-stage children are the natural seam, and each still
lands the book green.
