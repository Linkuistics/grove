# consumer-fixtures-k5 — brief

## Goal

Migrate retained-behavior configuration fixtures in Grove consumers to modular
form while production still accepts both forms. The generic keyed-launch batch
is `runner-fixtures-k4`; the parent brief owns the full migration contract.

## Scope

Inspect direct and helper-generated fixtures in grove-loop SessionConfig and
driver-lease tests, grove CLI config/loop/lifecycle tests, grove-llm kind-admission
and shared helpers, and grove-tui witnessed tests. Enumerate other consumers too.
Local files cannot define commands: put alternatives in personal policy and
redirect local bindings/routes. Preserve kind admission and semantic-error
assertions; escape literal dollars for the named scanner. Leave compatibility-
specific cases identified for `modular-only-k3`.

## Done when

- Retained consumer tests use modular fixtures and retain their purpose.
- Load/expansion tests and downstream tests pass; `bash scripts/check.sh` passes.
- Source-exact walkthroughs reflect any covered inline helper changes.

## Notes

Keep fake executables and the cargo signal guard. Do not run real agents.
Production removal and durable language-documentation changes belong to k3.

## Decomposition

- `loop-fixtures-k6`: grove-loop SessionConfig, driver-lease and inline launch fixtures.
- `cli-fixtures-k7`: grove CLI config inspection, loop and lifecycle fixtures.
- `admission-fixtures-k8`: grove-llm admission/shared helpers, grove-tui witnessed
  fixtures, and the final remaining-consumer inventory.

## Decisions (running log)

The consumer inventory spans separate configuration, lifecycle and admission
suites. Split along crate boundaries into independently green migrations; this
session completes only the first child, with production removal still in k3.

## Compatibility handoff

`loop-fixtures-k6` leaves two explicitly named `legacy_delta_*` cases in
`crates/grove-loop/tests/session_config.rs` for k3: flat-only shape aggregation
with eager-template validation, and eager validation of local flat templates.
Retained loop configuration/lease fixtures use modular declarations. Carry this
handoff upward when closing this node so the removal leaf can find it.
