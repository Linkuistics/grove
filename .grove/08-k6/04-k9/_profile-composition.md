# profile-composition-k9 — brief


## Goal

Complete the generic configuration language: resolve explicit profile selections
and included occurrences into reusable commands, with active-only semantic
validation and full provenance. Generic consumers can keep incomplete inactive
experiments beside a working selection.



## Context

Build on `reusable-commands-k8` and finish the complete Catalog contract. The
next root leaf owns choosing local versus personal defaults. Catalog exposes
those declarations and obeys only the Selection the consumer supplies. This
leaf also guards Grove's interim adapter so accepting the grammar cannot make
a declared selection disappear through the empty-selection convenience.

## Done when

- The full wrapper grammar and document-wide structural checks work, including
  inactive profiles. Optional captured selections distinguish absence from a
  present empty list and carry real declaration origins. Selection/include
  lists may repeat names; declarations and patch nodes may not duplicate keys.
- Resolve personal base, depth-first includes left to right, each profile's own
  patch, then overlay. Reapply every selected/include occurrence, including
  diamonds. Detect cycles on the active recursion stack and report the closed
  chain with include spans; an unknown reachable include or cycle cannot be
  hidden by a later override.
- Final values follow within-setting chronological precedence and across-scope
  parameter specificity. Rebinding, literal transitions, `unset`, overwritten
  invalid references and split route/binding/value profiles follow the spec.
- Unselected references, includes, missing parameters and unused command
  templates do not block success. Surviving selected errors fail the whole
  resolution. Every effective binding/template and shared values assignment is
  checked even without a route; parameter completeness is per admitted route.
  Legacy flat entries retain eager validation, even when overwritten.
- Active personal target authority is captured before overlay. A target only
  in an inactive profile does not admit a local key; selecting that personal
  profile does. Parameter-only personal routes without a target fail globally,
  with `missing_target` and their occurrence chain; no inspection snapshot is
  returned. Local-only keys remain nonblocking and non-admitted.
- Occurrence IDs, parent links, selection indices, application order, source
  spans, histories and contributing word origins faithfully distinguish repeats,
  removals and overrides. Success and error ordering is deterministic. External
  selections have no invented declaration span.
- `Templates::load` ignores captured selection declarations and resolves an
  explicit empty list through Catalog. Conformance uses exactly the supplied
  Catalog/Selection, reports active failures and empty admitted sets, and never
  rereads edited source files. A non-Grove test exercises profiles with its own
  key/slot vocabulary and explicitly selected policy.
- At the Grove SessionConfig seam, replace convenience loading with Catalog
  capture after the existing source admission. Before resolution, refuse if
  either source has a selection declaration, including an explicit empty one,
  with that declaration's source and a clear selection-not-yet-supported remedy.
  With neither declaration, resolve an explicit empty list; inactive profile
  definitions may coexist with valid base commands. Never change the generic
  convenience contract or add another parser to implement this temporary guard.
- Grove regression cases use valid base routes plus personal and local
  declarations whose selection would change argv. Through both mutating verbs
  and launch, each refuses before tree mutation or child spawn, including empty
  declarations. A no-declaration file with inactive profiles still launches its
  base command. Keep all existing source-discovery protections in force.

## Verification and documentation

Drive the approved public configuration seam with small discriminating source
fixtures: include diamonds versus deduplication, late shared values versus a
route exception, inactive versus selected failures, explicit empty selection,
and active personal patches versus local-only routes. Retain exact argv and
legacy tests from earlier children. Validate repository example combinations
through Catalog where useful, while leaving actual packaging/delivery ownership
with `configuration-examples-k12`.

Finish the generic engine's source-exact book, crate docs and module-interface
documentation, plus the grove-loop book and any other corpus changed by the
adapter guard. Update `docs/CONFIGURATION.md`, usage and glossary to distinguish
generic explicit selection from Grove's temporary declaration refusal and
no-selection base behavior. Remove pending-generic-engine notices once true;
keep Grove selection/CLI/example delivery explicitly pending. `k10` removes the
guard and its temporary documentation when actual selection works. Run
common checks and close the configuration-engine brief against its Done when.

## Decomposition

- `selection-declarations-k25` captures selection declarations and lands Grove's
  temporary guard, while profile syntax remains explicitly refused.
- `profile-occurrences-k26` completes profile composition and the remaining
  acceptance cases, including meaningful inactive/selected profile fixtures.

The declaration/consumer boundary is independently usable through Catalog and
keeps Grove fail-closed before profile syntax can be accepted.

## Constraints carried forward

Do not implement a global default-selection policy in Catalog. Do not suppress
an active error because a particular caller will request a different key. The
whole resolved selection must be sound before Templates exists.
