# plan-k1

## Goal

Establish, by grilling, what "the grove loop should detect VCS, not the LLM"
means — and cut the tree for it.

## Context

Fresh-grove bootstrap: empty root brief, no prior sessions. The workstream proved
small enough to cut leaves here rather than add a `planning` leaf.

## Done when

- Outcome recorded in the root `BRIEF.md` (goal, rejected mechanisms, non-goals).
- Leaves cut. — done: `mandate-states-vcs-k2`, `probe-carve-out-k3`.

## Notes

Decisions, in the order they were settled:

1. **Problem** — the harness banner lies in a jj tree, detection is skippable
   because it is instruction-driven, and it spends session context on a fact the
   driver already resolved deterministically. Nested-layout divergence between
   grove's closest-marker walk and `jj root`'s unbounded one was explicitly *not*
   selected, and is a non-goal.
2. **Mechanism** — a driver-computed line in `mandate_prompt`. Rejected:
   `${vcs}`, a `grove-llm vcs` verb, `grove-llm` owning the commit boundary.
3. **Line content** — identity, resolved root, do-not-probe / disregard-the-banner.
   Rejected: the marker kind, the commit-boundary commands.
4. **`using-jujutsu`** — a generic carve-out that never names grove, keeping the
   skills context decoupled.
5. **Records** — `docs/ARCHITECTURE.md` seam + a `CONTEXT.md` term. No ADR (fails
   the when-to-write test), no `content/SKILL.md` edit (its Commit step is
   already lane-conditional and never asks the session to detect).
6. **Seam** — one, driver-level: the existing `tests/loop_driver.rs` mandate
   capture over both lanes. A unit test of the formatter was rejected as
   asserting a subset of the same claim.
7. **Decomposition** — two leaves split by bounded context, so each commit has
   one owner.

No glossary term was resolved *during* the interview — **Stated VCS** is a
proposed name for something that does not exist yet, so it lands with the code in
`mandate-states-vcs-k2` rather than inline here.
