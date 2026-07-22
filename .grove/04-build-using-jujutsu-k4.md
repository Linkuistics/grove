# build-using-jujutsu-k4

**Kind:** work

## Goal

Create `plugins/linkuistics/skills/using-jujutsu/SKILL.md` — the workflow
skill that auto-fires on VCS work, probes repo state, and drives jj natively
in jj-enabled repos. All design decisions are settled (see Context); this
session writes, verifies, and micro-tests the skill.

## Context

Settled decisions (full rationale in `03-DONE-skill-design-k3.md`'s running
log and `docs/adr/symmetric-vcs-rule.md`):

- **Harness-neutral core** — must work in Pi, Codex, and Claude Code
  sessions (grove integration). Portable prose + shell probes only; nothing
  Claude Code-specific may be load-bearing.
- **Frontmatter description** per `authoring-conventions` (capability +
  "Use when"): names the git-shaped triggers — commit, branch, push, stash,
  merge, "version control", detached HEAD (the tell-tale of a colocated
  repo) — so the skill fires *before* a wrong git command.
- **First action = probe**, vcs-detect ordering: `jj root` succeeds →
  jj-enabled; else `git rev-parse --show-toplevel` succeeds → git; else no
  VCS. Prose + two commands, no shipped script.
- **Symmetric VCS rule** (cite the ADR by slug): jj-enabled → jj is the
  interface; otherwise git, silently. Never convert, never offer. Edge
  case: `.jj/` present but no jj binary → colocated: fall back to git and
  say so; native: stop and tell the user.
- **Commit lane**: two-verb native lane — `jj new` opens a change, work in
  `@`, `jj describe -m` early, `jj new` seals. `jj commit -m` mentioned
  once as unused shorthand. Amend = keep editing `@`.
- **Core content** (the evidence-verified failure modes, from
  `docs/research/jj-agent-prior-art.md` §Q3/§Synthesis-4):
  working-copy-is-a-commit reframing incl. "never ask 'want to commit?'";
  one change per logical step, never squash-flattening; no staging area,
  `jj diff --git`, verify with `jj st`; non-interactive discipline
  (`--no-pager`, `-m` everywhere, no TUI subcommands, resolve conflicts by
  editing files then `jj st`); bookmarks auto-follow rewrites but never
  advance to new changes — `bookmark move`/`set` before push, `--named`
  only for brand-new bookmarks; push rejects empty/undescribed changes;
  minimum revsets (`@`, `@-`, `trunk()..@`; `--limit` is not a jj flag);
  op-log undo as the safety net; destructive-command deny-list (`jj
  abandon`, `jj op restore`, `jj rebase -d`, force-push forms — unless the
  user explicitly asks); colocated policy: git strictly read-only, jj for
  all mutations (git fallback compounds desync); commit-signing sandbox
  caveat.
- **Workspaces section**: one workspace per concurrent agent,
  `jj workspace add`; explicit precedence over git-worktree skills
  (`superpowers:using-git-worktrees`) in jj-enabled repos. Word as
  discipline, not danger — the concurrent-races claim was REFUTED in
  research verification; do not propagate it.
- **Sharing work section**: bookmark set/move → `jj git push` → PR via
  `gh`; folded in, no separate PR skills.
- **Per-harness enforcement recipes section** (precedent:
  `doubt-driven-development`'s per-harness spawn recipes): the neutral
  behaviour is the contract; optional guard setup per harness where one
  exists. Claude Code: a PreToolUse deny-git-when-jj-enabled hook recipe,
  architecture re-implemented from kawaz (its prose is Japanese — write
  fresh). Other harnesses: add only what is verified to exist.
- **Cross-reference `git-to-jj-mapping` by name** (never `@path`) for
  translation needs. It may not exist yet when this leaf runs — the name
  is settled, reference it anyway.

## Done when

- The skill exists, follows `authoring-conventions` (description shape,
  ≤1024-char frontmatter, body well under ~500 lines, positive phrasing
  over bare prohibitions where possible, adapted-idea attributions to the
  MIT/Apache sources named in the k3 log).
- Every version-specific jj fact verified against local jj 0.43 (`jj
  <cmd> --help`); anything unverifiable is flagged `UNVERIFIED` inline.
- Wording micro-tested per `authoring-conventions`: check the no-skill
  control actually exhibits the git-defaulting failure before trusting any
  behaviour-shaping sentence; a discipline-shaped rule like "git read-only
  when colocated" deserves the cheap control check.
- One focused commit naming `build-using-jujutsu-k4`; leaf retired.

## Notes

AFK. Do not add the skill to README/plugin.json/CHANGELOG — that is
`reconcile-and-announce-k6`'s job. Known prior-art content errors to avoid
repeating: `jj tug` is a user alias, not a builtin; `jj split` accepts
fileset arguments non-interactively.
