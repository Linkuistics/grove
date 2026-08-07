# session-kind-tree-k23

**Kind:** impl

## Goal

Implement the current nineteen-kind filename grammar and format-aware
`grove-llm` tree interface while retaining only the bounded legacy path needed
by the later automatic migration.

## Context

- Binding design: `docs/specs/config-driven-sessions.md` sections "Session
  kinds live in filenames" and "Authoritative selection and mandate".
- Primary code surfaces: `src/leaf.rs`, `src/tree_id.rs`, `src/tree_read.rs`,
  `src/tree_grow.rs`, `src/tree_lifecycle.rs`, `src/tree_promotion.rs`,
  `src/llm_cli.rs`, and their focused tree-interface tests.
- The exact current marker is `.grove/FORMAT` containing `session-kinds-v1`.
  Legacy layouts and body markers are input only for
  `session-kind-migration-k27`.
- Preserve `Reviews` and `Integrates`; launch harness metadata and producer
  receipts are not composition relationships.

## Done when

- One shared session-kind type contains the nineteen configured kinds, and the
  leaf parser matches the longest known kind after an optional terminal infix
  while keeping `<slug>-k<key>` as identity.
- Every task-shaped current filename must carry a known kind; unknown or absent
  kinds fail visibly for live and terminal leaves, while foreign files remain
  ignored and node names remain kind-free.
- Current tree readers, pick/resolve/brief-chain, mutation verbs, promotion,
  and key allocation use the filename grammar and known format witness without
  reading `Kind`, `Harness`, or `Producer launch` from current task bodies.
- `grove-llm root-init` writes the current requirements filename and body plus
  `FORMAT` last; every grow verb writes filename kinds. Research pair
  construction has no harness flags and emits `research-a`, `research-b`,
  `combine-research`; finish is driver-reserved and skipped by pick while
  non-finish work remains.
- Tests cover longest matching, both terminal infixes, malformed names,
  finish eligibility/duplication/reservations, stable resolution, pair output,
  promotion relationships, and format-marker errors. The legacy adapter needed
  by migration remains isolated and explicitly tested.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Do not add automatic legacy migration or driver-side root/finish allocation
here. This slice owns deterministic current-format writing through the existing
agent-side tree interface so its readers never reject trees its own writers
produce.
