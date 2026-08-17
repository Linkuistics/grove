# kind-references-k23

**Integrates:** `kind-references-k22`

## Goal

Triage and integrate the actionable findings from the adversarial review of
`kind-references-k5`. Preserve the ten references' incremental shape while
restoring rules that the producer removed before their canonical owners carried
them, and make the interview-threshold evaluation reject the wrong rule.

## Context

### Findings

1. **Defect — `glossary-is-the-forcing-function` is no longer delivered at its
   own trigger.** `content/SKILL.md:116` sends a session resolving a term to
   `CONTEXT-FORMAT.md`, but `content/CONTEXT-FORMAT.md:66-72` says only when to
   create/select a glossary and never says to write the term inline rather than
   batch it. The surviving complete statements are in
   `content/grilling.md:77-81` and `content/references/grove.md:56-65`; those are
   reached under different situations, and `grilling.md` is unavailable below
   the grilling threshold. Removing the two requirements/execute mirrors therefore
   turned a pre-existing bad edge into an actually homeless rule for a normal
   requirements path. Land the complete inline/never-batched rule in
   `CONTEXT-FORMAT.md` now; the later corpus-split leaf may still remove the old
   duplicates.

2. **Defect — the `design.md` deletion made two more owner edges terminate at
   files that do not state their rules.** `content/SKILL.md:112-115` points ADR
   and spec decisions to their format files, but `content/ADR-FORMAT.md:1-5`
   delegates the when-to-write test to an optional plugin and
   `content/SPEC-FORMAT.md:3-7` describes the spec/flow without saying that a spec
   is written only at a genuine agreement point. The complete AND-form ADR test
   survives only at `content/grilling.md:83-88`, and the agreement-point rule
   only at `content/references/grove.md:67-71`; neither is on every bound kind's
   path for the triggering situation. Removing the old OR-form was correct, but
   the same commit needed to land the settled all-three test in `ADR-FORMAT.md`
   and the agreement-point rule in `SPEC-FORMAT.md`, rather than leave the
   canonical owners empty until later leaves.

3. **Defect — untouched `finish.md` is not incremental.** Its opening says a
   finish session "never asks `pick`" at `content/references/finish.md:1-3`,
   restating the all-kind `no-second-pick` rule already owned by
   `content/SKILL.md:53-57`. Its teardown step at
   `content/references/finish.md:39-50` also restates the stop/relaunch endings
   whose declared sole source is `content/SIGNAL-FINISH.md:5-9`. Remove those
   restatements or reduce them to pointers while preserving the genuinely
   finish-bound recovery, promotion, confirmation and decline rules. Do not move
   `finish-is-the-drivers-to-discover` early; `loop-step-references-k11` still
   owns that atomic move.

4. **Defect — the new threshold matcher accepts the off-by-one and opposite
   policies it is meant to reject.** The first claim explicitly accepts
   `"only above three"` at `tests/lifecycle_invariants.rs:795-805`, which means
   four or more rather than the canonical three or more. The reconciliation
   claim at `tests/lifecycle_invariants.rs:828-837` requires only the unordered
   words `unconditional` and `interview`, so "requirements is an unconditional
   interview kind" satisfies it while reversing the rule. Tighten both claims
   with direction-bearing wording/negative controls, and change
   `content/references/requirements.md:30` from "Above the threshold" to "When
   the threshold is met" so the prose does not reproduce the same exactly-three
   ambiguity.

### Reviewed and accepted

- **Trade-off accepted:** `content/references/prototype.md:1-5` tells a producer
  what to build, while `content/references/review.md:12-13` tells a reviewer what
  not to score. Their shared "polish is a defect" wording is two differently
  bound sides of the artifact, not one procedure-register restatement.
- **Noise:** the seven early additions at
  `content/references/research.md:8-34` and
  `content/references/integrate-review.md:14-17` preserve every clause of their
  current sources at `content/driving.md:70-134`,
  `content/TASK-FORMAT.md:69-81` and `content/references/execute.md:59-61`; the
  planned duplicates can be removed by `k11`/`k6` without weakening the
  surviving statements.
- **Noise:** `Binds::OnlyRequirements` at
  `tests/lifecycle_invariants.rs:234-240` is the correct scope for a
  `static({requirements})` row, and each near-miss at
  `tests/lifecycle_invariants.rs:807-837` is on topic. The defect is what the
  positive matchers also admit, not why those fixtures fail.

## Done when

- All four defects are triaged against the current corpus and fixed or accepted
  visibly with the reason recorded.
- Canonical owner files state the three prematurely removed rules before any
  remaining mirrors are deleted.
- The threshold evaluation rejects both explicit counterexamples above and the
  requirements prose is unambiguous at exactly three interdependent questions.
- The ten kind references remain complete for their routed kinds and contain no
  rule assigned to an already-populated upstream owner.
- Relevant post-fix verification is run by this integration session.

## Notes

This review was inspection-only. It ran no test, build, lint or format command
and edited no production or test file.
