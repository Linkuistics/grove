# modular-representation-k14 — brief


## Goal
Remove literal route and inspection compatibility representation, then finish
the parent cleanup contract and repository-wide configuration-forms audit.



## Context
The parser child removes flat validation and scanning. CapturedDocument.templates
is initialized empty solely for the old resolver; remove that field and all its
consumers here. The parent brief identifies the remaining public types and books.

## Done when
- Delete CapturedDocument.templates, literal Route variants and patches,
  parameter resets and fallback inspection word reconstruction.
- Delete AssignmentValue::LiteralTemplate/Reset; make successful CommandView
  command/binding fields required, updating consumers, tests and public specs.
- Reconcile both source-exact books and remaining current prose, enumerate and
  classify repository-wide old configuration forms, and pass scripts/check.sh.
- Decide subsystem review under Grove's threshold and close ancestors only when
  every criterion is met.

## Notes
Retain the settled public load/expansion seam; no new CLI acceptance suite.
The parser increment updates the capture and scanner explanation; remaining
literal-resolver descriptions and old examples elsewhere still belong here.

Parser handoff: Catalog::load still filters `invalid_template` diagnostics from
its capture failures, a remnant of eager flat validation. Capture now emits only
structural diagnostics; remove that dead phase filter when reconciling remaining
compatibility code and its book fragment. The module-decomposition public type
spec and keyed-launch structure brief also need their final current-state sweep.
The parser increment's complete repository check passed with all six books green.

## Decomposition
modular-types-k15 removes the representation and reconciles its immediate public
and source-exact consumers. modular-audit-k16 finishes the repository-wide forms
and prose audit and decides review of the completed subsystem.
