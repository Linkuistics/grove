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
