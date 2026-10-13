# grove-ravel-switch-k33 — brief


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

## Execution slices

The authority scrub is an independently verifiable expansion before either
caller changes its launch/result contract. The current lifecycle/standalone
lists, Cargo guard and direct test helper omit Ravel's inherited exit context;
the lifecycle list also omits the retired standalone channel. Add and prove
these guards first, then perform the remaining atomic migration.

- session-authority-scrub-k34 expands both production scrub lists, Cargo and the
  shared test helper to old/new control names. Real-policy controls prove both
  callers remove enclosing authority even when the owner grants it to policy;
  update the two affected source books and retain the existing dispatch path.
- grove-ravel-migrate-k35 completes all remaining migration requirements in this brief,
  including both actual launch/result readers, prompts/Creator/finish/help,
  installed Ravel fixtures, confinement and source books. No caller is switched
  by k34 and no ancestor closes at its retirement.

## Authority expansion handoff

k34 adds nine old/new names to both existing caller scrub lists, the Cargo
force-clear guard and the shared direct-test helper. `task test:authority`
injects inert values to prove the Cargo override; real owner-granted policy
controls refuse when called directly and complete through both Grove callers.
All 147 caller and 385 loop nextest cases, nine focused authority cases,
format/clippy and both affected final source-book checks pass. The complete
saved logs and 1,961 matching per-file before/after digests are under
`target/session-authority-scrub-k34/`. Build with the verified existing Bun
1.4.2 at `/Users/antony/Development/Ravel/target/tools/bun`; host PATH has 1.4.3.

Both actual launch paths, sibling locator, JSON ending readers and prompts still
use dispatch. k35 consumes the existing k26/k30/k32 APIs and this guard expansion
for the atomic migration. Adapt k34's two real-policy regression fixtures to
Ravel along with the other consumer fixtures; retain their observed dirty
control and ordinary-owner-grant checks. The installed original commands
retained their digests, and this live session still ends through dispatch.
