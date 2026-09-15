# idle-next-k23

**Integrates:** idle-next-k22

## Goal

Triage the `idle-next-k22` implementation review's findings against the typed
tree/runtime observer and its idle viewer — the artifacts of `captured-tree-k20`,
`bounded-runtime-k21` and `idle-activity-view-k19` — and apply the real ones
before `witnessed-activity-k13` plans against the landed observer.

## Context

Read the findings from the review's own commit, not from this body: they are
in the `## Findings` section of `idle-next-k22`'s leaf, each anchored to
`path:line` at the reviewed parent commit `1316c1275ce9`, with the evidence and
the classification the reviewer proposed. Grade each one yourself as a contract
stated unclearly, a real issue, a visible trade-off, or noise; the review's
"What held" list says which attack axes were checked and passed, so they need
not be re-derived. `docs/specs/item-status.md` remains the contract and is not
this leaf's to change; the `idle-next-k11` brief owns the increment's
acceptance conditions.

The artifacts are Rust source and tests under `crates/grove-loop` and
`crates/grove-tui`, and the prose in `docs/USAGE.md`, `docs/ARCHITECTURE.md`,
`docs/specs/module-decomposition.md`, `CONTEXT-MAP.md` and the grove-loop
walkthrough that explains the observer. A change to either observation file
moves the book fragments that reproduce it, so repair the affected fragments,
indexes and manifest in the same change. A finding that asks for the seam to be
redesigned rather than repaired becomes a new producer review chain beside this
leaf, not a fix here.

## Done when

- Every finding in `idle-next-k22` is graded in this leaf's running log, with
  the rejected ones named and the reason given.
- Each accepted finding is applied with a test through the existing public
  seams — `try_observe` and `Viewer::new`/`act`/`tick`/`render` — and the
  usage/architecture prose and book fragments say what now ships.
- Focused grove-loop and grove-tui tests and `bash scripts/check.sh` pass
  after all edits; no admission, lease or launch behaviour changes.
- Nothing is left for `witnessed-activity-k13` that this leaf decided; anything
  deliberately deferred to it is named in the `idle-next-k11` brief.

## Notes

An `integrate-review-*` session may spend one narrow in-session reviewer.
Substantial redesign is externalised as a new producer review chain beside this
leaf rather than absorbed here.

## Decisions (running log)

- F1: real issue. Current source drops runtime evidence through the outer tree
  Result and early root returns. Preserve independent two-sample activity even
  for tree errors, absence and contention; withhold NEXT until tree acceptance.
  Public viewer tests will cover failure, missing roots, recovery and both views.
- F2: contract stated unclearly in the diagnostic. An active epoch record is
  established; a live session is not. Change the wording to name the record,
  retaining Unavailable and verifying it through the real observer.
- F3: visible accepted trade-off. The post-lock and post-copy checks validate
  every pinned descriptor; the post-discovery check validates the workspace
  alias. The existing guarded replacement test exercises bounded retry, but
  does not inject the three other windows. Retain that explicit test limitation
  in the book; no new private hooks or deferred implementation obligation.
- F4: noise for correctness. Both calls use the same validated snapshot and
  shared selector; no alternative selection rule exists. Carrying selection
  would change the seam for an unmeasured optimization, so reject that change.
- Graph verification used generation 2026-09-15T11:44:22Z. Coverage reports the
  viewer paths changed and both loop observation files untracked by that
  generation. Current source, rather than stale symbol ranges, grounds triage.
- Implementation keeps runtime comparison outside the tree Result and performs
  both bounded captures even when the first tree fails. The viewer samples
  runtime before returning for root errors/absence and tracks accepted tree
  freshness separately for NEXT. No authority or lock-acquisition code changed.
- Public regression tests first failed for missing/malformed trees, then for
  the active-session wording after F1 was repaired. They now cover fresh idle,
  unsupported active records, missing roots, root-open failure, contention,
  recovery and filesystem preservation. A deterministic real-observer test
  compares changing activity when either tree sample fails.
- Focused grove-loop, grove-tui and jj-workspace tests pass. The first wider
  run exposed a missing-workspace diagnostic regression; retaining the original
  Missing state after runtime capture fixed it, and the complete focused rerun
  passes. Source fragments retain identical ranges; only the diagnostic literal
  changes in the loop book, so indexes and manifest ranges need no adjustment.
- Final verification: `bash scripts/check.sh` passes all eight principal checks,
  including workspace tests and all six final book validations. SHA-256 digests
  of 1,750 tracked files outside `.grove/` (source, documentation, configuration,
  scripts and fixtures) match before/after the run; no checked input moved.
- idle-next-k11's completion conditions are met by its shipped discovery,
  capture, runtime and viewer children plus k22/k23 review/integration. Promote
  the landed observer and accepted F3/F4 limits to the root brief, close k11,
  and leave witnessed-activity-k13 live. No ADR decision changed and no repair
  is deferred to that planner. No in-session reviewer was needed.
