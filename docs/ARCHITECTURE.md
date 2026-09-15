# Grove Architecture

Grove is a small Rust launcher around a filesystem task tree, with the agent
methodology it names shipped separately as a plugin. Durable work stays in
ordinary repository files and VCS;
Grove adds only enough coordination to own one working tree, select one task,
launch one configured agent session, and continue until the tree is complete.

Grove does not know what it launches. One personal file maps each session kind
to one complete command template, and the driver executes the expanded argv
directly. Everything below is what remains once launch policy leaves the binary
— process ownership, a task-tree data model, and the loop that composes them —
recorded as the decisions, the constraints and the measurements behind each.
The description of what the system does at its entry point, how its two command
surfaces are shaped and which module holds what is the
[system overview](walkthroughs/overview/README.md)'s; the description of each
crate's internals is that crate's own walkthrough's. Both left this document as
those books landed, and a stripped section opens with a pointer to the page that
carries what it lost. The description that remains is marked, and each mark says
why it could not go. There are no transactions: the version control system owns
them.

## Documentation ownership

| Subject | Canonical source |
|---|---|
| Project description and installation | [`README.md`](../README.md) |
| Human workflow and commands | [`USAGE.md`](USAGE.md) |
| Session configuration and launch policy | [`CONFIGURATION.md`](CONFIGURATION.md) |
| Cutting and publishing a release | [`RELEASING.md`](RELEASING.md) |
| Runtime and repository design — the decisions, the constraints, and the measurement records | this document |
| Grove vocabulary | [`CONTEXT.md`](../CONTEXT.md) |
| Relationships between this repository's bounded contexts | [`CONTEXT-MAP.md`](../CONTEXT-MAP.md) |
| Working rules for an agent in this checkout | [`CLAUDE.md`](../CLAUDE.md) — `AGENTS.md` is a symlink to it, so both harnesses read one file |
| `ordinal-fs-tree` design and vocabulary | [`ordinal-fs-tree/ARCHITECTURE.md`](ordinal-fs-tree/ARCHITECTURE.md) and [`ordinal-fs-tree/CONTEXT.md`](ordinal-fs-tree/CONTEXT.md) |
| `ordinal-fs-tree` source, read page by page | [`walkthroughs/ordinal-fs-tree/README.md`](walkthroughs/ordinal-fs-tree/README.md) — the code walkthrough: a reader-navigable book whose fragments reconstruct every byte of the crate's frozen corpus |
| `jj-workspace` source, read page by page | [`walkthroughs/jj-workspace/README.md`](walkthroughs/jj-workspace/README.md) — the code walkthrough: a book whose every chapter opens on something the crate declines to own, and whose fragments reconstruct every byte of the crate's frozen corpus |
| The `grove` binary's source, read page by page, and the system description its call reaches | [`walkthroughs/overview/README.md`](walkthroughs/overview/README.md) — the system overview: a book whose every chapter opens at one of the binary's own steps, and whose fragments reconstruct every byte of `crates/grove`'s frozen corpus |
| `grove-llm` source, read page by page | [`walkthroughs/grove-llm/README.md`](walkthroughs/grove-llm/README.md) — the code walkthrough: a book whose every chapter opens on the one thing a thin binary still has to get right, and whose fragments reconstruct every byte of the crate's frozen corpus |
| `keyed-launch` source, read page by page | [`walkthroughs/keyed-launch/README.md`](walkthroughs/keyed-launch/README.md) — the code walkthrough: a book whose every chapter opens on what the crate must not add and must not interpret, and whose fragments reconstruct every byte of the crate's frozen corpus |
| `grove-loop` source, read page by page | [`walkthroughs/grove-loop/README.md`](walkthroughs/grove-loop/README.md) — the code walkthrough: a book whose every chapter opens on what the crate kept when the domain-free crates took the rest, and whose fragments reconstruct every byte of the crate's frozen corpus |
| Methodology executed by agents | [`plugins/grove/skills/grove/SKILL.md`](../plugins/grove/skills/grove/SKILL.md), its adjacent format guides, and one `grove-<kind>` skill per session kind |
| Skill-plugin operation | [`plugins/README.md`](../plugins/README.md) |
| Scoping notes for work not yet started | `TODO.<subject>.md` at the repository root |
| Whether the campaign's six self-reported lessons survive its own evidence | [`candidate-lessons.md`](candidate-lessons.md) — six adjudicated claims, over the measurements in [`loop-record.md`](loop-record.md) and [`review-yield.md`](review-yield.md) |
| What the campaign taught about driving an LLM loop, and what it cost | [`driving-a-checkable-loop.md`](driving-a-checkable-loop.md) — the account for a reader outside this repository: the cost, the three lessons that now bind in the methodology, and the five that only get written down |
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

`docs/walkthroughs/` is the other kind of subdirectory: it holds the code
walkthroughs, one directory per book, each a document a reader navigates from a
contents page through chapters to two lookup indexes. A book root there is owed
a row above, and the obligation is machine-held rather than remembered —
`every_book_root_has_a_documentation_ownership_row`
(`crates/grove/tests/reference_navigation.rs`) discovers the directories and
fails on any this table names nothing canonical for. The same file's curated
user-documentation surface discovers them too, so a book's pages are
link-checked exactly as a guide's are. A sixth book therefore joins that surface
by existing, and joins this table by the single edit the check demands: its own
row.

The former decision-record slugs remain explicit HTML anchors in this document
(for example, `task-tree-scheme` and `symmetric-vcs-rule`). Source comments and
tests use those stable slugs as compact design references; changing a section
title does not change the anchor.

<a id="skills-monorepo"></a>
## Repository products

The repository contains two independently installed products:

| Product | Source | Delivery |
|---|---|---|
| Grove CLI | `crates/` | Homebrew installs `grove` and `grove-llm`. |
| Agent skill plugins | `plugins/grove/`, `plugins/linkuistics/`, `plugins/testanyware/` | Claude marketplace, or `plugins/install.sh` for the skills whose `harnesses:` key declares them installable off Claude Code. |

Grove and the skill plugins share a repository because their documented
interfaces evolve together, but they do not install one another.

**The methodology is a plugin, and grove installs nothing.** It used to sit in
both rows: the binary compiled a `content/` tree into itself and swept it into
every installed harness's skill directory on each invocation.
`delete-provisioning-k19` deleted that half, so `plugins/grove/` is the only
copy and it installs the way its two neighbours do
([`../plugins/grove/README.md`](../plugins/grove/README.md)). Grove names the
skill a session needs and does not check that it is there.

## Runtime flow

The path one invocation takes — the three steps the binary performs before the
call, and the shape of each foreground iteration behind it — is described in the
overview's [*Three steps*](walkthroughs/overview/03-three-steps.md#one-iteration).

**Two advisory steps used to open every iteration and are gone.** The driver
restored any skill directory another build had clobbered, and re-checked the
`grove-llm` a session would resolve through `PATH` against its own methodology
identity — a mid-loop `brew upgrade` being exactly the skew a start-time check
misses. Both had the same subject, the embedded corpus, and both went with it at
`delete-provisioning-k19`. Nothing in the iteration is advisory now:
configuration, lease, and workspace layout are facts the driver establishes
directly and stops on, and the methodology's delivery is the human's.

<a id="cli-binary-split"></a>
<a id="command-surfaces"></a>
## Command surfaces

Both surfaces — the human grammar, which selects nothing, and the agent grammar
of twelve flat verbs — are described in the overview's
[*The surface*](walkthroughs/overview/02-the-surface.md#no-arguments).

`grove-llm` has **twelve** verbs, not thirteen: `open-kind-k20` deleted the
research-pair verb by generalising `leaf-add` to take an ordered list of kinds,
which is what took the last list of kinds out of the machinery. The split into
two binaries keeps a discoverable human API without forcing the agent to
reproduce filesystem mutations from prose.

The agent surface is flat so that `crates/grove-llm/tests/instructed_verbs.rs`
can compare the verbs the shipped methodology instructs against the verbs the
CLI exposes by name, and the flatness is pinned there rather than merely
observed.

Two entries are gone rather than renamed. A twelfth verb, `methodology`, served
the embed by unit id to a session following a `defers=` marker; it went with the
mandate machinery, because the corpus routes by **path** and a session that needs
a procedure opens the file its condition names. And `grove-llm` answered
`--content-hash` with its build's methodology identity, which the driver read to
report the build pairing; that flag, that identity and that report all went with
provisioning at `delete-provisioning-k19`. There is no metadata argument left, so
`grove-llm` takes a verb or prints help.

Since `loop-crate-driver-k22` **both binaries are separate crates** over
`grove-loop`, so *the binary is thin* is compiler-enforced rather than reviewed
(`docs/specs/module-decomposition.md`, decision 1): a binary target declared
beside a library can compile that library's modules into itself and name the
items it keeps private, and a separate package cannot do that without a
`#[path]` attribute pointing outside itself, where it is visible in the entry
point's own files.

## Read-only viewer

Full-width Tree/File switching and leading lifecycle cues are implemented below.
The remaining [item-status design](specs/item-status.md) specifies activity
display, including the typed observation extension to the driver protocol.

`grove view [WORKTREE]` dispatches to the `grove-tui` library before jj workspace
resolution, driver lease acquisition or launch configuration. Its observation
path is the specified directory's `.grove`; it never searches upward.
`grove-tui` owns the terminal lifetime, application state and Ratatui rendering.
Only the human binary depends on it. `grove-loop` and `grove-llm` have no terminal
UI dependencies.

`Viewer::new`, `act`, `tick` and `render` form the application seam shared by the real
terminal and TestBackend fixtures. The adapter uses `grove_loop::try_read` and its
typed snapshot, retains identity, `Handle`, `Kind`, lifecycle, depth, expansion
and descendant totals as separate row data, and
uses the public `entry_path` helper for canonical file paths. It copies rows
and selected bytes under a short shared guard and drops it before layout or
input. There is no persisted viewer state. Busy retains the previous display
with a waiting indicator. A single 500 ms deadline drives automatic observation
and error recovery; selection and explicit refresh enter the same operation.
Two bounded typed captures compare rows and selected bytes, rejecting observed
inconsistency from non-cooperating edits without claiming an atomic transaction.
Every capture releases its guard before another acquisition. A retained open
root directory descriptor holds no tree lock; device/inode identity checks
around capture detect replacement and clear old item state. Root opening is
nonblocking and directory-only; metadata identifies even unreadable replacements.
The adapter rejects duplicate keys before selecting content. Permanent keys
preserve selection and branch expansion; disappearance selects the nearest
surviving ancestor, and moved selections reveal their ancestors.

Observation folds descendant totals in reverse preorder, including hidden leaves.
Both root and branch lifecycle are LIVE if any descendant is live, otherwise
DONE, otherwise ABANDONED, or EMPTY without leaves. Rendering owns all row text:
a fixed 22-cell prefix reserves cursor (2), lifecycle marker (2), explicit word
(10) and future activity (8, currently blank). DONE marker/word/item spans are
green, ABANDONED red, and LIVE/EMPTY normal. Ratatui's cursor gutter supplies
selection without a row-wide style override; fold markers stay beside the item.
At 60 columns the bordered tree has 36 cells after the prefix. Indentation is
capped to retain two fold cells and at least sixteen handle cells, with an
ellipsis for compressed depth. Slugs elide before their key suffix; kinds and
counts fit afterward. Fitting uses Ratatui's grapheme iterator and display width,
with inert control text. Valid handles and kinds remain ASCII by Grove's grammar.

The viewer starts in Tree; Tab switches the entire body to the selected File
and back without reloading or changing the observation deadline. Only the
active view renders. Tree owns its `ListState`; File retains its layout width,
page height, source anchor and horizontal offset while hidden. File reflows for
new dimensions and clamps only when shown; Tree adjusts only to keep its
selection visible. Page actions apply only in File, and help pauses navigation.
Titles, footer and help identify the active view and Tab destination.
Observation continues independently in either view, help and undersized frames.

The private Markdown module consumes pulldown-cmark events with source offsets.
It sanitizes rendered text after entity decoding and lays out styled prose by
terminal grapheme width. Code and tables retain columns and scroll horizontally;
prose reflows. Each rendered line keeps its source range, allowing width changes
to map the top reading anchor into a new layout. A rendered offset within
transformed events distinguishes wrapped inline code and link destinations.
Ordinary revisits save source anchors, shared immutable source text and horizontal
columns by permanent key, bounded to the current root lifetime and accepted
snapshot. Unchanged content keeps its layout and reading position. Successful
captures map from that item's saved source; unselected items map on revisit.
Edit mapping retains common source-line prefixes/suffixes, then chooses the
matching line with the longest contiguous unchanged context, breaking ties by
proximity and source order. Preserved prefixes/suffixes cannot be reused to
impersonate deleted duplicates. Linear prefix-match windows avoid a quadratic
diff on repeated lines. A deleted anchor falls back to the nearest surviving old
line, preferring the following line on ties, then clamps to the current viewport.
When no line survives, the source byte clamps to a UTF-8 boundary before layout.
Only surviving target lines retain their intra-line and transformed-event offsets.
File errors retain the saved anchor and display a diagnostic separately.
The renderer performs no I/O.

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
types rather than a convention grove keeps. `crates/grove-loop/src/session_config.rs`
is what is left of grove's side:
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

A working tree has at most one live driver. Before configuration validation or
any `.grove/` observation, bare `grove` acquires a
**driver lease**: a nonblocking exclusive advisory lock on a fixed file in a
control directory grove does not derive. It asks the resolved workspace for one
under the namespace `grove` and the VCS seam answers `<workspace>/.jj/grove/`,
for native, secondary, and colocated Jujutsu alike — guaranteed inside that exact
workspace, untracked, shared with no other namespace, and created if absent.
Grove supplies the one word the seam cannot know, because *where a lease file may
live* is not sayable without naming whose lease it is. The seam invokes no
repository discovery and ignores `GIT_DIR` and its relatives, so controls live in
the exact workspace's administration area rather than the tracked working copy or
an ambient temporary directory. Symlink and relative-path aliases contend on one
lease; separate workspaces stay independent.

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
stop `blocked` rather than silently park. The escalation signals the session's
process group, so an ambient command the session itself launched is reaped with
it and only a process outside that group can still hold the guard
([the launched child is a job](./adr/the-launched-child-is-a-job.md)). Manual `grove-llm` commands with no
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

A node directory is `NN-k<key>/` and its exactly-one node file is
`_<slug>.md`; the root's is `_BRIEF.md`. A leaf is
`NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md`. The
[naming decision](adr/task-names-are-canonical.md) owns canonicity and the
[node and handle design](specs/module-decomposition.md) owns their composition.

The directory pays only for position and key in each descendant path. Its node
file carries the title in its filename and the brief in its body. The name
module composes a node handle from both parsed names; file contents never
supply a routing field. The root has neither slug nor work-item handle.

A malformed name or level stops the snapshot read. The reader validates every
reachable level under the tree guard before selection or mutation can consume
it, so an early live leaf cannot hide a missing or second node file elsewhere.
`_BRIEF.md` is valid only at the root, and a titled node file only in a
positioned node. Errors name the affected level and canonical form.

The library enforces distinguished cardinality and invokes the name type's
level validation; it owns no label vocabulary. The same checks run on projected
plan results before effects. `leaf-decompose` supplies the source slug's node
file to promotion, and `root-init` supplies the root name and bytes to
initialization. No writer creates a charter outside that plan.

Every tree verb shares this classification, including key allocation. Grove
accepts this grammar only, performs no automatic conversion and leaves foreign
names outside the grammar alone. All tree movement remains inside
`ordinal-fs-tree` operations.

<a id="withdrawn-tree-algebra"></a>
### The withdrawn tree algebra

Grove used to carry its own tree algebra: a name model, a directory-walking
reader, a path-walking appender, and a version-control-aware move. Increment 2
of gh issue #13 moved each verb group onto `ordinal-fs-tree`, and the contract
stage deleted what was left — `src/tree_id.rs`, `src/tree_read.rs`,
`src/tree_grow.rs`, `src/tree_rename.rs`. Grove supplies a domain
implementation and nothing else; there is no second reader to choose between and
no second grammar to keep in step.

**One thing that looks like algebra deliberately survives**, and deleting it
because its name begins `tree_` is the available mistake. `tree_lifecycle` keeps
the lifecycle *around* the tree — the semantics task-tree-scheme fixed, and the
grove's own creation, which it now performs through the store's `Vacancy`. It has
no library counterpart, and it is not about ordinals or keys. `tree_access`, which
used to sit beside it holding Grove's own guard, is itself withdrawn:
`collapse-tree-access-k13` deleted the second lock layer once all three of its
recorded reasons had dissolved.

**The deletion is checked rather than asserted**, in `crates/grove-llm/tests/removed_surface.rs`,
by the method that file already used for the removed launch environment:
enumerate every module-shaped token under every package's `src/` and `tests/`,
and under `testing/` — prose included,
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

### Authoritative selection and mandate

**Finish is reserved, not blocking**, and both halves matter. Selection skips
the driver-owned `finish` leaf while any non-finish leaf is live, so
`leaf-insert` sequences work ahead of it — and ordinary `leaf-add` may also
*append* work behind it, which is the case that bites, since the finish leaf
keeps the earlier position and nothing but the skip rule stops teardown being
proposed while live work sits after it. Grove encodes no dependencies,
priorities, or scheduler outside sibling order.

<!-- residue marks. `architecture-residue-k75` deleted every passage a landed
     walkthrough book had made redundant. What the surviving marks record is why
     a descriptive passage was **not** deleted: `residue(none)` means no crate
     book covers it, because it describes the methodology plugin rather than a
     crate; `residue(pinned)` means a book does cover it, but
     `crates/grove-llm/tests/composition_guidance.rs` holds the wording on this
     document, and re-pointing that assertion elsewhere would drop a surface
     from a check rather than move it. -->

<!-- residue(none): the Bootstrap sequence is the methodology's, not a crate's -->
**One pick is authoritative, and it is a fact rather than a routing forecast.**
The driver performs exactly one per iteration, and that single value serves
readiness, the launch diagnostic line, template selection, and the mandate, with
no second tree read; it is not recomputed immediately before spawn. Its read
guard is released before the spawn, so the mandated session can take an exclusive
tree guard while the driver still owns the loop. What the prompt then carries,
and how the driver composes it, is the [`grove-loop`
walkthrough](walkthroughs/grove-loop/README.md)'s.

`prompt-names-the-kind-k18` deleted a nineteen-to-ten reference map, a
nineteen-to-two ending map, and the skill-directory list, and that is what leaves
Grove interpreting a kind nowhere: it renders the kind's own label into a skill
name and stops. No hidden leaf environment variable accompanies it. At Bootstrap
the session resolves the handle with `grove-llm resolve`, rejects a missing,
ambiguous, terminal, or non-leaf result, reads the glossary, cited decision
records, brief chain, and task, and executes it without calling `grove-llm
pick`. A leaf inserted during the launch window therefore does not preempt a
running session; it becomes the next iteration's work.

<a id="library-refusals"></a>
### How an `ordinal-fs-tree` refusal reaches an operator

**Verbatim, unchanged, and rarely** — because Grove resolves and classifies its
target before it calls the library, so the refusals that reach an operator are
exactly the ones whose words are already true of a Grove tree.

**Name and level errors speak Grove's words; algebraic refusals cannot.**
`Error::Malformed`, `Error::Reserved` and `Error::InvalidLevel` carry
`EntryName::Err`, Grove's own `TaskNameError`; `Error::Refused` carries
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
already produced for the same condition, moved rather than redesigned. **Which
arm answers is per entry point, not per crate**: `task_tree::write` refuses a
vacancy, because a verb that created a tree on the way past would turn a mistyped
root into a second workstream, and `task_tree::write_or_vacancy` hands it over —
and its only callers are the two verbs whose business is making a grove
(`collapse-tree-access-k13`). `docs/ordinal-fs-tree/CLI.md` had its read verbs *construct* a
`Refusal::TargetMissing` for want of a message of their own; Grove has one, and
adopting the library's would be clause 3 broken in the opposite direction.

#### Which verbs reach the algebra at all

Nine of twelve, plus the driver's own `materialize-finish`, and the refusals
they reach need a tree at the edge of the keyspace or the ordinal space.

| verb | library operation | `Refusal`s it can reach |
|---|---|---|
| `pick`, `brief-chain`, `kind`, `resolve` | `walk`, `by_key`, `ancestors`, `distinguished_chain` | **none** — a search answers with `Sought`, which is deliberately not a refusal (nothing was asked to change), so no refusal exists to raise |
| `leaf-add` | `append` for one kind, `append_many` for a list | `KeysExhausted`, `OrdinalsExhausted` — and **not** `TargetNotNode` or `DestinationOccupied`; `growing-k33` corrected both rows. It reaches the same two through either operation, which is why `open-kind-k20` could fold the pair verb into it without adding a row |
| `leaf-insert` | `insert` | `KeysExhausted`, `OrdinalsExhausted` — not `TargetNotNode`, because the target passed is the resolved entry's **container**, a node by construction; and not `DestinationOccupied`, per the row below |
| `leaf-decompose` | `promote` | `KeysExhausted` alone, from the **first child** — a promotion allocates no key for the node, the entity being unchanged; and no ordinal at all, since the node takes the leaf's own and the child takes the first. `promotion-k34` corrected the `DestinationOccupied` this row predicted |
| `leaf-retire`, `leaf-prune` | `rewrite` | **none**, and the row below says why the `DestinationOccupied` this table first predicted is unreachable |
| `root-init` | `Vacancy::initialize` — the root, its charter and the first leaf as one operation under the lock that found the vacancy | **none** — the root is not an entry and the level is empty. `lifecycle-k35` transcribed this row against `append` and it was **right**; `root-lifecycle-belongs-to-the-store` moved the creation without moving the answer |
| `materialize-finish` (the driver's, not an operator verb) | `append` at the root level | `KeysExhausted`, `OrdinalsExhausted` — the same two `leaf-add` reaches, and from no argument at all, because the verb takes none |
| `finish-commit` | `WriteGuard::delete` — it selects off the guard's snapshot, then consumes that guard to remove the root | **none**. `delete` refuses a root spelled through a link, which this verb has already refused unfollowed for its own reason (below), and reports the paths that went |
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
| `TargetNotNode` | **no**, and `growing-k33` corrected this row: it predicted *yes* for `leaf-add <a task file> <slug>` while naming its own contradiction in the next clause — *Grove keeps its own check in front of it*. Both are true of the design and only one can be true of an operator. The check is not optional either, which is what settles it: `.grove/_BRIEF.md` is an entry carrying **no key**, so it cannot be handed to the library as a target however the refusal were worded, and clause 2 therefore *forces* the classification that puts this refusal permanently behind one. Asserted in `crates/grove-loop/src/task_grow/tests.rs` over every parent argument that is an entry and not a node. |
| `NoOccupantAtOrdinal` | **no**, in none of its three messages — `leaf-insert` names the **entry** whose slot the new leaf takes, and Grove reads the ordinal off that entry in the snapshot the insert plans from, so `at` is occupied by construction. The syllabus CLI reached all three because `<at>` is an ordinal argument there; Grove's argument surface discharges the refusal `insert` spent two leaves getting right. |
| `PromoteNotLeaf` | **no** — `leaf-decompose` refuses a brief, a `DONE` leaf, an `ABANDONED` leaf and a `finish` leaf, none of which the library can see; a node falls out of the same match. Confirmed by `promotion-k34` over every argument that is an entry and not a live leaf, the grove root included, with a positive control that calls `promote` directly on the same tree and shows the refusal is there for Grove's check to hide. |
| `PromotePartsNotNode` | **no** — `leaf-decompose` always composes node parts, and `Parts::Node`'s species is `Node` by construction. |
| Invalid supplied distinguished name or projected level | **no from an ordinary Grove constructor** — decomposition supplies the leaf slug's node file and initialization supplies `_BRIEF.md`. Grove creates child nodes only by promotion and supplies leaf parts for each new child. Hand-edited invalid levels fail when the snapshot is read. |
| `RewriteSpeciesChange` | **no** — `leaf-retire` and `leaf-prune` compose leaf parts for an entry they have already matched as a live leaf. Confirmed by `marking-k32`: the classification reads `Parts::Leaf` off the snapshot and composes from its own `kind` and `slug`, so no path through either verb can hand `rewrite` node parts. |
| `DestinationOccupied` | **No from Grove tree verbs on a validated snapshot.** Marking would collide only with a twin carrying the same key, which `task_tree::addressable_key` refuses first. Appending uses a fresh tree-wide key. Inserting shifts highest-first, and the planner folds each rename through the snapshot, so a moving sibling has vacated its destination. Promotion keeps the leaf's ordinal and key: an existing destination node with its required node file duplicates that key and is refused before planning; a node missing its file is refused by snapshot validation even earlier. Promotion's later destinations are inside the directory its plan creates. This claim depends on whole-tree snapshot validation, the tree-wide ambiguity check and occupancy checks against the projected plan; narrowing any of those boundaries requires revisiting the fixtures. |
| `ContentForANode` | **no** — discharged by the verb set. A node arises only through `leaf-decompose`, whose node parts carry no bytes and whose first child is a leaf; `leaf-add` and `leaf-insert` compose leaf parts and nothing else. |
| `KeysExhausted` / `OrdinalsExhausted` | **yes** — a hand-written `-k4294967295`, or a position of `4294967295`. That is the exact edge: one more is refused by the grammar as [not canonical](adr/task-names-are-canonical.md), so nothing between the two states is representable. `KeysExhausted` reaches `leaf-decompose` too, and through the **first child** alone: a promotion allocates no key for the node, the entity being unchanged, so the verb's only `max + 1` is the child's. `OrdinalsExhausted` does not — the node takes the promoted leaf's own ordinal and the child takes the first. |

| non-`Io` `Error` variant | reachable from Grove's verbs? |
|---|---|
| `Malformed` | **yes** — a hand-edited name. Carries `TaskNameError` and therefore already speaks Grove's words, which is the whole reason that variant is generic. |
| `Reserved` | The library supports a domain-owned reserved-name verdict; Grove's filename partition uses malformed names and foreign names. |
| Malformed level | **yes** — a missing, competing or misplaced node file. The name type supplies Grove's canonical form and the reader attaches the level path. |
| `Failed` | **yes in the wild, from no argument** — the filesystem refuses mid-apply and the run unwinds. **The tree is as it was found**, so a retry is safe. |
| `FailedPartiallyRolledBack` | **yes in the wild, from no argument** — and the one message whose *recovery advice* is stated in the library's words. See below. |
| `NonUtf8Name` | **not on macOS** — APFS refuses such a filename, so the branch cannot be reached from a test on this host. Assert that fact rather than skipping it; `docs/formalism-findings.md` entry 006. |
| `NameIsNotOneComponent` | **no** — a `Slug` admits lowercase ASCII letters, digits and hyphens only, so no name `TaskName` renders can be more or less than one path component. |
| `NoContainingDirectory` | **no** — the tree root is always `<worktree>/.grove`, which always has a containing directory. |
| `RootIsNotATree` | **yes in the wild, from no argument** — a `.grove` that is a regular file, a socket, or a symbolic link naming one. Not producible by any verb: Grove creates the root as a directory or not at all, so reaching it takes a hand or another program. The message names what it found and stops, which is the whole of the right answer — the library will not move aside something it did not put there. |
| `RootIsNotSpelledDirectly` | **no** — the tree root is always `<worktree>/.grove`, composed by Grove rather than typed: its last component is a name Grove wrote, and no `..` appears in it. This is `delete`'s precondition and reaches no other operation. |
| `RemovalStopped` | **no** — Grove calls nothing that removes anything. `WriteGuard::delete` exists (`root-delete-k26`) and the finish teardown is wired to it at `loop-crate-verbs-k21`, which is the leaf that re-takes this row. When it does, the two readings matter: with nothing removed the tree is as it was found, and with anything removed it is in neither state and the repository's history is the way back. |

#### What verbatim costs, measured rather than assumed

The collision is real, and composing the offending message at design time is
what sizes it. Were clause 2 dropped, `grove-llm leaf-add 03-impl--extract-k7.md
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
node file carries no key and so cannot be handed to the library as a target at
all.

`FailedPartiallyRolledBack` is the one message left speaking the library's words
where it matters: *a node and a leaf sharing an ordinal and a key, with the node
holding no distinguished child, is an interrupted promotion*. In Grove's words
that is a node directory and a task file sharing a position and a key, with the
directory holding no node file. It prints verbatim, because it fires on a failed
rollback rather than on an argument and because
[`CONTEXT-MAP.md`](../CONTEXT-MAP.md) carries the six-term translation — a map
between two glossaries cannot drift from the messages it translates, where a
re-wording of each message can.

<a id="interrupted-promotion"></a>
A promotion interrupted before the leaf moves can leave a leaf and node sharing
a key, with the node lacking its own file. The next snapshot refuses that node
as a malformed level before handle lookup can inspect the duplicate key. The
error preserves the directory path and required node-file form, with conditional
recovery advice: before creating a brief, check whether this directory is empty
and a sibling leaf shares its position and key. If so, delete the empty
directory to keep the leaf, or move the leaf into it as `_<slug>.md`, using that
leaf's slug, to keep the node. Merely assigning a fresh key or manufacturing a
node file does not recover one entity interrupted while changing shape.

This advice appears on every missing positioned-node-file error and does not
assert that a matching sibling was found. The operator verifies the condition.
Automatic recognition is deliberately lost at this earlier refusal boundary:
a failed open returns no guarded snapshot, and another read of the invalid tree
cannot supply one. The naming ADR records why conditional advice is preferred
to extending the store's interface solely for this diagnostic.


<a id="task-kind-taxonomy"></a>
## Task kinds and composition

A session kind gives each session a discipline and gives the driver its
configuration key. **Grove holds no set of them**
([`a-kind-is-an-open-token`](adr/a-kind-is-an-open-token.md)): a kind is any
well-formed token — lowercase ASCII letters, digits and single hyphens, no `--` —
and the kinds that *exist* are the `grove-<kind>` skills the installed
methodology ships. The table below is the methodology's current set of twenty-three,
not the binary's; adding another is authoring a skill and declaring a template
for it, never editing this repository's Rust.

**Grove spells a kind token only where it writes the leaf itself**, with no
session to delegate to, and only for a leaf it can recognise afterwards: the
grow and terminal verbs refuse the driver-reserved `finish` as a kind or
operand, and that refusal is the licence for naming the token at all. Which
tokens those are is the [`grove-loop`
walkthrough](walkthroughs/grove-loop/README.md)'s.

<!-- residue(none): the methodology's current set of kinds, in two tables and the paragraphs around them; the open-token decision above and the editorial-pipeline measurement record stay -->

| Producer | Purpose | Review | Integration |
|---|---|---|---|
| `requirements` (HITL) | Establish what should be built through human dialogue. | `review-requirements` | `integrate-review-requirements` |
| `design` | Establish how; produce current-state specs or decisions. | `review-design` | `integrate-review-design` |
| `planning` | Decompose the design into vertical agent-sized leaves. | `review-planning` | `integrate-review-planning` |
| `prototype` (HITL) | Build a cheap artifact to provoke human reaction, not to ship. | `review-prototype` | `integrate-review-prototype` |
| `impl` | Produce shippable code, docs, or tests. | `review-impl` | `integrate-review-impl` |
| `research-a` | Produce a primary-source survey. | `research-b`, the independent second survey | `combine-research` |

Four more kinds are **not** producers in that sense, and the empty cells below
say so positively: a `—` means the kind takes no step of that species at all,
never that one is still to be authored.

| Editorial stage | Purpose | Review | Integration |
|---|---|---|---|
| `draft` | Produce a document from its sources and its human-authored structure brief, owning structure and technical truth. | — | — |
| `copy-edit` | Sentences, terminology and consistency with the document's prose contract; no restructuring. | — | — |
| `art` | The document's figures, in the medium its contract admits. | — | — |
| `proof` | The final whole-document read; every class in charter. | — | — |

Each editorial stage is itself a producer that reads the whole document and
*fixes* within one charter, so the stage after it is not an adversarial read of
it and buys it no `review-`/`integrate-review-` pair. Which stages exist was
measured rather than asserted
([`the-editorial-pipeline-is-four-kinds`](adr/the-editorial-pipeline-is-four-kinds.md)).

<!-- residue(none): what reviews and integrations are, and which kinds are HITL -->
Reviews are fresh-context adversarial reads that produce findings rather than
fixes. Integrations verify each finding, then fix the contract, fix the
artifact, accept a visible trade-off, or reject noise. `requirements` and
`prototype` are human-in-the-loop because human words or reactions are their
essential input; any other kind may still stop and ask.

<!-- residue(none): the three shapes as the methodology composes them, through *No shape gets a node directory*; the two deliberate differences and the feedback-edge record stay -->
Three documented composition shapes exist, all as **flat siblings** named off a
shared stem:

- Review chain: `X → review-X → integrate-review-X`
- Research pair: `research-a → research-b → combine-research`
- Editorial chain: `draft → copy-edit → art → proof`

None of the three is known to the machinery. A review chain is three separate
`leaf-add` calls; a pair is one `leaf-add` given three kinds, which lands them as
one unit at consecutive positions with consecutive keys; an editorial chain is
four `leaf-add` calls, each the last act of the stage before it. The tokens are
spelled on the command line by the session that owns them, which is what took the
last list of kinds out of grove's source.

No shape gets a node directory *of its own*. The editorial chain nonetheless
always runs inside one, because a document's leaf becomes a node the moment it is
decomposed into stages — an ordinary node, carrying an ordinary node file, which
is where the running `## Handed forward` list lives and whose live entries a stage
reads to decide whether its successor is already queued. It differs from a review chain in two further ways, and both are
deliberate: **membership is mandatory** — a stage is never skipped on a judgement
that it would find nothing, because which stages exist was measured — and there
is **no integrate step**, because each stage fixes rather than reports. A defect
an earlier stage owns is sent back as forward tree growth: a contiguous run of
re-run leaves from the owning stage through `proof`
([`a-feedback-edge-is-forward-tree-growth`](adr/a-feedback-edge-is-forward-tree-growth.md)).

<!-- residue(pinned): `resolve` on a chained stem and the grow verbs' refusal are the `grove-loop` book's, but *pick-style*, *exit zero* and the refusal sentence are pinned on this document -->
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
have known. A research pair stays **eager**, one all-or-nothing call — `leaf-add`
with an ordered list of kinds — because a `research-b` cut by `research-a`'s
session would inherit that session's framing and corpus and destroy the
independence the pair is run for. Three separate calls would be the same failure
in another form: three snapshots, three chances to stop half way, and a live
prefix of a pair that is indistinguishable from a deliberately hand-cut partial
one.

<!-- residue(none): the integration-placement rule; `composition_guidance.rs` pins *cross-leaf grammar*, *leaf-insert* and the placement condition here -->
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
before any later sibling of an ancestor. Nothing in the binaries enforces or
checks that. `research-a` and
`research-b` share one discipline but are separate configuration keys, which is
how a vendor pair reaches two different commands without any per-leaf metadata;
whether those two commands are materially independent is configuration-owner
policy, because Grove cannot compare opaque strings.

<!-- residue(none): the in-session reviewer allowance, `references/execute.md`'s -->
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

<!-- residue(none): the relationship lines; `composition_guidance.rs` pins `**Reviews:**` and `**Integrates:**` here -->
A chain's steps declare their relationships in their bodies: the review carries
`**Reviews:** <producer-handle>` and the integration carries `**Integrates:**
<review-handle>`. Those lines are **written by hand by the session authoring the
body and parsed by nothing** — a documented convention (the spine's `TASK-FORMAT.md`)
for the human and for the session that picks the step up, which is constraint 3:
task files are freeform markdown and nothing validates them. Names and positions
likewise remain presentation and walk order, never relationship grammar, and the
driver routes a scheduled review solely by its filename kind.

<a id="tree-access-lock"></a>
### Tree access lock

Ordinary reads and writes block and retain the CLI's waiting diagnostic.
Observers use `grove_loop::try_read`, acquired only through `task_tree.rs`.
The store attempts one real nonblocking shared lock: Busy carries no snapshot;
success checks presence and builds the snapshot under that same descriptor.
The observer prints no contention diagnostic and never probes then reopens.

**The Tree access lock is the store's own, and since `collapse-tree-access-k13`
it is the only one** — Grove kept a second layer of its own for as long as there
were things the library could not do to a tree it had to reach in order to read,
and there are none left. One exception is deliberate and recorded: a **bulk** mark holds one
guard per entry it marks, because a library mutation consumes its guard — see
[`bulk-marks-are-not-atomic`](adr/bulk-marks-are-not-atomic.md) and *One guard is
one mutation* below. The lock is taken on the *working-tree root* rather than on
`.grove/` because that is the one thing that exists before root initialization
and survives finish deletion, so a single seam covers creation, ordinary
mutation, and teardown.

**Nothing in Grove adds crash atomicity.** The one operation that used to need
more — the finish teardown — carried an in-tree `FINISHING-*` witness that every
other command refused while it existed. Both are gone: Jujutsu snapshots the
working copy before every command and its operation log is the transaction
record, so an interrupted teardown is restored with `jj undo` rather than by a
Grove-authored recovery (`delete-finish-transaction-k8`).

<!-- residue(pinned): *all-or-nothing on a reported error*, *Process death mid-run is not recovered* and *process-interruption recovery* are pinned on this document -->
`leaf-add` given a list of kinds is all-or-nothing **on a reported error** within
one exclusive lock, and since `growing-k33` that is the library's doing rather
than Grove's: Grove's own reconstruction of the same guarantee — an up-front
destination sweep, an `O_EXCL` claim per leaf, a per-run rollback list — went
with the verb, and nothing of it is left.

That guarantee covers the error return path and nothing else. **Process death
mid-run is not recovered**: the interpreter unwinds only when control returns
through the `Err` branch, so a `SIGKILL` after the first pair leaf lands leaves a
partial shape a reader cannot distinguish from a deliberately hand-cut one.
**No operation promises process-interruption recovery**, teardown included: it
is a delete and a commit, and what puts a half-finished one back is the operation
log. The residue of a partial add is a hand-editable file in a directory tree,
and recovering it is deleting it.

#### One lock, and it is the library's

`reading-k31` moved the read verbs onto `ordinal-fs-tree`'s own guard, and for a
while Grove kept a second guard beside it. The library takes the same lock on the
same directory for the same reason — the containing directory outlives the root —
but it takes it on **its own** descriptor, and `flock` is attached to an open
file description rather than to a process. So the two guards did not share a
lock, and a verb holding Grove's that called into the library's reader would
block on itself forever. The rule that followed was per verb, not per module: a
verb used one guard or the other, never both at once, which is why the migrate
stage moved whole verb groups at a time.

**`collapse-tree-access-k13` deleted the second layer**, and the deadlock with
it. It survived that long for three recorded reasons — the tree Grove had to
classify might be absent, legacy or mid transaction, and the library could read
none of those — and all three dissolved at once: `open-shape-k25` made an absent
tree a **shape** (`Reading::Vacant`, `Writing::Vacancy`), `delete-migration-k6`
left no legacy shape to recognise, and `delete-finish-transaction-k8` left no
transaction. The one thing Grove's guard covered that the library could not —
creating the root, which needs the root to not exist yet — is
`Vacancy::initialize`'s, taken under the exclusive lock the vacancy already
holds. Grove now takes exactly one `flock` of its own anywhere in its source —
the non-blocking contention probe below — and
`crates/grove-llm/tests/tree_lock.rs` holds that by enumerating every package's
`src/`.

**The one-at-a-time rule outlived the second layer, because two descriptions are
enough to express it.** A caller holding a `TreeWrite` whose guard is still
unspent, and calling something that opens the tree itself, blocks against its own
process exactly as the deleted layer did — which is why that type's header says
*take the opening you need, spend it, and let it go*. One verb genuinely needs
its own opening: the cross-reference lint reports on the tree a `leaf-insert`
left, and reads it under a shared lock (below, *A verb that reports on the tree
it changed needs a second opening*). It calls `TreeWrite::relinquish` first, so
it obeys the rule rather than excepting itself from it, and grove still never
holds two.

Two consumer-side obligations came out of that move, and every later flip leaf
inherits both.

**The waiting diagnostic is bought outside the library.** Locking is invisible
in the library's interface by design — no try-variant, no timeout, `read` and
`write` simply block — so nothing in it can say *someone else is holding this*.
Grove has always said so, and losing it in a refactor that promises to change no
behaviour would be a real regression, so grove buys the message back with a
non-blocking probe of its own. It is a diagnostic and never a decision: between
that probe's release and the library's acquisition a contender can arrive, and
the cost of that window is a missing message and nothing else.

**Refusal precedence is grove's; the halt is the library's.** The library halts
the whole tree on a name grove's grammar refuses, wherever it sits — that is the
decision, and it is taken under the lock. But the library can only say *this
filename is wrong*, and a root that is absent, or holds something that is not a
tree, is a condition grove states in its own words. So grove re-states a *failed*
read in the order it owes its operator and chooses only the wording. Something at
the root that is not a tree is ordered ahead of an absent root deliberately:
`is_dir` reads a dangling symbolic link at the root as *absent*, and the
library's `RootIsNotATree` already says what is there and that a tree is a
directory, which is more than grove's *not found* would.

#### One guard is one mutation, and a bulk mark is many

`marking-k32` moved `leaf-retire` and `leaf-prune` onto the library's `rewrite`,
which is the first mutation Grove performs through it. A mutating method
**consumes** its `WriteGuard`, so a bulk mark is *N* rewrites under *N* guards
where it was one critical section. Grove accepts that
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
first — an order neither model establishes. So Grove refuses a key that names
more than one entry, before any operation is called. Without that refusal,
`leaf-retire` aimed at one twin rewrote the other onto its own name, changed
nothing, and reported success. Every flipped verb goes through it, and
every verb the migrate stage has yet to move should: the hazard belongs to
*resolve a path, then call by key*, which is the shape of all of them.

#### A verb that reports on the tree it changed needs a second opening

`leaf-insert` lints stray position-prefixed cross-references left stale by the
renumber it just made. It has to read the tree the **shift left**, and the
mutation consumed the guard that could have shown it, so the verb opens the tree
again. That is a second observation, deliberately, and it announces its own wait
like any other.

**That second opening is shared, and it is gone before anything is printed.**
Both halves are `lint-lock-scope-k32`'s, and the leaf's subject was the exclusive
printing version it replaced.

The lint reads and never writes, so what it needs is *writers* held off while it
walks — which is what a shared lock is. The exclusive one held readers off too,
so `pick`, `kind` and `brief-chain` waited on a whole-tree content scan for
nothing. Holding **no** lock is not the third option it looks like: the store's
reader takes one to take a snapshot at all, so a lint holding nothing would have
to walk the tree outside the reader and could be handed a half-renamed level.

And the hits are **returned rather than written**. The old shape took a caller's
sink and `writeln!`d each hit into it under the lock — and a `writeln!` to a pipe
whose reader has stopped draining blocks, so a harness that captured a session's
stderr and stopped reading could wedge every grove process on the worktree behind
the tree's exclusive lock. The scan now hands its hits back and drops its guard
with them, so the printing happens with no lock held and there is no sink inside
the critical section to stall in — and a failed write is dropped rather than
returned, because the insert has already landed and a lint that cannot print must
not turn a reported mutation into a failure.

What this gives up is the claim the old test made — that a hit was *printed*
while the tree was held, so it could not name a path something had since
renamed. That claim never survived the call's own return: an operator reads
stderr long after the guard is gone, and any grove process may rename in that
window. What remains is what was load-bearing, and the tests now say so: every
hit comes from **one consistent snapshot**.

The lint also **scans the snapshot** rather than the directory, so what it reads
is every leaf and every charter — the same set every other verb calls the tree —
and a foreign `.md` a hand edit dropped into `.grove/` is no longer scanned.
Grove writes no such file, and the alternative is a second, wider notion of
*what is in the tree* than the reader has.

#### The library allocates the key; the consumer's content embeds it

Grove's leaf body opens with the position-free handle `# <slug>-k<key>`, and the
library takes those bytes **before** it composes the name that carries the key. A
content-carrying domain therefore cannot render its content from the answer, and
has to predict the allocation.

The alternative was to create with `NewEntry::empty` and write the body
afterwards, from the key the report carries. It was rejected because the guard is
consumed by the mutation, so that content write lands **outside** it — and
because it would hand `append_many` a run whose three files land atomically and
whose three bodies do not, which is the all-or-nothing property the composite
verb exists for. Predicting keeps the content atomic with the creation, and pays
for it with one check.

It is a prediction and it is checked, because the silent failure is a leaf whose
first line contradicts its own filename — permanently, and invisibly. The
prediction reads the same snapshot under the same guard, so it can only be wrong
if the library's allocation rule changes, which is exactly what the check exists
to catch. An exhausted keyspace predicts nothing and hands the library no bytes:
`Refusal::KeysExhausted` is the library's to state, a refusal writes nothing, and
the unrenderable content is never reached.

#### The grove's own creation is one store operation

An exclusive opening of an absent root returns `Writing::Vacancy`, holding the
lock on the containing directory. `Vacancy::initialize` accepts `_BRIEF.md`
and its bytes alongside the first leaf; it validates the complete plan before
creating the root and unwinds reported failures under that guard. `root-init`
and the driver's scaffold use the same operation. A present tree cannot be
passed as a vacancy.

A present root with only `_BRIEF.md` is taskless and refused. Any root lacking
its node file, including an empty or foreign-only root, is malformed and refused
before classification. A valid root holding foreign names alongside its node
file and no positioned work is unrecognised and refused. The ordered test is
[a witnessless root refuses what it cannot account for](adr/a-witnessless-root-refuses-what-it-cannot-account-for.md).
A process interruption can leave an incomplete root; no automatic repair assumes
ownership from a charter's bytes.

The root listing is read under the store's guard when lifecycle diagnostics
need to name foreign entries, which are deliberately absent from the snapshot.
That diagnostic does not introduce another grammar or tree reader.

`finish-commit` is the fourth verb, and what it stopped needing is the more
interesting half. It used to run a preflight of its own — `preflight_root`, which
re-read the root through an `O_NOFOLLOW` descriptor and refused a *reserved finish
transaction path*, the in-tree `FINISHING-*` witness the transaction wrote and
every other command refused while one existed. `delete-finish-transaction-k8`
deleted the transaction, so nothing Grove writes can produce that name and there
is no second condition for a second wording to disagree with: a stray
`FINISHING-*` is a **foreign** entry every reader walks past, which is the same
answer `delete-migration-k6` reached for a stray `.grove/FORMAT`, and
`crates/grove-llm/tests/session_kind_tree.rs` asserts it rather than assuming it.

What survives the preflight's deletion is the one check the guard cannot make: a
symbolic link to a directory elsewhere is a root the library would follow and
read, and **a teardown may not delete a directory elsewhere as if it were its
own**. Past that gate the guard is the authority and the removal is the store's,
which is what lets the session name the paths that went in the commit message it
writes by hand.

<a id="self-driving-loop"></a>
<a id="do-is-sole-lifecycle-verb"></a>
<a id="fresh-grove-start-contract"></a>
## Lifecycle and resumption

Bare `grove` is the sole start/continue/finish entry. Each iteration performs at
most one lifecycle transition, and full configuration validation precedes every
one of them, so a missing or malformed `config.kdl` — or an invalid or tracked
`.grove.kdl` delta — leaves the working tree byte-identical. Which state maps to
which transition is the [`grove-loop`
walkthrough](walkthroughs/grove-loop/README.md)'s.

A fresh grove creates its root node file and first *leaf* in one store operation,
so the first session has work to select. The driver refuses a root holding only
`_BRIEF.md` as taskless before selection; it does not finish that root.
The helper verbs `grove-llm pick`, `kind` and untargeted `brief-chain` perform
selection without that lifecycle classification: on a taskless root they report
"no live leaves; this grove is done". That helper result is not a driver finish
decision. Creation is working-tree only; the first session's focused commit
folds in the scaffold.

Task-root absence is the complete fresh-tree discriminator. Grove consults no VCS
history, abandoned signal channel, or unlocked lease bytes to decide that a
missing tree used to be a finished one: a teardown commit proves that Grove
deleted an earlier tree but not whether the present invocation means "recover
that" or "start another".

**Ending a session is the launcher's job**, because the launcher is the session's
parent, outside whatever sandbox the session runs under; an in-agent self-kill is
silently denied by sandboxes such as Codex's Seatbelt.

The session commits its artifact and terminal task-tree mutation before
signalling. **A session that exits without signalling stops the loop**, and the
driver does not infer `done` even if that child successfully committed teardown:
the filesystem and VCS already say what completed, and a later `grove` continues
from there.

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
Grove wrote before this grammar are positioned but *unkeyed*, so every one of
their names is foreign and invisible to the reader rather than refused by it, and
a root holding nothing but such names would read as an empty grove and take the
driver's finish sentinel. So the lifecycle transition treats *a root with no
Grove entry at all* as the anomaly and stops on it. That is one classification
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

**Finishing happens inside a real, resumable session**: declining or exiting
writes no signal and leaves the finish leaf for a later `grove`. No commit is
made *for* that leaf and it is never retired — its addition and deletion cancel
in the focused finish commit. `finish-commit` cannot attest that a human spoke
through an opaque command; it is the deterministic last-moment tree and VCS
guard, not a substitute for that HITL contract. Grove deliberately does not merge
branches/bookmarks or remove working trees.

### Finish teardown

**Teardown is a delete followed by a commit, and Grove implements no transaction
around it.** It used to: about 10,400 lines over a reserved in-tree
`FINISHING-<finish-handle>/` witness, a manifest recording each entry's type and
digest, an evacuation of every ordinary root entry, a proven rollback, a
workspace-control quarantine, and a recovery path a later driver ran. All of it
is gone (`delete-finish-transaction-k8`), because the version control system
already owns every guarantee it hand-built: Jujutsu snapshots the working copy
before every command, and its operation log is the transaction record.

Two preconditions of the teardown are grove's own, and both are refusals rather
than repairs. **Ordinary work that appeared after the session started refuses the
teardown** instead of being swept into it: the live leaf must be a `finish` leaf,
and its handle must be the one the caller named. And **an untracked task tree is
refused rather than deleted** — the operation log can only restore what it
tracks, so requiring a recoverable tree promises nothing and repairs nothing, but
keeps a deletion out of a state nothing could undo.

**No Grove-authored recovery runs**, and none exists to run: each mutating step
names the operation-log command that undoes it instead. That behaviour was
measured before it was relied on (jj 0.44.0, colocated): `rm -rf .grove/` with no
jj command run, then `jj restore .grove`, returned every file; a partial deletion
then `jj undo` reported *"Added 2 files"*, exactly the missing ones.

<a id="user-owned-worktrees"></a>
<a id="symmetric-vcs-rule"></a>
<a id="version-control-seam"></a>
## Version-control seam

**The seam is a crate, and grove is not in it.** `crates/jj-workspace` has never
heard of grove, `.grove/`, a leaf or a lease
(`docs/specs/module-decomposition.md`, decision 8), and its domain-freedom is
enforced at a method rather than asserted in a sentence: `control_dir` takes the
*consumer's* namespace, because *where a lease file may live* cannot be stated
without naming whose lease it is. Grove supplies the one word `grove` and
nothing else about itself. What the crate resolves, refuses and answers is the
[`jj-workspace` walkthrough](walkthroughs/jj-workspace/README.md)'s.

**jj is the only lane** — a tree without a `.jj/` is refused before any mutation,
with `jj git init --colocate` named as the remedy ([*jj is the only
lane*](adr/jj-is-the-only-lane.md)). That one gate states the precondition, and
nothing downstream branches on which version control owns the tree. A colocated
repository is jj's business: Grove never reads it, never spawns `git`, and makes
no promise about the colocated index.

**Choosing the right repository is the seam's guarantee**, so every child that
speaks to the version control system is spawned inside the crate and no call site
can be written without the `GIT_*` scrub it applies. Grove's own session-ending
authority is the complementary half and stays grove's: it names what `GROVE_*`
carries, which `jj` does not read, and the one spawn allowed to *grant* that
channel cannot be written without first removing whatever it inherited.

**Moves are not commits.** Every entry a flipped verb moves is renamed by
`ordinal-fs-tree`, which does `rename(2)`, detects no repository and requires no
tool on `PATH`. jj snapshots the working copy on its next command, so a leaf
marked `DONE` shows up as one rename and the commit records it as one — nothing
needs staging in between. See
[`grove-does-not-stage-its-own-renames`](adr/grove-does-not-stage-its-own-renames.md).

**Sessions do not probe for the version control system, because Grove states it
in the launch.** *Not to re-derive the answer* is the skill's to say, not the prompt's — the launch carries a
launch-varying **value**, and every normative consequence of a value stays in the
methodology. The driver already owns this fact; only the session was working it
out again, and working it out badly. A harness banner computed from `.git` alone
reads a native Jujutsu workspace as no repository at all
([claude-code#41435](https://github.com/anthropics/claude-code/issues/41435)),
and detection carried as skill instructions is skippable, so a session that never
loaded them commits with Git in a Jujutsu tree and bypasses the operation log.
Which commands a session uses stays in the methodology's Commit step, so there is
one source of truth rather than two.

The finish commit is fileset-scoped so unrelated user work survives, leaving
unrelated working-copy changes in the successor commit.

The user owns topology. Grove reads no branch or bookmark, creates no working
tree, and performs no integration or teardown. The working-tree basename is
the grove name, and `<repo-basename>: <grove-name> grove` is the session name a
template may request.

<a id="self-extension-core-and-methodology"></a>
## How the methodology reaches a session

**It is installed, and Grove has nothing to do with it.** The methodology is the
`grove` plugin — a spine skill plus one `grove-<kind>` skill per session kind
([`../plugins/grove/README.md`](../plugins/grove/README.md)) — installed by a
human through the Claude Code marketplace or `plugins/install.sh`. `${prompt}`
names the one skill this session's kind needs and carries nothing else about
delivery.

### What was here before, and why none of it survived

Grove used to carry the methodology itself. `content/` was a markdown corpus
compiled into **both** binaries with `include_dir!`, and every bare `grove` swept
it into each installed harness's personal skill directory before taking
ownership of a working tree. Six mechanisms hung off that one idea, and
`delete-provisioning-k19` deleted all six together:

| mechanism | what it was for |
|---|---|
| the embed, and `build.rs`'s per-file `rerun-if-changed` walk | making the binary carry a corpus `include_dir!` does not change-track on stable |
| the harness registry | naming the three directories to sweep into — a row was *a place to write files*, never a program to run |
| the sweep, its content-hash stamp, and its staging-and-rename | writing those directories idempotently and crash-atomically |
| the methodology identity (`sha2` over the embed) | naming *which build* a directory or a binary belonged to, since the crate version does not move between a release and an edited checkout at that version |
| `grove-llm --content-hash`, and the driver's per-iteration probe of it | reporting when the `grove-llm` a session's `PATH` resolves came from a different build than the driver |
| the per-verb foreign-skill-directory warning, and the absent-destination report | saying so when a directory carried another build's methodology, or when no destination existed at all |

Every one of them was machinery for making a **shared mutable directory** safe.
A plugin has an install route of its own, so there is no directory for two
builds to contend over, no stamp to compare, and no pairing to report. The two
records that argued the design — `skill-delivers-the-methodology` and
`one-build-owns-a-session` — are retired with it.

### The boundary that survives, and the one that does not

**A build boundary still separates an edit to grove's source from the sessions
it reaches.** A running driver is the build already in memory; it never re-execs,
so committing a change to the loop, the verbs or the prompt changes nothing any
session in that loop sees until the binary is rebuilt *and installed*. That is
what makes a meta-grove's cutover leaves cutover leaves.

**No boundary separates an edit to the methodology from the next session.**
Editing a skill in a checkout reaches a session as soon as the install route
resolves to that checkout — immediately through `install.sh`'s symlinks, at the
next update through the marketplace. The embed used to make that a build
question and to drive the skew between skill and CLI to zero by construction;
that guarantee is now weaker, and it is stated rather than pretended away.

**What still holds the two in step is one test rather than one artifact.**
`crates/grove-llm/tests/instructed_verbs.rs` asserts that the shipped methodology instructs no
`grove-llm` verb the CLI lacks — reading the skill set as markdown, file by file,
so a verb invented in prose or dropped from the CLI fails without anyone
remembering to add it. It is load-bearing precisely because the skill set is the
only thing teaching a session which verbs exist, and it pins the flat verb
surface that makes the comparison mean what it claims. Both halves are versioned
in this one repository and land in one commit; a user who upgrades one and not
the other is the residue, and it is
[`../plugins/grove/README.md`](../plugins/grove/README.md)'s to state.

**The delivery assertion is the plugin's own.** `plugins/grove/conformance.sh`
walks the shipped skill set and asserts every behavioural rule is present on the
composed loaded path of every kind that binds it, that no rule has two owners,
and that every file a skill names by path exists. The Rust suites that made those
claims over the embedded corpus went with it.

### The seven constraints, argued

[`../plugins/grove/skills/grove/SKILL.md`](../plugins/grove/skills/grove/SKILL.md)
carries the numbered spine itself, and
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
   by eye, because the handle is composed from the entry's names.
3. **Suggested shape, not enforced schema.** Task files and briefs are freeform
   markdown and the format files are guides. A schema over prose buys validation
   of the half that never fails and forbids the improvisation that makes a leaf
   body useful.
4. **Lazy and optional.** ADRs, specs and glossary entries are
   created when they earn their place. Every root and node requires its node
   file at creation; its brief is developed as the work becomes concrete. Lazy
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
in which that happens. The normative rule is the spine's `CONTEXT-FORMAT.md`'s; what
this section records is why the corpus spends a static-path condition on it.

<a id="corpus-shape"></a>
### The corpus's shape, and what is measured over it

<!-- residue(none): the corpus's shape; no crate book covers the plugin -->
The spine's `SKILL.md` states **conditions** and routes; `references/` states the
shared **procedures**, and a kind's own rules are inline in its
`grove-<kind>` skill. That split is the whole of what makes the methodology
progressively disclosed — a session is handed one skill, reads a page of
conditions, and opens only what its situation names.

**Which file states a given rule** is decided under
[every normative rule has one owner](adr/corpus-rules-have-one-owner.md): a rule
is filed by **when a session meets it** — the pair *which kinds must obey it* and
*at which moments*, resolved by an ordered first-match rule over a set of
occasions. Under [a restatement declares its
class](adr/restatement-declares-its-class.md) a `SKILL.md` restatement declares
one of three classes (`own`, a ≤25-word `trigger`, or `none`). The inventory of
every rule with its owner, class, load predicate and test is
[`../plugins/grove/conformance/rules.tsv`](../plugins/grove/conformance/rules.tsv),
which `plugins/grove/conformance.sh` reads as data — the spec it was transcribed
from went with `content/`.

One consequence belongs here rather than there: only `plugins/grove/skills/` is
installed, so a rule moved into `docs/` is unreachable to every session outside
this repository. *Normative material stays in the skill set* is the placement
function's own first case read backwards rather than a separate boundary.

**What is measured, and by what.** `plugins/grove/conformance.sh` is the whole
standing instrument: every behavioural rule present on the composed loaded path
of every kind that binds it, no rule with two owners, and every file a skill
names by path existing. It asserts nothing about how many kinds there are, on
purpose — a kind exists iff a skill of that name exists.

**Five numeric budgets used to stand beside it and are gone.** They measured
`SKILL.md`'s body and its `## The loop` section in words, and each kind's static
and reachable loaded path — the path composed out of `content/SKILL.md`,
`reference_file(kind)` and that kind's signal file. `prompt-names-the-kind-k18`
deleted the composition and `delete-provisioning-k19` deleted the corpus, so
their subject is gone; `crates/grove-loop/tests/prompt.rs`'s 4 KiB ceiling on each kind's
`${prompt}` is the one that remains, because the prompt is still composed. The
arguments the budgets were built on are worth keeping and are recorded below,
because the next person to reach for a size measure over prose will need them.

**Why words rather than lines.** A **line** is not a unit anyone reads: a line
budget is discharged by rewrapping, which changes nothing a session pays. A word
ceiling is constraint 7 — *one page of rules* — made recomputable, and "a page"
is otherwise unmeasurable. But a word ceiling puts no upper bound on lines at
all, so the deletion of the old line limits rested on density rather than
domination: the body measured 7.1 words per line and the loop section 6.9, and
reaching the old line limits under a 900-word ceiling would have needed prose
3.9× and 2.8× sparser.

**Why a section measure is not dominated by a body measure.** Constraint 7 is
specifically that *the loop* fit a page. A whole-body budget sees one number and
is indifferent to where inside it the words sit, so prose moved into *The loop*
leaves every other measure exactly as it was while the section constraint 7 names
grows without limit. That is why the section carried a structural claim beside
its number — the section is `- When …` items and nothing else — since a word
ceiling cannot distinguish a condition arriving from a paragraph of summary
arriving, and it is the summary constraint 7 forbids.

**Why words rather than tokens.** Tokens are what a session pays and are
model-specific; a reproducible token count needs a vendored tokenizer and
vocabulary, and a budget that needs a download is a budget that stops running.
Words track tokens monotonically across prose in one voice, which is what a
*growth* alarm needs — and the honest limit is that a word count cannot price a
register change, so the reading is always "this path grew", never "this path
costs N tokens".

**Why a budget is asserted from both sides.** Each was fitted to a measurement
recorded beside it, and held within a band above it, so a limit nothing
approaches failed as loudly as a path that outgrew one. Without the recorded
measurement the only checked interval was `measurement ≤ ceiling ≤ measurement +
25%`, which admits a ceiling sitting exactly on today's measurement — the
zero-width fit the band exists to prevent — and a ceiling raised straight to the
far edge without ever being fitted.

**And why none of it is coverage.** A budget says the path is small; only a
delivery assertion says it still carries the rules. A green budget over a corpus
that lost a rule is a smaller path that teaches less.

#### What the loaded paths measured, before and after

**A frozen record of one acceptance, not a live measure.** The instruments were
deleted at `delete-provisioning-k19` with the corpus they measured; the numbers
are kept because they are the evidence the corpus rewrite was accepted on.

The corpus rewrite's acceptance, recorded as the comparison rather than as a
claim. *Before* is the start of the workstream that produced this section
(`b6ecdbd0`): `content/` totalled 23,532 words and `SKILL.md` 3,152, its body
3,081. *After*, `content/` totalled 14,741 words and `SKILL.md`'s body 796.

The before figures are **recomputed here, per kind, and they correct the range
the workstream carried**. Its brief estimated the old static path at "roughly
3,200–3,700 words"; measured, it was **3,108–3,944** — `SKILL.md`'s body plus
that kind's reference file. The estimate was wrong at both ends, so every ratio
below is computed per kind against its own measured before-figure rather than
against a range — which is also why the ratios can be stated at all.

The *static* column below is **as it was measured then**, and included the
guaranteed core (314 words, 353 for `finish`), which no *before* figure had; the
*like-for-like* column strips it, so the two sides are `SKILL.md`'s body plus the
kind reference in both. `prompt-names-the-kind-k18` took the core off the static
path and put that kind's signal file on it, so the rows the budgets last
measured were already a different composition; this table is the record of that
acceptance and was never restated against it.

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
budget says the path is small; `plugins/grove/conformance.sh` says each rule is
present on every bound kind's path and that exactly one file states it. Neither
is evidence for the other.

What none of them establishes is the semantic limb, *no procedure in
`SKILL.md`*. That classification lost its classifier when the unit markers were
deleted, so it is a **review obligation** — discharged per section against
[`../plugins/grove/conformance/rules.tsv`](../plugins/grove/conformance/rules.tsv),
which names the file each rule's procedure must be found in — and never a passing
test. A
budget test going green says nothing about it. Reviving a marker grammar to make
it checkable is rejected there: enforcement is per rule, by the instrument that
fits that rule, because whether a page of conditions is *right* was never
gateable.

<a id="the-boundary-is-a-build-not-a-commit"></a>
### The boundary is a build, not a commit

**For the binary, and no longer for the methodology.** A running driver is the
build already in memory and never re-execs, so a change to the loop, the verbs
or the prompt reaches no session in the same loop: the boundary is a rebuild
*and* an install, not a commit. In a [meta-grove](../CONTEXT.md) that is the whole
reason a leaf whose deliverable the installed build cannot read has to publish a
release and stop the loop rather than signal.

**The methodology no longer sits behind that boundary at all**, and what was
bought by putting it there is worth stating, because it was real. `include_dir!`
read `content/` at **compile time**, so a session received the methodology its
own binary was built with, and Grove was the only writer of the skill
directories — two properties which together made the skew between a skill and
the CLI it instructs exactly zero:

| Skew | What breaks |
|---|---|
| Skill **newer** than binary | It instructs verbs added since that build; the binary lacks them. |
| Skill **older** than binary | It instructs verbs removed since that build; the binary lacks them too. |

The second row is the one that surprises, and this repository already holds its
ingredients: the `v17.0.0` methodology instructs the composition constructors
deleted after that tag (see the changelog's `### Removed`), so pairing it with
any post-`v17.0.0` binary hands a session a call that cannot succeed. Neither
direction is safe, so there was no version of "refresh the skill more eagerly"
that helped: **the only safe skew is none.**

`delete-provisioning-k19` gave that up deliberately, because the price of keeping
it was a shared mutable global directory and every mechanism that made one safe —
stamps, per-iteration repair, a build-pairing probe, a per-verb warning. The
methodology and the binaries now have separate lifetimes and separate install
routes, and a user who upgrades one and not the other can reach either row above.

**What replaces the guarantee is a test and a repository boundary.**
`crates/grove-llm/tests/instructed_verbs.rs` asserts that this checkout's skill set instructs no
`grove-llm` verb this checkout's CLI lacks — the internally-consistent claim, made
over the pair that ships together in one commit rather than over one linked
artifact. No test can inspect a future build, so "the installed skill is current"
was never a statable claim either; what is statable is that the two halves this
repository publishes agree with each other.

A stale *installed* binary is an ordinary upgrade concern, diagnosed with
`grove --version` against the workspace's one version field, and resolved by
rebuilding and installing — not by anything Grove does at runtime. A stale
installed methodology is the same kind of concern, resolved by updating the
plugin.

## Main module seams

The workspace map — the packages, what each is, and the responsibility of
every `grove-loop` module — is the overview's
[*What the call reaches*](walkthroughs/overview/05-what-the-call-reaches.md#the-package-map),
from its package map onward.

**The workspace root is not a package**, so a module's package is part of its
identity (`docs/specs/module-decomposition.md`, decision 1).

There is no `harness`, `methodology` or `provision` module. All three were
provisioning's — a registry of directories to sweep into, the embed and its
identity, and the sweep itself — and went at `delete-provisioning-k19`. There is
no `leaf` module either: it held `Kind`, which is `task_name`'s since
`open-kind-k20` made a kind an ordinary validated word of the filename grammar.

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
through a door a test cannot open (`grove_loop::driver::transition_to_current`),
and nothing else — a second exemption for **a frozen grammar kept whole** covered
`leaf_id`, the v1-flat parser, and retired with it when that layout stopped being
read. The list is reproduced by copying a package's `src/` to a scratch crate,
making every module private except its own entry surface — `cli` for
`grove-llm`, and `verbs`, `prompt`, `session_config` and the crate root for
`grove-loop` — and reading the compiler's reachability warnings. `crates/grove`
needs no such copy: it is a binary with no library, so every item in it is
already private to one target.

**Since `loop-crate-verbs-k21` the compiler does more of this on its own.** A
module that moved into `crates/grove-loop` is private to *that* crate, so
`dead_code` now reports an item whose only callers are another package's
tests — which is how the four path-taking compositions (`task_tree::read`,
`pick`, `select`, `kind`) turned out to be the tests' alone and moved into the
module's own `mod tests`. A crate boundary is a reachability boundary, and that
is a second thing the split bought besides the one decision 1 argues for.

## Verification

The principal checks are one command:

```sh
bash scripts/check.sh
```

It runs seven — `cargo fmt --all --check`, `shellcheck`, `cargo clippy
--workspace --all-targets`, `bash plugins/install.test.sh`, the two conformance
scripts, and `cargo test --locked --workspace` — announces the `cargo` and
`rustfmt` its PATH resolved, and ends on a punch list of whichever failed.
`--workspace` on both cargo lines because this root is *also* a package: a bare
invocation tests and lints `grove` alone and leaves the other five crates unread.

The two conformance scripts are the methodology's own, and they are shell rather
than Rust because the thing they assert about — the composed loaded path of an
installed skill set — has no counterpart in the binary
([`behavioural-coverage-asserts-delivery`](adr/behavioural-coverage-asserts-delivery.md)).

**Nothing gates these checks.** There is no CI in this repository, and the
script does not add one: it is run by a person or a session, and a release runs
it because it *is* [step 1 of the release](RELEASING.md#1-prepare-the-release).
That is a weaker claim than a server that refuses a push, and it is stated
rather than implied — while the list lived as prose in two documents, `cargo fmt
--all --check` was named in no leaf's `## Done when` and drifted red across four
leaves of the crate split with nothing to report it. The script exists so the
list is retyped by nobody; it does not make a green tree a guarded one. The
script's own header carries why it pins no toolchain.

Integration tests drive the real bare `grove` process in native jj and colocated
jj worktrees, with isolated home directories, a real
`config.kdl`, executable fake commands that record argv/cwd/environment/prompt,
and the real `grove-llm` binary. Clocks, wait policy, lock backends, and kill
graces are injected through internal module seams, never through supported
process configuration.

<a id="embed-test-seam"></a>
One claim cannot be reached that way and has a seam of its own: that the composed
core is what the design admits, which runs through `prompt` rather than through a
driver spawned per claim. That module is `pub` on exactly that ground; the rule
that otherwise governs module visibility is under [main module
seams](#main-module-seams).

**`methodology` used to be the second such seam**, because assertions about the
real corpus had to reach an embed whose only production door was `include_dir` —
a crate a test cannot open without making a runtime dependency a dev one as well.
The corpus is ordinary files under `plugins/grove/skills/` now, so a test opens
it with `std::fs` and needs no seam at all.

What is mechanically checkable here is narrower than what the design claims, and
the boundary is stated rather than blurred: the core's three-part shape, its
ending's bytes, its two open couplings and the size alarm are checkable; whether
the wording actually gets a session to read the skill is not, and is carried by
[`wording-micro-test`](research/wording-micro-test.md) and by the human-watched
acceptance run.
