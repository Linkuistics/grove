# ravel-session-channel-k21


## Goal

Deliver Ravel's ephemeral opaque-result transport through the existing local
runner, independently verified with real supervised fixture processes.

## Context

Ravel main starts at a6b9bab17ce2e7c261b4d063d237e8989db4c1d2. Its graph's
2026-09-08 generation predates the replacement tree; source reads are required.
Keep keyed-launch generic and compose it in the private `ravel` Rust library.
k22 adds the policy-routed binary; k23 owns installation and whole-pair evidence.

## Done when

- A private ravel package allocates a fresh channel in a private temporary
  directory, publishes its path as RAVEL_EXIT_FILE and accepts one opaque UTF-8
  result using inherited context, without explicit session-target arguments.
- Publication is atomic and first-writer-wins. Empty/multiline values survive;
  missing/stale context and nonregular channel entries refuse without blocking.
  Result reads occur only after confirmed reap/group cleanup, not cancellation.
- Existing runner process fixtures prove signal/escalation/reap, inherited
  nested-context isolation and cleanup; no persistent store or public CLI stub.
- Cargo and direct fixtures scrub live old/new authority. Taskfile focused
  tests and the full Rust check pass with complete saved diagnostics.
- Ravel commit and exact validation are recorded here. Retire only k21; k20,
  k18 and k15 remain open while k22/k23 are live.

## Decisions (running log)

**Boundary and plan.** The approved two-verb contract needs a result-carrying
channel; keyed-launch deliberately observes appearance only. Compose it with a
private ravel library instead of changing the runner's protocol. First add
contract tests, then atomic publication/anchored reads, then real-process
round trips and nested/stale/error cases. k22 owns stdout/session-output
separation at the front, routing, cancellation exit status and CLI validation.
The existing workspace dependency versions and nextest framework remain.

**Publication.** Publish a completely written regular file with a no-clobber
hard link in the channel's private directory. A second publication cannot
change the first result. Read through a held directory descriptor, reject
links/FIFOs/directories and bound reads to the maximum accepted result size.
The result bound is 1 MiB, above supported OS command-argument limits.

**Review and cleanup.** The one in-session adversarial reader found that
TempDir's pathname cleanup could delete a replacement directory after the
owned directory moved. This was actionable: the regression reproduced
`close() == Ok` with replacement deletion and owned-result leakage. Cleanup
now disarms TempDir's destructor and checks held/path directory identity;
explicit close refuses a mismatch, Drop preserves the replacement. Both
cleanup regression tests pass. The fix is covered by this executable seam;
no second reviewer is needed. The reader found no other actionable issue.

**Consumer obligations.** result_after requires the corresponding runner
Ended, rejects cancellation/group presence and treats no observed signal as
None, separately from an empty value. k22 must preserve that association,
report cleanup errors and keep result stdout apart from session output.
The package is private and has no binary/placeholder verbs. Reads are held-
directory anchored; same-user malicious code is not authenticated by context.
Abrupt death or an externally moved directory can leave ephemeral debris;
this is documented, not a persistent record subsystem.

## Notes

No model launches, personal configuration copying or installation. End this
original supervised session with harness-dispatch exit, not the new tool.

## Validation and handoff

Ravel commit: `fe99094029f4e00c9ec6f57312f0925c8702157b`
(`ravel-session-channel-k21: carry opaque results through an ephemeral supervised channel`).
Local main names this sealed commit; the next working copy is empty. No push,
installation, policy/configuration copying or release was performed.

- `task build` passed with the committed lockfile and 0.0.0 private runtime.
- `task test:session` verified the new focused task and process cases;
  `task test:session -- --lib` verified its optional nextest filter forwarding.
- `task check` passed rustfmt, ShellCheck, all-target Clippy and nextest:
  **74 passed**, 14 ignored subprocess fixture roles invoked by actual tests.
  Full lint/test log: `/Users/antony/Development/Ravel/target/check.xqGB7a`.
- All tracked source/config/documentation subjects from `jj file list` were
  SHA-256 digested before/after that complete check; every digest matched.
  The ignored target/session-check-{subjects,before,after} files hold the
  subject list and readings. No source was edited during measurement.
- The original session's real HARNESS_DISPATCH_EXIT_FILE remained absent
  before the ending signal; existing Cargo/direct-helper guards cover the
  RAVEL_EXIT_FILE spelling without adding new live authority names.

k22 must now compose this private runtime with the actual generic policy
worker and front. k23 owns complete installation/probes/executable documentation.
The parent k20/k18/k15 Done when conditions are not yet met; they stay open.
k16 remains actual configuration copying only, and all later release leaves
remain live. No caller-reference completeness or installed CLI claim is made.
