# grove-session-control-k8

## Goal

Make Grove honor Ravel's explicit force-stop while keeping continue, final
completion and the original lifecycle authority guarantees intact.

## Context

- Requires ravel-session-control-k7's installed release and documented public
  result. Use that report through existing launch-directory/lifecycle and
  standalone seams; no Ravel Rust source or private worker protocol is consumed.
- Grove currently reads an observed exit_signal as a boolean and owns the
  teardown record separately. Ravel now owns generic session-control prose;
  Grove's prompt and methodology retain only caller/task/teardown obligations.

## Done when

- Through real installed Ravel, continue advances to remaining work and a final
  completion satisfying Grove's teardown contract finishes. Explicit force-stop
  stops the sequence without another launch or false success, leaving live work
  resumable. Define and test precedence when force-stop and teardown coexist.
- Lifecycle and standalone fake-agent suites cover both endings, ordinary agent
  exit, refusal, cancellation, malformed/missing/unsupported reports, stale epoch
  and nested channels. Standalone force-stop cannot publish a successful artifact.
  Terminal recovery and group reaping remain covered by the existing PTY seams.
- Grove's prompt and methodology no longer duplicate Ravel's generic notice and
  ending instructions. Finish still records caller teardown before its final
  Ravel ending command; other kinds retain their caller-owned retirement rules.
- Update Grove's formula minimum dependency only if the new public contract
  requires it; verify independent versions/installation paths through archive
  and formula checks. Reconcile active docs, glossary and ADR/spec descriptions.
- Run `task check` and the relevant installed-layout checks. Keep the source
  migration separate from the live session's original launch/ending contract.

## Notes

The agent-server stays on the root brief's horizon. The driver-owned finish
session performs the authorized final Grove integration/release; this leaf
does not signal teardown or install replacement Grove while developing it.
