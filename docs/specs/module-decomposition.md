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
headers and tests across the shipped packages cite them as *decision N*. There
is no decision 6: it was Grove's launch configuration, which is deleted, and the
number is not reused. What each one *cost* is in [`docs/adr/`](../adr/), which describes the design's current
state and is cited here rather than restated.

## Decisions

### 1 — Four library crates, two binary crates, one plugin

| module | package | domain-free |
|---|---|---|
| tree store | `ordinal-fs-tree` | yes |
| runner, optional synchronous spawn/reap notifications | `keyed-launch` | yes |
| VCS seam | `jj-workspace` | yes |
| loop | `grove-loop` | no |
| read-only viewer | `grove-tui` | no |
| skills | the `grove` plugin | no |
| — | `grove`, `grove-llm` (binaries) | — |

The domain-bound viewer consumes the loop's quiet `try_observe` capture and
canonical `entry_path` helper. The loop captures validated names and selected-file
bytes, retaining an opaque `TreeLifetime` after releasing the tree guard through
`ReadGuard::into_snapshot`. Capture calls `select_snapshot` for whole-tree
duplicate-key and live-finish validation. The viewer owns application state and
terminal rendering; its Ratatui/Crossterm dependencies do not enter grove-loop
or grove-llm. The human
binary dispatches `view` before lifecycle setup. The loop owns the shared
selection rule over an already-read snapshot, including optional permanent-key
exclusion; the viewer does not reimplement it. See [Read-only viewer](../ARCHITECTURE.md#read-only-viewer).
The [item-status design](./item-status.md) defines the combined typed
tree/activity observation interface and full-width interaction. Runtime protocol
knowledge belongs to the loop; row styling and viewport state belong to the
viewer. The shipped capture returns independent tree and activity results. Its private
`driver_lease::observation` module reuses mandatory parsers and identity logic,
with read-only bounded acquisition and a runtime-only shared epoch guard.
The lease publishes a versioned mandate extension and exact Started marker via
its private paired-witness owner; mandatory admission does not depend on those
observational fields. The reader probes the captured directory before the private
witness and returns typed Running with a same-tree, previous-tree or no-readable-
tree relation. A successful final private probe establishes Idle despite marker
bytes or earlier directory errors. Unsupported active records remain Unavailable;
missing controls or matching inactive records are Idle. The viewer compares tree and activity consistency separately,
consuming consistent tree results even when activity changes or fails, and
preserving compared activity when tree reading or root opening fails. Accepted
Idle displays the shared selector's NEXT key in rows and persistent chrome;
verified same-tree Running binds the current row by key and excludes that key
before finish eligibility. Busy/Unavailable or failed tree acceptance clears old row activity and withholds
NEXT. Tree failure leaves the independent RUNNING summary available. The loop owns selection, while the viewer owns freshness and
label/key-preserving summary layout. The typed witnessed observer ships;
the viewer displays same-tree RUNNING with independent lifecycle colors and
key-preserving summaries. Running without a current matching item remains
unavailable until the exceptional-tree summary increment.

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

`keyed-launch` keeps the name it took when it also resolved a key to a command
template. What is left is the supervision that sat behind the key. Its
vocabulary is *argv*, *launch*, *child*, *channel*, *signal* and *escalation*,
and it deliberately avoids **session**, which would add a fourth row to the collision
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
pub fn try_read<N: EntryName>(root: &Path) -> Result<TryReading<N>, Error<N>>;
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
other kind is an opaque string that grove substitutes into a skill name and
passes to `harness-dispatch` as `--kind`, and interprets in neither.

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

### 7 — The runner

The runner takes a program and its arguments from its caller and launches them
as a job it supervises to the end. It reads no configuration and resolves no
name to a command, and it understands none of Grove's or dispatch's paths,
kinds or records. Its two callers build the argv themselves: harness-dispatch
launches the command its owner's policy selected, and Grove launches
`harness-dispatch run`, for a lifecycle session and for `grove run` alike
([harness selection and execution](harness-selection-and-execution.md#grove-integration)).

```rust
/// A program and its arguments, in order, ready to spawn: each string one
/// whole word, with no shell and no second reading.
pub struct Argv { /* program, args */ }
impl Argv {
    pub fn new(program: OsString, args: Vec<OsString>) -> Self;
    pub fn program(&self) -> &OsStr;
    pub fn args(&self) -> &[OsString];
    /// The whole launch as one word list, program first — the shape a
    /// `Command`-building consumer and a diagnostic both want.
    pub fn words(&self) -> Vec<OsString>;
}

/// The out-of-band exit channel: a fresh, collision-resistant path per
/// launch, naming that launch alone. **Its appearance is the whole signal**:
/// it carries nothing, and nothing reads its content.
pub struct Channel;
impl Channel {
    pub fn allocate(dir: &Path) -> Result<Self, LaunchError>;
    pub fn path(&self) -> &Path;
    pub fn discard(self) -> Result<(), LaunchError>;
}
/// Send the exit signal: create the file at `path`. One that exists already
/// is success.
pub fn signal(path: &Path) -> Result<(), LaunchError>;

pub struct Escalation { pub grace: Duration, pub kill_grace: Duration }

pub struct Launch<'a> {
    pub argv: &'a Argv,
    /// The exit channel and the variable its path is published under. A launch
    /// with none ends only when its child exits or its launcher is cancelled.
    pub channel: Option<(&'a Channel, &'a str)>,
    pub scrub: &'a [&'a OsStr],
    /// Values set after the scrub: the caller's own control variables.
    pub grant: &'a [(&'a OsStr, &'a OsStr)],
    /// The child's working directory. `None` inherits the launcher's, which is
    /// rarely what a launcher wants: it is wherever a human happened to be
    /// standing.
    pub cwd: Option<&'a Path>,
    pub escalation: Escalation,
}

pub fn run(launch: Launch<'_>) -> Result<Ended, LaunchError>;
/// As `run`, reporting successful spawn and confirmed reap to the caller.
pub enum LaunchEvent { Started, Reaped }
pub fn run_observed(launch: Launch<'_>, observer: &mut dyn FnMut(LaunchEvent))
    -> Result<Ended, LaunchError>;
/// A child in a new session, with no terminal or inherited input, whose output
/// goes to a caller-owned regular file.
pub fn run_noninteractive(launch: Launch<'_>, output: File) -> Result<Ended, LaunchError>;
/// A child in a new session with no terminal and null stdin, writing to the
/// launcher's own output, under mandatory filesystem confinement. The program
/// is an absolute path, and any other is refused.
pub struct Confinement<'a> { pub writable: &'a [PathBuf], pub runtime_read: &'a [PathBuf] }
pub fn run_confined(launch: Launch<'_>, policy: &Confinement<'_>)
    -> Result<Ended, LaunchError>;
/// Open one regular file in a directory held before untrusted work ran.
pub fn regular_file_at(directory: &File, name: &OsStr) -> std::io::Result<File>;

/// `signalled` is whether the exit channel existed once the child was reaped,
/// so a child that signals and exits at once has still signalled.
pub struct Ended { pub end: End, pub status: ExitStatus, pub elapsed: Duration, pub signalled: bool }
/// `Interrupted` is the *launcher's* own process signalled during this launch.
/// **The signal is carried rather than merely noted**, because a process that
/// catches a termination signal, tidies up and exits 0 has told its parent it
/// finished its work; the only way to say what actually happened is to die of
/// the same signal, and that needs its number. `reraise` is that ending, and
/// this field is its argument. `Escalated` is a child the escalation ended.
pub enum End { Exited, Escalated, Interrupted { signal: i32 } }

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

/// Opaque, implementing `Error + Display`. Its obligation is the design's, not
/// a variant list: it names what is wrong, where, and what fixes it.
pub struct LaunchError;
```

The block states the surface this decision settles and is not an inventory of
it: how a caller asks for its child to receive the caller's own entry signal
state, below, is the implementation's to shape.

The runner spawns the argv directly, with no shell. The child's environment is
the caller's, minus the scrubbed names, plus the granted values and, with a
channel, its path under the caller's chosen variable name. The child is a job
in a process group of its own; an interactive launch hands it the terminal and
takes the terminal back with the attributes it saved restored
([`the-launched-child-is-a-job`](../adr/the-launched-child-is-a-job.md)). Its
terminal-generated signal dispositions are the defaults, unless its caller is a
transparent wrapper that passes on its own entry signal state instead, as
dispatch does; and the runner installs no handler over a disposition its
launcher ignores. With a channel, the channel's appearance starts grace →
SIGTERM → kill-grace → SIGKILL, because a child that returns to an interactive
prompt is never reaped on its own; it is addressed to the child's **process
group**, so a command the child itself launched is reaped with it. The
launcher's own TERM or HUP cancels the launch, as does INT for a launch with no
terminal: an interactive or noninteractive child's group is sent the same
signal and, after the kill-grace, SIGKILL; a confined child's group is killed at
once, so that a cancellation nested inside another supervisor's grace finishes
inside it.

Dispatch launches its harness interactively or confined, with a channel and its
own constant graces. Grove launches dispatch interactively for a lifecycle
session and noninteractively for `grove run`, with no channel and a kill-grace
longer than dispatch's, because dispatch is itself a supervisor that needs that
long to end its harness. A confined launch is specified in
[harness selection and execution](harness-selection-and-execution.md#confinement).

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
    /// Read-only discovery at the exact location, without ancestor search or jj.
    /// Absence is None; the returned path does not pin filesystem identity.
    pub fn discover_control_dir(location: &Path, namespace: &str) -> Result<Option<PathBuf>, Refusal>;
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

pub fn run(workspace: &Workspace, lease: DriverLease, dispatch: &Path)
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

    /// Record in this launch's directory that the grove was torn down, once
    /// `.grove/` is gone, and return. Outside a loop it is a no-op that says so.
    pub fn record_teardown(worktree: &Path, launch_dir: Option<&Path>)
        -> Result<Recorded, Error>;
    pub enum Recorded { Wrote(PathBuf), NoLoop }
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
`verbs` ships a public function beside the twelve — `stale_cross_refs`, which
its own header calls *not a thirteenth verb* because it is the second half of
`leaf-insert`'s contract. It is not a verb, which is why it is not declared
here.

The verbs live here rather than with the store because ten of the twelve touch the
tree and every one is stated in grove's vocabulary — brief chains, kinds,
outcomes, handles, finishing — none of which the store has a word for.
Co-locating them gives the handle grammar one owner and puts the driver and the
verbs on one definition of a kind. One reaches outward, to the VCS seam
(`finish-commit`); `record-teardown` writes only into the launch directory the
driver allocated.

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
The prompt does not enumerate skill directories. The human binary provisions
bundled Codex-compatible skills before entering the loop; other harnesses use
their own installation route. A session still needs its harness's skill-loading
affordance, and independently authored kinds need separately installed skills.

**The signalling contract's own gap.** One contract for every kind replaces two
per-kind signal files whose split existed so that a `finish` prompt never carried
a bare *send the exit signal* — the ending that, taken alone by the one session
that may have just deleted the task tree, relaunches the loop onto a torn-down
grove, and whose stated precondition a completed teardown satisfies exactly.
The contract answers that by making the kind's own ending the sentence's object
and the ordinary verb, `harness-dispatch exit`, subordinate to it, so no prompt
ends on a bare imperative for the wrong action. A teardown is a record of its
own, written by `grove-llm record-teardown` before that verb, and the loop
finishes on the record rather than on what the exit signal carries, which is
nothing ([dispatch supervises the harness](../adr/dispatch-supervises-the-harness.md)).
What is not answered is the compound with decision 10's accepted residue: a
`finish` session whose `grove-finish` skill is missing or unread meets the
ordinary default and nothing contradicting it, where the old prompt alone was
fail-safe for that kind whatever was installed. The reopen condition is a
`finish` session observed sending the exit signal after a teardown it did not
record.

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

**The remaining delivery limit:** bare startup verifies the bundled Codex
snapshot, but does not inspect Claude marketplace state, validate independently
authored skills or pair every `grove-llm` on PATH with that snapshot. The prompt
states the driver version; each harness must still load its named skill.

### 11 — The methodology ships as a plugin, and how fat each skill is

The plugin tree is the authoritative methodology source. Claude Code uses the
marketplace with auto-update; bare `grove` installs its embedded snapshot of
Codex-compatible skills before launch. The manual installer remains available
for checkout delivery. Each skill declares its own harness eligibility. See
[codex-skill-provisioning](codex-skill-provisioning.md) for the binary adapter's
ownership and repair contract. A kind exists **iff** a skill of that name exists.

The plugin ships one `grove-<kind>` skill per kind over a shared `grove` spine.
The fatness rule:

- **Inline in `grove-<kind>`**: every rule owned by that kind or its family — its
  goal, its deliverable, its human-in-the-loop mark, its review allowance, and
  whether it records a teardown before its exit signal.
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

#### Scenario: a kind no launch policy routes
- **WHEN** a leaf of kind K is added and the owner's dispatch policy refuses K
- **THEN** the add succeeds, because no tree verb consults a policy; the
  refusal arrives when the leaf launches, and the leaf stays live

#### Scenario: a verb is asked to author several leaves at once
- **WHEN** an add names an ordered list of kinds
- **THEN** they land as one unit at consecutive ordinals with consecutive keys,
  or none of them lands, and no list of kinds appears in the machinery

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
   The driver, teardown and lease suites.
3. **A conformance kit as the cross-crate seam.** The store ships one that holds a
   consumer to the round-trip law. This is what keeps *reusable outside grove* true without a
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
- **Invoking a harness plugin.** The command a dispatch policy returns expresses
  this; if more is meant it is a new runner capability, belonging to decision
  7's contract and not to any registry.
