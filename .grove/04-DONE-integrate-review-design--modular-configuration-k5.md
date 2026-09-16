# modular-configuration-k5

**Integrates:** modular-configuration-k3

## Goal

Triage the findings of the `review-design` leaf `modular-configuration-k3`
against the modular configuration design produced by
`modular-configuration-k2`, and apply the ones that are real. Reject a finding
on its merits where the design already answers it; record why in the running
log. Leave the design, ADR set, glossary, examples and visual document as one
coherent contract for the planning leaf that follows.

## Context

Read the review's findings from its own commit, located by its handle
`modular-configuration-k3`; this body deliberately carries the handle and not
the finding list, so that rejecting a finding is not rejecting this charter.
The producer's artifact is `docs/specs/modular-configuration.md` with the
runner section of `docs/specs/module-decomposition.md`, the two configuration
ADRs, the glossary entries, the example set under
`docs/examples/modular-configuration/` and the visual document under
`docs/design/modular-configuration/`.

The root brief is the requirements contract. Several findings concern where a
contract is silent or where a seam sits; settling those may mean a spec edit, an
ADR rework in place, or a recorded rejection, and any of the three is a valid
outcome. The user has already approved reusing the existing jj trackedness
check for inspection; that is not open.

## Done when

- Every finding in the review's commit has a disposition in this leaf's
  running log: applied, applied differently, or rejected with the reason.
- Applied changes land in the durable artifacts, not in this task file, and the
  spec, ADRs, glossary, examples and diagram sources still describe one
  contract with no dangling citation.
- Any change to the module interface is reflected in both specs at the same
  grain, so the planning leaf can name test seams from signatures.
- Substantial redesign, if any finding demands it, is externalised as a new
  design review chain rather than done here.

## Notes

The design session recorded that no in-session reviewer was needed; this leaf
may spend one narrow reviewer on a single contested finding if the spec's
answer is genuinely unclear, per the spine's integration allowance. Do not
launch a local browser for the visual document; if a rendered check is needed,
drive it through the testanyware VM skill.

## Decisions (running log)

- Findings read from review commit `bc2a01647267`, not inferred from this leaf.
  Tier 2 graph evidence used generation `2026-09-16T11:54:39Z`; documentation
  is excluded, so the spec, ADRs, examples and glossary were read directly.
- **1 — applied, unclear contract.** An active personal parameter-only route
  without a personal target fails the whole resolution with `missing_target`.
  Local policy cannot repair it. This follows the selected-combination
  completeness requirement; local-only keys retain their non-blocking behavior.
- **2 — applied, real issue.** Specify Catalog loading, explicit selection,
  resolution, inspection/provenance and diagnostics at the existing module
  contract's signature grain. Keep resolver internals opaque; conformance takes
  the same Catalog and explicit selection. No new resolver seam is needed.
- **3 — applied differently, real interface duplication.** Keep the existing
  load signature as a convenience delegating to Catalog with an empty selection;
  accept wrappers and use one vocabulary rule. The graph also finds the live
  `SessionConfig::read` caller, omitted by the review's caller claim, but that
  adapter already migrates to Catalog. This does not justify a second parser.
  Grove's existing slot vocabulary is unaffected by reserving `param.` uniformly.
- **4 — applied, real issue.** Put operator commands at `grove config show` and
  `grove config examples`, preserving the architecture's human/agent split.
  Inspection still calls the same SessionConfig adapter and approved VCS check.
- **5 — applied, real documentation gap.** Record occurrence-based composition
  and parameter specificity with their rejected alternatives in the existing
  complete-session-configuration ADR. Remove fold detail from the delta ADR.
- **6 — applied, real packaging issue.** Make packaged instructions timeless and
  self-contained; keep delivery status in the spec, outside the installed bytes.
  The implementation must validate and install these exact repository files.
- **7 — applied, real example issue.** Use `daily` alone for the local override
  sample: removing proof's high override exposes medium, while impl becomes low.
- **8 — applied, missing terms.** Add Command definition, Kind route and Command
  parameter to the Grove glossary, distinguishing routes from driver Kind routing.

These are bounded repairs of the reviewed contract; none requires a new producer
chain or changes the approved requirements or test seams.

- Narrow doubt check: the new output records must represent the promised
  provenance without rereading changed files. A fresh reviewer inspected only
  the interface against that contract and found one representation ambiguity:
  binding targets and legacy literal targets shared `Set(String)`. Applied the
  finding as a tagged `LiteralTemplate` variant and matching JSON tag. This is
  a mechanical distinction between existing target forms, not new semantics;
  no second reviewer or redesign is needed.
- Reconciled the live planning leaf's concrete CLI and conformance pointers with
  the repaired design. Terminal producer/review leaves retain their historical
  wording; the next planner consumes the current contract.

## Validation

- `bash scripts/check.sh`: all eight principal checks passed, including workspace
  tests and final validation of all six source-exact books. SHA-256 digests of all
  1,776 tracked inputs matched before and after the run. Subsequent edits only
  clarify prose and the planning handoff; the focused document checks cover them.
- Relative file links and explicit glossary anchors in the edited documents
  were checked; the visual manifest's diagram sources exist. The changed diagram
  labels were checked against the spec; no browser/rendered check was performed.
- The examples remain design artifacts. Their production-reader and fake-argv
  acceptance belongs to implementation and is not claimed by this session.
