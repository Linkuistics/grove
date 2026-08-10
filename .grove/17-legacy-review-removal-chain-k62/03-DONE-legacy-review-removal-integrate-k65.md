# legacy-review-removal-integrate-k65

**Kind:** integrate-review-impl
**Integrates:** legacy-review-removal-review-k64

## Goal

Apply the verified findings from `legacy-review-removal-review-k64` while preserving the reviewed artifact's contract.

## Context

## Done when

## Notes

- Integrated the single finding F1 from `legacy-review-removal-review-k64`. The
  marker sweep's advertised "Git versus jj" control ran one code path twice: the
  Git fixture never staged `.grove/`, so `capture_git_index_entry` returned
  `None` and `rename_entry` selected `plain_rename` — the same move the jj
  fixture makes. `fixture(Tree::Git)` now stages the tree and asserts the
  producer is tracked, so the Git half exercises `git mv` plus the staged-path
  rewrite while the jj half keeps its `.git`-absence assertion.
- Corrected two prose claims the finding named: the jj test's comment reversed
  the production order (generated steps are written into the transaction
  *before* the producer moves, `src/tree_promotion.rs:162-177`), and the module
  header asserted a per-VCS difference that is really a per-trackedness one.
- Evidence that the fix changes which path runs, not just which comment is
  written: promoting the same fixture with and without the `git add` leaves the
  index naming the producer at its final in-node path
  (`.grove/01-sync-chain-k2/01-design-sync-k1.md`) versus leaving the index
  empty. The new assertion was also shown failing — removing the `git add` fails
  all four Git-fixture tests with its own message.
- No in-session reviewer was spent: the correction is a fixture precondition
  whose effect is demonstrated by an executable control, and no contract of the
  reviewed producer changed.
- Verification (2026-08-11): `cargo fmt --check` clean; `cargo test --locked`
  passed with zero failures across every binary (552 library tests, 59 and 19
  loop-driver/lifecycle integration tests among them); `cargo clippy --locked
  --test task_marker_surface` reported nothing. Clippy warnings elsewhere in the
  workspace are pre-existing and untouched.
