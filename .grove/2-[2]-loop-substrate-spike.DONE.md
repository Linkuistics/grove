# 2-[2]-loop-substrate-spike

**Kind:** work (research)

## Goal

Decide *nothing* — gather the primary-source evidence that lets `030` decide the
**loop substrate** for grove-on-a-workflow: how to drive a continuous loop that
runs one grove task per **fresh context** until the task tree is empty, hosting
both autonomous *work* tasks and interactive *grilling* tasks, and surviving
restart. Produce a cited options doc.

## Context

Read the root `BRIEF.md` and the retired `010-plan` running log (D1–D8) for the
full design rationale. In short: grove is being refactored down to its
self-extension core, runtime shed to a workflow substrate, TUI deleted,
distribution moved to a single global skill. The one open foundational fork is
**which substrate drives the loop** — this spike feeds the decision.

**Candidates to evaluate (each its own section):**
1. **Archon** (https://archon.diy, github.com/coleam00/Archon) — YAML workflow
   engine; nodes `id`/`depends_on`/`prompt`/`bash`/`loop`(`until`+`fresh_context`)/
   `interactive`; each run in an isolated git worktree.
2. **iTerm2 triggers** — regex-on-output triggers firing coprocess/keystrokes
   (the user only needs grove in iTerm). Sentinel print → trigger sends `/clear`.
3. **Thin grove PTY-wrap** — a small grove supervisor spawning Claude in a pty,
   watching for a completion sentinel, restarting with fresh context.
4. **Headless `claude -p` / Claude Code SDK shell-loop** — the "hot" Ralph-style
   agentic loop; new process per iteration = fresh context for free.
5. **Harness-native loop features** — whatever Claude Code itself now offers
   (e.g. loop/schedule/workflow primitives); pin down what exists and its limits.

## Done when

A doc exists at `docs/research/loop-substrate-options.md` that:

- Has **one section per candidate**, each scored against the three **pass/fail
  gates**:
  - **(C) fresh-context-per-task** — can it automate "run one task, clear/exit
    the context, start the next task fresh"?
  - **(D) interactive grilling INSIDE the loop** — can a multi-turn human
    interview happen mid-iteration, then the loop resume? *This is the
    make-or-break; a candidate that fails D is disqualified for planning tasks.*
  - **(B) restart-safety** — after a crash/quit mid-loop, does re-invoking
    resume correctly from tree state (not a marker file)?
- Tests the **named Archon hypothesis**: can Archon's conditional/DAG logic
  (`depends_on` + `loop`/`until` + conditionals) express *both* restart *and*
  loop-until-no-live-leaves with **one** declarative `grove-llm pick`
  evaluation (since for a stateless self-locating body, restart and
  loop-continuation are the same `pick`)? Confirm or refute with a source.
- Resolves the **Archon A–G mechanics**: (A) run/worktree lifecycle — fresh vs
  reusable worktree, can a run target a persistent branch so the committed
  `.grove/` tree survives an ephemeral worktree?; (B) restart/resume command &
  state storage; (C) how `until:` is evaluated + what `fresh_context` actually
  re-establishes + whether a `loop` body can contain `interactive`; (D)
  `interactive` pause/resume incl. across restart, and how the human responds;
  (E) `bash:`/`prompt:` node data-flow, whether a `prompt:` node runs a Claude
  Code SDK agent with the worktree as cwd able to call arbitrary CLIs like
  `grove-llm`; (F) exact YAML schema (all node fields), where workflows live,
  how one is invoked & distributed; (G) walk-away legibility.
- Resolves the **global-skill + backwards-compat distribution** mechanics
  (separable, pursued regardless of substrate): how a Claude Code skill installs
  **globally** (user-level dir / plugin / marketplace) instead of per-worktree
  materialisation; how one global skill **reads both** the old `NNN-slug/`
  directory format and the new dotted-decimal flat format so existing groves
  keep working with zero migration; what replaces the `grove install` /
  `VERSION.md` drift model.
- Includes a **walk-away check per candidate**: with the substrate uninstalled
  mid-grove, is the committed `.grove/` tree + git still fully legible, or does
  the substrate hide load-bearing state in a DB?
- **Ends with a recommendation** + a tradeoff matrix framed as *which complexity
  to own*: external dependency (Archon) vs brittle terminal config (iTerm) vs a
  small but real supervisor (PTY-wrap) vs autonomy-only (headless). Weigh
  grove's own history of shedding process machinery (trellis→rmux, ADR-0028)
  against the cost of an external dependency.

## Notes

- **Citation discipline (`driving.md`):** every capability claim cites a primary
  source by URL; quote the load-bearing bits; record "no primary source found"
  as an explicit finding rather than guessing. Training-data recall of Archon is
  known-stale (the project pivoted from MCP-knowledge-base to workflow-engine).
- This is research, not a build: read, don't run (Archon need not be installed
  to read its docs/source). A small local proof-of-concept is in-scope *only* if
  a doc claim genuinely can't be settled by reading — flag it if so.
- Primary sources to start from: archon.diy; coleam00/Archon README, docs,
  `.archon/workflows/` examples, and issues/discussions; iTerm2 trigger docs;
  Claude Code SDK + skills/global-install docs.
