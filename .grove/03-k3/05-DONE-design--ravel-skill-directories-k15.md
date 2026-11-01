# ravel-skill-directories-k15

## Goal

Reconcile Ravel's skill-delivery design with the human's corrected directory-
injection boundary before Grove adoption review and implementation planning.

## Context

Work in `/Users/antony/Development/AgenticOrchestration/Ravel` under its own
instructions. Read the human corrections recorded in grove-skill-delivery-k8
and the current root/parent briefs. The prior Ravel design and integration are
ravel-skill-discovery-k7/k14; their resulting Ravel commit is
`66aaec06f5c020215b9b0e4139c2ba2e0b969644`.

The human stated that Grove may name a skill, but the user supplies it within
each harness or through Ravel. Ravel does not resolve a requested skill: it
injects skill-directory paths into the prompt, and the agent finds and reads
applicable guidance. The prior generated skill catalog is not a settled contract
for implementation. Do not repeat the requirements interview.

Grove's corrected design is in `docs/specs/codex-skill-provisioning.md` and
`docs/specs/harness-selection-and-execution.md`; its evidence document records
the source audit and explicit upstream dependency. Grove has no skill registry,
location reader or delivery-route requirement. Native-only setup remains valid.

## Done when

- Ravel's current spec, ADR, glossary, evidence and visual set consistently
  describes configured directory injection with agent-owned skill discovery.
  Remove or rework the generated-catalog/name-resolution contract in place;
  leave no competing normative account.
- Preserve accepted local and exact pinned Git source acquisition, offline
  reuse without revision substitution, full resource reachability, reader
  lifetime, confinement, compaction and delegation requirements. Distinguish
  directory acquisition from resolving a requested skill.
- Settle how the injected instructions expose directory locations and support
  mandatory guidance and relevant-skill discovery without a skill lookup API.
  Define the disabled/no-source path, original task/ending preservation and
  owner policy interface, compatibility and actionable failure behavior.
- Reconcile any public setup/resolution operation with that boundary; do not
  carry forward catalog fields or duplicate-name admission merely because the
  prior proposal had them. Ravel remains generic and knows no Grove kind set.
- Specify source-directory/launch acceptance and actual harness observations,
  with native-only Grove compatibility independent of Ravel discovery.
- Record owning-repository commits and update the Grove adoption handoff if its
  stated session-interface assumptions need correction. Do not silently assume
  that Grove's existing caller accepts a new Ravel major or policy schema.

## Notes

Design only: no owner settings, installations, releases or product code changes.
Preserve this session's original ending command. Decide any earned Ravel design
review lazily and insert it before grove-skill-delivery-k16; that review consumes
the corrected upstream contract. Do not add an in-session reviewer beside a
scheduled review of this producer.

## Decisions (running log)

- Replace Ravel's skill-name and metadata catalog with configured search
  directories. Ravel validates and captures filesystem resources; the agent
  enumerates SKILL.md files, reads their descriptions and chooses applicable
  guidance. Empty directories and duplicate skill names are not launch failures.
- Mandatory guidance is an ordered list of source-relative file references,
  translated to absolute retained paths. It does not use skill identifiers or
  a name lookup. Explicit caller loading mechanisms remain binding.
- Retain immutable whole-source capture and exact pinned Git reuse. Publish a
  small directory descriptor for recovery and delegates, containing only source
  provenance, search directories and mandatory file paths; inject those paths
  and loading instructions directly into the prompt. No generated entry.md or
  skill enumeration exists. This costs agent-side discovery and ambiguity handling.
- Keep the existing Ravel 2/schema-3 migration decision because prompt semantics
  still change. Grove's native-only Ravel 1.1 path stays independent; Ravel 2
  admission and explicit policy examples require their own caller acceptance.
- The corrected configuration and recovery contract is load-bearing for later
  implementation. Schedule a review-design leaf before Grove's adoption review;
  do not use an in-session reviewer alongside that scheduled review.

## Delivered artifacts and verification

- Ravel commit `4c16f8e09d58c8f649a375029c8e8fa2933de2b4` rewrites the owning
  skill spec, aligns the session spec and two ADRs, replaces glossary catalog
  terms, and updates the evidence and three editable visual views. The Ravel
  working copy was sealed onto a fresh empty change.
- The Grove delivery spec, evidence and parent handoff now consume the corrected
  contract. No session-interface assumption changed: native-only 1.1 remains
  supported in the adoption design, while 2.x admission, schema-3 examples and
  actual caller acceptance remain implementation obligations.
- Verify-tier source evidence used Ravel generation `2026-11-01T07:59:38Z` and
  Grove generation `2026-11-01T11:49:04Z`. Exact preparation/request/version
  snippets and direct launch/worker reads support the material claims. The
  complete preparation trace contained heuristic edges, which were not relied
  on. Source coverage reported matching metadata with no recorded gaps;
  excluded Ravel docs and changed glossary/Taskfile were read directly.
- Document validation passed 72 local links, three diagram entries and five
  byte-exact served resources. Missing-file, missing-anchor, unknown-diagram and
  obsolete-interface controls failed as intended. All 54 subjects read by that
  run retained their SHA-256 digests. This measures documents, not product behavior.
- `mise exec -- task --list` parsed the existing tasks; `task docs:skills`
  served the preview on port 18776. Dia inspection checked the outline, current
  discussion and changed markers, rendered all three diagrams, and verified
  preparation/lifetime deep links. A sequence-label semicolon caused a parser
  error; replacing the separator and wrapping labels rendered successfully.
  Mobile and dark appearance were not measured.
- No product source, owner settings, native installation or release changed.
  No proposed acquisition, confinement, compatibility or actual-harness test is
  claimed. The independent ravel-skill-directories-k17 review runs next, before
  Grove adoption review.
  No in-session reviewer was used and current provider evidence was absent.
- No parent closes: review, planning and implementation/acceptance remain live.
