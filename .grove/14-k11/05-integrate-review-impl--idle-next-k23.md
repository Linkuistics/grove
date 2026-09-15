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
