# finish-lost-result-retry-k163

**Kind:** impl

## Goal

Make a confirmed finish session that lost its `finish-commit` result able to
retry idempotently, by proving the exact attempt-bound teardown commit instead
of trusting task-root absence.

## Context

- Binding design: `docs/specs/config-driven-sessions.md` "Crash and retry
  semantics"; ADR `task-tree-transactions-fail-closed`; glossary term Complete
  finish cycle.
- Current behavior: `tree_lifecycle::finish_commit` bails
  `"this grove is already finished"` on a missing `.grove/` before any
  repository seam runs, so a lost result can never succeed and the CLI cannot
  reach `complete --done`.
- The proof must be **self-contained**: successful cleanup may already have
  disposed the manifest, so it derives its parent from the result itself. Do not
  reuse or relax `repo::recover_finish`, whose anchors come from the witness.
- Required facts, per the spec: the immediate result's message names the
  requested handle *and* this launch's attempt identity exactly; its delta
  against its own parent deletes only `.grove/` paths; the result tracks no
  task root; and it never requires the working-tree-only finish leaf in the
  parent. Git checks `HEAD`; native and colocated jj check the committed parent
  of the successor working-copy commit.
- The attempt identity is the active launch nonce, already read by
  `finish_transaction::finish_attempt_identity`. A rootless retry requires a
  still-active session epoch, so a new grove or replacement launch — even one
  reusing the same handle — must not satisfy it.
- Reachable only while the task root is absent. This is narrow command-outcome
  verification for the current invocation, never lifecycle state a later bare
  driver may read.

## Done when

- A lost `finish-commit` result retried under the same active launch returns
  idempotent success in plain Git, native jj, and colocated jj, without making a
  second commit.
- Task-root absence with no matching commit stays a refusal that never licenses
  `done`, including a repository whose last commit has the right handle but a
  different attempt identity, touches a path outside `.grove/`, is not the
  immediate result, or still tracks `.grove/`.
- A prior completed grove with reused handles, followed by external root removal
  or reset, cannot satisfy a new epoch's confirmed session.
- Diagnostics distinguish "no Grove teardown result to verify" from a
  near-miss, naming what was required and what was observed.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

TDD at the repository seam and the `finish-commit` entry point. Keep the
existing `retrying_finish_commit_after_teardown_reports_already_finished`
expectation honest: a repository with no teardown commit must still refuse, and
its diagnostic may change wording but not disposition.

Reuse the acceptance fixtures in `tests/finish_lifecycle.rs`
(`init_git`, `init_jj`, `seed_committed_terminal_grove`, `grove_llm`) and the
existing `GROVE_SIGNAL_FILE`-derived attempt identity rather than a new
injection seam.
