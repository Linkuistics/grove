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
