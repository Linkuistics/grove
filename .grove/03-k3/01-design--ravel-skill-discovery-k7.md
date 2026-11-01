# ravel-skill-discovery-k7

## Goal

Specify Ravel's generic skill source resolution and automatic loading contract,
so implementation can proceed without inventing public behavior.

## Context

Work in `/Users/antony/Development/AgenticOrchestration/Ravel` under its own
instructions and VCS discipline. The durable artifact is an updated
`docs/specs/session-execution.md` plus a focused skill-discovery specification
if it earns a separate home. Record only decisions meeting the owning ADR bar.
The root brief supplies accepted requirements; do not repeat the interview.

The current specification promises unchanged prompts and no injected
instructions. Source verified in skill-discovery-k2 places request preparation
before policy selection; the worker freezes that request before context loading.
Recheck source and coverage before choosing the implementation interface.

## Done when

- Specify the owner configuration interface, source identity, discovery roots,
  metadata validation, mandatory guidance, no-source behavior and fresh bootstrap
  when a configured source is absent. Keep skill configuration separate from
  launch policy and require no per-policy augmentation helper.
- Specify local-source freshness and exact pinned Git revision resolution,
  cache identity/publication, concurrent readers, offline reuse, missing-revision
  failures and remedies. Never substitute a revision. Settle resource and catalog
  lifetime across session/delegate use, source updates and cleanup.
- Specify catalog names, trigger descriptions and absolute locations; mandatory
  guidance loads first and other skills load when relevant. Settle duplicate
  configured/native copies, ambiguity, relative supporting resources and
  compaction/delegate re-entry behavior.
- Specify original versus prepared prompt visibility in public request/context
  and policy APIs, including empty/no-source cases, limits and diagnostics.
  Preserve caller task bytes and exact ending commands intact in their separate
  block. Automatic preparation precedes policy command construction.
- Specify actual read access under confinement and delegation, including nested
  sessions and sources outside the task workspace. State the permission boundary
  without granting delegates enclosing session-ending authority.
- Specify public compatibility/version behavior and the transition from the
  no-injection promise. Keep Ravel generic: no Grove kinds, tree meanings,
  provider-receipt interpretation or result-token interpretation.
- Map each behavior to public launch/resolution tests with isolated temporary
  homes/workspaces and local Git fixtures; cover offline and failure paths,
  readable supporting resources, concurrent/lifetime behavior and unchanged caller
  ending commands. Name real-harness trial limits separately from fixture checks.
- The spec set is coherent and reviewable, its public seams are sufficient for
  the Grove adoption design, and the task records owning-repository commit
  references. Implementation and owner-configuration edits remain later work.

## Notes

This contract is load-bearing for all external packs. Once the artifact exists,
judge review under Grove's lazy review rule. If needed, insert review-design
with this bare stem before the first dependent sibling; do not let dependent
planning treat an unintegrated contract as settled. A design too large for one
focused session decomposes within this subtree.
