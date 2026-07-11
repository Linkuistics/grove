# fix-stale-kind-count-k7

**Kind:** work

## Goal

Two docs still say "the two task kinds" even though `task-kind-taxonomy` grew
the closed set to five (planning/research/prototype/work/review). Noticed as a
tangent while working `methodology-k3` (pruning); not fixed there because it
doesn't serve that leaf's goal — externalized per `driving.md`'s own rule.

## Context

- `docs/adr/self-extension-core-and-methodology.md:7` — "the two task kinds"
  in the self-extension-core enumeration.
- `content/SKILL.md:335` (or nearby — line numbers drift) — the
  `TASK-FORMAT.md` reference-file bullet: "the task-file shape and the two
  task kinds."
- ADR `task-kind-taxonomy` is the source of truth for the current five.

## Done when

- Both spots say five (or just point at `TASK-FORMAT.md` / the ADR rather than
  restating a count that will drift again).
- A quick grep for "two task kinds" across `docs/` and `content/` comes back
  empty.

## Notes

Small and mechanical — shouldn't need grilling. If more stale counts turn up
nearby while fixing these, use judgement: fix an obviously-related stale
reference in the same breath, but externalize anything that turns into its own
investigation.
