# figure-contract-k70

**Integrates:** figure-contract-k69

## Goal

Triage the design review of `figure-contract-k18` and apply every finding that
survives scrutiny before the publishing-pipeline kinds are designed against the
figure contract.

## Context

- Read `figure-contract-k69` from its committed review task; its findings and
  citations are the handoff, and are not restated here.
- The reviewed artifacts are `docs/adr/a-book-carries-no-asset.md` and the
  figure/asset clauses in `docs/specs/walkthrough-books.md`.
- `pipeline-kinds-k27` and `pipeline-skills-k28` are direct consumers of the
  settled contract and now follow this leaf in the walk.

## Done when

- Every review finding is classified against the current artifact as accepted,
  rejected, or already resolved, with the reasoning recorded in this task's
  running decision log.
- Every accepted finding is integrated into the minimum coherent ADR/spec set
  and any directly affected book artifact, without rewriting frozen measurement
  evidence to make the decision easier.
- The resulting contract is precise enough for the art kind to consume without
  inventing its own figure class or asset policy.
- `bash scripts/check.sh` passes.

## Notes

This leaf was inserted at the first live sibling after the review so no consumer
can silently move its path-and-line citations before triage.
