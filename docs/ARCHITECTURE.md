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
| Whether the campaign's six self-reported lessons survive its own evidence | [`candidate-lessons.md`](candidate-lessons.md) — six adjudicated claims, over the measurements in [`loop-record.md`](loop-record.md) and [`review-yield.md`](review-yield.md) |
| What the campaign taught about driving an LLM loop, and what it cost | [`driving-a-checkable-loop.md`](driving-a-checkable-loop.md) — the account for a reader outside this repository: the cost, the three lessons that now bind in `content/`, and the five that only get written down |
| Alloy 6 against Quint, compared on models this repository no longer carries | [`formalism-findings.md`](formalism-findings.md) — the models were deleted once the lessons were distilled; the log is the surviving evidence |
| How this grove's own sessions ran, session by session | [`loop-record.md`](loop-record.md) — derived, and frozen: its generator was retired with the models |
| What nine review chains actually found, and what survived integration | [`review-yield.md`](review-yield.md) — derived, and frozen, on the same terms |
| What the formal-methods trial returned, and where its unlanded findings go | [`results-of-formal-methods-trial.md`](results-of-formal-methods-trial.md) — the plain-language reading, and the `linkuistics` skills that are the remaining findings' home |
| The observable contract measured before the modularity refactor | [`preservation-baseline.md`](preservation-baseline.md) |

A `TODO.<subject>.md` is a **scoping note with an expiry**: measurements and open
questions for work a future grove will grill, written so the evidence is not
re-gathered, and deleted when the work lands or the question is settled in an
ADR. It is not a plan, not a backlog, and never the canonical description of
anything that exists — those rows are above. **There is none at present.**
`TODO.finish_process.md` was the last, and it ended the second way: its four
questions — two answered `keep`, two `defer` — went into
`docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`, and both the
note and that record are now gone. The record was retired at
`delete-finish-transaction-k8`, which deleted the layer its `keep` protected:
the version control system owns the transaction, so the questions have no
subject rather than new answers.

The rows above are the whole of what a file directly under `docs/` may be: a
maintained project guide, or a durable record a completed Grove workstream
earned. Do not restate that as a count of either — the set changes when a
workstream lands, and a count in this paragraph goes stale silently while
reading as a description. This is also not a ban on new records: when a real
decision, specification, or research result earns one, the methodology may
create focused files under `docs/adr/`, `docs/specs/`, or `docs/research/`. A subdirectory such as `docs/ordinal-fs-tree/` holds the
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
             │    lifecycle transition (root-init or nothing)
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

`~/.config/grove/config.kdl` carries user launch policy: a flat map of session
kinds to one complete command-template string each, with no defaults, families,
or inheritance.

**The whole of that is `crates/keyed-launch`, which has never heard of a
session.** It loads the file — and at most one overlay — into a key-to-template
map, validated whole against a *slot vocabulary* the consumer supplies at load,
and expands one selected template into an argv. It hides KDL handling, aggregate
schema diagnostics, POSIX shell-word splitting, substitution validation, and argv
construction; callers cannot ask it for a default, family, harness, or model, and
it holds no set of keys.

**And it runs what it expanded.** The same crate allocates the launch's
completion channel, spawns the argv directly with no shell, supervises the child
and applies the kill escalation — so `Argv`, which has no constructor, is both
the only thing expansion produces and the only thing a spawn accepts. *Nothing
reaches a spawn that a template did not author* is therefore a fact about the
types rather than a convention grove keeps. `src/session_config.rs` is what is left of grove's side:
the personal file's path, the four slots (`prompt`, `session_name`, `worktree`,
`repo`) grove's templates are written against, and the delta's search and
trackedness rules below. The user-facing grammar and diagnostics are in
[CONFIGURATION.md](CONFIGURATION.md).

**Presence is per kind and just-in-time**
(`docs/adr/complete-session-configuration.md`): both documents are validated
whole before every tree mutation and every launch, but whether a *particular*
kind resolves is asked at the two moments grove commits to it — before it writes
a leaf of that kind, and before it launches one.

At most one second file takes part: an untracked `.grove.kdl` **configuration
delta**, searched at the worktree root and then the main repository root, the
first one found selected outright and the two never merged. It declares any
subset of the kinds and each declared kind's whole template replaces the personal
file's. **It overrides and never supplies**: a kind resolves only if the personal
file declares it, so a file a project could hand you cannot introduce a program
its operator never chose. Resolution is therefore two deep and flat rather than a precedence lattice, and a
kind's launch remains one complete string read whole out of one file — which is
why this leaves [complete session
configuration](adr/complete-session-configuration.md) intact. The module takes
both roots from the driver rather than deriving them, so the search order cannot
disagree with what `${repo}` expands to in the template it selected.

That gives the module its one non-filesystem dependency: because a delta names a
program to execute, a **tracked** candidate is refused rather than trusted to an
ignore rule, so `session_config` asks the VCS seam one read-only question about
one path — and only when a candidate file exists. An unreadable, unparseable,
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
Driver-internal VCS children follow the opposite rule — they scrub both the loop
controls and the repository selectors and are pinned to the leased working tree
by their working directory — so personal launch context cannot redirect a
teardown commit.

## Process ownership

A working tree has at most one live driver. After provisioning, and before
configuration validation or any `.grove/` observation, bare `grove` acquires a
**driver lease**: a nonblocking exclusive advisory lock on a fixed file in a
control directory derived from the closest on-disk `.jj/` marker for that exact
workspace — `<workspace>/.jj/grove/`, for native, secondary, and colocated
Jujutsu alike. The resolver invokes no repository discovery and ignores `GIT_DIR`
and its relatives, so controls live in the exact workspace's administration area
rather than the tracked working copy or an ambient temporary directory. Symlink
and relative-path aliases contend on one lease; separate workspaces stay
independent.

Acquisition creates that control directory. It used to prove one thing more —
that the directory sits on the working tree's own filesystem — because teardown
ended in an atomic same-device rename of the whole `.grove/` root into it. There
is no rename: teardown deletes the tree and takes one path-scoped commit, so the
layout preflight, its device measurement, and the record that specified them are
all gone.

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

`task_name`'s verdict owns the species half for every tree verb, because the
library classifies a listing once and every verb reads the same snapshot — so
selection, resolution, growth, key allocation, and pruning share one answer about
what a sibling is, without which a subtree the reader refuses could stay
invisible to key allocation, lowering the visible maximum key so the next
`leaf-add` re-issues a live one.

There is no format witness and no format metadata inside the tree: **the
filenames are the format**. A tree whose names this grammar does not spell is
refused by name — `TaskNameError` carries what is on disk and the shape it should
have had — rather than classified and converted; Grove does not migrate an older
layout. Task bodies carry no launch metadata at all — only the `**Reviews:**` and
`**Integrates:**` composition relationships below.

`task_name` parses identities, `task_tree` walks and resolves them and owns the
read and write seams onto the library, `task_grow` creates leaves and composition
shapes, `tree_lifecycle` applies terminal outcomes and owns the grove-only
lifecycle, and `tree_access` owns grove's own guard. Every entry that moves,
moves inside an `ordinal-fs-tree` operation, so no module performs a VCS-aware
move — see [the withdrawn tree algebra](#withdrawn-tree-algebra) below.

<a id="withdrawn-tree-algebra"></a>
### The withdrawn tree algebra

Grove used to carry its own tree algebra: a name model, a directory-walking
reader, a path-walking appender, and a version-control-aware move. Increment 2
of gh issue #13 moved each verb group onto `ordinal-fs-tree`, and the contract
stage deleted what was left — `src/tree_id.rs`, `src/tree_read.rs`,
`src/tree_grow.rs`, `src/tree_rename.rs`. Grove supplies a domain
implementation and nothing else; there is no second reader to choose between and
no second grammar to keep in step.

**Two things that look like algebra deliberately survive**, and deleting one
because its name begins `tree_` is the available mistake. `tree_lifecycle` keeps
the lifecycle *around* the tree — the semantics task-tree-scheme fixed, and the
root's own creation, which the library cannot perform because it has to reach the
root in order to snapshot it. `tree_access` keeps grove's own guard, its
transaction sentinel, and the grove-specific refusal that halts a tree with a
pending teardown. Neither has a library counterpart, and neither is about
ordinals or keys.

**The deletion is checked rather than asserted**, in `tests/removed_surface.rs`,
by the method that file already used for the removed launch environment:
enumerate every module-shaped token under `src/` and `tests/` — prose included,
since an essay arguing about a module that no longer exists is worse than no
essay — and classify each against a live set read off disk and a listed
withdrawn set. It carries a positive control (the tokeniser finds a withdrawn
name in a line that has one) and a cross-tree control (the same tokeniser still
finds every withdrawn name in `docs/` and the changelog, where the history
legitimately lives). A clean grep alone would not be evidence: a broken
instrument reads clean everywhere, and the first run of this one reported stale
essays in twelve files that no `use`-line search would have reached.

Because the cross-tree control reads this documentation, **the durable record of
what was removed is load-bearing** — tidying the withdrawn modules out of the
decision records and the changelog breaks the check rather than passing it, which
is the intended direction.

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
answers with `Sought`, which is deliberately not a refusal, so `pick`,
`brief-chain`, `kind` and `resolve` construct nothing and keep the diagnostics
they have carried since before the library existed.

**A root holding no tree is a fourth shape, and clause 3 decides it too.**
`fs::read` and `fs::write` answer *is there a tree here* with a shape rather than
an error — `Reading::Vacant`, `Writing::Vacancy` — so an absent tree is no longer
a library message for Grove to re-state. It is a condition the library states
nothing about, which puts it squarely on Grove's side of clause 3: the vacant arm
raises Grove's own *grove root not found*, the sentence Grove's lock layer
already produced for the same condition, moved rather than redesigned. Grove does
not take the vacancy's other affordance — `Vacancy::initialize` — and `grove new`
still creates the tree root itself; moving that inside the store is
`collapse-tree-access-k13`'s, and until then a verb that created a tree on the
way past would turn a mistyped root into a second workstream. `docs/ordinal-fs-tree/CLI.md` had its read verbs *construct* a
`Refusal::TargetMissing` for want of a message of their own; Grove has one, and
adopting the library's would be clause 3 broken in the opposite direction.

#### Which verbs reach the algebra at all

Nine of thirteen, plus the driver's own `materialize-finish`, and the refusals
they reach need a tree at the edge of the keyspace or the ordinal space — or, for
the one verb the migrate stage has yet to move, one a hand edit or a failed
rollback has damaged.

| verb | library operation | `Refusal`s it can reach |
|---|---|---|
| `pick`, `brief-chain`, `kind`, `resolve` | `walk`, `by_key`, `ancestors`, `distinguished_chain` | **none** — a search answers with `Sought`, which is deliberately not a refusal (nothing was asked to change), so no refusal exists to raise |
| `leaf-add` | `append` | `KeysExhausted`, `OrdinalsExhausted` — and **not** `TargetNotNode` or `DestinationOccupied`; `growing-k33` corrected both rows |
| `leaf-add-pair` | `append_many` | the same two |
| `leaf-insert` | `insert` | `KeysExhausted`, `OrdinalsExhausted` — not `TargetNotNode`, because the target passed is the resolved entry's **container**, a node by construction; and not `DestinationOccupied`, per the row below |
| `leaf-decompose` | `promote` | `KeysExhausted` alone, from the **first child** — a promotion allocates no key for the node, the entity being unchanged; and no ordinal at all, since the node takes the leaf's own and the child takes the first. `promotion-k34` corrected the `DestinationOccupied` this row predicted |
| `leaf-retire`, `leaf-prune` | `rewrite` | **none**, and the row below says why the `DestinationOccupied` this table first predicted is unreachable |
| `root-init` | `append` into a tree it has just created | **none** — the root is not an entry and the level is empty. `lifecycle-k35` transcribed this row and it was **right**, which is the first time that has happened |
| `materialize-finish` (the driver's, not an operator verb) | `append` at the root level | `KeysExhausted`, `OrdinalsExhausted` — the same two `leaf-add` reaches, and from no argument at all, because the verb takes none |
| `finish-commit` | none of the algebra — it selects off the guard's snapshot, then deletes `.grove/` under Grove's own transaction | **none**. Since `lifecycle-k35` its guard is the library's, so a `FINISHING-*` or `PREPARING-FINISH-*` name halts it as `Error::Reserved` — a *parse* refusal in Grove's own words, not a `Refusal` |
| `complete` | none — it touches no tree | **none** |

#### Which refusals Grove's verbs can reach

Two of ten, and neither from an ordinary argument. A refusal no argument produces
is a case a contract test cannot cover and a reader should not go looking for.

**Four rows have since been corrected by the leaves that transcribed them, and
the count fell from four to two**; `lifecycle-k35` is the first leaf whose rows
survived transcription unchanged, and it added one — `materialize-finish`'s —
that reaches the two survivors from no argument at all. `marking-k32` found `DestinationOccupied`
unreachable from the two marking verbs; `growing-k33` found `TargetNotNode` and
`DestinationOccupied` unreachable from the three grow verbs; `promotion-k34`
found `DestinationOccupied` unreachable from `leaf-decompose`, which was the last
row still predicting it. Each correction is the check
[`refusals-k30` scheduled](#library-refusals) working as intended: the table's own
guarantee is that each migrate leaf writes its rows into a suite and finds them
wrong if they are, and it has fired on **every** migrate leaf that had a row to
write — four for four, which is a fact about the protocol rather than about any
one instrument (`docs/formalism-findings.md`, entries 022–024).

What survives is `KeysExhausted` and `OrdinalsExhausted`, and both need a tree at
the edge of the keyspace or the ordinal space. The consequential change is that
**no algebraic refusal reaches an operator from an ordinary argument any more**:
`TargetNotNode` was the only one that did, and the collision `refusals-k30`
weighed was its message. The decision that record reached — print verbatim,
re-word nothing — is unchanged and is now cheaper than it looked, because the
message that collides is one no argument produces.

| `Refusal` variant | reachable from Grove's verbs? |
|---|---|
| `TargetMissing` | **no** — clause 1. A reference naming nothing fails in Grove's resolution, before any operation is called. |
| `TargetNotNode` | **no**, and `growing-k33` corrected this row: it predicted *yes* for `leaf-add <a task file> <slug>` while naming its own contradiction in the next clause — *Grove keeps its own check in front of it*. Both are true of the design and only one can be true of an operator. The check is not optional either, which is what settles it: `.grove/BRIEF.md` is an entry carrying **no key**, so it cannot be handed to the library as a target however the refusal were worded, and clause 2 therefore *forces* the classification that puts this refusal permanently behind one. Asserted in `src/task_grow/tests.rs` over every parent argument that is an entry and not a node. |
| `NoOccupantAtOrdinal` | **no**, in none of its three messages — `leaf-insert` names the **entry** whose slot the new leaf takes, and Grove reads the ordinal off that entry in the snapshot the insert plans from, so `at` is occupied by construction. The syllabus CLI reached all three because `<at>` is an ordinal argument there; Grove's argument surface discharges the refusal `insert` spent two leaves getting right. |
| `PromoteNotLeaf` | **no** — `leaf-decompose` refuses a brief, a `DONE` leaf, an `ABANDONED` leaf and a `finish` leaf, none of which the library can see; a node falls out of the same match. Confirmed by `promotion-k34` over every argument that is an entry and not a live leaf, the grove root included, with a positive control that calls `promote` directly on the same tree and shows the refusal is there for Grove's check to hide. |
| `PromotePartsNotNode` | **no** — `leaf-decompose` always composes node parts, and `Parts::node(_).species()` is `Node` by construction. |
| `NoDistinguishedChild` | **no** — Grove's distinguished child is `BRIEF.md`, so `TaskName::distinguished()` is `Some` and the refusal is about the *domain* rather than about any call. Asserted rather than assumed. That covers both operations the refusal serves: `leaf-decompose`'s promotion, and the root initialization Grove does not yet call. |
| `RewriteSpeciesChange` | **no** — `leaf-retire` and `leaf-prune` compose leaf parts for an entry they have already matched as a live leaf. Confirmed by `marking-k32`: the classification reads `Parts::Leaf` off the snapshot and composes from its own `kind` and `slug`, so no path through either verb can hand `rewrite` node parts. |
| `DestinationOccupied` | **no from any flipped verb**, and it took three leaves to establish. **Not from `leaf-retire` or `leaf-prune`** (`marking-k32`): the occupying name must be exactly the name the mark would place, and an outcome infix and a key are both *parts of one name*, so a `DONE` twin beside the live leaf necessarily carries the live leaf's key — which `task_tree::addressable_key` refuses first. **Not from the grow verbs either** (`growing-k33`), though this row predicted *yes on a hand-edited tree: a copied leaf duplicating a key*, and composing that tree is what showed otherwise. An **append** composes its name with `max + 1` over the whole tree, so no entry in the snapshot can already carry it, whatever a hand edit did. A **shift** composes `(ordinal + 1, key, parts)`, and the only entry that could already carry that name is the sibling one ordinal higher — itself a mover, and already vacated, because the renames run highest-first and the plan is folded through the snapshot in that order. That is the second thing highest-first buys, after the intermediate state, and `ops.rs` says as much in passing — *lowest-first is refused only where a hand edit already duplicated a key and its parts at adjacent ordinals*, which is the tree this row was reaching for. Asserted against `operations.qnt`'s `corrupted` instance rendered in Grove's grammar. **And not from `leaf-decompose`** (`promotion-k34`), which is the row this table predicted longest and the one that looked most likely, since a promotion's destination is composed from an ordinal and a key that already exist. That is exactly why it cannot fire: the node is `compose(ordinal, key, node parts)` with the promoted **leaf's own** ordinal and key, so an occupant of that name is a node carrying that key, the key is duplicated tree-wide, and `addressable_key` refuses before anything is planned. Both shapes of occupant are asserted — the node with a `BRIEF.md`, which is an ordinary hand edit, and the node without one, which is the interrupted promotion below. An adversarial pass sharpened this and is worth carrying: a promotion's **only** exposure to the refusal is its first effect. The two later destinations sit in the directory the plan has just created, and `plan.rs::occupied` answers `false` for a `Level::Created` unconditionally, so no tree state whatsoever — hand-edited, nested, or rollback-damaged — can make them refuse. **So this row rests on exactly one line of Grove's code**, `task_tree::addressable_key`'s tree-wide twin scan, and that is what would reopen it: narrowing the scan from `snapshot.walk()` to a level or a subtree, downgrading its refusal to a warning, or adding a verb that hands `promote` a key without it. A `read` that attached entries unreachable from the root would do it too, since the twin scan and the occupancy scan would stop seeing the same set. |
| `ContentForANode` | **no** — discharged by the verb set. A node arises only through `leaf-decompose`, whose node parts carry no bytes and whose first child is a leaf; `leaf-add`, `leaf-add-pair` and `leaf-insert` compose leaf parts and nothing else. |
| `KeysExhausted` / `OrdinalsExhausted` | **yes** — a hand-written `-k4294967295`, or a position of `4294967295`. That is the exact edge: one more is refused by the grammar as [not canonical](adr/task-names-are-canonical.md), so nothing between the two states is representable. `KeysExhausted` reaches `leaf-decompose` too, and through the **first child** alone: a promotion allocates no key for the node, the entity being unchanged, so the verb's only `max + 1` is the child's. `OrdinalsExhausted` does not — the node takes the promoted leaf's own ordinal and the child takes the first. |

| non-`Io` `Error` variant | reachable from Grove's verbs? |
|---|---|
| `Malformed` | **yes** — a hand-edited name. Carries `TaskNameError` and therefore already speaks Grove's words, which is the whole reason that variant is generic. |
| `Reserved` | **yes** — `FINISHING-*`, `PREPARING-FINISH-*`. Carries `TaskNameError` likewise. |
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

<a id="interrupted-promotion"></a>
**But the process that meets that tree is never the process that left it**, and
that is `promotion-k34`'s finding. The run whose rollback failed reported it and
exited. Every command afterwards opens a tree carrying a duplicate key, and the
library reports *nothing at all* about it: key uniqueness is an obligation on the
domain, and no operation checks it. So the recovery advice for the state the
library warns about is only ever given by whoever meets it, and that is Grove.

This is not clause 3 broken, because there is no library wording in play — the
one that exists was printed in a process that has gone. `task_tree::addressable_key`
therefore recognises the signature exactly (two entries under one key, one a node
and one a leaf, at the same position, the node holding no `BRIEF.md`) and gives
**the library's own recovery**: removing either half resolves it. Its general
duplicate-key advice — *give one of them a fresh key* — is actively wrong here,
and that is why the special case earns its place rather than merely fitting: the
node and the leaf are **one entity** caught mid-shape-change, so giving either a
fresh key would make two entities out of one. Grove itself never writes a
childless node — the only `create_dir` on Grove's tree path is `.grove/` itself,
and every node arises through `promote`, which creates the brief in the same
unit — so no other route in the verb set produces this shape.

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
`<stem>-review` spelling remains a well-formed name.
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

The lock serializes live processes and adds no crash atomicity, and **nothing in
Grove adds any**. The one operation that used to need more — the finish teardown
— carried an in-tree `FINISHING-*` witness that every other command refused
while it existed. Both are gone: Jujutsu snapshots the working copy before every
command and its operation log is the transaction record, so an interrupted
teardown is restored with `jj undo` rather than by a Grove-authored recovery
(`delete-finish-transaction-k8`). The contract this lock offers is
process-interruption consistency between cooperating Grove processes, not
power-loss durability and not crash atomicity.

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
Finish teardown remains the only operation that promises process-interruption
recovery, which is why it alone carries a witness.
The residue is a hand-editable file in a directory tree, and recovering it is
deleting it.

#### Two locks, one at a time

`reading-k31` moved `pick`, `select`, `brief-chain`, `kind` and `resolve` onto
`ordinal-fs-tree`'s own guard (`src/task_tree.rs`). The library takes the same
lock on the same directory for the same reason — the containing directory
outlives the root — but it takes it on **its own** descriptor, and `flock` is
attached to an open file description rather than to a process. So the two guards
do not share a lock, and a verb holding `tree_access::write` that called into the
library's reader would block on itself forever. The rule that follows is per
verb, not per module: a verb uses one guard or the other, never both at once,
which is why the migrate stage moved whole verb groups at a time. Both guards
are still live and both are still needed: the library's is the authority on the
tree, and grove's covers the one thing the library cannot reach — the root's own
creation, which needs the root to not exist yet.

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
decision, and it is taken under the lock. But the library can only say *this
filename is wrong*, and an absent root or a tree held by the finish transaction
are conditions grove states in its own words.
So `task_tree::restate` re-states a *failed* read in the order grove owes its
operator — root, pending transaction, then the library's own message — and
chooses only the wording. The pending-transaction sentence itself
is the domain's: `tree_access::refuse_pending` raises `task_name`'s own
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

#### The root's own creation takes both guards, one after the other

`lifecycle-k35` moved `root-init`, `materialize-finish`, `transition-to-current`
and `finish-commit` onto the library's write path, and one of the four cannot be
done under a single guard at all. The library locks the directory **containing**
the tree root — deliberately, so the lock spans the root's creation and its
deletion — but it still has to reach the root to snapshot it, so it cannot create
one; nor can it create the distinguished child, since a `BRIEF.md` arrives
through `promote` and there is nothing here to promote. Both are therefore
Grove's, under Grove's own guard, and the first leaf is the library's, under its.
Nesting them is the deadlock *Two locks, one at a time* describes, so the scaffold
releases the first before taking the second:

1. Under `tree_access::write_for_lifecycle` — refuse an existing `.grove/`,
   create it, write the root `BRIEF.md`.
2. Under `task_tree::reopen_write` — `append` the first `requirements` leaf.

**The tree's own shape is what makes the window legible.** A root holding its
charter and no keyed entry is a *partial* root, and bare `grove` completes it —
`tree_lifecycle::root_shape` classifies it `PartialScaffold` and
`complete_partial_root_unlocked` writes back whatever half is missing. Nothing
else produces that shape: entries are marked and never removed
([`entries-are-never-removed`](adr/entries-are-never-removed.md)), so a tree that
has ever held a leaf still holds one.

That classification used to be a byte-exact match against the deterministic
fresh-tree content, gated on a missing `.grove/FORMAT` witness. It had to be,
because a witnessless root was *also* how a legacy tree presented and the two got
opposite treatment. Migration is gone (`delete-migration-k6`), and the
discrimination it needed went with it.

Two consequences follow, and both are held by tests rather than by this
paragraph.

- **Completing a scaffold is idempotent.** Another process can complete the
  partial root first, so the append happens only when the snapshot holds no
  positioned entry at all; appending unconditionally would give the tree two
  first leaves and no refusal, since the second would land at ordinal 2 with key
  2 quite legally.
- **A reader can meet a partial root without anyone having died.** Between the
  phases a concurrent `pick` reads a root with a charter and no leaf and reports
  the grove finished, where it used to block on Grove's guard and then read a
  complete tree. The window is two lock acquisitions and two small writes wide,
  and one bare `grove` closes it — but it is a behaviour change and not merely an
  implementation one.

`root_shape` itself does **not** go through the library, and that is not an
omission. It runs while Grove's exclusive guard is held, so reaching for the
library's would be exactly the nesting above — and all it needs is the root's own
listing, classified through the same `TaskName::parse` every reader uses.

`finish-commit` is the fourth verb and the only one whose guard changed what it
refuses. It now opens the tree through `task_tree::write`, so a `FINISHING-*` or
`PREPARING-FINISH-*` name in the root halts it at the guard, in the domain's own
`TaskNameError` words, rather than reaching `finish_transaction::preflight_root`'s
*reserved finish transaction path*. That is one condition with one wording again
([clause 3](#library-refusals)). The preflight check stays as defence against a
writer that ignored the lock, and it re-reads the root through its own
`O_NOFOLLOW` descriptor rather than by path — which is also why the verb still
classifies `.grove` itself before opening the tree: a symbolic link to a directory
elsewhere is a root the library would follow and read, and a no-follow teardown
must refuse it unfollowed.

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
grace → SIGTERM → kill-grace → SIGKILL. **The launch, the watch and the kill are
`crates/keyed-launch`'s** — grove chooses the control directory, the variable
name, the scrub list and the two graces, and reads a meaning out of the token
that comes back; nothing else about the child is its business. That kill is the
*launcher's* job because it is the session's parent, outside whatever sandbox
the session runs under; an in-agent self-kill is silently denied by sandboxes
such as Codex's Seatbelt. The
session commits its artifact and terminal task-tree mutation before signalling
`relaunch` or `done`. If it exits without a signal the driver stops instead of
guessing, reporting the child's exit status and elapsed time — and does not
infer `done` even if that child successfully committed teardown. The filesystem
and VCS already say what completed, and a later `grove` continues from there.

<a id="legacy-migration"></a>
<a id="no-migration"></a>
### No migration

Grove does not migrate. A tree whose names the current grammar cannot spell is
**refused by name**: `TaskNameError` carries the filename on disk and the shape
it should have had, and the operator renames it or starts a fresh grove. There is
no migrate command, no automatic conversion inside bare `grove`, and no format
witness to classify a tree by. Recovery machinery is not written where a sentence
and a human will do.

One shape needs saying because it is not a name the grammar refuses. The layouts
Grove wrote before this grammar — the original `NNN-slug/` + `done/` tree and the
v1 flat dotted-decimal tree — are positioned but *unkeyed*, so every one of their
names is `Foreign` and invisible to the reader rather than refused by it. A root
holding nothing but such names would read as an empty grove and take the driver's
finish sentinel. So the lifecycle transition treats *a root with no Grove entry
at all* as the anomaly and stops on it, naming the entries it disclaimed and the
grammar it does read (`tree_lifecycle::root_shape`). That is one classification
over the listing, not a per-layout matcher: what went with migration
(`delete-migration-k6`) was the recognition of *which* withdrawn layout this is,
which the operator does not need in order to act.

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

### Finish teardown

**Teardown is a delete followed by a commit, and Grove implements no transaction
around it.** It used to: about 10,400 lines over a reserved in-tree
`FINISHING-<finish-handle>/` witness, a manifest recording each entry's type and
digest, an evacuation of every ordinary root entry, a proven rollback, a
workspace-control quarantine, and a recovery path a later driver ran. All of it
is gone (`delete-finish-transaction-k8`), because the version control system
already owns every guarantee it hand-built: Jujutsu snapshots the working copy
before every command, and its operation log is the transaction record.

What `finish-commit` still does is what only Grove can say, and it happens under
one exclusive tree lock held across the whole teardown:

1. **Classify the task root before opening it.** `.grove` is stat'd
   *unfollowed*, so a symlink to a directory elsewhere is refused rather than
   deleted. An absent root is a plain refusal naming `jj op log` and `jj undo`;
   it is no longer routed to a proof that a previous attempt succeeded, because
   there is no longer an attempt that can die halfway.
2. **Revalidate the tree facts.** The live leaf must be a `finish` leaf, and its
   handle must be the one the caller named — so ordinary work that appeared
   after the session started refuses the teardown instead of being swept into
   it.
3. **Require a recoverable tree.** One read-only `jj file list root:.grove`. The
   operation log can only restore what it tracks, so an untracked task tree is
   refused with the command that tracks it rather than deleted into a state
   nothing could undo. This is a precondition, not a surviving piece of the
   transaction: it promises nothing and repairs nothing.
4. **Delete `.grove/`, then commit it.** `fs::remove_dir_all`, then
   `jj commit -m "<handle>: remove completed grove task tree" root:.grove`. The
   fileset scope is what keeps unrelated working-copy changes uncommitted — jj
   snapshots everything and commits only those paths.

Each of the two mutating steps names the operation-log command that undoes it if
it is the one that failed: `jj restore .grove` for a deletion that stopped part
way, since no jj command has run since the last snapshot, and `jj undo` for a
commit that did not land. **No Grove-authored recovery runs**, and none exists to
run. That behaviour was measured before it was relied on (jj 0.44.0, colocated):
`rm -rf .grove/` with no jj command run, then `jj restore .grove`, returned every
file; a partial deletion then `jj undo` reported *"Added 2 files"*, exactly the
missing ones.

<a id="user-owned-worktrees"></a>
<a id="symmetric-vcs-rule"></a>
<a id="version-control-seam"></a>
## Version-control seam

**The seam is a crate, and grove is not in it.** `crates/jj-workspace` resolves
a workspace, refuses a working tree that is not one, hands a consumer a
namespaced control directory, answers what is tracked, and takes a path-scoped
commit — and it has never heard of grove, `.grove/`, a leaf or a lease
(`docs/specs/module-decomposition.md`, decision 8). Its domain-freedom is
enforced at a method rather than asserted in a sentence: `control_dir` takes the
*consumer's* namespace, because *where a lease file may live* cannot be stated
without naming whose lease it is. Grove supplies the one word `grove` and
nothing else about itself.

Resolution walks upward from the current directory looking for one thing: a
`.jj/` directory. **jj is the only lane** — a tree without one is refused before
any mutation, with `jj git init --colocate` named as the remedy
([*jj is the only lane*](adr/jj-is-the-only-lane.md)). That one gate states the
precondition, and nothing downstream branches on which version control owns the
tree. A `.git` beside a `.jj` is a colocated repository and is jj's business:
Grove never reads it, never spawns `git`, and makes no promise about the
colocated index.

**Every child that speaks to the version control system is spawned inside the
crate**, which removes `GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR` and
`GIT_INDEX_FILE` from each one. Choosing the right repository is the seam's
guarantee, so no call site can be written without it. Grove's own
`loop_driver::LOOP_CONTROL_ENV` is the complementary half and stays grove's: it
names the session-ending authority `GROVE_*` carries, which `jj` does not read.
The configured session's spawn receives that list as `keyed_launch::Launch`'s
`scrub` field, so the one spawn allowed to *grant* the channel cannot be written
without first removing whatever it inherited.

**Moves are not commits.** Every entry a flipped verb moves is renamed by
`ordinal-fs-tree`, which does `rename(2)`, detects no repository and requires no
tool on `PATH`. jj snapshots the working copy on its next command, so a leaf
marked `DONE` shows up as one rename and the commit records it as one — nothing
needs staging in between. See
[`grove-does-not-stage-its-own-renames`](adr/grove-does-not-stage-its-own-renames.md).

Grove resolves that marker before a session exists and **states** the result in
`${prompt}`, which is why sessions do not probe: every launch is told that its
working tree is jj-enabled and which workspace root Grove resolved for it.
*Not to re-derive the answer* is the skill's to say, not the prompt's — the core
carries a launch-varying **value**, and every normative consequence of a value
stays in `content/`. The driver already owns this fact; only the session was
working it out again, and working it out badly. A harness banner computed from
`.git` alone reads a native Jujutsu workspace as no repository at all
([claude-code#41435](https://github.com/anthropics/claude-code/issues/41435)),
and detection carried as skill instructions is skippable, so a session that never
loaded them commits with Git in a Jujutsu tree and bypasses the operation log.
The line carries identity and root only. Which commands a session uses stays in
the embedded methodology's Commit step, so a rebuild moves one source of truth
rather than two.

The finish commit is fileset-scoped so unrelated user work survives: Grove
commits a `.grove/` fileset excluding the live witness, leaving unrelated
working-copy changes in the successor commit.

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
per bare `grove` (`loop_driver::bare_grove`, before lease acquisition), and
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
| `session_config` | Grove's side of launch configuration: the personal file's path, the four slots grove's templates are written against, and the delta — where it is searched, which candidate wins, and the refusal of a tracked one. The grammar, the validation and the expansion are `crates/keyed-launch`'s. Asks the VCS seam whether a delta candidate is tracked; nothing else leaves the filesystem. |
| `loop_driver` | Provisioning and lease acquisition on the way in, then foreground iteration and selection. Names the child-environment scrub list and the escalation's two graces; hands both to `crates/keyed-launch`, which owns the spawn, the supervision and the kill. |
| `driver_lease` | Driver lease, session epoch, and ambient-session validation. Supplies the control directory each launch's channel is allocated in; the channel itself is `crates/keyed-launch`'s. |
| `harness` | The provisioning-target registry — delivery destinations only. |
| `repo` | Git/Jujutsu detection, scoped commits, and the read-only trackedness probe. |
| `task_name` | Grove's `ordinal_fs_tree::EntryName` — the whole seam onto the tree library, and the only name grammar grove has. |
| `task_tree`, `task_grow` | The reading and growing verbs expressed through the library: one snapshot per command, path construction, key prediction, and the cross-reference lint. |
| `tree_lifecycle`, `tree_access` | The grove-only lifecycle around the tree, grove's own guard, and its transaction sentinel. |
| `finish_transaction` | The whole fail-closed teardown transaction: preflight, witness, evacuation, rollback, quarantine handoff, and recovery. |
| `finish_cleanup` | Post-commit quarantine of the completed task root, plus the lease-owned reaping of orphaned ones. |
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
