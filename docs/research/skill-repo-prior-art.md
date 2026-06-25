# Skill / Agent Repo Prior-Art Survey

Survey of major/popular skill & agent-workflow repos, extracting **incorporable
findings** split by two extraction targets (see `CONTEXT.md`):

- **skills project** — this repo (`Linkuistics/skills`). A finding here becomes a
  candidate `SKILL.md` (new skill, authoring technique, or packaging change).
- **grove project** — the *separate* `Linkuistics/grove` repo. A finding here is a
  **recommendation only**, carried to that repo later; never implemented from
  this worktree.

This document is built incrementally by the `02-survey-prior-art-k2` node:
§1 below is the ranked source **shortlist** (the triage gate); per-source
deep-dives and the final cross-target **synthesis** are appended by later leaves.

---

## 1. Ranked source shortlist

_Produced by `shortlist-sources-k3`, 2026-06-25. Light triage only — READMEs,
repo structure, popularity signal — not full analysis._

**Verification note.** The brief flagged that an earlier fast-summarizer returned
dubious star/commit counts. Every star figure below was re-checked **first-party
against the GitHub REST API on 2026-06-25** (`api.github.com/repos/<owner>/<repo>`).
The figures held up — the ecosystem is simply far larger in mid-2026 than a
pre-2026 intuition expects (e.g. `obra/superpowers` at 238k, `openclaw/openclaw`
at 380k are real). One repo had moved: `K-Dense-AI/claude-scientific-skills` →
`K-Dense-AI/scientific-agent-skills` (301 redirect, re-resolved). Counts are
point-in-time and will drift; they are a relevance signal, not a headline.

### 1a. Greenlit for deep-dive — one `dive-*` leaf each

Ranked by blended signal (per-target novelty × actionability × popularity), best
first. The two named seeds (`gstack`, `hermes-agent`) are both in.

| # | Source | Stars | skills | grove | Why it ranks / what the dive must answer |
|---|--------|------:|:------:|:-----:|------------------------------------------|
| 1 | [garrytan/gstack](https://github.com/garrytan/gstack) | 114,963 | High | High | Named seed. Dual-high: ships `SKILL.md` + `SKILL.md.tmpl` + a `skillify` authoring command (skills Q2), *and* a fully-worked staged pipeline (`autoplan` → `plan-*-review` gates → `review`/`qa` → `ship` → `retro`) with `canary`/`careful`/`guard` verification gates (grove Q5/Q6). |
| 2 | [obra/superpowers](https://github.com/obra/superpowers) | 237,866 | High | High | The entire **workflow/process skill class** we lack (TDD-as-discipline, systematic-debugging, brainstorming, writing/executing-plans, verification-before-completion, dispatching-parallel-agents) + a `writing-skills` authoring meta-skill (skills Q1/Q2). Plan/verify/subagent skills map onto grove (Q5/Q6). **N.B. we already depend on it** — this session is using it — so the dive is "which to fork/adapt into our marketplace," not "discover it." |
| 3 | [addyosmani/agent-skills](https://github.com/addyosmani/agent-skills) | 66,455 | High | High | Genuinely novel skills with no equivalent here: **`doubt-driven-development`**, **`context-engineering`**, **`source-driven-development`** (cite-your-sources). Plus a coherent Define→Plan→Build→Verify→Review→Ship SDLC taxonomy (grove Q5/Q6; skills Q1). |
| 4 | [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent) | 202,144 | Med | High | Named seed. Self-improving loop: **creates skills from experience**, cross-session procedural memory, trajectory compression, routines/cron (grove Q4). Skill auto-authoring transfers to skills-side (Q2). Non-Claude-Code (Python) format — adopt ideas, not files. |
| 5 | [eyaltoledano/claude-task-master](https://github.com/eyaltoledano/claude-task-master) | 27,683 | Low | High | The **closest external analog to grove's task-tree**: decomposes a PRD into persisted task/subtask files (`.taskmaster/`), dependency graph, status tracking, "next task" surface, cross-session continuity. Direct design comparison for grove (Q4/Q5) — punches above its stars. |
| 6 | [openclaw/openclaw](https://github.com/openclaw/openclaw) | 380,319 | Low | High | Tiered **file-based memory** with "no hidden state — memory is files on disk" (= grove's git-tracked-tree philosophy): `SOUL.md`/`IDENTITY.md` (immutable) → `MEMORY.md` (durable, session-start) → `memory/YYYY-MM-DD.md` (running notes), plus on-demand `memory_search`/`memory_get` retrieval rather than front-loading (grove Q4). |
| 7 | [mattpocock/skills](https://github.com/mattpocock/skills) | 144,986 | High | Med | Novel skill candidates: **`domain-modeling`**, **`codebase-design`**, `improve-codebase-architecture` (design-craft beyond our style guides). Authoring techniques: the **user-invoked vs model-invoked** skill split, `writing-great-skills`. `grilling`/`handoff` map to grove doubt/verification + session resumability (grove Q4/Q6). |
| 8 | [anthropics/skills](https://github.com/anthropics/skills) | 154,814 | High | Low | The **authoritative authoring/packaging reference**: the Agent Skills `spec/`, a `template/`, and the `skill-creator` skill (frontmatter, progressive disclosure, packaging). Dive **scoped to spec/template/skill-creator** (skills Q2); skip the document/design domain skills (docx/pptx/etc.). |
| 9 | [wshobson/agents](https://github.com/wshobson/agents) | 37,148 | High | Med | The only strong answer to skills Q3 (**packaging/distribution**): a single-Markdown-source → **multi-harness** artifact generator (Claude Code / Codex / Copilot) over a real corpus (84 plugins / 156 skills / 16 orchestrators). Even a "we should stay Claude-Code-only" finding is worth recording. |

### 1b. Examined but **not** deep-dived — synthesis mentions (record the silence)

Each was triaged out of its own session; the one-liner is its candidate finding
for the synthesis leaf to fold in (or explicitly drop).

| Source | Stars | Why no dive — and the note it leaves for synthesis |
|--------|------:|----------------------------------------------------|
| [continuedev/continue](https://github.com/continuedev/continue) | 34,433 | Closest external **mirror of our skill model** (`.continue/rules/*.md` with `globs`/`description`/`alwaysApply`, three inclusion pathways) + `create_rule_block` (agent authors its own rule mid-session = self-authoring) + "blocks"/hub distribution. **But the README states it is no longer maintained / read-only**, so it's prior-art-as-reference, not a live target. Capture the rules-inclusion model (skills) + `create_rule_block` (grove self-authoring) as notes. |
| [modu-ai/moai-adk](https://github.com/modu-ai/moai-adk) | 1,095 | grove triple-hit (Plan→Run→Sync pipeline + dedicated `plan-auditor`/`sync-auditor` review agents + `progress.md` resumability + TRUST-5 gates) but small and **conceptually overlaps gstack**. Note `progress.md` resume vs grove's "artifacts, not state" as a contrast in synthesis. |
| [backnotprop/plannotator](https://github.com/backnotprop/plannotator) | 6,488 | A concrete **human-in-the-loop plan-review gate**: intercepts `ExitPlanMode` via a permission hook, shows plan/diff for annotation, returns structured approve/deny. Mechanism note for grove's doubt/review stage; narrow (one mechanism), so not a full session. |
| [Aider-AI/aider](https://github.com/Aider-AI/aider) | 46,669 | **repo-map** (auto-ranked compressed codebase map = cheap re-orientation when resuming long work → grove) + **`CONVENTIONS.md` read-only via `/read` + prompt-cache-eligible** (immutable standing rules separated from mutable context → skills packaging). Two narrow patterns; fold into synthesis, no session. |
| [PatrickJS/awesome-cursorrules](https://github.com/PatrickJS/awesome-cursorrules) | 40,083 | **Validates our cost model**: `.cursorrules`/`.mdc` frontmatter = `description` + `globs` + `alwaysApply` (= description-is-the-standing-cost, glob/flag decides loading). Value is the corpus + 13-category taxonomy, not a new mechanism — already understood, so a note not a session. |
| [trailofbits/skills](https://github.com/trailofbits/skills) | 5,853 | ~40 professionally-authored **security/audit** skills incl. a "Verification" cluster. A candidate new-domain source + authoring-quality benchmark, but security is outside our coding-craft focus; synthesis mention. |
| [K-Dense-AI/scientific-agent-skills](https://github.com/K-Dense-AI/scientific-agent-skills) | 29,262 | Large (147) well-structured `SKILL.md` corpus, but **scientific domain is out of scope** for our coding-craft marketplace. Sample for authoring/structure conventions only if the anthropics/wshobson authoring dives want more examples. (Repo moved from `claude-scientific-skills`.) |
| [pchalasani/claude-code-tools](https://github.com/pchalasani/claude-code-tools) | 1,931 | Session-continuity **tooling**: full-text session search, cross-agent handoff, live-state "agent tunnel". Memory/resumability primitives for grove, but tooling-heavy and lower-leverage than task-master/hermes/openclaw; light synthesis mention. |

### 1c. Indexes mined (pointers, not sources)

Triaged as indexes — value is the repos they point to (mined into 1a/1b above),
not the list itself; no deep-dive.

| Index | Stars | Yield |
|-------|------:|-------|
| [hesreallyhim/awesome-claude-code](https://github.com/hesreallyhim/awesome-claude-code) | 47,233 | Flagship Claude-Code index. Catalog now in `THE_RESOURCES_TABLE.csv` (README mid-reorg) — that CSV is the real mining surface. Surfaced task-master, plannotator, trailofbits, pchalasani, K-Dense. |
| [wshobson/agents](https://github.com/wshobson/agents) | 37,148 | Both an index *and* a greenlit source (1a #9) — its corpus + generation pipeline are the finding. |
| [VoltAgent/awesome-claude-code-subagents](https://github.com/VoltAgent/awesome-claude-code-subagents) | 22,352 | 100+ subagent definitions in 10 categories incl. a meta-orchestration cluster. Agent files are thin; useful only as a pointer to orchestration kits (e.g. moai-adk). No dive. |

---

<!-- deep-dives appended below by dive-* leaves; synthesis appended last -->
