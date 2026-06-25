# plan-k1

**Kind:** planning

## Goal

Scope and plan the `research-new-skills` workstream. Decide:

1. What *kinds* of new skills are in-scope (more languages? workflow skills?
   tooling skills? domain skills?).
2. The selection bar — what makes a candidate skill worth its standing
   description cost.
3. The research method — how candidates are sourced and evidenced.
4. The deliverable shape and how the tree is grown (likely a prior-art research
   leaf, then per-candidate authoring leaves).

Then grow the tree accordingly.

## Context

Repo is the `linkuistics` + `testanyware` skills marketplace. Existing 9 skills:
`coding-style` (universal) + 6 per-language style guides (rust/python/elixir/
bash/swift/typescript) + `cli-tool-design`, plus `using-testanyware`. Design
philosophy (README): skills load lazily; a skill's one-line description is the
only standing context cost, so each must earn its place.

## Done when

- Scope, selection bar, and research method are settled and recorded.
- Root `BRIEF.md` reflects the agreed goal/done-when.
- The tree is grown into concrete next leaves.

## Decisions (running log)

- **Q1 — Workstream goal (revised by user steer).** Reframed from "propose new
  skills broadly" to a **prior-art survey**: investigate specific external
  skill/agent repos and extract anything worth incorporating, split by **two
  targets** — the **skills** project (this repo) and the **grove** project
  (separate repo, `Linkuistics/grove`). Named seed sources:
  `github.com/nousresearch/hermes-agent` and `github.com/garrytan/gstack`,
  plus "any other major/popular skill repos." _(settled)_

- **Recon done (this session, light).** Initial WebFetch of the two seed repos:
  - **hermes-agent** (Nous Research, Python): persistent, multi-platform
    (CLI/Telegram/Discord/…) agent; *autonomous skill creation* + procedural
    memory (FTS5 + summarization); model-agnostic; terminal abstraction over
    6 backends; built-in NL cron. Compatible with the agentskills.io standard.
  - **gstack** (Garry Tan, TypeScript/Bun): 23+ slash-command "virtual eng
    team" skills enforcing a staged sprint pipeline (Think→Plan→Build→Review→
    Test→Ship→Reflect); outputs cascade between stages; browser/QA primitives;
    prompt-injection defenses; persistent memory via GBrain; multi-sprint
    "conductor" parallelism.
  - ⚠️ Fast-model summary returned dubious stats (star/commit counts) — verify
    against primary sources in the research leaf before any finding relies on
    them.

- **Q2 — Source set.** Comprehensive survey (all clusters but "just the two
  named"):
  - *Seed (named):* `nousresearch/hermes-agent`, `garrytan/gstack`.
  - *Core comparables:* `obra/superpowers`, `anthropics/skills`,
    `mattpocock/skills`, `addyosmani/agent-skills`.
  - *Breadth via awesome-lists:* `awesome-claude-code`,
    `awesome-claude-code-subagents` (VoltAgent), `wshobson/agents` — used to
    *discover* more high-signal repos, then triage.
  - *Adjacent ecosystems:* Cursor rules (`awesome-cursorrules`), aider,
    Continue, OpenClaw — transferable workflow/memory patterns (mostly for
    grove). _(settled)_

- **Q3 — Depth strategy.** Triage, then deep-dive top N. Two-pass:
  (1) a `shortlist-sources` leaf enumerates the universe and ranks each source
  by relevance to **skills | grove**; (2) deep-dive research leaves are added
  *lazily* only for the top-ranked sources; (3) a `synthesis` leaf splits the
  findings by target. One repo per fresh session. _(settled)_

- **Tree shape.** Grow `02-survey-prior-art` as a **node** whose first child is
  `01-shortlist-sources`. The shortlist session adds the deep-dive leaves and
  the synthesis leaf itself, once it knows which repos made the cut (lazy).
  Greenlit findings later become authoring leaves (skills repo) or
  recommendation hand-offs (grove repo). _(settled)_

- **No ADRs / PRD yet.** Decisions so far are process choices for *this* grove,
  not durable, hard-to-reverse architecture a future skills-repo reader needs.
  Bar not met (grilling.md). A glossary *is* warranted — the skills|grove
  target split is load-bearing across every survey leaf — so `CONTEXT.md`
  pins it.
