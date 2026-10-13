# grove-lifecycle-control-k59

## Goal

Implement caller-owned lifecycle force-stop through installed Ravel 0.1.0,
preserving continuation, teardown, epoch invalidation and job cleanup.

## Done when

- Grove recognizes exactly `grove-continue` and `grove-force-stop` from held
  Ravel stdout. Missing, malformed and unsupported bytes remain rejected.
- Driver interruption wins first; a successful Ravel force-stop wins over
  teardown; teardown retains its existing precedence over other endings;
  surviving groups and unsuccessful/missing reports cannot continue.
- Real installed fake-agent lifecycle tests demonstrate continuation, stopping
  with live work and resuming, and force-stop coexisting with valid teardown.
  Existing cancellation, stale epoch, nested channel and PTY seams stay green.
- Lifecycle prompts supply both canonical commands and their meanings. Current
  lifecycle methodology/spec/ADR and affected source-pinned pages agree.
- Focused nextest suites and `task check` pass. No installation, release,
  teardown or replacement driver is started; this session ends with original
  `harness-dispatch exit` after retirement and sealing.

## Implementation plan

1. Extend lifecycle fake-agent tests first and demonstrate that force-stop is
   unsupported today, including a genuine teardown that currently reports success.
2. Add a typed result reader in `crates/grove-loop/src/session_result.rs` while
   retaining the continuation-only adapter for the standalone caller. Bound the
   read to the longest literal plus one, with no trimming or private reports.
3. Read once after reap/epoch invalidation in `loop_driver.rs`; apply the
   precedence above. Cover the full disposition/status/group/teardown matrix.
4. Supply an independently quoted force-stop command beside continuation in
   `session_command.rs`, `prompt.rs` and finish instructions; update tests and
   source-pinned walkthroughs at those seams.
5. Run the focused suites with real installed Ravel, then `task check`. Record
   outcomes here, retire this child and seal the handle-named jj change.

## Decisions (running log)

- k8 spans independently useful lifecycle, standalone and final acceptance
  increments. Decomposed it; this session owns only lifecycle k59, with k60 and
  k61 carrying the remaining parent criteria.
- The opaque token is `grove-force-stop`. Only a successful Ravel invocation's
  exact token overrides teardown. Driver interruption remains first. Neither
  unsupported reports nor a failed supervisor acquire new lifecycle authority.
- Keep `SessionResult::read_continue` for standalone until k60 changes its
  publication path and instructions. A typed reader makes lifecycle meaning
  explicit without changing Ravel or its release.
- Ordinary stops currently exit 0. Preserve that existing CLI behavior, but
  expose force-stop as a distinct `LoopOutcome::ForceStopped` with exit 1 so
  scripts cannot confuse an explicit force-stop with successful completion.
- The leaf's sole fresh-context reviewer found two actionable defects: pending
  TERM/HUP after reap could lose priority, and chained prompt substitutions
  reprocessed literal executable paths. Confirmed both with failing tests.
  Added post-reap latch consumption and template-only substitution; real PTY
  reset-window and literal-placeholder regressions cover these fixes. No
  second review is needed for these executable seams.

## Verification

- `task check`: all 18 principal checks passed, including 1,383 workspace
  nextest cases (31 skipped), nine doctests and all six final/all books.
  Complete log: `target/k59/check.log`.
- `task ravel:commands RAVEL=/opt/homebrew/bin/ravel`: two regular and two
  installed cases passed. The installed lifecycle case exercises both exact
  canonical ending commands and asserts unchanged policy prompt/task inputs.
- `task ravel:result RAVEL=/opt/homebrew/bin/ravel`: five reader and three
  explicit installed cases passed, including both stdout runner APIs, missing
  worker refusal and enclosing-authority scrubbing.
- `task test:authority RAVEL=/opt/homebrew/bin/ravel`: all nine sentinel cases
  passed. Separate logs: `target/k59/installed-{commands,result,authority}.log`.
- Real installed lifecycle regressions passed: continue to remaining work,
  force-stop with a reaped descendant and resumable leaf, genuine teardown
  overridden by force-stop, and post-reap TERM overriding either disposition.
  Their failing baselines and final runs are in `target/k59/`.
- All 2,983 frozen files matched before/after the full gate and still matched
  after installed checks: tracked repository files, installed Ravel 0.1.0,
  policy node_modules and pinned Bun. The inventory/digests are in
  `target/k59/check-inputs{,-after}.json`; this does not claim a freeze of every
  system/toolchain file read by those processes.
- `task legacy:verify` against the applied `.jj/legacy-driver-22.1.0` plan
  returned `ok`. Ravel checkout remained clean at published main 3916a4a1.
  Installed Ravel remains 0.1.0; no installation or release ran.
- Ravel's opaque-result/prompt contract was checked against its official
  v0.1.0 source at bbab823803bd2f2796f8bd9570f732dce900efa2. No new Ravel
  API or framework-version decision was needed.

## Handoff

k60 owns explicit standalone commands and publication behavior, consuming
`SessionResult::read`, `SessionDisposition` and `session_command::force_stop`.
k61 owns the parent's comprehensive active-surface and separate-target/archive
acceptance. The k8 node remains live; final integration/release still belongs
to the driver-owned finish and original installed 22.1.0 ending contract.
