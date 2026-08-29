# transfer-freeze-k58

## Goal

Prove the identity and executability of the already frozen historical transfer
instrument without selecting or authoring a new probe.

## Context

- Measurement draft: `measurement-design-k55`.
- Generic runner/auditor interfaces: `campaign-runner-k56` and
  `campaign-auditor-k57`.

## Done when

- The acceptance-transfer manifest references or byte-verifies the historical
  `enabled/transfer-probe/` prompt, fzf fixture/revision, criteria, thresholds,
  regression rule, schedule, and role chronology. It selects no new target and
  authors no new acceptance row.
- Each role's exact inputs, raw outputs, runtime identity, chronology, and
  digests are preserved, and the information flow is checkable rather than
  asserted.
- The target is outside this repository, absent from the historical campaign,
  and worded without `ordinal-fs-tree` domain assumptions.
- Runner/auditor fixture tests demonstrate exact historical compatibility and
  refuse a different target, prompt, row, or threshold without a live treatment
  or control execution.

## Notes

The historical transfer replication has its own verdict, never enters the same-
case primary set, and cannot change skill bytes. `supplemental-evaluation` owns
any new target or generality probe.
