# grove-session-control-k8 — brief

## Goal

Make Grove interpret its own force-stop result while keeping continue, final
completion and the original lifecycle authority guarantees intact.

## Context

- Requires grove-extraction-release-k13's installed two-verb Ravel integration.
  Extend Grove's own result/prompt conventions through existing lifecycle and
  standalone seams; no additional Ravel feature/release is needed.
- k5 establishes caller-owned continue instructions/results. Grove owns the
  meaning of end-session arguments and its teardown record; Ravel returns
  opaque values and never injects instructions or decides whether to continue.

## Done when

- Through real installed Ravel, continue advances to remaining work and a final
  completion satisfying Grove's teardown contract finishes. Explicit force-stop
  stops the sequence without another launch or false success, leaving live work
  resumable. Define and test precedence when force-stop and teardown coexist. Sessions
  invoke ravel end-session with Grove's opaque continue/force-stop tokens; Ravel
  interprets neither value.
- Lifecycle and standalone fake-agent suites cover both endings, ordinary agent
  exit, refusal, cancellation, malformed/missing/unsupported reports, stale epoch
  and nested channels. Standalone force-stop cannot publish a successful artifact.
  Terminal recovery and group reaping remain covered by the existing PTY seams.
- Grove's prompts/methodology supply all end-session instructions and result
  meanings. Finish records caller teardown before ravel end-session; other
  kinds retain caller-owned retirement rules. Ravel preserves these prompts
  unchanged. Source-pinned pages and command assertions change with their source.
- Homebrew continues to depend on Ravel by formula name. Its existing
  start-session/end-session contract carries both result values. Verify separate
  versions/paths using the already published target archives/digests; no notice
  or force-stop Ravel release is required. Reconcile active docs and ADR/spec clauses.
- Run `task check` and the relevant installed-layout checks. Keep the source
  migration separate from the live session's original launch/ending contract.

## Notes

### Lifecycle handoff — k59

k59 delivers lifecycle `grove-force-stop` through unchanged installed Ravel
0.1.0. Driver interruption wins, including pending post-reap signals; an exact
successful force-stop then overrides teardown and exits 1 without completion.
Teardown otherwise retains its original authority. Continue requires successful
status and a gone group. Real fake-agent tests cover continuation, remaining
work/resume, genuine teardown coexistence and descendant cleanup. Literal
template text in executable paths is preserved in both prompt commands.

All 18 `task check` checks passed (1,383 runtime cases, nine doctests, six books),
as did explicit installed command/result and authority tasks and original-driver
preservation verification. The child's verification section records exact logs,
frozen inputs and limits. k60 owns standalone's explicit stopping/publication
path; k61 owns full parent acceptance and remaining active-surface reconciliation.
No parent close, installation, release or teardown occurred in k59.

### Standalone handoff — k60

k60 supplies both exact canonical standalone commands and their meanings.
Continue publishes checked outputs; force-stop returns failure, records failed
status and publishes none. Both production callers now use the typed reader
directly; the temporary adapter is removed. Real installed fake-agent tests
cover both endings, valid whole output sets, descendant cleanup, unchanged
enclosing channels and nested completion/force-stop/cancellation. Existing
refusal, cancellation, natural exit, invalid-result and publication checks pass.

Final `task check` passes all 18 checks (1,385 runtime cases, nine doctests, six
books), alongside explicit installed command/result and authority tasks and
original-driver preservation verification. The host Bun 1.4.3 refusal was
resolved by selecting the existing pinned 1.4.2 binary only for verification;
all 2,983 frozen subjects match. The child's verification section names logs,
per-file digests and limits. k61 retains full parent acceptance and active
surface/separate-target reconciliation. No parent close, install, release or
teardown occurred in k60; the live original-driver boundary stays in force.

The agent-server stays on the root brief's horizon. The driver-owned finish
session performs the authorized final Grove integration/release; this leaf
does not signal teardown or install replacement Grove while developing it.
Final finish preserves k6's legacy installation/ending command until its last
signal; the running driver's launch contract still wins over installed updates.
