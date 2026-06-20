# 010-plan

**Kind:** planning

## Goal

Design how to refactor grove so that it is expressed as / runs as an **Archon
workflow** (Archon = https://archon.diy, the open-source *workflow engine /
harness builder* for AI coding agents). Walk the design tree, settle the
foundational decisions, and grow the child leaves that implement them.

## Context

**What Archon is (primary sources: archon.diy, github.com/coleam00/Archon
README, fetched 2026-06-20):**

> Tagline: "The first open-source harness builder for AI coding. Make AI coding
> deterministic and repeatable."

- A **workflow engine**: workflows are YAML files under `.archon/workflows/`.
- A workflow is a **DAG of nodes**. Per-node fields:
  - `id` — step identifier
  - `depends_on` — array of prerequisite node ids (the DAG edges)
  - `prompt` — AI instructions, executed by a Claude Code / Codex SDK agent
  - `bash` — deterministic shell command, no AI involvement
  - `loop` — iteration block with an `until:` condition and a `fresh_context`
    flag (each iteration can run in a fresh agent context)
  - `interactive: true` — pauses for human input / approval (an approval gate)
- Each workflow **run executes in an isolated git worktree** (parallel runs
  don't collide).
- Runs on **CLI** (`archon …`, usually invoked via a Claude Code agent), a
  **Web UI** (chat + monitoring + visual builder), and platform adapters
  (**Slack / Telegram / Discord / GitHub**).

**Why this is a strong fit for grove (and the central tension):**

grove *already* runs each workstream in an isolated git worktree
(`.grove-worktrees/<name>/`); its loop (pick → bootstrap → execute → commit →
retire) is a workflow; "one task per fresh session" is exactly Archon's `loop`
with `fresh_context`; planning grilling is Archon's `interactive: true`; the
`grove-llm` tree verbs are deterministic `bash:` nodes.

**The central tension:** grove is **self-extending** — the steps (the task
tree) are *generated at runtime* by planning tasks, not known up front. An
Archon workflow is a **predefined YAML DAG**. So the natural mapping is *not*
one Archon node per grove leaf (you can't enumerate leaves up front). The
candidate resolution is: a small fixed Archon workflow whose body is a `loop`
node that repeatedly `grove-llm pick`s and runs the picked task until the tree
is empty — grove's *dynamism stays inside the loop body as data*, while Archon
supplies the *engine* (worktree, loop/fresh_context, interactive gate,
bash/prompt nodes). This is the thing the grilling must confirm or break.

## Done when

- The foundational relationship (what "be an Archon workflow" concretely means
  for grove — replace vs. wrap vs. port) is settled and recorded.
- The mapping from grove's loop + artifacts onto Archon's node model is settled.
- Decisions are captured inline (running log below; ADRs where durable); child
  leaves are grown for the implementation work.

## Notes

### Decisions (running log)

**D1 — "Archon" identity (settled 2026-06-20).** Archon = https://archon.diy,
the coleam00 open-source *workflow engine / harness builder* for AI coding
agents (YAML workflows of bash/prompt/loop/interactive nodes, each run in an
isolated git worktree). *Not* the earlier "MCP knowledge-base / project-task
command center" framing of Archon — the project pivoted to the workflow-engine
identity. This is the Archon that grove is to be refactored onto.

**D2 — End-state: replace the runtime, keep the brain, shed aggressively
(settled 2026-06-20).** Target is interpretation **(A)**, taken to its lean
extreme. Stated goals, in the user's words:
- "A big part of my goal is to have **less in grove**" — shrink grove's surface.
- "especially to **get rid of the TUI**" — the entire rmux/ratatui TUI + fleet
  tower (ADR-0013/0022/0025/0026/0027/0028/0029/0030, `src/tui/`) is retired in
  favour of Archon's web UI + platform adapters. Not negotiable.
- "**The key part of grove is the self-extending task tree.**" — this is the
  irreducible core that survives the refactor.
- "Just about everything else **could be replaced by other third-party skills**,
  if not for the fact that **the self-extension mechanism is pervasive**." — the
  reason grove bundles grilling / driving habits / TDD / review etc. is only
  that self-extension threads through them. The refactor's job is to
  *de-pervade* the self-extension mechanism into a clean minimal core, so the
  rest can be shed to independent skills (or to Archon's engine) rather than
  bundled.

So the refactor = **isolate the self-extension core, express it as an Archon
workflow, and shed everything else** (TUI → deleted; bundled methodology →
third-party skills; worktree/fresh-context/approval/multi-surface → Archon).

**D3 — The core boundary (settled 2026-06-20).** User agreed the Q3 inventory.
SURVIVES = the self-extension core only: the task tree as data, the `pick`
walk, the grow verbs (`root-init`/`leaf-add`/`leaf-insert`/`leaf-decompose`/
`leaf-retire`/`brief-chain`), the two task kinds, the minimal loop skeleton.
SHEDS → third-party skills: grilling, the `driving.md` habits, CONTEXT/ADR/PRD
*format* guides, TDD/review/debugging. SHEDS → Archon engine: worktree
lifecycle, fresh-session looping + harness driving, multi-surface driving.
DELETED: the rmux/ratatui TUI + Fleet tower. **Both formerly-open items shed
too:** the **inbox / `grove-meta` branch** subsystem and the
**install/materialise-into-harness** machinery (incl. `VERSION.md` drift model)
are both removed — coordination/distribution that Archon's model or "less in
grove" makes redundant.

**D4 — Task identifiers: flat dotted-decimal numbering (direction settled
2026-06-20; exact scheme TBD, likely its own leaf).** Replace directory-nested
nodes (`NNN-slug/BRIEF.md` + `NNN-slug.md` leaves) with a **flat** namespace of
dotted-decimal-prefixed files: a node `1.2.5.2` carries `1.2.5.2.BRIEF.md`; its
children are `1.2.5.2.1.<task>.md`, `1.2.5.2.2.<task>.md`, … Rationale (user):
*infinitely extensible* (insert at any depth without restructuring dirs) and
*ordering corresponds to the DFS tree walk* (so `pick` falls out of a sort).
User-flagged cost: reordering across live vs `done/` is more complex **only when
changing the relative order of subtrees** — pure append-insertion is easy. User
notes there is prior art for the numbering mechanism. Open sub-decisions
(grilling below): exact sort semantics, insertion/reorder mechanics, how `done`
is represented, migration of the existing scheme.

**D5 — Numbering scheme specifics (settled 2026-06-20).** User accepted the
recommendation: **legible sequential dotted integers** (not fractional/LexoRank
keys) ordered by a **numeric per-segment version-sort comparator** (so true
infinite width *and* DFS order both hold; order is defined by grove's
comparator, not raw byte order). **Renumber-on-reorder is accepted** as the cost
(reorder of existing subtrees is rare; append + decompose, the common ops, stay
cheap). **Mark-done-in-place** replaces the `done/` directory: retired items
stay in the one flat list with a done marker (suffix or frontmatter), so there
is no separate `done/` numbering to keep in sync — the only cascading renumber
that ever happens is a deliberate sibling-reorder. Exact verb mechanics +
migration → dedicated leaf.

**D6 — Restartability: resume-safety by construction (settled 2026-06-20).**
grove's constraint 1 (artifacts-not-state; resume is state-checked, never a
marker file) is *what makes grove restartable on an engine with no durable
workflow state*. Design stance: write the Archon loop body to be **stateless
and self-locating** — each iteration re-derives position from `grove-llm pick`,
holds zero engine-side state — so "restart" = "re-invoke the workflow against
the grove's branch," working whether Archon resumes a long run or starts a
fresh short one. This makes Archon's run-durability *irrelevant* rather than
relied upon. Three existing mechanics already guarantee per-unit restart-safety:
commit-before-retire (mid-task crash → next `pick` returns the same leaf →
redo), the running-decision-log in the task file (mid-grill crash → re-read log,
continue), and done-ness living in the tree (mark-done-in-place). Of the three
mapping shapes — (i) long loop run, (ii) repeated single-task runs, (iii)
resume-safe loop — we target **(iii)**, which degrades to (ii) if Archon can't
resume and upgrades to (i) if it can. **First child leaf = a focused research
leaf** nailing Archon's actual run/worktree/restart/loop semantics against
primary sources, before the mapping is finalized.

**D7 — Target execution model: a continuous Archon loop, not repeated manual
runs (settled 2026-06-20).** The *core appeal of Archon* (user) is eliminating
grove's current human-as-scheduler model — today the user re-runs `grove do`
per task. The target: Archon runs a **`loop`** with **`fresh_context: true`**;
each iteration = one grove task in a fresh Claude context; **the grove process
signals when a step is complete**, which clears/exits that context, and the loop
proceeds to the next task with fresh context, continuing until the tree is empty
(`until:` = `grove-llm pick` returns no live leaf). grove's "one task per
session, fresh context per task" principle is *unchanged*; only the crank-turner
moves from human to engine. This makes shape **(i)/(iii)** the target and demotes
(ii) repeated-manual-runs to a *crash fallback*. **Two make-or-break Archon
facts** this rests on (both now top-priority research questions): **(C)** how an
iteration signals completion + how `until:` is evaluated — can `grove-llm pick`'s
result drive it?; **(D)** can an *interactive* multi-turn grilling live **inside**
a `loop` iteration? If Archon's loop cannot host a mid-iteration interactive
pause, the automated-loop model breaks for every planning task — the single
biggest risk. The research leaf gates the mapping on these two.

**D8 — Substrate premise REOPENED: Archon vs self-driven loop + global skill
(IN PROGRESS, 2026-06-20).** User is reconsidering whether Archon is needed at
all, on "less in grove" grounds. Amends D1/D2: **Archon is now one candidate,
not the settled substrate.** Two threads:

*(a) Distribution — leaning settled, refines D3.* Replace per-worktree skill
materialisation + the `VERSION.md` drift model (already slated for deletion in
D3) with a **single global grove skill** (+ globally-installed `grove-llm`
binary) that **handles backwards compatibility** by reading *both* the old
`NNN-slug/` directory format and the new dotted-decimal flat format — existing
groves keep working with zero migration. Post-refactor grove collapses to:
global `grove-llm` CLI + global skill + a loop driver.

*(b) Loop substrate — open, → spike.* Candidates for automating
fresh-context-per-task: **Archon** (`loop`+`fresh_context`); **iTerm2 triggers**
(user only needs iTerm — sentinel print → trigger sends `/clear`); a **thin
grove PTY-wrap** supervisor; **headless `claude -p` / SDK shell-loop** (the
"hot" loop techniques); possibly harness-native loop features. Discriminator:
the loop must host BOTH autonomous work AND interactive multi-turn grilling.
Headless loops give fresh context free but are non-interactive (break grilling);
interactive-Claude + a thin context-clear signal (iTerm-trigger / PTY-wrap)
grills natively and only automates the between-task `/clear`. Tension:
self-driven minimizes *distribution* surface but a PTY supervisor RE-GROWS the
process machinery grove just shed (trellis→rmux, ADR-0028) — so the real fork is
*which complexity to own*: external dep (Archon) vs brittle terminal config
(iTerm) vs small supervisor (PTY-wrap). **Resolution:** rescope the first child
from "Archon semantics" to a **loop-substrate spike** — Archon as one candidate,
all options through the same pass/fail gates (C fresh-context-per-task, D
interactive-grilling-in-loop, B restart-safety) + global-skill/backwards-compat
mechanics. 030 then decides the substrate.

*Archon stays in as a measured candidate (user, 2026-06-20).* **Named
hypothesis for the spike to confirm/refute:** Archon's conditional/DAG logic
(`depends_on` + `loop`/`until` + conditionals) may express **both** the restart
(resume-safety) **and** the loop-until-no-live-leaves with one declarative
mechanism — because for a stateless self-locating body (D6) restart and
loop-continuation are the *same* `grove-llm pick` evaluation, a restart being
merely an iteration that starts a fresh process. If true, Archon handles
restartability + termination for free (no grove-side machinery) — a strong
point in its favour. The spike must verify this against Archon's real semantics,
not assume it.
