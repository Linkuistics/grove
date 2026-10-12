# ravel-generic-pair-k18 — brief

## Goal

Deliver the generic two-verb Ravel process manager/router, its documentation and
verified temporary-prefix installation.

## Context

Requires ravel-local-runner-k17. Source front/worker remain in this Grove checkout.
Read the root/ravel-cli-k3 current contracts and Ravel's docs/session-cli.md.
The human narrowed the design during this session; this replaces prior acceptance
conditions that required storage, historical continuity or prompt injection.

## Done when

- Generic 0.0.0 pair and local runner build/check/install through root Taskfile;
  preserve pinned dependencies, matching worker identity, runtime probes/notices.
- Only start-session and end-session RESULT. End uses inherited process context;
  start returns the opaque supplied result after termination/reaping. Preserve
  the prompt unchanged; the caller supplies ending instructions/result semantics.
- Remove persistence, SQLite dependency, record/observation/lookup APIs and their
  worker/SDK/test surface. No importer, snapshot or catch-up instrument. Remove
  embedded caller modules/reporting and rewrite a useful generic routing sample.
- Reconcile executable spec, process/runtime ADRs and Bun evidence/instruments.
  Sweep the whole active tree for caller references with observed controls;
  preserve only named source-retirement and authority-guard exceptions.
- Deterministic, concise Taskfile checks and existing runner/worker/fake-agent
  seams cover the smaller contract. Test result round trips, nested/stale context,
  reaping/cancellation/terminal/confinement and direct/symlink installation without
  host runtimes or Grove. No model calls. Live exit authority stays absent.
- Commit Ravel and record exact evidence here; retire k20 and check/promote this
  node and k15 if complete. k16 remains live for configuration copying only.

## Decomposition

The session initially cut k19 for a general history-import instrument. The human
rejected historical continuity and then removed storage entirely; k19 is
abandoned and its prototype/development dependencies were removed. k20 owns the
entire remaining minimal runnable handoff. This session preserves the complete
old-tree retirement, records the new scope and improves deterministic check
output; it does not claim a runnable Ravel CLI has landed.

## Decisions (running log)

**Current boundary.** Two verbs, opaque end-session result, unchanged prompt,
process-context addressing and no persistent state. Caller instructions and
result/provenance meanings belong to the caller. Ravel is a process manager and
router at this stage. Preserve current configuration choices and pinned process
runtime instead of adding agent-server mechanisms.

**Testing efficiency.** Reuse deterministic Taskfile workflows, retain full
logs and return small pass/fail summaries. Use targeted tests while editing and
one full relevant suite before handoff; investigate failure logs selectively.
No new testing framework or legacy-store suite is needed.

## Testing runner

The human selected cargo nextest. Rust test tasks use nextest and preserve its
reporting UI; formatting/lint diagnostics may be saved with short summaries.
Use focused filters during edits and the full relevant suite at handoff. The
nextest runner replaces cargo test for runtime suites; retain doctest checks
when the source contains executable documentation examples.

## Earlier baseline handoff

Ravel commit: `a6b9bab17ce2e7c261b4d063d237e8989db4c1d2`
(`ravel-generic-pair-k18: narrow the session boundary and use nextest`).
Local main names it and the next working-copy change is empty. The commit
records the two-verb contract, complete local retirement and concise checks;
**it does not contain the front/worker CLI implementation**. k20 remains live
and owns that complete runnable handoff. k15/k3 remain open; k16 now copies
configuration only. k19 and Ravel prompt-injection k7 are abandoned on the
human's explicit scope correction. Grove result handling k8 remains caller work.

- `task check` passed formatting, ShellCheck and all-target Clippy, then
  cargo-nextest 0.9.148 ran 60 tests: 60 passed, 11 skipped subprocess roles
  driven by parent tests. Test execution took 3.672 seconds. Nextest's UI is
  presented directly; formatting/lint diagnostics are in target/check.WtF3Kh.
- A temporary fake cargo forced a formatting failure: the check stopped,
  returned nonzero and displayed saved failure diagnostics. No new framework.
- `task --dry test` confirms the standalone task uses the same nextest command.
  All 27 tracked validation inputs retained before/after SHA-256 digests.
- The retired local/origin bookmark remains at 035edfa30f3bf40eb870a525f49ac98a3b5abbb1.
  A known-old-source positive control finds the original files there; none remain
  in the active tracked tree or old local source/build/editor directories.
  Ignored remnants were moved to the sibling archive documented in Ravel.
- The discarded importer, SQL assets, Python manifest/lockfile, tests and
  generated environments/caches were removed. No personal history was opened
  or copied. Only source retirement was preserved.

The source graph supplied Tier 2 discovery. Ravel's graph still describes the
retired project, with changed/untracked metadata and excluded docs, so this
handoff used direct source/diff inspection and executable checks. No in-session
reviewer was needed for the explicit human scope decisions or tooling wrapper;
the executable failure control verifies the wrapper's material behavior.

## Complete runtime handoff

The runtime subtree closes with ravel-installed-pair-k23. Ravel local main is
`b1060a176d5fa7c6c5bc96f41cacc18480e0fb65`, with an empty following working copy.
k21/k22/k23 and the owned runner meet this node's full runtime contract; k23's
terminal task records exact checks, controls, coverage caveats and logs.
`task build`, `task check` (134 runtime tests passed; 14 subprocess roles) and
fresh-prefix direct/symlink `task smoke` passed. The pair and worker/SDK/notices
are 0.0.0, without a host runtime or source-project binary. Runtime docs and Bun
instruments accompany source. No publication or actual configuration copying
occurred; no old store was opened. k16 remains live for the absent destination
owner configuration, and k4 owns target/archive/release preparation.
