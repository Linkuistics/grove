# modular-types-k15

## Goal
Remove unreachable literal resolver and inspection representation and update its
public consumers and exact-source projections as one green increment.

## Done when
- CapturedDocument has only modular declarations; Route has no literal variant,
  resolver folds no literal patches or resets, and inspection uses instantiated words.
- AssignmentValue carries Set/Unset only; CommandView binding/command are required.
  Grove renderers, retained tests and the public type spec agree.
- Affected source-exact fragments and their directly related explanations agree
  with production code; scripts/check.sh passes.

## Decisions (running log)
- Split representation removal from the final repository-wide forms/prose audit.
  This child owns code and its immediate documentation consumers; modular-audit-k16
  owns the remaining current-prose sweep and subsystem review decision.
- Keep Missing/Binding routes: a targetless personal parameter patch must still
  fail before a local target can repair it. Remove only unreachable literal paths.
- Retain existing load/expansion and inspection tests as the regression seam;
  this deletion adds no new behavior and needs no new CLI acceptance suite.
- Removing the literal branch also removes Template's otherwise unused text/span
  copies. Inspection keeps the authoritative declaration spans; runtime expansion
  retains compiled words and the definition's path.
- Apply doubt-driven-development by preferring executable evidence for this
  deletion: existing named-command, parameter, profile and inspection tests exercise
  target replacement, targetless personal refusal, unset inheritance, history IDs
  and expansion after source removal. A fresh read of the same fold is weaker here.
  The final subsystem review decision remains explicitly chartered to k16.

## Verification
- `cargo test -p keyed-launch --tests` passed after the code cleanup.
- `bash scripts/check.sh` passed all eight principal checks: formatting,
  shellcheck, workspace clippy, plugin install, conformance, conformance regression
  suite, locked workspace tests and all six final source-exact books.
- The affected books reconstruct 3,715 keyed-launch lines and 959 overview lines,
  with no deferred ranges. Fragment ranges, ownership tables and prose roll-ups
  were updated together.
- SHA-256 digests of all 1,799 tracked files (production source, manifests,
  fixtures, scripts, book prose and task notes) matched before and after the
  complete check. No subject was edited during measurement.
- Parent modular-representation-k14 remains open: modular-audit-k16 owns the
  wider forms audit and subsystem review decision; no ancestor closes here.
