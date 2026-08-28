# module-decomposition

## Problem

Grove is one crate of roughly 27,300 non-test lines behind two binaries, and the
bulk of it defends guarantees the version control system already provides. A
finish teardown hand-builds a durable pre-operation record, a proven rollback and
a crash-atomic multi-path mutation; a migration path does the same for a legacy
tree shape no live tree wears; a provisioning sweep writes skill directories and
then re-verifies its own writes; and roughly twenty-five auto-repair functions
exist to unwind machinery that would not need unwinding.

Three costs follow, and the third is the one that bites daily. The code cannot be
reused outside grove, because grove's vocabulary reaches everywhere. Nothing is
testable except through the binaries — forty-one integration files and 27,719
lines almost all drive a subprocess. And adding a session kind means editing and
rebuilding a binary, because nineteen kinds are a compiled enum matched in five
places.

## Solution

Five modules with independent lifetimes, three of them reusable outside grove,
and what remains of grove shrunk to the loop that composes them: a launcher that
owns a loop and a vocabulary, and nothing else.

The version control system owns safety, history and transactionality — jj
snapshots the working copy before every command and its operation log is the
transaction record — so grove **takes** commits and implements no transaction.
Anomalies stop with a message that names both what is wrong and how to fix it;
recovery machinery is not written where a sentence and a human will do. A name is
parsed and unparsed in exactly one type, the handle included. The skills drive
and grove is ambient: grove keeps only what a session physically cannot do for
itself — relaunch itself with fresh context, be killed under a sandbox, have its
vendor chosen before it exists, and be told to load the methodology.

Where this design *changes* a recorded decision rather than applying it, the
record is named below. No record is rewritten here:
[`docs/adr/`](../adr/) describes the design's current state, and a record
rewritten to describe unbuilt code would make the set lie. This spec carries the
target design until the leaf that lands each change reworks its record in place.

## Decisions

### 1 — Four library crates, two binary crates, one plugin

| module | package | domain-free |
|---|---|---|
| tree store | `ordinal-fs-tree` (unchanged) | yes |
| runner | `keyed-launch` | yes |
| VCS seam | `jj-workspace` | yes |
| loop | `grove-loop` | no |
| skills | the `grove` plugin | no |
| — | `grove`, `grove-llm` (binaries) | — |

One workspace, one release version, one changelog. A module is a crate so that
*testable through its own interface without the other four* stops being a
discipline held by review and becomes a fact the compiler enforces.

The skills module has **no crate**: its artifact is markdown that ships by an
entirely different path, and its half of that guarantee is met by its own
conformance runner instead.

The two binaries are separate crates rather than binary targets inside
`grove-loop`, for the same reason: a binary target can reach its own library's
private items, so *the binary is thin* stops being compiler-enforced the moment
it is a target rather than a crate.

`jj-workspace` is fully domain-free, not partly. Its whole surface is *resolve a
jj workspace, refuse a tree that is not one, take a path-scoped commit*, and the
remedy its refusal carries is jj's — `jj git init --colocate` — not grove's.

`keyed-launch` is named for its interface rather than its behaviour: the key is
what a consumer names, supervision is what sits behind it. Its vocabulary is
*key*, *template*, *launch*, *child*, *signal* and *escalation*, and it
deliberately avoids **session**, which would add a fourth row to the collision
table in [`CONTEXT-MAP.md`](../../CONTEXT-MAP.md).

### 2 — The tree store: what is added

Present and unchanged: the read and write guards, `append`, `append_many`,
`insert`, `promote`, `rewrite`, `Snapshot`, `Walk`, `Entry`, `Refusal`, and the
conformance kit that holds a consumer to the round-trip law. The name seam is
untouched — [`entry-name-is-the-only-seam`](../adr/entry-name-is-the-only-seam.md)
becomes more load-bearing under this design, not less.

Four operations are added, and `exists?` is added **as a shape rather than as a
predicate**:

```rust
pub fn read<N: EntryName>(root: &Path)  -> Result<Reading<N>, Error<N>>;
pub fn write<N: EntryName>(root: &Path) -> Result<Writing<N>, Error<N>>;

/// A tree opened under a shared lock, or the fact that there is none.
pub enum Reading<N> { Tree(ReadGuard<N>), Vacant }

/// A tree opened under an exclusive lock, or the vacancy where one would go.
pub enum Writing<N> { Tree(WriteGuard<N>), Vacancy(Vacancy<N>) }

impl<N: EntryName> Vacancy<N> {
    /// Create the tree root and place its first entries, under the lock already
    /// held. There is no window between deciding a tree is absent and creating it.
    pub fn initialize(self, entries: Vec<NewEntry<N::Parts>>) -> Result<Report<N>, Error<N>>;
}

impl<N: EntryName> WriteGuard<N> {
    /// Remove the tree root and everything beneath it, following no symlink.
    /// Reports every name it removed.
    pub fn delete(self) -> Result<Report<N>, Error<N>>;
}
```

A separate `exists` predicate would be a check-then-act split, and check-then-act
over a locked tree is the disease the consumer's current two-phase
classify-then-settle dance exists to paper over. One lock acquisition instead,
and the answer hands back the only operation that is valid for it: initializing
over a live tree and deleting a vacancy are not expressible. Something at the
root that is neither a tree nor nothing — a regular file, a symlink — is an
`Error` carrying what was found, not a third variant.

The fourth operation is a **word for a search that matched nothing**:

```rust
/// A search's answer. Not a refusal: nothing was asked to change, and nothing
/// is wrong with the tree.
pub enum Sought<T> { Match(T), Nothing }

impl<N: EntryName> Snapshot<N> {
    pub fn seek(&self, predicate: impl FnMut(&Entry<'_, N>) -> bool) -> Sought<Entry<'_, N>>;
    pub fn by_key(&self, key: Key) -> Sought<Entry<'_, N>>;
}
```

Every one of `Refusal`'s variants is a refusal to *mutate*. A store whose only
other negative answer is `None` forces each consumer to invent a word for
*found nothing* in its own vocabulary, which is exactly what the loop's current
optional-selection type is. `Sought` is that word, in the store's vocabulary, and
it replaces the whole optional search surface so there is one word for one
concept.

**This obliges one clause of
[`entries-are-never-removed`](../adr/entries-are-never-removed.md).** Its argument
is untouched — removing an *entry* lowers the visible key maximum and the next
allocation re-issues a live key — but its opening sentence says the library
offers no removal operation, and `delete` removes the **root**. The two are
different operations and only the second is on the table. The leaf that lands
`delete` adds the distinguishing clause.

`initialize` and `delete` also settle
[`grove-does-not-stage-its-own-renames`](../adr/grove-does-not-stage-its-own-renames.md)
and [`bulk-marks-are-not-atomic`](../adr/bulk-marks-are-not-atomic.md) against a
store that now owns root creation and deletion; both are to be re-checked, not
assumed.

### 3 — The filename grammar gains a separator

Once the session kind is an open token, today's grammar has no single parse.
`design-decomposition` in the middle of a name reads as kind `design` with slug
`decomposition` **and** as kind `design-decomposition` with an empty slug, and a
three-word kind makes the ambiguity four ways deep. Today only matching against
the closed set resolves it — the very thing decision 6 removes. The store's
canonicality obligation forbids two filenames naming one entry; this is one
filename naming two entries, which is worse, because what differs between the
readings is the **handle**, the identity that crosses every module boundary.

    NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md      a leaf
    NN-<slug>-k<key>                                    a node directory

The middle splits at the **first** `--`; neither the kind nor the slug may
contain one. Round-tripping holds, the permanent key stays the terminal token,
node names are untouched, and the kind token stays byte-identical to the skill
suffix — which is the property decision 6 exists for.

Rejected: spelling multi-word kinds with an inner underscore, because the
filename token and the skill name would then differ by a rule, reintroducing the
second source decision 6 deletes; moving the kind after the key, which unseats
the terminal-key rule that resolution and the glossary both lean on and only
relocates the delimiter problem; and forbidding hyphens in slugs, which renames
just as much for a worse read.

This obliges
[`task-names-are-canonical`](../adr/task-names-are-canonical.md), and it renames
every live leaf by one character. **A grove whose subject is the grove machinery
runs against the installed binary**, so the rename and the reinstall are one
step and no session may run between them.

### 4 — One type owns the name, and the handle renders through it

```rust
pub struct Slug(String);
pub struct Kind(String);
pub enum   Outcome { Live, Done, Abandoned }

/// The permanent, position-free identity of a work item.
pub struct Handle { /* slug, key */ }

impl Handle {
    /// The handle of a positioned name. `None` for the charter brief, which has
    /// no key.
    pub fn of(name: &TaskName) -> Option<Self>;
    pub fn parse(text: &str) -> Result<Self, HandleError>;
    pub fn slug(&self) -> &Slug;
    pub fn key(&self) -> Key;
}
impl fmt::Display for Handle;   // <slug>-k<key>

pub enum Parts    { Leaf { outcome: Outcome, kind: Kind, slug: Slug }, Node { slug: Slug } }
pub enum TaskName { Positioned { ordinal: Ordinal, key: Key, parts: Parts }, Brief }

impl EntryName for TaskName { type Parts = Parts; type Err = TaskNameError; }
```

The disciplinary form of *one type owns a name* is a rule review has to hold. The
structural form is not: **both renderings end in the handle's own rendering**, so
there is exactly one place the `<slug>-k<key>` grammar is spelled and drift
between the filename and the handle is not expressible. That is also what the
separator buys — it leaves the handle a contiguous terminal substring of every
name that has one.

`Kind` keeps its place in the parsed parts and its rendering, and loses only the
closed set. Its constructor validates the token's *shape* — non-empty, lowercase
ASCII letters, digits and single hyphens, no separator, not a reserved word — and
nothing else. What replaces an unknown-kind refusal is a shape refusal naming the
character it refused. Every name refusal continues to carry both what is on disk
and what it should be; that is the model the rest of this design's errors follow.

### 5 — Grove names a kind only where grove writes the leaf

Two tokens, and no manifest of kinds anywhere in the machinery.

The loop reads the tree once per iteration and mutates it only where no session
exists to delegate to: root scaffolding before the first session, and the finish
sentinel between the last ordinary session and the finish session. Those two
writes mint the only two leaves grove itself authors, and they are the only two
kinds it may name — `requirements` for the first, `finish` for the second. Every
other kind is an opaque string that grove substitutes into a skill name and a
configuration key and interprets in neither.

That rule also resolves the three surviving places a kind was matched: `finish`
sorting last in selection, `finish` being refused to the grow verbs, and
`requirements` being root-init's default. All three are grove recognising the
leaf it wrote itself, not grove interpreting the methodology. Root-init takes a
kind option, so the second token is a default rather than a constant.

The other two matched places do not survive: the mapping from kind to a
reference file, and the mapping from kind to a session-ending file, both go with
decision 6.

### 6 — Configuration completeness becomes per-kind and just-in-time

[`complete-session-configuration`](../adr/complete-session-configuration.md)
requires the personal configuration to declare all nineteen kinds, validated in
full before every tree mutation and every launch, and that completeness is what
makes a partial second source safe. Grove can no longer check it: it holds no set
of kinds, writes no skill directory and keeps no registry, so it cannot enumerate
what the methodology declares.

Only the **quantifier** moves. The whole document is still validated eagerly —
before every tree mutation and again before every launch — for syntax,
duplicates, node shape, and every template rule, so a malformed entry for a kind
this iteration will not reach still fails before anything is spawned. What
becomes just-in-time is *presence*: before writing a leaf of kind K, and before
launching kind K, K must resolve to exactly one complete template read whole out
of one file.

The record's load-bearing property is preserved — nothing is merged within a
kind, one author per launch, and a source that does not mention a kind cannot
supply it. What is lost is only the early warning *for a kind
not yet reached*: a stale personal configuration now fails at the first
`leaf-add` of that kind rather than at the next tree mutation of any kind. What is bought back is that adding a kind no
longer wedges every operation in every stale configuration until each owner edits
their file. The record is amended;
[`untracked-configuration-delta`](../adr/untracked-configuration-delta.md) is
untouched, because what a delta selects is still one complete string read whole
out of one file.

### 7 — The runner

```rust
pub struct Templates;

impl Templates {
    /// A key resolves from the primary file or the overlay, never from both.
    pub fn load(primary: &Path, overlay: Option<&Path>) -> Result<Self, ConfigError>;
    /// The file this key's template was actually read from.
    pub fn source(&self, key: &str) -> Option<&Path>;
    pub fn expand(&self, key: &str, slots: &[Slot<'_>]) -> Result<Argv, ConfigError>;
}

/// A whole-word substitution. The runner never learns what a name means; the
/// caller says which slot must appear exactly once.
pub struct Slot<'a> { pub name: &'a str, pub value: &'a OsStr, pub required: Requirement }
pub enum Requirement { ExactlyOnce, Optional }

/// The out-of-band completion signal: a fresh, collision-resistant path per
/// launch, naming that launch alone.
pub struct Channel;
impl Channel {
    pub fn allocate(dir: &Path) -> Result<Self, LaunchError>;
    pub fn path(&self) -> &Path;
    pub fn read(&self) -> Option<Token>;
    pub fn discard(self) -> Result<(), LaunchError>;
}
/// Opaque to the runner. Its appearance ends the launch; its content is the
/// caller's to interpret.
pub struct Token(String);
pub fn signal(path: &Path, token: &str) -> Result<(), LaunchError>;

pub struct Escalation { pub grace: Duration, pub kill_grace: Duration }

pub struct Launch<'a> {
    pub argv: &'a Argv,
    pub channel: &'a Channel,
    pub channel_var: &'a str,
    pub scrub: &'a [&'a OsStr],
    pub escalation: Escalation,
}

pub fn run(launch: Launch<'_>) -> Result<Ended, LaunchError>;

pub struct Ended { pub end: End, pub status: ExitStatus, pub elapsed: Duration, pub token: Option<Token> }
pub enum End { Exited, Signalled, Interrupted }

pub mod conformance { pub fn check(config: &Path) -> Report; }
```

The runner spawns the expanded argv directly, with no shell. The child's
environment is the caller's, minus the scrubbed control values, plus the fresh
channel path under the caller's chosen variable name. Escalation runs grace →
SIGTERM → kill-grace → SIGKILL, because a child that returns to an interactive
prompt is never reaped on its own.

### 8 — The VCS seam

```rust
pub struct Workspace;

impl Workspace {
    /// Refuses a working tree that is not jj-enabled, naming the command that
    /// fixes it. This is the precondition gate, not a dispatch.
    pub fn resolve(path: &Path) -> Result<Self, Refusal>;
    pub fn root(&self) -> &Path;
    pub fn main_repo(&self) -> &Path;
    /// Where a lease file may live: untracked, and inside this workspace.
    pub fn control_dir(&self) -> &Path;
    pub fn is_tracked(&self, path: &Path) -> Result<bool, Refusal>;
    /// Take a path-scoped commit and seal the working copy.
    pub fn commit(&self, paths: &[&Path], message: &str) -> Result<Commit, Refusal>;
}

pub struct Commit { pub change_id: String }
```

Grove takes commits and implements no transaction: no witness, no manifest, no
rollback proof, no index image, no quarantine, no recovery path. jj snapshots the
working copy before every command and its operation log is the transaction
record, so a failed teardown is recovered by `jj undo` — which is what the
refusal says. This supersedes
[`task-tree-transactions-fail-closed`](../adr/task-tree-transactions-fail-closed.md)
outright: not by the reopen condition that record names — a durable finish
receipt — but because the version control system owns the transaction. It also
retires [`supported-workspace-layouts`](../adr/supported-workspace-layouts.md),
whose whole subject is the same-device rename the quarantine needed.

Dropping the plain-git lane is what makes this true on every lane rather than
one. A non-jj working tree is refused before any mutation.

### 9 — The loop

```rust
pub fn open(worktree: &Path) -> Result<Opened, Error>;
pub enum Opened { Tree(Tree), Vacancy(Vacancy) }

pub struct Selection { pub path: PathBuf, pub handle: Handle, pub kind: Kind }
pub fn select(tree: &Tree) -> Result<Sought<Selection>, Error>;

pub struct DriverLease;
impl DriverLease {
    pub fn acquire(workspace: &Workspace) -> Result<Self, Error>;
    pub fn worktree_root(&self) -> &Path;
    pub fn revalidate(&self) -> Result<(), Error>;
}

pub struct Mandate<'a> {
    pub handle: &'a Handle,
    pub kind: &'a Kind,
    pub workspace: &'a Workspace,
    pub version: &'a str,
}
pub fn compose(mandate: &Mandate<'_>) -> String;

pub fn run(workspace: &Workspace, lease: DriverLease, templates: &Templates)
    -> Result<LoopOutcome, Error>;
pub enum LoopOutcome { Finished, Stopped }

pub mod verbs {
    // The thirteen a session invokes: root_init, pick, brief_chain, kind,
    // resolve, leaf_add, leaf_add_pair, leaf_insert, leaf_decompose,
    // leaf_retire, leaf_prune, finish_commit, complete.
}
```

The verbs live here rather than with the store because eleven of the thirteen
touch the tree and every one is stated in grove's vocabulary — brief chains,
kinds, outcomes, handles, finishing — none of which the store has a word for.
Co-locating them gives the handle grammar one owner and puts the driver and the
verbs on one definition of a kind. The two that reach outward reach the runner
(`complete`) and the VCS seam (`finish-commit`).

The prompt is three driver-authored parts and carries no methodology: an
imperative naming `grove-<kind>`; the runtime facts — the selected handle, the
stated version control, and grove's published version; and grove's own signalling
contract. Its first part reproduces the element measured as load-bearing in
[the wording micro-test](../research/wording-micro-test.md) — one imperative
naming one target, so the session performs no selection and has nothing to defer.
The list of provisioned skill directories is dropped with provisioning, and the
gap is recorded there rather than argued away: a harness with a skill-loading
affordance is unaffected, one without loses its fallback, and the reopen
condition is a session that cannot reach the methodology by the affordance alone.

### 10 — Grove publishes its version in the prompt

The compatibility check inverts: the machinery states what it is, and the
methodology decides whether that is good enough and what to do when it is not.

The published value is the workspace's single release version, and it rides in
the prompt's runtime facts beside the handle and the stated version control. A
verb would need the CLI on `PATH` and would fire only if the session thought to
run it, which is the deferred read the micro-test measured; a value in the prompt
needs no command to succeed and cannot fail. The version-flag output of the
verb binary remains as a fallback, not as the mechanism.

The methodology's content hash, the build-pairing report and the content-hash
flag go with this. A release version orders and means something to a human, which
a content hash never did. This retires
[`one-build-owns-a-session`](../adr/one-build-owns-a-session.md) — there is no
build pairing once no build writes a skill directory — and
[`skill-delivers-the-methodology`](../adr/skill-delivers-the-methodology.md),
whose delivery path ceases to exist.

**The cost is the one that record existed to prevent:** grove no longer
guarantees the methodology is present, so a session can be launched pointing at a
skill that is not installed. That is a message, not machinery — grove states the
version it is and names the install command, and stops.

### 11 — The methodology ships as a plugin, and how fat each skill is

Grove writes no skill directory. The methodology installs the way this repo's
other skill plugins already do — a marketplace entry, and a symlink farm
elsewhere, where a skill declares its own harness eligibility rather than a
registry deciding for it. A kind exists **iff** a skill of that name exists.

The plugin ships one `grove-<kind>` skill per kind over a shared `grove` spine.
The fatness rule:

- **Inline in `grove-<kind>`**: every rule owned by that kind or its family — its
  goal, its deliverable, its human-in-the-loop mark, its review allowance, and
  whether it passes the done flag when it signals.
- **In the shared spine**: every rule shared across families — the seven
  constraints, the bootstrap, execution, decomposition, retirement and commit
  procedures, and the four format documents.
- **Nowhere twice.**
  [`corpus-rules-have-one-owner`](../adr/corpus-rules-have-one-owner.md) and
  [`restatement-declares-its-class`](../adr/restatement-declares-its-class.md)
  bind unchanged and are what make this checkable.

Where a rule belongs to a *family* rather than one kind — the five reviews, the
two research halves — the family's text is one file in the spine, and each
member's skill directs a load of it by name in its opening imperative. A directed
load is not a selection.

**One gap, recorded rather than claimed closed.** The micro-test measured one
hop, from a prompt naming two targets. Nothing measures the second hop, from
`grove-<kind>` to the spine. What is inline is unaffected; what is in the spine
loses its guarantee; the reopen condition is a session observed acting without a
spine rule.

## Requirements

### Requirement: a leaf filename has exactly one reading
The name parser SHALL yield at most one `(kind, slug)` split for any filename,
and SHALL render that split back to the byte-identical filename.

#### Scenario: a multi-word kind beside a multi-word slug
- **WHEN** a leaf is named with kind `integrate-review-design` and slug
  `module-decomposition`
- **THEN** parsing yields exactly that kind and that slug, and rendering them
  reproduces the filename

#### Scenario: a name without the separator
- **WHEN** a task-shaped filename carries no `--` between kind and slug
- **THEN** it is refused, and the refusal names both what is on disk and the
  canonical form

### Requirement: grove names only the kinds it writes
The machinery SHALL contain no enumeration of session kinds, and SHALL reference
a kind label literally only for the two leaves it authors itself.

#### Scenario: an unknown kind reaches the loop
- **WHEN** a leaf carries a kind for which no skill is installed
- **THEN** the tree parses, the launch proceeds, and the failure is reported by
  the session that could not load the skill

#### Scenario: a kind missing from the configuration
- **WHEN** a leaf of kind K is added and K resolves to no template
- **THEN** the add is refused before the tree is mutated, naming K and the file
  that should declare it

### Requirement: no module implements a version-control guarantee
The VCS seam SHALL take commits and SHALL implement no transaction, witness,
rollback or recovery path.

#### Scenario: a teardown commit fails
- **WHEN** the finish commit does not complete
- **THEN** the refusal names the operation-log command that restores the working
  copy, and no grove-authored recovery runs

## Test seams

Four, replacing a suite that today has effectively one.

1. **Each crate's public interface**, exercised without the other three. This is
   the primary seam and the done-when made mechanical; the pattern already exists
   for the tree store.
2. **One composed-loop seam** — the loop driving a fake harness binary end to
   end. Today's driver, completion and lease suites, much shrunk.
3. **Conformance kits as the cross-crate seam.** The store already ships one that
   holds a consumer to the round-trip law; the runner ships the equivalent for a
   template configuration. This is what keeps *reusable outside grove* true
   without a second repository, and it is why extraction can stay deferred
   without weakening the claim.
4. **The methodology's delivery assertion, in the plugin.**
   [`behavioural-coverage-asserts-delivery`](../adr/behavioural-coverage-asserts-delivery.md)'s
   rule survives and its instrument moves. Two of the four things its walk covers
   no longer exist in the binary, so the assertion cannot run in the Rust suite at
   all. The plugin ships a dependency-free shell conformance runner — the shape
   the skills context already uses to test its own installer — asserting that
   every behavioural rule is present on the composed loaded path of every kind
   that binds it, that no rule has two owners, and that every file a skill names
   by path exists. It asserts nothing about how many kinds there are.

## Out of scope

- **Migration.** Deleted rather than preserved: no legacy tree needs it. A legacy
  tree now fails on its names, through a refusal that already carries what is on
  disk and what it should be.
- **The plain-git lane.** Dropped. Narrowing the safety principle to *where the
  version control system can* would have kept the finish transaction alive on one
  lane and left the VCS seam the largest of the five modules.
- **Extracting the tree store to its own repository.** Deferred; its documents
  stay where four artifacts already link to them. The manifest exclusion that
  kept it out of the release cut is removed, because one release process answers
  *is this crate published on its own* deliberately rather than by accident.
- **Serving the methodology over MCP.** Rejected: it would not remove
  provisioning but change what is provisioned, and it appears nowhere in this
  repo today.
- **A harness registry row for a further harness.** The question is answered by
  deletion — there is no registry left to hold a row.
- **Invoking a harness plugin.** A command template expresses this today; if more
  is meant it is a new runner capability, belonging to the runner's contract and
  not to any registry.
