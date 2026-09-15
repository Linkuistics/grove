# module-decomposition

## Solution

Grove is **five modules with independent lifetimes**, three of them reusable
outside grove, and the launcher that composes them: a loop and a vocabulary, and
nothing else.

The version control system owns safety, history and transactionality — jj
snapshots the working copy before every command and its operation log is the
transaction record — so grove **takes** commits and implements no transaction.
Anomalies stop with a message that names both what is wrong and how to fix it;
recovery machinery is not written where a sentence and a human will do. A name is
parsed and unparsed in exactly one type, the handle included. The skills drive
and grove is ambient: grove keeps only what a session physically cannot do for
itself — relaunch itself with fresh context, be killed under a sandbox, have its
vendor chosen before it exists, and be told to load the methodology.

This document describes how the module boundaries work. The decisions below are
numbered, and the numbering is load-bearing: source comments, `Cargo.toml`
headers and tests across the shipped packages cite them as *decision N*. What each
one *cost* is in [`docs/adr/`](../adr/), which describes the design's current
state and is cited here rather than restated.

## Decisions

### 1 — Four library crates, two binary crates, one plugin

| module | package | domain-free |
|---|---|---|
| tree store | `ordinal-fs-tree` | yes |
| runner | `keyed-launch` | yes |
| VCS seam | `jj-workspace` | yes |
| loop | `grove-loop` | no |
| read-only viewer | `grove-tui` | no |
| skills | the `grove` plugin | no |
| — | `grove`, `grove-llm` (binaries) | — |

The domain-bound viewer consumes the loop's public typed reader and canonical
`entry_path` helper. It owns application state and terminal rendering; its
Ratatui/Crossterm dependencies do not enter grove-loop or grove-llm. The human
binary dispatches `view` before lifecycle setup. The observation path is not a
launch-policy selector. See [Read-only viewer](../ARCHITECTURE.md#read-only-viewer).

One workspace, one release version, one changelog, one tag. A module is a crate
so that *testable through its own interface without unrelated modules* is not a
discipline held by review but a fact the compiler enforces: the three domain-free
crates take no path dependency on the domain crates, and each carries its own
suites.

The skills module has **no crate**: its artifact is markdown that ships by an
entirely different path, and its half of that guarantee is met by its own
conformance runner instead.

The two binaries are separate crates rather than binary targets inside
`grove-loop`, for the same reason: a binary target declared beside the library
can compile the library's modules into itself — a `#[path]` attribute and a
`mod` — and then name the items that library keeps private, so *the binary is
thin* would stop being compiler-enforced for that shape.

It is only that shape, and two measurements bound it. A target that merely
depends on the library beside it sees what a separate crate sees: a `pub(crate)`
item named through the library's path is refused with `E0603`. And the modules
such a target compiles into itself land in **its own** crate rather than the
library's, so what it reaches is a second instance of those items rather than
the library's own — a type declared that way is distinct from the library's,
`E0308`, and rustc names it *defined in the current crate*. What the shape
defeats is therefore the source-level discipline, which is the only one *the
binary is thin* was ever about.

What a separate crate adds is not that the shape becomes impossible — a `#[path]`
can point outside a package, and three test targets in this workspace do exactly
that — but that it has to, and a reader of the entry point's own two files can
see whether it does. The boundary makes the move visible rather than
unavailable. A crate boundary is also a
reachability boundary, which is what lets `dead_code` report an item whose only
callers are another package's tests.

`jj-workspace` is fully domain-free, not partly. Its whole surface is *resolve a
jj workspace, refuse a tree that is not one, take a path-scoped commit*, and the
remedy its refusal carries is jj's — `jj git init --colocate` — not grove's.

`keyed-launch` is named for its interface rather than its behaviour: the key is
what a consumer names, supervision is what sits behind it. Its vocabulary is
*key*, *template*, *launch*, *child*, *signal* and *escalation*, and it
deliberately avoids **session**, which would add a fourth row to the collision
table in [`CONTEXT-MAP.md`](../../CONTEXT-MAP.md).

### 2 — The tree store's surface

The read and write guards, `append`, `append_many`, `insert`, `promote`,
`rewrite`, `Snapshot`, `Walk`, `Entry`, `Refusal`, and the conformance kit that
holds a consumer to the round-trip law. The name seam is one trait —
[`entry-name-is-the-only-seam`](../adr/entry-name-is-the-only-seam.md) is more
load-bearing under this design than before it, not less.

`exists?` is a **shape rather than a predicate**:

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
    /// The consumer supplies the root file's name and bytes together.
    /// `None` requests a root without its own file, only where the domain's
    /// level rule permits that. An empty byte vector still creates a file.
    pub fn initialize(
        self,
        distinguished: Option<(N, Vec<u8>)>,
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
over a locked tree is the disease a two-phase classify-then-settle dance exists to
paper over. One lock acquisition instead, and the answer hands back the only
operation that is valid for it: initializing over a live tree and deleting a
vacancy are not expressible. Something at the root that is neither a tree nor
nothing — a regular file, a symlink — is an `Error` carrying what was found, not a
third variant.

Both operations stay behind the name seam. `initialize` takes bytes and a name
the consumer supplies, exactly as promotion takes its destination name, which matters: without the
distinguished input the consumer would have to write the charter itself, outside
the lock and outside the store, and the whole *the store is the only thing that
touches the task tree* guarantee would fail at the first operation of every fresh
grove.

**Deletion reports paths, where every other mutation reports names, and the
asymmetry is the operation's and not an oversight.** The report has a created
bucket and a renamed bucket, both keyed by `N`, because every other mutation acts
on entries the domain named. Deletion acts on the *root* and therefore on
everything beneath it — including the entries the domain deliberately declines to
parse as `N`, which the walk skips and which the report has no `N` to name. A
third bucket of `N` would still be unable to say what it removed. `Removed` is
the honest postcondition: the paths that are gone, which is exactly what a caller
needs to say what it destroyed and is the whole of what the operation knows.

A search that matched nothing has a **word of its own**:

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
other negative answer were `None` would force each consumer to invent a word for
*found nothing* in its own vocabulary. `Sought` is that word, in the store's
vocabulary, and it is the whole optional search surface, so there is one word for
one concept.

Removing an **entry** and deleting the **root** are different operations and only
the second exists
([`entries-are-never-removed`](../adr/entries-are-never-removed.md), whose
distinguishing clause is what keeps its argument about key allocation intact).
Root creation and destruction are both the store's
([`root-lifecycle-belongs-to-the-store`](../adr/root-lifecycle-belongs-to-the-store.md));
they leave a repository-aware mutation path just as ruled out as before
([`grove-does-not-stage-its-own-renames`](../adr/grove-does-not-stage-its-own-renames.md)),
and they leave a subtree prune exactly as non-atomic as it was
([`bulk-marks-are-not-atomic`](../adr/bulk-marks-are-not-atomic.md)).

### 3 — The filename grammar separates children from the node's own file

[`task-names-are-canonical`](../adr/task-names-are-canonical.md) owns the
grammar, token rules, node-file cardinality and placement, and strict refusal
policy. This separation lets a directory move without repeating its title in
every descendant path. The root's display title comes from the working tree.

The whole reachable tree is validated before the snapshot can answer any
selection, even one that finds an early match. The reader attaches the level
path to grammar errors, including competing names when present. No selection
can bypass validation of a later subtree. The kind remains an open token whose
spelling is byte-identical to its skill suffix.

### 4 — The name module owns handles composed from names

Grove's positioned parts have two shapes: leaf parts carry outcome, kind and
slug; node parts carry none of those fields. A parsed node file carries a slug,
and the root node file is its own variant. The library sees only the positioned
triple or distinguished species and never reads a slug.

The name module owns `Slug`, `Kind`, `Outcome`, `TaskName` and `Handle`, including
all parsing and rendering. A leaf handle is constructed from its parsed slug
and key. A node handle is constructed from the directory's parsed key and its
validated node file's parsed slug. These are two explicit constructors:
`Handle::of_leaf` accepts a leaf name, and `Handle::of_node` accepts a node name
and its node-file name. Invalid species pairings return no handle. Neither the
root nor a node file alone has a work-item handle.

The tree module supplies the node's actual file from the same guarded read; the
name module cannot establish parentage from two name values. It never caches
that slug into node parts or reconstructs it by parsing a path string. A leaf
renderer uses the handle renderer for its terminal `<slug>-k<key>` substring;
a directory renderer emits only position and key. A node handle need not be a
substring of either filename. Handle parsing and rendering remain one grammar
in the name module, and the key token has one parser and renderer shared with
directory names.

`resolve` looks for leaf slugs in leaf names and node slugs in their node files,
returning the directory for a node. Node files themselves are not extra resolve
matches. Key lookup still names the positioned entry; a full handle checks its
slug against that entry's current title. An ambiguous slug reports the existing
candidate handles. `pick` returns only live leaves, `kind` has no kind for a
node or node file, and `brief-chain` prints every ancestor node file root-first;
missing files are refusals, not skipped levels.

`leaf-decompose` passes slugless node parts and `_<slug>.md` built from the
source leaf's slug to `promote`. The leaf's ordinal and key become the node's;
its bytes move verbatim to that file. `root-init` supplies `_BRIEF.md` and the
root brief bytes to initialization alongside the first leaf. No writer creates
a node file outside the library's exclusive guard and plan. After promotion,
Grove reacquires the exclusive guard and appends ` — brief` to the canonical
handle heading. That content edit is idempotent, leaves custom headings alone,
and belongs to Grove rather than the content-blind store. If it fails, the
promotion remains committed and the error identifies the file to repair.

A missing-node-file refusal can precede handle lookup. Its diagnostic includes
the conditional interrupted-decompose recovery advice specified by
[`task-names-are-canonical`](../adr/task-names-are-canonical.md). The caller
does not inspect an invalid tree after the failed open or add a second reader.

The library's `validate_distinguished` method remains on `EntryName`. It sees
root-or-node and the complete set of distinguished names, including an empty
or competing set; Grove returns its own grammar error for wrong cardinality or
placement. The library also enforces at most one independently. Readers invoke
this before exposing a level; planners invoke it on projected final levels
before effects. This includes ordinary node creation through append, batches
and insert, optional promotion children, initialization entries and node
rewrites. A required node file cannot be bypassed by another constructor.

### 5 — Grove names a kind only where grove writes the leaf

Two tokens, and no manifest of kinds anywhere in the machinery
([`a-kind-is-an-open-token`](../adr/a-kind-is-an-open-token.md)).

The loop reads the tree once per iteration and mutates it only where no session
exists to delegate to: root scaffolding before the first session, and the finish
sentinel between the last ordinary session and the finish session. Those two
writes mint the only two leaves grove itself authors, and they are the only two
kinds it may name — `requirements` for the first, `finish` for the second. Every
other kind is an opaque string that grove substitutes into a skill name and a
configuration key and interprets in neither.

That rule also covers the places a kind is asked about: `finish` sorting last in
selection, `finish` being refused to the grow verbs, and teardown. All of them go
through one predicate rather than carrying a token, and all of them are grove
recognising the leaf it wrote itself. `root-init` writes a `requirements` leaf and
takes no kind option; the rule holds either way, since grove authors that leaf.

**No verb carries a list of kinds, and neither does a default.** The ordinary add
takes an *ordered list* of kinds and appends them as one unit, at consecutive
ordinals with consecutive keys, so `leaf-add <parent> <stem> --kind research-a
--kind research-b --kind combine-research` is the research pair, spelled by the
methodology that owns those three tokens, and a one-kind list is the ordinary
add. Twelve verbs, not thirteen. `--kind` is **required** on the add and insert
verbs: a default is a literal under a friendlier name, and `impl` was the one kind
literal that would silently produce a *wrong* leaf rather than an error.

### 6 — Configuration completeness is per-kind and just-in-time

The whole of the personal document — and of any second source — is validated
eagerly, before every tree mutation and again before every launch, for syntax,
duplicates, node shape, and every template rule, so a malformed entry for a kind
this iteration will not reach fails before anything is spawned. What is asked at
the moment of use is *presence*: before writing a leaf of kind K, and before
launching kind K, K must resolve to exactly one complete template read whole out
of one file.

The quantifier is per-kind because grove can no longer state a set: it holds no
set of kinds, writes no skill directory and keeps no registry, so it cannot
enumerate what the methodology declares.

**The overlay overrides and never supplies**, one kind at a time: a key resolves
only if the primary file declares it; where the overlay also declares it, the
overlay's template is the one used, whole; where only the overlay declares it, the
key does not resolve, and the refusal names the key and the primary file that must
declare it. That is what stands between a file a project could hand you and a
program its operator never chose, and it is enforceable without either source
knowing what a kind is
([`complete-session-configuration`](../adr/complete-session-configuration.md) for
what a template must be,
[`untracked-configuration-delta`](../adr/untracked-configuration-delta.md) for the
second document and the safety property it now states as its own).

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
    /// Does this key resolve to exactly one complete template? The obligation a
    /// consumer discharges *before* it commits to a key — before it writes down
    /// work of that kind, or launches it — stated once, here, so the refusal's
    /// wording has one owner. `expand` asks the same question on its own way in.
    pub fn require(&self, key: &str) -> Result<(), ConfigError>;
    pub fn expand(&self, key: &str, values: &[Slot<'_>]) -> Result<Argv, ConfigError>;
    /// The keys the primary document declares, in name order. The conformance
    /// kit's one window into a loaded configuration — enough to say *this
    /// checked nothing*, and nothing more.
    pub fn keys(&self) -> Vec<&str>;
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
    /// The whole launch as one word list, program first — the shape a
    /// `Command`-building consumer and a diagnostic both want.
    pub fn words(&self) -> Vec<OsString>;
}

/// The out-of-band completion signal: a fresh, collision-resistant path per
/// launch, naming that launch alone.
pub struct Channel;
impl Channel {
    pub fn allocate(dir: &Path) -> Result<Self, LaunchError>;
    pub fn path(&self) -> &Path;
    pub fn read(&self) -> Option<Token>;
    pub fn discard(self) -> Result<(), LaunchError>;
    /// Remove every channel file in `dir` — the ones a previous launcher
    /// allocated and did not live to discard. **The name grammar is this
    /// crate's, so recognising an abandoned channel has to be too**: the
    /// alternative is a consumer open-coding the name in its own cleanup.
    pub fn discard_abandoned(dir: &Path) -> Result<(), LaunchError>;
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
    /// The child's working directory. `None` inherits the launcher's, which is
    /// rarely what a launcher wants: it is wherever a human happened to be
    /// standing.
    pub cwd: Option<&'a Path>,
    pub escalation: Escalation,
}

pub fn run(launch: Launch<'_>) -> Result<Ended, LaunchError>;

pub struct Ended { pub end: End, pub status: ExitStatus, pub elapsed: Duration, pub token: Option<Token> }
/// `Interrupted` is the *launcher's* own process signalled during this launch.
/// **The signal is carried rather than merely noted**, because a process that
/// catches a termination signal, tidies up and exits 0 has told its parent it
/// finished its work; the only way to say what actually happened is to die of
/// the same signal, and that needs its number. `reraise` is that ending, and
/// this field is its argument.
pub enum End { Exited, Signalled, Interrupted { signal: i32 } }

/// Which signal, if any, was sent to this process *outside* a launch — clearing
/// the latch. A signal arriving between two launches has no launch to be
/// reported against, and `run` discards it rather than spending it on the next
/// child, which has signalled nothing and done nothing wrong; a looping
/// launcher calls this at the top of its loop to honour it instead.
pub fn take_interrupt() -> Option<i32>;
/// Die of the signal that ended this launcher, so **its** parent sees the
/// conventional `128 + N` in the wait status — which an exit *code* cannot
/// express at all. **This crate owns the call because this crate installed the
/// handler**; what stays the consumer's is *whether* to re-raise.
pub fn reraise(signal: i32) -> !;

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
prompt is never reaped on its own; it is addressed to the child's **process
group**, so a command the session itself launched is reaped with it, and the
runner hands the terminal to the child and takes it back
([`the-launched-child-is-a-job`](../adr/the-launched-child-is-a-job.md)).

**The vocabulary is an input to `load` and not to `expand`, and that is what keeps
decision 6's *document-eager* half true.** Every template rule the runner enforces
is a rule about slot *names* — that a substitution is a whole word and not
embedded in one, that it names a declared slot, that a required slot appears
exactly once, that an optional one appears at most once. None is checkable by a
loader that will not learn the slot names until expansion, so a vocabulary
supplied per call would make every one of them just-in-time and reduce decision 6
from *presence* to *everything*. Supplied at load, the whole of both documents is
checked before anything is spawned, and expansion is left with one obligation:
that the values offered fill the slots the vocabulary declared.

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

The namespace parameter is what makes the crate domain-free at this method rather
than only in the sentence claiming it is. *Where a lease file may live* is a
postcondition that cannot be stated without naming the consumer, and returning the
administrative directory raw would put the consumer's generic filenames directly
into a namespace the version control system owns and may extend. Naming the
consumer makes the guarantee sayable in the crate's own vocabulary: this directory
is yours, it is inside the workspace, and nothing tracks it.

Grove takes commits and implements no transaction: no witness, no manifest, no
rollback proof, no index image, no quarantine, no recovery path. jj snapshots the
working copy before every command and its operation log is the transaction record,
so a failed teardown is recovered by the operation-log command the refusal names.
Every child that speaks to the version control system is spawned inside this
crate, which removes the ambient repository selectors from each one, so choosing
the right repository is the seam's guarantee and no call site can be written
without it.

**jj is the only lane** ([`jj-is-the-only-lane`](../adr/jj-is-the-only-lane.md)),
and that is what makes the paragraph above true on every lane rather than one. A
non-jj working tree is refused before any mutation, by this one gate; nothing
downstream branches on which version control owns the tree, because nothing else
can own it.

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

pub fn run(workspace: &Workspace, lease: DriverLease, templates: &TemplateSource)
    -> Result<LoopOutcome, Error>;

/// The loop's terminal disposition, so a clean whole-grove finish, a
/// non-signalled stop, and the driver itself being signalled away are three
/// answers rather than one. `Interrupted` carries the signal number, because a
/// caller that mapped it to a clean exit would tell its own parent that a grove
/// finished.
pub enum LoopOutcome { Finished, Stopped, Interrupted(i32) }

/// One error for the whole crate. Opaque, `Error + Display`, and under the same
/// obligation as the runner's: every one names what is wrong and what fixes it.
pub struct Error;

pub mod verbs {
    /// Scaffold a fresh grove: the charter brief and the first leaf. `kind` is
    /// `requirements` at the CLI — one of the two leaves grove authors — and is
    /// the only kind default that survives anywhere.
    pub fn root_init(vacancy: Vacancy, slug: &Slug, kind: &Kind)
        -> Result<Initialized, Error>;
    pub struct Initialized { pub brief: PathBuf, pub first_leaf: PathBuf }

    /// The next leaf to work, or the fact that there is none — which is the
    /// finish trigger, and is `Sought` rather than an option of the loop's own
    /// invention.
    pub fn pick(tree: &Tree) -> Result<Sought<Selection>, Error>;

    /// The kind of a named leaf, or of the picked one when none is named.
    pub fn kind(tree: &Tree, leaf: Option<&Path>) -> Result<Sought<Kind>, Error>;

    /// Every ancestor node file from the grove root down to the leaf, in order.
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
    pub struct Renumber {
        pub from: PathBuf,
        pub to: PathBuf,
        /// The positions either side of the shift, because what a report of a
        /// renumber reads as is the ordinals, not the paths.
        pub from_position: u32,
        pub to_position: u32,
    }

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

**One declaration above is deliberately short of the shipped type, and stays
short.** `Located` ships a fourth field, `outcome: Outcome` — live, `DONE` or
`ABANDONED` — so that `resolve` cannot let a retired or abandoned dead end look
live. It is left out of the listing above on purpose: `crates/grove-loop/src/task_tree.rs`
carries the field's own justification in the form *not in this record's listing
of this struct, and deliberately added back*, and a listing that absorbed the
field would falsify the sentence that explains it. The record states the surface
this decision settled; the field is the one place the code answers a question the
decision did not ask, and it is recorded here rather than reconciled away.

**The block is a statement of the surface, not an inventory of it.** Items it
declares are held to the shipped signature; items it omits are not thereby
denied. `Reference` ships `root`, `is_root` and `as_str` beside `parse`, and
`verbs` ships two public functions beside the twelve — `stale_cross_refs`,
which its own header calls *not a thirteenth verb* because it is the second half
of `leaf-insert`'s contract, and `signal_channel`, which is public only because
a caller has to ask which channel it is about to signal *before* `complete`
writes to it. Neither is a verb, which is why neither is declared here.

The verbs live here rather than with the store because ten of the twelve touch the
tree and every one is stated in grove's vocabulary — brief chains, kinds,
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

`TreeWrite` is a caller's **right to be the writer**, not one guard: it hands out
the guard it opened with before reopening for the next verb, and relinquishes it
before a second opening is taken, so no verb holds two
([`bulk-marks-are-not-atomic`](../adr/bulk-marks-are-not-atomic.md) carries the
window that leaves open, and why re-running the verb is the repair).

The prompt is three driver-authored parts and carries no methodology: an
imperative naming `grove-<kind>`; the runtime facts — the selected handle, the
stated version control, and grove's published version; and grove's own signalling
contract. Its first part reproduces the element measured as load-bearing in
[the wording micro-test](../research/wording-micro-test.md) — one imperative
naming one target, so the session performs no selection and has nothing to defer.
There is no list of provisioned skill directories, and the gap that leaves is
recorded rather than argued away: a harness with a skill-loading affordance is
unaffected, one without has no fallback, and the reopen condition is a session
that cannot reach the methodology by the affordance alone.

**The signalling contract's own gap.** One contract for every kind replaces two
per-kind signal files whose split existed so that a `finish` prompt never carried
*run `grove-llm complete`* — the ending that, taken by the one session that may
have just deleted the task tree, relaunches the loop onto a torn-down grove, and
whose stated precondition a completed teardown satisfies exactly. The contract
answers that by making the kind's own ending the sentence's object and the
ordinary verb subordinate to it, so no prompt ends on a bare imperative for the
wrong action. What is not answered is the compound with decision 10's accepted
residue: a `finish` session whose `grove-finish` skill is missing or unread meets
the ordinary default and nothing contradicting it, where the old prompt alone was
fail-safe for that kind whatever was installed. The reopen condition is a `finish`
session observed signalling `complete` after a teardown.

### 10 — Grove publishes its version in the prompt

The machinery states what it is, and the methodology decides whether that is good
enough and what to do when it is not.

The published value is the workspace's single release version, and it rides in
the prompt's runtime facts beside the handle and the stated version control. A
verb would need the CLI on `PATH` and would fire only if the session thought to
run it, which is the deferred read the micro-test measured; a value in the prompt
needs no command to succeed and cannot fail. The version-flag output of the verb
binary remains as a fallback, not as the mechanism. There is no methodology
content hash and no build-pairing report: a release version orders and means
something to a human, which a content hash never did.

**The cost is the one a build-pairing report existed to prevent:** grove does not
guarantee the methodology is present, so a session can be launched pointing at a
skill that is not installed. That is a message, not machinery — grove states the
version it is and names the install route, and stops.

### 11 — The methodology ships as a plugin, and how fat each skill is

Grove writes no skill directory. The methodology installs the way this repo's
other skill plugins do — a marketplace entry, and a symlink farm elsewhere, where
a skill declares its own harness eligibility rather than a registry deciding for
it. A kind exists **iff** a skill of that name exists.

The plugin ships one `grove-<kind>` skill per kind over a shared `grove` spine.
The fatness rule:

- **Inline in `grove-<kind>`**: every rule owned by that kind or its family — its
  goal, its deliverable, its human-in-the-loop mark, its review allowance, and
  whether it passes the done flag when it signals.
- **In the shared spine**: every rule shared across families — the seven
  constraints, the bootstrap, execution, decomposition, retirement and commit
  procedures, and the format documents.
- **Nowhere twice.**
  [`corpus-rules-have-one-owner`](../adr/corpus-rules-have-one-owner.md) and
  [`restatement-declares-its-class`](../adr/restatement-declares-its-class.md)
  bind unchanged and are what make this checkable.

Where a rule belongs to a *family* rather than one kind — the five reviews, the
five integrations, the two research halves — the family's text is one file in the
spine, and each member's skill directs a load of it by name in its opening
imperative. A directed load is not a selection. A skill that cites a skill from
another plugin states what binds in its absence
([`a-skill-states-what-binds-without-its-dependencies`](../adr/a-skill-states-what-binds-without-its-dependencies.md)).

**One gap, recorded rather than claimed closed.** The micro-test measured one hop,
from a prompt naming two targets. Nothing measures the second hop, from
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

### Requirement: every level has exactly one correctly placed node file

Every Grove reader SHALL refuse a reached level with zero or multiple node
files, or with the root marker at a positioned node or a title at the root.
Every mutation SHALL validate the reachable tree and its projected result before
its first effect.

#### Scenario: competing files beside an early live leaf
- **WHEN** a level holds `_alpha.md` and `_beta.md` beside a selectable leaf
- **THEN** selection refuses the level and names both files and `_<slug>.md`.

#### Scenario: missing root or child file
- **WHEN** a root lacks `_BRIEF.md` or a positioned node lacks `_<slug>.md`
- **THEN** the read refuses with the required form and never reports completion.

#### Scenario: a decomposed task retains its title and key
- **WHEN** `02-design--pilot-k12.md` decomposes
- **THEN** `02-k12/_pilot.md` carries its body and the node resolves as
  `pilot-k12`, from names even if the body's heading says something else.

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

## Test seams

Four.

1. **Each crate's public interface**, exercised without the other three. This is
   the primary seam and the done-when made mechanical: every crate carries its own
   suites, and the three domain-free ones compile and run with none of the rest of
   the workspace on their dependency list.
2. **One composed-loop seam** — the loop driving a fake harness binary end to end.
   The driver, completion and lease suites.
3. **Conformance kits as the cross-crate seam.** The store ships one that holds a
   consumer to the round-trip law; the runner ships the equivalent for a template
   configuration. This is what keeps *reusable outside grove* true without a
   second repository, and it is why extraction can stay deferred without weakening
   the claim.
4. **The methodology's delivery assertion, in the plugin.** A dependency-free
   shell conformance runner over the files a harness installs asserts that every
   behavioural rule is present on the composed loaded path of every kind that
   binds it, that no rule has two owners, and that every file a skill names by
   path exists. It asserts nothing about how many kinds there are, and it cannot
   run in the Rust suite at all, because two of the four things its walk used to
   cover do not exist in the binary
   ([`behavioural-coverage-asserts-delivery`](../adr/behavioural-coverage-asserts-delivery.md)).

The node grammar uses the existing model, conformance, name-unit and CLI
fixture seams. Conformance includes distinct distinguished names and contextual
level verdicts. Name tests exercise canonical parsing and node-handle
composition; CLI trees exercise decomposition, root initialization, slug and
handle resolution, brief chains, selection and malformed levels. Book checks
cover every source root changed by an implementation, with fragments and prose
updated in the same accepted change. No additional test service is introduced.

## Out of scope

- **Migration.** There is none: no legacy tree needs it, and a legacy tree fails
  on its names, through a refusal that carries what is on disk and what it should
  be.
- **A plain-git lane.** There is none
  ([`jj-is-the-only-lane`](../adr/jj-is-the-only-lane.md)). Narrowing the safety
  principle to *where the version control system can* would have kept a finish
  transaction alive on one lane and left the VCS seam the largest of the five
  modules.
- **Extracting the tree store to its own repository.** Deferred; its documents
  stay where four artifacts already link to them. What is **not** deferred is the
  question that exclusion was standing in for: `ordinal-fs-tree` is not published
  on its own. One workspace, one release version, one changelog, one tag — the
  crate ships inside grove's cut and wears grove's version, and no library member
  has a release lane of its own ([`RELEASING.md`](../RELEASING.md) carries the
  answer and what a second lane would cost).
- **Serving the methodology over MCP.** Rejected: it would not remove delivery
  machinery, only change what is served, and it puts a running server between a
  session and prose it can read off disk.
- **A harness registry row for a further harness.** Answered by deletion — there
  is no registry to hold a row. A row was only ever *a place to write files*, so a
  further harness is answered by that harness's own skill-install route.
- **Invoking a harness plugin.** A command template expresses this; if more is
  meant it is a new runner capability, belonging to decision 7's contract and not
  to any registry.
