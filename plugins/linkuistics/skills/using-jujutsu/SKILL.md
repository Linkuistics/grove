---
name: using-jujutsu
description: Drive version control through Jujutsu (jj) natively when the repo is jj-enabled (a .jj/ directory exists) — working-copy-as-commit, jj new/describe, bookmarks, op-log undo — and through git, silently, everywhere else. Use when about to commit, branch, merge, rebase, push, stash, tag, resolve conflicts, open a PR, or run any git or version-control command — and especially when HEAD is detached or a .jj/ directory is visible (a jj repo seen through git eyes).
---

# Using Jujutsu

In a jj-enabled repo, jj is the interface. That is a correctness rule, not a
style preference: raw git mutations bypass jj's operation log, and in a
colocated repo they desync the two views of history in ways that compound
(see *Colocated repos* below). This skill teaches jj's native model — not git
commands with jj spellings. For translating a specific git command or concept
to jj, load the `git-to-jj-mapping` skill.

> Adapted, with independent prose, from the MIT/Apache prior art surveyed in
> `docs/research/jj-agent-prior-art.md`: danverbraganza/jujutsu-skill (MIT —
> agent-environment rules), RealAdarsh/jj-skill (MIT — colocated
> git-read-only policy), carbon-language/carbon-lang (Apache-2.0 —
> symmetric detection), muloka/claude-plugins (Apache-2.0 — working-copy
> reframing), kawaz/claude-plugin-jj (MIT — guard-hook architecture,
> re-implemented). Command facts verified against jj 0.43.0.

## First action: probe the repo

Before the first VCS command of a session, detect which interface the repo
picks — in this order:

1. `jj root` succeeds → **jj-enabled**. Everything below applies.
2. Else `git rev-parse --show-toplevel` succeeds → **git**. Use git as
   normal; nothing below applies.
3. Else → no VCS; version control is out of scope.

Both probes walk up from the current directory, so subdirectories and
colocated repos resolve correctly. The repo's state alone picks the
interface — never convert a repo to jj and never suggest converting; repo
setup belongs to the human (`docs/adr/symmetric-vcs-rule.md`).

Trust the probe over harness metadata. A harness environment banner may
report a colocated repo as "a git repository" — that metadata is computed
from `.git/` alone and cannot see jj
([claude-code#41435](https://github.com/anthropics/claude-code/issues/41435)).

Edge case — `.jj/` exists but no `jj` binary is on PATH: in a colocated repo
(`.git/` also present), fall back to git and tell the user you did; in a
native jj repo, stop and tell the user — git cannot see that repo's history.

## The model: the working copy is a commit

jj has no staging area and no untracked/uncommitted limbo. The working copy
*is* a commit (named `@`), and every jj command starts by auto-snapshotting
your edits into it. Consequences:

- **Never ask "want to commit these changes?"** — they are committed. The
  meaningful checkpoint questions are: start a new change? describe this
  one? push?
- **Amend = keep editing.** Edits land in `@` automatically; there is no
  `--amend`. To amend the description, run `jj describe -m` again.
- **Nothing is ever lost by switching.** No stash exists or is needed: to
  set work aside, describe it and `jj new` elsewhere (e.g. `jj new trunk()`);
  the old change stays put and `jj log` still shows it.

## The lane: new → work → describe → new

```bash
jj new                      # open a fresh empty change on top of @
# ...edit files; every jj command snapshots them into @...
jj describe -m "<why this change exists>"   # record intent — do it early
jj new                      # seal the change; opens the next one
```

Describe *early*, not at the end: an undescribed change cannot be pushed,
and the description is cheap to update (`jj describe -m` again replaces it).

**One logical step = one change.** Implement, test, document = three
`new`/`describe` cycles, not one flattened commit — squash-flattening
multi-step work is the documented way agents destroy useful history.

`jj commit -m` exists and is exactly `jj describe -m` + `jj new`; this skill
uses the two-verb form everywhere so there is a single mental model. To
split a change that grew too big, `jj split <fileset>` (e.g.
`jj split src/lib.rs`) moves the named paths into their own commit
non-interactively.

## Reading state

| Want | Run |
|---|---|
| Status (always after mutating) | `jj st` |
| What's in `@` | `jj diff --git` |
| Recent history | `jj log -n 10` |
| This branch's work | `jj log -r 'trunk()..@'` |
| Undo target / audit trail | `jj op log` |

- Revsets to know: `@` (working copy), `@-` (its parent), `trunk()..@`
  (your work). More is rarely needed.
- `jj diff`'s default output is jj's own color-words format — unfamiliar,
  not corrupted. Pass `--git` for the familiar format.
- Verify with `jj st` after each mutation, the way you would check
  `git status`.

## Non-interactive discipline

Agent sessions have no human at an editor, so every command must terminate
on its own:

- Pass `-m` to every `jj describe` — the bare form opens `$EDITOR` and
  hangs the session.
- Pass `--no-pager` (global flag) on log/diff/show output.
- Use the non-interactive forms: `jj split <fileset>` instead of
  `jj split -i`; edit conflict markers in the files directly (then `jj st`
  to confirm the conflict cleared) instead of `jj resolve`, which invokes an
  external merge tool; skip `jj diffedit` entirely.
- Conflicts don't block you: jj records them *in* commits and keeps going.
  Resolve by editing the marked files in a change on top, then verify with
  `jj st`.

## Bookmarks are not branches

Nothing "checks out" a bookmark, and no bookmark is ever current. Two rules
cover almost everything:

- Bookmarks **follow rewrites** of the change they point at (describe,
  squash, rebase) — no manual re-pointing after history edits.
- Bookmarks **never advance** onto newly created changes — after sealing
  work with `jj new`, the bookmark still points where it did. Point it at
  the sealed change before pushing:

```bash
jj bookmark set <name> -r @-    # create or move the bookmark
```

(`jj bookmark move <name> --to <rev>` also works but only moves existing
bookmarks; `set` covers both cases.)

## Sharing work

```bash
jj bookmark set <name> -r @-        # point the bookmark at the sealed change
jj git push                          # push tracking bookmarks
jj git push --named <name>=@-        # first push of a brand-new bookmark
gh pr create ...                     # PRs are a forge concern — gh as usual
```

- `--named` is for *new* bookmark names only; for an existing bookmark,
  `bookmark set` + plain `jj git push`.
- Push rejects empty and undescribed commits by design. The fix is a
  description (or abandoning a stray empty change), not
  `--allow-empty-description`.
- Push is safety-checked like `git push --force-with-lease`: it updates the
  remote only if it matches the last fetch, so no force flag exists or is
  needed.
- Fetch with `jj git fetch`.
- Commit signing: if the repo or user config enables it, jj commands that
  write commits can fail in sandboxes with no signing key or agent
  (UNVERIFIED against 0.43 — reported by prior art). Surface it to the
  user rather than working around it.

## The safety net, and what stays human-gated

Every mutation — including fetches and snapshots — is one entry in the
operation log. `jj undo` reverts the most recent operation; repeating it
walks further back, and `jj redo` walks forward again. This makes honest
mistakes cheap: a wrong squash, a bad describe, even a botched fetch are one
`jj undo` away — recover yourself rather than asking.

Discarding *work* is different from undoing your own last step. Run these
only when the user explicitly asks:

| Command | Why it waits for the user |
|---|---|
| `jj abandon` | throws a change away — and on a conflicted commit, throws away the conflict state |
| `jj op restore` | rewinds the whole repo to an earlier operation, discarding everything after |
| `jj rebase -o` | rewrites history onto a new destination |
| `jj bookmark delete` + push | deletes the branch on the remote |

## Colocated repos: git is read-only

A colocated repo has both `.jj/` and `.git/` over one working copy, letting
git-only tooling coexist with jj. The policy: **jj performs every mutation;
git is for reading only** (`git log`, `git status`, or whatever a build tool
shells out to). This is stricter than taste. A field post-mortem documents
the failure: after out-of-band git mutations, files tracked by jj appeared
"deleted" in git's staging area, and agents that then fell back to git *made
the desync worse* (`docs/research/jj-agent-prior-art.md` §Q3). If git output
ever looks impossible in a colocated repo, stop running git and check
`jj st` — jj's view is the authoritative one. Detached HEAD in git output
is normal here, not an error: jj does not keep git's HEAD on a branch.

## Concurrent agents: one workspace each

jj's native isolation unit is the workspace — a second working copy on the
same repo, each with its own `@`:

```bash
jj workspace add ../<name>      # new working copy; workspace named after the basename
```

In a jj-enabled repo, use a workspace wherever a git-worktree skill or habit
(e.g. `superpowers:using-git-worktrees`) would reach for `git worktree` —
those flows assume git and their tooling can fail silently in colocated
repos. Discipline for sharing a repo: one agent per workspace, and `jj edit`
only changes no other workspace has as its `@`.

## Per-harness enforcement (optional)

Everything above is the contract, and it depends only on a shell — it works
in any harness. Where a harness has a mechanical pre-tool gate, adding a
guard makes the git-read-only rule hold even when prose is forgotten
mid-session. Recipes:

### Claude Code — PreToolUse deny-git guard

Architecture after kawaz/claude-plugin-jj (MIT), re-implemented. Two files
in the *project* (this is a per-repo choice, not something this skill
installs). Unlike the `guardrail` skill's ask-only gate, this one denies:
inside a jj-enabled repo a git mutation is never the right call, and the
read-only git forms stay available.

`.claude/hooks/jj-guard.sh`:

```bash
#!/usr/bin/env bash
# Deny raw git mutations when the repo is jj-enabled. PreToolUse, matcher: Bash.
set -euo pipefail
input=$(cat)
cwd=$(printf '%s' "$input" | jq -r '.cwd // "."')
# Only guard where jj itself is usable — keeps the no-binary git fallback open.
command -v jj >/dev/null 2>&1 || exit 0
jj --ignore-working-copy -R "$cwd" root >/dev/null 2>&1 || exit 0
cmd=$(printf '%s' "$input" | jq -r '.tool_input.command // ""')
if printf '%s' "$cmd" | grep -qE '(^|[;&|(][[:space:]]*|&&[[:space:]]*|\|\|[[:space:]]*)git[[:space:]]+(add|am|apply|branch|checkout|cherry-pick|clean|commit|fetch|merge|mv|pull|push|rebase|reset|restore|revert|rm|stash|switch|tag)\b'; then
  jq -n '{hookSpecificOutput: {hookEventName: "PreToolUse",
    permissionDecision: "deny",
    permissionDecisionReason: "jj-enabled repo: drive mutations through jj; git is read-only here (see using-jujutsu)"}}'
fi
exit 0
```

`.claude/settings.json` (merge into any existing `hooks` block):

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          { "type": "command",
            "command": "bash \"$CLAUDE_PROJECT_DIR/.claude/hooks/jj-guard.sh\"" }
        ]
      }
    ]
  }
}
```

The decision-JSON contract matches the `guardrail` skill's tested hook in
this repo; see its script for the field-name provenance.

### Other harnesses

No pre-tool guard mechanism verified for Pi or Codex as of 2026-07 — there,
the probe-first contract above is the only layer. Add a recipe here only
once its mechanism is verified to exist.
