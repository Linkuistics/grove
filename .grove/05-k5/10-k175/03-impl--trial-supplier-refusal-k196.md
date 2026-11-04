# trial-supplier-refusal-k196

## Goal

The harness trial runner refuses a case before any launch when a required
path is absent. The supplier of that path does not hold it at the declared
revision.
This change must land before design-removal-k198. That removal is the first
that takes away paths that existing cases require from the prepared commit:
`decision-records` and `cli-tool-design` in `plugins/linkuistics`.

## Context

- `docs/specs/pack-membership.md`: "The trial cases that name moving skills",
  with its supplier table, the four `trial:controls` readings and the rejected
  options. Also its test seam "The trial runner".
- `docs/common-loading-acceptance-record.md`, section "The runner". It owns
  each runner field.
- `scripts/harness_trial.py`, its controls and `scripts/harness-trials/cases/`.
  At planning, cases named `decision-records` through a planted file
  (`explicit-path`) and through native copies with a `commit`
  (`problem-stale-native`, `explicit-native-match`, `explicit-native-error`).
  A native copy of `cli-tool-design` had a `commit` (`problem-stale-native-cli`).
  Enumerate the required paths of every case again before you rely on this
  list.

## Done when

- Before any launch, and before a `moveAway` or `moveRoot` fault, the runner
  checks each required path against its one supplier, as the supplier table
  states. A missing path refuses with a nonzero status. The diagnostic names
  the case, the path, the supplier and the revision. It also states that the
  case measures a build whose supplier held the path.
- A case can declare a skill that its task relies on without naming it. A case
  can also declare a skill absent. The refusal requires a declared skill and
  does not require an absent one. Cases that rely on implicit discovery
  declare their skills, and `missing-kind` declares `grove-impl` absent. No
  earlier reading of a case changes. The runner reads the declarations only
  for the refusal.
- The check runs without a launch and without a model. A later leaf can
  therefore read it against a named Grove commit, and the record of this leaf
  states the command.
- `trial:controls` shows the four readings of the specification. A case
  against a commit that holds its local paths passes. The same case against a
  commit without one of them refuses with the diagnostic. A case with an
  external source passes, although the prepared commit holds neither the
  external directory nor its kind skill. A case with a native copy from an
  older commit passes, although the prepared commit no longer holds the copied
  directory. Each refusal control is seen to fail first against the unchanged
  runner.
- Every existing case passes the check against the Grove commit that its
  record names. Record the cases checked and the commits.
- "The runner" in the acceptance record names the refusal and the new case
  fields.
- `bash scripts/check.sh` passes.

## Notes

Do not retarget or remove any case. The specification rejects both, because
the acceptance record cites each case by the build that it measured.

A case that expects `skill_source_missing` from Ravel must still reach Ravel.
The refusal must not pass such a case for the wrong reason. The `moveAway`
and `moveRoot` faults happen after the check, so the check sees the source
intact.

Codex and Claude Code runs are not needed. The refusal and its controls run
without a harness.
