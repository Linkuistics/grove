# methodology-k3

**Kind:** work

## Goal

Teach the methodology to prune. `content/` is canonical (the binary embeds it and
extracts it to the global skill dir), so every prose change lands there — never in
`~/.claude/skills/grove/`, and never in a project mirror.

## Context

Beyond the brief chain and ADR *pruning*:

- `content/SKILL.md` — the loop. Pruning is **not** a new step in the seven-step
  spine; it is the second possible *outcome* of a leaf, so it belongs alongside
  **Retire** (which currently assumes every leaf finishes done).
- `content/driving.md` — "Externalizing surfaced work" is the natural neighbour:
  that section is about work that *arrives*; this is about work that *dies*.
- `content/TASK-FORMAT.md`, `README.md` — the verb list and any `DONE`-only prose.
- `CONTEXT.md` and `docs/adr/{pruning,task-tree-scheme}.md` are **already written**
  by `plan-k1`. Read them; do not rewrite them. If prose here contradicts them, the
  ADR wins — or come back and fix the ADR in place, but do not let the two drift.

## Done when

- **`SKILL.md`'s Retire step covers both outcomes.** A leaf ends `DONE` (harvested)
  or `ABANDONED` (pruned); `leaf-prune` is named next to `leaf-retire`; the
  parent-chain cascade is unchanged (a node's done-ness is the absence of a *live*
  leaf, however its leaves finished).
- **The HITL guard is explicit and unmissable**: an agent never prunes on its own.
  An AFK session that finds a leaf dead says so and stops. Say plainly that the loop
  stalling on an abandonment decision is the system working, not a fault.
- **The durable-record rule is stated where a session will actually meet it** — the
  rejection goes to the ADR set (the positive fact the abandonment establishes),
  carrying *what / why / what would reopen it*; and if it is too small to clear the
  when-to-write bar, nothing durable is written. Cite ADR *pruning*; do not restate
  it (the grain rule: an ADR records the decision, prose points at it).
- **`driving.md` gains the judgement** that the CLI cannot encode: **prune vs
  reorder vs issue.** Not-now-but-still-ours is a **reorder**; not-ours-at-all is a
  **GitHub issue**; decided-against is a **prune**. Getting this wrong is how a
  tree starts lying — and the taxonomy the ADR rejected is exactly what a reader
  will otherwise reach for.
- Verb lists in `README.md` / `--help` prose mention `leaf-prune`.
- No file under `~/.claude/skills/grove/` is hand-edited.

## Notes

The worked example is `git show 5177ea4` — the abandonment done entirely by hand.
The prose should be good enough that a session facing that situation again does it
with two commands and no invention.

Keep it **short**. `SKILL.md` is spine-constrained to one page of rules
(constraint 7); pruning must cost it a few lines, not a section. If it cannot be
said briefly, the design is wrong, not the prose.
