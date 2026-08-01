# doubt-grove-implementation-k7 — brief

**Kind:** impl

## Goal

Implement the reviewed doubt/Grove composition design as a sequence of green,
independently testable slices across task-tree mutation, producer handoff,
review launch routing, and the canonical guidance surfaces.

## Context

Bootstrap from the integrated design artifact and root brief. Canonical Grove
skill content lives under `content/`; the doubt skill lives under
`plugins/linkuistics/skills/doubt-driven-development/`. Use the codebase graph
for Rust discovery and preserve jj/git symmetry.

## Done when

- Atomic promotion and advisory target-diversity warning match the reviewed
  design, including help/error text and all-or-nothing behavior.
- Grove and doubt-driven guidance implement the complete kind matrix, one-review
  budget, escalation behavior, routing ownership, and outside-Grove compatibility.
- Focused and full relevant test suites pass, including formatting and lints.
- Architecture, glossary, usage/help, and generated/canonical content are
  reconciled without leaving duplicate authorities.

## Notes

The implementation proved larger than one focused session after bootstrap. Its
ordered children preserve the design's dependency direction: promotion first,
then the receipt written during producer handoff, then the warning consumed at
review launch, then a final canonical-guidance reconciliation.
