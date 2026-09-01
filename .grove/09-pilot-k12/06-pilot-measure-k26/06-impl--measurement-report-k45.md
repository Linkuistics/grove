# measurement-report-k45

## Goal

Apply `pilot-preregistration-k24`'s judged outcome, attribution rule and decision
rule across the six stage commits, and commit the measurement report: per stage,
what it changed, what the judged outcome says, and keep / merge / drop.

## Context

- The subjects are the six commits — the draft from `jj-workspace-book-k25` and
  the five stage commits in this node. **Digest them before and after the reading
  and say which they were**, so a run whose subject moved under it is visible
  rather than merely wrong.
- The preregistration is fixed. If one of its four parts turns out to be
  unworkable, report it as a protocol breach with the evidence, exactly as
  `docs/evaluations/writing-code-walkthroughs/` reports its own shortfall. Do not
  substitute a better rule; a rule chosen after seeing the data is not a
  preregistered one.
- Nothing may be edited in the book during this session. Finish every edit, then
  measure — and the edits finished at `proof-k44`.
- The preregistration `pilot-preregistration-k24` committed is binding and not
  reopenable here. Read its attribution rule before you start and satisfy it as
  you go; reconstructing afterwards what a stage changed is what the one-commit
  boundary exists to make unnecessary.
- The baseline this stage is measured against is the previous stage's commit, and
  the record of what this stage did is the diff between that commit and yours. So
  this stage lands in **exactly one commit**, carrying its edits and nothing else.
- The corpus is frozen: do not edit `crates/jj-workspace/`. A defect found here
  becomes its own leaf under the root brief's cross-book rule.

## Done when

- A report is committed under `docs/evaluations/`, stating per stage what it
  changed, what the judged outcome says about it, and which of keep / merge /
  drop the decision rule selects.
- A stage whose verdict the evidence leaves genuinely undetermined says so **in
  those words**. An undetermined stage that reads as kept is how six stages get
  shipped on the strength of nothing.
- The report states whether the six stages beat the stated alternative of two, and
  a result saying they did not is recorded as a successful outcome.
- Any re-run is confirmed item by item, not by matching totals.
- Final validation over `docs/walkthroughs/jj-workspace/` is green — this stage
  leaves the book provable, not merely improved.
- The stage's record exists in the form the preregistration's attribution rule
  requires.
- `bash scripts/check.sh` passes.

## Notes

**This is the evidence `pipeline-kinds-k27` extracts from, and there is no human
between the two.** Everything that leaf is allowed to conclude has to be legible
here, including the conclusions this report refuses to reach.

**One measurement, one writer.** If the reading is produced by a script, finish
the script before running it, and do not infer from a launcher's return that the
work behind it finished.