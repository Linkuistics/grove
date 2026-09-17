# modular-audit-k18

**Integrates:** modular-audit-k17

## Goal
Triage the nine findings of modular-audit-k17 against the current source and
contract, apply the real ones, and leave the modular-only subsystem's durable
documents and source-exact books stating only current behaviour.

## Context
Read the findings from the review's commit (the review leaf's `## Findings`
section). They are anchored to commit `0ba4869c6233` line numbers. Eight are
prose or contract statements that describe retired behaviour: eager legacy
template checking, flat-entry namespaces, the deleted `LiteralTemplate`/`Reset`
variants, a delta-supplied command source, whole-file template reading, and a
retired duplicate-report rendering. They sit in
`docs/specs/modular-configuration.md`, `docs/CONFIGURATION.md`,
`docs/configuration-forms-audit.md`, a `loop_driver.rs` comment, a
`grove-llm` `cli.rs` doc comment, and the overview, grove-loop, grove-llm and
keyed-launch books. One is a dead compatibility branch in `Templates::expand`.
Source comments are reproduced as exact fragments, so a comment edit moves its
book's fragment ranges; keep line counts stable where the k16 leaf did, or
update the ranges, ownership tables and roll-ups together and rerun
`book-check --final --check all` for each affected book.

## Done when
- Every finding is classified (contract unclear, real, visible trade-off, or
  noise) in this leaf's running log, with the reason.
- The spec, reference, audit document, both source comments and the four books
  no longer assert eager legacy checking, flat namespaces, deleted variants,
  delta-supplied command sources or the retired duplicate rendering.
- The forms audit records how old prose claims were enumerated and controlled,
  not only quoted tokens.
- `bash scripts/check.sh` passes, including all six books.
- Ancestors close only when modular-representation-k14, modular-cleanup-k12,
  modular-loader-k10 and modular-only-k3 `Done when` conditions all hold.

## Notes
No runtime behaviour change is required by any finding; keep the public
load/expansion test seam and add no CLI acceptance suite. Finding 9's older
`_source`/`_document` retention may be accepted visibly. A finding you cannot
derive from the source is noise; say so rather than editing around it.
