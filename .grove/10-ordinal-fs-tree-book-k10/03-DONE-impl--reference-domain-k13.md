# reference-domain-k13

## Goal

Add the syllabus reference domain and conformance kit as a worked implementation
of the name seam that the remaining book can reuse without ambiguity.

## Context

- Inputs: `name-seam-k12`, `book-system-k6`, and this subtree's brief.
- Primary source emphasis: `src/reference.rs` and `src/conformance.rs`.

## Done when

- Labels, status attributes, lessons, modules, and overviews are introduced as
  one consumer's vocabulary and mapped precisely to the generic name algebra.
- Parsing, formatting, positioned species, distinguished names, and recovery
  advice are followed through representative valid, foreign, malformed, and
  reserved names.
- The conformance obligations explain what the library assumes, what the type
  system enforces, and what the reusable check can demonstrate.
- The worked examples are stable enough for later read, mutation, rollback, and
  CLI chapters to reuse without restating the entire domain.
- Assigned fragments tangle exactly and all scoped mechanical and relevant crate
  checks pass.

## Notes

Repeat the small part of the example needed by a later mechanism; use links only
for optional expansion or navigation.

## Decisions (running log)

Keep the slice as one leaf. The natural source seam between `reference.rs` and
`conformance.rs` is not an independently valid book increment: page 3 and the
`reference-domain-k13` scoped prefix become mechanically complete only when both
roots, the ownership ledger, and the page navigation land together. Partition
each root into smaller conceptual fragments inside the page instead.

Partition `reference.rs` into vocabulary, name/error representation, parsing,
trait methods, and parser helpers. Partition `conformance.rs` into obligations,
reporting, compose/canonical checks, component/distinguished checks, and
filesystem-species agreement. Each partition is line-aligned, gapless, and
keeps the exact source beside the concept its prose introduces.

Use the architecture document's syllabus tree as the stable worked corpus, then
add near-miss rows for the ownership boundary: a genuinely foreign README, an
empty owned label, alternate ordinal padding, an unknown status, a filesystem
species contradiction, and the reserved publishing witness. Later pages can
reuse the same lesson, module, overview, ordinal, and key without reintroducing
the complete grammar.
