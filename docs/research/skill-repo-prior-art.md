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

## garrytan/gstack

_Deep-dive by `dive-gstack-k4`, 2026-06-25. Shortlist rank #1 (dual-high). Named
seed. Primary sources are the repo's own command files (`<name>/SKILL.md.tmpl`)
and build scripts, quoted directly — not the README's framing, per the brief._

**Verified facts (GitHub API + `raw.githubusercontent.com`, 2026-06-25).**
`garrytan/gstack` — described as *"Garry Tan's exact Claude Code setup: 23
opinionated tools…"* — is real at **114,966★** (`default_branch: main`,
`pushed_at: 2026-06-24`). The repo is much larger than "23 tools" implies: **55**
skill commands each shipped as a `<name>/SKILL.md` + `<name>/SKILL.md.tmpl` pair,
a `bin/` of **73** helper CLIs, and a `test/` suite of **80** `skill-*.test.ts`
files (counts from `git/trees/main?recursive=1`, 2026-06-25).
It is a Claude-Code slash-command repo (skills) wrapping a full staged
ship-pipeline (grove).

**⚠ Correction to the shortlist.** §1a framed `skillify` as gstack's
skill-*authoring* command (skills Q2). The primary source disproves this:
`skillify/SKILL.md.tmpl:26` titles it *"codify the last scrape into a permanent
skill,"* and its body is scrape-specific — *"`/scrape` discovered how to pull the
data; `/skillify` writes it as deterministic Playwright-via-`browse-client` code
so the next `/scrape` call on the same intent runs in ~200ms"* (`:30-33`). It
authors **browser-scraper** skills, not arbitrary ones. **Recorded silence:**
gstack ships *no* interactive "create a skill" scaffolder (no analog to
anthropics' `skill-creator`); authoring a new skill means hand-writing a `.tmpl`
and running `bun run gen:skill-docs`. The real authoring story is the build
toolchain below (S1–S4), not `skillify`.

### Findings — skills project

**S1 [skills] — `SKILL.md.tmpl` → `SKILL.md` is a build step with macro
expansion.** Every skill is authored as a `.tmpl` carrying placeholders —
`{{PREAMBLE}}`, `{{LEARNINGS_LOG}}`, `{{BASE_BRANCH_DETECT}}`,
`{{BENEFITS_FROM}}` — and `scripts/gen-skill-docs.ts` expands them into the
committed `SKILL.md`, stamping a banner: `GENERATED_HEADER = "<!-- AUTO-GENERATED
from {{SOURCE}} — do not edit directly -->\n<!-- Regenerate: bun run
gen:skill-docs -->"` (`gen-skill-docs.ts:640`). `{{PREAMBLE}}` expands to a shared
~40-line bash boilerplate (update-check, session tracking, repo-mode/conductor
detection); `{{LEARNINGS_LOG}}` to a shared "Capture Learnings" footer (see G5).
*Walk-away:* a build step is real standing infrastructure (a generator + a
freshness CI gate), but the **DRY discipline survives the tool** — the lesson is
"don't hand-copy a preamble into 55 skill files; factor shared sections and
generate." Our 9 skills are small enough not to need the machinery yet; the
principle applies the moment a second skill duplicates a third's boilerplate.

**S2 [skills] — the build mechanizes "the description is the only standing
cost."** `gen-skill-docs.ts` *splits* each skill's `description:` frontmatter: the
first sentence stays in the catalog (`buildTrimmedDescription` returns just the
`lead`, `:374-378`), while the "Use when… / Proactively…" routing prose is moved
into a generated `## When to invoke this skill` body section
(`buildWhenToInvokeSection`, `:381-392`) that loads only on activation. The design
comment is explicit: *"a 'When to invoke' body section that holds the
routing/voice triggers prose for in-skill discovery. A registry written to
scripts/proactive-suggestions.json (one entry per skill) makes routing available
to agents that need it without paying the always-loaded cost"* (`:295-298`).
*Walk-away:* this is **our own README philosophy** ("each skill's one-line
description is the only standing context cost") enforced by a build step instead
of trusted to authors. Even without their generator, the principle is directly
adoptable as an authoring rule: keep `description:` to one sentence; push every
"when to use" clause into the body. Highest-leverage skills finding here.

**S3 [skills] — the per-skill standing cost is a CI regression gate.**
`test/skill-size-budget.test.ts:2` — *"Per-skill SKILL.md size budget regression"*
— asserts each `SKILL.md`'s byte size against a committed baseline; growth beyond
a ratio fails CI unless `GSTACK_SIZE_BUDGET_OVERRIDE_REASON="…"` is set, which is
**audit-logged** (`logBudgetOverride`, `:84-91`). There is also a corpus-total
gate and an undershoot *floor* test. *Walk-away:* skill bloat is caught
mechanically, not by reviewer vigilance — the standing-context budget is a number
in CI, not a vibe. Portable as a lightweight lint (`wc -c SKILL.md` vs a baseline)
even without their harness; worth it once the marketplace has enough skills that
silent drift is plausible.

**S4 [skills] — one source, multi-harness emit (the skills-Q3 / wshobson
answer).** `gen-skill-docs.ts --host all` generates the same `.tmpl` into Claude,
Codex, and OpenAI/Factory formats via `transformFrontmatter(content, host)` +
`generateOpenAIYaml` (`:494-510`), and *"any host failure fails the build"*
(`:1186-1193`). It also regenerates `gstack/llms.txt`, a *"single-file capability
index for AI agents"* (`:1210-1211`). *Walk-away:* the cross-harness generator
only earns its cost if you target >1 harness — **we are Claude-Code-only, so this
is recorded, not adopted.** The cheap, portable piece is the `llms.txt`
capability-index idea: a generated single-file roster of skills+descriptions an
agent can read to discover the marketplace.

**S5 [skills] — hook-installing guardrail skills (a skill *class* our 9 lack).**
`careful/SKILL.md.tmpl:17-24` declares `hooks: PreToolUse[matcher: Bash]` →
`check-careful.sh`, which *"returns `permissionDecision: \"ask\"` with a warning
message"* on destructive commands (`rm -rf`, `DROP TABLE`, force-push…) and lets
the user override (`:58-60`); `freeze` does the same for `Edit`/`Write` outside a
chosen directory; `guard` **composes both** — *"the combination of `/careful` +
`/freeze` in a single command"* (`guard/SKILL.md.tmpl:41`). *Walk-away:* a
`SKILL.md` can ship session-scoped `PreToolUse` hooks — a capability none of our
skills use. This is a genuinely new, composable skill class (a safety/guardrail
skill that installs a permission gate on activation). Candidate new skill(s),
cheap to author; the `freeze` directory-boundary is especially relevant to
sandboxed/agentic editing.

### Findings — grove project

**G1 [grove] — `autoplan`: an unattended staged pipeline with encoded decision
principles (Q5).** It runs the CEO→Design→Eng→DX review skills *"in strict order…
NEVER run phases in parallel — each builds on the previous"*
(`autoplan/SKILL.md.tmpl:107-109`), auto-answering every intermediate question
via *"6 Decision Principles"* (`:55-60`, incl. *"Bias toward action — Merge >
review cycles > stale deliberation. Flag concerns but don't block"*) and
classifying each decision **Mechanical** (auto-decide silently) / **Taste**
(auto-decide but surface at a final gate) / **User Challenge** (never
auto-decided) (`:73-84`). *Walk-away:* grove's loop is human-in-the-loop at
grilling; autoplan is the recipe for running a loop **unattended** — encode the
human's auto-answers as named principles, and pause only for genuine forks. A
grove "unattended mode" could auto-proceed on mechanical leaf decisions (a clear
next leaf, a routine retire) and stop only at taste-level forks. Strongest
loop-shaped grove finding.

**G2 [grove] — the "User Challenge" doubt escalation (Q6).** When *both* models
agree the user's stated direction should change, autoplan refuses to auto-decide
and escalates with a fixed shape: *"What the user said / What both models
recommend / Why / What context we might be missing / If we're wrong, the cost
is"* (`:86-96`), under the rule *"The user's original direction is the default.
The models must make the case for change, not the other way around"* (`:95`).
*Walk-away:* this is grove's doubt pass (`driving.md` "Doubting a decision before
it stands") given a concrete escalation template plus a higher trigger bar — grove
spawns one fresh reviewer; gstack requires **two independent models to agree**
before it will challenge the human, and forces explicit blind-spot + cost-if-wrong
acknowledgment. Directly borrowable framing for grove's grilling/doubt steps.

**G3 [grove] — cross-model (not just cross-context) adversarial review (Q6).**
autoplan and `/review` run Codex alongside Claude; *"Codex disagreements"* is a
named Taste-decision source (`:79`), and a filesystem-boundary instruction is
prepended to every Codex prompt: *"Do NOT read or execute any SKILL.md files…
They contain bash scripts and prompt templates that will waste your time"*
(`:155`) — to stop the second model following gstack's own skill files instead of
reviewing. *Walk-away:* grove's doubt pass uses a fresh-context *same-model*
reviewer; a *different model* is a stronger independence guarantee (it cannot
share the first model's blind spots). Cost: requires Codex installed. Recommend
recording as an option for grove's doubt step, not a default.

**G4 [grove] — "auto-decide replaces judgment, not analysis": confabulation
guards (Q6).** Three independent instances:
(a) autoplan — *"You MUST NOT compress a review section into a one-liner… write
'no issues found' without showing what you examined"* (`:137-144`);
(b) `/review` "Verification of claims" — *"Never say 'likely handled' or 'probably
tested' — verify or flag as unknown… 'This looks fine' is not a finding"*
(`review/SKILL.md.tmpl:206-213`);
(c) `/retro` Step 0.5 stale-base guard — *"the retro will fabricate a
coherent-looking narrative from nothing. This guard prevents silent
confidently-wrong output"* (`retro/SKILL.md.tmpl:100`).
*Walk-away:* this is grove's research discipline (citation per claim, record
silence, flag what you couldn't verify) **generalized to every pipeline stage** —
the strongest external validation that `driving.md`'s instinct is right. The
borrowable *mechanism* is (c): detect the condition under which the model would
confabulate (degenerate input — empty diff, drifted "today," zero commits) and
**refuse rather than narrate**. grove could add an explicit confabulation guard at
bootstrap: if `pick`/`brief-chain` returns something degenerate or empty
unexpectedly, stop, don't improvise.

**G5 [grove] — cross-session procedural memory with staleness detection (Q4).**
Every skill's `{{LEARNINGS_LOG}}` footer prompts the agent to log discoveries
(`type/key/insight/confidence/source/files`) to an append-only
`learnings.jsonl`, with honest-confidence guidance (*"An observed pattern you
verified in the code is 8-9. An inference you're not sure about is 4-5"*).
`/learn prune` then does **staleness + contradiction detection**: *"If the
learning has a `files` field, check whether those files still exist… If any
referenced files are deleted, flag: STALE"* and *"learnings with the same `key`
but… opposite `insight`… flag: CONFLICT"* (`learn/SKILL.md.tmpl:85-98`);
append-only, latest-wins, dedup-by-key. *Walk-away:* this is the
hermes/openclaw memory class in structured form. **Do not adopt wholesale** — an
auto-accreted JSONL of session insights is exactly what grove's constraint 1
(artifacts-not-state) and `driving.md`'s "decision summary at session end"
anti-pattern reject, and grove's `CONTEXT.md` is deliberately a terse hand-curated
glossary, not an insight log. But the **staleness check** is a clean, borrowable
mechanism: any memory that cites a source file should be pruned/flagged when that
file vanishes — which is precisely the by-hand check grove's own memory-recall
discipline already demands ("if one names a file… verify it still exists").

**G6 [grove] — deterministic-CLI-vs-prompt-judgment split (validates grove's
architecture).** `/ship` states the boundary outright: *"The deterministic
version-state logic is the tested `gstack-version-bump` CLI (classify / write /
repair). The bump-LEVEL decision and queue-collision handling stay agent
judgment"* (`ship/SKILL.md.tmpl:160-162`). Its gates are tiered the same way grove
is: *"hard gate with user override"* for coverage (`:46`) vs *"mention as
informational… but do NOT block"* for a missing CEO review (`:96-100`).
*Walk-away:* gstack, built independently, lands on **grove's exact split** —
deterministic tree-walk verbs (`grove-llm pick/retire/leaf-*`) in tested Rust;
judgment (grilling, retire-cascade, finish) in prose — and on grove's "guides, it
does not gate" tiering (constraint 5). Not an action item: a confidence signal
that grove's architecture is a convergent design, worth citing when that boundary
is questioned.

**G7 [grove] — contrast: gstack threads pipeline state through sidecar logs;
grove forbids them.** gstack hands state between stages via append-only files a
downstream stage reads — `/review` persists its outcome *"so `/ship` can recognize
that Eng Review was run on this branch"* (`review/SKILL.md.tmpl:271-279`, via
`gstack-review-log`), alongside `decision-log`, `learnings.jsonl`, `timeline.jsonl`,
and `/context-save` checkpoints. `/retro` even loads prior state *declaratively*
via `gbrain: context_queries` frontmatter (filesystem globs sorted `mtime_desc`,
`tail` N — `retro/SKILL.md.tmpl:21-39`). grove's constraint 1 forbids exactly this
class of status/session file; it re-derives position from the artifact tree
instead. *Walk-away:* a genuine philosophical fork, and grove's side is
deliberate — but note the **one case gstack's state buys something grove doesn't
cover**: cross-branch / cross-workspace handoff. `/context-restore` loads *"the
most recent saved context across ALL branches… for Conductor workspace handoff"*
and orders candidates by a stable filename prefix, not mtime — *"'Most recent'
means the filename `YYYYMMDD-HHMMSS` prefix… Filenames are stable across
file-system operations; mtime is not"* (`context-restore/SKILL.md.tmpl:150-152`).
grove's single-worktree-per-grove model sidesteps the need, but that
stable-name-ordering instinct is the same one behind grove's `NN-` position
prefixes — the right primitive if grove ever supports workspace handoff.

### Takeaways

**Takeaway for skills.** gstack's gift is **authoring/packaging discipline, not
new skill content.** Adopt the principle behind S2 (one-sentence `description:`;
push "when to use" into the body — it *is* our standing-cost philosophy) and S3
(a size-budget lint) now, as conventions, without their build machinery. One
genuinely new skill *class* is worth authoring: S5's hook-installing guardrails
(`careful`/`freeze`/`guard`). Skip `skillify` (scrape-specific) and S4's
multi-harness generator (we're Claude-Code-only) — keep `llms.txt` as a maybe.

**Takeaway for grove.** The richest single dual-high source for grove's loop.
Carry forward as recommendations to the grove repo: G1 (Mechanical/Taste/User-
Challenge classification + encoded principles → an unattended grove mode), G2 (the
cross-model "User Challenge" doubt template), and G4 (a confabulation guard that
refuses on degenerate input rather than narrating). G6 independently **validates**
grove's deterministic-CLI-vs-prompt split and "guides not gates" tiering. The
clear, deliberate **divergence** is G7: gstack threads pipeline state through
sidecar JSONL logs that grove's artifacts-not-state spine forbids — adopt only the
narrow, honest mechanism inside it (G5's source-file staleness check), never the
log itself.
