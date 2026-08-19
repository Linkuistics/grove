## How the pick walks

The driver makes **one authoritative pick** per session, in-process, before the
session exists: the first live leaf in a depth-first **pre-order** walk of
`.grove/`, visiting each directory's children in per-level position order (the
`BRIEF.md` charter first — and skipped — then by numeric position) and
descending node directories in place. Briefs, terminal leaves (`DONE` and
`ABANDONED` alike) and foreign files are skipped, and a driver-owned `finish`
leaf is passed over while any ordinary work is live, becoming eligible once it
is the only live leaf. Nothing else modulates the walk — no priority, no
grouping, no set of leaves that must finish before another is considered. That
selection's **stable handle** is what the driver hands you as your mandate.

**Why a second walk can disagree.** `grove-llm pick` remains a diagnostic and
tree-interface verb — it is how you or a human read the tree's next answer
directly, the same one `find .grove` gives by eye — but it is not this session's
dispatcher. A leaf inserted while your configured command was starting can move
your leaf's *path* and would make a second walk disagree with your mandate; the
mandate wins, and the inserted leaf is simply the next iteration's work.

## What the one configuration carries

Every session is launched from personal configuration — it lives at
`~/.config/grove/config.kdl` — which gives each session kind exactly one
complete command template. That template chooses the executable or wrapper and
every user-controlled argument — harness, model, reasoning effort, approval,
permission and sandbox policy — and grove neither knows nor infers which harness
it eventually reaches: it reads the selected leaf's kind from the filename,
looks that one kind up, expands its own substitutions, and executes the result
directly (no shell). `${prompt}` carries what grove has to say to the session,
and `${session_name}`, `${worktree}` and `${repo}` are the only others.
**Nothing else routes a session** — no environment variable, no command-line
flag, no field in a task file, and no default, family or inheritance.

Grove never creates or edits that file, because it cannot choose personal model
or wrapper policy, and it re-validates the whole file before every tree mutation
and again before every launch: an edit lands on the next session, and an invalid
file launches nothing while leaving the selected leaf live and resumable.

## The loop is stateless, which is why restart ≡ continuation

Bare `grove` drives the **whole loop**, not one task: one fresh foreground
harness session per task (owning the real TTY, so grilling / resize / Ctrl-C are
all native), each launched with fresh context, so every task is a clean-context
session without a manual `/clear`+relaunch crank. Because the loop body holds
zero engine state and re-derives its position from the tree every iteration,
**restart ≡ continuation** by construction: a task that crashes before its
retire-and-commit boundary leaves its leaf live and is simply re-selected and
redone, and a loop that has stopped is continued by re-running `grove` from the
same working tree. There is no PTY wrapper and no daemon — a plain shell `while`
loop could stand in (constraint 6).

## What the scaffold creates, and why it is not yours to create

A brand-new grove has a working tree but no `.grove/` yet, and every loop step
assumes one exists; the driver resolves that chicken-and-egg before an agent
exists, because a rootless tree has no leaf to select and the grow verbs need a
root too. It creates `.grove/`, the root `BRIEF.md` stub, a first
**requirements** leaf `01-requirements-<slug>-k1.md` (default slug `plan`), and
the format witness, then selects that leaf and launches it. Creating the first
*leaf*, not just the brief, is load-bearing: a brief is not a leaf, so a
brief-only `.grove/` would look like a grove with no live work — a newborn
indistinguishable from a finished one, mis-triggering the finish cycle
(fresh-grove-start-contract).

So a session **never scaffolds the tree itself**: it starts at Bootstrap like
every other one, and its own commit folds the scaffold in as a working-tree
change.

## Deriving the session name yourself

The driver computes this grove's session name — `<repo-basename>: <name> grove`
— and offers it to the configured command as `${session_name}`; it never renames
a session itself. If your template does not pass it and the session name doesn't
already match, suggest `/rename <repo-basename>: <name> grove` once per session
and move on. The skill can derive both names: `<name>` from the working tree's
own basename (`jj workspace root` in a jj-enabled tree; `git rev-parse
--show-toplevel` otherwise), `<repo-basename>` from the **main repo**'s basename
(`jj workspace root --name default`'s basename in a jj-enabled tree, or `git
rev-parse --git-common-dir`'s parent — the repo a linked worktree or secondary
workspace belongs to, not the working tree's own path).

## A legacy tree was already migrated, or already refused

If the tree is in the one older format still converted — v2 directories whose
leaves carry no session kind in their filenames — the first bare `grove`
**migrates it before driving**, as one recoverable transaction and one focused,
reviewable commit; migration is idempotent once a tree is current-format, and
there is **no** transitional dual-format reader (task-tree-scheme). The two
earlier layouts — the original `NNN-slug/` directories and the v1 flat
dotted-decimal scheme — are no longer migrated: bare `grove` recognises either,
refuses, names the entries and changes nothing. Neither conversion is yours to
perform by hand.
