# ravel-runtime-k15 — brief

## Goal

Deliver Ravel's independent two-verb process/router pair, its executable-owned
documentation and verified fresh-prefix installation.

## Context

Requires ravel-retired-project-k14. Ravel's runner is committed at ee5e64b1.
Read Ravel's README, docs/local-runner.md, docs/retired-project.md and
docs/session-cli.md. Source front/worker remain under crates/harness-dispatch.
The root and ravel-cli-k3 briefs carry the full current acceptance contract.
k16 owns actual configuration copying only; no historical migration remains.

## Done when

- Generic runnable 0.0.0 front/worker with matching identity; only start-session
  and end-session RESULT. Caller prompt unchanged; opaque result returned after
  supervised process termination/reaping. End addresses inherited process context.
- Root Taskfile builds/checks/installs the pair and local runner. Preserve pinned
  Bun, relevant dependency/toolchain/lint gates, notices and hostile probes. Tests
  save full logs and expose short summaries. No cross-repository Rust dependency.
- No persistent store, SQL/record/observation/lookup subsystem, old history
  migration, embedded Grove module, caller policy fixture or injected instruction.
  Generic SDK/sample/examples and docs expose the agreed process boundary only.
- Applicable executable spec/ADRs/Bun evidence and instruments move with their
  source citations. Enumerate/classify active caller references with controls.
- Package/runner checks and real-worker/fake-agent seams pass; temporary-prefix
  direct/symlink smoke works without host Bun/Node or Grove. Scrub old/new live
  authority from cargo/direct helpers and verify the live channel remains absent.
- Commit Ravel through jj, name exact commit/validation here, retire children,
  check/promote this node only when all runtime conditions hold. k16 stays live.

## Notes

No publication, release preparation, replacement of the old installed command,
model launch, actual personal config copying or store access in this node.
Preserve the original driver's canonical executable and ending contract.

## Decomposition

k17 delivered the standalone runner. k18 owns the remaining generic pair.
The current human scope removes store migration and all extra CLI verbs rather
than treating them as a mechanical requirement to carry into Ravel.

## Testing runner

The human selected cargo nextest. Rust test tasks use nextest and preserve its
reporting UI; formatting/lint diagnostics may be saved with short summaries.
Use focused filters during edits and the full relevant suite at handoff. The
nextest runner replaces cargo test for runtime suites; retain doctest checks
when the source contains executable documentation examples.
