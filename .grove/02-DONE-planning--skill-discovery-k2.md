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

## Decisions (running log)

The working boundaries are common loading with the corpus still here, a working
external research pack, the remaining independent repository moves, and final
rollout. Each becomes a separate subtree with a verifiable working handoff.
The first subtree will establish Ravel's public contract and Grove's adoption
contract before a fresh planner cuts implementation from those designs. Later
subtrees retain their obligations in briefs and start with focused planning;
their implementation details depend on the preceding proof.

Source verification supports that boundary: Ravel's `run` prepares selection
before command construction, its worker freezes the request before context and
selection, and Grove's `ensure_codex_skills` has a direct caller in CLI execution.
Grove's `session_command::lifecycle` passes its mandate as public Ravel input.
This is bounded Verify-tier evidence, not an exhaustive delivery audit. Graph
generations were `2026-11-01T10:23:45Z` for this workspace and
`2026-11-01T07:59:38Z` for Ravel. The inspected Rust/TypeScript paths had matching
metadata and no recorded gap; Ravel excludes `docs/`, so its session-execution
specification was read directly. Both specifications still state the old
unchanged-prompt contract, and Grove's provisioning spec still requires broad
native Codex delivery. The design leaves must replace those contracts coherently.

One fresh-context reviewer examined the increment briefs and task obligations
against the accepted root contract. Its one finding was valid and actionable:
the rollout node required the completed Grove release before closing, while
its own decomposition correctly assigned that release to the later finish
session. The node now ends at verified readiness and a concrete finish handoff;
Ravel's independent release precedes it, while final Grove integration, repeated
integrated checks, minor release and installed verification stay explicit root
outcomes owned by finish. This is a correction of conflicting completion text,
not a new runtime design; it does not require a second reviewer. The leaf's
single in-session review allowance is spent.

The resulting tree has four increment nodes and six actionable children. All
ten new handles resolve and every distinct parent brief chain reads correctly.
The next task after this retirement is ravel-skill-discovery-k7. No node closes
with this leaf: the new subtrees retain the implementation and acceptance work.
No product code, external repository, owner configuration or installation was
changed by the planning work.

Verification ran through the existing Taskfile. The initial
`mise exec -- task check` completed all 20 check groups, with 19 passing:
1,333 runtime tests passed (21 skipped), nine doctests passed, all five books
validated, and the conformance suite's 21 controls passed. The sole failed group
was Ravel caller assets/provenance because ambient Bun 1.4.4 did not meet that
script's exact 1.4.3 pin. The installed mise Bun 1.4.3 was verified, then
`mise exec bun@1.4.3 -- task ravel:review RAVEL=/opt/homebrew/bin/ravel`
passed the complete failed group, including policy/compiler checks, nine
installed-policy tests, 13 creator/environment tests, two enclosing-control
tests, 17 composition-guidance tests and corpus conformance. No persistent tool
configuration was changed. Product/test inputs remained unchanged; the working
copy diff contains only this task tree. The initial whole-command exit remains
a failure, with its one failed group separately verified under the required pin.
