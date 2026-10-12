# ravel-routed-session-k22


## Goal

Deliver the working two-verb Ravel command front and generic compiled policy
worker, composing k21's ephemeral channel with the existing supervised runner.



## Context

Requires ravel-session-channel-k21. Read its handoff and Ravel's
docs/session-channel.md plus the current brief chain. Transfer the applicable
source from crates/harness-dispatch, not its store/record/observation modules.
Preserve pinned Bun 1.4.2 and the real-worker/fake-agent seams.

Start from Ravel main at `fe99094029f4e00c9ec6f57312f0925c8702157b`
with an empty working copy. The private ravel package has no binary yet.
Its SessionChannel/end_session/result_after/close API is documented in
docs/session-channel.md. The new task test:session accepts nextest filters
after `--`; task check saves nextest's full visible reporting in its log too.

## Done when

- Exactly start-session and end-session RESULT plus ordinary help/version flags.
  Route actual policies, pass the prompt unchanged, keep session/diagnostic
  streams separate and return only the exact opaque result on stdout after
  reap/group cleanup. Explain and test child exit without a result/cancellation.
- Preserve settings/bounds, environment grants, context reads, choice-file
  and routing semantics. Generic ravel/sdk/examples/sample; no Grove adapter,
  prompt injection, store, SQL dependency, run lookup or record protocol.
- Build/typecheck the matched 0.0.0 front/worker using Taskfile workflows;
  existing command seams cover routing, refusal, bounds, context, cancellation
  and result round trips. Extend the session-authority scrubs for any new names.
- Record Ravel commit/validation here and retire this leaf. k23 remains live
  for complete installed-layout/probe/documentation evidence.

## Notes

No public bypass-policy/direct-command mode, third compatibility verb, model
launch, configuration copying, publication or replacement of the live driver.
Read the source graph first and check coverage; fall back to source where stale.

## Decisions (running log)

**Implementation plan.** Reuse the current source's policy authority, inputs,
bounds, environment, validation, context reads, framed worker conversation,
program resolution and cancellation. Remove store/lookups and embedded Grove
adapter at the source boundary. Transfer the pinned compiled worker build and
source-identity check with generic SDK, static example and choice sample.
Add command tests first, then transfer/evaluate, compose SessionChannel with
keyed-launch, and verify through Taskfile. Existing configuration semantics and
owner labels remain; no new runtime or test framework. k23 owns installation,
hostile-probe completion and comprehensive executable documentation.

**Streams and endings.** The public front reserves stdout for the exact result
bytes (no added newline), routes the managed process's stdout to stderr before
spawn, and retains stdin/terminal ownership through keyed-launch. A result is
successful only after corresponding confirmed reap/group cleanup, regardless
of the managed process's natural status. An unreported normal child exit is
an error (exit 1); cancellation returns its signal with no result. Missing/stale
end-session context refuses. Ravel interprets no result value.

**Pinned development build.** The host Bun was 1.4.3; the worker remains pinned
to 1.4.2. `task setup` fetches the supported host's committed runtime tarball,
verifies its existing SHA-256 pin and installs it beneath ignored target/tools,
then uses the frozen TypeScript/Bun declaration dependencies. Build/check wrappers
save complete logs. Both build identity implementations omit the removed caller
modules. The SDK has only generic reads/selection/choice and a static example;
review provenance belongs in caller context facts, with no lookup protocol.

**One reviewer, four actionable findings.** The in-session reviewer challenged
only the process/signal/stdout composition. Cancellation precedence on prelaunch
errors, explicit early cleanup, reporting both cleanup errors and surviving-group
warnings under cancellation were all valid and actionable. Execution now joins
one explicit cleanup/settlement path, checks both selection and runner signal
latches, and reports every cleanup/group error before choosing cancellation.
Unit contract seams exercise cancelled prelaunch failure with two cleanup errors
and a surviving group. An actual command regression verifies unwritable result
stdout refuses before launch while a failing private-directory cleanup is reported.
That regression was observed failing before the descriptor check was added and
passing after it. These executable seams cover the fixes; no second reviewer or
review leaf is needed. No finding was classified as noise or accepted trade-off.

## Validation and handoff

Ravel commit: `e3dcd9c5e8522ad05eedee193a8cf49bdb2e08b9`
(`ravel-routed-session-k22: route and supervise opaque-result sessions through the generic policy pair`).
Local main names the sealed commit and the next working copy is empty. The
retired local/origin bookmark remains at
`035edfa30f3bf40eb870a525f49ac98a3b5abbb1`. No push, installation,
publication, model launch, personal configuration copying or store access occurred.

- `task setup` fetched/verified native Bun 1.4.2 against the committed runtime
  digest and installed the frozen dependencies. The host Bun remains unchanged.
- `task build` passed with the matching 0.0.0 front/worker. Complete final build
  log: `/Users/antony/Development/Ravel/target/k22-build.log`.
- Focused command checks cover the two-verb/help/version surface, exact ordinary,
  empty, multiline, Unicode and flag-like results, unchanged prompts, diagnostic
  separation, policy refusal, unreported child exit, stale context, nested
  isolation/first publication, context reads/measurement, params/choice files,
  environment grants/exclusions, bounds, real-worker deadline, cancellation and
  group reap, confinement/overlap, missing/mismatched pairs and prelaunch output
  failure. Only fixture processes and temporary owner resources are involved.
- `task check` passed rustfmt, ShellCheck, worker compilation, source/declaration
  typechecking, all-target Clippy and nextest: **119 passed**, 14 ignored fixture
  roles invoked by acceptance tests. Runtime suite took 9.232 seconds. Full
  visible nextest/lint log: `/Users/antony/Development/Ravel/target/check.wm0aKa`.
- All 1,121 tracked source/config/docs, worker development dependency files,
  checkout-local Bun, generated declarations/notices and front/worker executable
  inputs retained matching before/after SHA-256 digests, compared individually.
  `target/k22-check-{subjects,before,after}.json` hold those inputs and readings.
  No source was edited during the measurement.
- Tier 2 source discovery used Grove's 2026-10-12 graph, the exact
  choose_prepared outbound trace/snippet and exact transfer-path coverage checks
  (metadata matches, no recorded code gaps). Ravel's graph remains the retired
  2026-09-08 generation: current files are not tracked by that generation and docs
  are excluded, so current source reads and executable evidence supplied the
  fallback. No exhaustive graph-completeness claim is made.
- The original live HARNESS_DISPATCH_EXIT_FILE remained absent after tests.
  Cargo/direct fixture guards already scrub the sole new authority spelling
  RAVEL_EXIT_FILE; command fixtures additionally start from an empty environment.

k23 remains live. It owns installation tasks, complete fresh-prefix direct and
symlink smoke, the transferred hostile-probe test suite and runtime evidence,
executable spec/ADR transfer and whole-active-tree caller-reference classification
with observed controls. The probe build mechanism and runtime notices are present,
but this leaf did not run the full hostile/probe or installed-layout suite.
The source-pair development layout is target/debug/ravel plus
 target/libexec/ravel/ravel-policy. SDK declarations/examples/sample are generated
beside the worker. Obsolete storage settings now refuse without opening a store.

k20/k18/k15 remain open because k23's installed/documentation conditions are not
yet met. k16 remains actual configuration copying only, and release/caller leaves
stay live. The original driver's installed commands and launch authority remain
unchanged; this session ends with its mandated harness-dispatch exit.
