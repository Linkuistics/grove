# Orientation
<!-- book-page id="orientation" slice="no-dependencies" order="1" -->
[Contents](README.md) | [Next: The gate](02-the-gate.md)

<a id="what-it-declines"></a>
## A crate defined by what it declines

`jj-workspace` resolves a Jujutsu workspace, refuses a working tree that is not
one, and takes a path-scoped commit. That is the whole crate: four operations,
two values, and one error type, over roughly seven hundred and fifty lines with
no dependencies.

The first of those refusals is this chapter's, and it is declared rather than
argued: **no dependencies**. `std` owns what was subtracted — it spawns the child
process, it reads the directory, and it supplies the `Error` trait the crate's one
error type implements — so nothing else is taken, and nothing is imposed on a
consumer that takes this crate. The manifest is where that refusal is written
down, and it is read first below.

A crate that small is normally explained by listing what it does. This book
explains it by listing what it does **not** do, because that is where its
decisions are. Version control is a domain with a large surface — transactions,
rollback, repository discovery, error taxonomies, working-copy state — and every
one of those is already owned by the tool underneath. Each chapter of this book
opens on something this crate declines to own and names who owns it instead, and
the last chapter turns the six refusals into a test a reader can apply to a
boundary of their own.

The crate has exactly one consumer, and it is not in this crate. grove's loop
driver reserves a control namespace through it and grove's task sessions commit
through it. That consumer is named on every page, because the argument that the
crate has no vocabulary for its consumer is only checkable if you can watch the
consumer's name being passed **in** as an ordinary string. To keep the boundary
visible while doing that, every passage that speaks for the consumer rather than
for the crate is marked, like this:

> **The consumer's half.** grove calls `control_dir("grove")` once per loop and
> keeps a lock file and a signal file in the directory it gets back. The crate
> cannot say any of that: it does not know what a loop is, and `"grove"` reaches
> it as four characters with no meaning attached.

A sentence inside such a block is one the crate could not have written. Every
other sentence in this book is about the crate itself.

<a id="package-contract"></a>
## The manifest, read as the first refusal

The crate's manifest is production source and this chapter reconstructs all
forty-four lines of it. It is read first because the first refusal is declared
there rather than argued anywhere else: the `[dependencies]` table is empty, and
that emptiness is the deliverable.

<!-- fragment «manifest-no-dependencies» owner="no-dependencies" source="crates/jj-workspace/Cargo.toml" lines="1-44" parent="source-crate-manifest" -->
<!-- insert «manifest-package-identity» -->
<!-- insert «manifest-empty-dependencies» -->
<!-- insert «manifest-dev-dependency» -->
<!-- insert «manifest-lints» -->
<!-- insert «manifest-release-lane» -->
<!-- /fragment -->

The package block identifies the crate and inherits five fields from the
workspace root rather than restating them. `rust-version` is the interesting
one, and its comment carries the reason it is inherited rather than lowered: the
crate itself would build on a much older toolchain, but a member that promises
more than the workspace can deliver promises nothing. The input here is a
`cargo` version and the output is a refusal to build; the invariant is that one
workspace states one floor.

<!-- fragment «manifest-package-identity» owner="no-dependencies" source="crates/jj-workspace/Cargo.toml" lines="1-12" parent="manifest-no-dependencies" -->
````toml
[package]
name = "jj-workspace"
version.workspace = true
edition.workspace = true
description = "Resolve a Jujutsu workspace, refuse a working tree that is not one, and take a path-scoped commit"
license.workspace = true
repository.workspace = true
# Inherited from the workspace root, whose comment carries the evidence: the
# locked dependency graph cannot be parsed by a cargo below 1.85. This crate has
# no dependencies of its own and would build far lower, but a member that
# promises more than the workspace can deliver promises nothing.
rust-version.workspace = true
````
<!-- /fragment -->

Here is the first refusal, and it is an empty table. The crate spawns a child
process and reads a directory, and `std` does both, so nothing else is taken.
The consequence is not the crate's own build time — it is that a dependency this
crate takes is a dependency it **imposes** on grove and on everything grove is
linked into. The comment names the one place that could have leaked: errors.
An error-handling crate in the public signature of `Refusal` would make every
consumer's error strategy partly this crate's decision, so `Refusal` is the
crate's own type implementing `std::error::Error` and nothing more, which is the
same rule `crates/ordinal-fs-tree` states for `EntryName::Err`.

<!-- fragment «manifest-empty-dependencies» owner="no-dependencies" source="crates/jj-workspace/Cargo.toml" lines="13-19" parent="manifest-no-dependencies" -->
````toml

# **No dependencies, and that is the deliverable rather than an accident.** The
# crate spawns `jj` and reads a directory; `std` does both. Errors are its own
# opaque `Refusal` implementing `std::error::Error`, so a consumer using
# `anyhow`, `thiserror` or nothing at all takes on nothing here — the same rule
# `crates/ordinal-fs-tree` states for `EntryName::Err`.
[dependencies]
````
<!-- /fragment -->

An empty `[dependencies]` table with a populated `[dev-dependencies]` table is
the distinction the refusal actually rests on, so the two are read together. The
interface tests need a real jj repository per case — created, used and removed —
and `tempfile` supplies exactly that. A dev-dependency is not inherited by a
consumer, so the imposed set stays empty while the tests stay honest. The
alternative was hand-rolling temporary-directory creation and cleanup, which is
a worse version of a crate this workspace already locks.

<!-- fragment «manifest-dev-dependency» owner="no-dependencies" source="crates/jj-workspace/Cargo.toml" lines="20-26" parent="manifest-no-dependencies" -->
````toml

# `tempfile` is a *dev*-dependency, so no consumer inherits it. The interface
# tests need a real jj repository per case, unique per run and removed
# afterwards, and hand-rolling that is a worse version of a crate this workspace
# already locks.
[dev-dependencies]
tempfile = "3.10"
````
<!-- /fragment -->

The lint configuration is inherited for the same reason `rust-version` is: one
workspace, one standard. There is nothing crate-specific to say about it, and
that is itself the point — a crate with no dependencies has no dependency-shaped
lint exceptions to declare.

<!-- fragment «manifest-lints» owner="no-dependencies" source="crates/jj-workspace/Cargo.toml" lines="27-29" parent="manifest-no-dependencies" -->
````toml

[lints]
workspace = true
````
<!-- /fragment -->

The last block is an **answered** question rather than a deferred one, and it is
worth reading closely because `release = false` looks like a placeholder and is
not. `cargo release` cuts `crates/grove`, and this line means this crate takes no
tag, no changelog section and no publish of its own. It does not freeze the
version: `version.workspace = true` above means a cut moves this crate with every
other crate the release ships. The comment states the consequence of deleting the
line — it corrupts the cut rather than reopening the question — because that is
the fact a future reader needs and the fact the code cannot show.

<!-- fragment «manifest-release-lane» owner="no-dependencies" source="crates/jj-workspace/Cargo.toml" lines="30-44" parent="manifest-no-dependencies" -->
````toml

# `cargo release` cuts *grove* (`crates/grove`), and `release.toml` configures
# that cut. `release = false` here means no tag, no changelog section and no
# publish of its own. It does **not** mean a frozen version: this crate takes
# `version.workspace = true`, so a cut moves it with every other crate the
# release ships — one workspace, one release version
# (`docs/specs/module-decomposition.md`, decision 1).
#
# **This crate is not published on its own, and that is settled**
# (`docs/RELEASING.md`, *One release, six packages, one tag*): it ships inside
# grove's cut, wearing grove's version, and no library member has a release lane
# of its own. Removing this line does not reopen the question — it corrupts the
# cut, which was measured rather than assumed.
[package.metadata.release]
release = false
````
<!-- /fragment -->

> **The consumer's half.** What this means for a reader of grove's changelog is
> that `jj-workspace` never appears in it. There is one release, six packages and
> one tag, and this crate ships inside grove's cut wearing grove's version.

<a id="crate-thesis"></a>
## The crate's own account of itself

The library's module documentation is a list of refusals with headings, and this
chapter owns all of it. Reading it here rather than paraphrasing it is deliberate:
three of the six refusals are stated in these forty-five lines, and later chapters
argue them at the sites that create them.

<!-- fragment «library-crate-thesis» owner="no-dependencies" source="crates/jj-workspace/src/lib.rs" lines="1-54" parent="source-library" -->
<!-- insert «library-purpose-sentence» -->
<!-- insert «library-thesis-no-consumer» -->
<!-- insert «library-thesis-no-transactions» -->
<!-- insert «library-thesis-reads-add-no-history» -->
<!-- insert «library-module-surface» -->
<!-- /fragment -->

The opening is the crate's purpose in one sentence, followed by the boundary that
sentence implies. There is no repository abstraction and no second lane behind it,
so a working tree with no `.jj/` at or above it is not a case to dispatch on — it
is a refusal, returned before anything is created or changed. The last clause
disposes of the question a reader arriving from Git will ask: a `.git` beside a
`.jj` is a colocated repository, it is jj's business, and nothing here reads it,
spawns `git`, or branches on its presence. *The gate* takes that clause as its
whole thesis.

<!-- fragment «library-purpose-sentence» owner="no-dependencies" source="crates/jj-workspace/src/lib.rs" lines="1-9" parent="library-crate-thesis" -->
````rust
//! Resolve a Jujutsu workspace, refuse a working tree that is not one, and take
//! a path-scoped commit.
//!
//! That sentence is the whole crate. There is no repository abstraction here
//! and no second lane behind it: **jj is the version control system**, and a
//! working tree with no `.jj/` at or above it is refused with the command that
//! fixes it, before anything is created or changed. A `.git` beside a `.jj` is a
//! colocated repository and is jj's business — nothing here reads it, spawns
//! `git`, or branches on its presence.
````
<!-- /fragment -->

The second refusal is the one `CONTEXT-MAP.md` builds its argument on: the crate
has no vocabulary for its consumer, so the consumer supplies the name. Two
alternatives are rejected in a single sentence here, and they fail differently.
Handing back a grove-shaped directory would put this crate's name inside a crate
that refuses to know its consumer. Handing back jj's administrative directory
raw would put a consumer's generic filenames straight into a namespace jj owns
and may extend, which is a collision the next jj release can create. Naming the
consumer is what makes the postcondition sayable at all: *this directory is
yours, it is inside the workspace, and nothing tracks it.* *The namespace it
will not name* owns that argument.

<!-- fragment «library-thesis-no-consumer» owner="no-dependencies" source="crates/jj-workspace/src/lib.rs" lines="10-20" parent="library-crate-thesis" -->
````rust
//!
//! # It knows nothing about its consumer
//!
//! Every name in this crate is one jj already uses. Where a guarantee cannot be
//! stated without naming *whose* files it is about, the consumer supplies the
//! name: [`Workspace::control_dir`] takes a namespace rather than handing back a
//! grove-shaped directory, and rather than handing back jj's administrative
//! directory raw — which would put a consumer's generic filenames straight into
//! a namespace jj owns and may extend. Naming the consumer is what makes the
//! postcondition sayable in this crate's own vocabulary: *this directory is
//! yours, it is inside the workspace, and nothing tracks it.*
````
<!-- /fragment -->

The third refusal enumerates what is absent and names the owner of each: no
witness, no manifest, no rollback proof, no index image, no quarantine and no
recovery path, because jj snapshots the working copy before every command and its
operation log *is* the transaction record. The observable consequence is that
`commit` is one path-scoped `jj commit` and the only thing the crate adds is the
refusal returned when that command does not complete. *Scope and commit* and
*Refusal* split that claim between them: the first argues why there is nothing to
build, the second reads what the crate says when the command declines.

<!-- fragment «library-thesis-no-transactions» owner="no-dependencies" source="crates/jj-workspace/src/lib.rs" lines="21-30" parent="library-crate-thesis" -->
````rust
//!
//! # It takes commits; it does not implement transactions
//!
//! There is no witness, no manifest, no rollback proof, no index image, no
//! quarantine and no recovery path, because jj already owns all of them: it
//! snapshots the working copy before every command, and its operation log *is*
//! the transaction record. So [`Workspace::commit`] is one path-scoped
//! `jj commit`, and the only thing this crate adds to it is the refusal it
//! returns when that command does not complete — which names jj's own repair
//! and runs none of it.
````
<!-- /fragment -->

The fourth heading states the read discipline and then immediately states its own
exception, which is the most carefully argued paragraph in the crate. Probes that
ask jj about something the working copy cannot change pass `--ignore-working-copy`
so they cannot record an operation. `is_tracked` deliberately does not, because
its answer *does* depend on the working copy. The justification is a
**measurement** on jj 0.44.0 rather than a preference: a snapshotting probe
records an operation only when the working copy has actually changed, and that is
the same snapshot jj would take at the next command — taken earlier, not taken
twice. *Scope and commit* re-reads this beside the code it governs.

<!-- fragment «library-thesis-reads-add-no-history» owner="no-dependencies" source="crates/jj-workspace/src/lib.rs" lines="31-45" parent="library-crate-thesis" -->
````rust
//!
//! # Reads add no history
//!
//! [`Workspace::resolve`] is a filesystem walk, and the probes that must ask jj
//! about something the working copy cannot change — which workspace holds the
//! repository, what a commit's change id is — pass `--ignore-working-copy` so
//! they cannot record an operation at all.
//!
//! [`Workspace::is_tracked`] deliberately does **not**, because its answer does
//! depend on the working copy. jj's model is that the working copy is always
//! snapshotted, so a probe that skipped the snapshot would answer about a state
//! the tree has already left. Measured (jj 0.44.0) rather than assumed: a
//! snapshotting probe records an operation only when the working copy has
//! actually changed, and that is the same snapshot jj would take at the next
//! command for any reason — taken earlier, not taken twice.
````
<!-- /fragment -->

The rest of the file's opening is the crate's whole module structure: two private
modules, one re-export, and two imports from `std`. `Refusal` is the only name
re-exported, so the public surface is this file plus that one type. The absence
of any other `use` line is the empty dependency table showing through into the
code.

<!-- fragment «library-module-surface» owner="no-dependencies" source="crates/jj-workspace/src/lib.rs" lines="46-54" parent="library-crate-thesis" -->
````rust

mod jj;
mod refusal;

pub use refusal::Refusal;

use std::fs;
use std::path::{Path, PathBuf};

````
<!-- /fragment -->

<a id="public-surface"></a>
## The public surface

This section is the vocabulary the worked example below needs, and nothing more.
Each name is introduced at the minimum a reader must hold to follow the trace;
the chapter that owns the source explains it properly, and `source-index.md`
records that debt as an early use.

There are three public types.

`Workspace` is a resolved workspace. It is a value whose existence is the proof
that the precondition passed: it cannot be constructed for a working tree that is
not jj-enabled, so an operation reached through it never has to re-ask. It
carries the workspace root and the root of the workspace that holds the
repository. *The gate* owns it.

`Refusal` is the one error type. It is an opaque value carrying what is wrong,
where, and the jj command that fixes it, with no matchable variants — every case
is a stop, so there is nothing for a consumer to branch on. *Refusal* owns it.

`Commit` is what a taken commit returns. It carries a change id rather than a
commit id, because a change id still names the work after a rewrite. *Scope and
commit* owns it.

There are four operations, all of them `Workspace`'s, plus two accessors that
answer from fields already held. `resolve` walks up from a path to the
closest ancestor holding `.jj/` and is the only constructor. `control_dir` takes
a namespace and returns a directory reserved for it. `is_tracked` asks whether
the workspace holds a path. `commit` takes a path-scoped commit and returns a
`Commit`. `root` and `main_repo` are getters.

**Four operations plus two accessors is six public functions, and that is the
count every later chapter uses.** The book counts a function into `Workspace`'s
surface when it is `pub`, and it counts it once: `resolve` is an associated
function rather than a method — it takes no `&self`, because constructing the
proof is what it is for — and the other five are methods. `impl Workspace` also
holds two private helpers, `fileset` and `relative`, which *Scope and commit*
reads and which are never part of this count; they are the crate's own working,
not its offer. So *four operations*, *two accessors*, *six functions* and *five
methods* are four true statements about one set, and a chapter that means one of
them says which.

A namespace, in this crate's vocabulary, is one plain directory name the consumer
supplies. The directory it names is inside the workspace, untracked, never shared
with another namespace, and created if absent. *The namespace it will not name*
owns it.

<a id="commit-tour"></a>
## One command, end to end

This is the operation the book carries. It appears three times: here at low
resolution, in *Scope and commit* at full resolution with every argument and
return value, and in *Refusal* as the identical attempt ending in a refusal
instead of a commit. The tree, the paths and the change id below are fixed here
and reused unchanged by both.

The starting tree is a native jj workspace — one that is not colocated with Git
and holds its own repository rather than borrowing another's — with a grove task
tree inside it:

```text
/work/atlas/
├── .jj/
│   ├── repo/                                   a directory: this workspace holds its own
│   └── working_copy/
├── .grove/
│   ├── BRIEF.md
│   └── 01-DONE-impl--rate-limit-k3.md
└── crates/
    └── gateway/
        └── src/                                the caller's working directory
```

The caller is somewhere inside the tree, not at its root, which is the ordinary
case: a session's working directory is wherever its harness started. It has just
renamed a task file to record that the task is retired, and it wants that one
file committed and nothing else in the working copy. Here is what happens, step
by step, with the values this tree produces.

```text
Workspace::resolve("/work/atlas/crates/gateway/src")
  the ancestor walk asks four directories for a `.jj/` child:
    /work/atlas/crates/gateway/src   no
    /work/atlas/crates/gateway       no
    /work/atlas/crates               no
    /work/atlas                      yes
  the found directory is canonicalised          -> /work/atlas
  main_repo_of("/work/atlas"):
    `.jj/repo` is a directory, not a pointer file
    -> the workspace holds its own repository; no jj is spawned
  -> Workspace { root: "/work/atlas", main_repo: "/work/atlas" }

Workspace::control_dir("grove")
  "grove" is one plain directory name and is not one jj owns
  create_dir_all("/work/atlas/.jj/grove")        absent, so it is created
  -> "/work/atlas/.jj/grove"

Workspace::commit(
    [".grove/01-DONE-impl--rate-limit-k3.md"],
    "rate-limit-k3: refuse a request over the burst ceiling",
)
  the path is made relative to the root and rendered as a jj fileset
    -> root:".grove/01-DONE-impl--rate-limit-k3.md"
  jj is spawned in /work/atlas with four arguments:
    ["commit", "-m", "rate-limit-k3: refuse a request over the burst ceiling",
     "root:\".grove/01-DONE-impl--rate-limit-k3.md\""]
  and then, read-only, to name what was just taken:
    ["log", "-r", "@-", "--no-graph", "--ignore-working-copy", "-T", "change_id"]
    -> vrxqnwzomtklpsuvyzqrnwmtkxlpsoun
  -> Commit { change_id: "vrxqnwzomtklpsuvyzqrnwmtkxlpsoun" }
```

`main_repo_of` is the one step in that trace that could have spawned jj and did
not, and it is named here because the reader meets it two chapters before the page
that owns it. It decides which workspace holds the repository: `.jj/repo` is a
directory in a workspace that holds its own and a pointer file in one that borrows
another's, so only the borrowed case has a pointer to follow — and following it is
jj's job rather than this crate's. *The gate* owns it.

Argument lists are shown rather than command lines because the boundaries
matter: the message is one argument however many spaces it contains, and the
fileset is one argument however it is quoted. *The subprocess seam* shows where
those lists are turned into a process, and *Scope and commit* shows where the
fileset string is built.

Five things in that trace are the whole book. The walk is the **filesystem's**,
not jj's, so nothing in the environment can redirect it — *The gate*. The second
spawn passes `--ignore-working-copy` and the first does not, and the difference
is not an oversight — *Scope and commit*. Both spawns are built at one seam that
fixes the working directory and strips four environment variables — *The
subprocess seam*. The string `"grove"` arrives from outside and the crate never
looks at what it means — *The namespace it will not name*. And every arrow in
that trace has a second ending in which a `Refusal` is returned instead —
*Refusal*.

> **The consumer's half.** The commit above is a
> [task commit boundary](../../../CONTEXT.md#task-commit-boundary), and the
> reserved directory `.jj/grove` holds the
> [driver lease](../../../CONTEXT.md#driver-lease) and the
> [loop control channel](../../../CONTEXT.md#loop-control-channel). None of those
> three terms exists in this crate; all three are what its one consumer does with
> what it returns.

<a id="the-six-refusals"></a>
## The six refusals

Now that one operation has been seen end to end, the map. Each row is a chapter,
each chapter opens on the refusal in its row, and the last chapter assembles them.

| # | The refusal | Who owns it instead | Chapter |
|---:|---|---|---|
| 1 | No dependencies | `std` spawns a process and reads a directory | Orientation |
| 2 | No second lane, and no repository abstraction | jj is the version control system | The gate |
| 3 | Nothing ambient chooses the repository | `current_dir`, with the selectors removed | The subprocess seam |
| 4 | No vocabulary for its consumer | the consumer, which supplies the namespace | The namespace it will not name |
| 5 | No transactions, and no history added by a read | jj's snapshot and its operation log | Scope and commit |
| 6 | No remedy of its own to offer | jj, whose repair the refusal quotes | Refusal |

Refusal 5 has an exception, and it is the one place in the crate where a claim is
supported by a measurement rather than by an argument. `is_tracked` is the single
probe whose answer depends on the working copy, and so the single one that lets
jj snapshot before answering. Every other probe passes `--ignore-working-copy`.
*Scope and commit* owns that asymmetry and states what was measured.

Refusal 1 is this chapter's, and it is now fully read: an empty `[dependencies]`
table, a dev-dependency no consumer inherits, an error type that leaks no error
crate, and a module surface whose only `use` lines are `std`. The remaining five
are argued at the sites that create them, beginning with the gate.

[Contents](README.md) | [Next: The gate](02-the-gate.md)
