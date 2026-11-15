# incremental-delivery-k2

## Goal

Update Grove's planning and decomposition guidance to deliver the smallest useful
working increment, finish that grove, and improve it in subsequent groves. Prevent
implementation, review and prerequisite work from indefinitely deferring that
delivery while preserving necessary preparation and material defect repair.

## Context

The human's requirements are settled in plan-k1 and the root brief. Read plan-k1's
Decisions, Implementation scope and Verification plan; its Evidence section
records the four requested live-tree examples. Do not repeat that investigation
or re-interview the human about the agreed delivery and review boundaries.

The shared procedure currently encourages freely externalizing work and permits
narrowing acceptance into successive remainders. Placement and retained ownership
alone did not prevent the observed loops. Planning already calls for working
increments; strengthen and align the rules that agents follow while doing the
work so incremental delivery takes precedence over seeking perfection.

Start at the rule owners in plugins/grove/skills/grove/references/decompose.md and
plugins/grove/skills/grove-planning/SKILL.md. Inspect related shared execution,
review, integration and retirement guidance where necessary. Keep one canonical
statement of each rule and update existing affected references and checks rather
than adding parallel procedures or a new test framework.

## Done when

- Planning defines a bounded useful working outcome and proportionate acceptance;
  later improvements do not silently enlarge the current grove.
- Splits and prerequisites explain the acceptance work they complete or directly
  unblock. Repeated unchanged handoffs trigger an acceptance attempt in the current
  task, necessary inline repairs, or an explicit blocker with the task live.
- Review and integration add work for defects blocking agreed acceptance or
  presenting material risk. Optional polish is recorded for later; a repeat review
  needs a concrete change or new evidence. Ordinary checks stay in-session.
- Necessary working slices, material risk checks, declared caps and independent
  review remain available. Existing agreed obligations cannot be silently dropped.
- The six bounded verification cases in plan-k1 have been assessed, existing
  checks appropriate to the edited surfaces have run, and results and limits are
  recorded. Do not launch a cross-project measurement campaign.
- The actual guidance changes are committed with this leaf's retirement. Decide
  whether that artifact warrants a fresh independent review; create only that
  next step if warranted, with no speculative repair/review tail.

## Notes

This is one focused guidance increment. No runtime gate, task-state or filename
change, loop detector or generic numeric cap is requested. The four evidence
groves remain read-only. Installation, integration and release belong to the
repository's later finish procedure, with its required human confirmation.

## Decisions (running log)

- 2026-11-16: Keep bounded outcome and acceptance planning in grove-planning;
  keep scope admission and the repeated-handoff response in decompose; put the
  actionable finding threshold in integrate-review and link callers to it.
  Update the spine's routing sentence and retirement's missing-work reference
  so neither bypasses the scope rule. No runtime or task grammar changes.
- 2026-11-16: Verification is the six agreed scenario decisions plus existing
  skill conformance, pack checks and composition guidance tests. A bounded
  before/after helper applies the current rules; it is ordinary contract
  verification, not the independent adversarial review. No live-tree rerun or
  wording-variant measurement campaign. Graph indexing failed with
  `CBM index worker could not start: active daemon coordination could not be verified safely`;
  this Markdown-only change uses direct source reads and makes no graph claims.

- 2026-11-16: The settled policy is durable in the edited methodology itself;
  no parallel ADR or runtime enforcement is needed. One fresh `review-impl`
  sibling is warranted because this shared guidance changes how all producers,
  reviewers and integrations admit work, and must preserve safety and existing
  acceptance. It will inspect the committed artifact; no integration or repeat
  review is pre-created. The scenario helper was ordinary verification, not
  that independent adversarial read.
- 2026-11-16: Conformance initially failed because two inventory entries pinned
  the superseded free-externalization wording. Updated the existing rule
  inventory for scope admission and bigger-than-brief splitting and registered
  bounded delivery, stalled evidence and review thresholds at their owners.
  No new test framework or runtime rule was introduced.

## Six-case assessment

A bounded helper applied the same six scenarios before and after the edits.
Before, the desired decisions needed judgment beyond the text around repeated
handoffs, prep chains, optional polish and stopping hardening. After, each had an
explicit governing rule. A final targeted reread confirmed that clarifying the
trigger as stalled evidence (rather than unchanged criteria) and replacing a
repeat-review restatement with its canonical reference changed none of the six
interpretations. These are reasoned decisions, not observed live-tree outcomes.

1. Runtime: the recorded 14 slices/reviews and four integrations do not justify
   another peripheral split with the same unattempted composed acceptance. Keep
   the task live, attempt acceptance and repair bounded defects inline; record
   an evidenced prerequisite if it actually prevents the attempt.
2. APIAnyware: a reader directly supplying a finite original execution remains
   valid. Another readability map without that unblock does not earn a leaf;
   stalled readback chains must return to acceptance. The original finite
   execution obligations are retained.
3. Koine: compare evidence for the next window/resize behavior across differently
   named prerequisites. Attempt it now, or record the concrete missing dependency
   and finite unblock; task names do not establish progress.
4. Editorial/repair: perform the bounded real fix and material safety repair,
   accept a genuine trade-off visibly, and keep optional wording for later.
   Adverse verification and agreed quality obligations still bind.
5. Useful first delivery: when its agreed bar and required work are satisfied,
   stop adding optional hardening. Separately, retain broader preexisting
   acceptance and live work unless scope is explicitly changed. A smaller green
   result alone never proves the whole charter complete.
6. Positive controls: independently verifiable slices, necessary preparation,
   expand → migrate → contract and warranted independent review remain valid.
   Declared steps and caps remain binding. Counts, depth and names do not decide
   progress, and no generic numeric cap was added.

## Verification and limits

- `mise exec -- task test:grove-llm RAVEL=/opt/homebrew/Cellar/ravel/2.0.0/bin/ravel CLI_ARGS='--test composition_guidance'`:
  16 passed, zero skipped, including the final documentation revision.
- The first repaired `mise exec -- task check:skills` completed successfully:
  98 conformance controls, all five recorded packs alone and in aggregate,
  22 pack-script controls and 13 plugin/native-delivery tests passed. A final
  run covers the final two prose edits; its result is recorded below.
- No full Rust suite, installed-guidance rollout or live-grove behavioral trial
  was run for this documentation change. The four evidence groves were not
  mutated or remeasured. This guidance requires agent judgment about actual
  blockers and materiality; it is not a mechanical guarantee against loops.
- `GROVE_PROVIDER_EVIDENCE` is absent in this session. Per the retirement
  contract, the new review carries no invented Creator reference. Any review
  policy requiring attribution must obtain the owner's declaration.

- 2026-11-16: Final `check:skills` rerun passed: 98 conformance controls,
  five recorded packs alone and aggregate, 22 pack-script controls and all
  13 plugin/native-delivery tests. All eight production-file SHA-256 digests
  matched the pre-verification values. The final composition run passed all
  16 tests. A direct whitespace scan was clean after respecting TSV empty-field
  delimiters; this jj version does not offer `diff --check`. Documentation
  verification is complete.
