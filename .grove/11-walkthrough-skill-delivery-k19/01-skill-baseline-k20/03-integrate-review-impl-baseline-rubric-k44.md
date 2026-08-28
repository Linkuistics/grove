# baseline-rubric-k44

**Integrates:** baseline-rubric-k43

## Goal

Verify each of the sixteen review findings against the frozen rubric, fix or
visibly accept every one, and leave a rubric that can be frozen again before the
first evaluated run.

## Context

- The findings: the `## Findings` section of
  `.grove/11-walkthrough-skill-delivery-k19/01-skill-baseline-k20/02-DONE-review-impl-baseline-rubric-k43.md`.
  Each names an exact line in
  `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md` at producer
  commit `0fc320b4f9ac2cfce99aefb791e381594fd82dc2`, the contradicted brief
  clause or sibling sentence, and a recommended fix.
- Primary artifact: `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.
- Binding contracts: this node's `BRIEF.md` (`Done when`), the parent brief
  `.grove/11-walkthrough-skill-delivery-k19/BRIEF.md`, and
  `docs/research/walkthrough-method.md:425-448`.
- The mechanical layer was checked clean by the review and can be assumed: the
  committed fixture matches the frozen digest, the producer commit contains no
  run output, and every flag in the frozen command shape exists in
  `codex-cli 0.150.1`. Any fix that changes the fixture or the command shape
  re-opens that check.

## Done when

- Every finding is independently verified against the line, brief clause or
  harness behaviour it cites, then fixed or argued down with the reason written
  into the running log. Verify before fixing: a finding accepted without
  checking is the same defect the review exists to catch.
- Findings 1, 2 and 3 are resolved, not qualified. The control environment is
  declared such that the arm difference and the absolute
  reliable-without-guidance classification are both supportable; the `:51-52`
  and `:75-79` contradiction about non-discovery is removed in one direction;
  and the contemporaneous control's skill-installation state and manifest
  handling are stated.
- Finding 1's flag question is settled empirically before it is written down:
  confirm whether `--ignore-user-config` and `--ignore-rules` suppress
  `$CODEX_HOME/skills`, `$CODEX_HOME/AGENTS.md` and `$CODEX_HOME/hooks.json`,
  and record the method and result. A probe run for this purpose is a harness
  check, not an evaluated repetition; it uses a throwaway prompt unrelated to
  any frozen case, and its transcript is recorded as harness evidence so it can
  never be mistaken for a sample.
- Findings 4, 5, 6, 7 and 8 leave the rubric with a named adjudicator and
  blinding rule, a scale that scores absence criteria, a frozen campaign-level
  success condition, an unambiguous regression guard, and Case A rows that each
  state their own evidence.
- Findings 10, 11, 12 and 13 leave invalid-attempt counts, truncation status,
  the transfer probe's success rule and selection independence, and the run
  directory's location declared.
- Findings 9, 15 and 16 are each either fixed or accepted visibly in the rubric
  as a stated limitation with its reason; silence on them is not an outcome.
- Finding 14 is applied last, so the restated freeze clause names this session's
  own commit.
- No evaluated scenario is run, no skill is authored or scaffolded, and the
  four run leaves that follow are left otherwise untouched.

## Notes

Substantial redesign is not this session's work. If a finding turns out to
require rebuilding the campaign rather than repairing the rubric — the most
likely candidate is finding 6, if freezing an aggregate endpoint reopens what
the enabled evaluation is measuring — cut a new producer review chain beside
this leaf rather than landing an unreviewed redesign here.

The rubric is re-frozen by this leaf's commit. Every later leaf in this node
reads it unchanged, so a fix that lands after
`baseline-scope-elicitation-k42` has run costs the campaign its first case.
