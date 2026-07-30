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
  frontmatter. **Done, but under an isolated `HOME` — the real `$HOME` was
  deliberately not written** (see Notes § Distribution).

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
| `docs-reconcile-k6` | `impl` | reconcile the spec and plan to what shipped |

## Scope — this grove also carries grove-methodology work

The leaves below are **not** about `using-codebase-memory`. They belong to the
**grove** bounded context (`CONTEXT.md`, not `plugins/CONTEXT.md`) and were raised
during this grove's sessions. `chain-as-node-k7` argued in its own Notes that it
was likely a grove of its own; the human was asked at the start of that session and
**chose to keep it here** rather than prune it and start a new grove. The
Done-when above is therefore no longer the whole of what this grove will produce.

| Leaf | Kind | Covers |
|---|---|---|
| `install-workspace-guard-k8` | `impl` | **done** — `install.sh` hijacking a skill install from a secondary workspace; found by `distribution-k5`, and the reason this grove's third done-when is only half-verified |
| `chain-as-node-k7` | `design` | **decided**: a review chain / vendor pair becomes a node directory |
| `chain-node-k9` … `-integrate-k11` | `impl` chain | implement that decision |
| `retire-confirmation-k12` | `design` | whether the Retire cascade needs confirmation at all |
| `changelog-unreleased-k13` | `impl` | nothing this grove shipped is in `CHANGELOG.md`, and `v16.2.0` is already tagged — raised by `k8` |
| `stale-module-headers-k14` | `impl` | five `src/` module headers still isolate themselves from a deleted v1 verb path — raised by `k11` |

**What `chain-as-node-k7` decided**, since the tree below builds on it and
`.grove/` dies at the finish cycle. A chain gets its own **node directory** —
reversing a decision the ADR, the spec and the glossary each recorded. The three
arguments that had rejected it lapsed: `leaf-add-chain` made node creation
proactive, the node is **brief-less by rule** so it buys no unearned `BRIEF.md`,
and a brief-less node is never asked the Retire cascade's confirmation. The
decisive new argument *for* it is that a directory makes the group **structural**
in every tree viewer (`yazi`, Finder, `ls -R`), not just in the one grove controls.
Children keep the stem (`skill-review-k4`, not `review-k4`) because `resolve`
matches bare slugs exactly and commit messages outlive `.grove/`. Existing flat
chains are **not** migrated — detecting one needs the suffix parsing the design
forbids, and a flat chain is a valid tree. Recorded in
`docs/specs/task-kind-taxonomy.md`, `docs/adr/task-tree-scheme.md`,
`docs/adr/cli-binary-split.md` and `CONTEXT.md`; nothing here is the durable
record.

**What `chain-node-k9` shipped, and the three things it found.** The two verbs
now write `NN-<stem>-chain-k<key>/` (or `-pair-`) holding their steps at `01`–`03`
— four keys, four paths on stdout, node first. The three properties the spec marks
as worth falsifying by mutation are each pinned by a test: the kind derivation, the
absence of `BRIEF.md`, and a mid-write failure leaving **no directory**. Verified
end-to-end against a real fixture that `pick`, `brief-chain`, `resolve`,
`leaf-retire`, `kind --with-harness` and the tree viewer all cope unchanged.

1. **`leaf-add` was *not* untouched, and the leaf asked for this to be surfaced.**
   Its parent guard required a `BRIEF.md` at `<parent>`, so it refused a chain
   node — breaking the one affordance the node shape exists to buy
   (`leaf-add <chain-node> <stem>-late-step`). The guard now reads the directory's
   **name**, which is what ADR *task-tree-scheme* already said node-ness was; the
   charter distinguishes the two *species*, not node from non-node. Recorded in
   that ADR's *Comparator and verbs*. The other seven verbs needed nothing.
2. **`docs/adr/task-kind-taxonomy.md` still carried the reversed reasoning** —
   `chain-as-node-k7` reconciled the spec and two other ADRs but missed this one,
   which was still arguing "a chain — deliberately not a node — already closes with
   none" and that the cascade cost is *created* by giving a chain a directory.
   Reworked in place (never appended to), so the ADR set is coherent again. Worth a
   sceptical pass in `chain-node-review-k10`: if one ADR was missed, check the rest.
3. **`CHANGELOG.md` got an `## Unreleased` heading, and that is
   `changelog-unreleased-k13`'s decision to ratify or undo.** This change had to be
   logged somewhere and `## v16.2.0` is closed history — editing it would falsify a
   tagged release. So the heading exists now with one entry under it; k13 still owns
   the question and now decides it against a live instance rather than in the
   abstract. It should add the other four rows of its table, not re-litigate this
   one entry's placement unless it is discarding the heading entirely.

**What `chain-node-integrate-k11` found on triage.** All three of `k10`'s findings
were real — each reproduced or grepped before being touched, none accepted on
assertion — and all three are applied. Two things are worth carrying forward.

1. **The High finding was a contract violation, not an arithmetic slip, and the
   fix says where.** `add_run` created the node directory and *then* derived each
   child's key as unchecked `node_key + 1 + i`, which smuggled one resolution step
   past the mutation boundary — so the only failure it could express there was a
   partial tree. Reproduced exactly as `k10` reported (a live two-step node left
   behind); now the whole four-key run is allocated before the first write, beside
   slug validation and the destination check. `next_key` became fallible and
   `next_keys` joined it, so key exhaustion is a modelled fact in one place rather
   than a panic in three. The release half was worse than the debug half: wrapping
   gives the last step `k0`, which both breaks the consecutive-keys contract and
   *lowers the visible max*, so the next `leaf-add` re-issues a live key.
2. **Grepping for the claim found three more stale surfaces than reading found**,
   and two were `grove-llm --help` — the only surface a human at a terminal reads,
   still documenting three contiguous leaves and three printed paths. `k10`'s own
   item 4 said to grep rather than trust `k9`'s file list, and then found its three
   by reading. A file list is written before the work and goes stale; the claim
   cannot, because it *is* what went stale. The spec now records that as the
   lesson, with its own normative staleness as the second half of it.

`k10`'s **rejected candidate** (concurrent composite calls racing for the same
keys) is upheld as rejected: ADR *task-tree-scheme* defines a grove tree as
single-worktree, single-writer, and the one-snapshot logic is correct under it.
The `u32` ceiling is now a *refusal* under that same assumption, not a lock.

Externalized rather than absorbed: `stale-module-headers-k14`, five `src/` module
headers still declaring themselves isolated from a v1 verb path this repo deleted.
Same failure class, different generation — noticed while editing those very
headers for the chain-node claim.

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

- The `| jq -r '.error'` idiom cannot work — errors never reach stdout. (It is
  the **plan**'s, at Task 1 Step 4, not the spec's as this brief first said; the
  spec's version of the same error was an unqualified "`| jq` is therefore clean
  with no redirection needed".)
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

**`docs/` is reconciled** (`docs-reconcile-k6`). Both documents now open with a
status banner naming the shipped `SKILL.md` as the authority for every CLI
behaviour claim; the plan's Global Constraints and both of its embedded
`SKILL.md` drafts are **deleted** rather than annotated, on the grounds that a
second copy of a file that already exists is the defect, not the wording. What
survives in each is the design reasoning and the record of what ran. The
falsified claims are kept only quoted-and-refuted, so the correction is legible
without the assertion surviving anywhere.

**Distribution — `install.sh` cannot be run from this working tree.**
`distribution-k5` verified the install path against an **isolated `HOME`**, not
the real one, and left `$HOME` untouched. The script derives its link source from
`${BASH_SOURCE[0]}` and unconditionally re-links, so running it here — a
*secondary* jj workspace, while the default is `/Users/antony/Development/grove`
— would have re-pointed all 15 already-installed linkuistics skills at a tree
that dies with this grove, silently. The evidence it did produce is stronger for
the actual claim: `48 = 16 skills × 3 harnesses`, all three targets exercised,
including `~/.gemini`, which does not exist on this machine and would otherwise
have printed `skip`. The manifest edit and the glob-pickup are therefore fully
verified; **only the real-machine install is outstanding**, and it is not
grove's to do — `install.sh` should be run from the default workspace once this
grove's work is integrated.

**The underlying defect is now fixed** (`install-workspace-guard-k8`).
`install.sh` probes whether the tree it lives in is the repo's main checkout —
jj-first, mirroring the binary's `repo::vcs_of`, which is load-bearing rather
than merely consistent because a secondary jj workspace of a colocated repo is
not a git worktree and a git-first probe would miss it entirely. It **refuses**
rather than warns, since the damage is silent and delayed; `--force` opts in for
the one legitimate case, testing an unmerged skill live. `install.test.sh` covers
nine tree shapes against an isolated `HOME`. `docs/adr/symmetric-vcs-rule.md` now
names three enforcers, not two. **This does not change the outstanding item
above** — the real-machine install still has to be run from the default
workspace, and the guard is precisely what now makes running it from here fail
loudly instead of silently.

**Authoring authority.** The plan cites `superpowers:writing-skills`. This repo
ships `linkuistics:authoring-conventions`, a **house delta that overrides
upstream's description rule** — house is capability + "Use when …", upstream is
when-only, and upstream's version is injected every session and will tempt an
implementer to strip the capability clause. Read both; house wins.

**Test fixture:** `Users-antony-Development-herdr` — a *live, drifting* index
(23,641/97,504 at plan time, 23,681/97,906 at scoping). This repo is **not**
indexed. Treat every exact figure as a re-check, and keep counts out of the
skill.
