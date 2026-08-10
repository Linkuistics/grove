# post-teardown-restart-contract-integrate-k104

**Kind:** integrate-review-impl
**Integrates:** post-teardown-restart-contract-review-k103

## Goal

Apply the verified findings from `post-teardown-restart-contract-review-k103`
while preserving the reviewed artifact's contract.

## Context

- Verify every finding against the post-teardown design before changing the
  producer artifact.
- Preserve `.grove/` task-root absence as fresh start, signal-only session
  disposition, narrow handle-named retry proof, and epoch-scoped handle reuse.
  Do not absorb pre-commit failure recovery from `finish-failure-recovery-k100`.

## Done when

- Every review finding has a recorded disposition and every verified issue is
  fixed at the narrowest seam.
- The methodology, lifecycle behavior, and process regressions agree on the
  post-commit/no-observed-done window for Git and jj.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

- F1 (the producer's full-suite verification gate is not satisfied): **valid,
  discharged here without changing the producer**. The finding is about missing
  *evidence*, not about a defect in `post-teardown-restart-contract-k102` — the
  producer's diff adds two `finish_lifecycle` tests, a `content/SKILL.md`
  paragraph and a spec rewording, and touches no cleanup or barrier code. The
  narrowest seam for missing evidence is a recorded run, so this integration ran
  the gate on exactly the reviewed tree state (the producer's diff plus this
  chain's task bookkeeping, no production change): `cargo fmt --check` exited 0
  and `cargo test --locked` exited 0, including
  `process_cleanup_does_not_unlink_a_substituted_non_directory_entry` — the test
  whose failure withheld the gate. A second, targeted
  `cargo test --locked --test finish_lifecycle` run also passed, 59 tests in
  31.96s.
- That green run satisfies the producer's recorded gate; it does **not** discharge
  `cleanup-barrier-readiness-flake-k165`. The flake is load-dependent, so passing
  twice is evidence the barrier did not lose the race on these runs, not evidence
  it cannot. The durable "passes repeatedly" guarantee, and the atomic-publication
  fix behind it, stay owned by that leaf, which is live and unabsorbed — as its own
  notes and this leaf's context both require.
- No restart-contract finding to apply. The review's four no-finding claims were
  independently spot-checked rather than taken on trust, and each holds:
  `seed_jj_terminal_grove` leaves `outside.txt` rewritten to `after` in the
  working copy, so the jj lost-result retries really do run against a successor
  carrying unrelated work rather than a clean synthetic topology;
  `verify_lost_jj_finish` destructures `@`'s parent list to a single candidate,
  then demands the exact attempt-bound message, an untracked `.grove/`, and a
  diff of nothing but `.grove/` deletions, so no walk of older teardown history
  can become a lifecycle discriminator; and `EPOCH_HANDOFF_TIMEOUT` is
  `Duration::from_secs(30)`, matching both the methodology's "waits up to 30
  seconds" and the process regression's asserted diagnostic and floor.
- Methodology, lifecycle behaviour and process regressions agree on the
  post-commit/no-observed-done window in both VCS shapes.
  `content/SKILL.md`'s new paragraph makes three checkable claims, and each has a
  process-level regression: the no-signal stop reporting the child's real status
  and elapsed time, then a fresh `plan-k1`
  (`a_no_signal_exit_after_successful_teardown_stops_and_then_starts_a_fresh_grove`,
  asserting `status exit status: 23`, `elapsed `, and the absence of
  `grove finished`); the abandoned `done` a killed driver never interpreted, with
  the child ordering its parent's death and reaping ahead of the write
  (`a_done_signal_abandoned_by_a_killed_driver_reinitializes_instead_of_finishing`,
  matching the spec's reworded acceptance item); and the orphaned guard producing
  a bounded stop that creates no task tree
  (`a_shared_epoch_guard_blocks_the_post_finish_replacement_without_creating_a_tree`).
  The jj side of the same window is
  `bare_driver_reaps_orphans_and_ignores_abandoned_signals_in_jj_workspaces`,
  which runs native and colocated: after a successful teardown commit it plants an
  abandoned `done` in the control directory and proves the next bare invocation
  reports "without a completion signal", never "grove finished", and initializes
  `01-requirements-plan-k1.md`.
- One efficiency observation, considered and deliberately not acted on: the
  orphan-guard regression spends 30s of real wall clock re-proving a bound that
  `src/driver_lease.rs`'s unit tests already assert deterministically through an
  injected clock. The process test buys something the unit tests cannot — that no
  `.grove/` is created and no session launched while blocked — so the wall clock
  is the price of the end-to-end proof, not waste. Too marginal to earn a leaf.
- No production, test, or durable-record artifact changed in this integration.
  `docs/specs/config-driven-sessions.md`, `content/SKILL.md`, the ADR set and
  `CONTEXT.md` already describe the contract the review found intact.
