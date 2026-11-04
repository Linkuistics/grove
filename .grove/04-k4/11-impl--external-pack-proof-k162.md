# external-pack-proof-k162

## Goal

Close the research-pack proof. Record the proven contract and the evidence
that repository-moves-k5 needs. Audit the Done when of
external-research-pack-k4 one criterion at a time.

## Context

- `docs/specs/external-pack-contract.md` and its section for proven facts.
- The records of grove-research-removal-k158, research-directory-trials-k160
  and research-native-namespace-k161.
- The "Completion audit" of `docs/common-loading-acceptance-record.md`, as a
  model for the audit.

## Done when

- The contract's section "Proven facts" fills each row of its table. It
  states the proven layout, resource references, checkout boundary and
  conformance boundary. It names the source
  directory and roots inside any checkout of the pack. It names the reusable
  pack tooling: the conformance invocation, the Taskfile tasks, the plugin
  manifest and the trial-runner fields.
- The section names the evidence that repository-moves-k5 needs before it
  starts, with commits. The node brief lists that evidence.
- ResearchSkills' own checks pass from a clean checkout of the published pack
  commit that records the removal commit. Grove's `bash scripts/check.sh` passes on the integrated line with
  the Ravel 2.0 build. Record both build identities.
- Each criterion of external-research-pack-k4 has a verdict and evidence. A
  gap has repair work, a covering case or a recorded human decision.
- Any missing work is a new leaf in this node before the node closes.

## Notes

Where a criterion fails and you cannot name the work, stop and ask the human.
