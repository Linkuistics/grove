# harness-on-leaf-k3

**Kind:** planning

## Goal

Decide how a grove sequences work **across harnesses**, and grow the tree for
it. The shape proposed in planning: harness becomes a per-leaf property declared
in the task file beside `**Kind:**`, replacing the per-grove
`GROVE_<KIND>_HARNESS` env reroute.

## Context

- `src/loop_driver.rs` — `resolve_launch` and its helpers (`harness_override`,
  `any_harness_override_env`, `validate_all_harness_overrides`, the
  `KindPeek::Degraded` refusal) are the machinery this would replace. The driver
  already peeks the picked leaf's kind via `grove-llm kind` before launching; it
  would peek harness the same way.
- `src/harness.rs` — the three-harness table, and the per-harness quirks that
  stay necessary under this model (codex has no launch-time name flag; codex
  selects `--profile` not `--model`).
- ADR *model-per-task-kind* — this reworks the mechanism it describes. Per
  `linkuistics:decision-records`, rework that record **in place**; do not append
  a superseding one.
- ADR *task-kind-taxonomy* — kind is a closed set of five. Whether harness gets
  the same gates-on-write / degrades-on-read treatment is a question for this
  session.

## Done when

Shared understanding is reached and the tree is grown. The decisions this
session owes:

- **Where harness is declared** — a `**Harness:**` line on the leaf, a node
  brief, or something else. If on the leaf: does it gate-on-write and
  degrade-on-read the way kind does?
- **How the sequence is expressed.** Planning's finding was that
  impl → review → integrate needs *no new machinery* — it is ordered sibling
  leaves under a node, each with its own harness. Confirm that, and decide
  whether the triple gets a named shape / a grow verb, or stays hand-built from
  `leaf-add`.
- **Independent parallel work.** Planning's finding: running two harnesses on
  the same question is expressible as *sequential* leaves that don't read each
  other's output, plus an integrate leaf — no concurrency, no worktree fan-out,
  and the independence is real because sessions share no context anyway. Decide
  whether that's sufficient or whether real concurrency is wanted (it would need
  separate workspaces and breaks the single-signal-file loop).
- **What happens to `GROVE_<KIND>_HARNESS`** — deleted, or kept as a
  coarse override.

## Notes

The structural insight from planning, which this session should test rather than
assume: **grove sessions never share context**, whatever harness runs them —
bootstrap is "read the glossary, the brief chain, the task file" and nothing
else. So cross-vendor handoff is already free; the artifacts *are* the protocol
(constraint 1). The only thing missing is expressing harness at leaf
granularity.

The cost to weigh: under env reroute a review leaf runs **as** a codex session
— codex forms its own judgement, writes the findings, commits, retires. Under
in-session delegation (claude calling codex via MCP) the judgement is filtered
through claude before it reaches disk. Per-leaf harness keeps the former, which
is why it is preferred — but confirm that reasoning holds before committing to
it.

Independent of `02`; either may go first.
