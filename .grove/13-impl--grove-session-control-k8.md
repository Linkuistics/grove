# grove-session-control-k8

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

The agent-server stays on the root brief's horizon. The driver-owned finish
session performs the authorized final Grove integration/release; this leaf
does not signal teardown or install replacement Grove while developing it.
Final finish preserves k6's legacy installation/ending command until its last
signal; the running driver's launch contract still wins over installed updates.
