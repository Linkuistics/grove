# post-teardown-restart-contract-review-k103

**Kind:** review-impl
**Reviews:** post-teardown-restart-contract-k102
**Producer launch:** {"producer":"post-teardown-restart-contract-k102","session":"post-teardown-restart-contract-k102","generation":"k102","harness":"claude","model":"opus"}

## Goal

Adversarially review `post-teardown-restart-contract-k102` and record concrete
findings for its integration step.

## Context

- Review against `docs/specs/config-driven-sessions.md` sections "Fresh tree",
  "Existing live tree", and "Crash and retry semantics", plus ADR
  `one-live-driver-per-working-tree`.
- Attack accidental finish inference, hidden durable state, VCS-specific
  classification drift, loss of the child's real no-signal outcome, stale epoch
  access after `plan-k1` reuse, false-positive retry proof from older teardown
  history, jj successor-working-copy mistakes, and methodology wording that
  treats task-root absence itself as a receipt.
- Inspect the producer's committed diff and recorded evidence. Produce findings
  only; implementation belongs to the integration leaf.

## Done when

- Findings cite exact source, test, methodology, or design locations and name
  the threatened contract, or record an explicit no-finding result.
- Git and jj restart shapes, including unrelated successor work in jj, are
  considered without inventing a history-based lifecycle discriminator.
- No production, test, or documentation artifact outside this task file is
  changed.

## Findings

### F1 — the producer's full-suite verification gate is not satisfied

`post-teardown-restart-contract-k102` requires both `cargo fmt --check` and
`cargo test --locked` to pass, but
`cleanup-barrier-readiness-flake-k165` records that the producer's full-suite
run failed
`process_cleanup_does_not_unlink_a_substituted_non_directory_entry`; only the
isolated retry passed. No later successful full-suite run is recorded. The
flake is correctly externalized as its own leaf, so its fix does not belong in
this review or integration chain, but the producer cannot be treated as having
met its recorded green-suite gate until that leaf establishes a passing run.

No restart-contract correctness finding was found. In particular:

- `tests/finish_lifecycle.rs` tests
  `a_done_signal_abandoned_by_a_killed_driver_reinitializes_instead_of_finishing`
  and
  `a_shared_epoch_guard_blocks_the_post_finish_replacement_without_creating_a_tree`
  cover abandoned `done`, fresh-root classification, and the bounded pre-tree
  epoch handoff in plain Git.
- `assert_lost_jj_finish_result_retry_is_idempotent` starts from
  `seed_jj_terminal_grove`, whose `outside.txt` change remains in the jj
  successor, so native and colocated jj exercise exact-parent retry proof with
  unrelated successor work rather than a clean synthetic topology.
- `bare_driver_reaps_orphans_and_ignores_abandoned_signals_in_jj_workspaces`
  covers fresh restart and abandoned-signal cleanup in both jj shapes, while
  `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session` rejects
  stale reads, mutation, and completion after handle reuse.
- `src/repo/finish_commit.rs::verify_lost_jj_finish` examines only the current
  working-copy successor's sole parent and then requires the exact
  attempt-bound message and `.grove/`-only deletion, so older teardown history
  cannot become a lifecycle discriminator.

## Notes
