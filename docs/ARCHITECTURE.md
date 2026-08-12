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
| Relationship between Grove and the skill plugins | [`CONTEXT-MAP.md`](../CONTEXT-MAP.md) |
| Methodology executed by agents | [`content/SKILL.md`](../content/SKILL.md) and its adjacent format guides |
| Skill-plugin operation | [`plugins/README.md`](../plugins/README.md) |

The four files under `docs/` are the maintained project guides. This is not a
ban on durable artifacts produced by future Grove work: when a real decision,
specification, or research result earns a repository record, the methodology
may create focused files under `docs/adr/`, `docs/specs/`, or `docs/research/`.
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
| Agent skill plugins | `plugins/linkuistics/`, `plugins/testanyware/` | Claude marketplace or `plugins/install.sh` for portable Linkuistics skills. |

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
             ├─ load and fully validate ~/.config/grove/config.kdl
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
`leaf-prune`; `finish-commit`; `complete`; and `methodology`. This split keeps a discoverable human API without forcing the agent
to reproduce filesystem mutations from prose.

`methodology` is the odd one out and is shaped by that: it mutates nothing and
reads only the binary's **own embed**, serving a unit's source bytes by id or —
given no argument — listing every unit as five tab-separated fields (`<id>`,
`<class>`, `<scope>`, `<defers>`, `<file>`, with `-` in either optional field).
Because it touches no working tree it is dispatched **ahead of the session-epoch
guard**, exactly as `--content-hash` is: the environments a session follows a
`defers=` id from are the ones a tree-resolving verb is refused in, and a refused
lookup there would be a split-brain inside one rule. An unknown id is an ordinary
runtime error naming the id; an unknown id *inside* `content/` is a contributor's
mistake and fails the build.

`grove-llm` also answers `--content-hash` with its build's [methodology
identity](#the-boundary-is-a-build-not-a-commit). That is a flag rather than a
verb on purpose: the driver reads it to report on the pair, no session ever
calls it, and the embedded methodology instructs nothing about it — so it stays
out of the agent grammar the provisioning test scans. A binary old enough not to
answer it is unidentifiable rather than mismatched, and is reported the same way.

`src/main.rs` and `src/bin/grove-llm.rs` are thin entry points. `src/cli.rs`
owns the human grammar; `src/llm_cli.rs` owns the agent grammar.

## Session configuration

`~/.config/grove/config.kdl` is the entirety of user launch policy: a flat map
of all nineteen session kinds to one complete command-template string each, with
no defaults, families, or inheritance. The configuration module loads that one
file into a total kind-to-template map and expands one selected template from a
context of prompt, session name, worktree, and repository root. It hides KDL
handling, aggregate schema diagnostics, POSIX shell-word splitting, substitution
validation, and argv construction; callers cannot ask it for a default, family,
harness, or model. The user-facing grammar and diagnostics are in
[CONFIGURATION.md](CONFIGURATION.md).

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
only as the sole remaining work. More than one live finish leaf is malformed and
stops selection rather than letting eligibility depend on encounter order.
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

`${prompt}` carries the embedded launcher, that stable handle as an explicit
mandate, and the [version control](#symmetric-vcs-rule) Grove resolved for the
working tree. No hidden leaf environment variable accompanies it. At Bootstrap the
session resolves the handle with `grove-llm resolve`, rejects a missing,
ambiguous, terminal, or non-leaf result, reads the glossary, cited decision
records, brief chain, and task, and executes it without calling `grove-llm
pick`. A leaf inserted during the launch window therefore does not preempt a
running session; it becomes the next iteration's work.

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
through validation, rollback, or success output. The working-tree root is used
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
exclusive lock: it validates every slug, resolves the parent, allocates all
positions and keys from one snapshot, and refuses up front on any destination it
cannot prove free, so the only failure that reaches a partial state is a
mid-write error, which unwinds every leaf it created — including the one whose
creation succeeded and whose write did not. Each destination is taken by an
atomic non-clobbering create, so a racing writer that ignored the lock cannot be
truncated or written through, and every path unwound is one Grove provably owns.

That guarantee covers the error return path and nothing else. **Process death
mid-run is not recovered**: rollback runs only when control returns through the
`Err` branch, so a `SIGKILL` after the first pair leaf lands leaves a partial
shape a reader cannot distinguish from a deliberately hand-cut one, and a killed
`leaf-add` can leave a created-but-empty leaf. Finish teardown and the
session-kind migration remain the only operations that promise
process-interruption recovery, which is why they alone carry a witness. The
residue is a hand-editable file in a directory tree, and recovering it is
deleting it.

<a id="self-driving-loop"></a>
<a id="do-is-sole-lifecycle-verb"></a>
<a id="fresh-grove-start-contract"></a>
## Lifecycle and resumption

Bare `grove` is the sole start/continue/finish entry. Each iteration performs at
most one lifecycle transition, and full configuration validation precedes every
one of them, so a missing or malformed `config.kdl` leaves the working tree
byte-identical:

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
command. One planning pass lowers both older directory layouts — the original
`NNN-slug/` tree and the v1 flat dotted-decimal tree — and current-layout trees
whose leaves lack filename kinds, into the current grammar, mapping each legacy
body's `**Kind:**` to a filename kind (absent defaults to `impl`; `work` maps to
`impl`; the two children of an unambiguous legacy vendor pair map to `research-a`
and `research-b`, and a standalone legacy `research` maps to `research-a`,
because the kind names one configured research discipline rather than structural
membership in a pair). It strips every `**Kind:**`, `**Harness:**`, and
`**Producer launch:**` line while preserving all other bytes, including the
composition relationships. An ambiguous pair or unknown marker stops migration
with exact paths rather than guessing a target.

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
Git. Jujutsu working copies are mutated with ordinary filesystem renames and
committed with Jujutsu. Git working copies use `git mv` for tracked moves and
Git for commits. This preserves Jujutsu's operation log and avoids mutating the
Git index behind a colocated repository.

Grove resolves that marker before a session exists and **states** the result in
the mandate, which is why sessions do not probe: every launch is told whether its
working tree is jj-enabled or plain Git, which root Grove resolved for it, and
not to re-derive the answer. The driver already owns this fact and every
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

**This section describes the delivery path as built. It is decided to go.**
[The mandate delivers the
methodology](adr/mandate-delivers-the-methodology.md) settles that `${prompt}`
becomes the sole delivery path, composed from kind-selected byte-exact slices of
the driver's own embed, and that provisioning — the sweep, the stamps, the shared
directory, and the harness registry — retires with it.
[`docs/specs/mandate-delivered-methodology.md`](specs/mandate-delivered-methodology.md)
carries the design. The **build boundary** below is unchanged by that decision
and survives it intact; what changes is only how the embed reaches a session.

`build.rs` embeds `content/` into **both** binaries — `grove` to extract it, and
`grove-llm` to serve units out of it. On every bare `grove`, `provision` sweeps
that content into each installed harness's personal skill directory — a row of
the registry is a place to write files, never a program to run, and an absent
home root is skipped rather than created. A content hash makes this idempotent
while still updating the skill when the binary changes.

That hash is the build's **methodology identity**, and it is the identity
because the crate version does not move between a released binary and an edited
checkout at the same version. It covers the embedded **file payload** — every
embedded file's path and bytes — and deliberately not the embedded directory
structure, so an empty directory is not part of a build's identity; hashing
typed directory entries would make a traversal reproduce `include_dir`'s
directory semantics as well as its file selection. Both binaries compute it from
the linked embed through one implementation (`methodology::identity`). It used
to be a compile-time constant the build script emitted, precisely so that
*naming* the identity did not link `content/`; once `grove-llm methodology`
made the agent-facing binary link it anyway, that reason ended, and the
build-script traversal, the constant and the equality test that kept two
traversals in step went with it. "Both binaries carry it" is a claim about
linked artifacts rather than about source, so it is asserted by scanning
binaries: an integration test scans the pair `cargo test` built, and the release
path scans each staged pair before archiving it, which is where the
cross-compiled `--release` targets a local test never sees are covered.

`build.rs` still walks `content/` — to emit the per-file change tracking
`include_dir!` does not register with Cargo, and to **gate** the embed. Every
embedded markdown file is fully classified by HTML-comment **unit markers**
partitioning its body, and a malformed one fails `cargo build` with the file and
offset. Constraint 5 — grove guides and does not gate — governs the human's task
tree, not grove's own compile-time artifact, which the very build that produced
it can fully observe. The gate reads through the crate's own parser
(`src/methodology/parse.rs`, `#[path]`-included by the build script) rather than
a second implementation, which is the duplication the removed hash traversal had
already had to be defended against.

Because a configured command is opaque, Grove cannot infer which harness a
session eventually reaches and does not try: every known installed root is
refreshed, so whichever one the command lands in already carries the current
methodology. `content/prompts/continue.md` is the single surviving launcher,
augmented with the selected stable-handle mandate and the resolved version
control; it deliberately stays small
and tells the target to use the provisioned skill, which makes that provisioned
methodology a prerequisite of every configured target and one Grove cannot
verify from the outside.

The binary refuses to overwrite an unstamped foreign directory and replaces an
old symlink as a link rather than following it. `content/` is the canonical
source; repository-local or hand-edited copies are not supported.

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
the embedded CLI exposes (`tests/provision.rs`). No test can inspect a future
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
| `session_config` | The whole personal configuration: load, validate, expand one template to argv. |
| `loop_driver` | Foreground iteration, selection, child lifecycle, and completion signals. |
| `driver_lease` | Driver lease, session epoch, signal-channel allocation, and ambient-session validation. |
| `harness` | The provisioning-target registry — delivery destinations only. |
| `repo`, `tree_rename` | Git/Jujutsu detection, scoped commits, and the mutation seam. |
| `tree_id`, `tree_read`, `tree_grow`, `tree_lifecycle`, `tree_access`, `tree_format` | Filesystem task-tree model, lock, and format witness. |
| `tree_migrate`, `tree_migration_transaction`, `leaf_id` | Legacy planning, its fail-closed mutation owner, and the v1-flat name parser. |
| `finish_transaction` | The whole fail-closed teardown transaction: preflight, witness, evacuation, rollback, quarantine handoff, and recovery. |
| `finish_cleanup` | Post-commit quarantine and VCS-administration auxiliaries, plus the lease-owned reaping of orphaned ones. |
| `leaf`, `llm_cli`, `complete` | Task formats and the deterministic agent command surface. |
| `methodology` | The embed itself: the unit reader `build.rs` shares, the embed's unit set, and the build's methodology identity. |
| `provision` | Embedded methodology installation. |

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
that module's `mod tests` where the test still needs the convenience. Two
surfaces are exempt and argued where they live: a **seam**, where production
reaches the same behaviour through a door a test cannot open
(`tree_lifecycle::transition_to_current`), and a **frozen grammar kept whole**
(`leaf_id`, the v1-flat parser, deliberately not trimmed to what the one-time
migration happens to call). The list is reproduced by copying `src/` to a
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
