# evaluation-recovery-k74

**Integrates:** evaluation-recovery-k73

## Goal

Apply the `evaluation-recovery-k73` findings to the `recovery-campaign-k54`
decomposition so the campaign that runs is smaller, harder to bias, and
fail-closed — before `measurement-design-k55` executes.

## Context

- Findings, verbatim and severity-ordered: the `## Findings` section of
  `evaluation-recovery-k73`.
- Producer plan being corrected: `evaluation-recovery-k53` and the
  `recovery-campaign-k54` subtree it created.
- Parent acceptance contract: `walkthrough-skill-delivery-k19` and the root
  brief.
- Historical hazards the findings are grounded in:
  `docs/evaluations/writing-code-walkthroughs/README.md` and the byte-frozen
  `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md` (its
  "Invalid runs and sampling" and Case A/B/C criterion tables).

## Done when

- **F1** — the three generation leaves precede all three adjudication leaves in
  `same-case-evidence-k60`, and its brief no longer defends interleaving as
  harmless resource ordering.
- **F2** — the replacement gate is a deterministic function of the runner's
  recorded exposure phase, owned and stub-tested by `campaign-runner-k56` and
  `campaign-auditor-k57`; every adjudication leaf re-verifies replacement
  legality over the preserved attempt history before scoring, with a frozen
  consequence when an illegal replacement is found.
- **F3** — the pre-exposure replacement ceiling is replaced by a resource budget
  with a predeclared non-selective resumption rule; hard ceilings remain only
  where an outcome is retained rather than replaced.
- **F4** — settled explicitly one way: either the transfer probe is named as the
  conjunct of a stated parent clause it discharges, or `transfer-freeze-k58` and
  the `transfer-evidence-k68` subtree are pruned from this cycle and the
  generality question is externalised as later tree work. If pruned, the
  same-case source/fragment fixture is stated to be external and non-Rust.
- **F5** — same-case criterion authoring carries `transfer-freeze-k58`'s role
  separation, restricted inputs, and preserved input/output chronology; the
  skill-to-criterion coverage map is a separate later step that cannot add or
  weaken a row.
- **F6** — the regression rule covers every requirement-derived row outside the
  target set, and mixed-control rows have a stated judging rule.
- **F7** — `measurement-design-k55` publishes a retained / reworded / dropped map
  against the frozen rubric, with a reason per drop, as non-weakening evidence
  only.
- **F8** — the freeze states fail-closed direction: missing, unavailable,
  protocol-failed and unblindable data never contribute to attainment and leave
  the parent conjunct open.
- **F9** — one named leaf owns building and testing the scorer and blind-resolver
  invocation harness, and `campaign-freeze-k59`'s stubbed end-to-end tests cover
  it.
- **F10** — a forced per-bundle arm guess is predeclared and reported as a
  limitation of the primary result.
- **F11** — dual blind scoring applies to every scored bundle, not only complete
  cases.
- **F12** — a pair's two arms execute back-to-back, with per-arm timestamps
  recorded against the pair id.
- **F13** — `campaign-auditor-k57` carries the explicit pre-freeze
  no-live-execution guard its siblings carry, and states that its
  prohibited-access fixtures are synthesized records.
- **F14** — settled either way, with the reason recorded: a scheduled
  `review-impl` beside `campaign-freeze-k59`, or an explicit decision to rely on
  that leaf's in-session allowance.
- Every finding is classified as applied, applied differently, or declined with a
  reason. No historical evaluation record, no frozen rubric byte, and no
  committed campaign result is edited.

## Notes

Fixes belong in the `.grove/` task files and briefs of the campaign subtree, not
in `docs/evaluations/`; this cycle has produced no campaign evidence yet.

F4 and F14 are scope calls rather than defects — decide them against the parent
brief and record the decision, do not treat "recommended" as settled. F2 and F3
interact: making the exposure-phase proof deterministic is what makes removing
the pre-exposure ceiling safe, so apply F2 before judging F3.

If applying a finding turns out to need substantial redesign of the campaign
shape rather than an edit to these task files, externalise it as a new producer
leaf beside this one rather than absorbing it.
