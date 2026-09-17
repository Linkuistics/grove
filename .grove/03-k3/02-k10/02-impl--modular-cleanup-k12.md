# modular-cleanup-k12

## Goal

Remove the internal compatibility machinery made unreachable by modular-input-k11
and complete the modular-loader-k10 and root acceptance conditions.

## Context

The preceding child rejects unsupported top-level input before legacy validation.
Remove that temporary boundary's dead downstream machinery, without changing
modular resolution, authority, parameters, provenance or expansion semantics.

## Done when

- Delete the flat parser and legacy template scanner in templates.rs, keeping
  the shell-comment scanner used by modular compilation.
- Remove CapturedDocument.templates, literal Route variants, literal patches,
  parameter resets and fallback inspection word reconstruction in named.rs.
- Remove AssignmentValue::LiteralTemplate/Reset and make successful CommandView
  binding/command fields required; reconcile public docs, renderers and tests.
- Both source-exact books explain and reconstruct the final source. Finish the
  durable documentation and repository-wide configuration-forms audit, classifying
  every old-form survivor as rejection or historical evidence.
- `bash scripts/check.sh` passes; decide review for the completed subsystem under
  Grove's ordinary threshold and close the parent chain only when its criteria hold.

## Notes

Keep the public load/expansion test seam; no new CLI acceptance suite. The root
handoff and this node's preceding child record the migrated compatibility cases.

## Handoff from modular-input-k11

- The new public boundary is in `parse_and_validate`: every node not recognized
  as a wrapper is rejected before `validate_document`. Fold rejection into the
  modular parser while deleting flat validation; retain exact spans and remedy.
- Core reference/spec/ADR/glossary language and the release smoke recipe already
  describe modular input. The public module spec still faithfully lists the old
  enum variants, annotated unreachable; delete those with the actual types.
- The runner book's new capture-boundary paragraph labels downstream flat code
  unreachable, but its surrounding old examples/narrative still require the
  final reconciliation. Include all chapters, source comments in keyed-launch
  lib.rs, overview chapters 1/5, and the Architecture configuration-delta section.
  These are known remaining examples, not an exhaustive forms audit.
- Exact-source line ranges, manifest block ranges, source-index tables and
  chapter 10 roll-ups must move together. `book-check --final --check all`
  diagnoses each surface independently.
