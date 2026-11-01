# skill-discovery-k2

## Goal

Plan the smallest independently useful increments for the accepted skill
discovery and repository-move workstream. Create the next actionable design or
implementation leaves, with design work ordered before the changes that depend
on it. Requirements are settled; implementation does not belong to this session.

## Context

- `plan-k1` records the interview, evidence, and final human confirmation.
  Its later corrections supersede the earlier proposals retained in the log.
- [Issue #25](https://github.com/Linkuistics/grove/issues/25) reflects the accepted
  scope. [#26](https://github.com/Linkuistics/grove/issues/26) and
  [#27](https://github.com/Linkuistics/grove/issues/27) are independent follow-ups,
  not prerequisites or leaves to grow here.
- Ravel source is at
  `/Users/antony/Development/AgenticOrchestration/Ravel`. Its
  `docs/specs/session-execution.md` currently promises unchanged prompts and no
  injected instructions. Request preparation in `crates/ravel/src/run.rs` and
  `crates/ravel/worker/src/main.ts` precedes owner-policy command construction.
- Grove's `docs/specs/codex-skill-provisioning.md` and
  `crates/grove/src/provision.rs` describe its current automatic delivery.
  `docs/specs/harness-selection-and-execution.md` describes the caller boundary.
  Recheck the source and current graph coverage before using this evidence to
  choose design or implementation boundaries.

## Done when

- The next increment is scoped to demonstrate Ravel's common discovery/loading contract while
  skill sources remain in this checkout, with only the necessary Grove delivery
  adjustments. The next selected leaf has a concrete artifact and completion
  conditions.
- Design leaves name the decisions needed for source/configuration interfaces,
  exact-revision caching, catalog and resource lifetime, missing-source bootstrap,
  duplicate handling, compatibility, and confinement/delegation. These are design
  questions within the agreed requirements.
- The work sequence places the external research-pack proof before moving the remaining
  domains, personal skill source, TestAnyware skill, marketplace, and four writing
  projects. Grow only the detail that is justified now; preserve later obligations
  in the relevant briefs.
- The public test seams already accepted in the root brief reach the appropriate
  leaves: Ravel launch/resolution, existing Grove tree/launch behavior and pack
  conformance, real Claude Code/Codex discovery, and migration preservation.
- The plan accounts for repository checks, durable documentation, paused-grove
  handoff text, and independent integration/release procedures. It preserves
  running sessions' original ending contracts.
- The tree and brief chain are valid, and this planning leaf is retired and
  committed with its resulting handoff.

## Notes

Ravel resolves configured skills and adds their discovery instructions to the
prompt. Grove's loop remains unaware of those added paths. Kinds stay open;
existing review/integration labels, filenames, stable handles, and policy routes
stay compatible. Do not recreate declaration-based admission, Grove discovery
commands, or generic review parameters while planning the move.

The root brief owns the accepted destinations, boundaries, and acceptance tests.
Use it rather than copying the whole requirements contract into each new leaf.
Follow each affected repository's instructions when its work becomes concrete,
and preserve unrelated edits and workspace registrations during relocation.
