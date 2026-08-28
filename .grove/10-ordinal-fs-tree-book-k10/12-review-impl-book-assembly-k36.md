# book-assembly-k36

**Reviews:** book-assembly-k35

## Goal

Perform a fresh-context editorial review of the complete ordinal filesystem tree book. Review the whole assembled book, not only the producer diff.

## Context

Read the complete `docs/ordinal-fs-tree/book/` in order, together with the root brief, the `book-assembly-k10` node brief, and `docs/specs/ordinal-fs-tree-book.md`. The committed producer under review is `book-assembly-k35`.

This is an editorial review. Do not edit the book or implementation while reviewing it.

## Done when

- The whole book has been assessed for clarity, concept order, cognitive load, self-containedness, repetition, and cross-reference choices.
- The review checks that the prose remains declarative and satisfies the book specification's prose contract.
- Findings identify precise locations and actionable corrections; do not run implementation or model checks for this review.
- If there are substantive findings, append an adjacent `integrate-review-impl` leaf with the same bare stem, set `**Integrates:** book-assembly-k36`, and require the integrator to rerun the complete book, crate, Quint, and Alloy checks after making corrections.
- If there are no substantive findings, retire this leaf without creating an integration leaf.

## Notes

Approach the material as a reader encountering the assembled explanation for the first time. Do not limit the review to changes made by `book-assembly-k35`.
