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

## Decisions (running log)

1. **The probe is independent, not the fallback.** *Part 2*'s producer was
   materialised as a separate agent context before this session opened any
   post-draft state, given only the draft-state book, the four corpus files, the
   taxonomy with its *Charter owner* column removed, and the frozen prompt
   verbatim. The fallback clause — and its withholding of every `Keep` — does not
   apply. The exact inputs and what was withheld are recorded in `probe.md`.
2. **`CONTEXT.md`'s digest has moved from the value the preregistration froze**
   (`c7dde4d6…` → `c0d46984…`), at the draft's first commit `orientation-k55`,
   which the *Validity* allowlist expressly permits the draft to make. It is
   reported as a protocol tension already recorded by `0-draft.md` rather than as
   a new breach, and it touches no verdict: all five scored stages read the same
   post-draft value.
3. **De-duplication is re-applied by this session and lands differently on two
   stages.** Art's six `A1` claims stay six: a single subject-free sentence
   ("draw the relation as a table") states the class's remedy shape rather than
   the correction, and a reading on which it counts as one family would make `A1`
   incapable of ever reaching the threshold of three, which the instrument plainly
   does not intend. The copy edit's own collapse of five `C2` figures of speech
   into one `book-wide` claim is honoured rather than reversed, in the direction
   that costs the stage rather than the one that buys machinery; the recount is
   reported and does not change its row.
4. **No regression was found in any stage.** Each of the five diffs was read in
   full against the taxonomy for defects *opened* rather than closed; `r(S) = 0`
   for all five, and the three candidates considered and rejected are named in the
   report.
5. **The result is `N = 4`** — draft, copy edit, art, proof — with the
   developmental and technical edits `Merge`d into the draft at `net = 2` each,
   and no stage `Undetermined` and none `Drop`. Neither the six-stage design nor
   the two-stage alternative is upheld, which is *Part 4*'s middle outcome and a
   legitimate result.
6. **One formatting accommodation in `probe.md`, and no textual change.** Entry 2
   of the probe's list quotes a Markdown link out of the book; carried unfenced
   into `docs/evaluations/`, that link resolves against the wrong directory and
   `every_repository_markdown_reference_resolves` goes red. The citation is
   carried in a fenced block instead — the producer's bytes, this session's block
   delimiters — and `probe.md` says so where the list begins.
7. **No ADR is written for this result.** The durable record is the report itself,
   which lives under `docs/evaluations/` and outlives `.grove/`; the decision an
   ADR would record — which stages become `grove-<kind>` skills — is
   `pipeline-kinds-k27`'s to make and to record, and this leaf produces the
   evidence for it rather than the decision.
