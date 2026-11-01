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

## Decisions (running log)

- Use Ravel's existing preparation boundary before policy evaluation. The policy
  receives both the original caller prompt and the prepared launch prompt.
  Configuration belongs to owner settings, with no augmentation helper in
  individual policies. The changed prompt meaning requires an explicit major
  release and policy/request schema transition.
- Resolve local and Git sources into immutable, retained resource snapshots.
  Local sources are read afresh for each independent resolution; a running
  session and its delegates keep their original catalog. Published snapshots
  are not automatically pruned, avoiding cleanup races with untracked delegates
  and independently started readers. Manual cleanup requires those readers to
  have stopped. This storage cost is an explicit trade-off.
- The human approved the presented design shape: preparation before policy,
  immutable retained catalogs/resources, the major-version prompt transition,
  and the separate responsibilities for delegate reads and ending authority.
  Detailed specification and acceptance work continues within this design leaf;
  implementation and owner-configuration changes remain later work.
- Specify `skills` in owner settings, required local/Git sources with discovery
  roots, ordered mandatory qualified IDs, a policy-free `ravel skills resolve`
  seam, and refusal of duplicate configured unqualified names. Configured paths
  take precedence over native copies through loading instructions; Ravel cannot
  retract native text that a harness already injected.
- Keep context schema 1 and exact caller ending bytes; propose Ravel 2.0 with
  policy/request schema 3 and private protocol 2. Both policy phases receive
  originalPrompt, prepared prompt and immutable discovery metadata. Disabled
  discovery leaves prompt bytes unchanged. Refuse schema 2 with a migration
  remedy rather than reinterpret its unchanged-prompt promise.
- Confined launches grant only published catalog/resource trees. Delegates must
  receive those reads but no enclosing ending context through their harness
  adapter. Explicit nested launches resolve afresh within their inherited OS
  restrictions; inaccessible owner configuration/cache causes refusal, not
  widened grants. Real harness observations remain distinct from fixture checks.
- Retention avoids a reader-lease/collection state machine. No formal model is
  claimed; public resolution/launch fixtures and real-filesystem concurrency
  controls are the chosen instruments. A load-bearing review-design leaf will
  precede Grove's adoption design; no duplicate in-session reviewer is used.

## Delivered artifacts and verification

Owning repository: `/Users/antony/Development/AgenticOrchestration/Ravel`.
Design commit: `b68af6e492316494dfce7ec2c0ee77a1a996e60b`
(jj change `kqtnkxrunqnmuypxwtmsqnmvmvuxxukk`), titled
`Specify automatic skill discovery (ravel-skill-discovery-k7)`.

- `docs/specs/skill-discovery.md` owns configuration, metadata, source identity,
  local/Git capture, offline behavior, retained publication, catalog loading,
  prompt/context APIs, limits, permissions, compatibility and acceptance cases.
- `docs/specs/session-execution.md` references that contract and distinguishes
  the Ravel 2 design from installed 1.1 behavior. `CONTEXT.md` defines its terms.
- `docs/skill-discovery-design-evidence.md` records exact existing source seams,
  graph generation/coverage limits, alternatives, costs and verification limits.
- `docs/design/skill-discovery/` contains editable views; `mise exec -- task
  docs:skills` serves them at `http://127.0.0.1:8776/#discussion` while running.

Design checks passed for 32 local links/anchors, diagram manifest consistency,
whitespace/placeholders and five exact HTTP resources across 11 unchanged
subjects. The Taskfile parsed and its new serving recipe ran successfully.
Browser rendering could not be measured because the available browser inventory
was empty. No product implementation, owner configuration, behavioral acceptance
tests or release occurred; those remain later work.

The independent `review-design` leaf is `ravel-skill-discovery-k13`, placed before
`grove-skill-delivery-k8`. Its doubts are named in its charter. Current provider
evidence was absent, so its Creator line is deliberately absent under the
retirement contract. No parent closes: the review, adoption design, planning and
the subtree's implementation/acceptance obligations remain live.
