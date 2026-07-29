# grove.using-codebase-memory — brief

## Goal

Ship one portable skill, `using-codebase-memory`, that lets an agent in any of
the four harnesses (Claude Code, Codex, Gemini, Pi) query the codebase knowledge
graph from the shell, and compose multi-step graph queries in bash rather than
as chained tool calls.

## Done when

- `plugins/linkuistics/skills/using-codebase-memory/SKILL.md` exists and every
  command it documents has been executed against a live indexed graph.
- The `linkuistics` plugin manifest names the skill.
- `./install.sh` places the skill in `~/.codex/skills`, `~/.gemini/skills` and
  `~/.pi/agent/skills`, verified by resolving one symlink and reading its
  frontmatter.

## Decomposition

Requirements, design and planning were **done and committed before this grove
started** — see Pointers. `scope-k1` absorbed them, checked them independently,
and cut the leaves below. It diverged from the plan in two ways.

| Leaf | Kind | Covers |
|---|---|---|
| `skill-k2` | `impl` | the whole `SKILL.md` — plan Tasks 1 **and** 2 |
| `skill-review-k3` | `review-impl` | disprove every claim in it |
| `skill-integrate-k4` | `integrate-review-impl` | apply the findings |
| `distribution-k5` | `impl` | plan Task 3, unchanged |

**Divergence 1 — Tasks 1 and 2 merged.** They write the *same file*, and the
plan says to run it under `superpowers:subagent-driven-development`, where
splitting is cheap because subagents share the parent's context. Grove leaves are
cold-start sessions: a second leaf would re-read the file and re-establish the
whole CLI contract just to append two sections. A ~120-line `SKILL.md` fits one
session, so the split bought nothing and cost a bootstrap.

**Divergence 2 — a review chain was added.** The plan argues per-task
verification suffices. The evidence says otherwise: the plan's own Global
Constraints assert a "verified" fact that is false (see the `min_degree` note
below). The commands had been run and the projection checked *did* match — the
prose was still wrong. That is a **reading** failure, not an execution failure,
and only a fresh context re-deriving the claim catches it. The BRIEF's own
done-when is a verification claim, so it deserves an independent certifier.
Confirmed with the human at scoping time, at a cost of two extra sessions.

**Not changed:** `distribution-k5` stays separate — different file, and its
verification writes symlinks into `$HOME`, a side effect outside the repo.

## Pointers

- Spec: `docs/superpowers/specs/2026-07-29-portable-codebase-memory-skill-design.md`
- Plan: `docs/superpowers/plans/2026-07-29-using-codebase-memory-skill.md` —
  three tasks, each with verified commands and expected output.
- Prior art in-repo: `plugins/linkuistics/skills/using-jujutsu/` (naming and
  house style), `install.sh` (distribution — globs the skills directory, needs
  no change).

## Notes

**Why a shell path at all.** Pi refuses MCP by design ("*No MCP.* Build CLI
tools with READMEs"), so a capability shipped as an MCP server strands one
harness and needs three config dialects for the other three. `SKILL.md` plus a
CLI reaches all four. `codebase-memory-mcp` exposes the same fourteen tools both
ways.

**Verified contract** (against `codebase-memory-mcp` 0.8.1, 2026-07-29 — all
four surprise on first use, and the third contradicts the installer-managed
skill):

1. `project` is required; it is **not** inferred from the working directory.
2. Logs go to stderr, JSON to stdout — `| jq` is clean.
3. `min_degree` gates on **total** degree (in + out); `relationship` and
   `direction` do not narrow it. The "high fan-in" recipe in
   `~/.claude/skills/codebase-memory/SKILL.md` is wrong for this reason.
4. Exit status is honest (0/1) but only observable **without** a pipeline.

**Correction to the plan, found at scoping (`scope-k1`).** Item 3 above is
right, but the *plan*'s Global Constraints overstate it as "produces
byte-identical results". Re-measured against 0.8.1:

```
min_degree:10, label:Function, limit:5      → total=2460
  + relationship:CALLS, direction:inbound   → total=2458
results[] identical; both include  in=0 out=11 above_pane_sets_autoscroll_up
```

The *filter semantics* are unchanged — that is the real gotcha, and it holds.
The *responses* are not identical. The skill must state the former, not the
latter.

**Authoring authority.** The plan cites `superpowers:writing-skills`. This repo
ships `linkuistics:authoring-conventions`, a **house delta that overrides
upstream's description rule** — house is capability + "Use when …", upstream is
when-only, and upstream's version is injected every session and will tempt an
implementer to strip the capability clause. Read both; house wins.

**Test fixture:** `Users-antony-Development-herdr` — a *live, drifting* index
(23,641/97,504 at plan time, 23,681/97,906 at scoping). This repo is **not**
indexed. Treat every exact figure as a re-check, and keep counts out of the
skill.
