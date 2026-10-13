# grove-ravel-switch-k33


## Goal
Complete the atomic lifecycle and standalone migration to installed Ravel's
public start-session/end-session boundary.



## Context
Read the inherited k31/k27/k5 contracts and session-result-capture-k32's API
and committed tests. k32 is only an expansion: actual callers still use the old
locator/report/prompts. Consume its held-file reader through both k30 runner
stdout APIs. Interpret only after reap and lifecycle epoch invalidation;
preserve teardown precedence, cancellation and survivor checks.

The shared API is `grove_loop::session_result::SessionResult::allocate()`,
`capture.stdout()` (owned File for either k30 runner) and
`capture.read_continue()` (consumes the capture, returns the opaque Grove Error
on missing/unsupported/unreadable bytes). `CONTINUE_RESULT` is the frozen
literal. The file is fresh, mode 0600, held and unlinked; no launch-directory
result path is needed. Writer clones share its offset; the reader rewinds and
reads at most 15 bytes. Do not interpret it before the caller's authority and
supervision checks. k32's real installed fixture is
`crates/grove-loop/tests/session_result_installed.rs`; its task is
`task ravel:result RAVEL=/absolute/path/to/ravel`.

## Done when
- Every remaining grove-ravel-consumers-k31 and grove-ravel-runtime-k27 condition
  holds. Both actual callers accept exactly `grove-continue` via the shared
  capture and refuse missing/unsupported results without transcript parsing.
- Use the PATH/version resolver once for launch, ending commands and documented
  runtime confinement reads; remove staged dead-code expectations and the old
  sibling locator. Migrate prompts/help/finish/Creator compatibility instructions
  and all old/new authority scrub lists together, including Cargo and tests.
- Installed Ravel/worker fixtures cover both actual consumers, stream isolation,
  separate versioned prefixes, missing/old/malformed/cwd/sibling decoys and the
  inherited lifecycle/process/PTY/epoch/stale cases. Full relevant nextest passes.
- Update every changed source book and preserve bundled delivery until k6.
  k28 owns generic adapter/provenance assets and k29 the final surface audit.
  Never install Grove or alter this session's original v22 ending contract.

## Notes
