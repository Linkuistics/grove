# capture-issues-for-later-groves — brief

## Goal

Establish a mechanism for capturing observations made during one grove that
belong to a *different* grove — future, parallel, or in another repo —
without losing fidelity, blocking the current grove, or rebuilding a full
project management system. The mechanism must satisfy grove's
walk-away-ability (SKILL.md constraint 6): if you delete grove tooling, the
captured material remains legible.

## Done when

- A convention (and optionally tooling) is documented for capturing a
  [[seed]] (see CONTEXT.md).
- The convention is shown to handle the four driving use cases listed below.
- Any seed file format introduced is plain markdown; any tooling is opt-in.

## Driving use cases

Four scenarios this grove must address:

1. **Deferred-future capture** — bug found during grove X, target grove `Y`
   does not yet exist; seed sits until someone runs `grove start Y`.
2. **Parallel-grove handoff** — bug found during grove X belongs in a
   currently-running grove Y; X must not block on Y, Y must see it soon.
3. **Multi-source aggregation** — several parallel groves each discover bits
   of the same future grove (e.g. multiple groves uncovering Racket bugs);
   the seed accumulates from many sources.
4. **Cross-repo capture** — observation made while working in a downstream
   repo belongs in an upstream repo's future grove (or vice versa).

## Decomposition

First leaf is pure research — survey existing distributed/local
issue-tracker and work-queue tools/conventions against the four use cases
above. The research is deliberately biased toward *non-obvious paradigms*:
Linear/GitHub Issues/Jira integration is the obvious fallback and the
baseline to beat, not the starting candidate. We do not decompose further
yet; the research output drives the next planning task.

## Pointers

- Glossary: `Seed` (CONTEXT.md).
- Methodology constraints: `.claude/skills/grove/SKILL.md` — especially
  constraint 6 (walk-away-able) and constraint 4 (lazy and optional).
