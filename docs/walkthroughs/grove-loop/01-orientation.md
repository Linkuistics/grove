# Orientation
<!-- book-page id="orientation" slice="allowed-to-mean" order="1" -->
[Contents](README.md) | [Next: The tokens, and the four verdicts](02-the-tokens.md)

<a id="allowed-to-mean"></a>
## What this crate is allowed to mean

`grove-loop` is the layer that stayed. Three domain-free crates sit underneath
it — `ordinal-fs-tree` has an ordered filesystem tree, `keyed-launch` has a key
and a template, `jj-workspace` has a workspace and a commit — and none of them
has a word for a *kind*, a *brief chain*, an *outcome*, a *handle* or
*finishing*. Those words are here, because meaning is the one thing a
domain-free crate cannot hold, and every chapter of this book opens on what this
module kept when the others took the rest and on why that part could not move.

Two files state that for the crate as a whole, and this chapter reconstructs
both: `Cargo.toml`, whose comment marks the crate *not domain-free* and permits
five dependencies on that basis, and `src/lib.rs`, whose first
paragraph is the claim and whose remaining lines are the crate's own map of
itself.

**The comparison with the two books written before this one is made here and
nowhere else.** The `jj-workspace` book opens each chapter on something that
crate declines to own and names who owns it instead; the `keyed-launch` book
opens each chapter on what a stage must not add and must not interpret. Every
refusal in those two crates assigns work to this one: what arrives here is the
part nobody underneath could take.

**One collision is worth naming before any code is read.** This crate takes the
store's `Key` and `Entry` and adds a vocabulary of its own beside them — a kind,
a handle, an outcome — and `CONTEXT-MAP.md` keeps the two apart by hand: the
store's *leaf* and *node* are not grove's, and the store's *ordinal* is grove's
*position*. Where a page of this book speaks of both trees, it says which one it
means in each sentence rather than relying on the word to carry it.

<a id="the-package"></a>
## The package: five dependencies, and the set it imposes

The manifest is production source and this chapter reconstructs all sixty-eight
lines of it, in six fragments that follow the file's own blocks. It is read
first because the crate's central claim is checkable there before any Rust is
read: a crate that is allowed to be domain-bound says so in the one file that
holds no code, and the dependency declarations expose the cost of that
permission.

<!-- fragment «manifest-domain-bound» owner="allowed-to-mean" source="crates/grove-loop/Cargo.toml" lines="1-68" parent="source-crate-manifest" -->
<!-- insert «manifest-package-identity» -->
<!-- insert «manifest-dependencies» -->
<!-- insert «manifest-extracted-tree» -->
<!-- insert «manifest-dev-dependencies» -->
<!-- insert «manifest-lints» -->
<!-- insert «manifest-release» -->
<!-- /fragment -->

The first fragment is the package block. Five of its seven fields are inherited
from the workspace root rather than stated here, so the package carries no
version, edition, licence, repository or minimum toolchain of its own — a fact
the release block at the end of the file returns to. The two fields stated
rather than inherited are `name` and `description`, and the description is this
book's subject in one line: the task tree in grove's vocabulary, and the verbs a
session invokes over it. Every claim the rest of this chapter makes is a reading
of that sentence.

<!-- fragment «manifest-package-identity» owner="allowed-to-mean" source="crates/grove-loop/Cargo.toml" lines="1-10" parent="manifest-domain-bound" -->
````toml
[package]
name = "grove-loop"
version.workspace = true
edition.workspace = true
description = "Grove's loop: the task tree in grove's vocabulary, and the verbs a session invokes over it"
license.workspace = true
repository.workspace = true
# Inherited from the workspace root, whose comment carries the evidence: the
# locked dependency graph cannot be parsed by a cargo below 1.85.
rust-version.workspace = true
````
<!-- /fragment -->

The second fragment is the chapter's evidence, and its comment is the crate's
own statement of the permission this book is about: **this crate and the viewer are domain-bound**, and `docs/specs/module-decomposition.md`,
decision 1, marks it *not domain-free* for that reason. The comment then accounts
for the dependency table beneath it as three modules the crate composes —
`jj-workspace`, `keyed-launch` and `ordinal-fs-tree` — plus `anyhow` and `libc`,
and gives a reason for three of them here: `anyhow` is internal only, so a
consumer takes on no error library of grove's; `libc` is attributed to three
modules, the lock-contention probe in `task_tree` first among them; and
`keyed-launch` is called the runner, reached once by the verb surface and in
four more places by the crate. The fifth entry's reason is a comment of its own
in the next fragment, and `jj-workspace` has none. The table declares five
dependencies and this fragment carries four of them. Those last two attributions
are the longest in the file, and the paragraph after the fragment says what made
them so.

<!-- fragment «manifest-dependencies» owner="allowed-to-mean" source="crates/grove-loop/Cargo.toml" lines="11-39" parent="manifest-domain-bound" -->
````toml

# **This crate and grove-tui are domain-bound.** The three reusable
# library crates say nothing about grove; this one is grove — kind, handle, brief
# chain, outcome, finishing — and `docs/specs/module-decomposition.md`, decision
# 1, marks it *not domain-free* for exactly that reason.
#
# Its dependencies are the three modules it composes plus `anyhow` and `libc`.
# `anyhow` is **internal only**: every public entry point answers this crate's
# own opaque [`Error`], so a consumer takes on no error library of ours — the
# same rule `crates/jj-workspace`, `crates/keyed-launch` and
# `crates/ordinal-fs-tree` state for theirs. `libc` supplies Unix calls and flags.
# `observation` uses open flags; the lock-contention probe in `task_tree`
# needs `flock(2)` non-blocking before it announces a wait; `driver_lease`, the
# largest user, for the lease's own `flock(2)` and for the `fcntl(2)`
# close-on-exec descriptors it carries across a spawn; and `loop_driver`, for
# `isatty`, `tcgetpgrp` and `signal`. `keyed-launch` is the runner, and the
# **verb surface** reaches it exactly once: `complete` writes the relaunch flag
# into the channel the driver allocated, through the runner's own child-side
# half of it. The crate reaches it in four more places, all of which arrived
# with the driver at `loop-crate-driver-k22`: `driver_lease` discards an
# abandoned channel, `loop_driver` spawns and reaps through it, `session_config`
# compiles grove's templates against the runner's own types, and `lib.rs`
# re-exports `reraise` so grove's binary can die of a signal without depending
# on the runner itself.
[dependencies]
anyhow = "1.0"
jj-workspace = { path = "../jj-workspace" }
keyed-launch = { path = "../keyed-launch" }
libc = "0.2"
````
<!-- /fragment -->

**Both of those attributions are longer than they need to be, and the reason is
a move.** Until `loop-crate-driver-k22` the lease, the loop and the launch
configuration were not in this crate, and the comment could name one user of
each: the contention probe for `libc`, and the `complete` verb for
`keyed-launch`. The driver's arrival widened both, and it widened them in two
different ways, which is why the two clauses are not parallel.

`libc` supplies the loop's Unix calls and open flags. The contention probe in
`task_tree` uses nonblocking `flock`, the lease uses locking and close-on-exec
operations, and `loop_driver` uses terminal and signal operations. The captured
observer adds directory-only, nonblocking and close-on-exec flags to a read-only
open. [Chapter 5](05-opening.md#captured-observation) explains why this pin can
outlive the tree lock. Tests also reach `libc` to exercise those guarantees;
those calls do not add runtime dependencies to the viewer.

`keyed-launch` gained a **distinction**, and that is the more interesting half.
*Reached by exactly one verb* was true when it was written and is still true —
`complete` is the only one of the twelve verbs chapter 15 counts that touches
the runner — but it was doing duty as a statement about the whole crate, and as
that it stopped being true the moment the driver arrived. So the clause now says
the two things separately rather than letting one stand for the other: the
**verb surface** reaches the runner once, and the crate reaches it in four more
places. Three of those came with the driver — `driver_lease` discards an
abandoned channel, `loop_driver` spawns and reaps through the runner, and
`session_config` compiles grove's templates against the runner's own types — and
the fourth is `src/lib.rs`, this chapter's other root, which re-exports
`reraise` so that grove's binary can end on a signal without naming the runner
as a dependency of its own. That re-export is one of four names this crate
lifts out of its dependencies into its own public surface, and the account of
the library root's exports later in this chapter reads all four.

**A clause that names one user is a class of defect rather than an accident
here**, and [chapter 6](06-paths.md#canonicalise-to-compare) is where the book
makes that case: it groups this clause with two others — *canonicalisation
appears once* and *`<worktree>/.grove`, spelled in exactly one place* — as
uniqueness claims written from the shape of the design rather than from an
enumeration of the code, each true of the intent and false of the source. This
one was found while this chapter was being drafted and corrected at
`manifest-dependency-clauses-k133`, in the same commit as this paragraph, under
the rule that a source change carries its ledger and its pages with it. The
paragraph was rewritten rather than deleted, because the reason a sentence is
worded oddly outlives the defect that forced it. The second member of that class
is `src/lib.rs`'s and belongs to this chapter too: *[the two
openings](#the-two-openings)* reads it, and `grove-root-join-clauses-k148`
corrected it the same way. The third is `task_tree.rs`'s, reproduced by chapter 5
and adjudicated by chapter 6, and `canonicalisation-sites-k149` corrected it
last, so none of the three is still in the corpus.

The third fragment is the fifth dependency and the file's most consequential
claim.
`ordinal-fs-tree` is taken with `default-features = false`, which turns off the
`cli` feature that exists for that crate's own `syllabus` binary and would
otherwise pull in `clap`. The consequence is stated as the point of the line
rather than as a note beside it: with that feature off, the dependency set this
crate **imposes** on a consumer is exactly `libc`. A comment cannot hold that,
because the set is a property of the resolved graph rather than of this file, so
`crates/grove-loop/tests/library_dependency.rs` holds it against `cargo metadata`
in `the_library_imposes_only_libc` and
`every_consumer_takes_the_library_with_default_features_off`. Those two tests are
evidence rather than corpus, and chapter 21 returns to them as the third
question's proof.

<!-- fragment «manifest-extracted-tree» owner="allowed-to-mean" source="crates/grove-loop/Cargo.toml" lines="40-47" parent="manifest-domain-bound" -->
````toml
# The extracted tree library (gh issue #13). `default-features = false` turns off
# its `cli` feature, which exists for its own `syllabus` binary and pulls in
# `clap`: with it off, the dependency set this crate *imposes* is exactly `libc`,
# which is already taken above. That claim is the point of the line rather than a
# note beside it, so `tests/library_dependency.rs` holds it against `cargo
# metadata` — the crate's own manifest says the set is `libc`, and nothing but a
# test says the consumer asked for that set.
ordinal-fs-tree = { path = "../ordinal-fs-tree", default-features = false }
````
<!-- /fragment -->

The dev-dependencies table names one dependency. `tempfile` is what the crate's
tests build a real `.grove/` in, and it is the only thing they need that the
standard library does not supply. That is a consequence of what the crate is: a
tree whose shape is its state cannot be tested against a mock of the filesystem
without testing the mock instead, so the 3,984 lines of inline tests this book
reproduces run against directories on disk.

<!-- fragment «manifest-dev-dependencies» owner="allowed-to-mean" source="crates/grove-loop/Cargo.toml" lines="48-50" parent="manifest-domain-bound" -->
````toml

[dev-dependencies]
tempfile = "3.10"
````
<!-- /fragment -->

The lints table inherits the workspace's lint configuration rather than declaring
its own, which is the mechanism by which this crate is held to the same clippy
and rustc settings as every other member.

<!-- fragment «manifest-lints» owner="allowed-to-mean" source="crates/grove-loop/Cargo.toml" lines="51-53" parent="manifest-domain-bound" -->
````toml

[lints]
workspace = true
````
<!-- /fragment -->

<a id="the-release-block"></a>
## `release = false`, and what it does not freeze

The last block is the one most likely to be misread, and its comment exists to
close the misreading. `release = false` removes this package from what
`cargo release` cuts: no tag of its own, no changelog section, no publish. It
does **not** freeze the version, because the package block above takes
`version.workspace = true`, so a cut moves this crate with every other crate the
release ships. The second paragraph records that publication is an answered
question rather than an open one, and names where the answer lives.

<!-- fragment «manifest-release» owner="allowed-to-mean" source="crates/grove-loop/Cargo.toml" lines="54-68" parent="manifest-domain-bound" -->
````toml

# `cargo release` cuts *grove* (`crates/grove`), and `release.toml` configures
# that cut. `release = false` here means no tag, no changelog section and no
# publish of its own. It does **not** mean a frozen version: this crate takes
# `version.workspace = true`, so a cut moves it with every other crate the
# release ships — one workspace, one release version
# (`docs/specs/module-decomposition.md`, decision 1).
#
# **This crate is not published on its own, and that is settled**
# (`docs/RELEASING.md`, *One release, seven packages, one tag*): it ships inside
# grove's cut, wearing grove's version, and no library member has a release lane
# of its own. Removing this line does not reopen the question — it corrupts the
# cut, which was measured rather than assumed.
[package.metadata.release]
release = false
````
<!-- /fragment -->

That answer matters to a reader of this book in one specific way. A crate whose
publication was undecided would have an argument for a documented error taxonomy,
because consumers outside this repository would be matching on it. This one ships
inside grove's cut, and the one-error rule the library root states next reads
differently in that light.

<a id="the-library-root"></a>
## The library root: five passages, and the chapters they name

`src/lib.rs` is 418 lines. The first fifty are the module's own documentation, in
five passages — two opening paragraphs and three headed sections — and the
remaining 327 are the crate's public surface: its modules, its version, its
exports, the two openings, the reference grammar and the one error. The book reads the whole file here, in nineteen
fragments, and the five passages are read in the file's own order — which is not
this book's chapter order, because the header opens on the crate and the book
opens on the grammar underneath it.

<!-- fragment «library-root» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="1-436" parent="source-library-root" -->
<!-- insert «library-root-thesis» -->
<!-- insert «library-root-and-the-driver» -->
<!-- insert «library-root-opening-mirrors» -->
<!-- insert «library-root-three-shapes» -->
<!-- insert «library-root-one-error» -->
<!-- insert «library-root-modules» -->
<!-- insert «library-root-version» -->
<!-- insert «library-root-imports-and-exports» -->
<!-- insert «library-root-tree-and-vacancy» -->
<!-- insert «library-root-reading-and-writing» -->
<!-- insert «library-root-tree-write» -->
<!-- insert «library-root-tree-write-impl» -->
<!-- insert «library-root-read-and-write» -->
<!-- insert «library-root-grove-root» -->
<!-- insert «library-root-reference» -->
<!-- insert «library-root-reference-display» -->
<!-- insert «library-root-selection» -->
<!-- insert «library-root-error» -->
<!-- insert «library-root-error-traits» -->
<!-- /fragment -->

The first fragment is the spine, and every chapter of this book is a reading of
it. It states the permission — a domain-bound library alongside the read-only viewer — and then it states the boundary by enumeration: the other three
crates have a tree, a key and a template, and a workspace and a commit, and none
of them has a word for a kind, a brief chain, an outcome, a handle or finishing.
The last clause is the crate's own count of its surface, and chapter 15 is where
twelve is checked against the fourteen functions `verbs` declares.

<!-- fragment «library-root-thesis» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="1-9" parent="library-root" -->
````rust
//! **Grove's loop: the task tree in grove's own vocabulary, and the verbs a
//! session invokes over it.**
//!
//! Grove vocabulary lives here and in the read-only `grove-tui` viewer.
//! Its reusable seams are domain-free: `ordinal-fs-tree` has an ordered tree,
//! `keyed-launch` has a key and a template, `jj-workspace` has a workspace and a
//! commit — and none of them has a word for a *kind*, a *brief chain*, an
//! *outcome*, a *handle* or *finishing*. Those are here, and so are the twelve
//! verbs stated in them.
````
<!-- /fragment -->

The second fragment is the crate's one structural oddity, and it is where the
book divides. Since `loop-crate-driver-k22` the crate is also the driver: the
lease that keeps one driver per working tree, the prompt it composes, the launch
configuration it reads, and `run` itself. What is left outside is one binary per
audience, each a command-line surface over what is in here, and those binaries
are the overview's book and the `grove-llm` book rather than this one. Chapters
16 to 20 are this paragraph; chapters 2 to 15 are the rest of the crate.

<!-- fragment «library-root-and-the-driver» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="10-14" parent="library-root" -->
````rust
//!
//! Since `loop-crate-driver-k22` it is also the **driver**: the one-driver-per
//! -working-tree lease, the prompt composition, the launch configuration grove
//! reads, and [`run`] — the loop itself. What is left outside is one binary per
//! audience, each a command-line surface over what is in here.
````
<!-- /fragment -->

The third fragment is the first headed section and the claim chapters 5 to 13
prove. Opening mirrors the store's one level up: `read` and `write` answer
`Reading` and `Writing`, so a caller cannot scaffold over a live grove or read
one that is not there **because the types do not offer it**. The paragraph is
careful about which half of that is absolute. `verbs::root_init` takes a
`Vacancy` and the vacancy is consumed, so the refusal to clobber a live grove is
compiler-enforced and there is no check to skip; a `Tree` or a `TreeWrite`, by
contrast, is proof only that a tree was there when it was opened. The second
paragraph is the join: `read` and `write` take a **worktree**, not a grove root,
and `grove_root` is the join those two share. It used to claim more — that no
caller could spell `<worktree>/.grove` a second way — and *[the two
openings](#the-two-openings)* below reads the function, counts the callers that
do, and says what the corrected wording claims instead.

<!-- fragment «library-root-opening-mirrors» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="15-29" parent="library-root" -->
````rust
//!
//! # Opening mirrors the store's, one level up
//!
//! [`read`] and [`write`] answer [`Reading`] and [`Writing`] for the same reason
//! `ordinal_fs_tree::fs::read` and `write` do: a caller cannot scaffold over a
//! live grove or read one that is not there, **because the types do not offer
//! it**. [`verbs::root_init`] takes a [`Vacancy`] and so cannot run over a live
//! grove — that half is absolute, because the vacancy is consumed. The other
//! half is weaker and says so where it lives: a [`Tree`] or a [`TreeWrite`] is
//! proof that a tree was there **when it was opened**, and [`TreeWrite`]'s own
//! header carries what that does and does not buy across two verbs.
//!
//! They take a **worktree**, not a grove root, and `grove_root` is the join
//! these two share (`docs/ARCHITECTURE.md#tree-access-lock`). It is not the
//! crate's only one: three more production openings spell it for themselves.
````
<!-- /fragment -->

The fourth fragment is the shape of every verb signature in the crate, stated
once so that chapter 15 can read the surface without re-deriving it. A verb that
reads takes a `Tree` and a verb that writes takes a `TreeWrite`, which makes the
lock a verb needs visible in its signature rather than acquired inside it; a
search that matched nothing answers the store's `Sought` rather than an `Option`
each verb re-interprets; and every verb returns the paths it wrote, because its
caller is a session that has to name them in a commit message it writes by hand.
The third bullet is the one that only makes sense from outside the crate: the
caller is an LLM session, and the paths are what it has to name.

<!-- fragment «library-root-three-shapes» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="30-40" parent="library-root" -->
````rust
//!
//! # Three shapes recur across [`verbs`], and each is deliberate
//!
//! * **A verb that reads takes a [`Tree`]; a verb that writes takes a
//!   [`TreeWrite`].** The lock a verb needs is visible in its signature rather
//!   than acquired inside it.
//! * **A search that matched nothing answers [`Sought`]** — the store's word —
//!   rather than an `Option` each verb re-interprets. Reintroducing `Option`
//!   here would move the problem rather than solve it.
//! * **Every verb returns the paths it wrote**, because its caller is a session
//!   that has to name them in a commit message it writes by hand.
````
<!-- /fragment -->

The fifth fragment is the one-error rule, and this chapter owns both halves of
it: the type is defined at the end of this same file, and the last section of
this chapter reads it. `anyhow` lives inside the modules, where errors are prose
with context stacked on them, and stops at this boundary — which is the manifest's
dependency argument restated from the consumer's side. The obligation the type
carries is the runner's: every message names what is wrong **and** what fixes it.

<!-- fragment «library-root-one-error» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="41-49" parent="library-root" -->
````rust
//!
//! # One error for the whole crate
//!
//! [`Error`] is opaque, implements `std::error::Error` and `Display`, and is
//! under the same obligation as the runner's: every one names what is wrong
//! **and what fixes it**. Inside, the modules carry `anyhow` — the crate's
//! errors are prose with context stacked on them, which is what `anyhow` is for
//! — and it stops at this boundary, so a consumer takes on no error library of
//! ours.
````
<!-- /fragment -->

The table below is where a reader checks which chapter answers for each claim the
header makes. Each row is one passage of the module header and the chapters that
own what it claims; the second row is the whole driver half of the book, and the
fourth is the one passage no chapter before 15 returns to.

| `src/lib.rs` section | Lines | Chapters |
|---|---:|---|
| the opening thesis | 1–10 | 1, and the count in 15 |
| the driver paragraph | 12–15 | 16–20 |
| *Opening mirrors the store's, one level up* | 17–30 | 5, and the two openings later in this chapter |
| *Three shapes recur across `verbs`, and each is deliberate* | 32–41 | 15 |
| *One error for the whole crate* | 43–50 | 1 |

<a id="the-crates-own-map"></a>
## The crate's own map

The example this book carries is one grove's whole life, told strictly from the
crate's side: what a verb was given, and what it wrote. It never says what the
session that invoked the verb was for. A reader who has driven a grove supplies
that; a reader who has not is not this book's.

Its anchor is not invented. `DEFAULT_ROOT_SLUG` is `"plan"`
(`crates/grove-loop/src/tree_lifecycle.rs` line 56) and `requirements` is a
reserved kind (`src/task_name.rs` line 246), and `default_root_slug` is what a
caller with no slug of its own gets — the driver's own scaffold has no other. A
grove created that way is a charter and one live leaf, reported in that order,
and the leaf's name is:

```text
01-requirements--plan-k1.md
```

That filename is the example's fixed value and every later chapter takes one part
of it apart: `01` is the position, `requirements` the kind, `plan` the slug, `1`
the permanent key, and `plan-k1` the handle. The same literal appears nine times
in the crate's own inline tests, all of them in `src/tree_lifecycle.rs`, and it
is the first entry of the tree this book was written inside — which is what makes
it checkable rather than illustrative.
The one other value fixed here is the root itself, `<worktree>/.grove`, which the
join read later in this chapter builds.

This chapter's step of the example is the smallest one: the two files just read
go in, and what comes out is the crate's cast — five dependencies, one error, and
twelve verbs. The table names the twelve and the chapter that reads each, and it
is the map the rest of the book fills in; nothing here says what any of them
does beyond the name.

| Verb | Read in |
|---|---:|
| `root_init` | 11 |
| `pick` | 7 |
| `kind` | 8 |
| `brief_chain` | 8 |
| `resolve` | 9 |
| `leaf_add` | 10 |
| `leaf_insert` | 10 |
| `leaf_decompose` | 12 |
| `leaf_retire` | 13 |
| `leaf_prune` | 13 |
| `finish_commit` | 14 |
| `complete` | 15 |

`verbs` declares fourteen `pub fn`, and the two this table does not name —
`stale_cross_refs` and `signal_channel` — are not verbs. Each says so in its own
doc comment, and chapter 15 owns both the count and the argument.

<a id="the-cast"></a>
## The cast

The rest of the file is the crate's public surface. This section reads the module
declarations, the version constant and the export list; the four sections after
it read the types those exports name that this chapter owns.

Eleven modules, four of them public. The seven private ones are where the tree
work happens, and everything a consumer touches from them arrives through the
export list below rather than through a module path — which is what makes the
three shapes of the previous fragment a property of the surface rather than a
convention inside it. The four public modules are `driver`, `prompt`,
`session_config` and `verbs`, and `driver` is the one that needs a reason: it
holds two tree operations the loop calls directly, and chapter 15 reads why
putting them beside the twelve would misstate the size of the surface.

<!-- fragment «library-root-modules» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="50-63" parent="library-root" -->
````rust

mod complete;
pub mod driver;
mod driver_lease;
mod loop_driver;
mod observation;
mod task_grow;
mod task_name;
mod task_tree;
mod tree_lifecycle;

pub mod prompt;
pub mod session_config;
pub mod verbs;
````
<!-- /fragment -->

`VERSION` is the version this repository ships, read from the package's own
`CARGO_PKG_VERSION` and published by both binaries and by the prompt. Its comment
argues that reading one constant makes their agreement a fact about a single
definition rather than about several manifests staying in step.

<!-- fragment «library-root-version» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="64-73" parent="library-root" -->
````rust

/// The version this repository ships, and the only one.
///
/// **One workspace, one release version** (`docs/specs/module-decomposition.md`,
/// decision 1): every crate an operator installs takes `version.workspace = true`,
/// so this constant is the workspace's field however reached. The two binaries and
/// the prompt's published version all read it — `crates/grove-llm` would otherwise
/// answer `--version` with a package version of its own, and the prompt would
/// publish one, neither of which names anything an operator can install.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
````
<!-- /fragment -->

The comment quantifies over the crates an operator installs rather than over the
workspace's members, and the two sets differ by one crate. Seven of the eight
workspace members take `version.workspace = true` — the two binaries and the
five libraries an operator's install is built from. The eighth,
`book-validation`, is the authoring tool behind this book: it carries a
`version = "0.1.0"` of its own, nothing an operator installs reads it, and the
workspace root's manifest records that it does not inherit deliberately, because
its home is with the walkthrough skill rather than this workspace. That is why
the sentence quantifies the way it does, and the invariant it states is what
makes `VERSION` the only version an operator can install: every crate the
install is built from inherits the workspace's field, so there is no second
version for the constant to disagree with. The identical clause appears in
`crates/grove/src/cli.rs`, which the overview's chapter 2 reads.

The last block of the file's declarations is the import list and the export list,
and the export list is the cast in one place. Nine `pub use` lines publish this
crate's own names and three other crates': `jj_workspace::{Commit, Workspace}`
is the version-control seam, which five modules of this crate reach directly and
this line republishes;
`ordinal_fs_tree::Sought` is the store's word for a search that matched nothing,
which the third shape above requires every verb to answer with; and
`keyed_launch::reraise` carries a doc comment of its own explaining the division
it preserves — undoing the signal handler is the runner's, because the runner
installed it, while *whether* to die of the signal is the binary's, and
`LoopOutcome::Interrupted` is how the loop says it may.

The observation exports let a viewer retain captured names and selected bytes
after releasing the shared guard. [Chapter 5](05-opening.md#captured-observation)
owns that capture, the opaque directory lifetime and typed runtime results.
`RunningMandate` and `TreeRelation` let a caller consume witnessed launch evidence
without inferring a binding from numeric filesystem identities.

<!-- fragment «library-root-imports-and-exports» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="74-97" parent="library-root" -->
````rust

use std::cell::RefCell;
use std::fmt;
use std::path::{Path, PathBuf};

pub use complete::{interpret, Disposition};
pub use driver_lease::{admit_ambient_session, DriverLease, SessionEpochGuard};
pub use jj_workspace::{Commit, Workspace};
/// Re-exported so the binary that owns this process's exit status can end on it
/// without taking a direct dependency on the runner. Undoing the handler is the
/// runner's to do — it is the crate that installed one
/// (`crates/keyed-launch/src/run.rs`); *whether* to die of the signal is the
/// binary's, and [`LoopOutcome::Interrupted`] is how the loop says it may.
pub use keyed_launch::reraise;
pub use loop_driver::{run, LoopOutcome};
pub use observation::{
    try_observe, ActivityObservation, CapturedTree, LaunchTreeIdentity, ObservationGuard,
    RunningMandate, TreeLifetime, TreeObservation, TreeRelation,
};
pub use ordinal_fs_tree::Sought;
pub use prompt::{compose, Mandate};
pub use session_config::{SessionConfig, TemplateSource};
pub use task_name::{Handle, HandleError, Kind, Outcome, Parts, Slug, TaskName, TokenError};
pub use task_tree::entry_path;
````
<!-- /fragment -->

Four names in that block belong to no chapter of this book — `Commit` and
`Workspace` are `jj-workspace`'s, `Sought` is the store's, and `reraise` is the
runner's, and each of those crates has a book of its own. Every other name in it
belongs to a later chapter of this one. The types this chapter owns — `TreeWrite`,
`Reading`, `Writing`, `Reference` and `Error` — are declared further down the
file and are what the four sections after this one read. The table below states the
minimum a reader needs to follow this chapter and nothing more; each row's owning
chapter is where the full account lives. Every row is an **early use** — a name
this page must state a minimum for because its owner is ahead of it — and the
source index carries these thirteen verbatim. It carries further rows besides,
each first used on a page later than this one, and every chapter may add to them
as it reproduces blocks a still later chapter owns. The ledger is the whole set;
this table is the part of it this page owes.

| Names | Minimum statement | Chapter |
|---|---|---:|
| `Outcome`, `TokenError` | The terminal marks a name can carry — `DONE` and `ABANDONED` — and the refusal a token that is not well-formed produces. | 2 |
| `Handle`, `HandleError`, `Kind`, `Parts`, `Slug` | The named parts of a task name: a kind token, a slug, and the `<slug>-k<key>` handle that is the entry's identity. `Parts` is the set of them a positioned name decomposes into. | 3 |
| `TaskName` | One parsed entry name, which renders back to the bytes it was parsed from or refuses to be computed at all. | 4 |
| `Tree`, `Vacancy`, `task_tree::Guard`, `task_tree::write` | The tree read under the store's shared lock; the lock over a root that holds no tree; the store guard one mutation consumes; and the reopening a `TreeWrite` performs when it no longer holds one. | 5 |
| `Selection` | The leaf a session was launched to work: its path, its identity and its kind. | 7 |
| `verbs::resolve`, `Resolution` | Resolution of one reference against the tree, whose `Ambiguous` case lists the keys of every entry a bare slug matched. | 9 |
| `verbs::root_init` | The verb that consumes a `Vacancy` and creates the whole grove — charter and first live leaf — as one store operation. | 11 |
| `interpret`, `Disposition` | What the child side of the loop makes of a token written to the control channel: relaunch, or stop. | 15 |
| `verbs`, `verbs::stale_cross_refs`, `verbs::signal_channel` | `verbs` declares fourteen public functions; twelve of them are the tree's verb surface, and `stale_cross_refs` and `signal_channel` each say in their own doc comment why they are not verbs. | 15 |
| `admit_ambient_session`, `DriverLease`, `SessionEpochGuard` | The lease that keeps one live driver per working tree, the epoch that decides which calls it admits, and the check a session runs when there is no driver at all. | 16 |
| `SessionConfig`, `TemplateSource` | Whose configuration file a launch is expanded from, and whether a second one beside it is admissible. | 18 |
| `compose`, `Mandate` | The prompt a session is launched with, composed from the parts a skill cannot supply because by the time it could speak the moment has passed. | 19 |
| `run`, `LoopOutcome` | The loop itself, and how it ends: relaunched with fresh context, stopped resumably, or interrupted. | 20 |

<a id="the-two-openings"></a>
## The two openings, and the join they share

Two type aliases, two enums, one struct and two functions are the whole of how a
caller gets at the tree, and they are in this file rather than in `task_tree`
because the join to `.grove` is here. Chapter 5 reads the module behind them; this section reads what
the crate publishes.

The first two aliases are the read side and the create side. `Tree` is the task
tree read once under the store's shared lock, and it derefs to its snapshot so a
caller that wants to look at names directly can. `Vacancy` is the lock over a
root that holds **no** tree, together with the affordance to create one under it;
its comment states the consequence this crate relies on everywhere else — because
`verbs::root_init` consumes one, there is no check against clobbering an existing
grove, since there is no way to call the verb.

<!-- fragment «library-root-tree-and-vacancy» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="98-112" parent="library-root" -->
````rust

/// The task tree, read once under the store's **shared** lock.
///
/// Derefs to its snapshot, so a caller that wants to look at names directly can.
/// Every read verb in [`verbs`] takes one of these.
pub type Tree = task_tree::Tree;

/// The lock over a root that holds **no** tree, and the affordance to create one
/// under it.
///
/// [`verbs::root_init`] consumes one. That it can only be obtained from
/// [`write`] over a genuinely empty root is the whole of grove's refusal to
/// clobber an existing grove — there is no check, because there is no way to
/// call the verb.
pub type Vacancy = task_tree::TreeVacancy;
````
<!-- /fragment -->

The two enums are what the openings answer, and the pair of them is the shape the
module header called mirroring the store's. `Reading` distinguishes a tree from
an empty root and says in its own comment that the empty case is not an error,
because `grove` asks it of a fresh checkout on every iteration. `Writing` makes
the same distinction under the **exclusive** lock, and its two cases are the two
things a writer can be given: the tree, or the vacancy where one could be
created.

The observer adds `TryReading`: Ready contains the ordinary `Reading`, while
Busy contains no guard or snapshot. Public `try_read` below delegates to the
same `task_tree` acquisition owner and prints nothing, leaving a viewer free to
render waiting state and retry without blocking terminal input.

<!-- fragment «library-root-reading-and-writing» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="113-137" parent="library-root" -->
````rust

/// What [`read`] found.
pub enum Reading {
    /// A tree, under the shared lock.
    Tree(Tree),
    /// No tree at this worktree. Not an error: `grove` asks this of a fresh
    /// checkout on every iteration.
    Vacant,
}

/// A quiet observer opening, with contention distinct from absence or errors.
pub enum TryReading {
    /// A shared reading, including a guarded determination of absence.
    Ready(Reading),
    /// A writer holds the tree. No snapshot or file bytes were captured.
    Busy,
}

/// What [`write`] found — under the **exclusive** lock either way.
pub enum Writing {
    /// A tree, and the affordance to mutate it.
    Tree(TreeWrite),
    /// No tree, and the affordance to create one.
    Vacancy(Vacancy),
}
````
<!-- /fragment -->

`TreeWrite` is the right to be the writer, and its header is the longest argument
in the file because the type is easy to misread. A store mutation consumes its
guard, so one guard is one operation and a verb signature taking `&TreeWrite`
cannot hold a guard across calls; this type holds the guard `write` opened with,
hands it to the first verb that asks, and reopens for the next one. What it is
therefore **not** is the tree under a lock held for as long as the value is held.
Three consequences follow and the comment gives all three: a second verb can find
the tree gone, a second verb waits on a new lock and announces contention of its
own, and nothing that opens the tree itself may be called while one is in hand,
because two file descriptions on one directory do not share an `flock` and the
call would block against this process forever. The gap between one guard closing
and the next opening is the gap `docs/adr/bulk-marks-are-not-atomic.md` records,
and chapter 13 is where a subtree prune spends *N* guards for *N* marks.

<!-- fragment «library-root-tree-write» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="138-193" parent="library-root" -->
````rust

/// **The right to be the writer** — the surface every mutating verb is on.
///
/// # Why this is a wrapper, and what it is *not*
///
/// A store mutation **consumes** its guard (`crates/ordinal-fs-tree/src/fs/mod.rs`,
/// *A mutation consumes its guard*), so one guard is one operation and a verb
/// signature of `&TreeWrite` is uncallable against that directly. This holds the
/// guard [`write`] opened with, hands it to the first verb that asks, and
/// **reopens** for the next one.
///
/// So it is not *the tree under a lock held for as long as you hold this*. The
/// lock is real from [`write`] until the first verb returns, and after that this
/// value holds nothing until something asks again. Three consequences, and none
/// of them is a detail:
///
/// * **A second verb can find the tree gone.** Between verbs another writer may
///   mutate it, or an operator may remove `.grove/` — in which case the reopen
///   answers the *no tree here* refusal rather than a guard. A `TreeWrite` in
///   hand is not a standing proof that a tree is there; it was one when it was
///   made.
/// * **A second verb waits on a *new* lock**, so it announces contention of its
///   own. The reopen goes through [`task_tree::write`] rather than
///   `reopen_write` for exactly that reason: `reopen_write` skips the diagnostic
///   because *one verb's* later guards are part of a wait already announced, and
///   the gap between two verbs is not that wait.
/// * **Do not hold one while calling anything that opens the tree itself.**
///   [`read`], [`write`], [`verbs::finish_commit`] and both of the driver's own
///   two tree operations take their own lock on a second file description,
///   and two descriptions on one directory do not share an `flock` — so the call
///   blocks forever, against this process. That is the deadlock
///   `collapse-tree-access-k13` deleted a whole layer to remove, and the shape
///   is expressible again as soon as a caller holds a lock across a call. The
///   rule is one sentence: **take the opening you need, spend it, and let it
///   go.** A verb that genuinely needs its own opening — there is exactly one,
///   [`verbs::stale_cross_refs`] — calls `relinquish` below first, which
///   is how it obeys that rule rather than an exception to it.
///
/// The gap between one guard closing and the next opening is exactly the gap
/// `docs/adr/bulk-marks-are-not-atomic.md` records: a subtree prune is *N*
/// rewrites under *N* guards, and grove takes commits rather than implementing
/// transactions (principle 1).
///
/// It is `Send` but **not `Sync`**, and that is the cheap answer rather than a
/// considered one: nothing in this workspace shares one across threads, and
/// making it `Sync` would let two threads race for the same guard — which is the
/// harmful case, where sharing a `&TreeWrite` within one thread is the harmless
/// one. A consumer that needs `Sync` should pass the worktree path and open per
/// use, which is what every caller here does anyway.
pub struct TreeWrite {
    root: PathBuf,
    /// The guard [`write`] opened with, until the first verb takes it.
    ///
    /// `RefCell` rather than `Cell` only because the guard is not `Copy`.
    opened: RefCell<Option<task_tree::Guard>>,
}
````
<!-- /fragment -->

The three methods are the mechanism the header describes. `root` returns the
spelling the value was opened with rather than an inode, so a worktree replaced
between two verbs is a different directory under the same name. `relinquish` is
the one safe way to open the tree while a `TreeWrite` is in hand, and
`verbs::stale_cross_refs` is the single function in `verbs` that
needs it — the deadlock is unexpressible for it rather than merely unlikely,
which is how the rule is obeyed rather than excepted. It is also, as chapter 15
reads, one of the two functions there that are not verbs. `guard` hands over the opening this value was made
with or takes a fresh one, and its comment records why the borrow ends on its own
line: under edition 2021 an `if let` over `borrow_mut().take()` holds the `RefMut`
for the whole body, so a later edit touching `self` inside it would panic rather
than fail to compile. The process-level property this care is spent on is held by
`one_process_creating_and_reading_a_grove_never_waits_on_itself`, an inline test
inside `src/tree_lifecycle.rs` that chapter 11 owns and reproduces.

<!-- fragment «library-root-tree-write-impl» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="194-246" parent="library-root" -->
````rust

impl TreeWrite {
    /// The grove root — `<worktree>/.grove` — in the spelling this was opened
    /// with.
    ///
    /// It is the spelling, not the inode: a reopen re-resolves it, so a worktree
    /// replaced between two verbs is a different directory under the same name.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Give up the guard this was opened with, if it still holds one.
    ///
    /// **The one safe way to open the tree while a `TreeWrite` is in hand.**
    /// The type's header forbids that in general, and the reason is mechanical:
    /// two file descriptions on one directory do not share an `flock`, so a
    /// second opening blocks against this process's own exclusive lock and
    /// never wakes. A verb that reads through its *own* opening therefore calls
    /// this first, and the deadlock is then unexpressible rather than merely
    /// unlikely — [`verbs::stale_cross_refs`] is the one such verb.
    ///
    /// It costs nothing a caller was promised: this value never was *the tree
    /// under a lock held for as long as you hold it*, and the next verb to ask
    /// reopens exactly as it would have after any other verb spent the guard.
    fn relinquish(&self) {
        drop(self.opened.borrow_mut().take());
    }

    /// One operation's guard: the one this was opened with, or a fresh one.
    ///
    /// **Never call this twice without spending the first**, and never call it
    /// while another guard of this process is live — see the type's own header.
    ///
    /// # Errors
    ///
    /// A tree that has gone since this was opened, or one the store cannot read.
    fn guard(&self) -> Result<task_tree::Guard, Error> {
        // The borrow ends on this line, deliberately and not incidentally: an
        // `if let Some(_) = …borrow_mut().take()` holds the `RefMut` for the
        // whole `if let` under edition 2021, so anything added to that body that
        // touched `self` would panic rather than fail to compile.
        let opened = self.opened.borrow_mut().take();
        match opened {
            Some(guard) => Ok(guard),
            // Announced, because this is a **new** wait: the lock this value was
            // made with has already been released, so a contender that arrived
            // in the gap blocks the caller with nothing said
            // (`docs/ARCHITECTURE.md#tree-access-lock`).
            None => Ok(task_tree::write(&self.root)?),
        }
    }
}
````
<!-- /fragment -->

`read` and `write` are the two functions the whole crate is entered through, and
each is four lines over a `task_tree` call. Both take a worktree and join
`.grove` to it; both translate the store's answer into this crate's own enum.
`write`'s comment records the one asymmetry worth knowing at the call site: the
lock is taken either way, so a refusal from `write` has already waited for
whatever else held it.

<!-- fragment «library-root-read-and-write» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="247-290" parent="library-root" -->
````rust

/// Open the grove at `worktree` for reading, or find that there is none.
///
/// # Errors
///
/// A root that is there but unreadable, or a name in it grove refuses — the
/// store halts the whole tree on a name it cannot spell, and [`Error`] carries
/// what is on disk and what it should be.
pub fn read(worktree: &Path) -> Result<Reading, Error> {
    let root = grove_root(worktree);
    match task_tree::read_or_vacant(&root)? {
        task_tree::Vacant::Tree(tree) => Ok(Reading::Tree(tree)),
        task_tree::Vacant::Nothing => Ok(Reading::Vacant),
    }
}

/// Attempt to read this worktree's grove without waiting or printing diagnostics.
///
/// A successful reading holds the real shared guard; Busy holds nothing.
///
/// # Errors
///
/// As [`read`]. Contention is [`TryReading::Busy`], not an error.
pub fn try_read(worktree: &Path) -> Result<TryReading, Error> {
    Ok(task_tree::try_read(&grove_root(worktree))?)
}

/// Open the grove at `worktree` for writing, or take the lock over the vacancy
/// where one could be created.
///
/// # Errors
///
/// As [`read`]. The lock is taken either way, so a refusal here has already
/// waited for whatever else holds it.
pub fn write(worktree: &Path) -> Result<Writing, Error> {
    let root = grove_root(worktree);
    Ok(match task_tree::write_or_vacancy(&root)? {
        task_tree::Opening::Tree(guard) => Writing::Tree(TreeWrite {
            root,
            opened: RefCell::new(Some(guard)),
        }),
        task_tree::Opening::Vacancy(vacancy) => Writing::Vacancy(vacancy),
    })
}
````
<!-- /fragment -->

The join itself is three lines and it is the reason the two openings take a
worktree rather than a grove root: `read` and `write` both call it, so neither
signature mentions `.grove` and neither can be handed a root that is not one.

**Both comments claimed more than that until this commit, and the claim is one of
the three that [chapter 6](06-paths.md#canonicalise-to-compare) groups as a
class.**
The function's line read *`<worktree>/.grove`, spelled in exactly one place*, and
the module header said that putting the join here *means no caller can spell it a
second way*. Enumerated at the time of the fix rather than taken from the leaf
that recorded it — `grep -rn 'join(".grove")' crates/*/src/`, thirty hits, of
which twenty-three are inside test modules: nineteen in `tree_lifecycle.rs`, one
in `task_tree.rs`, and three in `task_grow/tests.rs`, whose whole file is the
`mod tests` that `task_grow.rs` declares under `#[cfg(test)]`. That leaves
**seven** production sites, and this function is one.

Three of the other six are in this crate and are openings. `tree_lifecycle.rs`
line 76 (`transition_to_current`) and `driver.rs` line 55 (`materialize_finish`)
are the driver's two tree operations, and `driver.rs`'s own header says why
neither could have gone through `read` or `write`: the driver runs them *before*
it has an opening to give them. The third, `tree_lifecycle.rs` line 197, is not
the driver's — `finish_commit` is a session verb, and it does not take a worktree
either but a `&Workspace`, off which it reads one. What the three have in common
is only that each opens the tree for itself, and that is the same list
`TreeWrite`'s header gives from the other side, where `read`, `write`,
`verbs::finish_commit` and the driver's two are named as the calls that must not
be made while a guard is in hand: five openings, two of them through this
function.

The last three are not openings and are not in this crate.
`crates/grove-llm/src/cli.rs` spells the root at lines 505, 570 and 881 to
**name** it: in the refusal when a grove is already there, in `resolve`'s answer
for the root itself, and in the refusal when there is no tree. It has to, because
the library hands back no path for a root it did not open, and the comment above
line 570 says as much — the root is *the caller's own spelling of the tree, not
something read out of it*. That a consumer outside this crate spells `.grove`
three times is the plainest refutation of *no caller*, and it is not a reading
this book had to supply: the `grove-llm` book owns those lines and counts the
same three from inside them.

What survives is narrower and is what both comments now say: `grove_root` is the
join `read` and `write` share, which is what lets both take a worktree, and it is
not the crate's only one. One **true** clause went with the false one, and it is
worth naming because nothing in the corrected wording replaces it: the header
also said `<worktree>/.grove` was *the only spelling grove has ever opened*, which
is a claim about the name rather than about the number of joins, and it still
holds. `grove_name` in `tree_lifecycle.rs` states it in almost those words, and
[chapter 11](11-a-grove-begins.md) reads it there.

**Nothing holds even the narrowed claim.** There is no test over `join(".grove")`
call sites, and this leaf decided against adding one rather than leaving the
question open. `the_librarys_tree_lock_is_taken_from_exactly_one_module`, which
chapter 5 reads, is the instrument this class is usually measured against, and
the disanalogy is the point: that test pins a deadlock, because a second module
taking the store's lock is the failure `collapse-tree-access-k13` deleted a layer
to remove. A count of join sites pins tidiness. A sixth opening that takes a
worktree would need a join of its own and would violate nothing, so a test over
the count would go red on a change that is correct — which is a worse instrument
than none. The absence is recorded here instead, and both comments were narrowed
to what this function's two callers show.

`grove-root-join-clauses-k148` carried both comments, this paragraph, the two
fragments that reproduce them, the three pages of this book that restated the
claim elsewhere and the structure brief's record of it, in one commit. Both
rewordings stayed inside their own line
counts, so no ownership range moved, no ledger row changed and no other book was
touched — which is the cheapest shape a source fix can take while the corpus is
frozen.

<!-- fragment «library-root-grove-root» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="291-295" parent="library-root" -->
````rust

/// `<worktree>/.grove`, for [`read`] and [`write`]. Not the crate's only join.
fn grove_root(worktree: &Path) -> PathBuf {
    worktree.join(".grove")
}
````
<!-- /fragment -->

<a id="naming-an-entry"></a>
## How a session names an entry

A session names an existing entry by text, and the type that carries that text is
in this file rather than in the module that resolves it. Chapter 9 owns the
resolution; this section owns the grammar's statement and the one refusal
`Reference` makes on its own.

`Reference` is a newtype over a string, and its comment carries the whole
grammar: `.` for the grove root, a permanent key as `7` or `[7]` or `[7]-slug`, a
handle as `<slug>-k<key>`, a bare slug, or a path. The paragraph that matters is
the second one: **which form a reference is gets decided against the tree, not
against the text**. A bare slug and a path are told apart by whether the path
exists, and a slug may match several entries, which is why `verbs::resolve`
answers an ambiguous case rather than refusing. What `parse` settles is only that
there is something to look for, and the one thing it refuses is an empty or blank
reference — a refusal whose message names all five spellings, which is the
crate-wide obligation that every error say what fixes it.

<!-- fragment «library-root-reference» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="296-346" parent="library-root" -->
````rust

/// How a session names an existing entry.
///
/// Four forms, and the whole grammar is here: `.` for the grove root, a
/// permanent key (`7` or `[7]`, optionally `[7]-slug`), a [`Handle`]
/// (`<slug>-k<key>`) or a bare slug, and a path — absolute, or relative to the
/// grove root.
///
/// **Which form a reference *is* is decided against the tree, not against the
/// text.** A bare slug and a path are told apart by whether the path exists, and
/// a slug may match several entries — which is why [`verbs::resolve`] answers
/// [`verbs::Resolution::Ambiguous`] rather than refusing. What [`Reference::parse`]
/// settles is only that there is something to look for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference(String);

impl Reference {
    /// Read a reference off a session's command line.
    ///
    /// # Errors
    ///
    /// An empty or blank reference, which names nothing and would otherwise
    /// reach the tree as a slug that cannot match.
    pub fn parse(text: &str) -> Result<Self, Error> {
        if text.trim().is_empty() {
            return Err(Error::msg(
                "an empty reference names nothing. Give `.` for the grove root, a key (`7` or \
                 `[7]`), a handle (`<slug>-k<key>`), a bare slug, or a path under `.grove/`.",
            ));
        }
        Ok(Self(text.to_string()))
    }

    /// The grove root, without going through the text.
    #[must_use]
    pub fn root() -> Self {
        Self(".".to_string())
    }

    /// Whether this reference is the root's own spelling.
    #[must_use]
    pub fn is_root(&self) -> bool {
        self.0 == "."
    }

    /// The text, for a message quoting back what the session asked for.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
````
<!-- /fragment -->

`Display` writes the text back unchanged, which is what lets a diagnostic quote a
session's own spelling rather than a normalised form of it.

<!-- fragment «library-root-reference-display» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="347-352" parent="library-root" -->
````rust

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
````
<!-- /fragment -->

`Selection` is the last alias in the file: the leaf a session was launched to
work, carrying its path, its identity and its kind. It is named here because the
driver half of the crate returns one and the loop launches from it; chapter 7
reads the walk that produces it.

The public `select_snapshot` operation beside it takes the root path spelling,
an already-read typed snapshot and an optional permanent key to exclude. It
returns a `Selection`, no candidate, or the crate's opaque `Error`. The loop owns
the validation and selection rule; this wrapper exposes it to read-only consumers
without opening another tree or granting mutation authority. The viewer uses
this operation to validate before copying rows and selected file content.
[Chapter 7](07-the-walk.md#the-cost-of-the-finish-rule) explains why validation
must precede exclusion and how the remaining candidates are ordered.

<!-- fragment «library-root-selection» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="353-376" parent="library-root" -->
````rust

/// The leaf a session was launched to work: its path, its identity, and its
/// kind.
pub type Selection = task_tree::Selection;

/// Validate and select from an already-read snapshot without filesystem effects.
///
/// Duplicate keys across all items and multiple live finishes are refused before
/// exclusion. Among live leaves other than `excluded_key`, return the first
/// ordinary leaf in depth-first position order, or the sole remaining finish,
/// or `None`. Excluding a branch does not exclude its descendants.
/// `root` supplies the snapshot's path spelling for selections and diagnostics.
/// This operation never allocates a finish sentinel or acquires another lock.
///
/// # Errors
///
/// An ambiguous tree, even if exclusion would hide the ambiguity.
pub fn select_snapshot(
    root: &Path,
    snapshot: &ordinal_fs_tree::Snapshot<TaskName>,
    excluded_key: Option<ordinal_fs_tree::Key>,
) -> Result<Option<Selection>, Error> {
    Ok(task_tree::selected(root, snapshot, excluded_key)?)
}
````
<!-- /fragment -->

<a id="one-error"></a>
## One error for the whole crate

The file ends with the type the module header promised, and reading it is how
this chapter closes: the manifest bought `anyhow` for internal use, the header
said the dependency stops at this boundary, and these forty-five lines are where
it stops.

`Error` is a newtype over an `anyhow::Error` and carries no variant, no code and
no public accessor. Its comment states the obligation that replaces a taxonomy —
every message names what is wrong **and** what fixes it — and attributes the same
rule to `keyed-launch`'s and `jj-workspace`'s errors, which is the workspace-wide
form of it. `Error::msg` is `pub(crate)`, so the only messages this type can
carry are ones a module of this crate wrote.

<!-- fragment «library-root-error» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="377-410" parent="library-root" -->
````rust

/// **One error for the whole crate**, opaque by construction.
///
/// It carries the context stack the modules behind it built, and it is under the
/// obligation `keyed-launch`'s and `jj-workspace`'s errors are under: a message
/// that only reports detection is unfinished, so every one of these names what
/// is wrong **and** what fixes it (principle 2).
///
/// It implements `std::error::Error` with its cause chain intact, so a consumer
/// using `anyhow`, `thiserror` or nothing at all renders it the way it renders
/// any other error — and takes on no dependency of ours to do it.
pub struct Error(anyhow::Error);

impl Error {
    /// Structured configuration refusals, retaining the runner's records or
    /// Grove's source-discovery/admission record. Other failures return no records.
    #[must_use]
    pub fn diagnostics(&self) -> &[keyed_launch::Diagnostic] {
        if let Some(error) = self.0.downcast_ref::<keyed_launch::ConfigError>() {
            error.diagnostics()
        } else if let Some(error) = self.0.downcast_ref::<session_config::SourceError>() {
            std::slice::from_ref(&error.0)
        } else if let Some(error) = self.0.downcast_ref::<Self>() {
            error.diagnostics()
        } else {
            &[]
        }
    }

    /// An error from a message this crate states itself.
    pub(crate) fn msg(message: impl Into<String>) -> Self {
        Self(anyhow::Error::msg(message.into()))
    }
}
````
<!-- /fragment -->

`Error::diagnostics` retrieves runner configuration records or the adapter's
source-admission record through the stored context stack. It also unwraps nested
Grove errors; unrelated failures return an empty slice. The error remains opaque,
while consumers can inspect configuration failures without parsing display text.

The four trait implementations are what make the opacity affordable for a
consumer. `Display` and `Debug` both defer to the inner error, and `Debug`
renders the whole chain the way `anyhow` does, because `Debug` is what a
`Result`-returning `main` prints and is therefore the operator-facing form.
`std::error::Error` keeps the cause chain intact, so a consumer using `anyhow`,
`thiserror` or nothing at all renders this the way it renders any other error and
takes on no dependency of grove's to do it. The `From<anyhow::Error>` is what
lets every module inside the crate keep stacking context with `?` and have it
arrive here as one type.

<!-- fragment «library-root-error-traits» owner="allowed-to-mean" source="crates/grove-loop/src/lib.rs" lines="411-436" parent="library-root" -->
````rust

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for Error {
    /// The whole chain, the way `anyhow` renders one. `Debug` is what a
    /// `Result`-returning `main` prints, so this is the operator-facing form.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}
````
<!-- /fragment -->

Two files, then, and between them the crate's whole claim: a permission stated in
a manifest, and a surface that spends it on a grammar, a walk, a lifecycle, a
verb list and a loop. What could not move is named in each of the header's five
passages and proved nowhere yet. Chapter 2 takes the first step, at the narrowest place the
claim can be checked — a name on disk, and the four verdicts grove can reach
about it.

[Contents](README.md) | [Next: The tokens, and the four verdicts](02-the-tokens.md)
