# 020-design-seed-convention

**Kind:** planning

## Goal

Drive a grilling session over the prior-art research in
`docs/research/seed-capture-prior-art.md` to converge on a designed seed
convention. Sharpen the glossary as terms resolve; raise ADRs sparingly
for the decisions that are hard to reverse or are real trade-offs; write
a PRD if (and only if) the increment becomes a genuine agreement point.
Then grow the tree — replace this leaf with a node whose child leaves
implement the chosen convention.

## Context

- `docs/research/seed-capture-prior-art.md` — the prior-art survey to
  grill on. Pay particular attention to its **Shortlist** (top of doc),
  its **Cross-cutting findings**, and its **ADR candidates flagged**
  section (six listed).
- `CONTEXT.md` — current `Seed` definition; expect to refine it.
- `.claude/skills/grove/SKILL.md` — especially constraints 1 (artifacts,
  not state), 4 (lazy and optional), and 6 (walk-away-able). The research
  identified WAW as the decisive divider across the paradigm space; the
  design must honour it.
- The parent brief's four use cases (deferred-future, parallel-grove,
  multi-source, cross-repo) remain the rubric.

## Done when

The grilling has settled enough of the design space that a decomposition
exists:

- The six flagged ADR candidates from the research artifact have each
  been visited; for each, either an ADR is raised, the decision is
  recorded inline in a brief, or the candidate is explicitly deferred.
- `CONTEXT.md` is updated where the grilling resolved or sharpened terms
  (e.g. the `Seed` entry; possibly new entries like *seed inbox*,
  *seed promotion*, *seed germination* — only if they earn their place).
- This leaf is replaced by a `020-design-seed-convention/` node whose
  `BRIEF.md` captures the agreed shape and whose child leaves carry the
  implementation work (likely a mix of *work* leaves for any tooling and
  *planning* leaves for sub-decisions that need their own grilling).
- A PRD under `docs/prd/` exists **only if** the grilling reaches a
  genuine human-facing agreement point — not as a checkbox.

## Notes

- **Grill the shortlist first.** Dangling-link markdown seeds vs.
  maildir-shaped inbox are not mutually exclusive (the research suggests
  they combine); confirm or reject that hypothesis before fanning out.
- **The six flagged ADR candidates** (storage location, identifier shape,
  state transition, multi-source aggregation, cross-repo handoff,
  promotion/germination) are an *agenda*, not a script. Drop candidates
  the grilling makes moot; surface new ones that emerge.
- **Resist building tooling here.** The deliverable of this planning task
  is *more tree*, not code. CLI affordances and `grove seed` verbs are
  implementation leaves, raised after the convention is settled.
- **Walk-away-ability is the binding constraint.** Any proposal that
  fails it should be rejected explicitly, with the rejection captured —
  future revisits will otherwise rediscover the same dead end.
- Time-box ambition: one focused session converges on the convention;
  follow-up planning is fine for sub-decisions that turn out to need
  their own deep dive (especially anything CRDT/multi-writer, which the
  research deferred).
