# Grove Architecture

Grove is a small Rust launcher around a filesystem task tree and an embedded
agent methodology. Durable work stays in ordinary repository files and VCS;
Grove adds only enough coordination to own one working tree, select one task,
launch one configured agent session, and continue until the tree is complete.

Grove does not know what it launches. One personal file maps each session kind
to one complete command template, and the driver executes the expanded argv
directly. Everything below is what remains once launch policy leaves the binary:
process ownership, a task-tree data model, and three fail-closed transactions.

## Documentation ownership

| Subject | Canonical source |
|---|---|
| Project description and installation | [`README.md`](../README.md) |
| Human workflow and commands | [`USAGE.md`](USAGE.md) |
| Session configuration and launch policy | [`CONFIGURATION.md`](CONFIGURATION.md) |
| Cutting and publishing a release | [`RELEASING.md`](RELEASING.md) |
| Runtime and repository design | this document |
| Grove vocabulary | [`CONTEXT.md`](../CONTEXT.md) |
| Relationships between this repository's bounded contexts | [`CONTEXT-MAP.md`](../CONTEXT-MAP.md) |
| `ordinal-fs-tree` design and vocabulary | [`ordinal-fs-tree/ARCHITECTURE.md`](ordinal-fs-tree/ARCHITECTURE.md) and [`ordinal-fs-tree/CONTEXT.md`](ordinal-fs-tree/CONTEXT.md) |
| Methodology executed by agents | [`content/SKILL.md`](../content/SKILL.md) and its adjacent format guides |
| Skill-plugin operation | [`plugins/README.md`](../plugins/README.md) |
| Scoping notes for work not yet started | `TODO.<subject>.md` at the repository root |

A `TODO.<subject>.md` is a **scoping note with an expiry**: measurements and open
questions for work a future grove will grill, written so the evidence is not
re-gathered, and deleted when the work lands or the question is settled in an
ADR. It is not a plan, not a backlog, and never the canonical description of
anything that exists — those rows are above.
[`TODO.finish_process.md`](../TODO.finish_process.md) is the current one.

The four files directly under `docs/` are the maintained project guides. This is
not a ban on durable artifacts produced by future Grove work: when a real
decision, specification, or research result earns a repository record, the
methodology may create focused files under `docs/adr/`, `docs/specs/`, or
`docs/research/`. A subdirectory such as `docs/ordinal-fs-tree/` holds the
guides of a bounded context that does not yet ship by its own path, and travels
with that context's code when it does.
Those sets describe current state and should be merged or deleted when they no
longer do; VCS holds their history.

The former decision-record slugs remain explicit HTML anchors in this document
(for example, `task-tree-scheme` and `symmetric-vcs-rule`). Source comments and
tests use those stable slugs as compact design references; changing a section
title does not change the anchor.

<a id="skills-monorepo"></a>
## Repository products

The repository contains two independently installed products:

| Product | Source | Delivery |
|---|---|---|
| Grove CLI and methodology | `src/`, `content/`, `build.rs` | Homebrew installs `grove` and `grove-llm`; bare `grove` provisions the embedded methodology. |
| Agent skill plugins | `plugins/linkuistics/`, `plugins/testanyware/` | Claude marketplace, or `plugins/install.sh` for the skills whose `harnesses:` key declares them installable off Claude Code. |

Grove and the skill plugins share a repository because their documented
interfaces evolve together, but they do not install one another.

## Runtime flow

```text
human: grove
        │
        ├─ provision embedded content/ into every installed harness skill dir
        ├─ resolve the nearest jj or Git working-tree root
        ├─ acquire that working tree's driver lease
        └─ foreground loop
             │
             ├─ revalidate the lease
             ├─ re-verify each skill dir's stamp; restore a clobbered one
             ├─ identity-check the grove-llm the session will resolve
             ├─ load and fully validate ~/.config/grove/config.kdl, then
             │    resolve any untracked .grove.kdl delta over it
             ├─ reap orphaned finish quarantine, then recover or perform the one
             │    lifecycle transition (root-init, migration, or nothing)
             ├─ one authoritative pick — or materialize the finish leaf
             ├─ reload configuration and expand the selected kind's template
             ├─ allocate a fresh signal channel and activate the session epoch
             ├─ spawn that argv directly and watch the foreground child
             └─ reap, invalidate the epoch, then read the completion signal
                    ├─ relaunch → next fresh session
                    ├─ done     → stop
                    └─ absent   → stop safely; the next `grove` resumes
```

`grove --help` and `grove --version` stop before this flow: they provision
nothing, discover no repository, and acquire no lease. Provisioning precedes
ownership, so even a refused second driver receives the current methodology.

The driver stays in the foreground and owns its child process. Completion
signals are temporary control messages, not durable workflow state. Each
iteration re-checks the `grove-llm` a session would resolve through `PATH`,
because a mid-loop `brew upgrade` is exactly the skew a start-time check misses
and a split signal protocol looks like every session hanging with nothing
relaunching. The comparison is of [methodology
identity](#the-boundary-is-a-build-not-a-commit), not crate version, and the
subject is the `PATH` binary rather than the driver's own sibling — the driver
never invokes `grove-llm`, so the sibling agrees with it by construction while
the binary the session actually runs would go unchecked. The check **reports and
launches anyway**: it measures the driver's environment, which is the session's
only when the configured command inherits it, so a refusal would stall the loop
on a proxy the driver cannot confirm ([one build owns a
session](adr/one-build-owns-a-session.md)). Nothing else in the iteration is
advisory — configuration, lease, and workspace layout are facts the driver
establishes directly and stops on.

<a id="cli-binary-split"></a>
<a id="command-surfaces"></a>
## Command surfaces

The `grove` binary is for humans and has no subcommands and no lifecycle flags.
Bare `grove` reads the task tree for what to do and the personal configuration
for how to launch it, so there is nothing left for an argument to select;
`--help` and `--version` are the only other accepted arguments.

`grove-llm` is for deterministic operations invoked by the embedded methodology:
`root-init`; `pick`, `brief-chain`, `kind`, and `resolve`; `leaf-add`,
`leaf-insert`, `leaf-decompose`, and `leaf-add-pair`; `leaf-retire` and
`leaf-prune`; `finish-commit`; and `complete`. Every one of them mutates or
resolves a task tree, so every one of them is admitted through the session-epoch
guard. This split keeps a discoverable human API without forcing the agent to
reproduce filesystem mutations from prose.

The surface is **flat**: a verb name is a whole invocable command, with no
nesting. That is what lets `tests/methodology.rs` compare the verbs the corpus
instructs against the verbs the CLI exposes by name, and it is pinned there
rather than merely observed.

There was a twelfth, `methodology`, which served the embed by unit id to a
session following a `defers=` marker. It went with the mandate machinery: the
corpus routes by **path** now, so a session that needs a procedure opens the
file its condition names.

`grove-llm` also answers `--content-hash` with its build's [methodology
identity](#the-boundary-is-a-build-not-a-commit). That is a flag rather than a
verb on purpose: the driver reads it to report on the pair, no session ever
calls it, and the embedded methodology instructs nothing about it — so it stays
out of the agent grammar `tests/methodology.rs` scans. A binary old enough not to
answer it is unidentifiable rather than mismatched, and is reported the same way.

`src/main.rs` and `src/bin/grove-llm.rs` are thin entry points. `src/cli.rs`
owns the human grammar; `src/llm_cli.rs` owns the agent grammar.

## Session configuration

`~/.config/grove/config.kdl` carries user launch policy: a flat map of all
nineteen session kinds to one complete command-template string each, with no
defaults, families, or inheritance. The configuration module loads that one file
into a total kind-to-template map and expands one selected template from a
context of prompt, session name, worktree, and repository root. It hides KDL
handling, aggregate schema diagnostics, POSIX shell-word splitting, substitution
validation, and argv construction; callers cannot ask it for a default, family,
harness, or model. The user-facing grammar and diagnostics are in
[CONFIGURATION.md](CONFIGURATION.md).

At most one second file takes part: an untracked `.grove.kdl` **configuration
delta**, searched at the worktree root and then the main repository root, the
first one found selected outright and the two never merged. It declares any
subset of the kinds and each declared kind's whole template replaces the personal
file's, while the personal file stays mandatorily complete and fully validated.
Resolution is therefore two deep and flat rather than a precedence lattice, and a
kind's launch remains one complete string read whole out of one file — which is
why this leaves [complete session
configuration](adr/complete-session-configuration.md) intact. The module takes
both roots from the driver rather than deriving them, so the search order cannot
disagree with what `${repo}` expands to in the template it selected.

That gives the module its one non-filesystem dependency: because a delta names a
program to execute, a **tracked** candidate is refused rather than trusted to an
ignore rule, so `session_config` asks `repo` one read-only question about one
path — and only when a candidate file exists. An unreadable, unparseable,
invalid, or tracked delta fails the load at both read points, with the same
aggregate diagnostics attributed to the delta's own path and location. See [the
untracked configuration delta](adr/untracked-configuration-delta.md).

Grove executes the expanded argv directly — no shell, no proxy, no router
service, and no harness-specific argument or environment injection. Because a
command string is opaque, Grove cannot identify the program it launches, which
is what removes harness detection, model routing, session-name arguments, Codex
sandbox grants, and launch-target comparison from the binary altogether. Those
choices are visible in the template instead. See [Complete session
configuration](adr/complete-session-configuration.md).

Two environment rules follow from that opacity. Immediately before spawning the
configured child, Grove clears its own loop-control variables and grants only
this launch's `GROVE_SIGNAL_FILE`; everything else the caller had, including Git
repository selectors, is preserved as the configuration owner's policy.
Driver-internal VCS children follow the opposite rule — they scrub both the
loop controls and the repository selectors and anchor Git explicitly to the
leased working tree — so personal launch context cannot redirect a migration or
teardown commit.

## Process ownership

A working tree has at most one live driver. After provisioning, and before
configuration validation or any `.grove/` observation, bare `grove` acquires a
**driver lease**: a nonblocking exclusive advisory lock on a fixed file in a
control directory derived from the closest on-disk VCS marker for that exact
workspace — `<workspace>/.jj/grove/` for native, secondary, and colocated
Jujutsu, or the canonical per-worktree Git directory's `grove/` child, never
Git's common directory. The resolver invokes no repository discovery and ignores
`GIT_DIR` and its relatives, so controls live in the exact workspace's
administration area rather than the tracked working copy or an ambient temporary
directory. Symlink and relative-path aliases contend on one lease; separate
worktrees and workspaces stay independent.

Acquisition creates that control directory and proves the **workspace layout**
can supply what teardown will need: an atomic same-filesystem rename target for
the whole `.grove/` root. `<workspace>/.jj/grove/` and a `.git/` directory keep
resolution inside the working tree, so every jj shape and a plain checkout pass;
a `.git` *file* — a linked worktree or submodule — sends resolution to the main
repository or superproject and is the only family whose devices can differ. Grove
measures rather than classifies, because a symlinked marker escapes the working
tree without changing its kind. A refusal is a resumable no-mutation stop, which
is the point: an unfinishable workspace is named before it holds a task tree
rather than at the finish gate. The finish transaction still repeats the
comparison against its exact rename operands — see [supported workspace
layouts](adr/supported-workspace-layouts.md).

Every acquisition opens, locks, and then compares the locked descriptor's
device/inode against the path's current identity, retrying a bounded number of
times on an open/lock replacement race. The driver holds the root and lease
descriptors until the loop reaches a terminal disposition and revalidates before
every lifecycle transition and launch. Kernel release on return, panic, or
process death is what makes restarting after a crash ordinary continuation. A
second driver fails immediately rather than queueing, because two drivers would
issue two mandates for one task. Every descriptor is close-on-exec, so an opaque
configured command cannot pass ownership to a descendant.

The same control directory holds one **session epoch** file binding the driver's
fresh 128-bit OS-random nonce, the working-tree identity, and the current signal
path. The driver rewrites it under a separately scoped exclusive guard at three
points: inactive right after acquiring the lease, active right before each
spawn, and inactive right after reaping the child and before interpreting its
signal. Each launch draws an independent 128-bit random `signal-*` name in the
same directory; allocation picks an absent name but creates no file, so a failed
spawn leaves nothing to clean up.

An ambient agent-side `grove-llm` operation — one running with
`GROVE_SIGNAL_FILE` set — takes a *shared* epoch guard, verifies the exact
signal path, nonce, and worktree, and probes lease liveness with a separate
nonblocking exclusive attempt on its own descriptor. Holding that shared guard
through the whole operation is what closes the probe's race: a replacement
driver cannot install the next epoch until an already-admitted operation
returns. Each of the four epoch acquisitions — driver, pre-spawn, post-reap, and
ambient — tries without blocking, prints one diagnostic on contention, then
waits a fixed internal 30-second handoff bound; a timeout does no tree access
and no epoch rewrite, so an orphan that outlives its parent makes the driver
stop `blocked` rather than silently park. Manual `grove-llm` commands with no
loop-control context keep their ordinary behavior.

The lock order is fixed: lease, then a scoped exclusive epoch guard, released
before any task-tree operation; an ambient `grove-llm` command takes its shared
epoch guard before the Tree access lock. The driver never waits for an epoch
guard while holding a tree guard. The lease serializes loop lifetimes, the epoch
serializes launch authority, and the Tree access lock serializes individual tree
observations and mutations. These are untracked coordination files whose bytes
mean nothing once their locks release; `.grove/` remains the only durable
workflow state. See [One live driver owns each working
tree](adr/one-live-driver-per-working-tree.md).

<a id="task-tree-scheme"></a>
## Task-tree data model

The task tree is the state:

```text
.grove/
  FORMAT                     # positive format witness: `session-kinds-v1`
  BRIEF.md
  NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md
  NN-<slug>-k<key>/
    BRIEF.md                 # a node's charter
    NN-...                   # children use the same grammar
```

There is **one node species**. A node means work proved bigger than one session,
so it carries a charter; no constructor emits a charterless node, and both
composition shapes are flat siblings. The reader nevertheless tolerates a
missing `BRIEF.md` — a hand-authored lapse, or a node left by the deleted chain
constructor — by skipping that level of the brief chain silently. Tolerance is
not a second species: a node close still checks its `Done when` and promotes
what survives, and a charter that is merely absent is a gap to fill rather than
a signal to skip that rollup.

`NN` is a gapless, per-directory position and may change when inserting work.
`k<key>` is a permanent, globally unique identity and survives moves,
decomposition, and completion; `<slug>-k<key>` is the stable handle. A node is a
directory; a leaf is Markdown. `DONE` and `ABANDONED` are terminal filename
infixes, so picking and rendering need not parse file contents.

`<session-kind>` is one member of the closed set, separated after the optional
outcome infix. The set maintains a non-prefix invariant — no kind label followed
by `-` prefixes another — so a shorter kind plus a slug can never render the same
bytes as a longer kind plus a different slug. Node directory names stay
kind-free, and kind is routing metadata rather than identity.

Every positioned, keyed name is task-shaped, and its `.md` suffix declares which
species it is — present a leaf, absent a node directory. Such a name must parse
completely as that species and name an entry that *is* that species on disk. A
leaf with a missing or unknown kind, a node directory wearing a `DONE` or
`ABANDONED` infix, a directory at a leaf's name, and a file or symlink at a
node's name are all malformed trees that stop reads and mutations. Entries
outside the grammar remain foreign and ignored at either species; `BRIEF.md`,
carrying neither position nor key, stays outside the rule because a charterless
node is legal everywhere. The rule reaches directory names because the loss is
larger there: a skipped leaf costs one task, a skipped node costs its whole live
subtree while picking reports the grove finished.

`tree_read`'s level reader owns the species half for every tree verb, so
selection, resolution, growth, key allocation, and pruning share one answer about
what a sibling is — without which a subtree the reader refuses could stay
invisible to key allocation, lowering the visible maximum key so the next
`leaf-add` re-issues a live one.

`FORMAT` makes "already current" independent of slug text, so a legacy
`01-design-notes-k3.md` cannot masquerade as a kind-bearing current leaf merely
because `design` is a valid kind. It is format metadata inside the tree, not
lifecycle state; root initialization and migration write it last, by atomic
same-directory rename. Task bodies carry no launch metadata at all — only the
`**Reviews:**` and `**Integrates:**` composition relationships below.

`tree_id` parses identities, `tree_read` walks and resolves them, `tree_grow`
creates leaves and composition shapes, `tree_rename` performs VCS-safe moves,
`tree_lifecycle` applies terminal outcomes, `tree_access` owns the lock, and
`tree_format` owns the witness. `tree_migrate` plans the conversion of prior
layouts and `tree_migration_transaction` is its only mutation owner; neither is
another live storage model.

Picking is a stateless depth-first pre-order walk over numeric sibling
positions. It returns the first live leaf and skips terminal entries. The one
eligibility rule beyond terminal filtering is the driver-owned `finish` leaf:
while any non-finish leaf is live the walk skips finish, which becomes eligible
only as the sole remaining work. Finish is therefore **reserved, not blocking**,
and both halves matter. `leaf-insert` sequences work ahead of it, and ordinary
`leaf-add` may also *append* work behind it — the case that bites, since the
finish leaf keeps the earlier position and nothing but the skip rule stops
teardown being proposed while live work sits after it. More than one live finish
leaf is malformed and stops selection rather than letting eligibility depend on
encounter order.
`grove-llm pick` applies the same rule, so its diagnostic answer can never
disagree with the driver. Grove encodes no dependencies, priorities, or
scheduler outside this order.

### Authoritative selection and mandate

The driver performs exactly one authoritative pick per iteration, after any
required lifecycle mutation. One guarded read copies the selected leaf's path,
stable handle, and filename kind, and the read guard is released before the
second configuration load and the spawn — so the mandated session can take an
exclusive tree guard while the driver still owns the loop. That single value
serves readiness, the launch diagnostic line, template selection, and the
mandate, with no second tree read. It is a fact, not a routing forecast, and it
is not recomputed immediately before spawn.

`${prompt}` carries the **guaranteed core** and nothing else: an instruction to
load the provisioned skill and this kind's reference file by path, then the two
facts Grove resolved at runtime — the stable handle as an explicit mandate, and
the [version control](#symmetric-vcs-rule) Grove resolved for the working tree —
then the session-ending text, which is the embed's own signal file for that kind —
`SIGNAL.md`, or `SIGNAL-FINISH.md` for `finish`, whose ending is a choice between
three outcomes — inlined byte-exact. The order is the session's own timeline, and the methodology itself
arrives as the provisioned skill rather than in argv
([the skill delivers the methodology](adr/skill-delivers-the-methodology.md)). No
hidden leaf environment variable accompanies it. At Bootstrap the
session resolves the handle with `grove-llm resolve`, rejects a missing,
ambiguous, terminal, or non-leaf result, reads the glossary, cited decision
records, brief chain, and task, and executes it without calling `grove-llm
pick`. A leaf inserted during the launch window therefore does not preempt a
running session; it becomes the next iteration's work.

<a id="library-refusals"></a>
### How an `ordinal-fs-tree` refusal reaches an operator

**Verbatim, unchanged, and rarely** — because Grove resolves and classifies its
target before it calls the library, so the refusals that reach an operator are
exactly the ones whose words are already true of a Grove tree.

**Half the library's error surface already speaks Grove's words and half cannot.**
`Error::Malformed` and `Error::Reserved` carry `EntryName::Err`, so a *parse*
failure arrives as Grove's own `TaskNameError`; `Error::Refused` carries
`Refusal`, which is not generic over the name type and holds no domain value at
all, so every algebraic refusal speaks the library's vocabulary and no domain can
change it ([`entry-name-is-the-only-seam`](adr/entry-name-is-the-only-seam.md)).
That
record names Grove as the condition that would reopen it — a domain whose
vocabulary *collides* with the library's rather than merely differing — and the
condition was met and did not fire. What follows is why.

#### The rule this rests on

Three clauses, and every verb the migrate stage moves transcribes them. Break
one and the table below is wrong rather than merely incomplete.

1. **Resolve the argument to an entry, then call by key** — against the same
   snapshot the operation plans from, which one guard already guarantees. Grove's
   reference grammar (a path, `[n]`, `n`, `<slug>-k<key>`, a bare slug) is wider
   than a key, and its **ambiguous** outcome has no library counterpart at all, so
   resolution is Grove's and a reference naming nothing is Grove's own refusal —
   which can say that two namespaces were tried, where the library's cannot.
2. **Classify the resolved entry before calling.** Every mutating verb has a
   precondition the library cannot see — an outcome infix, a session kind,
   `finish`-reservation, brief-ness — so the classification is not optional, and
   once it has run, the library's own species refusals sit behind a check Grove
   needed anyway. Where that check duplicates one the library also makes, make it
   the **same predicate read off the snapshot** — a node is an entry whose
   contents are `Some`, never a path that `is_dir` — because a second predicate
   for one condition is clause 3 broken at the level of code rather than of
   prose, and would let Grove refuse where the library would have proceeded.
3. **Never write a second wording for a condition the library states.** Where
   Grove refuses it refuses on its *own* precondition, naming its own verbs;
   where the library refuses, its message is printed unchanged. The drift a
   second wording produces is what `docs/formalism-findings.md` entry 017
   measured.

No read verb can produce a `Refusal` at all: the library's reading surface
answers with `Option`, so `pick`, `brief-chain`, `kind` and `resolve` construct
nothing and keep the diagnostics they have carried since before the library
existed. `docs/ordinal-fs-tree/CLI.md` had its read verbs *construct* a
`Refusal::TargetMissing` for want of a message of their own; Grove has one, and
adopting the library's would be clause 3 broken in the opposite direction.

#### Which verbs reach the algebra at all

Nine of thirteen, and the refusals they reach need a tree at the edge of the
keyspace or the ordinal space — or, for the one verb the migrate stage has yet to
move, one a hand edit or a failed rollback has damaged.

| verb | library operation | `Refusal`s it can reach |
|---|---|---|
| `pick`, `brief-chain`, `kind`, `resolve` | `walk`, `by_key`, `ancestors`, `distinguished_chain` | **none** — the reading surface answers with `Option`, so no refusal exists to raise |
| `leaf-add` | `append` | `KeysExhausted`, `OrdinalsExhausted` — and **not** `TargetNotNode` or `DestinationOccupied`; `growing-k33` corrected both rows |
| `leaf-add-pair` | `append_many` | the same two |
| `leaf-insert` | `insert` | `KeysExhausted`, `OrdinalsExhausted` — not `TargetNotNode`, because the target passed is the resolved entry's **container**, a node by construction; and not `DestinationOccupied`, per the row below |
| `leaf-decompose` | `promote` | `DestinationOccupied`, `KeysExhausted` — the node takes the leaf's own ordinal and the first child takes the first, so no ordinal is allocated past the end |
| `leaf-retire`, `leaf-prune` | `rewrite` | **none**, and the row below says why the `DestinationOccupied` this table first predicted is unreachable |
| `root-init` | `append` into a tree it has just created | **none** — the root is not an entry and the level is empty |
| `finish-commit` | none — it reads the tree, then deletes `.grove/` under Grove's own transaction | **none** |
| `complete` | none — it touches no tree | **none** |

#### Which refusals Grove's verbs can reach

Three of ten, and none from an ordinary argument. A refusal no argument produces
is a case a contract test cannot cover and a reader should not go looking for.

**Three rows have since been corrected by the leaves that transcribed them, and
the count fell from four to three.** `marking-k32` found `DestinationOccupied`
unreachable from the two marking verbs; `growing-k33` found `TargetNotNode` and
`DestinationOccupied` unreachable from the three grow verbs. Each correction is
the check [`refusals-k30` scheduled](#library-refusals) working as intended: the
table's own guarantee is that each migrate leaf writes its rows into a suite and
finds them wrong if they are, and it has fired on every migrate leaf that had a
row to write.

What survives is `KeysExhausted` and `OrdinalsExhausted` from the grow verbs, and
`DestinationOccupied` from `leaf-decompose` — a row no leaf has transcribed yet,
and `promotion-k34`'s to check. The consequential change is that **no algebraic
refusal reaches an operator from an ordinary argument any more**: `TargetNotNode`
was the only one that did, and the collision `refusals-k30` weighed was its
message. The decision that record reached — print verbatim, re-word nothing — is
unchanged and is now cheaper than it looked, because the message that collides is
one no argument produces.

| `Refusal` variant | reachable from Grove's verbs? |
|---|---|
| `TargetMissing` | **no** — clause 1. A reference naming nothing fails in Grove's resolution, before any operation is called. |
| `TargetNotNode` | **no**, and `growing-k33` corrected this row: it predicted *yes* for `leaf-add <a task file> <slug>` while naming its own contradiction in the next clause — *Grove keeps its own check in front of it*. Both are true of the design and only one can be true of an operator. The check is not optional either, which is what settles it: `.grove/BRIEF.md` is an entry carrying **no key**, so it cannot be handed to the library as a target however the refusal were worded, and clause 2 therefore *forces* the classification that puts this refusal permanently behind one. Asserted in `src/task_grow/tests.rs` over every parent argument that is an entry and not a node. |
| `NoOccupantAtOrdinal` | **no**, in none of its three messages — `leaf-insert` names the **entry** whose slot the new leaf takes, and Grove reads the ordinal off that entry in the snapshot the insert plans from, so `at` is occupied by construction. The syllabus CLI reached all three because `<at>` is an ordinal argument there; Grove's argument surface discharges the refusal `insert` spent two leaves getting right. |
| `PromoteNotLeaf` | **no** — `leaf-decompose` refuses a brief, a `DONE` leaf, an `ABANDONED` leaf and a `finish` leaf, none of which the library can see; a node falls out of the same match. |
| `PromotePartsNotNode` | **no** — `leaf-decompose` always composes node parts. |
| `PromoteNoDistinguished` | **no** — Grove's distinguished child is `BRIEF.md`. |
| `RewriteSpeciesChange` | **no** — `leaf-retire` and `leaf-prune` compose leaf parts for an entry they have already matched as a live leaf. Confirmed by `marking-k32`: the classification reads `Parts::Leaf` off the snapshot and composes from its own `kind` and `slug`, so no path through either verb can hand `rewrite` node parts. |
| `DestinationOccupied` | **no from any flipped verb**, and it took two leaves to establish; `leaf-decompose`'s row above is still predicted and still unchecked. **Not from `leaf-retire` or `leaf-prune`** (`marking-k32`): the occupying name must be exactly the name the mark would place, and an outcome infix and a key are both *parts of one name*, so a `DONE` twin beside the live leaf necessarily carries the live leaf's key — which `task_tree::addressable_key` refuses first. **Not from the grow verbs either** (`growing-k33`), though this row predicted *yes on a hand-edited tree: a copied leaf duplicating a key*, and composing that tree is what showed otherwise. An **append** composes its name with `max + 1` over the whole tree, so no entry in the snapshot can already carry it, whatever a hand edit did. A **shift** composes `(ordinal + 1, key, parts)`, and the only entry that could already carry that name is the sibling one ordinal higher — itself a mover, and already vacated, because the renames run highest-first and the plan is folded through the snapshot in that order. That is the second thing highest-first buys, after the intermediate state, and `ops.rs` says as much in passing — *lowest-first is refused only where a hand edit already duplicated a key and its parts at adjacent ordinals*, which is the tree this row was reaching for. Asserted against `operations.qnt`'s `corrupted` instance rendered in Grove's grammar. |
| `ContentForANode` | **no** — discharged by the verb set. A node arises only through `leaf-decompose`, whose node parts carry no bytes and whose first child is a leaf; `leaf-add`, `leaf-add-pair` and `leaf-insert` compose leaf parts and nothing else. |
| `KeysExhausted` / `OrdinalsExhausted` | **yes** — a hand-written `-k4294967295`, or a position of `4294967295`. That is the exact edge: one more is refused by the grammar as [not canonical](adr/task-names-are-canonical.md), so nothing between the two states is representable. |

| non-`Io` `Error` variant | reachable from Grove's verbs? |
|---|---|
| `Malformed` | **yes** — a hand-edited name. Carries `TaskNameError` and therefore already speaks Grove's words, which is the whole reason that variant is generic. |
| `Reserved` | **yes** — `MIGRATING-session-kinds`, `FINISHING-*`, `PREPARING-FINISH-*`. Carries `TaskNameError` likewise. |
| `Failed` | **yes in the wild, from no argument** — the filesystem refuses mid-apply and the run unwinds. **The tree is as it was found**, so a retry is safe. |
| `FailedPartiallyRolledBack` | **yes in the wild, from no argument** — and the one message whose *recovery advice* is stated in the library's words. See below. |
| `NonUtf8Name` | **not on macOS** — APFS refuses such a filename, so the branch cannot be reached from a test on this host. Assert that fact rather than skipping it; `docs/formalism-findings.md` entry 006. |
| `NameIsNotOneComponent` | **no** — a `Slug` admits lowercase ASCII letters, digits and hyphens only, so no name `TaskName` renders can be more or less than one path component. |
| `NoContainingDirectory` | **no** — the tree root is always `<worktree>/.grove`, which always has a containing directory. |

#### What verbatim costs, measured rather than assumed

The collision is real, and composing the offending message at design time is
what sizes it. Were clause 2 dropped, `grove-llm leaf-add 03-impl-extract-k7.md
sweep` would answer:

> the entry with key 7 is a leaf, which holds nothing. Children go in a node —
> promote it first, or name a node.

Six clauses, read against [Grove's glossary](../CONTEXT.md): *the entry with key
7* ✓ — the library's *key* is Grove's **Permanent key**, and `resolve` takes it
bare; *is a leaf* ✓ **extensionally** — every
positioned regular-file entry under `.grove/` is a Grove **Leaf**; *which holds
nothing* ✗ — a Grove Leaf holds the task body, and the library means *holds no
children*; *Children go in a node* ✓ — Grove's **Node directory**; *promote it
first* ✗ — names no Grove verb, the operation being `leaf-decompose`; *or name a
node* ✓.

**Two clauses of six, and one of the two is the recovery advice.** That is the
whole of the collision the seam record predicted, and it is smaller than *the
library's vocabulary is foreign* implies — the nouns coincide inside `.grove/`
and the verbs do not. It also lands on the audience that can least afford it:
`grove-llm`'s operator is the LLM driving a session
([cli-binary-split](#cli-binary-split)), which will try the verb it is told to
try, so a wrong recovery clause is not confusing but executed.

Keeping Grove's own check is therefore cheaper than either alternative. It
re-words nothing — Grove refuses on its own precondition, as it already does for
a `DONE` leaf — and it is a check Grove cannot drop anyway, because `.grove/`'s
`BRIEF.md` carries no key and so cannot be handed to the library as a target at
all.

`FailedPartiallyRolledBack` is the one message left speaking the library's words
where it matters: *a node and a leaf sharing an ordinal and a key, with the node
holding no distinguished child, is an interrupted promotion*. In Grove's words
that is a node directory and a task file sharing a position and a key, with the
directory holding no `BRIEF.md`. It prints verbatim, because it fires on a failed
rollback rather than on an argument and because
[`CONTEXT-MAP.md`](../CONTEXT-MAP.md) carries the six-term translation — a map
between two glossaries cannot drift from the messages it translates, where a
re-wording of each message can.

#### What would reopen this

A Grove verb that takes an **ordinal** or a bare **key** straight through to an
operation, without resolving it first. That breaks clause 1 and makes
`TargetMissing` and `NoOccupantAtOrdinal` reachable — the second in three
distinct messages — so the vocabulary question stops being about one refusal and
this table's count is simply wrong. Nothing in the current verb set does it: all
seventeen of `grove-llm`'s parsed arguments are paths, references, new-name
inputs or flags, and not one is an ordinal. No verb should be added that changes
that without re-deriving this table.

<a id="task-kind-taxonomy"></a>
## Task kinds and composition

The closed set of nineteen session kinds gives each session a discipline and
gives the driver its configuration key.

| Producer | Purpose | Review | Integration |
|---|---|---|---|
| `requirements` (HITL) | Establish what should be built through human dialogue. | `review-requirements` | `integrate-review-requirements` |
| `design` | Establish how; produce current-state specs or decisions. | `review-design` | `integrate-review-design` |
| `planning` | Decompose the design into vertical agent-sized leaves. | `review-planning` | `integrate-review-planning` |
| `prototype` (HITL) | Build a cheap artifact to provoke human reaction, not to ship. | `review-prototype` | `integrate-review-prototype` |
| `impl` | Produce shippable code, docs, or tests. | `review-impl` | `integrate-review-impl` |
| `research-a` | Produce a primary-source survey. | `research-b`, the independent second survey | `combine-research` |

The nineteenth kind, `finish`, is driver-reserved: only the lifecycle creates a
finish leaf, and the grow and terminal verbs refuse it as a kind or operand.

Reviews are fresh-context adversarial reads that produce findings rather than
fixes. Integrations verify each finding, then fix the contract, fix the
artifact, accept a visible trade-off, or reject noise. `requirements` and
`prototype` are human-in-the-loop because human words or reactions are their
essential input; any other kind may still stop and ask.

Two documented composition shapes exist, both as **flat siblings** named off a
shared stem — neither gets a node directory:

- Review chain: `X → review-X → integrate-review-X`
- Research pair: `research-a → research-b → combine-research`

Every step carries that stem as its **whole slug**, so a shape's leaves differ
only by kind and key. The kind field is the canonical statement of a step's role
and the slug names the artifact; a step marker in the slug would restate the kind
beside it, giving a second and unvalidated statement of a fact Grove already
parses and routes on. That is convention rather than grammar in both directions:
nothing generates or checks it for a chain, and a leaf slugged under the older
`<stem>-review` spelling remains a well-formed name that no migration rewrites.
The one consequence is that a bare stem stops naming one leaf: `resolve <stem>`
on a chain is ambiguous and lists each match's kind-bearing path — pick-style, so
empty stdout, the diagnostic on stderr and **exit zero**, since a listing is
information rather than a failure. Every *recommended* reference is unaffected,
because the mandate, the relationship lines, commit messages and grow-verb targets
all name a `<slug>-k<key>` handle, a key or a path, and keys stay unique
tree-wide. The bare slug the grow verbs *also* accept as a target convenience is
the one reference that loses its step, and there ambiguity is a refusal naming the
matching keys rather than a listing.

They are constructed in opposite ways, and the asymmetry is the design. A review
chain is **lazy**: each step is an ordinary `leaf-add` performed as the last act
of the session before it, so a producer cuts `review-<producer>` only when review
is required and a review cuts `integrate-review-<producer>` only when it has
findings worth acting on. That removes the empty triage session, and it lets the
creating session — the one that knows why the step is needed — write the new
leaf's body with the specific case, finding, or datum a constructor could not
have known. A research pair stays **eager**, one all-or-nothing call, because a
`research-b` cut by `research-a`'s session would inherit that session's framing
and corpus and destroy the independence the pair is run for.

Grove does not validate a cross-leaf grammar, so nothing groups the steps, orders
them, or requires that a chain be complete or contiguous. Where a step *should*
land is therefore methodology, not mechanism, and it differs by hop: a `review-*`
step re-derives its citations from the producer's commit — located by the stable
handle its body names — and goes wherever `leaf-add` puts it, while an
`integrate-review-*` step consumes citations its review already froze into prose,
against a working tree that has since moved and can shift them silently. So the
integration is cut where `select` would reach it next: `leaf-insert` at the first
sibling entry after the review whose subtree still holds live work — an *entry*,
because `collect_live_leaf_entries` descends a node directory in place, and
directory-local, because that same pre-order finishes the review's own directory
before any later sibling of an ancestor. Nothing in `src/` enforces or checks
that. `research-a` and
`research-b` share one discipline but are separate configuration keys, which is
how a vendor pair reaches two different commands without any per-leaf metadata;
whether those two commands are materially independent is configuration-owner
policy, because Grove cannot compare opaque strings.

Once a session has run Bootstrap and adopted its prompt mandate, a plain
producer may spend one in-session fresh-context reviewer across the whole leaf.
A second review need is the signal to `leaf-add` a `review-<producer>` leaf, with
the specific doubt written into its body. Producers that already have a review
leaf beside them, `review-*`, and research-pair leaves spend none; an
`integrate-review-*` leaf may spend one narrow reviewer and externalises
substantial redesign as a new producer review chain beside the leaf it is
integrating.
Sessions outside that procedural predicate retain standalone doubt behavior. See
[Grove owns escalated review](adr/grove-owns-escalated-review.md) and
[doubt-grove-review-mechanics](specs/doubt-grove-review-mechanics.md).

A chain's steps declare their relationships in their bodies: the review carries
`**Reviews:** <producer-handle>` and the integration carries `**Integrates:**
<review-handle>`. Those lines are **written by hand by the session authoring the
body and parsed by nothing** — a documented convention (`content/TASK-FORMAT.md`)
for the human and for the session that picks the step up, which is constraint 3:
task files are freeform markdown and nothing validates them. Names and positions
likewise remain presentation and walk order, never relationship grammar, and the
driver routes a scheduled review solely by its filename kind.

### Tree access lock

Every steady-state task-tree reader holds a shared **Tree access lock** on an
open descriptor for the *working-tree root*; every mutator holds it exclusively
through validation, rollback, or success output. One exception is deliberate and
recorded: a **bulk** mark holds one guard per entry it marks, because a library
mutation consumes its guard — see
[`bulk-marks-are-not-atomic`](adr/bulk-marks-are-not-atomic.md) and *One guard is
one mutation* below. The working-tree root is used
rather than `.grove/` because it is the one thing that exists before root
initialization and survives finish deletion, so a single seam covers creation,
ordinary mutation, and teardown. A contended caller prints one waiting
diagnostic and then waits.

The lock serializes live processes and adds no crash atomicity. Two operations
need more than that and carry their own in-tree witness: the finish teardown
(`FINISHING-*`, below) and the one-time session-kind migration
(`MIGRATING-session-kinds`). Every other task-tree command refuses while either
witness exists and names its recovery. The contract in both cases is
process-interruption consistency, not power-loss durability. See [Task-tree
transactions fail closed](adr/task-tree-transactions-fail-closed.md).

Composite grow verbs need neither, and the promise they make is correspondingly
narrower. `leaf-add-pair` is all-or-nothing **on a reported error** within one
exclusive lock, and since `growing-k33` that is the library's doing rather than
Grove's: `append_many` plans the whole run from one snapshot — so the ordinals
are contiguous and the keys consecutive by construction — checks the plan against
that snapshot before a byte is written, and unwinds its own effects when the
filesystem refuses part way. Grove's own reconstruction of the same guarantee —
an up-front destination sweep, an `O_EXCL` claim per leaf, a per-run rollback
list — went with the verb, and what is left of it serves the lifecycle verbs
that still allocate under Grove's own guard.

That guarantee covers the error return path and nothing else. **Process death
mid-run is not recovered**: the interpreter unwinds only when control returns
through the `Err` branch, so a `SIGKILL` after the first pair leaf lands leaves a
partial shape a reader cannot distinguish from a deliberately hand-cut one.
Finish teardown and the session-kind migration remain the only operations that
promise process-interruption recovery, which is why they alone carry a witness.
The residue is a hand-editable file in a directory tree, and recovering it is
deleting it.

#### Two locks, one at a time, while the flip is in flight

`reading-k31` moved `pick`, `select`, `brief-chain`, `kind` and `resolve` onto
`ordinal-fs-tree`'s own guard (`src/task_tree.rs`). The library takes the same
lock on the same directory for the same reason — the containing directory
outlives the root — but it takes it on **its own** descriptor, and `flock` is
attached to an open file description rather than to a process. So the two guards
do not share a lock, and a verb holding `tree_access::write` that called into the
library's reader would block on itself forever. The rule that follows is per
verb, not per module: a verb uses one guard or the other, never both, which is
why the migrate stage moves whole verb groups at a time and why `tree_read`'s
lock-neutral helpers stay alive until their last exclusive-guard caller has gone.

Two consumer-side obligations came out of that move, and every later flip leaf
inherits both.

**The waiting diagnostic is bought outside the library.** Locking is invisible
in the library's interface by design — no try-variant, no timeout, `read` and
`write` simply block — so nothing in it can say *someone else is holding this*.
Grove has always said so, and losing it in a refactor that promises to change no
behaviour would be a real regression, so `task_tree::announce_contention` probes
the same directory in the same mode non-blockingly, prints the one diagnostic,
releases, and lets the library block. It is a diagnostic and never a decision:
between the probe's release and the library's acquisition a contender can
arrive, and the cost of that window is a missing message and nothing else.

**Refusal precedence is grove's; the halt is the library's.** The library halts
the whole tree on a name grove's grammar refuses, wherever it sits — that is the
decision, and it is taken under the lock. But a legacy tree's leaves are
task-shaped names carrying no session kind, so they are `Malformed`, and an
operator holding one needs to be told to migrate rather than to fix a filename.
So `task_tree::diagnose` re-states a *failed* read in the order grove owes its
operator — root, pending transaction, format witness, then the library's own
message — and chooses only the wording. The pending-transaction sentence itself
is the domain's: `tree_access::refuse_pending_*` raises `task_name`'s own
`TaskNameError`, which is the identical value the library carries when it halts
on a `Verdict::Reserved` mid-tree, so the pre-check and the halt cannot drift
into two wordings of one condition.

#### One guard is one mutation, and a bulk mark is many

`marking-k32` moved `leaf-retire` and `leaf-prune` onto the library's `rewrite`,
which is the first mutation Grove performs through it. A mutating method
**consumes** its `WriteGuard`, so `leaf-prune` on a node — which marks every live
leaf in a subtree — is *N* rewrites under *N* guards where it was one critical
section returning one `PruneResult`. Grove accepts that
([`bulk-marks-are-not-atomic`](adr/bulk-marks-are-not-atomic.md)) rather than
asking a checked library for a batched rewrite, and two properties are what make
it affordable:

- **Validation still precedes every rename.** The subtree is planned and checked
  against the *first* guard's snapshot, so a leaf that cannot be marked fails the
  whole call with nothing renamed — the property the suite has always held.
- **The verb is re-runnable.** An already-`ABANDONED` leaf is skipped silently
  and a `DONE` one is reported and left alone, so re-running `leaf-prune` on the
  node is the repair for a run that stopped part way, and is what an operator
  does.

The window between guards is the real cost, and it is the one thing that changed:
a concurrent writer or a filesystem fault can now stop a bulk mark part way.
`pruning_a_node_takes_one_guard_per_mark` asserts the count, so a later change
moves a number rather than quietly contradicting this paragraph.

**A path argument is only as good as the key it resolves to.** Clause 1 of
[*How an `ordinal-fs-tree` refusal reaches an operator*](#library-refusals) says
resolve the argument to an entry and call **by key**, and that is sound only
while keys are unique tree-wide. The library states uniqueness as the domain's
obligation and cannot enforce it; a hand edit or a failed rollback can put two
entries under one key, and `by_key` then answers with whichever the walk reaches
first — an order neither model establishes. So `task_tree::addressable_key`
refuses a key that names more than one entry, before any operation is called.
Without it, `leaf-retire` aimed at one twin rewrote the other onto its own name,
changed nothing, and reported success. Every flipped verb goes through it, and
every verb the migrate stage has yet to move should: the hazard belongs to
*resolve a path, then call by key*, which is the shape of all of them.

#### A verb that reports on the tree it changed needs a second guard

`growing-k33` moved `leaf-add`, `leaf-add-pair` and `leaf-insert` onto `append`,
`append_many` and `insert`, and one of them has an epilogue: `leaf-insert` lints
stray position-prefixed cross-references left stale by the renumber it just made.
The lint reads the tree the **shift left** — a shifted node took its whole
subtree's paths with it — and the mutation consumed the guard that could have
shown it, so the verb reopens one. That is a second observation, deliberately,
and the property it preserves is the one that mattered: the output is written
while the tree is held, so a hit naming a path is a path nothing has renamed
underneath it. The reopen takes no second waiting diagnostic, for the same reason
a bulk mark's later guards do not.

The lint also **scans the snapshot** rather than the directory, so what it reads
is every leaf and every charter — the same set every other verb calls the tree —
and a foreign `.md` a hand edit dropped into `.grove/` is no longer scanned.
Grove writes no such file, and the alternative is a second, wider notion of
*what is in the tree* than the reader has.

#### The library allocates the key; the consumer's content embeds it

Grove's leaf body opens with the position-free handle `# <slug>-k<key>`, and
`NewEntry` takes its bytes **before** the library composes the name that carries
the key. A content-carrying domain therefore cannot render its content from the
answer, and has to predict the allocation: `task_tree::next_key` is `max + 1`
over the same snapshot the operation plans from, which is the library's own rule
mirrored on the consumer's side.

The alternative was to create with `NewEntry::empty` and write the body
afterwards, from the key the report carries. It was rejected because the guard is
consumed by the mutation, so that content write lands **outside** it — and
because it would hand `append_many` a run whose three files land atomically and
whose three bodies do not, which is the all-or-nothing property the composite
verb exists for. Predicting keeps the content atomic with the creation, and pays
for it with one check.

It is a prediction and it is checked. Every grow verb compares it against the key
the library reports and refuses to claim success on a disagreement, because the
silent failure is a leaf whose first line contradicts its own filename —
permanently, and invisibly. The prediction reads the same snapshot under the same
guard, so it can only be wrong if the library's allocation rule changes, which is
exactly what the check exists to catch. An exhausted keyspace predicts nothing
and hands the library no bytes: `Refusal::KeysExhausted` is the library's to
state, a refusal writes nothing, and the unrenderable content is never reached.

<a id="self-driving-loop"></a>
<a id="do-is-sole-lifecycle-verb"></a>
<a id="fresh-grove-start-contract"></a>
## Lifecycle and resumption

Bare `grove` is the sole start/continue/finish entry. Each iteration performs at
most one lifecycle transition, and full configuration validation precedes every
one of them, so a missing or malformed `config.kdl` — or an invalid or tracked
`.grove.kdl` delta — leaves the working tree byte-identical:

| Observed state | Transition |
|---|---|
| No `.grove/` | Create the root brief, `01-requirements-plan-k1.md`, and the format witness. |
| Legacy layout or missing witness | Migrate in one focused commit. |
| Live leaves | None. |
| No live leaf | Append, or reuse, the driver-owned finish leaf. |

A fresh grove creates a first *leaf*, not just a brief, because `pick` skips
briefs: a brief-only tree would report "no live leaves" and be indistinguishable
from a finished one. Creation is working-tree only; the first session's focused
commit folds in the scaffold. A partial scaffold is recognized as an exact
subset and completed before any missing-witness classification.

Task-root absence is the complete fresh-tree discriminator, and that inference
is only sound because the finish transaction below never exposes it before its
deletion commit is proven. Grove consults no VCS history, abandoned signal
channel, or unlocked lease bytes to decide that a missing tree used to be a
finished one: a teardown commit proves that Grove deleted an earlier tree but
not whether the present invocation means "recover that" or "start another".

The loop launches one foreground session at a time and watches it: poll the
child alongside the completion-signal file, and once the file appears apply
grace → SIGTERM → kill-grace → SIGKILL. That kill is the driver's job because it
is the session's parent, outside whatever sandbox the session runs under; an
in-agent self-kill is silently denied by sandboxes such as Codex's Seatbelt. The
session commits its artifact and terminal task-tree mutation before signalling
`relaunch` or `done`. If it exits without a signal the driver stops instead of
guessing, reporting the child's exit status and elapsed time — and does not
infer `done` even if that child successfully committed teardown. The filesystem
and VCS already say what completed, and a later `grove` continues from there.

### Legacy migration

Migration is automatic inside bare `grove`; there is no human-facing migrate
command. **One** legacy shape is still converted: a current-layout tree whose
leaves lack filename kinds. One planning pass maps each legacy
body's `**Kind:**` to a filename kind (absent defaults to `impl`; `work` maps to
`impl`; the two children of an unambiguous legacy vendor pair map to `research-a`
and `research-b`, and a standalone legacy `research` maps to `research-a`,
because the kind names one configured research discipline rather than structural
membership in a pair). It strips every `**Kind:**`, `**Harness:**`, and
`**Producer launch:**` line while preserving all other bytes, including the
composition relationships. An ambiguous pair or unknown marker stops migration
with exact paths rather than guessing a target.

**Two layouts are classified and refused**: the original `NNN-slug/` + `done/`
tree, and the v1 flat dotted-decimal tree. Each gets the same exact-paths
diagnostic and no mutation. Classification is the load-bearing part and is why
those shapes are still recognised at all: a tree Grove could not classify would
read as having no task entries, and migration would then install the format
witness over it, after which every entry is foreign and picking reports a
finished grove. Recognition costs a private name matcher per layout; the
alternative costs a workstream.

Because both withdrawn layouts were the ones that **relocated** entries — the
`done/` mirror folded in, flat files becoming node directories — what remains
never creates or removes a directory. Every planned move keeps its parent, which
is the assumption the migration transaction's remaining directory handling rests
on.

Directory and kind migration are planned together, so no successful invocation
exposes an intermediate layout as current. Landing runs beneath a reserved
`MIGRATING-session-kinds/` witness holding both the untouched originals and the
staged destination; recovery infers progress from each entry's location and
never reallocates keys. The witness alone makes every other reader and mutator
refuse. The format witness lands last, inside the same focused commit, whose
message identifies the grove and the migration rather than a work-item handle —
migration precedes any task session. In plain Git that commit runs with the same
empty hooks path as the finish commit: rollback restores `.grove/` from an index
image, which cannot undo what a rejecting `pre-commit` reformatter did to
unrelated files.

<a id="pruning"></a>
<a id="confirmation-boundary"></a>
<a id="in-session-finish-cycle"></a>
## Human authority and completion

Grove guides rather than gates. Any session may ask for clarification, but
the CLI has two explicit authority boundaries:

- Abandoning a planned leaf or subtree is human judgment and requires explicit
  confirmation before the agent marks it `ABANDONED`.
- Deleting the completed `.grove/` tree is the one routine finish confirmation.

Finishing happens inside a real, resumable session. When the tree has no live
leaf the driver appends `NN-finish-finish-k<key>.md` at the root and launches the
`finish` target; declining or exiting writes no signal and leaves that same leaf
for a later `grove`. On confirmation the session promotes durable information,
runs `grove-llm finish-commit <finish-handle>`, and signals `done` last. No
commit is made *for* the finish leaf and it is never retired — its addition and
deletion cancel in the focused finish commit. `finish-commit` cannot attest that
a human spoke through an opaque command; it is the deterministic last-moment
tree and VCS guard, not a substitute for that HITL contract. Grove deliberately
does not merge branches/bookmarks or remove working trees.

### Finish transaction

Teardown is not a delete followed by a commit. It is one fail-closed
transaction over a reserved in-tree witness, because the interval
between removing `.grove/` and recording that removal is exactly the shape a
later invocation would read as a fresh grove.

`finish-commit` revalidates the live finish leaf and the absence of ordinary
work, then validates the repository without mutating it: a non-empty tracked
deletion fingerprint, an untracked witness prefix, a same-device
workspace-control quarantine target, and `.grove/` itself opened as a no-follow
directory whose device/inode still matches the `.grove` entry in the locked
working-tree root — so a symlinked or swapped task root is refused rather than
followed. Every later step reuses that descriptor and rechecks the same
identity, which is what makes a mid-transaction swap a refusal instead of a
mutation applied somewhere else. The transaction then evacuates every ordinary
root entry beneath a manifest-backed `FINISHING-<finish-handle>/` witness, which
it builds under a `PREPARING-FINISH-` name and publishes with one atomic rename
so an interrupted build is discardable rather than interpretable. The manifest
records the stable handle,
the session epoch's opaque finish-attempt identity, the repository-start anchor,
the expected tracked deletion fingerprint, and each entry's type and canonical
no-follow recursive digest. Git and jj commit only those deletions at their
original paths, excluding the witness; the task root stays visibly present and
unwalkable throughout.

One deep repository seam hides the Git, native-jj, and colocated-jj mechanics
and returns one of three dispositions, classified from the recorded anchor and
the exact immediate result rather than from command exit status:

- **Committed** — the exact handle-and-attempt-named, `.grove/`-scoped commit is
  proven. Recovery never restores the tree; it finishes index activation, then
  atomically renames the whole root to a workspace-control quarantine before
  descriptor-rooted no-follow disposal. The quarantine is cleanup garbage, never
  a finish receipt, and a later lease-owning driver reaps only entries carrying
  Grove's own cleanup manifest.
- **Not committed** — that commit is absent *and* the recorded starting topology
  still holds, so the tree is restored and the witness removed, leaving the same
  finish leaf selectable for retry.
- **Recovery pending** — neither can be proven. The state stays blocked and
  operator-recoverable: the diagnostic names the artifact holding the
  transaction, the recorded and observed topology, and the two restorable exits
  (restore the recorded start to roll back, or make the exact teardown result
  immediate to finish forward). Grove never rewrites history to clear it.

The disposition is revalidated immediately before and after the filesystem
handoff, so no caller acts on a stale one. A retry that has lost the helper's
result does not trust task-root absence either: with `.grove/` gone it verifies
the immediate VCS result against the same handle and attempt identity, which
binds the proof to the still-active session epoch. Plain Git runs this internal
commit — and the migration commit below — with an empty hooks path, because an
arbitrary hook could mutate the unrelated working-tree bytes those transactions
promise to preserve. See
[Task-tree transactions fail closed](adr/task-tree-transactions-fail-closed.md).

<a id="user-owned-worktrees"></a>
<a id="symmetric-vcs-rule"></a>
<a id="version-control-seam"></a>
## Version-control seam

Grove walks upward from the current directory and lets the closest repository
marker decide. `.jj/` wins over a colocated `.git`; otherwise `.git` selects
Git. Working copies are committed with the tool the marker names — Jujutsu's
operation log is preserved, and a colocated repository's Git index is never
written behind it.

**Moves are not commits, and no longer branch on the lane.** Every entry a
flipped verb moves is renamed by `ordinal-fs-tree`, which does `rename(2)`,
detects no repository and requires no tool on `PATH`. So a tracked entry marked
`DONE` leaves Git's index holding the old path, and `git status` shows an
unstaged deletion beside an untracked file where `git mv` once showed a staged
rename. Both lanes still commit byte-identical trees — Git infers renames at
diff time by content similarity — *provided the commit stages the tree*, which
is why `content/references/commit.md` says so and `tests/leaf_ops.rs` asserts
the three outcomes rather than describing them. See
[`grove-does-not-stage-its-own-renames`](adr/grove-does-not-stage-its-own-renames.md).
`src/tree_rename.rs`'s trackedness dispatch survives only for the verbs the
migrate stage has not reached, and the contract stage deletes it.

Grove resolves that marker before a session exists and **states** the result in
`${prompt}`, which is why sessions do not probe: every launch is told whether its
working tree is jj-enabled or plain Git and which root Grove resolved for it.
*Not to re-derive the answer* is the skill's to say, not the prompt's — the core
carries a launch-varying **value**, and every normative consequence of a value
stays in `content/`. The driver already owns this fact and every
tree-mutation verb already branches on it; only the session was working it out
again, and working it out badly. A harness banner computed from `.git` alone
reads a native Jujutsu workspace as no repository at all
([claude-code#41435](https://github.com/anthropics/claude-code/issues/41435)),
and detection carried as skill instructions is skippable, so a session that never
loaded them commits with Git in a Jujutsu tree and bypasses the operation log.
The line carries identity and root only. Which commands each lane uses stays in
the embedded methodology's Commit step, so a rebuild moves one source of truth
rather than two.

Migration and finish commits are path- or fileset-scoped so unrelated user work
survives: Git uses `--only` with an explicit `.grove` pathspec excluding the live
witness, and Jujutsu commits a `.grove/` fileset with the same exclusion,
leaving unrelated working-copy changes in the successor commit. A colocated
workspace backs up the user's Git index before jj's preflight snapshot and
activates its `.grove/`-free success image only after the exact jj result is
proven.

The user owns topology. Grove reads no branch or bookmark, creates no working
tree, and performs no integration or teardown. The working-tree basename is
the grove name, and `<repo-basename>: <grove-name> grove` is the session name a
template may request.

<a id="self-extension-core-and-methodology"></a>
## Embedded methodology

**Provisioning is the delivery path.** [The skill delivers the
methodology](adr/skill-delivers-the-methodology.md) settles that the sweep, the
stamps, the shared directory and the harness registry all stay, that `${prompt}`
carries only a short guaranteed core pointing at what they wrote, and the
too-late test that decides what may join that core. **The mandate machinery is
gone** — the composer, the unit
markers and the file directives, the two readers over them, the build gate, and
`grove-llm methodology`. `content/` is plain markdown a harness reads as a skill,
with no grammar over it and no grain finer than a file. The **build boundary** at
the end of this section is unchanged by any of that.

`build.rs` embeds `content/` into **both** binaries — `grove` to extract it, and
`grove-llm` to hash it for the identity its foreign-skill-directory warning
needs. On every bare `grove`, `provision` sweeps that content into each installed
harness's personal skill directory — a row of the registry is a place to write
files, never a program to run, and an absent home root is skipped rather than
created. A content hash makes this idempotent while still updating the skill when
the binary changes.

That hash is the build's **methodology identity**, and it is the identity
because the crate version does not move between a released binary and an edited
checkout at the same version. It covers the embedded **file payload** — every
embedded file's path and bytes — and deliberately not the embedded directory
structure, so an empty directory is not part of a build's identity; hashing
typed directory entries would make a traversal reproduce `include_dir`'s
directory semantics as well as its file selection. Both binaries compute it from
the linked embed through one implementation (`methodology::identity`). It used
to be a compile-time constant the build script emitted, precisely so that
*naming* the identity did not link `content/`; once the agent-facing binary
linked it anyway, that reason ended, and the build-script traversal, the constant
and the equality test that kept two traversals in step went with it. "Both
binaries carry it" is a claim about linked artifacts rather than about source, so
it is asserted by scanning binaries: an integration test scans the pair `cargo
test` built, and the release path scans each staged pair before archiving it,
which is where the cross-compiled `--release` targets a local test never sees are
covered.

`build.rs` walks `content/` for **one** reason now: to emit the per-file change
tracking `include_dir!` does not register with Cargo. That walk is load-bearing
on its own — without it, editing a content file leaves a stale embed baked into
the binary and nothing complains — so `tests/methodology.rs` compares the linked
embed against the directory to catch it going wrong. That comparison is on
**contents**, not on paths: the failure it exists to catch is a missed *edit*,
which moves no filename, so comparing path sets would report success on precisely
that case.

**What the deleted gate bought, and what pays for it now.** The gate refused to
embed a corpus whose unit markers did not fully partition every file, whose ids
were not unique, or whose `defers=` chains did not resolve and terminate. It
bought a contributor a build error, with a file and an offset in hand, in place
of a stranger's stalled loop. What replaces it is narrower and is the right
grain for a corpus of prose: `tests/methodology.rs` asserts that `SKILL.md`'s
routing table names a reference file for every kind and that each one exists,
and that the body stays inside its progressive-disclosure budget. A contributor
sees those at `cargo test` rather than at `cargo build`, which is the trade — a
grammar can be gated at compile time, and whether a page of conditions is right
never could be.

**The one enforceable half of the build boundary survives unchanged**, and it is
the check that matters most now: the embedded methodology instructs no
`grove-llm` verb the embedded CLI lacks. It reads the corpus as **markdown**, file
by file, rather than unit by unit — a unit boundary would have cut a wrapped
invocation in half, so the file was always the safer corpus — and it is
load-bearing precisely because the skill is once again the only thing teaching a
session which verbs exist. `tests/methodology.rs` also pins the flat verb
surface that makes the comparison mean what it claims.

Because a configured command is opaque, Grove cannot infer which harness a
session eventually reaches and does not try: every known installed root is
refreshed, so whichever one the command lands in already carries the current
methodology. **A session depends on reaching it**, which is what the core's
wording and the absent-destination report exist for: nothing else teaches a
session the loop, the kinds, or which verbs exist. `content/SIGNAL.md` and
`content/SIGNAL-FINISH.md` are the files that travel both channels — provisioned
like every other, and one of them inlined into `${prompt}` byte-exact as its last
part, so the instruction a session performs after everything else is the last
thing it reads. One source, two deliveries, and no build boundary between them.

The binary refuses to overwrite an unstamped foreign directory and replaces an
old symlink as a link rather than following it. `content/` is the canonical
source; repository-local or hand-edited copies are not supported.

### The seven constraints, argued

[`content/SKILL.md`](../content/SKILL.md) carries the numbered spine itself, and
carries it as the corpus's canonical statement because six other corpus files
cite the constraints **by number** while only `SKILL.md` is on every kind's
static path. What belongs here is the argument for each — why a rule the
methodology treats as non-negotiable earns that status. The subject throughout is
how Grove drives long work *without* becoming brittle, constraining machinery.

1. **Artifacts, not state.** A phase file, a session log or a status file is a
   second source of truth about where the work is, maintained by hand across
   fresh-context sessions that cannot see each other. The directory tree under
   `.grove/` is derivable by eye and by `find`, and the VCS already holds the
   history a status file would be reinventing.
2. **Read, don't run.** A session that must execute something before it can begin
   is a session that can fail to begin. Bootstrap is reading markdown; the one
   command a session runs, `grove-llm resolve <handle>`, is a lookup it could do
   by eye, because the handle is in the filename.
3. **Suggested shape, not enforced schema.** Task files and briefs are freeform
   markdown and the format files are guides. A schema over prose buys validation
   of the half that never fails and forbids the improvisation that makes a leaf
   body useful.
4. **Lazy and optional.** Every artifact — brief, ADR, spec, glossary entry — is
   created only when it earns its place, never because a step demands it. Lazy
   means *just-in-time, not few*: a tree that keeps sprouting small, concrete
   leaves is healthy, and rationing leaves to keep it tidy is the failure this
   constraint names.
5. **Grove guides, it does not gate.** Grove never refuses to proceed. A task may
   be done by hand, reordered or skipped, because a methodology that can block a
   human is one they route around entirely.
6. **Walk-away-able.** Delete the skill and `.grove/` is still a legible folder
   of notes; every durable output is standard, team-readable markdown. This is
   what makes adopting Grove reversible, and it is why the ephemeral task tree is
   the only grove-specific artifact.
7. **One page of rules.** If the loop does not fit on a page it is too complex,
   and the cut is to the rules rather than to the page. `SKILL.md`'s word ceiling
   below is this constraint made recomputable.

### Why the glossary is the forcing function

The acute failure mode of multi-session work is terminology drift: a later
session, with no memory of an earlier one, reinvents its predecessor's term under
a new name, or reuses the same words with a shifted meaning. Neither is visible
in a diff, and both compound.

`CONTEXT.md` — read every session, and appended *inline* whenever a term is
resolved — is the one forcing function against that. Inline rather than batched
is the whole of it: a term resolved and not written down is a term the next
session re-resolves differently, and the batching interval is exactly the window
in which that happens. The normative rule is `content/CONTEXT-FORMAT.md`'s; what
this section records is why the corpus spends a static-path condition on it.

<a id="corpus-shape"></a>
### The corpus's shape, and what is measured over it

`SKILL.md` states **conditions** and routes; `references/` states the
**procedures**. That split is the whole of what makes the skill progressively
disclosed — a session reads one page of conditions and opens the single row its
kind names. It is also why the opening screen *routes* rather than introduces: an
opening that summarises the workflow becomes a shortcut a session takes instead
of reading the body, and a routing table gives every kind a row, so a session
that arrived by description match rather than by a Grove mandate still lands in
its own reference file. Ten rows serve nineteen kinds, because each family is
already one unit.

**Which file states a given rule** is decided by
[`corpus-rule-ownership`](specs/corpus-rule-ownership.md), under
[every normative rule has one owner](adr/corpus-rules-have-one-owner.md): a rule
is filed by **when a session meets it** — the pair *which kinds must obey it* and
*at which moments*, resolved by an ordered first-match rule over a set of
occasions. Under [a restatement declares its
class](adr/restatement-declares-its-class.md) a `SKILL.md` restatement declares one
of three classes (`own`, a ≤25-word `trigger`, or
`none`). That spec also carries the inventory of every rule with its owner, class,
load predicate and test, and it is where the condition/procedure split above stops
being a description of the corpus and becomes the rule that governs edits to it.
One consequence belongs here rather than there: because only `content/` is
provisioned, a rule moved into `docs/` is unreachable to every session outside
this repository, so *normative material stays embedded* is the placement
function's own first case read backwards rather than a separate boundary. A
second: exactly three files are ever on a static path — `SKILL.md`,
`reference_file(kind)`, and the **signal file** the guaranteed core inlines
(`content/SIGNAL.md` for eighteen kinds, `content/SIGNAL-FINISH.md` for `finish`)
— so a rule owned by any other file states the file whose sentence triggers it,
and those references form a chain that must terminate at a static path.

Five numeric measures stand over that shape. All five are **budgets fitted to a
measurement** rather than alarms set well clear of one — `SKILL.md`'s 900 against
a measured 796 is 13% of headroom, the same order as the loaded paths' 10%, so
calling one an alarm and the other a budget would be a distinction the numbers do
not support:

| Measure | Limit | Held by |
|---|---|---|
| `SKILL.md`'s body — the frontmatter is a routing header a harness reads, so counting it would let a description rewrite eat the ceiling | 900 words, and exactly 26 trigger sentences | `tests/methodology.rs` |
| `SKILL.md`'s `## The loop` section — the unit constraint 7 actually names | 275 words, and trigger bullets only | `tests/methodology.rs` |
| Each kind's composed `${prompt}` | 4 KiB | `tests/prompt.rs` |
| Each kind's **static** loaded path — the guaranteed core, `SKILL.md`, and `reference_file(kind)` | per kind, the recorded measurement + 10%, held within +25% of the current one | `tests/loaded_path_budgets.rs` |
| Each kind's **reachable** loaded path — the static path plus the transitive closure of the pointer graph | per kind, the recorded measurement + 10%, held within +25% of the current one | `tests/loaded_path_budgets.rs` |

Each loaded-path row records **the measurement it was fitted to**, beside the
ceiling and in the same table, so the +10% is a comparison a test makes rather
than a convention a comment describes. Without it the only checked interval was
`measurement ≤ ceiling ≤ measurement + 25%`, which admits a ceiling sitting
exactly on today's measurement — the zero-width fit the band exists to prevent —
and a ceiling raised straight to the far edge without ever being fitted.

`SKILL.md`'s word ceiling is constraint 7 — *one page of rules* — made
recomputable: "a page" is otherwise unmeasurable, and a limit no reader can
reproduce is an assertion with no verification boundary. It replaced a 500-line
ceiling on the same body, and a 100-line alarm on the loop section beside it.

**Neither was dominated, and it would be convenient but wrong to say so.** A word
ceiling puts no upper bound on lines at all — blank lines cost nothing and short
lines cost one word — so a 900-word body could in principle run to 900 lines and
fail both line measures while passing the word one. What is true is narrower and
is a claim about *density*: the body measures 7.1 words per line and the loop
section 6.9, and reaching the line limits under a 900-word ceiling would need
prose about 3.9× and 2.8× sparser respectively. Both are density arguments, and
they differ in degree rather than in kind.

The ground for deleting them is therefore not domination. It is that a **line**
is not a unit anyone reads: a line budget is discharged by rewrapping, which
changes nothing a session pays. That argument retires the *unit*, and it is the
whole of what was wrong with the loop alarm.

**It does not retire the scope, and treating it as though it did was an
overreach.** Constraint 7 is specifically that *the loop* fit a page. A
loaded-path budget sees the whole body as one number and is indifferent to where
inside it the words sit, so prose moved out of *Artifacts* into *The loop* leaves
the body ceiling, every static path and every reachable path exactly as they
were while the section constraint 7 names grows without limit. The section is
therefore measured again, in words rather than lines, and with a structural claim
beside the number — the section is `- When …` items and nothing else — because a
word ceiling cannot distinguish a condition arriving from a paragraph of summary
arriving, and it is the summary that constraint 7 forbids.

The routing check stayed, and on different grounds again: it is not a shape
measure at all — it asserts ten kind→file pairs resolve, which is a reachability
claim about the first screen, and the only thing standing over the table a
session that arrived without a mandate actually reads.

**The two loaded-path budgets are the shape measure this corpus is actually held
to**, and they are the reason a per-file line count was not worth keeping: a file
is not what a session reads. A kind reads the core, one page of conditions and
one reference file, and the budget measures exactly that, **through
`prompt::compose` and `prompt::reference_file`** rather than through a second
notion of composition that would drift from the runtime and then lie. They are
asserted from both sides: the corpus must stay under the ceiling, and the ceiling
must stay within the stated headroom of the corpus, so a limit nothing approaches
fails as loudly as a path that outgrew one.

The budgets are in **words**, not tokens. Tokens are what a session pays and are
model-specific; a reproducible token count needs a vendored tokenizer and
vocabulary, and a budget that needs a download is a budget that stops running.
Words track tokens monotonically across prose in one voice, which is what a
*growth* alarm needs — and the honest limit is that a word count cannot price a
register change, so the reading is always "this path grew", never "this path
costs N tokens".

#### What the loaded paths measure, before and after

The corpus rewrite's acceptance, recorded as the comparison rather than as a
claim. *Before* is the start of the workstream that produced this section
(`b6ecdbd0`): `content/` totalled 23,532 words and `SKILL.md` 3,152, its body
3,081. *After*, `content/` totals 14,741 words and `SKILL.md`'s body 796.

The before figures are **recomputed here, per kind, and they correct the range
the workstream carried**. Its brief estimated the old static path at "roughly
3,200–3,700 words"; measured, it was **3,108–3,944** — `SKILL.md`'s body plus
that kind's reference file. The estimate was wrong at both ends, so every ratio
below is computed per kind against its own measured before-figure rather than
against a range — which is also why the ratios can be stated at all.

The *static* column includes the guaranteed core (314 words, 353 for `finish`),
which no *before* figure had; the *like-for-like* column strips it, so the two
sides are `SKILL.md`'s body plus the kind reference in both.

| Kind | Static | Reachable | Like-for-like | Before | Ratio |
|---|---|---|---|---|---|
| `design` | 1,149 | 11,741 | 835 | 3,153 | 0.27 |
| `prototype` | 1,169 | 11,761 | 855 | 3,108 | 0.28 |
| `combine-research` | 1,225 | 11,817 | 911 | 3,225 | 0.28 |
| the five `review-*` | 1,260 | 11,852 | 946 | 3,231 | 0.29 |
| the five `integrate-review-*` | 1,280 | 11,872 | 966 | 3,201 | 0.30 |
| `planning` | 1,317 | 11,909 | 1,003 | 3,378 | 0.30 |
| `impl` | 1,334 | 11,926 | 1,020 | 3,125 | 0.33 |
| `research-a`, `research-b` | 1,383 | 11,975 | 1,069 | 3,153 | 0.34 |
| `requirements` | 1,562 | 12,533 | 1,248 | 3,655 | 0.34 |
| `finish` | 1,938 | 12,530 | 1,585 | 3,944 | 0.40 |

**Every kind's unconditional read is between a quarter and two-fifths of what it
was**, with `finish` a shade over the upper end — `1585/3944` is 0.4019, and the
column rounds it to 0.40. The ratio is stated per kind because the aggregate
hides the spread.
`finish` is the largest and `design` the smallest, which is the intended shape: a
kind reference now states what is true of that kind and no sibling, so a kind
with little of its own has little to read — and `finish`, which has the most, is
also the one that shrank least.

The *reachable* column barely varies, and that is a property of the design rather
than a defect in the measure: ten of the pointer graph's fourteen edges leave
`SKILL.md`, so almost the whole conditional corpus hangs off every path. What a
session pays is the static column; the reachable column is what it can be sent
into, and its near-constancy is the price of `SKILL.md` being a router.

**What none of this establishes is that the paths still carry the rules.** A
budget says the path is small; the behavioural coverage in
`tests/lifecycle_invariants.rs` says it still delivers each rule, and
`tests/rule_ownership.rs` says exactly one file states each. Neither is evidence
for the other, and a green budget over a corpus that lost a rule is a smaller
path that teaches less.

What none of them establishes is the semantic limb, *no procedure in
`SKILL.md`*. That classification lost its classifier when the unit markers were
deleted, so it is a **review obligation** — discharged per section against
[`corpus-rule-ownership`](specs/corpus-rule-ownership.md)'s inventory, which names
the file each rule's procedure must be found in — and never a passing test. A
budget test going green says nothing about it. Reviving a marker grammar to make
it checkable is rejected there: enforcement is per rule, by the instrument that
fits that rule, because whether a page of conditions is *right* was never
gateable.

### The boundary is a build, not a commit

`include_dir!` reads `content/` at **compile time**, so what a session receives
is the methodology the running binary was *built* with. The content hash that
makes provisioning idempotent hashes that **embed**, not any working tree — a
warm no-op is therefore correct even when a checkout's `content/` has moved far
ahead of the binary. Nothing in the loop changes this: the full sweep runs once
per bare `grove` (`launch::bare_grove`, before lease acquisition), and
re-*extracting* per iteration would write identical bytes, because a driver
never re-execs and so carries one embed for its whole life. What the loop does
do each iteration is *re-verify* the stamps and restore a directory another
build has taken — the question there is ownership rather than freshness, and it
is answered below.

The consequence is sharpest in a [meta-grove](../CONTEXT.md): a session here can
commit `content/SKILL.md` and the next session in the same loop still reads the
old one. **That is the design, not a defect**, because *any* skew between the
skill and the CLI it instructs is unsafe, in both directions:

| Skew | What breaks |
|---|---|
| Skill **newer** than binary | It instructs verbs added since that build; the binary lacks them. |
| Skill **older** than binary | It instructs verbs removed since that build; the binary lacks them too. |

The second row is the one that surprises, and this repository already holds its
ingredients: the `v17.0.0` skill instructs the composition constructors deleted
after that tag (see the changelog's `### Removed`), so pairing that skill with
any post-`v17.0.0` binary hands a session a call that cannot succeed. Neither
direction is the safe one, so there is no version of "refresh the skill more
eagerly" that helps: **the only safe skew is none.**

Zero skew is what the design actually delivers, by two properties together —
one embed per build, and Grove as the *only* writer of these directories, always
writing its own. Neither row above is reachable while both hold; a skill and its
binary are the same artifact, seen twice. That is why there is no mechanism, and
should be none, for a session to consume freshly committed methodology ahead of
its binary — and why the exposure worth worrying about is not staleness but
anything that breaks the second property (see the shared directory, below).

What is enforceable at this boundary is that the embed is **internally
consistent**: every `grove-llm` verb the embedded methodology instructs is a verb
the embedded CLI exposes (`tests/methodology.rs`, scanning the embed itself). No test can inspect a future
build, so "the installed skill is current" is not a statable claim; "a binary
that ships cannot hand a session a verb it lacks" is, and it is the claim that
actually protects a session.

A stale *installed* binary is therefore an ordinary upgrade concern, diagnosed
with `grove --version` against the repository's `Cargo.toml`, and resolved by
rebuilding and installing — not by anything Grove does at runtime.

### The shared directory, and who owns it

"A session reads the embed its own binary carries" is a claim about one
`grove`, and the skill directories are *global and shared* — the driver lease is
per working tree, so it serializes nothing here. Two builds can write one
directory: a second grove in another working tree, and — more likely inside this
repository — `cargo run --bin grove` from a checkout, which lays that checkout's
`content/` over the installed copy while the session's `PATH` still reaches the
installed `grove-llm`. Grove acts on it in three places:

| Where | What it does |
|---|---|
| Before each launch | Re-verify each installed skill directory's stamp; restore this driver's embed and say so when another build has taken one. |
| Before each launch | Report — never refuse — when the `PATH` `grove-llm` a session would resolve is missing, unidentifiable, or carries a different methodology identity. The driver's environment is a proxy for the session's, exact only when the configured command inherits it. |
| Inside a session | `grove-llm` warns — never refuses — when the installed directories are not stamped with its own identity. Its two operands are the ones that matter, and it is the only check a mid-session clobber can reach. |

Only the middle row is still settled by [one build owns a
session](adr/one-build-owns-a-session.md). That record now describes the pairing
after the mandate becomes the delivery path, where there is no shared directory
left to clobber, so it keeps the launch-time pairing report and no longer carries
the stamp repair or the in-session directory warning. Those two are built
behaviour whose reason *was* the shared directory, and they retire with it — the
rationale is deliberately not restated in a record that would then have to be
deleted at retirement.

Concurrent groves at different builds stay unsupported: one directory cannot
serve two builds, and the reports above make the alternation visible instead of
silent. The supported way to run a build, dogfooding included, is to make it the
installed one a session's `PATH` resolves first — which `cargo install --path .`
achieves only where `~/.cargo/bin` outranks every other prefix carrying a
`grove-llm`, so the diagnostic names the path it actually resolved rather than
prescribing one command.

## Main module seams

| Module | Responsibility |
|---|---|
| `launch` | Provisioning, lease acquisition, and child-environment scrubbing rules. |
| `session_config` | The whole of launch configuration: load and validate the personal file, resolve at most one untracked delta over it per kind, expand one template to argv. Asks `repo` whether a delta candidate is tracked; nothing else leaves the filesystem. |
| `loop_driver` | Foreground iteration, selection, child lifecycle, and completion signals. |
| `driver_lease` | Driver lease, session epoch, signal-channel allocation, and ambient-session validation. |
| `harness` | The provisioning-target registry — delivery destinations only. |
| `repo`, `tree_rename` | Git/Jujutsu detection, scoped commits, the read-only trackedness probe, and the mutation seam. |
| `tree_id`, `tree_read`, `tree_grow`, `tree_lifecycle`, `tree_access`, `tree_format` | Filesystem task-tree model, lock, and format witness. |
| `tree_migrate`, `tree_migration_transaction` | Legacy classification, planning and admission, and its fail-closed mutation owner. |
| `finish_transaction` | The whole fail-closed teardown transaction: preflight, witness, evacuation, rollback, quarantine handoff, and recovery. |
| `finish_cleanup` | Post-commit quarantine and VCS-administration auxiliaries, plus the lease-owned reaping of orphaned ones. |
| `leaf`, `llm_cli`, `complete` | Task formats and the deterministic agent command surface. |
| `methodology` | The embed itself and the build's methodology identity. Nothing else — the corpus is plain markdown, so there is no reader over it. |
| `prompt` | The guaranteed core: the whole of `${prompt}`, the kind→reference-file map, and the too-late test the core's contents are admitted by. Depends on `methodology` for the embed. |
| `provision` | Embedded methodology installation, and the provisioned locations the core names. |

The modules are intentionally file-sized rather than wrapped in another
service layer. The task tree, subprocess boundary, and VCS adapter are the
important seams and are tested through public behavior. No harness abstraction
replaces the removed routing registry: opaque command targets have exactly one
production adapter — direct process execution — so another port would be
hypothetical indirection.

Module visibility is load-bearing rather than incidental: a `pub` item in a
`pub` module is reachable by definition, so `dead_code` never reports one, and a
module stays `pub` only while something outside the crate genuinely calls into
it. A public item whose only callers are tests therefore stops being module
API — deleted where a test can assert on what production reads, demoted into
that module's `mod tests` where the test still needs the convenience. Two *kinds*
of surface are exempt and every item the sweep reports falls under one of them,
argued where it lives: a **seam**, where production reaches the same behaviour
through a door a test cannot open (`tree_lifecycle::transition_to_current`, and
`methodology::markdown_files` — see [the embed test seam](#embed-test-seam)), and
and nothing else — a second exemption for **a frozen grammar kept whole** covered
`leaf_id`, the v1-flat parser, and retired with it when that layout stopped being
read. The list is reproduced by copying `src/` to a
scratch crate, making every module private except `cli` and `llm_cli`, and
reading the compiler's reachability warnings.

## Verification

The principal checks are:

```sh
cargo fmt --check
cargo test --locked
bash plugins/install.test.sh
```

Integration tests drive the real bare `grove` process in temporary Git, native
jj, and colocated jj worktrees, with isolated home directories, a real
`config.kdl`, executable fake commands that record argv/cwd/environment/prompt,
and the real `grove-llm` binary. Clocks, wait policy, lock backends, and kill
graces are injected through internal module seams, never through supported
process configuration.

<a id="embed-test-seam"></a>
Two claims cannot be reached that way, and they have a seam of their own. Every
assertion about the **real embedded corpus** — that each kind's reference file
exists, that the methodology instructs no `grove-llm` verb the CLI lacks, that
the composed core is what the design admits — runs through `methodology` and
`prompt` rather than through a spawned driver. Production's own door onto the
embed is `include_dir`, which a test cannot open without making a runtime
dependency a dev dependency as well, and a driver spawned per claim would pay a
process for each. Those two modules are `pub` on exactly that ground; the rule
that otherwise governs module visibility is under [main module
seams](#main-module-seams).

What is mechanically checkable here is narrower than what the design claims, and
the boundary is stated rather than blurred: the core's three-part shape, its
ending's bytes, its two open couplings and the size alarm are checkable; whether
the wording actually gets a session to read the skill is not, and is carried by
[`wording-micro-test`](research/wording-micro-test.md) and by the human-watched
acceptance run.
