# ravel-caller-prose-repair-k46


## Goal
Repair the remaining active caller prose and complete k29/k5 acceptance without
changing the public runtime or the live original-v22 ending contract.



## Context
Consumes ravel-caller-acceptance-k45 and the final acceptance section of
`docs/ravel-caller-surface-audit.md`. Its frozen gate passes all 14 checks and
all explicit installed tasks, with 17,062 unchanged subjects and four unchanged
generated probes. Runtime acceptance is green; current documentation is not.

`docs/adr/harness-selection-is-owned-by-policy.md` lines 108–116 still describe
`run --confine`, dispatch-owned scrubbing, a run ID and a run record. Lines
141–162 retain an installation subcommand, record-directory/store lookup costs,
executable-supplied provenance and the previously rejected independent repo/release
route. Those contradict this ADR's repaired opening and the settled extraction.
`docs/USAGE.md` lines 435–440 direct readers to run an absent `inspect:` line
until it reports a command; real Ravel has no inspection verb, and the example
immediately above already contains no such line. k45 saves exact passages and
source hashes in `target/k45/acceptance-gaps.json`.

These are active caller instructions/decisions, not the broader ownership prose
already assigned to k12. k6 still owns retained package/delivery removal; k12
still owns its existing broader guide/glossary/spec/design reconciliation.

## Done when
- Reconcile the policy ADR in place against its entire current decision, costs
  and considered options. Describe public `start-session --confine`, caller-owned
  receipts, manual readable-asset copying and independent delivery accurately;
  preserve meaningful rejected alternatives without preserving false current
  mechanisms. Read the decision-records skill before editing the ADR.
- Replace the usage refusal-recovery prescription with actionable policy repair
  and rerunning Grove. No inspection/dry-run capability is implied. Check nearby
  recovery prose, summaries and linked caller ADRs/specs for the same omissions.
- Refresh the complete surface inventory and audit exceptions/source pins using
  whole tracked-source enumeration and observed positive/cross-tree/mutation
  controls. The k44 classification alone does not prove semantic reconciliation.
  Update any affected source-book fragments/manifests/indexes and validate them.
- Freeze the final inputs, run `task check` with explicit installed Ravel 0.1.0
  and pinned Bun 1.4.2, plus `ravel:fixtures`, `ravel:commands`, `ravel:review`
  and authority acceptance. Preserve complete logs and compare individual
  hashes/outcomes; do not credit k45's pre-repair measurement as the repaired run.
- Recheck every k29/k5 Done when item against delivery and the recorded matrix.
  Promote concise final evidence, retire and commit, and close k29/k5 only if
  their complete contract holds. Add precisely named missing work otherwise.
  Use this session's original ending command as the final action. No release,
  integration, installation, teardown or new driver belongs to this leaf.

## Notes
