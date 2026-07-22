# skill-design-k3

**Kind:** planning

## Goal

Digest `docs/research/jj-agent-prior-art.md` with the user and settle the
design of both skills, then grow the work leaves that build them.

## Context

Read the research report first; its **Synthesis for skill-design-k3** section
pre-answers much of the grilling. Decisions this leaf must land:

- **Adopt / adapt / write** — if a quality prior skill exists, adapt it
  (licence permitting) instead of writing from scratch.
- **Names** — canonical skill names for the workflow skill and the mapping
  skill (glossary working names; rename the CONTEXT.md entries once settled).
- **Triggers** — how each loads: the workflow skill by description on VCS
  work; the mapping skill on demand (user-invoked vs description-triggered).
- **Harness scope** — Claude Code only (cf. `guardrail`) or harness-neutral
  like the coding-style suite.
- **Native-workflow coverage** — which jj concepts the workflow skill must
  teach (working-copy-as-commit, `jj new`/`describe`, bookmarks, op-log undo,
  first-class conflicts, colocation etiquette), informed by the researched
  failure modes.
- **Reconciliation scope** — exact edits for `guardrail`, `decision-records`,
  `cli-tool-design`.

Then grow work leaves (expected: one per skill, one for reconciliation +
README/CHANGELOG) shaped as vertical slices.

## Done when

- Decisions logged inline; CONTEXT.md updated as terms resolve.
- Work leaves exist with briefs sharp enough to run AFK.
- Committed; this leaf retired.

## Decisions (running log)

- **Adopt/adapt/write:** WRITE both skills fresh in house style, adapting
  specific ideas from the MIT/Apache candidates with attribution
  (danverbraganza's agent-environment rules, RealAdarsh's colocated
  git-read-only policy, carbon-lang's symmetric detection, muloka's
  behavioural reframing, codex-jj-plugin's mapping table as the Mapping
  skill's seed). No wholesale vendoring — the best-shaped candidate is
  unlicensed and each licensed one is strong on only one axis.
- **Harness scope:** harness-NEUTRAL. The skills must integrate with the
  grove workflow (../grove), whose sessions run in multiple harnesses — at
  least Pi, Codex, and Claude Code, with Claude/Codex/Kimi models on
  subscriptions. Core = portable prose + shell probes (`jj root`, `git
  rev-parse` — every harness has a shell). Claude Code-specific mechanisms
  (PreToolUse hook, `allowed-tools` frontmatter) can only be optional
  extras, never load-bearing.
- **Enforcement:** per-harness enforcement recipes as a section in the
  workflow skill (precedent: doubt-driven-development's per-harness reviewer
  spawn recipes, commit 86facad). The neutral behaviour is the contract;
  each harness gets its optional guard setup where one exists (Claude Code
  PreToolUse deny-git recipe now, kawaz architecture re-implemented; other
  harnesses as discovered). Mitigates the documented prose-only failure
  mode without making any harness load-bearing.
- **Names:** `using-jujutsu` (workflow) + `git-to-jj-mapping` (reference).
  CONTEXT.md entries renamed inline; old working names kept as
  aliases-to-avoid.
- **Mapping-skill loading:** model-invoked with a tightly-scoped description
  ("git→jj command and concept mapping. Use when translating a specific git
  command or concept to jj"). One description line of standing context;
  portable; `using-jujutsu` cross-references it by name (never @path).
- **Trigger + detection:** (a) frontmatter description names git-shaped
  triggers (commit, branch, push, stash, merge, "version control", detached
  HEAD) so the skill fires before a wrong git command; (b) first action is
  the probe, vcs-detect ordering: `jj root` OK → jj-enabled; else `git
  rev-parse --show-toplevel` OK → git; else no VCS → silent. Prose + two
  commands, no shipped script; (c) carbon-lang symmetry both directions.
- **Colocation offer DROPPED (root-brief rework):** during grilling the
  user withdrew the offer-to-convert design entirely. New semantics =
  **symmetric VCS rule**: the repo's state alone picks the interface —
  jj-enabled → jj; not jj-enabled → git, silently; never convert, never
  offer, `jj git init` never required. Root brief and CONTEXT.md reworked
  in place. (Research's novel-design point on offer wording is moot; the
  verified `--colocate`-by-default fact stays in the research doc only.)
  Edge the work leaf must still cover: `.jj/` present but jj binary
  missing → colocated: fall back to git and say so; native: stop and tell
  the user.
- **Commit lane:** the two-verb native lane — `jj new` opens a change, work
  happens in `@`, `jj describe -m` records intent (early is fine; mitigates
  push-rejects-undescribed), `jj new` seals and opens the next. `jj commit
  -m` mentioned once as equivalent shorthand the skill deliberately doesn't
  use. One mental model (amend = keep editing `@`); embodies
  working-copy-is-a-commit; fits grove's one-task-one-commit sessions.
- **Native coverage:** the evidence-settled core (working-copy-is-a-commit
  reframing incl. "never ask 'want to commit?'"; one change per logical
  step — never squash-flattening; no staging area / `jj diff --git` /
  verify with `jj st`; non-interactive discipline: `--no-pager`, `-m`
  everywhere, no TUIs, resolve conflicts by editing files; bookmarks
  auto-follow rewrites but never advance to new changes, `bookmark
  move`/`set` before push, `--named` only for new; push rejects
  empty/undescribed; minimum revsets `@`, `@-`, `trunk()..@`, no `--limit`;
  op-log undo as safety net; destructive-command deny-list per rivet;
  colocated policy: git strictly read-only, jj for all mutations;
  commit-signing sandbox caveat) PLUS both optional sections: (1)
  workspaces — one workspace per concurrent agent, `jj workspace add`,
  explicit precedence over git-worktree skills in jj-enabled repos, worded
  as discipline not danger (the races claim was refuted); (2) sharing
  work — bookmark → `jj git push` → PR via gh, folded in, no separate PR
  skills.
- **Reconciliation scope:** (1) guardrail — add jj destructive patterns
  (`jj abandon`, `jj op restore`, plus any force-push form verified against
  jj 0.43) to hook script + SKILL.md table + tests; (2) decision-records —
  generalise "git holds the past/history" to "the VCS holds the past" at
  all 6 sites; (3) cli-tool-design — no edit ("like `git`" is an apt style
  example regardless of jj).
- **Grove integration:** tracked in THIS grove as a planning leaf
  (`grove-jj-integration`), but all edits/commits land in the
  Linkuistics/grove repo (~/Development/grove) — the house rule stands; the
  leaf is the task, not the location of the changes. Scope is foggy (prose
  says `git mv`/"git is the history"; grove-llm verbs shell out to git;
  `grove do` drives worktrees) → its own grilling later.
- **ADR:** exactly one — `docs/adr/symmetric-vcs-rule.md` (written this
  session). Records the rule, the rejected colocation-offer path with its
  reopen condition, and the rejected silent-conversion path. No other
  decision cleared the when-to-write bar; names/lane/coverage live in the
  skills themselves.
- **Tree grown (confirmed):** four root-level leaves —
  `build-using-jujutsu-k4` (work), `build-git-to-jj-mapping-k5` (work),
  `reconcile-and-announce-k6` (work), `grove-jj-integration-k7` (planning,
  HITL, edits land in ~/Development/grove). Order: core skill → companion →
  announce → grove, since the grove grilling needs the finished skills as
  baseline.

## Notes

HITL leaf — needs the user present for the grilling. Root-level semantics
were reworked mid-grilling by the user (see the running log's dropped
colocation-offer entry): the symmetric VCS rule in the root brief and
CONTEXT.md is now the settled form.
