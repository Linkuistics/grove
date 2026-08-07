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

## Done when

- `docs/specs/config-driven-sessions.md` records the KDL shape, substitutions,
  validation/errors, filename grammar, authoritative pick, automatic
  requirements/migration/finish transitions, and removed public surfaces.
- The minimum coherent ADR/spec set is reworked in place: obsolete routing and
  receipt decisions are merged, rewritten, or deleted rather than superseded.
- The agreed `grove` process and `grove-llm` tree test seams are explicit.
- No implementation decomposition is performed; that belongs to
  `implementation-slices-k10`.

## Notes
