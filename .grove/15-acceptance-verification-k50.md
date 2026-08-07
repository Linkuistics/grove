# acceptance-verification-k50

**Kind:** impl

## Goal

Prove the integrated config-driven Grove through its public process, tree, VCS,
documentation, and release seams, closing only small defects found by the
acceptance sweep.

## Context

- Depends on `durable-docs-reconciliation-k49` and every preceding reviewed
  implementation slice.
- Binding matrix: `docs/specs/config-driven-sessions.md` section "Test seams"
  and the root brief's complete Done-when list.
- Verification commands include `cargo fmt --check`, `cargo test --locked`, and
  `scripts/release-doctor.sh`; use isolated homes and Git/native-jj/colocated-jj
  worktrees for black-box lifecycle cases.
- This leaf verifies and fixes only focused omissions. If a subsystem contract
  is materially incomplete, decompose this leaf and do the first named child
  rather than absorbing redesign.

## Done when

- The spec's process-level matrix is mapped to executable tests or an explicit
  already-covered case: KDL diagnostics/argv, two config reads, sibling-tool
  skew, metadata-only flags, root/migration/live/finish transitions, one pick
  and mandate, direct child lifecycle, leases/epochs/races/timeouts, Git/jj
  scoped commits, and Herdr opt-in behavior.
- The `grove-llm` matrix covers current filename parsing, all nineteen kinds,
  malformed/terminal trees, finish rules, pair generation, stable resolution,
  promotion after insertion, and migration-witness refusal; viewer fixtures
  cover the same filename grammar.
- Legacy-surface sweeps prove removed runtime and documentation policy with
  positive and cross-tree controls, while legitimate loop-control and durable
  historical discussion remain classified.
- `cargo fmt --check`, `cargo test --locked`, and `scripts/release-doctor.sh`
  succeed from the final tree, and `jj st` shows only the focused acceptance
  change before retirement.
- The root brief's Done-when conditions can be checked directly against landed
  code, tests, content, docs, ADRs, and specs with no unnamed residue.

## Notes

Do not publish or integrate the branch; release and worktree teardown are
outside this grove.
