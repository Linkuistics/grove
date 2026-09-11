# valid-levels-k18

**Reviews:** valid-levels-k16

## Goal

Adversarially review the completed distinguished-level enforcement boundary
before `node-files-k7` builds Grove's required node-file grammar on it.

## Context

Read the producer's commit and this parent's charter. The contracts are
`docs/ordinal-fs-tree/ARCHITECTURE.md`, the entry-name seam ADR, and formalism
finding 049. The supplied-name review and its integration at
`supplied-names-k17` establish the preceding API boundary.

The changed implementation includes full snapshot reads, final plan projection,
pre-effect checks, path-bearing domain and cardinality errors, the explicit
`LevelSample` conformance API, API consumers, regression fixtures, and the
ordinal-fs-tree/grove-loop books and validator inventories.

## Done when

- Inspect whether projected names and parent identities preserve every reachable
  level through initialization, append/batch/insert, promotion with its optional
  child, rewrites and sibling shifts. Look for successful plans leaving a level
  unreadable, skipped levels, incorrect projected diagnostic paths or an effect
  occurring before rejection. Distinguish legal plans from impossible internal
  effect sequences.
- Check complete-listing validation, full-tree validation before searches under
  both guards, domain-error precedence and independent competing-name rejection.
  Check the unchanged reported-error rollback and process-interruption limits.
- Examine whether expected conformance verdicts are independent, whether sampling
  claims match actual coverage, and whether missing samples remain untested.
  Check the different root/node names and node-parts fixtures separately from
  the conformance kit and the bounded model evidence.
- Review affected source fragments, explanatory prose, ledgers and validator
  corruption fixtures for semantic drift beyond byte reconstruction. Grove's
  current filename grammar must remain consistent at this library boundary.
- Record findings without fixing them. If integration is earned, place it in
  this node after the review and before the grammar consumer can run.

## Notes

No in-session reviewer was used by the producer. The installed binary, plugin
and live-tree grammar remain outside this review's mutation scope.
