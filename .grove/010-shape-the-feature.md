# 010-shape-the-feature

**Kind:** planning

## Goal

Pin down what `grove status` should show — specifically, what "active grove"
means and which "versions" are load-bearing — by grilling the user. Grow this
tree with whatever work and (if warranted) further planning leaves the
agreement implies.

## Context

- `src/status.rs` — current implementation. Lists harness installs with their
  `VERSION.md` (with a drift warning) and the groves under `.grove-worktrees/`
  with `(live, done)` leaf counts and a harness stamp.
- `CONTEXT.md` "Flagged ambiguities" — the three senses of "grove" (CLI tool,
  methodology, workstream). "active grove" almost certainly belongs to sense 3
  (a named task tree / worktree), but needs a glossary entry.
- The grove CLI's own version (Cargo crate version) is **not** in the current
  status output. Whether it should be is one of the things to grill.

## Done when

- "Active grove" is defined to the user's satisfaction and added inline to
  `CONTEXT.md`.
- The version surfaces the user wants visible are enumerated (CLI binary?
  methodology per harness? methodology at grove-start time? others?).
- This tree has grown: either child leaves under `010-shape-the-feature/`, or
  sibling work/planning leaves (`020-…`, …) that carry the agreed scope into
  implementation.

## Notes

- Follow `grilling.md` — one question at a time, propose a recommended answer,
  walk the design tree.
- Update `CONTEXT.md` **inline** as terms resolve; do not batch.
- Raise an ADR only if a decision is hard to reverse, surprising, or a real
  trade-off — e.g. if "active" is given a non-obvious semantic (lockfile-
  based? session-PID-based? "has uncommitted changes"?). Otherwise no ADR.
- A PRD is probably overkill for this scope; revisit only if the agreement
  point clearly warrants one.
