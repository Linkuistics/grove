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
rebuilding a binary, because nineteen kinds are a compiled enum, matched in some
places and spelled literally in others. Counting the `match` arms has undercounted
that surface twice; decision 5 enumerates it instead.

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
    /// Create the tree root, write its distinguished child, and place its first
    /// entries, under the lock already held. There is no window between deciding
    /// a tree is absent and creating it.
    ///
    /// `distinguished` is bytes and nothing else, because the distinguished
    /// child is the one entry a `NewEntry` cannot express: it carries no parts,
    /// its name is `N::distinguished()`, and the library already writes one this
    /// way when a promotion moves a leaf's bytes into a new node. `None` creates
    /// a root without one; `Some` in a domain whose `distinguished()` is `None`
    /// is the refusal a promotion gives for the same reason.
    pub fn initialize(
        self,
        distinguished: Option<Vec<u8>>,
        entries: Vec<NewEntry<N::Parts>>,
    ) -> Result<Report<N>, Error<N>>;
}

impl<N: EntryName> WriteGuard<N> {
    /// Remove the tree root and everything beneath it, following no symlink.
    pub fn delete(self) -> Result<Removed, Error<N>>;
}

/// What a root deletion removed: paths, in the order they went.
pub struct Removed { pub root: PathBuf, pub entries: Vec<PathBuf> }
```

A separate `exists` predicate would be a check-then-act split, and check-then-act
over a locked tree is the disease the consumer's current two-phase
classify-then-settle dance exists to paper over. One lock acquisition instead,
and the answer hands back the only operation that is valid for it: initializing
over a live tree and deleting a vacancy are not expressible. Something at the
root that is neither a tree nor nothing — a regular file, a symlink — is an
`Error` carrying what was found, not a third variant.

Neither operation widens the name seam. `initialize` takes bytes and a name the
trait already supplies, exactly as promotion does, so
[`entry-name-is-the-only-seam`](../adr/entry-name-is-the-only-seam.md) holds with
no new trait method — which matters, because without the distinguished input the
consumer would have to write the charter itself, outside the lock and outside the
store, and the whole *the store is the only thing that touches the task tree*
guarantee would fail at the first operation of every fresh grove.

**Deletion reports paths, where every other mutation reports names, and the
asymmetry is the operation's and not an oversight.** The existing report has a
created bucket and a renamed bucket, both keyed by `N`, because every other
mutation acts on entries the domain named. Deletion acts on the *root* and
therefore on everything beneath it — including the entries the domain
deliberately declines to parse as `N`, which the walk already skips and which the
report has no `N` to name. A third bucket of `N` would still be unable to say
what it removed. `Removed` is the honest postcondition: the paths that are gone,
which is exactly what a caller needs to say what it destroyed and is the whole of
what the operation knows.

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

**Two further sites carry a kind literal, and both are removed rather than
resolved.** Neither is a `match`, which is why counting `match` arms undercounts.

- **The research-pair verb** appends three flat siblings as one unit, and the
  three kinds it gives them are a constant in the machinery — a fourth token
  beside the two grove may name, naming three kinds grove has no business
  knowing. The atomicity is real and worth keeping: three separate appends are
  three snapshots and three chances to stop half way, and a live prefix of a pair
  is indistinguishable from a deliberately hand-cut partial one. So the verb is
  not deleted but **generalised** — the ordinary add takes an ordered list of
  kinds and appends them as one unit, at consecutive ordinals with consecutive
  keys. `leaf-add <parent> <stem> --kind research-a --kind research-b --kind
  combine-research` is then the pair, spelled by the methodology that owns those
  three tokens; a one-kind list is the ordinary add. Twelve verbs, not thirteen,
  and no list of kinds anywhere in the machinery.
- **The add and insert verbs default `--kind` to `impl`.** A default is a literal
  under a friendlier name, and it is the one kind literal that would silently
  produce a *wrong* leaf rather than an error. `--kind` becomes required on both.
  Root scaffolding keeps its default because `requirements` is one of the two
  leaves grove authors; a verb a session invokes has a session to name the kind.

The rule stands after both: grove names a kind only where grove writes the leaf,
and the two tokens are the whole of it.

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
their file.

**The overlay overrides and never supplies, and this is the per-kind restatement
of what completeness was for.** The all-nineteen rule is what made the second
source safe: a partial delta could only ever override a kind the complete
personal file already declared, so a project-supplied file could never introduce
a kind — and therefore a program to execute — that the operator had not already
chosen for themselves. Delete the quantifier and that argument goes with it,
because nothing else in the old rule says the primary must mention the key at
all. So it is restated one kind at a time: **a key resolves only if the primary
file declares it**; where the overlay also declares it, the overlay's template is
the one used, whole; where only the overlay declares it, the key does not
resolve, and the refusal names the key and the primary file that must declare it.
That is the same property the completeness rule bought, checked at the moment the
kind is used rather than over a set nobody can enumerate, and it is enforceable
without either source knowing what a kind is.

Both records move.
[`complete-session-configuration`](../adr/complete-session-configuration.md) is
amended for the quantifier, and
[`untracked-configuration-delta`](../adr/untracked-configuration-delta.md) is
amended for its own safety argument, which currently rests on the sentence *that
record's completeness rule still binds the personal file whatever a delta says*.
It does not, after this; the primary-declares rule is what binds instead, and the
record states that as its own property rather than borrowing one.

### 7 — The runner

```rust
/// The slot vocabulary a consumer's templates are written against. Supplied at
/// load, because every template rule is checked there.
pub struct Vocabulary<'a> { pub slots: &'a [SlotRule<'a>] }
pub struct SlotRule<'a> { pub name: &'a str, pub requirement: Requirement }
pub enum Requirement { ExactlyOnce, AtMostOnce }

pub struct Templates;

impl Templates {
    /// A key resolves from the primary file or the overlay, never from both, and
    /// only if the **primary** declares it: the overlay overrides and never
    /// supplies. Validates the whole of both documents against `vocabulary`.
    pub fn load(
        primary: &Path,
        overlay: Option<&Path>,
        vocabulary: Vocabulary<'_>,
    ) -> Result<Self, ConfigError>;
    /// The file this key's template was actually read from. `None` when the
    /// primary does not declare it, whatever the overlay says.
    pub fn source(&self, key: &str) -> Option<&Path>;
    pub fn expand(&self, key: &str, values: &[Slot<'_>]) -> Result<Argv, ConfigError>;
}

/// A value for one declared slot, at expansion. Substitution is whole-word: the
/// runner never learns what a name means, and never rewrites part of a word.
pub struct Slot<'a> { pub name: &'a str, pub value: &'a OsStr }

/// A program and its arguments, in order, ready to spawn. Built only by
/// expansion, so nothing reaches a spawn that a template did not author.
pub struct Argv { /* program, args */ }
impl Argv {
    pub fn program(&self) -> &OsStr;
    pub fn args(&self) -> &[OsString];
}

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
/// caller's to interpret, which is why the content is readable.
pub struct Token(String);
impl Token {
    pub fn as_str(&self) -> &str;
    pub fn into_string(self) -> String;
}
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

/// Both errors are opaque types implementing `Error + Display`. Their obligation
/// is the design's, not a variant list: every one names what is wrong, where —
/// file and location for a configuration error — and what fixes it.
pub struct ConfigError;
pub struct LaunchError;

pub mod conformance {
    /// Holds a consumer's configuration to the crate's own contract, so
    /// *reusable outside grove* is checked without a second repository.
    pub fn check(config: &Path, vocabulary: Vocabulary<'_>) -> Outcome;
    pub struct Outcome { pub failures: Vec<String> }
    impl Outcome { pub fn passed(&self) -> bool; }
}
```

The runner spawns the expanded argv directly, with no shell. The child's
environment is the caller's, minus the scrubbed control values, plus the fresh
channel path under the caller's chosen variable name. Escalation runs grace →
SIGTERM → kill-grace → SIGKILL, because a child that returns to an interactive
prompt is never reaped on its own.

**The vocabulary is an input to `load` and not to `expand`, and that is what
keeps decision 6's *document-eager* half true.** The template rules the current
implementation enforces are all rules about slot *names* — that a substitution is
a whole word and not embedded in one, that it names a declared slot, that the
required slot appears exactly once, that an optional one appears at most once.
None of them is checkable by a loader that will not learn the slot names until
expansion, so a vocabulary supplied per-call would make every one of them
just-in-time and reduce decision 6's amendment from *presence* to *everything*.
Supplied at load, the whole of both documents is checked before anything is
spawned, and expansion is left with one obligation: that the values offered fill
the slots the vocabulary declared.

### 8 — The VCS seam

```rust
pub struct Workspace;

impl Workspace {
    /// Refuses a working tree that is not jj-enabled, naming the command that
    /// fixes it. This is the precondition gate, not a dispatch.
    pub fn resolve(path: &Path) -> Result<Self, Refusal>;
    pub fn root(&self) -> &Path;
    pub fn main_repo(&self) -> &Path;
    /// A directory this workspace reserves for the named consumer's own
    /// untracked coordination files: inside the workspace, never tracked,
    /// never shared with another namespace, and created if absent. The
    /// consumer's filenames are its own and cannot collide with the version
    /// control system's, which is the whole of what the namespace buys.
    pub fn control_dir(&self, namespace: &str) -> Result<PathBuf, Refusal>;
    pub fn is_tracked(&self, path: &Path) -> Result<bool, Refusal>;
    /// Take a path-scoped commit and seal the working copy.
    pub fn commit(&self, paths: &[&Path], message: &str) -> Result<Commit, Refusal>;
}

pub struct Commit { pub change_id: String }
```

The namespace parameter is what makes the crate domain-free at this method
rather than only in the sentence claiming it is. The implementation being moved
reaches its answer by hard-coding a grove-named directory inside jj's
administrative one, and *where a lease file may live* is a postcondition that
cannot be stated without naming the consumer. Returning the administrative
directory raw would not fix it either — it would put the consumer's generic
filenames directly into a namespace the version control system owns and may
extend. Naming the consumer is what makes the guarantee sayable in the crate's
own vocabulary: this directory is yours, it is inside the workspace, and nothing
tracks it.

Grove takes commits and implements no transaction: no witness, no manifest, no
rollback proof, no index image, no quarantine, no recovery path. jj snapshots the
working copy before every command and its operation log is the transaction
record, so a failed teardown is recovered by `jj undo` — which is what the
refusal says. This supersedes `task-tree-transactions-fail-closed`
outright: not by the reopen condition that record names — a durable finish
receipt — but because the version control system owns the transaction. It also
retires `supported-workspace-layouts`, whose whole subject is the same-device
rename the quarantine needed.

Dropping the plain-git lane is what makes this true on every lane rather than
one. A non-jj working tree is refused before any mutation.

### 9 — The loop

```rust
/// Opening mirrors the store's, one level up, and for the same reason: a caller
/// cannot scaffold over a live grove or read one that is not there, because the
/// types do not offer it.
pub fn read(worktree: &Path)  -> Result<Reading, Error>;
pub fn write(worktree: &Path) -> Result<Writing, Error>;
pub enum Reading { Tree(Tree), Vacant }
pub enum Writing { Tree(TreeWrite), Vacancy(Vacancy) }

/// How a session names an existing entry: `.` for the root, a key, a handle, or
/// a path.
pub struct Reference(String);
impl Reference { pub fn parse(text: &str) -> Result<Self, Error>; }

pub struct Selection { pub path: PathBuf, pub handle: Handle, pub kind: Kind }

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

/// One error for the whole crate. Opaque, `Error + Display`, and under the same
/// obligation as the runner's: every one names what is wrong and what fixes it.
pub struct Error;

pub mod verbs {
    /// Scaffold a fresh grove: the charter brief and the first leaf. `kind`
    /// defaults to `requirements` at the CLI — one of the two leaves grove
    /// authors — and is the only kind default that survives anywhere.
    pub fn root_init(vacancy: Vacancy, slug: &Slug, kind: &Kind)
        -> Result<Initialized, Error>;
    pub struct Initialized { pub brief: PathBuf, pub first_leaf: PathBuf }

    /// The next leaf to work, or the fact that there is none — which is the
    /// finish trigger, and is `Sought` rather than an option of the loop's own
    /// invention.
    pub fn pick(tree: &Tree) -> Result<Sought<Selection>, Error>;

    /// The kind of a named leaf, or of the picked one when none is named.
    pub fn kind(tree: &Tree, leaf: Option<&Path>) -> Result<Sought<Kind>, Error>;

    /// Every `BRIEF.md` from the grove root down to the leaf, in that order.
    pub fn brief_chain(tree: &Tree, leaf: &Path) -> Result<Vec<PathBuf>, Error>;

    /// What a session's reference names. Ambiguity is an answer, not an error:
    /// the caller is a session that can re-ask with a narrower reference.
    pub fn resolve(tree: &Tree, reference: &Reference)
        -> Result<Sought<Resolution>, Error>;
    pub enum Resolution { Root, Entry(Located), Ambiguous(Vec<Located>) }
    pub struct Located { pub path: PathBuf, pub handle: Handle, pub kind: Option<Kind> }

    /// Append one or more leaves under `parent`, all carrying `slug`, as **one**
    /// unit: consecutive ordinals, consecutive keys, all of it or none of it.
    /// A one-kind list is the ordinary add; the research pair is a three-kind
    /// one, and the three tokens are the methodology's, not grove's.
    pub fn leaf_add(tree: &TreeWrite, parent: &Reference, slug: &Slug, kinds: &[Kind])
        -> Result<Vec<PathBuf>, Error>;

    /// Take `target`'s slot, shifting it and every later sibling up by one.
    pub fn leaf_insert(tree: &TreeWrite, target: &Reference, slug: &Slug, kind: &Kind)
        -> Result<Inserted, Error>;
    pub struct Inserted { pub path: PathBuf, pub renumbered: Vec<Renumber> }
    pub struct Renumber { pub from: PathBuf, pub to: PathBuf }

    /// Turn a leaf into a node, its bytes becoming the node's charter, with one
    /// first child. `kind` overrides the inherited kind rather than defaulting.
    pub fn leaf_decompose(
        tree: &TreeWrite,
        leaf: &Path,
        first_child: &Slug,
        kind: Option<&Kind>,
    ) -> Result<Decomposed, Error>;
    pub struct Decomposed { pub brief: PathBuf, pub first_child: PathBuf }

    /// Mark one leaf `DONE` in place. Filename only.
    pub fn leaf_retire(tree: &TreeWrite, leaf: &Path) -> Result<PathBuf, Error>;

    /// Mark abandoned work `ABANDONED` in place: one leaf, or every *live* leaf
    /// beneath one node. Filename only, and not atomic across a subtree.
    pub fn leaf_prune(tree: &TreeWrite, path: &Path) -> Result<Pruned, Error>;
    pub struct Pruned { pub marked: Vec<PathBuf>, pub left_done: Vec<PathBuf> }

    /// Commit the teardown the finish session performed. Reaches the VCS seam.
    pub fn finish_commit(workspace: &Workspace, finish: &Handle)
        -> Result<Commit, Error>;

    /// Write the relaunch flag to the signal file and return. Reaches the
    /// runner's channel. Outside a loop it is a no-op that says so.
    pub fn complete(signal_file: Option<&Path>, done: bool)
        -> Result<Signalled, Error>;
    pub enum Signalled { Wrote(PathBuf), NoLoop }
}
```

The verbs live here rather than with the store because ten of the twelve touch
the tree and every one is stated in grove's vocabulary — brief chains, kinds,
outcomes, handles, finishing — none of which the store has a word for.
Co-locating them gives the handle grammar one owner and puts the driver and the
verbs on one definition of a kind. The two that reach outward reach the runner
(`complete`) and the VCS seam (`finish-commit`).

Three shapes recur across the surface and are deliberate. A verb that reads takes
a `Tree` and one that writes takes a `TreeWrite`, so the lock a verb needs is
visible in its signature rather than acquired inside it. A search that matched
nothing answers `Sought`, the store's word, rather than an option each verb
re-interprets — that is the whole point of decision 2's fourth operation, and a
loop that reintroduced `Option` here would have moved the problem rather than
solved it. And every verb returns the paths it wrote, because its caller is a
session that has to name them in a commit message it writes by hand.

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

#### Scenario: a verb is asked to author several leaves at once
- **WHEN** an add names an ordered list of kinds
- **THEN** they land as one unit at consecutive ordinals with consecutive keys,
  or none of them lands, and no list of kinds appears in the machinery

### Requirement: a second configuration source overrides and never supplies
Launch policy SHALL resolve a key only when the primary file declares it, and an
overlay SHALL be able to replace such a key's template but never to introduce
one.

#### Scenario: a key only the overlay declares
- **WHEN** a project-supplied overlay declares kind K and the personal file does
  not
- **THEN** K does not resolve, and the refusal names K and the personal file that
  must declare it

#### Scenario: a malformed template for a kind this run will not reach
- **WHEN** any template in either source violates a rule of the slot vocabulary
- **THEN** it is refused at load, before any tree mutation and before any launch

### Requirement: no module implements a version-control guarantee
The VCS seam SHALL take commits and SHALL implement no transaction, witness,
rollback or recovery path.

#### Scenario: a teardown commit fails
- **WHEN** the finish commit does not complete
- **THEN** the refusal names the operation-log command that restores the working
  copy, and no grove-authored recovery runs

## ADR reconciliation

No record is rewritten by this spec, for the reason stated at the top: the set
describes the design's current state, and a record rewritten to describe unbuilt
code would make it lie. What follows is the target set and who lands each change,
so that decomposition can schedule the rework rather than discover it. **Every
record in `docs/adr/` is accounted for below**, because a record this design
makes false and nobody listed is a record that quietly stops being true.

| record | disposition | landed by |
|---|---|---|
| `task-tree-transactions-fail-closed` | **retired** — the VCS owns the transaction (decision 8), and the tree-access lock it also specifies goes with the store owning `initialize` and `delete` | the leaf that deletes the finish transaction |
| `supported-workspace-layouts` | **retired** — its whole subject is the same-device rename the quarantine needed | the same leaf |
| [`skill-delivers-the-methodology`](../adr/skill-delivers-the-methodology.md) | **retired** — the provisioned-skill delivery path ceases to exist | the leaf that deletes provisioning |
| [`one-build-owns-a-session`](../adr/one-build-owns-a-session.md) | **retired** — no build writes a skill directory, so there is no pairing to report | the same leaf |
| [`one-live-driver-per-working-tree`](../adr/one-live-driver-per-working-tree.md) | **reworked** — the lease survives; independent provisioning, the Git lane, the Git-or-jj control-directory derivation, the same-device gate and the Git-or-jj lost-result path do not. The control directory becomes the namespace the VCS seam hands back | the leaf that extracts the VCS seam |
| [`a-skill-states-what-binds-without-its-dependencies`](../adr/a-skill-states-what-binds-without-its-dependencies.md) | **reworked at `plugin-kind-skills-k17`** — it was `grove-binds-without-the-plugin`, opening on the binary sweeping its own `content/` into every harness's skill directory. The methodology *is* a plugin now, so the record's subject is what binds when a skill's own dependencies are absent | the leaf that ships the plugin |
| [`complete-session-configuration`](../adr/complete-session-configuration.md) | **amended** — the quantifier becomes per-kind and just-in-time (decision 6) | the leaf that lands the configuration change |
| [`untracked-configuration-delta`](../adr/untracked-configuration-delta.md) | **amended** — its safety argument currently borrows the completeness rule; it states the primary-declares rule as its own property instead (decision 6) | the same leaf |
| [`task-names-are-canonical`](../adr/task-names-are-canonical.md) | **amended** — the separator (decision 3); its migration clauses go with migration | the leaf that lands the grammar |
| [`entries-are-never-removed`](../adr/entries-are-never-removed.md) | **amended** — one clause distinguishing removing an *entry* from deleting the *root* (decision 2) | the leaf that lands `delete` |
| [`behavioural-coverage-asserts-delivery`](../adr/behavioural-coverage-asserts-delivery.md) | **amended** — the rule survives, the instrument moves to the plugin's shell runner, and two of the four things its walk covers no longer exist in the binary | the leaf that ships the plugin |
| [`corpus-rules-have-one-owner`](../adr/corpus-rules-have-one-owner.md) | **amended** — the filing rule survives unchanged; its register is the plugin's spine rather than an embedded `content/`, its reachability edge is the composed loaded path rather than a prompt module, and its all-nineteen mapping loses the set it quantified over | the same leaf |
| [`restatement-declares-its-class`](../adr/restatement-declares-its-class.md) | **amended** — the class distinction survives; the condition register relocates from the embedded corpus to the plugin spine | the same leaf |
| [`grove-does-not-stage-its-own-renames`](../adr/grove-does-not-stage-its-own-renames.md) | **amended** — the decision survives and gets simpler; its Git-lane consequences and its migration references go | the leaf that drops the Git lane |
| [`bulk-marks-are-not-atomic`](../adr/bulk-marks-are-not-atomic.md) | **re-checked, expected unchanged** — a subtree prune is still *N* rewrites under *N* guards. Its implementation pointer moves into the loop crate | the leaf that extracts the loop |
| [`entry-name-is-the-only-seam`](../adr/entry-name-is-the-only-seam.md) | **unchanged**, and more load-bearing. `initialize`'s distinguished input adds no trait method | — |
| [`grove-owns-escalated-review`](../adr/grove-owns-escalated-review.md) | **unchanged** — a methodology rule, whose text moves into the spine without its decision moving | — |
| **`jj-is-the-only-lane`** | **added** — dropping plain Git is hard to reverse, surprising without the safety principle behind it, and a real trade-off with a rejected alternative (narrowing the principle to *where the version control system can*, which keeps the finish transaction alive on one lane). Once this spec is rewritten to current state there is no other record saying why | the leaf that drops the Git lane |
| **`a-kind-is-an-open-token`** | **added** — the compiled enum going open is hard to reverse (it forces the filename grammar), surprising, and a real trade-off with two rejected alternatives (a kind manifest, and enumeration by reading the installed skill set); what it costs is a typo'd kind failing at `leaf-add` rather than at compile time | the leaf that opens `Kind` |

**This spec is not one of the retirements.** Its `## Problem` and its
*what changes* framing are transient, but what it describes — how the module
boundaries work — is a spec's own grain, and four artifacts already link into
this area. The leaf that lands the last decision rewrites it to current state
rather than deleting it. The two records added above are the decisions that carry
a trade-off of their own and would otherwise be recorded nowhere once that
rewrite drops the argument.

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
