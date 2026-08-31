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

This guide covers the whole installed command surface — the `grove` binary you
run, and the `grove-llm` verbs a session runs over the tree — plus every journey
from scaffolding a grove to tearing one down. It does **not** carry the
configuration schema ([CONFIGURATION.md](CONFIGURATION.md)), installation
([README](../README.md)), grove's vocabulary ([CONTEXT.md](../CONTEXT.md)), or
the methodology a session executes (the `grove` plugin's own skills). Each is
linked where it is reached. The set this guide is obliged to cover is written
down separately, in
[the coverage inventory](specs/user-guide-coverage.md), and the
[coverage map](#usage-coverage-map) at the end says which section answers each
row of it.

Every transcript below is real output with the working tree rewritten to
`/home/you/app`.

<a id="usage-running-grove"></a>
## Running Grove: start, resume, and finish

Run Grove from anywhere inside the working tree:

```console
$ grove
grove: launching requirements with configured "claude" — plan-k1
```

That is the whole human command surface. There are no subcommands and no
lifecycle flags — `grove --help` and `grove --version` are the only other
arguments, and both stop before Grove touches a repository:

```console
$ grove --help
Grove: hierarchical workstream tool for AI agents

Usage: grove

Options:
  -h, --help     Print help
  -V, --version  Print version

$ grove --version
grove 20.1.0
```

Bare `grove` inspects the filesystem and does the appropriate next thing:

| What it finds | What it does |
|---|---|
| No `.grove/` | Creates the root brief and a first `requirements` leaf, then launches it. |
| Live leaves | Launches the first one in tree order. |
| No live leaves | Materializes a `finish` leaf and launches the teardown session. |

It then keeps going: when a session signals that its task is complete, Grove
relaunches with fresh context for the next leaf. Any other ending — you exit the
session, press Ctrl-C, or the process dies — stops the loop, and Grove says which
ending it was:

```console
grove: launching impl with configured "claude" — api-k7
grove: session ended without a completion signal — status exit status: 0, elapsed 41.106s; loop stopped.
```

A session that signalled the grove's *end* rather than one task's prints
`grove: grove finished — loop complete.` instead.

Because `grove` takes no arguments, **the working directory is the only thing
that selects a workstream**. There is no tree to name and no confirmation step:
Grove scaffolds and commits against whichever working tree encloses the
directory you ran it from. That is what makes the command short, and it is worth
knowing before you run it in a repository that holds several Jujutsu workspaces —
running it in the wrong one starts a grove there, not in the one you meant.
`jj op restore` recovers it if that happens; see
[Undoing a mistake](#undoing-a-mistake) below.

To resume, run `grove` again. Grove has no progress database; it re-derives its
position from the task tree every iteration, which is what makes restart and
continuation the same thing.

Full configuration validation precedes every one of those tree mutations, so a
missing or malformed `config.kdl` leaves your working tree byte-identical.

Grove makes one commit of its own — the teardown commit at the end. It touches
only `.grove/`, leaving unrelated working-copy changes in the successor commit.
Repository failures surface normally, and `jj op restore` undoes the commit like
any other jj operation.

### If the tree is not jj-enabled

Grove refuses before creating anything and names both remedies:

```console
$ grove
Error: not a Jujutsu working tree
  looked for a `.jj` directory at and above: /home/you/app

Make the tree jj-enabled and rerun:
      jj git init --colocate     # an existing Git repository, history kept
      jj git init                # no repository here yet

Nothing was created or changed.
```

`jj git init --colocate` in an existing Git repository keeps every commit and
leaves `git` itself working; `jj git init` is for a directory with no repository
at all. Run one and rerun `grove`. Jujutsu's own binary must be on `PATH` —
Grove asks jj for facts it will not guess, and says so if it cannot run it.

### Stopping the loop

**Ctrl-C is the session's, not the loop's.** While a session runs it owns the
terminal in its own right, so an interrupt you type reaches the session and
leaves the driver standing to make the relaunch-or-stop decision. Ending the
loop from outside is `kill` on the `grove` process:

```console
# the shell running the loop
$ grove
grove: launching impl with configured "claude" — api-k7
grove: interrupted by signal 15 — stopping the loop.
$ echo $?
143
```

```console
# a second shell
$ kill -TERM "$(pgrep -x grove)"
$ echo $?
0
```

**Read the status where Grove exits, not where you signal it.** `$?` after the
`kill` is the *signal delivery*'s status and is `0` whenever the signal was
sent; only the wait status of the `grove` process itself carries `128 + N`. A
wrapper script that backgrounds Grove reads it with `wait "$grove_pid"`.

It forwards the signal to the session's whole process group, waits for it,
restores your terminal, and then **dies of the same signal** — so a wrapper
script, a `timeout`, or a systemd unit reads `128 + N` and can tell an
interrupted grove from a finished one. A grove that ran to a clean finish, or
stopped because a session ended without signalling, exits `0`.

<a id="undoing-a-mistake"></a>
### Undoing a mistake

Grove writes through Jujutsu and hand-builds no transaction of its own, so
jj's operation log is the undo:

```console
$ jj op log --limit 3
$ jj undo                    # reverse the most recent operation
$ jj op restore <operation>  # go back to a named point
```

That covers the two mistakes worth naming: a grove started in the workspace you
did not mean, and a teardown commit you want back. `jj undo` reverses the last
operation; `jj op restore` returns the whole repository to a chosen one. Grove
runs no recovery of its own and never rewrites history on your behalf.

<a id="usage-task-tree"></a>
## The task tree and its filename grammar

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
- `<session-kind>` is any well-formed token — lowercase ASCII letters, digits and
  single dashes, no `--` — and is what selects the command template from your
  configuration and names the skill a session loads. Grove holds no list of
  kinds; the ones that exist are the `grove-<kind>` skills you have installed.
  The `--` between the kind and the slug is what makes the name unambiguous.
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
which is skipped while any other work is live.

**Reading the tree by eye.** Because the state is entirely in the names, you can
answer "what runs next?" without running anything — the first name in
depth-first order with no outcome infix:

```console
$ find .grove -name '*.md' | sort
.grove/01-DONE-requirements--plan-k1.md
.grove/02-DONE-design--auth-k2.md
.grove/03-review-design--auth-k3.md
.grove/04-ship-k4/01-impl--api-k7.md
.grove/04-ship-k4/BRIEF.md
.grove/BRIEF.md
```

`03-review-design--auth-k3.md` is next: it is the first entry carrying neither
`DONE` nor `ABANDONED`. `grove-llm pick` computes the same answer, and
[the verbs section](#usage-tree-verbs) shows it; the point of the grammar is that
you do not need it.

Review chains and research pairs are flat siblings named off a shared stem — the
`auth` producer and its `review-design` step above share one slug and differ only
by kind and key — so a listing shows the shape without any nesting. The kind
states each step's role, so the slug does not restate it. Nothing groups them: the
stem is a reading convention, not grammar. See
[Architecture: task kinds and composition](ARCHITECTURE.md#task-kind-taxonomy).

<a id="usage-session-lifecycle"></a>
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

Watching one leaf through, from the outside, is three lines of driver output:

```console
grove: launching impl with configured "claude" — api-k7
grove: launching review-impl with configured "claude" — api-k8
```

The second line is the whole of step 5 as you see it: the session ran
`grove-llm complete`, which wrote the relaunch flag; Grove ended that session and
launched the next leaf with fresh context. Between the two lines the session made
its own commit, so `jj log` shows `api-k7: …` before the relaunch.

The session does not pick its own leaf. If a new leaf is inserted ahead of the
running session's mandate, it becomes the next iteration's work rather than
preempting the session already launched.

A task that proves too large is decomposed in place into a node with its own
brief. Work that should no longer be done can be marked `ABANDONED`, but pruning
requires explicit human confirmation — Grove guides that decision and never makes
it autonomously. Both are verbs, and
[the verbs section](#usage-tree-verbs) works them.

The runtime methodology is the **`grove` plugin**, whose spine is
[`plugins/grove/skills/grove/SKILL.md`](../plugins/grove/skills/grove/SKILL.md)
and which ships one `grove-<kind>` skill per session kind. A launch prompt names
the one skill this session's kind needs, and the session loads it through its
harness's own skill-loading affordance. **A kind exists iff a skill of that name
exists** — adding one is authoring a skill and declaring a template for it, never
editing or rebuilding a binary.

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

<a id="usage-tree-verbs"></a>
## The `grove-llm` verbs over the tree

`grove-llm` is the agent-facing tree interface a session drives during those
steps. Its own help text says the verbs "are [not] meant for direct human use",
and that is a statement about who *drives* them, not about who has to understand
them: reading a task tree means reading the result of these verbs, four of them
are diagnostics you may want by hand, and one is gated on your confirmation and
happens nowhere else.

What follows is what each verb does to the tree, what it prints, and whether it
commits. It is not the calling contract a session works from — that is the
`--help` text and the methodology skills, and **each verb's own `--help` is the
authority on what exists**:

```console
$ grove-llm --help          # the twelve verbs, one line each
$ grove-llm leaf-add --help # one verb's arguments and flags
$ grove-llm help leaf-add   # the same, spelled as a verb
$ grove-llm --version
grove-llm 20.1.0
```

Two facts nearly hold across the verbs below, and the three verbs that break them
are named rather than glossed over. **Only `finish-commit` commits** — every
other verb makes a working-tree change and leaves it there, for the session that
ran it to commit with its own work. And a verb's *answer* is absolute paths on
stdout, with diagnostics on stderr, so a caller can read it without parsing
prose. The exceptions:

| Verb | Commits? | On stdout |
|---|---|---|
| `kind` | No | A kind **token**, not a path — the one answer that is not a location. |
| `finish-commit` | **Yes** — deletes `.grove/` and commits that deletion | Nothing; the change id goes to stderr. |
| `complete` | No | Nothing, ever; it reports what it signalled on stderr. |

One thing to expect if you run these inside a running session's own terminal:
`grove-llm` binds itself to the working tree that session was launched for, and
refuses another (`Error: wrong working tree for grove-llm <verb>: …`). Run them
in your own shell instead.

### Reading the tree

`pick` prints the next live leaf — the same depth-first pre-order answer the
driver computes:

```console
$ grove-llm pick
/home/you/app/.grove/01-requirements--auth-k1.md
```

`kind` prints just that leaf's kind token, and `brief-chain` prints the
`BRIEF.md` chain root→leaf, one absolute path per line. Both default to `pick`'s
leaf and both take an optional explicit leaf path:

```console
$ grove-llm kind
requirements

$ grove-llm brief-chain
/home/you/app/.grove/BRIEF.md

$ grove-llm brief-chain 04-ship-k3/01-impl--api-k7.md
/home/you/app/.grove/BRIEF.md
/home/you/app/.grove/04-ship-k3/BRIEF.md
```

A directory level with no `BRIEF.md` is skipped silently, so a short chain means
a level had no charter, never that the walk stopped. When nothing is live all
three print the same diagnostic on stderr, nothing on stdout, and exit `0`:

```console
$ grove-llm pick
grove app: no live leaves; this grove is done
```

`resolve` turns a reference into a path. It searches live, `DONE` and
`ABANDONED` entries alike, and accepts a permanent key (`[n]`, bare `n`, or
`[n]-slug`), a full `<slug>-k<key>` handle, or a bare slug:

```console
$ grove-llm resolve 1
/home/you/app/.grove/01-requirements--auth-k1.md

$ grove-llm resolve auth-k1
/home/you/app/.grove/01-DONE-requirements--auth-k1.md
note: referenced task is retired (DONE): /home/you/app/.grove/01-DONE-requirements--auth-k1.md
```

A node resolves to its **directory** — append `/BRIEF.md` to read its charter:

```console
$ grove-llm resolve ship-k3
/home/you/app/.grove/04-ship-k3
```

A bare slug that several entries share is ambiguous, and the diagnostic lists
each match's key so you can re-query by key. Not-found says so. Both still exit
`0`, because "which entry is this?" has an answer either way:

```console
$ grove-llm resolve auth
resolve: reference "auth" is ambiguous; re-query by key:
  [1] /home/you/app/.grove/01-requirements--auth-k1.md
  [2] /home/you/app/.grove/02-design--auth-k2.md
  [4] /home/you/app/.grove/05-review-design--auth-k4.md

$ grove-llm resolve nosuch
resolve: no entry matches reference "nosuch"
```

### Growing the tree

`root-init` scaffolds a brand-new grove: `.grove/`, the root `BRIEF.md` charter,
and a first `requirements` leaf whose slug defaults to `plan`. It prints the
charter's path, then the leaf's. **Bare `grove` does this for you**, so you run
it by hand only to lay a tree down without launching a session:

```console
$ grove-llm root-init auth
/home/you/app/.grove/BRIEF.md
/home/you/app/.grove/01-requirements--auth-k1.md
```

It refuses rather than merging into a grove that already exists:

```console
$ grove-llm root-init
Error: grove root already exists: /home/you/app/.grove
```

`leaf-add` appends work under a parent — `.` for the grove root, or a node by
key or path. `--kind` is required and **repeatable**, and one call adds one leaf
per `--kind` in the order given, as a single unit, all carrying the same slug:

```console
$ grove-llm leaf-add . ship --kind impl
/home/you/app/.grove/03-impl--ship-k3.md

$ grove-llm leaf-add . auth --kind review-design --kind integrate-review-design
/home/you/app/.grove/04-review-design--auth-k4.md
/home/you/app/.grove/05-integrate-review-design--auth-k5.md
```

The kind is what tells a shape's steps apart, which is why they share the slug.
`finish` is the driver's own and is refused here:

```console
$ grove-llm leaf-add . x --kind finish
Error: `finish` is driver-reserved and cannot be created by `leaf-add`
```

`leaf-insert` puts a new leaf at the slot an existing entry holds, shifting that
entry and every later sibling up one. It takes exactly one `--kind`. The new leaf
gets a fresh key; the shifted subtrees keep their own names and keys, and **no
file contents are rewritten** — which is why the renumber summary on stderr also
warns that position-prefixed cross-references in your prose are yours to fix:

```console
$ grove-llm leaf-insert 3 spike --kind prototype
/home/you/app/.grove/03-prototype--spike-k6.md
leaf-insert spike: renumbered 3 siblings:
  03 -> 04  (04-impl--ship-k3.md)
  04 -> 05  (05-review-design--auth-k4.md)
  05 -> 06  (06-integrate-review-design--auth-k5.md)
cross-references to review (verb does not auto-rewrite):
```

**Decomposing an oversized leaf** is `leaf-decompose`: the leaf file becomes a
node directory *with its key preserved*, its body moves in as the node's
`BRIEF.md`, and a first child is grown atomically so the node is never childless.
The child inherits the decomposed leaf's kind unless `--kind` overrides it:

```console
$ grove-llm leaf-decompose 04-impl--ship-k3.md api
/home/you/app/.grove/04-ship-k3/BRIEF.md
/home/you/app/.grove/04-ship-k3/01-impl--api-k7.md
```

`04-impl--ship-k3.md` is now `04-ship-k3/`, still `k3`, so every reference to
the handle `ship-k3` still resolves — to the directory.

### Ending work

`leaf-retire` marks a live leaf done in place by adding the `DONE` infix. The
leaf keeps its position and key, and the file's contents are untouched:

```console
$ grove-llm leaf-retire .grove/01-requirements--auth-k1.md
/home/you/app/.grove/01-DONE-requirements--auth-k1.md
leaf-retire: two steps remain:
  1. commit this session's work, including this rename
  2. run `grove-llm complete` as your last action
```

It refuses a brief, an already-`DONE` leaf, and an `ABANDONED` one — there is no
second retirement of anything.

`leaf-prune` marks abandoned work `ABANDONED` in place, and **this is the one
verb gated on you**. Grove never abandons planned work on its own; a session that
reaches the question stops and asks. Given a node it marks every *live* leaf in
the subtree, leaving `DONE` ones alone, because that work really was done:

```console
$ grove-llm leaf-prune 04-ship-k3
/home/you/app/.grove/04-ship-k3/01-ABANDONED-impl--api-k7.md
leaf-prune: two steps remain:
  1. commit this session's work, including this rename
  2. run `grove-llm complete` as your last action
```

It refuses the grove root, because abandoning a whole workstream is a
branch-delete rather than a tree mark:

```console
$ grove-llm leaf-prune .grove
Error: cannot prune the grove root (abandoning a whole grove is a branch-delete, not a tree mark)
```

Pruning only a producer leaves its review leaf live and next, deliberately
uncheckable; see [Review composition](#usage-review-composition).

**Closing a node has no verb, and that is deliberate.** A node is never marked:
its done-ness *is* the absence of a live leaf anywhere in its subtree, so `pick`
walks past a fully terminal directory without being told to. What a close costs
is judgement rather than a rename — the session that retires the last child
checks the node's `BRIEF.md` `Done when` against what the subtree delivered,
`leaf-add`s any nameable gap, promotes what is still relevant up to the parent
brief or a decision record, and names the node's handle in its commit message
alongside the leaf's. You review a close after the fact, in that diff.

### Cutting a research pair

Research is the one shape cut eagerly, in a single all-or-nothing call, because
independence is the property being bought:

```console
$ grove-llm leaf-add . protocols --kind research-a --kind research-b --kind combine-research
/home/you/app/.grove/06-research-a--protocols-k8.md
/home/you/app/.grove/07-research-b--protocols-k9.md
/home/you/app/.grove/08-combine-research--protocols-k10.md
```

If `research-a` cut `research-b` at the end of its own session, `b` would inherit
`a`'s framing and corpus — destroying the independence the pair is run for; and
three separate calls would leave a live prefix indistinguishable from a
deliberately hand-cut partial pair. There is no pair verb: `leaf-add` takes an
ordered list of kinds, so the three tokens are spelled by the methodology that
owns them rather than held in the binary.

### Ending the session, and ending the grove

`complete` is a session's last action, after it has committed and retired. It
writes the relaunch flag to the signal file the driver watches and returns;
ending the session is the driver's job. `--signal-file` defaults to
`$GROVE_SIGNAL_FILE`, which the driver sets, and `--done` ends the whole grove
instead of relaunching:

```console
$ grove-llm complete            # this task is done; relaunch for the next leaf
grove complete: signalled; the loop will start the next task.

$ grove-llm complete --done     # the last action of the Finish cycle
grove complete: signalled; the grove is finished — the loop will stop.
```

Run outside a loop it is a safe no-op that tells you so, and still exits `0`:

```console
$ grove-llm complete
grove complete: no GROVE_SIGNAL_FILE — not running under the loop driver; exit this session manually.
```

`finish-commit` is the teardown, covered in [Finish](#usage-finish) below. It
takes the launched finish leaf's stable handle, revalidates under the tree lock
that this really is the live finish leaf and that no ordinary work has appeared,
and only then deletes and commits `.grove/`. On success it names the change it
made, on stderr:

```console
$ grove-llm finish-commit finish-k42
finish-commit finish-k42: committed as qrsuvwxy
$ ls .grove
ls: cannot access '.grove': No such file or directory
```

That commit is path-scoped to `.grove/` alone, so anything else in your working
copy is left uncommitted and untouched. It is the **one verb here that commits**,
and the deletion it records is the only thing in it.

It **does not** stand in for the human confirmation the finish session owes you —
it enforces tree and VCS facts and infers nothing about your consent. Both of its
refusals leave the tree exactly as it was; this is the stale-handle one, and
[Finish](#usage-finish) below shows the live-work one:

```console
$ grove-llm finish-commit finish-k42
Error: `grove-llm finish-commit finish-k42`

Caused by:
    the requested finish leaf is no longer live
```

<a id="usage-review-composition"></a>
## Review composition and escalation

Review in Grove happens two ways, and only one of them is yours to see in the
tree. A session may spend at most **one in-session** fresh-context reviewer over
its whole leaf — *which* kinds may spend it, and on what, is methodology the
`grove` plugin's skills own (`references/execute.md` in the `grove` skill) and
this guide does not restate. That reviewer leaves no trace in `.grove/`. What
does, and what this section covers, is the other way: a **review leaf**, a whole
session of its own, standing beside the producer as a file you can read.

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

<a id="usage-driver-lease"></a>
## One driver per working tree

A working tree can have only one live Grove driver. A second `grove` in the same
tree exits immediately, names the canonical working tree, and leaves the existing
driver as owner:

```console
$ grove
Error: another Grove driver already owns /home/you/app; the existing Grove driver must stop before this one can start
```

It does not queue, because two drivers would issue two mandates for the same
task. Stop the first — `kill` on its `grove` process, or let it finish — and
rerun. Different jj workspaces are independent even when they share a repository;
path aliases and symlinks to the same tree are not, which is why the refusal
names the canonical path rather than the one you typed.

Ownership is held by a kernel lock, so normal exit, a panic, and process death
all release it. Restarting after a crash is ordinary continuation.

<a id="usage-workspace-layouts"></a>
## Supported workspace layouts

Grove resolves the working tree by walking up from your current directory to the
first `.jj/` directory, and keeps its own coordination files — the driver lease,
the session epoch, the signal file — in that workspace's `.jj/grove/`. Three
consequences are worth knowing.

**Every jj workspace layout works.** Native (`jj git init`), colocated
(`jj git init --colocate`, where `.git` and `.jj` sit side by side), and
secondary workspaces created with `jj workspace add` are all ordinary to Grove.
A secondary workspace borrows the default workspace's repository, and Grove asks
jj which one that is rather than guessing — one more reason its binary has to be
on `PATH`.

**Each workspace is its own grove.** The lease, the control directory and the
`.grove/` tree all belong to the workspace you ran `grove` in, so two workspaces
sharing one repository run two independent drivers over two independent
workstreams. That is the behaviour to keep in mind before running `grove` in a
repository with several workspaces checked out — see
[Running Grove](#usage-running-grove).

**The control directory must be usable.** Grove creates `.jj/grove/` on start
and refuses if it cannot, naming the directory and the underlying cause:

```console
$ grove
Error: the control directory /home/you/app/.jj/grove is not usable: Permission denied (os error 13)

It must exist and be writable before anything can coordinate through it. Check the permissions on the workspace's `.jj` directory.
```

Nothing is created or changed when that refusal lands. Fix the permissions on
`.jj/` and rerun. Because the control directory lives under `.jj/` rather than in
the tracked working copy, jj never snapshots it, and Grove's coordination files
never turn up in a commit.

<a id="usage-finish"></a>
## Finish

When the last live leaf is retired, Grove materializes a `finish` leaf and
launches a session that proposes one complete finish cycle:

1. Promote durable knowledge from the briefs into the repository's normal docs,
   decision records, specs, or context files where it still belongs.
2. Tear `.grove/` down with `grove-llm finish-commit <finish-handle>`, which
   deletes the tree and records the deletion in one focused commit.
3. Signal that the grove is done — `grove-llm complete --done` — stopping the
   loop cleanly.

```console
grove: launching finish with configured "claude" — finish-k42
grove: grove finished — loop complete.
```

This is Grove's one routine human confirmation point, because it deletes the
workstream tree. Declining, or exiting before teardown begins, writes no signal
and leaves the finish leaf live for a later `grove`. If new work appears after
the finish session launched, teardown refuses and names that work, leaving the
tree untouched for the next iteration:

```console
$ grove-llm finish-commit finish-k42
Error: cannot finish while live work remains: api-k7 (/home/you/app/.grove/04-ship-k3/01-impl--api-k7.md)
```

**Integration is yours, and it comes after step 2.** Branch or bookmark
integration and working-tree teardown remain yours; Grove never creates, merges,
or removes them. Whoever integrates should do so after teardown has committed, so
the integrated history never carries `.grove/`.

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
  what it tracks. A `.grove` that is a symlink to a directory elsewhere is
  refused too: Grove will not delete a tree that is not its own.
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

<a id="usage-coverage-map"></a>
## Coverage map

Every row of [the coverage inventory](specs/user-guide-coverage.md), and the
section of this guide that answers it. It is here so completeness is checkable
row by row rather than asserted in aggregate, and
`crates/grove/tests/user_guide_coverage.rs` compares the two documents, so a row
added to the inventory and forgotten here is a test failure rather than a silence.

| Row | Covered in |
|---|---|
| G1 | [Running Grove](#usage-running-grove) |
| G2 | [Running Grove](#usage-running-grove) |
| G3 | [Running Grove](#usage-running-grove) |
| G4 | [Running Grove](#usage-running-grove) |
| G5 | [Stopping the loop](#stopping-the-loop) |
| L1 | [Growing the tree](#growing-the-tree) |
| L2 | [Reading the tree](#reading-the-tree) |
| L3 | [Reading the tree](#reading-the-tree) |
| L4 | [Reading the tree](#reading-the-tree) |
| L5 | [Reading the tree](#reading-the-tree) |
| L6 | [Growing the tree](#growing-the-tree) |
| L7 | [Growing the tree](#growing-the-tree) |
| L8 | [Growing the tree](#growing-the-tree) |
| L9 | [Ending work](#ending-work) |
| L10 | [Ending work](#ending-work) |
| L11 | [Ending the session, and ending the grove](#ending-the-session-and-ending-the-grove) |
| L12 | [Ending the session, and ending the grove](#ending-the-session-and-ending-the-grove) |
| L13 | [The `grove-llm` verbs over the tree](#usage-tree-verbs) |
| J1 | [Running Grove](#usage-running-grove) |
| J2 | [If the tree is not jj-enabled](#if-the-tree-is-not-jj-enabled) |
| J3 | [Running Grove](#usage-running-grove) |
| J4 | [What happens in a session](#usage-session-lifecycle) |
| J5 | [Stopping the loop](#stopping-the-loop) |
| J6 | [The task tree and its filename grammar](#usage-task-tree) |
| J7 | [Growing the tree](#growing-the-tree) |
| J8 | [Review composition and escalation](#usage-review-composition) |
| J9 | [Cutting a research pair](#cutting-a-research-pair) |
| J10 | [Ending work](#ending-work) |
| J11 | [Ending work](#ending-work) |
| J12 | [Undoing a mistake](#undoing-a-mistake) |
| J13 | [One driver per working tree](#usage-driver-lease) |
| J14 | [Supported workspace layouts](#usage-workspace-layouts) |
| J15 | [Finish](#usage-finish) |
| J16 | [Finish](#usage-finish) |
