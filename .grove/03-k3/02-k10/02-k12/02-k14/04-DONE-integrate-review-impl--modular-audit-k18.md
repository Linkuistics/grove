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

## Decisions (running log)

- Read findings from review commit `9b93069b`. Tier 2 graph discovery used
  generation `2026-09-17T05:07:07Z`; coverage reports changed metadata for the
  resolver and consumers and excludes docs. Current source reads replace stale
  symbol ranges and inferred generic-call edges as behavioral evidence.
- Findings 1 and 2 are real contract defects: capture rejects non-wrapper
  declarations, and resolution compiles only definitions reached by effective
  bindings. Remove eager-legacy and flat-namespace claims and the obsolete
  compatibility hedge; preserve local-only non-admission behavior.
- Finding 3 is real: AssignmentValue has only Set and Unset. Correct the
  overview's exhaustive list to match its fragment.
- Finding 4 is real: both documents receive structural checks; active composition
  receives semantic checks. Correct the book's validation summary and stale line
  reference.
- Finding 5 is real: named::resolve copies the primary definition's source into
  every Template. Local route/binding/parameter origins belong to inspection.
  Correct driver and personal_path comments, their fragments, surrounding prose
  and the reference; retain the personal fallback for an unresolved kind.
- Finding 6 is real: require_declared loads and resolves composition before
  checking presence; inactive definitions are not eagerly compiled. Correct both
  the comment and the books' whole-file/eager claims without changing load points.
- Finding 7 is real: duplicate() reports a duplicate declaration and retains the
  second occurrence as a related span. Correct the rendered example and explain
  the structured location rather than claiming the human message lists both.
- Finding 8 is real: the token controls do not establish prose correctness.
  Enumerate configuration-bearing prose paragraphs and comments separately,
  classify their claims, and exercise controls against known stale paragraphs.
- Finding 9's role branch is real dead compatibility code: definitions are
  primary-only, so runtime errors carry Primary directly. Keep Templates.overlay
  because unresolved() still uses it for local-only-key diagnostics. Accept the
  older captured bytes/document retention visibly: removing that storage is not
  needed for the modular-only contract. Correct its misleading provenance comment
  to state retention and that resolution uses captured declarations.
- The prose enumeration also found the same validation-timing drift in the
  keyed-launch structure brief and grove-loop chapter 18's Vocabulary explanation.
  Corrected both, and clarified expand's comment to say resolution checks the
  command. Updated chapter 10's unvalidated source-range diagram alongside the
  keyed-launch manifest, fragment ranges, index and roll-ups after deleting five
  lines; the final book validator passes at 3,710 lines.
- `bash scripts/check.sh` passed all eight principal checks, including locked
  workspace tests and all six `book-check --final --check all` runs. Existing
  tests for dormant versus active definitions, command-source versus route-origin
  provenance, and contextualized runtime errors passed. All 1,802 tracked subjects
  (sources, fixtures, manifests, scripts, books and notes) matched their individual
  pre-run SHA-256 digests after the check. No new acceptance suite was added.
- Rechecked the ancestor Done when conditions against the cumulative removal,
  review findings and this verification. modular-representation-k14,
  modular-cleanup-k12, modular-loader-k10 and modular-only-k3 are satisfied:
  compatibility machinery is removed, modular behavior and rejection evidence
  remain covered, examples and durable contracts are reconciled, the prose/forms
  audit is corrected, and the scheduled subsystem review has been integrated.
  The root brief's remaining conditions also hold. No missing work or new design
  decision requires another leaf or ADR; durable material is already in the
  existing spec, reference and audit. No further brief promotion is needed.
