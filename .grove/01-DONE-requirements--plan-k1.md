# plan-k1

## Goal

Establish the reader, scope, form, editorial contract, completeness criterion,
verification seams, and reusable-skill requirement for the ordinal filesystem
tree code-walkthrough book.

## Context

The bootstrap brief was empty. The human supplied and approved the requirements
through a one-question-at-a-time requirements session.

## Done when

- The book's intended reader and learning outcome are explicit.
- The reproduced source corpus and exclusions are explicit.
- The literate fragment notation and exact-source criterion are explicit.
- The editorial constraints and allowed research scope are explicit.
- The reusable skill's purpose, scope elicitation, destination, and validation
  obligations are explicit.
- The shared test seams are agreed with the human and promoted to the root
  brief.
- A fresh planning leaf exists for decomposition.

## Notes

This is a large workstream: the production library and demonstration CLI exceed
six thousand lines before explanatory prose, and the generic skill is a second
validated artifact. Requirements therefore stop at the agreement and hand the
decomposition to a fresh planning session.

## Decisions (running log)

The deliverable is a multi-page Markdown book for developers who already
understand Rust, common crates, and standard operating-system APIs. It applies
an explain-like-I-am-ten sequencing philosophy to the crate's own concepts, not
to the reader's general technical competence.

The book covers the production `ordinal-fs-tree` library, its demonstration CLI,
and the crate manifest. Test files, test-only fixtures, and Alloy and Quint model
source are excluded from reproduced code, while their findings may support the
explanation.

The exposition follows the order that best builds understanding rather than
source-file order. It starts from purpose and public behavior, follows a complete
operation through the system, and then expands the participating layers and the
CLI.

Every code fragment has a unique `«fragment-id»`. A fragment may contain a
`«fragment-id»` reference at an insertion point for another fragment. Root
fragments identify source files, and recursive expansion must reconstruct every
in-scope source file exactly with no missing, duplicated, unresolved, or
unreachable fragment.

The prose is declarative, direct, and non-emotive. It avoids rhetorical
questions, narrative suspense, metaphors, idioms, and persuasive framing. It
explains purpose, architecture, API behavior, implementation, invariants,
failure behavior, and design trade-offs through concrete examples and explicit
relationships.

The book is self-contained. Existing ADRs, tests, formal models, repository
documentation, and external sources are author evidence, never prerequisites
the reader must follow to understand the text. Every required fact is restated
inside the book.

Internal cross-references are reserved for navigation and optional depth. A
short fact needed to combine the current explanation with the current code is
repeated at the point of use; a link does not replace it. Repetition is omitted
when the sources are independently intelligible and the repeated material adds
no explanatory value. This applies the split-attention and redundancy evidence
by testing the logical relationship between the passages rather than imposing a
fixed preference for either linking or duplication.

External research is limited to how to write and communicate: literate
exposition, worked examples, cognitive-load management, progressive disclosure,
and precise technical style. Technical claims derive from the repository, its
tests and models, and authoritative dependency documentation when required.

The final instructions that prove effective become a generic deployable skill,
not an `ordinal-fs-tree`-specific procedure. The skill starts by eliciting the
target codebase, included source, audience, depth, output form, style, and
verification requirements one question at a time.

The skill ships at
`plugins/linkuistics/skills/writing-code-walkthroughs` through the repository's
existing Linkuistics plugin mechanism and follows that marketplace's authoring
conventions.

The agreed seams are exact-source tangling, complete source-to-fragment
coverage, Markdown and local-link validation, the existing crate verification,
technical review against source/tests/models, editorial review against the prose
contract, and baseline plus skill-enabled behavioral scenarios for the skill.
