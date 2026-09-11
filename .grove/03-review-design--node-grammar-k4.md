# node-grammar-k4

## Goal

Adversarially review the node grammar design as a contract for both Grove and
`ordinal-fs-tree`, before `node-grammar-k3` cuts implementation work.

**Reviews:** node-grammar-k2

## Context

- Requirements: `plan-k1` and the root brief. The interview is complete; do not
  re-interview. The design's commit is found by its stable handle.
- The artifact is the producer's diff: the canonical naming and seam ADRs,
  iteration and root-lifecycle reasoning, both architectures, the design specs,
  both glossaries and context map, the structural and operational models and
  the Quint runner. `docs/formalism-findings.md` entry 049 records the modelling
  evidence and limits.
- No file under `crates/` is part of this design change. Review the proposed
  contract, using source to test whether the handoff accounts for its consumers.

## Review questions

- Can a consumer implement the name-and-bytes input and
  `validate_distinguished(root-or-node, names)` through one name seam, with
  useful errors for missing, competing and misplaced files? Does the library's
  independent at-most-one check preserve its obligation even for a permissive
  consumer?
- Is every way to create or change a level covered before effects, including
  append, batches, insert, initialization entries, promotion's optional first
  child and rewritten node parts? Are whole-tree snapshot validation, interrupted
  promotion and rollback consequences stated consistently?
- Does comparing canonical renderings for distinguished-name identity preserve
  the name laws without teaching the library labels? Can Grove form a node
  handle solely from the actual parsed directory and node-file names, without
  duplicating the slug into node parts or reading content?
- Do the models exercise acceptance as well as refusal, distinguish different
  distinguished filenames, and avoid passing claims only by assuming their
  conclusions? Check the runner's complete claim inventory and the stated limits
  of random simulation and of the root-versus-node policy instances.
- Is the ADR/spec set coherent and current, with no naming history, stale handle
  substring claim, optional Grove node file, changed glossary anchor or design
  requirement that planning cannot consume?

## Done when

- Findings are inspection-only, tied to the producer's commit and current
  artifact coordinates, with their contract and practical consequence stated.
- If actionable findings exist, insert an `integrate-review-design` leaf using
  the bare slug `node-grammar` ahead of `node-grammar-k3`. Its body points to this
  review's handle, rather than adopting the findings as its charter. If there
  are no actionable findings, create no integration leaf.
- Planning still follows the review and any integration it earns.
