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

## Decisions (running log)

1. Finding 1 is real. In `codex-cli 0.150.1`, the two isolation flags are
   scoped to `config.toml` and exec-policy rules, not to all of `CODEX_HOME`.
   A writable-home probe populated bundled system skills despite both flags;
   exact-version source inspection shows global `AGENTS.md` and `hooks.json`
   use separate loaders. A second preflight established that a fresh home plus
   `skills.bundled.enabled=false` leaves skills, global instructions, hooks, and
   config absent. The rubric now requires sealed control/enabled homes and
   records injected context. The network sandbox prevented a final model
   response, so the transcript says so rather than presenting the probe as an
   evaluated or successful model-visible run.
2. Finding 2 is real. Skill non-discovery is now a valid enabled outcome, the
   user prompts are byte-identical, explicit invocation is absent, and discovery
   rate is reported instead of used as a replacement filter.
3. Finding 3 is real. Baseline and contemporaneous controls use fresh copies of
   one sealed control-home template; enabled runs use a second template differing
   only by the target skill. Pre-run manifests are per repetition, and there is
   no install/uninstall transition on a shared home.
4. Finding 4 is real. Scoring is assigned to a fresh blind adjudicating context
   independent of authoring and execution, with randomized arm-stripped bundles;
   one whole case receives a second independent blind score and reports every
   disagreement.
5. Finding 5 is real. Absence criteria now require an exhaustive named-surface
   scan, a nearest near-miss citation for success, and the first prohibited
   instance for failure.
6. Finding 6 is real but does not require campaign redesign. The primary endpoint
   is frozen as material improvement on at least two-thirds of baseline repeated
   judgment gaps. Freezing the missing aggregate decision repairs the comparison
   contract without changing its cases or causal question.
7. Finding 7 is real. Per-criterion improvement no longer contains a
   campaign-wide quantifier; the regression guard is a separate aggregate rule
   over baseline present-in-sample non-compliance criteria.
8. Finding 8 is real. Every ambiguous Case A row now names a later question as
   its evidence, and rows about frozen depth, form, prose, and navigation also
   require the closing contract to record the answer.
9. Finding 9 is a real limitation and is accepted visibly rather than weakening
   the prompts. `B01`, `C02`, and `C03` are compliance controls, reported
   separately and excluded from reliability, skill-rule, and improvement claims.
10. Finding 10 is real. Invalid attempts are counted by machine-checkable reason
    for every case and arm and carried into the final report.
11. Finding 11 is real. The command now has one turn and a fixed wall-clock
    bound, applies no client-side output truncation, records termination state,
    and reports service-truncated rows as incomplete rather than zero.
12. Finding 12 is real. Transfer selection is blind to the skill and campaign
    outcomes, criteria are frozen before their author receives source, and a
    fixed half-improve/no-two-regress rule applies with report-regardless counts.
13. Finding 13 is real. Every run directory must resolve under the system
    temporary directory, outside this repository and any ancestor containing it;
    its absolute path is part of the record.
14. Finding 15 has one fix and one visible limitation. The origin remote is now
    recorded as `https://github.com/Linkuistics/APIAnyware.git`; the real source
    fixture remains unchanged, and both reports must state that its opening
    verification comment primes `B13`-`B20`, precluding an absolute unguided-
    reliability claim on those rows.
15. Finding 16 is real and fixed rather than excluded. Case B now requests and
    atomically scores a worked execution's inputs, outputs, stages, invariants,
    observable results, and transition reasons. Case C now requests and scores
    claim-leading prose, stable vocabulary, explicit actors, and evidence-bound
    refusal/failure/defect distinctions.
16. Finding 14 is real. After all substantive rubric repairs, the freeze clause
    is restated to name `baseline-rubric-k38` as the initial predeclaration and
    this leaf's `baseline-rubric-k44` commit as the final pre-run freeze point;
    only scenario-running leaves are forbidden from editing it afterward.

The command-shape change re-opened the review's mechanical check. The installed
runner is still `codex-cli 0.150.1`; a throwaway launch accepted
`skills.bundled.enabled=false` and reached `thread.started`; GNU `timeout 9.11`
accepts the pinned wrapper flags; and the unchanged committed fixture still
hashes to `2624183a8836364b5fdbcbeae7bf62de20d88550e6e2358aad13812da4cb0f0e`.
The producer artifact had no intervening rubric drift between commit
`0fc320b4f9ac2cfce99aefb791e381594fd82dc2` and this integration.
