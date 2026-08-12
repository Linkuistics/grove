# Using Grove

Grove drives a long-running workstream as a sequence of fresh agent sessions.
The workstream lives in a `.grove/` task tree inside a Git or Jujutsu working
tree that you create and own.

Before starting, install Grove as described in the [README](../README.md) and
write the complete personal configuration described in
[CONFIGURATION.md](CONFIGURATION.md). Grove will not start without it.

## Start, resume, and finish

Run Grove from anywhere inside the working tree:

```sh
grove
```

That is the whole human command surface. There are no subcommands and no
lifecycle flags — `grove --help` and `grove --version` are the only other
arguments, and both stop before Grove touches a repository. Bare `grove`
inspects the filesystem and does the appropriate next thing:

| What it finds | What it does |
|---|---|
| No `.grove/` | Creates the root brief and a first `requirements` leaf, then launches it. |
| An older `.grove/` layout | Migrates it in one focused commit, then continues. |
| Live leaves | Launches the first one in tree order. |
| No live leaves | Materializes a `finish` leaf and launches the teardown session. |

It then keeps going: when a session signals that its task is complete, Grove
relaunches with fresh context for the next leaf. Any other ending — you exit the
session, press Ctrl-C, or the process dies — stops the loop.

Because `grove` takes no arguments, **the working directory is the only thing
that selects a workstream**. There is no tree to name and no confirmation step:
Grove adopts, migrates, and commits against whichever working tree encloses the
directory you ran it from. That is what makes the command short, and it is worth
knowing before you run it in a repository that holds several linked worktrees or
Jujutsu workspaces — running it in the wrong one migrates that tree, not the one
you meant. `jj op restore` and `git reset` recover the migration commit if it
happens.

To resume, run `grove` again. Grove has no progress database; it re-derives its
position from the task tree every iteration, which is what makes restart and
continuation the same thing.

Full configuration validation precedes every one of those tree mutations, so a
missing or malformed `config.kdl` leaves your working tree byte-identical.

Grove makes two commits of its own — the migration commit above and the teardown
commit at the end. Both touch only `.grove/`, and in plain Git both run with your
Git hooks disabled: an arbitrary hook can modify unrelated files even while
rejecting the commit, and neither commit's rollback could put those files back.
Signing and other repository failures still surface normally.

### One driver per working tree

A working tree can have only one live Grove driver. A second `grove` in the same
tree exits immediately, names the canonical working tree, and leaves the existing
driver as owner — it does not queue, because two drivers would issue two
mandates for the same task. Different Git worktrees and jj workspaces are
independent even when they share a repository; path aliases and symlinks to the
same tree are not.

Ownership is held by a kernel lock, so normal exit, a panic, and process death
all release it. Restarting after a crash is ordinary continuation.

### Supported workspace layouts

Grove keeps its controls in your workspace's VCS administration directory
(`.jj/grove/`, or the per-worktree Git directory's `grove/`), and teardown ends by
moving `.grove/` there in a single atomic rename. That rename cannot cross a
filesystem boundary, so **your working tree and its administration directory must
be on the same filesystem**.

Almost every layout satisfies this for free, because the administration directory
sits inside the working tree: plain checkouts, and native, secondary, and
colocated jj workspaces alike. The exception is a **linked Git worktree or a
submodule**, whose `.git` is a file pointing at the main repository — put one on
an external volume, a network mount, or a container bind-mount that does not
include the main repository, and the two are on different filesystems.

Grove checks this when it starts, not when you finish. An unsupported layout
exits before creating or touching anything, names both directories and their
filesystems, and tells you what to move; fix the layout and rerun. The check
happens on every run, so relocating a worktree mid-workstream is caught the next
time you start rather than months later at teardown.

## The task tree

A small workstream might look like this:

```text
.grove/
├── BRIEF.md
├── FORMAT
├── 01-DONE-requirements-plan-k1.md
├── 02-DONE-design-auth-k2.md
├── 03-review-design-auth-review-k3.md
└── 04-impl-ship-k4.md
```

A leaf is one agent-sized task. A directory is a node holding smaller tasks,
headed by its `BRIEF.md` charter — a leaf that proved bigger than one session.
The filename carries everything Grove needs:

```text
NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md
```

- `NN` is the mutable position among siblings.
- `DONE` and `ABANDONED` are the two terminal outcomes, kept in place until the
  grove finishes.
- `<session-kind>` is one of the nineteen kinds, and is what selects the command
  template from your configuration.
- `<slug>` is the human-readable name.
- `k<key>` is the permanent identity. `<slug>-k<key>` is the **stable handle**,
  the way a work item is named in commit messages; it survives renumbering and
  slug edits.

`.grove/FORMAT` records the filename grammar in use. Grove writes and reads it;
it is not a status or phase file.

Grove picks the first live leaf in depth-first pre-order. There is no hidden
scheduler and no dependency inference from prose: ordering in a grove is
contiguity, at every level. The one exception is the driver-owned `finish` leaf,
which is skipped while any other work is live. You can compute the next session
by eye with `find .grove` — the first name with no outcome infix.

Review chains and research pairs are flat siblings named off a shared stem —
`auth`, `auth-review`, `auth-integrate` above — so a listing shows the shape
without any nesting. Nothing groups them: the stem is a reading convention, not
grammar. See
[Architecture: task kinds and composition](ARCHITECTURE.md#task-kind-taxonomy).

## What happens in a session

Grove launches the configured command for the selected leaf's kind and hands it
that leaf's stable handle as an explicit mandate. The session:

1. Resolves the mandated handle and reads the glossary, ancestor briefs, cited
   decision records, and the task file.
2. Applies the discipline named by the leaf's session kind.
3. Does and verifies the work.
4. Marks the leaf `DONE`, closes any completed parent nodes, and commits all of
   that as one focused commit naming the stable handle.
5. Signals Grove, which relaunches for the next leaf.

The session does not pick its own leaf. If a new leaf is inserted ahead of the
running session's mandate, it becomes the next iteration's work rather than
preempting the session already launched.

A task that proves too large is decomposed in place into a node with its own
brief. Work that should no longer be done can be marked `ABANDONED`, but pruning
requires explicit human confirmation — Grove guides that decision and never makes
it autonomously.

The runtime methodology is [`content/SKILL.md`](../content/SKILL.md), which the
binary provisions to each installed harness's personal skill directory on every
bare `grove` invocation. It provisions the copy **embedded in the running
binary** — a session always reads the methodology its own `grove` was built
with. Those directories are global, so Grove also checks the pairing on every
iteration: it restores a skill directory another `grove` build has written, and
reports — without refusing — when the `grove-llm` a session would find on `PATH`
comes from a different build. It reports rather than refuses because it resolves
in its *own* environment, which is the session's only when your configured
command inherits it; a wrapper, login shell, `ssh` hop, or container may reach a
different binary in either direction. Editing `content/` in a checkout therefore
changes nothing any session reads until that checkout is **installed** — not
`cargo run`, which provisions the checkout's methodology beside the installed
CLI and is announced on every iteration as exactly that mismatch. Installing
means making the build you are driving the one a session's `PATH` resolves
first: `cargo install --path .` does that only if `~/.cargo/bin` outranks every
other prefix holding a `grove-llm`, and the diagnostic names the path it
actually resolved so you can tell. See
[Embedded methodology](ARCHITECTURE.md#self-extension-core-and-methodology).

`grove-llm` is the agent-facing tree interface the session drives during those
steps. It is available for diagnostics — `grove-llm pick`, `grove-llm resolve`,
`grove-llm brief-chain` — but it is not a human workflow, and running a session
outside bare `grove` gives it no mandate.

## Review composition and escalation

A session that Grove launched, and that adopted its mandate, may use at most
**one in-session** fresh-context reviewer across that whole leaf. A producer that
already has a review leaf beside it, a `review-*` session, and the three
research-pair sessions use none. An `integrate-review-*` session may use one
narrow reviewer; substantial redesign becomes a new reviewed producer beside the
leaf being integrated. Outside a Grove-launched session, the standalone
doubt-driven-development procedure is unchanged.

**A review chain is built one step at a time, by the session that needs the next
one.** There is no chain verb and no chain node — each step is an ordinary
`leaf-add`, performed as the *last act* of the session before it:

```sh
# the producer's last act, if its artifact needs an adversarial read
grove-llm leaf-add <parent> <stem>-review --kind review-<producer>

# the review's last act, if it found something worth acting on — but
# `leaf-insert <first blocking sibling entry>` instead, if there is one
grove-llm leaf-add <parent> <stem>-integrate --kind integrate-review-<producer>
```

**The integration is placed next to its review on purpose.** An
`integrate-review-*` step consumes what the review wrote down — findings anchored
to files and line numbers — and resolves them against the working tree as it
*then* stands, so anything that edits a cited file in between moves those lines
and the drift is **silent**, leaving the integrating session guessing what the
reviewer meant. A `review-*` step re-derives by contrast: its body names the
producer's stable handle, task commits name their work item by that handle, so it
finds the producer's commit and reads that diff against the current source. It
can land anywhere, which is why only one of the two hops needs care.

`leaf-add` appends at the *end* of the directory, so use `leaf-insert` for the
integration whenever there is a blocking sibling, and target **the first sibling
entry after the review whose subtree still holds live work**. *Entry*, not leaf:
`pick` descends a node directory in place, so a later sibling node with a live
leaf anywhere beneath it blocks too, and the **node directory** is the target,
never the live leaf inside it (that inserts at the wrong level). A later `DONE`
or `ABANDONED` leaf, **a node whose subtree is wholly terminal**, and the
driver's `finish` sentinel are all stepped over, so none of them blocks — and
when nothing blocks, `leaf-add` is exactly right, because the walk finishes the
review's own directory, including the leaf just appended to it, before any later
sibling of an ancestor. There is no exception: at the moment the leaf is cut, the
blocking work has not run and no leaf's eventual file set is part of its
contract, so nothing could establish one.

A review that finds nothing creates nothing and simply retires — that empty
triage session is what the lazy shape removes. But the bigger payoff is that the
**creating session writes the new leaf's body**: it can name the exact case the
producer could not cover, or carry the findings verbatim, which is strictly more
than a constructor rendering a goal sentence from a handle could ever supply.

The producer finishes only to a coherent **reviewable boundary**, **retires
itself, then** commits the artifact, the new review leaf and that retirement
together under its own handle, and hands control back to Grove. Retirement comes
before the commit because the `DONE` rename belongs to this task: commit first
and the rename is either left uncommitted or swept into the next task's change.
Nothing about the producer's leaf moves, so its stable handle and bytes are
preserved by construction.

Write the relationship into the new leaf's body by hand — `**Reviews:**
<producer-handle>`, or `**Integrates:** <review-handle>`. Grove neither writes
nor reads those lines; they are a convention for you and for the session that
picks the step up.

Grove then launches the review kind's configured command. Whether that command
differs in harness or model from the producer's is **your** configuration policy:
Grove executes opaque command strings, so it cannot compare two targets, and it
records no launch receipts and emits no diversity warnings. The tree guarantees a
fresh session; choosing a materially different command is up to the configuration
owner.

Pruning only the producer leaves its review live and next, deliberately
uncheckable. To abandon the whole reviewed path, prune each of its live steps —
usually just the one, since a review leaf exists only because a producer decided
review was required.

Research is the exception that stays eager: `grove-llm leaf-add-pair <parent>
<stem>` cuts all three steps in one all-or-nothing call. If `research-a` cut
`research-b` at the end of its own session, `b` would inherit `a`'s framing and
corpus — destroying the independence the pair is run for.

## Finish

When the last live leaf is retired, Grove materializes a `finish` leaf and
launches a session that proposes one complete finish cycle:

1. Promote durable knowledge from the briefs into the repository's normal docs,
   decision records, specs, or context files where it still belongs.
2. Tear `.grove/` down through Grove's finish transaction, which records the
   deletion in one focused commit.
3. Signal that the grove is done, stopping the loop cleanly.

This is Grove's one routine human confirmation point, because it deletes the
workstream tree. Declining, or exiting before teardown begins, writes no signal
and leaves the finish leaf live for a later `grove`. If new work appears after
the finish session launched, teardown refuses and names that work, leaving the
tree untouched for the next iteration.

Branch or bookmark integration and working-tree teardown remain yours; Grove
never creates, merges, or removes them. Whoever integrates should do so after
step 2, so the integrated history never carries `.grove/`.

### What teardown guarantees

Step 2 runs as one fail-closed transaction rather than a plain delete and commit.
`.grove/` stays present — visible, and refused by every ordinary Grove command —
until the repository has proven the exact commit that records its deletion. Its
contents are held under a `FINISHING-…` directory inside the tree while that
happens.

What this means for you:

- The deletion commit touches only `.grove/`. Unrelated staged changes,
  working-tree edits, and Jujutsu working-copy changes are preserved, and plain
  Git runs this internal commit with hooks disabled, because an arbitrary hook
  could modify files the transaction promises to leave alone.
- If teardown fails or the session dies mid-way, you get either your live
  workstream tree back — rerun and it retries — or a blocked tree that says
  exactly what is wrong. You never get a half-deleted tree, and an absent
  `.grove/` is never taken as evidence that teardown succeeded.
- A blocked teardown reports **`Recovery pending`** and names the directory
  holding it, what repository state it recorded, and what it observed instead. It
  offers two ways out: preserve any divergent work and restore the recorded
  starting state so it can roll back, or make the exact teardown commit the
  current result so it can finish forward — then rerun. Grove will not reset,
  rebase, or rewrite history on your behalf, so nothing you did outside Grove is
  discarded to unblock it.
- After a successful teardown, the tree's bytes move to a quarantine directory
  inside your VCS administration directory (`.git/` or `.jj/`) and are deleted
  from there. That quarantine is disposable cleanup, never workflow state; a
  later Grove run tidies up any a crash left behind.
- Once `.grove/` is gone, that workstream is over. A later `grove` in the same
  tree starts a **new** grove rather than recovering the finished one — Grove
  reads no VCS history to tell "recover" from "start again".

For why these boundaries exist, see [ARCHITECTURE.md](ARCHITECTURE.md).
