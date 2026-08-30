# Using Grove

Grove drives a long-running workstream as a sequence of fresh agent sessions.
The workstream lives in a `.grove/` task tree inside a **Jujutsu** working tree
that you create and own. Grove drives jj and nothing else: run it in a tree with
no `.jj/` and it stops before touching anything, naming
`jj git init --colocate` — which makes an existing Git repository jj-enabled
while keeping its history and leaving every Git tool working.

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
| Live leaves | Launches the first one in tree order. |
| No live leaves | Materializes a `finish` leaf and launches the teardown session. |

It then keeps going: when a session signals that its task is complete, Grove
relaunches with fresh context for the next leaf. Any other ending — you exit the
session, press Ctrl-C, or the process dies — stops the loop.

Because `grove` takes no arguments, **the working directory is the only thing
that selects a workstream**. There is no tree to name and no confirmation step:
Grove scaffolds and commits against whichever working tree encloses the
directory you ran it from. That is what makes the command short, and it is worth
knowing before you run it in a repository that holds several Jujutsu workspaces —
running it in the wrong one starts a grove there, not in the one you meant.
`jj op restore` recovers it if that happens.

To resume, run `grove` again. Grove has no progress database; it re-derives its
position from the task tree every iteration, which is what makes restart and
continuation the same thing.

Full configuration validation precedes every one of those tree mutations, so a
missing or malformed `config.kdl` leaves your working tree byte-identical.

Grove makes one commit of its own — the teardown commit at the end. It touches
only `.grove/`, leaving unrelated working-copy changes in the successor commit.
Repository failures surface normally, and `jj op restore` undoes the commit like
any other jj operation.

### One driver per working tree

A working tree can have only one live Grove driver. A second `grove` in the same
tree exits immediately, names the canonical working tree, and leaves the existing
driver as owner — it does not queue, because two drivers would issue two
mandates for the same task. Different jj workspaces are independent even when
they share a repository; path aliases and symlinks to the same tree are not.

Ownership is held by a kernel lock, so normal exit, a panic, and process death
all release it. Restarting after a crash is ordinary continuation.

### Supported workspace layouts

Grove keeps its controls in your workspace's `.jj/grove/`, and teardown ends by
moving `.grove/` there in a single atomic rename. That rename cannot cross a
filesystem boundary, so **your working tree and its `.jj/` directory must be on
the same filesystem**.

Every ordinary layout satisfies this for free, because `.jj/` sits at the root of
the working tree: native, secondary, and colocated jj workspaces alike. It fails
only where `.jj/` has itself been put elsewhere — a mount point, or a symlink
onto another volume.

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
├── 01-DONE-requirements--plan-k1.md
├── 02-DONE-design--auth-k2.md
├── 03-review-design--auth-k3.md
└── 04-impl--ship-k4.md
```

A leaf is one agent-sized task. A directory is a node holding smaller tasks,
headed by its `BRIEF.md` charter — a leaf that proved bigger than one session.
The filename carries everything Grove needs:

```text
NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md
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

There is no witness file and no format stamp: the filenames *are* the format. A
tree whose names this grammar does not spell is refused by name, with the
offending path and the shape it should have had — Grove does not migrate an
older layout.

Grove picks the first live leaf in depth-first pre-order. There is no hidden
scheduler and no dependency inference from prose: ordering in a grove is
contiguity, at every level. The one exception is the driver-owned `finish` leaf,
which is skipped while any other work is live. You can compute the next session
by eye with `find .grove` — the first name with no outcome infix.

Review chains and research pairs are flat siblings named off a shared stem — the
`auth` producer and its `review-design` step above share one slug and differ only
by kind and key — so a listing shows the shape without any nesting. The kind
states each step's role, so the slug does not restate it. Nothing groups them: the
stem is a reading convention, not grammar. See
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

The runtime methodology is the **`grove` plugin**, whose spine is
[`plugins/grove/skills/grove/SKILL.md`](../plugins/grove/skills/grove/SKILL.md)
and whose nineteen `grove-<kind>` skills carry one session kind each. A launch
prompt names the one skill this session's kind needs, and the session loads it
through its harness's own skill-loading affordance.

**Grove does not install it, and does not check that it is there.** The binary
used to carry the methodology as an embedded `content/` tree and sweep it into
every installed harness's personal skill directory on each bare `grove`
invocation; that is gone, along with the build-pairing report the shared
directories made necessary. Installing the plugin is a human step — see
[Install the skill plugins](../README.md#install-the-skill-plugins) — and the
cost of the change is that a session can now be launched pointing at a skill
that is not installed. Grove states the version it is and names the skill; a
harness with a skill-loading affordance reads it, and one without has lost its
fallback.

Editing the methodology in a checkout reaches a session as soon as the plugin
resolves to that checkout, which for the symlink farm is immediately and for
Claude Code is the next marketplace update — the build boundary that used to sit
between an edit and a session is gone with the embed.

`grove-llm` is the agent-facing tree interface the session drives during those
steps. It is available for diagnostics — `grove-llm pick`, `grove-llm resolve`,
`grove-llm brief-chain` — but it is not a human workflow, and running a session
outside bare `grove` gives it no mandate.

## Two habits for the human in the loop

Both are yours rather than the session's — a session *is* the LLM and cannot
perform either on itself. The session-facing counterparts are already in the
methodology: naming the trade-off you want input on
([`plugins/grove/skills/grove/references/execute.md`](../plugins/grove/skills/grove/references/execute.md))
and giving a recommended answer per question
([`plugins/grove/skills/grove-requirements/grilling.md`](../plugins/grove/skills/grove-requirements/grilling.md)).

**Ask the LLM "WDYT" before committing.** When a question feels close to
settled, the easy default is to nod and move on. Don't — ask what it thinks,
explicitly, including when you already hold a strong view. Two things happen.
It produces a recommendation it would otherwise suppress out of deference,
informed by the bootstrap context (glossary, briefs, decision records,
research), which is exactly the evidence base you want surfaced before you
commit. And where its recommendation diverges from yours, that divergence is
the cheapest signal available that the question still has something to teach —
interrogate it rather than dismissing it.

**Ask for pushback when the LLM agrees too easily.** Models default to
agreement under social pressure. When one agrees with your proposal without
surfacing trade-offs, push back yourself: "what would push you toward the other
option?", "what breaks if we do it this way?", or simply "pushback please". The
grilling format is built around recommended-answers-with-evidence precisely to
make pushback structural rather than personal — when the recommendation cites
primary evidence, the pushback is a debate about that evidence rather than
about whose preference wins.

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
grove-llm leaf-add <parent> <stem> --kind review-<producer>

# the review's last act, if it found something worth acting on — but
# `leaf-insert <first blocking sibling entry>` instead, if there is one
grove-llm leaf-add <parent> <stem> --kind integrate-review-<producer>
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
producer could not cover, which is strictly more than a constructor rendering a
goal sentence from a handle could ever supply. An **integration's** body is the
exception, and it carries the review's *handle* rather than its findings: a body
that is the finding list makes that list the integration's charter, leaving it no
structural place to reject one. The findings are read from the review's own
commit instead.

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
2. Tear `.grove/` down with `grove-llm finish-commit`, which deletes the tree
   and records the deletion in one focused commit.
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

Step 2 is a plain deletion followed by a path-scoped `jj commit`, and **Grove
implements no transaction around it**. It does not need one: Jujutsu snapshots
the working copy before every command and its operation log is the transaction
record, so the guarantees a hand-built transaction would offer are already
yours, from the tool that owns them.

What this means for you:

- The deletion commit touches only `.grove/`. Unrelated working-copy changes are
  preserved, staying uncommitted rather than being swept into the teardown.
- Before deleting anything, Grove revalidates that the live leaf really is the
  finish leaf you named and that no ordinary work has appeared, and refuses a
  task tree Jujutsu does not track — because the operation log can only restore
  what it tracks.
- If teardown fails part way, Grove stops and names the command that puts the
  tree back: `jj restore .grove` if the deletion is what failed, `jj undo` if the
  commit is. **No Grove-authored recovery runs.** Once the tree is back, fix what
  failed and rerun the same command with the same handle. Grove will not reset,
  rebase, or rewrite history on your behalf, so nothing you did outside Grove is
  discarded to unblock it.
- `jj op log` is where you look if you are unsure what happened. It is the record
  of every operation, including the snapshot that captured the deletion.
- Once `.grove/` is gone, that workstream is over. A later `grove` in the same
  tree starts a **new** grove rather than recovering the finished one — Grove
  reads no VCS history to tell "recover" from "start again".

For why these boundaries exist, see [ARCHITECTURE.md](ARCHITECTURE.md).
