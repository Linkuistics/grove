# measurement-design-k55

## Goal

Write the complete exact-historical-instrument recovery contract independently
of the deployed skill's wording and new campaign outcomes.

## Context

- Parent contract: `walkthrough-skill-delivery-k19` and `recovery-campaign-k54`.
- Authority and claim boundary: `acceptance-contract-reconciliation-k75` and
  the settled `acceptance-replication-authority` decision.
- Hazards only, not a replacement rubric:
  `docs/evaluations/writing-code-walkthroughs/README.md`.
- Preserve `baseline/rubric.md` byte-for-byte.

## Done when

- The acceptance manifest records the frozen rubric digest and references or
  byte-verifies every Case A/B/C prompt, fixture, atomic criterion, sample rule,
  invalid-run rule, scoring rule, target set `R`, regression set `G`, `2/5`
  materiality rule, `10/15` endpoint, and regression guard. It contains no
  reworded or dropped row and no new absolute or per-family gate.
- The accepted pre-skill operands are the historical records only: Case A's
  five valid baseline repetitions and the preserved Case B/C shortfalls. A new
  no-skill arm is named a contemporaneous comparator and serves only the role
  the historical rubric assigns it.
- The design preserves the original failed enabled campaign as a distinct
  historical result and implements exactly the replication authority granted by
  `acceptance-replication-authority`; it does not infer that another five-run
  sample may supersede the failed arm.
- A conformance table enumerates every historical prompt, fixture, row, set,
  threshold, and rule and proves there is no retained/reworded/dropped choice.
  Any proposed new absolute gate, per-family gate, mixed-control rule, expanded
  regression row, or scoring control is externalized to
  `supplemental-evaluation` rather than drafted here.
- The source/fragment prompt retains the historical Case B fixture,
  `targets/ocaml/check_floor.ml`, at its frozen external-source digest. The
  measurement design states explicitly that this is an external, non-Rust
  same-case surface.
- Five enabled and five contemporaneous no-skill repetitions per case follow the
  historical ABBA/BAAB interleaving and runtime-record contract exactly.
- Replacement, truncation, access, scoring, and incomplete-sample outcomes
  remain exactly the historical rubric's rules, including the two-replacement
  ceiling. Exposure-phase gates, automatic resource windows, pair-atomic
  resumption, fail-closed missing-data semantics, dual scoring beyond the
  historical requirement, and arm-guess probes are excluded and routed to
  `supplemental-evaluation`.
- The historical transfer prompt, fixture, criteria, threshold, and regression
  bytes are the only transfer instrument. This leaf selects no new target and
  derives no new transfer row.
- The design separates deterministic access/digest/manifests from judged answer
  behavior and limits every conclusion to this bounded sample; it does not
  claim reader comprehension, population generality, or causal use of wording.
- No evaluated treatment or control context is launched in this leaf.

## Notes

The historical gaps identify protocol hazards but cannot authorize instrument
changes. F1–F14 are preserved as inputs to `supplemental-evaluation`; only a
finding compatible with byte-identical historical semantics may shape this
campaign's apparatus. If the requirements decision does not authorize
replication, this leaf records the non-execution disposition and externalizes
the next authorized work instead of preparing live runs.
