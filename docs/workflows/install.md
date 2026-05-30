# `grove install` — walkthrough

Add the grove methodology to a repo for the first time. By the end of this walkthrough, the demo repo `acme/orders-api` has `.claude/skills/grove/` materialised on disk and one new commit recording it.

> This page is about driving the **grove CLI**. For *what grove is and why*, see [`docs/grove.md`](../grove.md); for the methodology agents read at runtime, see `content/SKILL.md`. For the full flag surface of `grove install`, run `grove install --help`.

## Start from a clean repo

We assume a working tree with no pending changes — `grove install` does not sweep up unrelated work, but it's easier to read the resulting diff if nothing else is in flight.

```
$ cd acme/orders-api
$ git status
On branch main
nothing to commit, working tree clean
```

The repo already has a `.claude/` directory because someone has run Claude Code here at least once. `grove install` uses that to auto-detect the harness; without it you would pass `--harness claude` explicitly.

## The default install

```
$ grove install
grove: target /Users/you/code/acme/orders-api @ v2.0.0
grove: installed → /Users/you/code/acme/orders-api/.claude/skills/grove @ v2.0.0
```

Two things happened. First, grove materialised the methodology into `.claude/skills/grove/`:

```
$ tree -L 1 .claude/skills/grove
.claude/skills/grove
├── ADR-FORMAT.md
├── BRIEF-FORMAT.md
├── CONTEXT-FORMAT.md
├── LICENSES
├── SKILL.md
├── TASK-FORMAT.md
├── VERSION.md
├── grilling.md
└── prompts
```

Second, it committed exactly those paths:

```
$ git log --oneline -1
1a2b3c4 Install grove v2.0.0

$ git show --stat HEAD
commit 1a2b3c4...
    Install grove v2.0.0

 .claude/skills/grove/ADR-FORMAT.md     | NNN +++++
 .claude/skills/grove/BRIEF-FORMAT.md   | NNN +++++
 .claude/skills/grove/CONTEXT-FORMAT.md | NNN +++++
 ...
```

The commit is a **path-scoped commit**: grove ran `git add -- .claude/skills/grove` and then `git commit -- .claude/skills/grove`, naming the install scope explicitly on both ends. Anything you happened to have modified elsewhere in the tree is *not* in this commit — verify with `git status` afterward if you want to see your work-in-progress still sitting there untouched.

## Opting out: `--no-commit`

If you want to inspect or amend the materialisation before committing, pass `--no-commit`:

```
$ grove install --no-commit
grove: target /Users/you/code/acme/orders-api @ v2.0.0
grove: installed → /Users/you/code/acme/orders-api/.claude/skills/grove @ v2.0.0
grove: --no-commit; stage and commit yourself with:
  git add -- .claude/skills/grove
  git commit -m "Install grove v2.0.0"
```

The files are written to disk but neither staged nor committed; `git status` will show them as untracked. The printed `git add` / `git commit` pair is the exact follow-up you would otherwise have run automatically.

## Overriding the message: `--message`

When the default `Install grove v<version>` doesn't fit your project's commit conventions, override it. Short and long forms both work:

```
$ grove install --message "chore: install grove skill (v2.0.0)"
```

or equivalently:

```
$ grove install -m "chore: install grove skill (v2.0.0)"
```

The resulting commit's subject is whatever you passed; the install scope (and the path-scoped staging) is unchanged.

## Refreshing an existing install

`grove install` is **idempotent** (ADR-0008): re-running it on an already-installed repo is safe and *is* how you refresh — there is no separate `update` verb. The verb compares the bundled (canonical) version to each harness's installed stamp and prints a per-harness outcome: no-op when they match, an update when they differ.

Upgrade the CLI first — the CLI and the materialised skill are versioned together, so `grove install` materialises whatever version the *currently-installed CLI* defaults to:

```
$ brew upgrade grove
==> Upgrading Linkuistics/taps/grove 2.0.0 -> 2.1.0
$ grove status
grove cli 2.1.0, installs in /Users/you/code/acme/orders-api:
  claude → 2.0.0
```

`grove status` confirms the mismatch to resolve: CLI at `2.1.0`, materialised content still at `2.0.0`. Then just re-run `grove install` — because the installed version differs from the bundled one, the outcome line reports an **update**:

```
$ grove install
grove: target /Users/you/code/acme/orders-api @ v2.1.0
grove: /Users/you/code/acme/orders-api/.claude/skills/grove → updated 2.0.0 → 2.1.0
grove: record this bump as an ADR in docs/adr/ (grove's discipline for version changes).
```

The contents under `.claude/skills/grove/` are re-extracted at `v2.1.0` (the existing tree is cleared first, so removed files are removed, not left behind), and the install scope is committed as a single path-scoped commit. Its default subject is `Update grove to v2.1.0` rather than `Install grove v<version>`, because the harness already had an install:

```
$ git log --oneline -2
5e6f7a8 Update grove to v2.1.0
1a2b3c4 Install grove v2.0.0
```

The outcome line is the durable record of *what happened* — `installed @ X`, `already at X, no change`, or `updated X → Y`. It is always printed, which makes `grove install` safe to lean on in CI and setup scripts: a re-run never errors on "already installed" and tells you, every time, whether anything actually changed.

**Pin a version** with `grove install --version <tag>`: it honours the pin and stays idempotent — no-ops if the harness is already at `<tag>`, updates otherwise.

**Record the bump as an ADR.** The nudge above is convention, not enforcement — it fires only on a real bump (a no-op refresh stays silent), and grove does *not* scaffold the ADR. Author it by hand and commit it separately from the materialisation, since it is its own decision:

```
$ git add docs/adr/0009-update-grove-to-v2.1.0.md
$ git commit -m "docs(adr): record grove v2.1.0 bump"
```

You end up with two commits for the bump — the materialisation diff and the decision record — answering both *what changed* and *why*.

**Nothing to refresh.** If the bundled content already matches what's materialised (you ran `grove install` twice, or the new release didn't change any bundled files), the outcome line reports no change and grove finishes silently without producing an empty commit; the ADR nudge does not fire:

```
$ grove install
grove: target /Users/you/code/acme/orders-api @ v2.1.0
grove: /Users/you/code/acme/orders-api/.claude/skills/grove → already at 2.1.0, no change
grove: no changes to commit
```

## Refusing with pre-existing staged changes in scope

Because grove commits using explicit paths, it refuses *only* when something is already staged inside those paths — that's the one case where an automatic commit could silently bundle your in-flight work with the materialisation:

```
$ git add .claude/skills/grove/SKILL.md  # imagine a leftover stage from a prior session
$ grove install
grove: target /Users/you/code/acme/orders-api @ v2.0.0
Error: refusing to proceed: install-scope paths have pre-existing staged changes (.claude/skills/grove). Commit or unstage them before running grove install.
```

The fix is whichever you intended — `git commit` the staged hunks separately, or `git restore --staged .claude/skills/grove` to unstage them — then re-run `grove install`. Unrelated staged hunks elsewhere in the repo are *not* a problem and do not block the install.

## If the commit step fails (e.g. a pre-commit hook rejects)

Grove leaves the materialised files in place, exits non-zero, and prints the exact `git commit` to retry once you've fixed the underlying issue. The materialisation is not rolled back, so any pre-commit hook failure is yours to resolve and then complete the commit manually.

## Codex harness

If your repo uses `.codex/` instead of `.claude/`, everything above is identical except the install path:

```
$ grove install --harness codex
grove: target /Users/you/code/acme/orders-api @ v2.0.0
grove: installed → /Users/you/code/acme/orders-api/.codex/skills/grove @ v2.0.0
```

The default commit message is the same; only the path inside the install scope changes.
