# grove-jj-docs-k10

**Kind:** work

## Goal

The dual-VCS documentation pass over grove's prose, per
`grove-jj-integration-k7` Q3: git stays the default vocabulary, jj is
named alongside wherever a concrete command appears, conceptual lines go
VCS-neutral. Execution in Linkuistics/grove (`~/Development/grove`); this
leaf only tracks.

## Context

- Decision (Q3, full log in `07-DONE-grove-jj-integration-k7.md`):
  dual-VCS, git default. Rejected: fully-neutral prose, jj appendix.
- Known surface (re-verify with a grep for `git` before editing):
  - `content/SKILL.md`: "a git-tracked tree of task files", "git is the
    history" (×2), "Starting a new grove: git init / clone / worktree" →
    add `jj git init --colocate` / `jj git clone` / `jj workspace add`,
    the finish cycle's "plain git/gh", the session-naming probe paragraph
    (`git rev-parse …` → name both probes), `git mv` mentions in
    Decompose (the verb's behaviour is now VCS-dependent per
    `grove-jj-plumbing-k8`).
  - `docs/adr/user-owned-worktrees.md`: rework in place to user-owned
    *working trees* — jj workspaces equally valid, grove reads no branch
    and no bookmark. Fix citations of the ADR if its title/slug shifts
    (grep before renaming; prefer keeping the slug).
  - `docs/adr/codex-gitdir-grant.md`: touched by
    `grove-codex-jj-sandbox-k9`; only reconcile, don't duplicate.
  - `content/prompts/`, other `content/*.md`, `docs/*.md`: sweep for git
    mentions.
- Sequenced last so the prose describes shipped behaviour
  (`grove-jj-plumbing-k8`, `grove-codex-jj-sandbox-k9` first).

## Done when

- The dual-VCS pass is applied across `content/` and `docs/`; no prose
  claims a git-only behaviour that the plumbing no longer has.
- `user-owned-worktrees` reworked in place; all its citations reconciled.
- One focused commit in Linkuistics/grove; this leaf retired with a
  bookkeeping commit here naming `grove-jj-docs-k10`.

## Notes

Keep grove walk-away-able: the pass must not make the linkuistics
`using-jujutsu` skill a prerequisite — grove's own docs stay
self-contained (constraint 6).

## Outcome

Shipped as Linkuistics/grove `ce0131a` (main, pushed). The dual-VCS pass
landed across `content/`, `docs/`, the README, and — one step beyond the
planned surface — the CLI help text in `src/cli.rs`, which claimed the
git-only precondition verbatim. Conceptual lines went VCS-neutral ("the
VCS holds the history"); every concrete command now names both
interfaces; `git mv` mentions carry the jj-enabled plain-rename note
where they describe verb behaviour, and stay plain git in the git-repo
walkthrough transcripts (git default, per Q3). `user-owned-worktrees`
reworked in place to user-owned working trees, slug kept — all citing
prose checked, none dangling. `codex-gitdir-grant` needed no touch (k9
had already reworked it). Also fixed in passing: grove.md's stale
"completed branches retire to an archive" claim, in a sentence already
under edit. 474 tests green; jj commands stay inline so grove keeps its
walk-away-ability (no skill prerequisite added).
