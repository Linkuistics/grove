# walkthrough-books-spec-k47

**Integrates:** walkthrough-books-spec-k46

## Goal

Triage the adversarial review of the walkthrough-book system design and rework
the agreement point wherever the findings survive, before either validator leaf
implements against it.

## Context

- Read `walkthrough-books-spec-k46` from its committed review task; its findings
  are the input to triage, not obligations copied into this charter.
- The producer is `walkthrough-books-spec-k20`; its durable artifacts are
  `docs/specs/walkthrough-books.md` and
  `docs/adr/a-book-cannot-witness-its-own-corpus.md`, and its running log records
  the decisions the review attacked.
- This leaf is inserted immediately ahead of `validator-structure-k21`; preserve
  that ordering so neither validator implementation lands against a design still
  under integration.

## Done when

- Every review finding is classified against current source and requirements as
  accepted, narrowed, or rejected, with the evidence and resulting decision
  recorded in this task's running log.
- Every surviving design correction is applied to the current spec and ADR set,
  and any newly explicit work is placed on the live leaf that owns it.
- The resulting manifest and assurance contract can be implemented by
  `validator-structure-k21` and `validator-fragments-k22` without inventing a
  policy or weakening the root brief's exact-corpus requirement.
- Repository documentation checks pass.

## Notes

This is design integration, not validator implementation. Do not absorb either
consumer leaf or edit the frozen walkthrough source corpus.
