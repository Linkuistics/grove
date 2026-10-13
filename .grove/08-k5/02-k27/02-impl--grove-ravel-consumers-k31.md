# grove-ravel-consumers-k31


## Goal
Switch both lifecycle and standalone callers atomically to installed Ravel's
public start-session/end-session boundary, completing grove-ravel-runtime-k27.



## Context
Consume session-stdout-capture-k30's separate stdout entry points and k26's
PATH/version resolver. Read the node's full inherited contract. Capture the
stdout result into a held, caller-owned regular file outside managed writes;
the captured bytes, never stderr/transcripts or a private worker report, are
the result. Preserve reap/epoch invalidation and teardown precedence.

The new APIs are `keyed_launch::run_observed_with_stdout(Launch, File, observer)`
and `keyed_launch::run_noninteractive_with_stdout(NoninteractiveLaunch, log, stdout)`.
Their Files are owned, regular-file checked before spawn, and passed directly
to child stdout. They do not truncate, rewind, bound or interpret result bytes;
those are caller responsibilities. Existing capture-free APIs remain available.

## Done when
- Complete every grove-ravel-runtime-k27 Done when, including both actual result
  readers accepting exactly `grove-continue` and rejecting missing/unsupported
  results, both executable paths resolved once, prompts/help/end commands,
  confinement and complete old/new authority scrubbing.
- Remove staged resolver dead-code expectations and obsolete sibling locator.
  Installed Ravel 0.1.0/worker tests exercise both readers/launches, stream
  isolation, distinct versioned prefixes and all specified decoys.
- Full relevant runtime nextest, continuation/completion/refusal/resume,
  cancellation/process group/PTY/epoch/stale checks pass. Update every changed
  source-pinned book; leave bundled delivery green until k6. k28 owns the generic
  adapter/provenance assets and k29 the final surface audit/task check.
- Do not install Grove or alter this live v22 ending/admission/teardown contract.

## Notes
If the remaining fan-out exceeds a focused session, decompose at another green
expand/migrate seam. Do not split lifecycle and standalone into inconsistent
current prompt/result contracts. Parent k27 stays live until both migrate.
