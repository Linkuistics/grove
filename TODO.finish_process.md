# TODO — the finish process

A scoping note, not a plan and not a decision. It records what the finish
process costs today and which questions a `requirements` session would have to
grill, so the next person starts from measurements rather than from an
impression. **Delete this file** when the work lands, or when the answer turns
out to be "keep it as it is" — in which case the reasoning belongs in an ADR,
not here.

## Why it is written down

The 2026-08-17 simplification pass measured the repository and left this one
subject untouched on purpose: reshaping it is a redesign, not a
contract-preserving simplification, and it was outside what that pass was asked
to do. What follows is that pass's evidence, so it is not re-gathered.

## What it costs

| Module | Lines | Role |
|---|---|---|
| `src/finish_transaction.rs` | 3,632 | Preflight, witness, evacuation, rollback, quarantine handoff, recovery |
| `src/repo/finish_commit.rs` | 2,953 | The Git / native-jj / colocated-jj commit seam and its three dispositions |
| `src/finish_cleanup.rs` | 950 | Post-commit quarantine disposal |
| `src/finish_cleanup/auxiliary.rs` | 1,257 | The cleanup marker protocol |
| `src/finish_cleanup/auxiliary/marker_replacement.rs` | 960 | A crash-safe marker-replacement sub-transaction |
| `src/finish_cleanup/unix.rs` | 535 | Raw `openat` / `renameat2` / `unlinkat` wrappers (31 `unsafe` blocks) |
| `src/finish_cleanup/reaper.rs` | 79 | Lease-owned reaping of orphaned quarantine |
| **Production total** | **10,366** | **34% of `src/`** |

Plus 6,701 lines of test: `src/finish_cleanup/tests.rs` (936),
`src/finish_cleanup/auxiliary/tests.rs` (1,634), `tests/finish_lifecycle.rs`
(4,131).

For one operation, run once per grove, at the end.

## Where the complexity actually is

Three **nested crash-safe transactions**, each with its own on-disk protocol:

1. **The finish transaction** — builds a `PREPARING-FINISH-<handle>/` witness,
   publishes it with one atomic rename to `FINISHING-<handle>/`, evacuates every
   ordinary root entry under a manifest recording each entry's type and
   canonical no-follow recursive digest.
2. **The commit seam** — classifies the outcome as *Committed*, *Not committed*,
   or *Recovery pending* from the recorded anchor and the exact immediate
   result, across three VCS shapes, revalidated either side of the filesystem
   handoff.
3. **Quarantine cleanup** — its own marker documents, its own staging and
   replacement protocol built on `renameat2` exchange, and an orphan reaper that
   only touches entries carrying Grove's own cleanup manifest.

Each is individually well argued (see
[`task-tree-transactions-fail-closed`](docs/adr/task-tree-transactions-fail-closed.md)
and [`ARCHITECTURE.md`](docs/ARCHITECTURE.md#finish-transaction)). The question
is whether all three layers are still load-bearing together, or whether the
third grew to protect an intermediate state the first two could avoid producing.

## Questions worth grilling

Ordered by how much they would change the answer. None is rhetorical — each has
a plausible "no, it earns its keep" outcome.

1. **Does the quarantine need to exist?** It is cleanup garbage, never a receipt.
   If the evacuation witness could be disposed of in place after a proven commit,
   layers 1 and 3 collapse into one, and `auxiliary.rs` +
   `marker_replacement.rs` + most of `unix.rs` (3,000+ lines, all 31 `unsafe`
   blocks) go with it. What would break: disposal is not atomic, so a crash
   mid-disposal must be resumable — which is the problem the marker protocol was
   written to solve. Is there a cheaper resumption story now that the migration
   transaction has one?
2. **Can the three dispositions become two?** *Recovery pending* is the
   operator-recoverable state and the one that generates the most surface. It
   exists because neither commit nor its absence can always be proven. How often
   is it genuinely reachable per VCS shape, and is the Git case the only one that
   needs it?
3. **Is the marker-replacement sub-transaction reachable?** It is a whole
   crash-safe protocol nested inside the cleanup of a crash-safe protocol.
   Enumerate the states that require *replacing* rather than creating or removing
   a marker.
4. **What does `finish` still owe the user?** Teardown deletes `.grove/` in one
   focused commit. Grove deliberately does not merge, remove worktrees, or delete
   branches. Given that, how much of the machinery protects the repository versus
   protecting Grove's own intermediate artifacts?

## Constraints any answer must hold

- **The interval is the whole problem.** Between removing `.grove/` and recording
  that removal, a later invocation would read a fresh grove. Nothing may
  reintroduce a window where that is observable.
- **Never rewrite history to clear a blocked state.** An unresolvable outcome
  stays blocked and operator-recoverable, naming the artifact, the recorded and
  observed topology, and the two restorable exits.
- **Three VCS shapes stay symmetric** — Git, native jj, colocated jj — per
  [the VCS seam](docs/ARCHITECTURE.md#symmetric-vcs-rule).
- **The HITL boundary is not machinery.** `finish-commit` cannot attest that a
  human spoke through an opaque command; it is the deterministic last-moment
  guard, not a substitute for the confirmation contract.
- Deleting a fail-closed step is not simplification if it converts a refusal into
  a silent wrong state. The 2026-08-17 pass has a worked example: withdrawing a
  legacy migration reader while *keeping* its format classification, because a
  tree that classifies as empty gets a format witness written over live work.

## Adjacent, and deliberately not in this file

`tests/lifecycle_cutover.rs` (1,884 lines) is the live end-to-end driver suite —
launch, config reload, spawn failure, build pairing, re-provisioning — under a
name that reads as history. Renaming it is unrelated to the finish process and
costs only churn; it is noted here so it is not lost.
