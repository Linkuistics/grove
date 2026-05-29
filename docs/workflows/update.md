# `grove update` — walkthrough

Refresh an existing grove materialisation in `acme/orders-api`, going from `v2.0.0` (left by [`install.md`](install.md)) to `v2.1.0`. By the end of this walkthrough the repo has one more commit on top of the install commit and a fresh ADR recording the bump.

> This page is about driving the **grove CLI**. For *what grove is and why*, see [`docs/grove.md`](../grove.md). The auto-commit rules are identical to install — they are specified in [ADR-0001](../adr/0001-install-and-update-create-commits.md); we won't restate them here, only show them in action.

## Starting state

We pick up exactly where [`install.md`](install.md) left off:

```
$ cd acme/orders-api
$ git log --oneline -1
1a2b3c4 Install grove v2.0.0

$ git status
On branch main
nothing to commit, working tree clean
```

`.claude/skills/grove/VERSION.md` records `v2.0.0`. Some weeks later, a new grove release goes out and we want to pick it up.

## Upgrade the CLI first

The CLI and the materialised skill are versioned together — `grove update` materialises whatever version the *currently-installed CLI* defaults to, so the CLI upgrade comes first:

```
$ brew upgrade grove
==> Upgrading Linkuistics/taps/grove 2.0.0 -> 2.1.0
...
$ grove status
grove cli 2.1.0, installs in /Users/you/code/acme/orders-api:
  claude → 2.0.0
```

`grove status` confirms the mismatch we are about to resolve: CLI at `2.1.0`, materialised content still at `2.0.0`.

## The default update

```
$ grove update
grove: target /Users/you/code/acme/orders-api @ v2.1.0
grove: updated → /Users/you/code/acme/orders-api/.claude/skills/grove @ v2.1.0
grove: record this bump as an ADR in docs/adr/ (grove's discipline for version changes).
```

Three things happened. The contents under `.claude/skills/grove/` were re-extracted at `v2.1.0` (the existing tree was cleared first, so removed files are removed, not left behind). The install scope was committed as a single path-scoped commit. And grove printed an explicit nudge — the last `eprintln` line above — reminding you to author an ADR for this bump. It does *not* scaffold one; that's the next step, by hand.

```
$ git log --oneline -2
5e6f7a8 Update grove to v2.1.0
1a2b3c4 Install grove v2.0.0

$ git show --stat HEAD
commit 5e6f7a8...
    Update grove to v2.1.0

 .claude/skills/grove/SKILL.md      |  NN ++--
 .claude/skills/grove/grilling.md   |  NN +-
 .claude/skills/grove/VERSION.md    |   2 +-
 ...
```

The update commit sits cleanly on top of the install commit; together, `git log .claude/skills/grove/` is the entire on-repo history of the materialisation.

## Authoring the ADR

The nudge above is convention, not enforcement. `docs/grove.md` puts it plainly: *"By discipline, record version bumps in an ADR (`docs/adr/`) so the update decision is traceable — `VERSION.md` only carries the current version, not the history."* The walkthrough convention looks like this:

```
$ $EDITOR docs/adr/0007-update-grove-to-v2.1.0.md
```

```markdown
# Update grove to v2.1.0

## Status
accepted

## Context
We were on v2.0.0 (installed in commit 1a2b3c4). v2.1.0 ships [whatever the release notes call out — e.g. a tightened grilling procedure and a new `takeover` verb].

## Decision
Adopt v2.1.0 across the repo.

## Consequences
- New grove sessions get the v2.1.0 methodology immediately.
- Long-running groves on existing worktrees continue to read the materialised copy under their worktree until they pick the change up.
```

Commit the ADR separately — `grove update` already produced its own focused commit, and the ADR is its own decision:

```
$ git add docs/adr/0007-update-grove-to-v2.1.0.md
$ git commit -m "docs(adr): record grove v2.1.0 bump"
```

You now have two commits for the bump: the materialisation diff, and the decision record. Together they answer both *what changed* and *why*.

## `--no-commit` and `--message`

These flags behave exactly as they do for install — see [`install.md`](install.md#opting-out---no-commit). The only difference is the default message: `Update grove to v<version>` rather than `Install grove v<version>`.

## Nothing to update

If the CLI's bundled content matches what's already materialised — for example you ran `grove update` twice in a row, or you upgraded the formula but the new release didn't change any of the bundled files — grove finishes silently without producing an empty commit:

```
$ grove update
grove: target /Users/you/code/acme/orders-api @ v2.1.0
grove: updated → /Users/you/code/acme/orders-api/.claude/skills/grove @ v2.1.0
grove: no changes to commit
grove: record this bump as an ADR in docs/adr/ (grove's discipline for version changes).
```

The materialisation step still ran (the directory was cleared and re-extracted), but the resulting tree was byte-identical to the previous one, so the path-scoped `git diff --cached` came back clean and grove skipped the commit instead of producing an empty one. The ADR nudge still fires; ignore it when there is no real bump to record.

## Codex harness

Same flow with `--harness codex`; only the install-scope path differs (`.codex/skills/grove/` rather than `.claude/skills/grove/`). The default commit message and the ADR nudge are identical.
