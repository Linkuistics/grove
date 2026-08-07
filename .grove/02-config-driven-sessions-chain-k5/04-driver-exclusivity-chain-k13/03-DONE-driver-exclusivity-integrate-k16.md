# driver-exclusivity-integrate-k16

**Kind:** integrate-review-design
**Integrates:** driver-exclusivity-review-k15

## Goal

Apply the verified findings from `driver-exclusivity-review-k15` while preserving the reviewed artifact's contract.

## Context

- Verify every finding against the single-driver and tree-only-state contracts;
  preserve confirmed launch policy and externalize unrelated redesign.

## Done when

- Real findings are integrated, accepted trade-offs are explicit, and the
  resulting ownership contract is a stable input to implementation planning.

## Notes

## Integration

- `driver-exclusivity-review-k15` E1–E13 were verified against the reviewed
  artifact and current implementation seams. E1, E2, E5–E13 are integrated
  directly.
- E3/E4 identified a real duplicate-owner failure in temp-directory controls.
  The contract now uses the exact Git-worktree or jj-workspace administration
  area, so `TMPDIR`, private temp namespaces, and routine temp sweeps do not
  select or remove ownership. Locked-path identity is still revalidated;
  deliberate mutation of VCS administration files is recorded as outside the
  cooperative consistency boundary.
- The former stable signal-path invariant is explicitly reversed: process and
  launch identities use independent 128-bit OS randomness, abandoned paths are
  cleaned only after exclusive handoff, and implementation planning must retire
  the old stability test.
- Epoch acquisition now has explicit modes, separate guard scopes, one waiting
  diagnostic, and an internal 30-second bound. A post-reap orphan stops the loop
  `blocked` without consuming the signal, preserving crash handoff without an
  indefinite silent stall.
- The leaf's one permitted narrow fresh-context review found six further issues,
  all integrated: ambient Git selectors are ignored; `.jj/grove/` and the
  per-worktree Git directory are the normative workspace mapping; every epoch
  open/lock race gets identity validation; shared-guard acquisition is the
  admission boundary; and an internal lock/filesystem backend with barriers and
  event tracing makes protocol order deterministic in tests.
- Two findings were real trade-offs rather than literal guarantees. Deliberate
  unlink/recreate of VCS-administration controls remains outside cooperative
  ownership, and cross-process random identity reuse has an accepted
  one-in-`2^128` bound. Guaranteeing either absolutely would require the
  substantial durable coordination this brief excludes.
