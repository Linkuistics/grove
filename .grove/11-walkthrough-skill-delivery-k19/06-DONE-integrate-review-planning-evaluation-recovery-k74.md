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

## Decisions (running log)

- **F1 — applied.** All three same-case generation leaves run before any
  adjudication leaf. The campaign and same-case briefs no longer describe
  serial case adjudication as harmless resource ordering.
- **F2 — applied.** Replacement legality is a deterministic runner/auditor
  function of the recorded exposure phase. Every adjudication replays that
  function over the complete attempt history; any illegal replacement marks
  the affected case protocol-failed and excludes it from attainment.
- **F3 — applied.** Proven pre-exposure failures consume a frozen wall-clock
  resource budget rather than an attempt ceiling. Resumption follows the
  frozen schedule's earliest incomplete assignment without regard to arm or
  observed outcome; the first post-exposure attempt is retained and ends that
  assignment.
- **F4 — applied differently.** The transfer probe stays in this cycle as a
  required, separately reported conjunct of the parent brief's requirement
  that the skill apply across codebases and languages. The same-case external
  OCaml fixture supplies one non-Rust surface; the independently selected
  transfer target tests the same generic-applicability clause without reusing
  that fixture or its criteria. It remains unable to rescue the primary
  same-case endpoint or alter skill wording.
- **F5 — applied.** Same-case criteria are authored from restricted inputs by
  a role separated from the later coverage mapper. Exact inputs, outputs,
  runtime identity, chronology, and digests are preserved; the post-freeze
  map cannot add, delete, or weaken a criterion row.
- **F6 — applied.** Every requirement-derived row outside the target set is a
  regression row. The freeze must state a paired judging rule for mixed-control
  rows rather than hiding a drop inside an aggregate count.
- **F7 — applied.** Measurement design publishes a retained / reworded /
  dropped map against the byte-frozen rubric, with a reason for every drop, as
  non-weakening audit evidence only.
- **F8 — applied.** Missing, unavailable, protocol-failed, and unblindable data
  never contribute to attainment; each leaves its parent conjunct open.
- **F9 — applied.** `campaign-auditor-k57` owns the scorer and blind-resolver
  invocation harness, and `campaign-freeze-k59` exercises it end to end with
  stubs.
- **F10 — applied.** Every scorer makes a forced arm guess for every bundle;
  guess accuracy is reported as a limitation, not a sample filter or a reason
  to reinterpret scores.
- **F11 — applied.** Every scored bundle receives two independent blind scores,
  including bundles from incomplete or irregular cases.
- **F12 — applied.** A pair's arms execute back-to-back in frozen order and
  carry per-arm start/end timestamps under the pair id.
- **F13 — applied.** Auditor tests use synthesized event records and launch no
  live evaluated model before the joint freeze.
- **F14 — declined as a trade-off.** `campaign-freeze-k59` keeps its one narrow
  in-session adversarial review over the complete freeze candidate and parent
  contract. A scheduled review leaf would add stronger temporal separation, but
  it would ask the same artifact-and-contract question as that fresh context,
  while forcing every finding through a later integration session before the
  manifest can be signed. The in-session path catches defects at the last
  reversible point; its explicit second-review boundary still externalizes any
  substantive follow-up.
- **Doubt D1 — valid, substantial redesign externalized.** The recovery design can
  reword or drop historical rows even though the parent brief requires the same
  scenarios and unchanged rubric after a pre-skill control. Resolving whether
  the recovery campaign can discharge that contract requires redesign rather
  than repair, so `acceptance-contract-reconciliation` is inserted before the
  campaign as a new planning producer that must commission its own review.
- **Doubt D2 — valid and applied.** The campaign now fixes the same-case
  source/fragment fixture as external OCaml evidence, so the retained F4 branch
  cannot pass without its stated non-Rust surface.
- **Doubt D3 — valid and applied.** Resource replenishment is automatic,
  globally bounded, and pair-atomic; it occurs only between pairs, and a
  mid-pair exhaustion becomes unavailable rather than separating the arms.
- **Doubt D4 — valid and applied.** Deterministic verification now owns a
  terminal pass, failed, or unavailable record, so a failed check reaches the
  report instead of stranding the leaf.
- **Doubt D5 — valid and applied.** F14's rationale now states the actual
  temporal-separation trade-off between a scheduled review chain and the one
  fresh in-session review, rather than implying the scheduled reviewer would
  inspect an artifact before it exists.
