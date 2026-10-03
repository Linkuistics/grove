# supervised-run-k22

**Integrates:** supervised-run-k11

## Goal

Triage `supervised-run-k11`'s seven findings against the source and apply those
that hold, so that `confined-run-k12` builds on an agreed supervisor. This
session owns the corrections to `runner-job-k8`, `dispatch-supervises-k9` and
`run-ending-k10` and their verification, not the confinement work that follows.

## Context

- Read the findings in `supervised-run-k11`'s committed review artifact. Classify
  each on evidence (a contract stated unclearly, a real issue, a visible
  trade-off, or noise), rather than treating the list as agreed work.
- The reviewed producer is `supervised-run-k7`: `runner-job-k8` (`75af05fd`),
  `dispatch-supervises-k9` (`464499fe`) and `run-ending-k10` (`f6fc19aa`).
- F1 (a caller that ignores SIGCHLD breaks selection) and F2 (handlers dropped
  before the end is recorded) rest on reading. Reproduce each before fixing it.
- F3 and F4 each pick between two corrections, one in code and one in the spec.
  Settle which, and keep the spec, the glossary and the walkthrough books that
  describe it in step. A leaf that changes a crate's source updates that crate's
  walkthrough book in the same commit, and logs the change under `## Unreleased`
  in `CHANGELOG.md`.
- The owner's mid-session request about `harness-dispatch init` (`supervised-run-k11`'s
  R2) is not a finding. It needs the owner, so leave it for them.

## Done when

- Each finding is classified, and each real one is fixed with a test through
  dispatch's command or the runner, or accepted visibly.
- F6's missing seam cases exist through `harness-dispatch run`, or the spec's seam
  row says which are runner-level.
- `bash scripts/check.sh` passes.

## Notes

- This is a meta-grove: end the session with the installed v22 `grove-llm`, never
  `./target/debug/grove-llm`, as the root brief says.

## Decisions (running log)

- **I1 — F1: noise raised for want of context.** The requested reproduction,
  `inspect` with SIGCHLD ignored at entry, succeeds. `worker::evaluate` only uses
  the failed wait status to describe a conversation that ended without a result;
  a completed selection is returned independently of that status. Keep a command
  regression for inspection and add SIGCHLD to the harness entry-state table
  (F6), rather than changing selection on an unreproduced premise.
- **I2 — F2–F5: real issues; repair the implementation.** A store held with
  `BEGIN EXCLUSIVE` reproduces dispatch dying on TERM without an end notice
  (F2) and the ending file waiting on the append (F4). Runner seam tests
  reproduce cancellation injected during reclaim (F3) and duration including
  delayed observation/reclaim (F5). Preserve the existing reap boundary: latch
  cancellation and duration immediately after the wait, before the observer;
  keep late signals for the caller. Dispatch writes the ending file before the
  append and keeps handlers through the append and notice. A late signal ends
  dispatch after recording without relabelling the reaped run's ending.

- **I3 — F6: real issue, repaired with an explicit seam trade-off.** The
  dispatch signal-state table now covers ignored SIGCHLD, and inspection under
  the same caller also passes. The spec's seam row and producer/root briefs
  explicitly assign stopped-child and both other-foreground-group cases to the
  existing runner process/PTY tests. The runner owns these branches; dispatch
  uses that same runner. Keep the raw-mode and Ctrl-C dispatch PTY coverage.
- **I4 — F7: visible trade-off.** Include polling and the second group kill in
  decision 7's derivation: 8.53 s within 10 s under ordinary scheduling. Store
  migration is inside the append's one exclusive transaction, with no second
  lock wait. Scheduling suspension and slow I/O remain outside that budget.

- **I5 — Narrow signal-boundary review reconciled.** The reviewer identified
  the physical kernel-reap/read interval (contract stated unclearly), a second
  cancellation not drained when the first was already known (real issue), and
  tests placing signals by a delay rather than an observed boundary (real issue).
  Define the reap sample as the adjacent latch read immediately after wait and
  before elapsed/observer/recovery; a signal delivered in that tiny interval
  counts, later signals do not. No syscall can atomically order signal delivery
  with wait and an application latch. Eagerly drain the final latch, retaining
  the earlier cancellation; its new runner test failed before the repair.
  Use the ending file as the command tests' post-reap fence while holding the
  store lock; require the append to succeed after release so a lock timeout
  cannot masquerade as early publication. The dispatch exit also preserves an
  earlier cancellation's signal over a late signal. These repairs have
  executable coverage; no second in-session reviewer is needed.

- **I6 — Verification before the full check.** The ignored-SIGCHLD inspection
  and dispatched probe pass on macOS; the stopped-child and both other-group
  PTY cases pass at the runner seam. The three new runner boundary tests and
  dispatch's locked-store cases pass. Removing the earlier-cancellation
  precedence makes the command test fail (HUP recorded, TERM death instead);
  restoration passes. Keyed-launch's book reconstructs all seven roots, 2,234
  source lines, with no deferred lines. The graph generation predates the
  reviewed changes, so changed source and excluded documents were read directly.

- **I7 — Full-check failure investigated before rerun.** The first `task check`
  passed eleven checks, including every book, but cargo stopped at the unchanged
  deciding-agent timeout fixture: no agent PID had appeared within its 3 s
  selection bound. That case passes in isolation (both inspect and run), and all
  1,921 frozen repository-file digests match after the run. The bound includes
  worker/policy startup, so the fixture requires startup within its bound even
  under concurrent tests. Rerun the full check with `RUST_TEST_THREADS=4` to limit
  resource contention without changing assertions or the fixture. Refine the
  spec's cancellation table to include an earlier observed cancellation, and
  explicitly preserve the group-present failure's precedence over a late signal.

- **I8 — Acceptance verified.** `RUST_TEST_THREADS=4 task check` finishes with
  all 12 principal checks passing: the workspace tests, dispatch worker/probes
  and types, formatting/linting, install/conformance/release fixtures, and all
  six walkthrough books. All 1,921 repository-file digests match before/after
  this run. All seven findings are classified, with F2–F5 repaired, F1 rejected
  on source and command evidence, F6's seam allocation explicit, and F7's budget
  documented. The root retains live work in `confined-run-k12` and later steps;
  this leaf retires without closing an ancestor.
