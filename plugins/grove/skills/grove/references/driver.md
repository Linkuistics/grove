## How the pick walks

The driver makes **one authoritative pick** per session, in-process, before the
session exists: the first live leaf in a depth-first **pre-order** walk of
`.grove/`, visiting each directory's positioned children in numeric order and
descending node directories in place. Node files, terminal leaves (`DONE` and
`ABANDONED` alike) and foreign files are skipped, and a driver-owned `finish`
leaf is passed over while any ordinary work is live, becoming eligible once it
is the only live leaf. Nothing else modulates the walk — no priority, no
grouping, no set of leaves that must finish before another is considered. That
selection's **stable handle** is what the driver hands you as your mandate.

**Why a second walk can disagree.** `grove-llm pick` remains a diagnostic and
tree-interface verb — it is how you or a human read the tree's next answer
directly, the same one `find .grove` gives by eye — but it is not this session's
dispatcher. A leaf inserted while your session was starting can move
your leaf's *path* and would make a second walk disagree with your mandate; the
mandate wins, and the inserted leaf is simply the next iteration's work.

## What launched this session

Grove has no launch configuration. The driver runs the once-resolved installed `ravel` for every
session, and the owner's policy there returns the command: the executable or
wrapper and every argument — harness, model, reasoning effort, approval,
permission and sandbox policy. Grove neither knows nor infers which harness it
eventually reaches. It reads the selected leaf's kind from the filename and
passes that kind, the task file, the handle and the prompt. The request's `cwd`
is the working-tree root, and Grove passes no named parameter. The launch's
exit directory and held stdout result are run mechanics and reach no policy. The
prompt carries what grove has to say to the session.
**Nothing else routes a session** — Grove reads no environment variable, no
command-line flag and no field in a task file to choose one, and has no implicit
kind default or family. What the policy itself consults is its owner's.

**The policy is evaluated when a leaf launches, and nowhere earlier.** No tree
verb consults it, so a leaf of a kind the policy does not route is written
without complaint and refuses at its launch: nothing runs, the leaf stays live
and resumable, and the loop stops until the owner corrects the policy and
reruns `grove`. Grove never creates or edits the policy, because it cannot
choose personal model or wrapper policy. An edit lands on the next session.

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

## What ends the run and what continues the loop

The prompt supplies the exact canonical executable's
`end-session grove-continue` command. Ravel ends and reaps the managed group,
then returns that opaque result on stdout. Managed output stays on stderr.
Grove holds a separate unlinked regular file for stdout and never parses a
private report or transcript.

After reap, Grove restores the terminal and invalidates the session epoch.
The driver's interruption takes precedence, then a recorded teardown finishes.
Without teardown, a surviving supervisor group or unsuccessful Ravel status
stops the loop before result interpretation. Only the exact supported
`grove-continue` bytes relaunch. Missing, unsupported or unreadable results,
refusal, cancellation and supervisor death stop with the live leaf resumable.
A replacement driver invalidates the old epoch before removing abandoned launch
directories without interpreting their contents.

Each launch has a fresh directory in the workspace's VCS-administration control
area. Grove grants it as `GROVE_LAUNCH_DIR` after scrubbing old/new enclosing
controls; policy sees it only through an explicit owner grant. Ravel creates its
ephemeral context below that writable exit area. The teardown marker is Grove's;
the returned result is held outside pathname-based managed writes.

Legacy sessions launched by Grove 22.1.0 through dispatch retain their original
`harness-dispatch exit` command and inherited channel/admission/teardown. New
source or installed skills do not change a running launch prompt.

## What the scaffold creates, and why it is not yours to create

A brand-new grove has a working tree but no `.grove/` yet, and every loop step
assumes one exists; the driver resolves that chicken-and-egg before an agent
exists, because a rootless tree has no leaf to select and the grow verbs need a
root too. It creates `.grove/`, the root `_BRIEF.md` stub and a first
**requirements** leaf `01-requirements--<slug>-k1.md` (default slug `plan`),
then selects that leaf and launches it. Creating the first leaf is load-bearing:
a root holding only its node file is taskless and is refused, not selected for
finishing (fresh-grove-start-contract).

So a session **never scaffolds the tree itself**: it starts at Bootstrap like
every other one, and its own commit folds the scaffold in as a working-tree
change.

## Deriving the session name yourself

Naming the session is the methodology's. Grove passes no session-name
parameter and never renames a session itself. If the session name
doesn't already match, suggest `/rename <repo-basename>: <name> grove` once per session
and move on. The skill can derive both names: `<name>` from the working tree's
own basename (`jj workspace root`), `<repo-basename>` from the **main repo**'s
basename (`jj workspace root --name default`'s basename — the repo a secondary
workspace belongs to, not the working tree's own path).

## A tree must satisfy the grammar before driving

**Grove performs no migration.** The driver opens only the grammar stated in
`TASK-FORMAT.md` and `BRIEF-FORMAT.md`. A malformed tree is refused with the
offending names and canonical form; a read changes nothing. An operator must
resolve the refusal before restarting the loop.
