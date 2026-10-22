## How the pick walks

The driver makes **one authoritative pick** per session, in-process, before the
session exists. The pick is the first live leaf in a depth-first **pre-order** walk of
`.grove/`.

The walk visits the positioned children of each directory in numeric order. It
descends node directories in place. It skips node files, terminal leaves (`DONE` and
`ABANDONED` alike) and foreign files. It passes over a driver-owned `finish` leaf
while any ordinary work is live. The `finish` leaf becomes eligible once it is the
only live leaf.

Nothing else modulates the walk: no priority, no grouping, and no set of leaves that
must finish before the walk considers another. The **stable handle** of that selection
is what the driver hands you as your mandate.

**Why a second walk can disagree.** `grove-llm pick` remains a diagnostic and
tree-interface verb. You or a human use it to read the next answer of the tree
directly, the same one that `find .grove` gives by eye. It is not the dispatcher of
this session.

A leaf inserted while the driver launched your session can move the *path* of your leaf. That
insertion would make a second walk disagree with your mandate. The mandate wins. The
inserted leaf is the work of the next iteration.

## What launched this session

Grove has no launch configuration. The driver runs the once-resolved installed `ravel`
for every session. The owner's policy in Ravel returns the command: the executable or
wrapper and every argument. Those arguments carry the harness, the model, the reasoning
effort, and the approval, permission and sandbox policy. Grove neither knows nor infers
which harness it eventually reaches.

Grove reads the kind of the selected leaf from the filename. It passes that kind, the
task file, the handle and the prompt. The `cwd` of the request is the working-tree
root. Grove passes no named parameter. The exit directory and the held stdout result of
the launch are run mechanics, and they reach no policy. The prompt carries what Grove
has to say to the session.

**Nothing else routes a session**. Grove reads no environment variable, no command-line
flag and no field in a task file to choose one. Grove has no implicit kind default and
no family. What the policy itself consults belongs to its owner.

**The policy runs when a leaf launches, and nowhere earlier.** No tree verb consults
it. A leaf of a kind that the policy does not route is written without complaint. It
refuses at its launch. Nothing runs, and the leaf stays live and resumable. The loop
stops until the owner corrects the policy and reruns `grove`.

Grove never creates or edits the policy, because it cannot choose a personal model or
wrapper policy. An edit lands on the next session.

## The loop is stateless, which is why restart ≡ continuation

Bare `grove` drives the **whole loop**, not one task. It runs one fresh foreground
harness session per task. The session owns the real TTY, so grilling, resize and
Ctrl-C are all native. Grove launches each session with fresh context. Every task is
therefore a clean-context session, with no manual `/clear` and relaunch crank.

The loop body holds zero engine state and re-derives its position from the tree every
iteration. So **restart ≡ continuation** by construction. A task that crashes before
its retire-and-commit boundary leaves its leaf live. The loop selects that leaf again
and redoes it.

A stopped loop continues when you rerun `grove` from the same working tree. There is
no PTY wrapper and no daemon. A plain shell `while` loop could stand in (constraint 6).

## What ends the run and what continues the loop

The prompt supplies the exact canonical `end-session grove-continue` and
`end-session grove-force-stop` commands of the executable. Ravel ends and reaps the
managed group. Then it returns that opaque result on stdout. Managed output stays on
stderr. Grove holds a separate unlinked regular file for stdout. Grove never parses a
private report or transcript.

After the reap, Grove restores the terminal and invalidates the session epoch.
Interruption of the driver takes precedence.

An exact `grove-force-stop` result from a successful Ravel invocation then stops the
loop without completion. This holds even if teardown was recorded. The CLI exits 1. It
does not retire work, and it does not restore a removed tree.

Teardown otherwise finishes the loop, whatever the status, the result or the group.
Without teardown, a surviving supervisor group or an unsuccessful Ravel status stops
the loop. Only a successful status and exact `grove-continue` bytes relaunch, with a
gone group. A missing, unsupported or unreadable result stops the loop with the live
leaf resumable. So do a refusal, a cancellation and the death of the supervisor.

A replacement driver invalidates the old epoch. Then it removes abandoned launch
directories without interpreting their contents.

Each launch has a fresh directory in the VCS-administration control area of the
workspace. Grove grants it as `GROVE_LAUNCH_DIR` after it scrubs the old and new
enclosing controls. The policy sees it only through an explicit owner grant. Ravel
creates its ephemeral context below that writable exit area. The teardown marker
belongs to Grove. Grove holds the returned result outside the pathname-based managed
writes.

Sessions that Grove 22.1.0 launched through dispatch retain their original
`harness-dispatch exit` command. They also retain the inherited channel, admission and
teardown controls. New source or installed skills do not change a running launch prompt.

## What the scaffold creates, and why it is not yours to create

A brand-new grove has a working tree but no `.grove/` yet, and every loop step assumes
one exists. The driver resolves that chicken-and-egg problem before an agent exists. A
rootless tree has no leaf to select, and the grow verbs need a root too.

<!-- tc: depart synonym-rotation because fresh-grove-start-contract is an identifier -->
The driver creates `.grove/`, the root `_BRIEF.md` stub and a first **requirements**
leaf `01-requirements--<slug>-k1.md` (default slug `plan`). Then it selects that leaf
and launches it. Creating the first leaf is load-bearing. A root that holds only its
node file is taskless. The driver refuses that root, and it does not select it for
finishing (fresh-grove-start-contract).

So a session **never scaffolds the tree itself**. It starts at Bootstrap like every
other session. Its own commit folds the scaffold in as a working-tree change.

## Deriving the session name yourself

The methodology names the session. Grove passes no session-name parameter, and it never
renames a session itself. If the session name does not already match, suggest
`/rename <repo-basename>: <name> grove` once per session and move on.

The skill can derive both names. Derive `<name>` from the basename of the working tree
(`jj workspace root`). Derive `<repo-basename>` from the basename of the **main repo**
(`jj workspace root --name default`). The main repo is the repo that a secondary
workspace belongs to. It is not the path of the working tree itself.

## A tree must satisfy the grammar before driving

**Grove performs no migration.** The driver opens only the grammar that `TASK-FORMAT.md`
and `BRIEF-FORMAT.md` state. The driver refuses a malformed tree. It reports the
offending names and the canonical form. A read changes nothing. An operator must resolve
the refusal before rerunning `grove`.
