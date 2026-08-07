# config-driven-sessions-k6

**Kind:** design

## Goal

Produce the durable design for configuration-driven sessions and the simplified
single-command lifecycle, including a spec and only the ADRs that still earn
their place.


## Context

- Reconcile rather than layer over the current routing, harness stamp, receipt,
  provisioning, loop-driver, tree-name, and migration designs.
- The design must preserve direct foreground process ownership and Git/jj
  support while treating command templates as opaque launch policy.
- Closed requirements are the strict nineteen-kind config, POSIX-shell-word
  command templates executed without a shell, one driver pick whose prompt
  carries a stable-handle mandate, automatic restart-safe lifecycle transitions,
  removal of receipts/diversity routing, and explicit config-owned herdr
  integration.
- Design remains responsible for the exact KDL node shape, diagnostics,
  transaction mechanics, and coherent record boundaries; it must not reopen the
  confirmed product choices above.

## Done when

- `docs/specs/config-driven-sessions.md` records the KDL shape, scalar
  substitutions and `${herdr_settings}` splice, validation/errors and
  kind-addition compatibility, leaf-only filename grammar, prompt-carried
  mandate, authoritative pick, automatic requirements/migration/finish
  transitions, and removed public surfaces.
- Migration specifies all legacy directory and task-body inputs, terminal
  leaves, aliases, ambiguity errors, metadata deletion, fail-closed recovery,
  config-before-mutation ordering, and path/fileset-scoped Git/jj commits.
- Finish specifies driver-only creation, next position/key allocation,
  idempotent resume, decline-without-signal behavior, deletion commit naming,
  rejection by generic terminal verbs, path/fileset-scoped Git/jj deletion, and
  why the finish leaf is neither separately committed nor retired.
- The minimum coherent ADR/spec set is reworked in place: obsolete routing and
  receipt decisions are merged, rewritten, or deleted rather than superseded;
  the review-ownership predicate moves from session-side pick to the
  prompt-visible mandate, while the promotion transaction and tree-lock design
  survive independently of deleted receipt sections.
- The design records the herdr tree viewer's dependency on both the node scheme
  and the leaf filename/kind grammar, and preserves turn hooks plus `HERDR_AGENT`
  without hidden harness inference.
- The agreed `grove` process and `grove-llm` tree test seams are explicit.
- No implementation decomposition is performed; that belongs to
  `implementation-slices-k10`.

## Notes
