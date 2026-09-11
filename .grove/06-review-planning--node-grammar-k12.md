# node-grammar-k12

**Reviews:** node-grammar-k3

## Goal

Adversarially review the ordered implementation plan against the reviewed design
and the human's cutover requirements before any implementation starts.

## Context

- Find the producer commit by `node-grammar-k3`. Its artifact is the root brief
  and the six implementation bodies `distinguished-names-k6`, `node-files-k7`,
  `node-methodology-k8`, `node-documentation-k9`, `node-cutover-k10` and
  `grove-migration-k11`.
- Requirements are `plan-k1` and the root brief. The design at
  `node-grammar-k2` was reviewed at `node-grammar-k4` and integrated at
  `node-grammar-k5`. Use the current ADRs, specs and library model limits,
  including formalism finding 049; do not re-interview.
- The task's explicit scope keeps these working increments in one grove.
  `docs/RELEASING.md` supplies the release route, and the book manifests
  identify the source ownership that every source-changing leaf must honor.
- Parent discovery used targeted source reads after the graph CLI refused to
  start because of a generation conflict; there is no graph generation or
  coverage evidence to inherit. Re-establish graph availability or inspect
  source directly for claims about consumers.

## Review questions

- Does the library leaf deliver a usable, green API with every required consumer
  adaptation, while the Grove grammar leaf changes every coupled reader, writer,
  handle and fixture together? Is either too large for one focused session, and
  is there a smaller independently working boundary the plan missed?
- Are read-level validation and projected-final-level validation both covered,
  including generic duplicate refusal, root-specific policy, bare node creation,
  batches, optional promotion children and node rewrites? Do conformance samples
  have independent expected verdicts and tests for the model's stated gaps?
- Are the library/Grove content-write and recovery contracts preserved, including
  the accepted conditional interrupted-decompose advice and root classification
  after validation? Has the plan accidentally restored rejected machinery?
- Can each source leaf land with its affected books green, including adapters,
  reference-domain CLI changes, fixtures inside book roots, indexes and versioned
  manifest roots? Is the later documentation leaf confined to work that earlier
  green boundaries can actually leave for it?
- Is the human handoff executable when the installed old driver is already in
  memory, the release uses the default workspace and binaries are global? Can
  preparation, approval, stopping, publication, plugin refresh, this-tree rename,
  retirement and restart occur without a false completion or an incompatible
  relaunch? Are the other drivers kept stopped until their trees are verified?
- Does migration cover all known trees and newly discovered ones, preserve
  identity/content/order, handle interruptions and `FORMAT` without guessing,
  and verify actual installed-tree reads rather than only scratch fixtures?
  Is the no-product-migration constraint intact?

## Done when

- Findings are inspection-only, linked to the producer's commit and precise
  artifact coordinates, with a practical consequence and proposed correction.
- If actionable findings exist, insert `integrate-review-planning` using the
  bare slug `node-grammar` before the first following live sibling entry,
  presently `distinguished-names-k6`. Its body names this review's handle
  under `**Integrates:**`, rather than adopting a findings list as its charter.
  If there are no actionable findings, create no integration leaf.
- The review retires with implementation still after any integration it earned.

## Notes

The review consumes the plan, not unfinished implementation. No in-session
reviewer and no code changes are part of this leaf.
