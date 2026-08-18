# Git → jj mapping

One direction: you arrive knowing the git verb and need the jj one. Behaviour and
workflow — when to describe, bookmark discipline before pushing, the
colocated-repo policy, what stays human-gated — are in `using-jujutsu`'s
`SKILL.md`; this is the lookup table.

> Seeded from jaredramirez/codex-jj-plugin's jj-guide (MIT) and cross-checked
> against mtaran/jj-guide (MIT), with independent notes. Every row verified
> against jj 0.43.0 (`jj <cmd> --help`).

Where a jj column says `<fileset>`, any fileset works — a path, `glob:src/**`,
`~excluded` — jj commands take filesets wherever git takes paths.

## Concepts

| Git concept | In jj |
|---|---|
| Staging area / index | None. The working copy is a commit (`@`); every jj command auto-snapshots edits into it. `git add` has no translation — there is nothing to stage. |
| `HEAD` | `@`. `HEAD~1` → `@-`, `HEAD~2` → `@--`. |
| Branch | Bookmark — a name pointing at a change. It follows rewrites of that change but never advances onto new changes; repoint before pushing (see `using-jujutsu`). |
| Detached HEAD | Normal, permanent state — jj has no "current bookmark". A colocated repo always looks detached through git. |
| Stash | `jj new <elsewhere>`. The current change stays in place; `jj new <change>` or `jj edit <change>` returns to it. Nothing to pop. |
| `commit --amend` | Keep editing — edits land in `@` automatically. `jj describe -m` rewrites the message; `jj squash` folds `@` into its parent. |
| Interactive rebase | No single command — combine `jj rebase`, `jj squash`, `jj split`, `jj abandon`. |
| `.gitignore` | Honoured as-is — same syntax, same locations. |
| Untracked files | Almost none — every non-ignored file is auto-tracked at the next snapshot. `jj file untrack` exists but only accepts already-ignored paths. |

## Reading state

| Git | jj | Notes |
|---|---|---|
| `git status` | `jj st` | |
| `git log` | `jj log` | `-r <revset>` scopes, `-n <N>` limits; your branch's work: `jj log -r 'trunk()..@'` |
| `git diff` | `jj diff --git` | bare `jj diff` prints jj's color-words format — unfamiliar, not corrupted |
| `git diff --cached` | — | no staging area; `jj diff` already shows all of `@` |
| `git diff A..B` | `jj diff --from A --to B` | |
| `git show <rev>` | `jj show <rev>` | |
| `git blame <file>` | `jj file annotate <path>` | |
| `git grep <pat>` | `jj file search <pattern>` | early command: glob matching, prints file names only — plain `grep`/`rg` is still fine |
| `git bisect run <cmd>` | `jj bisect run --range <revset> -- <cmd>` | automated bisect only |
| `git reflog` | `jj op log` | whole-repo operation history; one change's history: `jj evolog` |

## Making commits

| Git | jj | Notes |
|---|---|---|
| `git add` | — | auto-tracked |
| `git add -A && git commit -m` | `jj commit -m "msg"` | ≡ `jj describe -m` + `jj new`; `using-jujutsu` prefers the two-verb lane |
| `git add <file> && git commit -m` | `jj commit -m "msg" <fileset>` | |
| `git commit --amend -m` | `jj describe -m "msg"` | message only |
| `git commit --amend` (content) | nothing, or `jj squash` | edits are already in `@`; `jj squash [<fileset>]` folds `@` (or part of it) into `@-` |
| `git add -p && git commit` | `jj split <fileset>` | non-interactive with fileset arguments |
| `git commit --allow-empty` | `jj commit -m "msg"` | empty changes are normal (push rejects them — see `using-jujutsu`) |
| `git cherry-pick <rev>` | `jj duplicate <rev> -o <dest>` | `-o/--onto`, or `-A`/`-B` to insert |
| `git revert <rev>` | `jj revert -r <rev> -o <dest>` | one of `-o`/`-A`/`-B` is required |
| `git merge <other>` | `jj new <a> <b> -m "msg"` | a merge is just a change with two parents |

## Branches → bookmarks

| Git | jj | Notes |
|---|---|---|
| `git branch` | `jj bookmark list` | `-a/--all-remotes` includes remotes |
| `git branch <name>` | `jj bookmark create <name>` | `-r` defaults to `@` |
| `git branch -f <name> <rev>` | `jj bookmark set <name> -r <rev>` | `set` = create-or-move |
| `git checkout <branch>` | `jj new <bookmark>` | new change on top; `jj edit <rev>` to modify a change in place |
| `git checkout -b <name>` | `jj new <base>`, bookmark later | usually at push time — see `using-jujutsu` |
| `git branch -d <name>` | `jj bookmark delete <name>` | propagates to the remote at next push; `forget` is local-only |
| `git branch -m old new` | `jj bookmark rename old new` | |
| `git branch -u origin/<name>` | `jj bookmark track <name>@origin` | |
| `git tag` | `jj tag list` / `jj tag set <name> -r <rev>` / `jj tag delete` | |

## Remotes

| Git | jj | Notes |
|---|---|---|
| `git fetch` | `jj git fetch` | |
| `git pull` | `jj git fetch`, then `jj rebase -o <bookmark>@origin` | no pull command; rebase only if your work needs to move |
| `git push` | `jj git push` | tracked bookmarks; lease-checked by default — no force flag exists |
| `git push origin <branch>` | `jj git push -b <bookmark>` | |
| `git push -u origin <new>` | `jj git push --named <name>=<rev>` | new bookmark names only |
| `git clone <url>` | `jj git clone <url>` | colocated by default |
| `git init` | `jj git init` | colocates with an existing `.git/` by default. Translation only — never convert a repo you are working in (see `using-jujutsu`) |
| `git remote add <n> <url>` | `jj git remote add <n> <url>` | |

## Rewriting history

| Git | jj | Notes |
|---|---|---|
| `git rebase <base>` | `jj rebase -o <dest>` | 0.43 spells it `-o/--onto` (the older `-d` still parses); descendants follow automatically, conflicts don't stop it |
| `git rebase --onto <new> <old>` | `jj rebase -s <source> -o <dest>` | |
| move one commit | `jj rebase -r <rev> -o <dest>` | |
| `git commit --fixup` + autosquash | `jj absorb` | routes each hunk of `@` to the ancestor that last touched those lines |
| squash a range | `jj squash --from <revset> --into <rev>` | |
| edit an old commit | `jj edit <rev>` | later edits snapshot into it; descendants rebase automatically |
| reword an old commit | `jj describe <rev> -m "msg"` | |
| author/date/metadata only | `jj metaedit -r <rev>` | `--author`, `--update-author`, timestamp flags |
| serial commits → siblings | `jj parallelize <revsets>` | no git equivalent |

## Undoing

| Git | jj | Notes |
|---|---|---|
| `git restore <file>` | `jj restore <fileset>` | restores from `@-` by default |
| `git restore --source <rev>` | `jj restore --from <rev> <fileset>` | |
| discard all working-copy changes | `jj restore` | |
| `git reset --hard HEAD~1` | `jj abandon` | discards a change — human-gated, see `using-jujutsu` |
| undo the last VCS action | `jj undo` / `jj redo` | covers any operation, fetches included |
| `git reset --hard <reflog-id>` | `jj op restore <op-id>` | rewinds the whole repo — human-gated, see `using-jujutsu` |

## Worktrees → workspaces

| Git | jj | Notes |
|---|---|---|
| `git worktree add <path>` | `jj workspace add <path> [--name <n>]` | one workspace per concurrent agent — see `using-jujutsu` |
| `git worktree list` | `jj workspace list` | |
| `git worktree remove <path>` | `jj workspace forget <name>` | then delete the directory |
| — | `jj workspace update-stale` | after another workspace rewrote this one's `@` |
| `git rev-parse --show-toplevel` | `jj workspace root` | `jj root` works too |

