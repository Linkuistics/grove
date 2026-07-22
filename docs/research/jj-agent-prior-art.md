# Prior art: making coding agents use Jujutsu (jj) instead of git

Research for `skill-design-k3`. Surveyed 2026-07-22. Method: direct GitHub
code/repo search (`gh search code`, `gh api`, raw-file reads) plus a
five-angle web-search sweep whose claims were adversarially verified (3-vote
panels; 24 of 25 top claims confirmed, 1 refuted). Every works/fails claim
carries a primary-source URL; where the search came up silent, that is
recorded — silence is a finding.

## Q1 — Published jj skills and plugins

A real ecosystem exists — every packaging style we might use is already
occupied — but it is young (created 2026-01 through 2026-07) and thin (only
one artifact above 11 stars). The table lists every candidate found, with
the walk-away check (licence; could we vendor/adapt into this repo's
`plugins/linkuistics/skills/` layout).

| Candidate | Form | Licence | Vendorable? | Character |
|---|---|---|---|---|
| [danverbraganza/jujutsu-skill](https://github.com/danverbraganza/jujutsu-skill) | single SKILL.md, 55★/11 forks (highest adoption found) | MIT | **Yes** | Native workflow; richest agent-hygiene + failure-mode content; tested against jj v0.37.0 |
| [HotThoughts/jj-skills](https://github.com/HotThoughts/jj-skills) | Claude Code plugin (`.claude-plugin/` + `skills/`), three skills (`jj-workflow`, `jj-create-pr`, `jj-update-pr`) | MIT | **Yes** — lowest-friction layout match | Pure native (no git-mapping table at all); PR-lifecycle skills are a scope idea we lack |
| [kawaz/claude-plugin-jj](https://github.com/kawaz/claude-plugin-jj) | Claude Code marketplace plugin: **PreToolUse hook + skill + agent** | MIT | Architecture yes; prose is Japanese — re-implement, don't copy | `jj-guard.sh` hook denies git Bash commands when `.jj/` exists — the only mechanism found that doesn't depend on model compliance |
| [RealAdarsh/jj-skill](https://github.com/RealAdarsh/jj-skill) | cross-agent: canonical SKILL.md + per-runtime adapters (CLAUDE.md, AGENTS.md, GEMINI.md, `agents/openai.yaml`) | MIT | **Yes** | Native loop (`jj new`/`describe`/`absorb`/`squash`); best colocated-repo *policy* text: "use jj for mutations and keep git usage read-only unless the user explicitly requests otherwise" |
| [megumish/jj-skill](https://github.com/megumish/jj-skill) | standalone repo on the [agentskills.io](https://agentskills.io) standard (Claude Code, Cursor, Gemini CLI, Copilot, Windsurf), 4★ | Apache-2.0 | **Yes** (attribution + licence retention) | Native + auxiliary mapping table; opinionated prohibitions (see the `jj commit` tension below); `.min-jj-version` 0.41.0 |
| [netresearch/jujutsu-workflow-skill](https://github.com/netresearch/jujutsu-workflow-skill) | Claude Code plugin layout, v0.2.0 (2026-07-13) | dual MIT + CC-BY-SA-4.0 — check per-file before copying prose | jj-as-local-layer / git-as-remote-interface framing; ships a tri-state detection script (see Q4); prescribes one-workspace-per-agent |
| [mtaran/jj-guide](https://github.com/mtaran/jj-guide) | SKILL.md + `references/` (git-to-jj, revsets, bookmarks, conflicts, op-log, troubleshooting), 8★ | MIT | **Yes** | Both native and translation; the licensed twin of the reference-directory shape below |
| [carbon-language/carbon-lang `.agents/skills/jj/SKILL.md`](https://github.com/carbon-language/carbon-lang/blob/trunk/.agents/skills/jj/SKILL.md) | in-repo skill of a 33.8k★ project | Apache-2.0 WITH LLVM-exception (file header) | Yes | Minimal native usage; the cleanest *symmetric* detection rule found |
| [jaredramirez/codex-jj-plugin `jj-guide`](https://github.com/jaredramirez/codex-jj-plugin/blob/main/plugins/jj/skills/jj-guide/SKILL.md) | Codex plugin, 0★, active 2026-07 | MIT | **Yes** | Best compact git→jj table; covers newer verbs (`absorb`, `parallelize`, `metaedit`, `jj bisect run`, `jj file search`); two-tier `jj-guide`/`jj-expert` split |
| [muloka/claude-plugins `project-setup-jj`](https://github.com/muloka/claude-plugins/tree/main/plugins/project-setup-jj) | plugin that *injects* a jj section into a project's CLAUDE.md | Apache-2.0 (plugin dir) | Yes | Setup/injection approach; includes superpowers-skill overrides (see Q2) |
| [antstanley/jj-workspace-skill](https://github.com/antstanley/jj-workspace-skill) | plugin/marketplace layout, worktree-interception scope only | **none** (API `license: null`, no grant in tree) | **No** — all-rights-reserved; the *mechanism* (probe + precedence claim over `superpowers:using-git-worktrees`) is unprotectable and free to re-implement | Pure git→jj translation for worktrees only, deliberately |
| [martinemde/marketplace `use-jj-not-git`](https://github.com/martinemde/marketplace/blob/main/plugins/skills/use-jj-not-git/SKILL.md) | Claude Code marketplace plugin skill, 3★ | **none found** | No | Native workflow + 13 `references/` files incl. full git→jj mapping — the shape to imitate, via mtaran's licensed equivalent |
| [cryfs/cryfs `.claude/skills/jujutsu/SKILL.md`](https://github.com/cryfs/cryfs/blob/main/.claude/skills/jujutsu/SKILL.md) | in-repo skill | repo LGPL | content trivial to re-derive | Compact tables; colocated framing |
| [causes-tracker `jj-for-claude.md`](https://github.com/causes-tracker/causes-tracker/blob/master/.claude/skills/jj/references/jj-for-claude.md) | in-repo skill reference | none stated | No — but its failure *catalogue* is factual content | "Common git-brain mistakes": 10 concrete agent failure modes (Q3) |
| [AdityaZxxx/dotfiles `vcs-detect`](https://github.com/AdityaZxxx/dotfiles/blob/master/home/.config/opencode/skill/vcs-detect/SKILL.md) | opencode skill | none stated | No; probe logic re-derivable | Detection-first design (Q4) |
| [ruvnet/agentic-flow `agentic-jujutsu`](https://github.com/ruvnet/agentic-flow/blob/main/.claude/skills/agentic-jujutsu/SKILL.md) | skill inside a framework, mirrored into several registries | repo MIT | not worth it | **Avoid**: marketing copy ("quantum-resistant", "23x faster than Git"); a JS wrapper API, not workflow guidance |
| dotfiles-embedded long tail: [relekang](https://github.com/relekang/dotfiles), [adampetrovic](https://github.com/adampetrovic/dotfiles), [sm17p `jj-push`](https://github.com/sm17p/dotfiles), [DerGernTod](https://github.com/DerGernTod/pathfinder-combat-pad) | personal | various/none | No | Confirms the pattern is widespread; adds no new ideas |

Cross-cutting observations:

- **Native vs translation:** every serious general-purpose candidate teaches
  *native* jj first, with the git→jj table secondary or absent; only
  antstanley is a pure translation layer, deliberately scoped to worktrees.
  Our root-brief split (workflow skill vs on-demand mapping) matches how the
  best prior art organises itself internally (mapping pushed to
  `references/` in use-jj-not-git, mtaran, codex-jj-plugin).
- **Two-tier precedent:** codex-jj-plugin splits daily-driver `jj-guide`
  from `jj-expert` ("revset, fileset, template, conflict, divergence, or
  bookmark troubleshooting") — the same resident/on-demand split as our
  Workflow/Mapping design. kawaz layers hook → skill → expert agent.
- **Content-quality caveats found in verification:** HotThoughts presents
  `jj tug` (a user alias) as a built-in command, and both HotThoughts and
  danverbraganza overstate `jj split`'s interactivity (fileset arguments
  work non-interactively). Adapt critically, not verbatim.
- Skill-registry mirrors (majiayu000/claude-skill-registry, X-Skills,
  skillsh scrapes) carry copies of the above, not independent work. No
  agentskills.io-exclusive listing was found beyond megumish.

## Q2 — What people put in CLAUDE.md / AGENTS.md to force jj

### The escalation ladder (as written)

Prior-art forcing mechanisms, weakest to strongest:

1. **Cheat-sheet only, no imperative** —
   [alper's CLAUDE.md gist](https://gist.github.com/alper/7035e19ebe40e32be8a94bb2768d6ffe):
   a jj command reference with no "always/never" phrasing at all.
2. **Blunt prose prohibition + corruption warning** —
   [megumish](https://github.com/megumish/jj-skill/blob/main/skills/jj/SKILL.md):
   "This repository uses **jujutsu (jj)** … NOT git. **NEVER use git
   commands** … can corrupt the repository state."
   [NovyWave](https://github.com/NovyWave/NovyWave/blob/main/CLAUDE.md) is
   the ALL-CAPS variant with ✅/❌ command blocks.
   [hans.lhoest.eu](https://hans.lhoest.eu/how-to-use-jujutsu-jj-with-claude-code)
   leads with "**CRITICAL: This repository uses Jujutsu (jj), not git**" +
   mappings, reported by the author as "the quick solution that works for
   most users".
3. **Conditional rule, both directions** —
   [carbon-lang](https://github.com/carbon-language/carbon-lang/blob/trunk/.agents/skills/jj/SKILL.md):
   "check for a `.jj` directory … If present, you **must** use `jj` and
   **must not** use `git`. If absent, you **must not** use `jj`." The only
   candidate that also forbids jj in non-jj checkouts.
4. **Behavioural reframing, not just command substitution** —
   [muloka's CLAUDE.md template](https://github.com/muloka/claude-plugins/blob/main/plugins/project-setup-jj/templates/CLAUDE.md.template):
   "the working copy IS a commit. There is no uncommitted state. Never ask
   'want to commit?' … The only meaningful checkpoint questions are 'want
   to start a new change?', 'want to describe this change?', or 'want to
   push?'" Also **overrides superpowers skills** that assume git
   (`finishing-a-development-branch` → a jj-native `/finish`) — direct
   precedent for our reconciliation pass.
   [rivet-dev/rivet](https://github.com/rivet-dev/rivet/blob/main/CLAUDE.md)
   (production OSS) adds a process mandate — "**MUST run `jj new` before
   making any file edits** … before reading, before planning, before
   editing" — plus a destructive-command deny-list (`jj git push`,
   `jj abandon`, `jj op restore`, `jj rebase -d main`, `git reset --hard`
   … "unless the user explicitly requests it").
5. **Frontmatter engineered to win skill selection** —
   [danverbraganza](https://github.com/danverbraganza/jujutsu-skill/blob/main/jujutsu/SKILL.md):
   description reads "**REQUIRED** — Always activate FIRST on any git/VCS
   operations … especially when HEAD is detached … raw git commands can
   corrupt data … DO NOT IGNORE" (detached HEAD being the tell-tale state
   of a colocated jj repo seen through git eyes).
6. **Harness-level tool restriction** — danverbraganza's
   `allowed-tools: Bash(jj *)` in the skill frontmatter.
7. **Hard mechanical enforcement** —
   [kawaz's `jj-guard.sh`](https://github.com/kawaz/claude-plugin-jj)
   PreToolUse hook (matcher `Bash`): `[[ ! -d ".jj" ]] && exit 0`,
   otherwise emits `permissionDecision: "deny"` for git commands. Layered
   as "Hook (Protection) → Skill (Guide) → Agent (Expert)".
8. **Delegate-to-skill one-liner** —
   [wincent](https://github.com/wincent/wincent/blob/main/.claude/CLAUDE.md):
   "This repository uses the Jujutsu version control system (see the
   `/jujutsu` skill for details)" — near-zero standing context; matches our
   auto-firing-skill design better than inlining rules.

### What reportedly works or fails

Efficacy evidence is thin and *contested* — no candidate ships test data:

- **Fails:** the author of jj upstream FR
  [#8780](https://github.com/jj-vcs/jj/issues/8780): "I tried to add a
  'this repo uses jj don't use git' to the AGENTS.md. **Which does not
  work well.**" (The user's seed-question scenario, answered from the
  field.)
- **Fails (harness-level):** Claude Code issue
  [#41435](https://github.com/anthropics/claude-code/issues/41435): in
  colocated repos the built-in environment metadata says only "Is a git
  repository: true", and "the model treats this as authoritative system
  context and defaults to git commands, **even when users have configured
  CLAUDE.md instructions, SessionStart hooks, and PreToolUse guards**"
  (closed as duplicate; unresolved as surveyed). Claude Code issue
  [#56865](https://github.com/anthropics/claude-code/issues/56865) shows
  the same shape on Claude Code Web: a project CLAUDE.md rule forbidding
  auto-commits inconsistently loses to an injected platform prompt.
- **Works (claimed, unvalidated):**
  [Amplify Partners](https://www.amplifypartners.com/blog-posts/will-agents-like-git-any-more-than-we-do)
  asserts "Simply adding 'always use jj, never use Git' to a global
  CLAUDE.md is sufficient", with no supporting evidence; hans.lhoest.eu
  reports its CRITICAL-block works "for most users". Author self-reports,
  not tests.
- **Opt-out entirely:**
  [panozzaj](https://www.panozzaj.com/blog/2025/11/22/avoid-losing-work-with-jujutsu-jj-for-ai-coding-agents/)
  inverts the goal: let agents keep using git while jj colocation
  passively snapshots everything as an uncontaminated safety net — a third
  strategy (neither force nor translate) worth acknowledging in design.

The honest summary: *prose-only forcing is documented to fail at least
sometimes; nothing stronger than kawaz's hook removes the dependence on
model compliance; nobody has published a controlled comparison.*

## Q3 — Reported failure modes when agents drive jj

### The one genuine field post-mortem

[2389-research/agentjj](https://github.com/2389-research/agentjj) (MIT,
archived 2026-02-17) embedded jj as its agent-VCS engine and removed it —
"by v0.3.1, bug fixes had independently migrated three operations from jj
to git". Its two documented jj-specific failures:

1. **Squash-based commit mapping flattens agent history**: "An agent doing
   multi-step work — implement feature, write tests, update docs — ended up
   with one fat squashed commit instead of three." A design-choice failure
   (their 'commit' squashed `@` into its parent), not jj misbehaving — our
   skill avoids it by teaching one `jj new`/`describe` per logical step.
2. **Colocated desync compounds git fallback**: "Files tracked by jj
   appeared 'deleted' in git's staging area. Agents that fell back to git
   saw impossible state." Git fallback in colocated repos is thus not just
   a style violation — it compounds.

A third agentjj-derived claim — that jj's single-working-copy model caused
concurrent-subagent races where one agent's commit captured both agents'
changes — was **refuted in verification (1-2)** and must not be cited. The
safe framing: one working copy per workspace requires one-workspace-per-
agent discipline (netresearch's skill prescribes exactly that).

### Failure catalogues embedded in skills (convergent evidence)

From [causes-tracker's "Common git-brain mistakes"](https://github.com/causes-tracker/causes-tracker/blob/master/.claude/skills/jj/references/jj-for-claude.md)
(10 numbered mistakes, each with its correction):

1. Running raw git commands (bypasses the op log; corruption risk).
2. Hunting for a staging area / `jj add`; misreading `jj diff` as
   `git diff --staged`.
3. Passing `--limit`/`-n` to `jj log` (not a jj flag) instead of revsets.
4. Manual `jj bookmark set` after a rewrite (bookmarks auto-follow).
5. `jj git push --named` on an *existing* bookmark (creates a conflict).
6. Looking for `jj commit` where the house workflow is describe→new.
7. Losing the scratch position after `jj rebase -r @ -A … -B …`.
8. `jj abandon` on a conflicted commit (throws away the conflict state).
9. `-s` vs `-r` confusion in rebase graph surgery.
10. Redundant `jj config set` in a pre-configured environment.

From [danverbraganza](https://github.com/danverbraganza/jujutsu-skill/blob/main/jujutsu/SKILL.md)
and [HotThoughts](https://github.com/HotThoughts/jj-skills), independently
converging (verified verbatim with line numbers):

- **Interactive UI hangs**: editors opened by bare `jj desc`/`jj squash`
  (always `-m`), pagers (always `--no-pager`; also carbon-lang), `jj
  squash -i`/`jj split`/`jj resolve` TUIs banned with non-interactive
  substitutes. (Both overstate `jj split` — fileset args work
  non-interactively — but the ban is the prior-art norm.)
- **`jj git push` rejects undescribed/empty changes** (HotThoughts;
  megumish's troubleshooting: find and `jj abandon` the empty change).
- **Commit-signing config can make jj fail in sandboxes** (HotThoughts).
- **Bookmark non-advancement**: "Unlike git branches, jj bookmarks do not
  automatically move when you create new commits. You must manually update
  them before pushing" (danverbraganza). Upstream confirms it's by design —
  a maintainer explains jj deliberately has no "current bookmark" and
  normalises what git calls detached HEAD
  ([jj discussion #6832](https://github.com/jj-vcs/jj/discussions/6832)).
  Note the causes-tracker doc warns against the *opposite*
  over-correction (pointless `bookmark set` after rewrites); both are
  real — bookmarks follow rewrites of the same change but never advance to
  new changes.
- **`jj diff` default format misread as corruption**: "This is normal and
  correct — it is NOT corrupted"; prescribe `jj diff --git`
  (danverbraganza).
- **Workspace races**: stale working copies when another workspace
  rewrites your `@`; divergent commits (`xyz??`) if the stale side had
  un-snapshotted edits; "Don't `jj edit` a change another workspace
  already has as its `@`" (danverbraganza).

### Harness-side failures (Claude Code specific)

- [claude-code#27466](https://github.com/anthropics/claude-code/issues/27466):
  `claude --worktree` **silently** fails in jj-colocated repos — no error,
  the session just starts in the main repo.
- [claude-code#41435](https://github.com/anthropics/claude-code/issues/41435):
  environment metadata reports colocated repos as plain git (see Q2).
- Upstream [#9814](https://github.com/jj-vcs/jj/issues/9814): without
  non-interactive hunk selection, agents "will perform these operations in
  an inefficient and error-prone manner, e.g. using `jj new` or `jj edit`
  and replaying changes from the context window line-by-line".

### A design tension prior art does not settle

**`jj commit` pro vs con.** carbon-lang: "Prefer using `jj commit` over the
combination of `jj describe` and `jj new`". megumish: `jj commit`
**PROHIBITED** ("splits working copy in confusing ways"), mandating
`jj new` + `jj describe`. danverbraganza: "There is no need to run `jj
commit`" (describe-first). rivet: `jj new` *before* any edit.
`skill-design-k3` must pick a lane and say why.

## Q4 — Detection heuristics and colocation offers

Detection converges on three heuristics, all found as primary sources:

- **`.jj/` directory existence** — kawaz's hook checks it *mechanically*
  (`[[ ! -d ".jj" ]] && exit 0` — the only executable check surveyed);
  danverbraganza states it as prose in the frontmatter description (read at
  skill-selection time, before any command); carbon-lang's rule is the
  symmetric version (absence forbids jj).
- **`jj root` exit code** — RealAdarsh ("Run `jj root` to confirm Jujutsu
  context"); [vcs-detect](https://github.com/AdityaZxxx/dotfiles/blob/master/home/.config/opencode/skill/vcs-detect/SKILL.md)
  gives the priority order: `jj root` succeeds → jj (handles colocation,
  both probes walk up the filesystem); else `git rev-parse --show-toplevel`
  → git; else no VCS.
- **`jj workspace root` exit code** — antstanley: exit 0 → use the jj
  skill; non-zero + `.git` exists → defer to
  `superpowers:using-git-worktrees` (an explicit skill-precedence
  declaration).
- **Tri-state script** — netresearch ships
  `scripts/detect_jj_state.sh`: `.jj/` only = native jj; `.jj/` + `.git/`
  = colocated; `.git/` only = plain git.

Colocation:

- **No prior art implements our ask-first colocation offer.**
  Adversarially confirmed: RealAdarsh's `colocated-git.md` covers only
  already-colocated repos (no conversion path); megumish has *no*
  repo-state detection at all (pure keyword trigger, installed
  user-globally). **No primary source found** for offer-to-colocate — that
  part of our design is novel and must be written fresh.
- The nearest thing is [rivet's CLAUDE.md](https://github.com/rivet-dev/rivet/blob/main/CLAUDE.md),
  which *mandates* silent conversion: "check whether jj is initialized by
  running `jj status`. If it fails … run `jj git init --colocate` … Do NOT
  run `jj git init` without `--colocate`." Note a version wrinkle for
  `skill-design-k3`: current upstream docs state `jj git init` in an
  existing git repo colocates *by default* (`--no-colocate` to disable)
  ([git-compatibility docs](https://docs.jj-vcs.dev/latest/git-compatibility/));
  verify against our local jj 0.43 before wording the offer.

## Q5 — What jj upstream provides for agents

- **Nothing in-tree, and deliberately so.** The jj-vcs/jj repo root has no
  CLAUDE.md, no AGENTS.md, and no AI/agent page under `docs/` (checked
  directly 2026-07-22). FR
  [#8780 "add a skills.md file to jj repo (ai documentation)"](https://github.com/jj-vcs/jj/issues/8780)
  (2026-02) asked for exactly what we're building; long-time contributor
  PhilipMetzger stated it belongs in community spaces (a blog post or
  [Necior/awesome-jj](https://github.com/Necior/awesome-jj)), and the issue
  closed with the author opening an awesome-jj PR instead. As surveyed,
  awesome-jj itself still lists **zero** agent/AI entries — the
  "community home" is empty too.
- **Agent needs folded into general scriptability:**
  [#9814](https://github.com/jj-vcs/jj/issues/9814) (agent-friendly
  split/squash interfaces) closed as duplicate of
  [#8218](https://github.com/jj-vcs/jj/issues/8218) (programmatic hunk
  selection, open) — maintainer yuja: "I don't think agents are quite
  different from programs to select hunks."
  [#9561](https://github.com/jj-vcs/jj/issues/9561) (`jj asc`, syncing AI
  agent sessions) is open and unimplemented.
- **Community stopgaps, all immature:**
  [laulauland/jj-hunk](https://github.com/laulauland/jj-hunk)
  (non-interactive hunk selection, cited in #9814);
  [kmarxican/jj-mcp](https://github.com/kmarxican/jj-mcp) (MCP server,
  MIT, 1★) and [solle458/jujutsu-mcp](https://github.com/solle458/jujutsu-mcp)
  (0★) — too immature to depend on, and an MCP server is the wrong shape
  for us anyway (a skill teaching a CLI needs no server process).

Net: assume no upstream-blessed agent guidance exists to adopt, and none is
coming — upstream has explicitly deferred this space to exactly the kind of
artifact we are about to write.

## Synthesis for skill-design-k3

**Recommendation: WRITE our own two skills, ADAPTING freely from the
MIT/Apache candidates; vendor nothing wholesale.**

Why not adopt: the ecosystem is real but immature (all candidates 2026,
six of seven single-author, only danverbraganza above 11 stars), and no
single artifact combines executable detection + enforcement + full native
workflow + colocated policy + a colocation offer — each is strong on
exactly one axis. The unlicensed items (use-jj-not-git, antstanley,
causes-tracker, vcs-detect) cannot be vendored; their *ideas* (reference
layout, probe ordering, precedence-over-worktrees, failure catalogue) are
unprotectable facts to re-implement with independently written prose.

What to adapt, with attribution: danverbraganza's agent-environment rules
(`--no-pager`, `-m` everywhere, no interactive TUIs, `jj diff --git`,
verify with `jj st`); HotThoughts' plugin layout and PR-lifecycle scope
idea; RealAdarsh's colocated git-read-only policy line; carbon-lang's
symmetric detection rule; muloka's behavioural reframing ("never ask 'want
to commit?'"); kawaz's hook *architecture* (re-implemented — its prose is
Japanese); codex-jj-plugin's mapping table as the Mapping skill's seed.

Specific answers for the planning leaf:

1. **Skill names.** Prior art trends imperative/descriptive
   (`use-jj-not-git`, `vcs-detect`, `jj-workflow`, `jj-guide`/`jj-expert`).
   Working proposal: `using-jujutsu` (workflow — matches the house
   `using-*` convention) and `git-to-jj-mapping` (mapping). Settle in
   grilling.
2. **Trigger/detection: layer it.** (a) Frontmatter description names the
   git-shaped triggers (commit, branch, push, stash, "version control",
   detached HEAD) so the skill fires *before* a wrong git command —
   danverbraganza's is the strongest prose trigger surveyed; (b) first
   action inside the skill is an executable probe — prefer `jj root` →
   `git rev-parse` ordering (vcs-detect) or netresearch's tri-state over a
   bare `.jj/` stat, keeping carbon-lang's symmetry (no `.jj/`, no accepted
   offer → git, silently); (c) *optionally*, and worth grilling: a
   kawaz-style PreToolUse hook denying raw git mutations when `.jj/`
   exists — the only compliance-independent layer, and the documented
   failures of prose-only forcing (#8780, #41435, #56865) argue for it.
   Note our CONTEXT.md scopes the skills to Claude Code; the hook is a
   Claude Code-native mechanism, so this fits.
3. **The colocation offer is novel — design it fresh.** No prior art asks
   first. Open design points (also flagged by verification): is
   `jj git init --colocate` safe on a dirty tree or mid-rebase repo;
   once-per-session vs once-per-repo; teammates who stay git-only
   (RealAdarsh's colocated-git.md covers coexistence, not conversion);
   the `--colocate`-by-default version wrinkle vs our local jj 0.43.
4. **Native-workflow guidance must cover** (union of the verified failure
   evidence, roughly ranked): working-copy-is-a-commit reframing incl.
   "never ask to commit"; the describe/new/commit lane choice (resolve the
   `jj commit` tension explicitly); one change per logical step — never a
   squash-only commit mapping (the agentjj lesson); no staging area /
   `jj diff --git`; non-interactive discipline (`--no-pager`, `-m`, no
   TUIs, conflict resolution by editing files then `jj st`); bookmarks
   (auto-follow rewrites but never advance to new changes; `bookmark
   move`/`set` before push; `--named`-only-for-new rule); push rejection
   on empty/undescribed changes; minimum revsets (`@`, `@-`,
   `trunk()..@`; no `--limit`); op-log undo as the safety net; a
   destructive-command deny-list (rivet's); colocated policy: git
   read-only, jj for all mutations (RealAdarsh), because git fallback in
   colocated repos compounds (agentjj); workspaces/one-workspace-per-agent
   only if we bless concurrent agents; sandbox caveat re commit-signing.
5. **Reconciliation pass.** muloka's superpowers-override table is direct
   precedent: git-assuming guidance gets explicit jj-native replacements
   rather than parallel prose. Our `guardrail` destructive-pattern list
   should grow jj equivalents (`jj abandon`, `jj op restore`,
   `jj git push --force`-alikes, `jj rebase -d main`); `decision-records`'
   "git holds the history" phrasing generalises to "the VCS holds the
   history".

### Evidence gaps to carry forward

- Phrasing *efficacy* is anecdote-contested, never tested; a small
  experiment (frontmatter-only vs hook-backed) would settle the biggest
  open question before committing to a design.
- The RQ5 upstream silence was confirmed by direct repo inspection, not
  just pipeline silence.
- One refuted claim (concurrent-agent races from the single working copy)
  must not be propagated into our skills.
