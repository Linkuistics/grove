# implementation-slices-k10

**Kind:** planning

## Goal

Turn the integrated configuration-driven session design into ordered, vertical
implementation leaves that each land green and demonstrate useful behavior.


## Context

- Plan from the reviewed `docs/specs/config-driven-sessions.md` and reconciled
  ADR set, not from the pre-design assumptions in this initial brief.
- Use review chains for load-bearing implementation artifacts; keep ordinary
  mechanical slices as lone leaves when review would add no value.

## Done when

- The root tree contains the full implementation sequence, including migration,
  deletion of obsolete routing/receipt surfaces, docs, and verification.
- Each slice crosses the agreed process or tree seam, has concrete acceptance
  criteria, and can pass independently without waiting for a later sibling.
- Ordering and dependencies are explicit in the tree; no implementation is
  performed in this planning session.

## Notes
