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

**Verified contract** (against `codebase-memory-mcp` 0.8.1, re-derived
end-to-end by `skill-k2`; the shipped `SKILL.md` is now the authoritative
statement of all of it):

1. `project` is required; it is **not** inferred from the working directory.
   `list_projects` and `index_repository` are the exceptions.
2. On **success** JSON goes to stdout and logs to stderr, so `| jq` is clean.
   On **failure** stdout is *empty* and the error goes to stderr — so `| jq`
   shows nothing and the pipe masks exit `1` as `jq`'s exit `0`.
3. Malformed JSON is **discarded, not reported**: arguments become empty and
   you are told "project not found", the same message an unindexed project
   gives. Build interpolated arguments with `jq -n`.
4. `search_graph` truncates at `limit`, default **200**, flagged only by
   `has_more`/`total`. `trace_path` caps callers at 100 with no flag at all.
5. `min_degree` gates on **total** degree (in + out); `relationship` and
   `direction` do not make it directional. The "high fan-in" recipe in
   `~/.claude/skills/codebase-memory/SKILL.md` is wrong for this reason.
6. Exit status is honest (0/1), observable **without** a pipeline, and
   defeated by `--json`, which also duplicates its envelope to both streams.

**Corrections `skill-k2` made to the plan, the spec, and this brief.** Running
every documented command falsified several claims all three carried as verified:

- The spec's `| jq -r '.error'` idiom cannot work — errors never reach stdout.
- "produces byte-identical results" (plan) and "results[] identical" (this
  brief, at scoping) are both false. `relationship`/`direction` drop 2 rows of
  2460; the earlier check saw agreement only because `limit:5` hid it. The
  durable, reproducible claim is item 5 plus: exactly **half** the rows that
  filter returns have `in_degree: 0`.
- The plan's flagship composition recipe passes `limit:200` — the default — and
  sorts client-side, so its "top 20 fan-in" ranks an arbitrary 8% of 2460
  matches. Aggregation and global top-N belong in `query_graph`'s Cypher.
- `trace_path` on a bare `function_name` shared by several symbols resolves to
  **none** of them: 8 `wait_for_socket` symbols, 0 callers, exit 0. The
  `qualified_name` returns 67.

`docs/` still carries the falsified wording — see `docs-reconcile-k6`.

**Authoring authority.** The plan cites `superpowers:writing-skills`. This repo
ships `linkuistics:authoring-conventions`, a **house delta that overrides
upstream's description rule** — house is capability + "Use when …", upstream is
when-only, and upstream's version is injected every session and will tempt an
implementer to strip the capability clause. Read both; house wins.

**Test fixture:** `Users-antony-Development-herdr` — a *live, drifting* index
(23,641/97,504 at plan time, 23,681/97,906 at scoping). This repo is **not**
indexed. Treat every exact figure as a re-check, and keep counts out of the
skill.
