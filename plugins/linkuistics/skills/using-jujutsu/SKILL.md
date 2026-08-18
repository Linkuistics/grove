---
name: using-jujutsu
description: Drive version control through Jujutsu (jj) natively when the repo is jj-enabled (a .jj/ directory exists) — working-copy-as-commit, jj new/describe, bookmarks, op-log undo, and a git→jj command and concept mapping — and through git, silently, everywhere else. Use when about to commit, branch, merge, rebase, push, stash, tag, resolve conflicts, open a PR, run any git or version-control command, or translate a specific git command or concept to jj — and especially when HEAD is detached or a .jj/ directory is visible (a jj repo seen through git eyes).
---

# Using Jujutsu

In a jj-enabled repo, jj is the interface. That is a correctness rule, not a
style preference: raw git mutations bypass jj's operation log, and in a colocated
repo they desync the two views of history in ways that compound.

This file is the contract — which interface the repo picks, the model, the lane,
and what waits for a human. **The command surface is in the references**, one
level down:

- [`references/jj-commands.md`](references/jj-commands.md) — reading state,
  bookmarks, pushing, undo, workspaces, and an optional per-harness guard hook.
- [`references/git-to-jj-mapping.md`](references/git-to-jj-mapping.md) — the
  git→jj lookup table, when you arrive knowing the git verb.

> Adapted, with independent prose, from the MIT/Apache prior art listed in
> [`../../PROVENANCE.md`](../../PROVENANCE.md). Command facts verified against
> jj 0.43.0.

## First action: establish the repo's interface

Before the first VCS command of a session, settle which interface the repo picks.
Three sources can tell you, and they rank: **an authoritative statement of the
repo's VCS in your prompt** outranks **a probe**, which outranks **the harness
environment banner** — that one never wins.

A prompt that states the repo's VCS, and the root it was resolved for, did this
work before the session started: take it and skip the probe. A banner may report a
colocated repo as "a git repository" — that metadata is computed from `.git/`
alone and cannot see jj
([claude-code#41435](https://github.com/anthropics/claude-code/issues/41435)).

Otherwise probe, in this order:

1. `jj root` succeeds → **jj-enabled**. Everything below applies.
2. Else `git rev-parse --show-toplevel` succeeds → **git**. Use git as normal;
   nothing below applies.
3. Else → no VCS; version control is out of scope.

Both probes walk up from the current directory, so subdirectories and colocated
repos resolve correctly. The repo's state alone picks the interface, and a
statement or probe only reports it — never convert a repo to jj and never suggest
converting; repository setup belongs to the human.

Edge case — `.jj/` exists but no `jj` binary is on PATH: in a colocated repo
(`.git/` also present), fall back to git and tell the user you did; in a native jj
repo, stop and tell the user — git cannot see that repo's history.

## The model: the working copy is a commit

jj has no staging area and no untracked/uncommitted limbo. The working copy *is* a
commit (named `@`), and every jj command starts by auto-snapshotting your edits
into it. Consequences:

- **Never ask "want to commit these changes?"** — they are committed. The
  meaningful questions are: start a new change? describe this one? push?
- **Amend = keep editing.** Edits land in `@` automatically; there is no
  `--amend`. To amend the description, run `jj describe -m` again.
- **Nothing is ever lost by switching.** No stash exists or is needed: describe
  the work and `jj new` elsewhere (e.g. `jj new trunk()`); the old change stays
  put and `jj log` still shows it.

## The lane: new → work → describe → new

```bash
jj new                      # open a fresh empty change on top of @
# ...edit files; every jj command snapshots them into @...
jj describe -m "<why this change exists>"   # record intent — do it early
jj new                      # seal the change; opens the next one
```

Describe *early*: an undescribed change cannot be pushed, and the description is
cheap to replace (`jj describe -m` again).

**One logical step = one change.** Implement, test, document = three
`new`/`describe` cycles, not one flattened commit — squash-flattening multi-step
work is the documented way agents destroy useful history.

`jj commit -m` exists and is exactly `jj describe -m` + `jj new`; this skill uses
the two-verb form everywhere so there is a single mental model. To split a change
that grew too big, `jj split <fileset>` moves the named paths into their own
commit non-interactively.

## Non-interactive discipline

Agent sessions have no human at an editor, so every command must terminate on its
own: pass `-m` to every `jj describe` (the bare form opens `$EDITOR` and hangs the
session), pass `--no-pager` on log/diff/show, and use the non-interactive forms —
`jj split <fileset>` rather than `jj split -i`, editing conflict markers in the
files directly rather than `jj resolve` (which invokes an external merge tool),
and no `jj diffedit` at all. Conflicts don't block you: jj records them *in*
commits and keeps going. Resolve by editing the marked files in a change on top,
then verify with `jj st`.

## What stays human-gated

Every mutation is one entry in the operation log, and `jj undo` reverts the most
recent one — so honest mistakes are cheap to recover yourself rather than asking.
Discarding *work* is different. Run these only when the user explicitly asks:
`jj abandon` (throws a change away, conflict state included), `jj op restore`
(rewinds the whole repo, discarding everything after), `jj rebase -o` (rewrites
history onto a new destination), and `jj bookmark delete` + push (deletes the
branch on the remote).

## Colocated repos: git is read-only

A colocated repo has both `.jj/` and `.git/` over one working copy, letting
git-only tooling coexist with jj. The policy: **jj performs every mutation; git is
for reading only** (`git log`, `git status`, or whatever a build tool shells out
to). This is stricter than taste. A field post-mortem documents the failure: after
out-of-band git mutations, files tracked by jj appeared "deleted" in git's staging
area, and agents that then fell back to git *made the desync worse* (the failure
catalogue linked from [`../../PROVENANCE.md`](../../PROVENANCE.md)). If git output
ever looks impossible here, stop running git and check `jj st` — jj's view is
authoritative. Detached HEAD in git output is normal, not an error.

`git submodule` and `git lfs` have no jj equivalent — those operations still run
through git, the narrow exception to the rule above.

## Concurrent agents: one workspace each

jj's native isolation unit is the workspace — a second working copy on the same
repo, each with its own `@`: `jj workspace add ../<name>`. In a jj-enabled repo,
use one wherever a git-worktree skill or habit (e.g.
`superpowers:using-git-worktrees`) would reach for `git worktree` — those flows
assume git and can fail silently in colocated repos. One agent per workspace, and
`jj edit` only changes no other workspace has as its `@`.
