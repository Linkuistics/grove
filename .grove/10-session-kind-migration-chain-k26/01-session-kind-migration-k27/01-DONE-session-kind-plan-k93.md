# session-kind-plan-k93

**Kind:** impl

## Goal

Extend the deterministic migration planner so every accepted legacy layout is
validated and rendered directly into the current filename-kind tree without
mutating the source tree.

## Context

- Binding design: `docs/specs/config-driven-sessions.md` sections "Accepted
  inputs and mapping" and "Module interfaces".
- Extend the existing pure planning seam in `src/tree_migrate.rs`; transaction,
  recovery, and VCS commits remain later children of `session-kind-migration-k27`.
- Reuse the current session-kind grammar from `src/leaf.rs` / `src/leaf_id.rs`
  rather than maintaining a migration-only kind list.

## Done when

- Original `NNN` trees, v1 dotted-flat trees, and v2 body-kind trees all produce
  one complete current-format destination plan while a known `FORMAT` is a no-op
  and an unknown witness is rejected.
- Missing kinds map to `impl`; `work` aliases map to `impl`; known kinds map
  directly; standalone `research` maps to `research-a`; and only an unambiguous
  structural vendor pair assigns `research-a` / `research-b`.
- Empty, repeated, or unknown kind markers and ambiguous vendor-pair structures
  fail with source paths before mutation.
- Planned names preserve existing positions, keys, outcomes, relationships, and
  foreign files; rewritten bodies remove obsolete kind, harness, and producer
  launch metadata; destination collisions fail during planning.
- Focused planner tests cover the mappings, kind-prefixed legacy slugs,
  terminal leaves, relationships, ambiguity, and collisions.

## Notes

The deliverable is the pure, fully validated plan consumed by
`session-kind-transaction-k94`; do not add the transaction witness or commit
behavior here.
