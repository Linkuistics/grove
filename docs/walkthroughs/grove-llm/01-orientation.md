# Orientation
<!-- book-page id="orientation" slice="one-call-plus-rendering" order="1" -->
[Contents](README.md) | [Next: The grammar and the openings](02-the-grammar.md)

<a id="one-call-plus-rendering"></a>
## One call plus rendering

`grove-llm` is the binary the LLM inside a grove session drives. The driver
launches a session for one leaf, the session runs these verbs to find its leaf,
read its brief chain, grow the tree, mark its own leaf done and signal the
driver, and each of those verbs is a call into `grove_loop::verbs` plus the
rendering of what came back. Its whole source is four files and 1,017
lines, and 944 of them are one module; nearly half of that module is comment,
and the comment at its head states the book's organizing claim.

This book explains that module to a reader who has already run most of the
twelve verbs and read what the guide says each does to the tree. It does not
say that again. Each chapter opens on the one thing a thin binary still has to
get right at that point in a session — the package it is, which is this
chapter; the admission every verb asks for before it is dispatched; the reading
verbs, whose absent answer is information; the growing verbs, which read their
text and ask the presence rule before any lock; the two terminal marks, which
say on stderr what remains; and the two verbs that leave the loop. What the
loop does behind each call is named on those pages and explained on none of
them, and *What order holds* states that boundary in one place.

The chapter's thesis is the header's own, and it has two halves. **The binary
is thin, and the compiler holds it so**: `grove-llm` is a crate rather than a
`[[bin]]` target inside the library it calls, so everything its three Rust
files reach is something a dependency chose to publish, and a verb that
walked a tree or spelled a filename would fail to compile rather than fail
review. That half is the overview's organizing claim and is stated here once,
as a premise the rest of the book reuses. The second half is this book's:
**what is left that is not rendering is order.** The header names three
orders — the operator's text read by the type that owns it *before* a lock is
taken, the just-in-time presence rule asked *before* the mutation, and the
session admitted against the completion channel *before* it is written to.
Each is stated in the source where it happens, each has a test that pins it,
and each has a cost if reversed: a process blocking against itself, a tree
mutated by a command that was refused, a signal sent to a loop that did not
launch this session. Those three, and the test for each, are what the reader
takes away — given a thin surface of their own, what is left there that is not
rendering, and which order would it be wrong to reverse.

<a id="the-package"></a>
## The package: a crate, not a target

The manifest is production source and this chapter reconstructs all
fifty-four lines of it, in eight fragments that follow the file's own blocks.
It is read first because the first half of the thesis is declared there rather
than argued anywhere else: the package is a crate, the crate has one binary
target, and what the binary can reach is what its dependencies publish.

<!-- fragment «manifest-thin-by-crate» owner="one-call-plus-rendering" source="crates/grove-llm/Cargo.toml" lines="1-54" parent="source-crate-manifest" -->
<!-- insert «manifest-package-identity» -->
<!-- insert «manifest-crate-not-a-target» -->
<!-- insert «manifest-grove-dependency-removed» -->
<!-- insert «manifest-bin-target» -->
<!-- insert «manifest-dependencies» -->
<!-- insert «manifest-dev-dependencies» -->
<!-- insert «manifest-lints» -->
<!-- insert «manifest-release» -->
<!-- /fragment -->

The first fragment is the package block. Five of its fields are inherited from
the workspace root rather than stated, and the one that matters to a later page
is `version`: the package carries no number of its own, so `grove-llm
--version` cannot report one. *The grammar and the openings* reads the
attribute that makes both binaries report the same constant, beside a comment
that argues for that constant against the inheritance this line supplies. The
`description` is the one field that is this crate's alone, and it is the
audience split in a sentence; it and `name` are the two fields stated rather
than inherited.

<!-- fragment «manifest-package-identity» owner="one-call-plus-rendering" source="crates/grove-llm/Cargo.toml" lines="1-8" parent="manifest-thin-by-crate" -->
````toml
[package]
name = "grove-llm"
version.workspace = true
edition.workspace = true
description = "The verbs the LLM driving a grove session invokes mid-session"
license.workspace = true
repository.workspace = true
rust-version.workspace = true
````
<!-- /fragment -->

The second fragment is the chapter's argument, and it names the alternative it
rejects: a `[[bin]]` target declared inside `grove-loop`'s own manifest. Rust
privacy is drawn at the crate, so a binary target that lists the library's
modules as its own — compiling the same files a second time as part of itself
— can name any `pub(crate)` item in them; a binary target that depends on the
library beside it sees only what the library publishes, exactly as a separate
crate does. What the package boundary adds is that this entry point cannot take
the first shape without a `#[path]` attribute pointing outside its own package,
which none of its three Rust files carries. The comment's own sentence states
the clause without naming the shape it holds for, and is reproduced as written.
Inside `grove-loop`'s package, *the binary is thin* would be a fact about which
shape the current commit uses, held by whoever reviews the next one; as a
separate crate it is a fact the compiler holds, and no test is needed to assert
it.

The comment's last sentence is a claim this page checks two ways, and the two
answers differ. Every `grove_loop::` name the module uses — the fourteen
imported items and the module under *The imports*, and the five reached by
path — is a `pub` item of that crate, twelve of the imports at its root and
two in its `pub mod verbs`; and the one type it imports from elsewhere,
`Workspace`, is re-exported by that root too, at
`crates/grove-loop/src/lib.rs` line 81. So the binary *reaches* nothing
`grove-loop` did not also publish. But the dependency table below declares
`jj-workspace` directly, and a declared dependency makes the whole of that
crate's public surface *reachable*: a `use jj_workspace::` of any item it
publishes would compile today. The sentence is about what *can* be reached, and
as written it does not hold; what holds is the narrower fact that nothing
reached is unpublished by `grove-loop`. Dropping the line and importing
`grove_loop::Workspace` would make the sentence true and is a source change,
which is a defect leaf's under the corpus freeze and not this book's; the page
states both facts and reproduces the comment as written.

<!-- fragment «manifest-crate-not-a-target» owner="one-call-plus-rendering" source="crates/grove-llm/Cargo.toml" lines="9-15" parent="manifest-thin-by-crate" -->
````toml

# **A crate, not a `[[bin]]` target, and that is the whole point**
# (`docs/specs/module-decomposition.md`, decision 1). A binary target can reach
# its own library's private items, so *the binary is thin* stops being
# compiler-enforced the moment it is a target rather than a crate. Here the
# compiler holds it: everything this binary can reach is something
# `grove-loop` chose to publish.
````
<!-- /fragment -->

The third fragment records a dependency that is gone, and it is owned here
because *The library root* below still names it. Two of this binary's concerns
were once the driver's own — the admission every verb passes through, and the
launch-template check the leaf-writing verbs make — and while they lived in
`crates/grove` this manifest depended on that package too. The leaf the comment
names moved both into `grove-loop`, and the dependency line went with them.
The consequence for a reader of this book is that *admitted* and *declared*,
the two words chapters 2 and 4 turn on, are both answered by calls into the
same crate every other verb calls, and nothing in this binary reaches the human
binary at all.

<!-- fragment «manifest-grove-dependency-removed» owner="one-call-plus-rendering" source="crates/grove-llm/Cargo.toml" lines="16-22" parent="manifest-thin-by-crate" -->
````toml
#
# It once depended on `grove` as well, because two of its concerns were the
# driver's: the session-epoch admission every verb passes through, and the
# just-in-time launch-template check `root-init` and the leaf-writing verbs make.
# `loop-crate-driver-k22` moved both into `grove-loop`, and this manifest lost
# that line with them — so `grove-loop` is now the whole of what this binary can
# reach of grove.
````
<!-- /fragment -->

The `[[bin]]` table is the one binary target: `grove-llm`, at `src/main.rs`.
Unlike the human binary's package, this one also has a library — the target
Cargo infers from `src/lib.rs` without a `[lib]` table — and *The library root*
says why. The binary is three lines over that library, so the table names the
three-line file and the library carries everything else.

<!-- fragment «manifest-bin-target» owner="one-call-plus-rendering" source="crates/grove-llm/Cargo.toml" lines="23-25" parent="manifest-thin-by-crate" -->
````toml
[[bin]]
name = "grove-llm"
path = "src/main.rs"
````
<!-- /fragment -->

Four dependencies, two of them workspace crates. `anyhow` supplies the error
type every handler returns, so a refusal from any verb reaches `main` as a
printed error and a non-zero exit. `clap` with `derive` supplies the grammar,
which *The grammar and the openings* reads. `grove-loop` is the loop, and the
call every verb makes is into its `verbs` module. `jj-workspace` is the second
workspace dependency and the one the paragraph above is about: the module
imports one type from it, `Workspace`, and names it in two functions — the
helper that resolves the working tree from the current directory before any
grove is opened, and `finish-commit`'s handler, which resolves it again from
that root to commit through it. That
type is also published by `grove-loop`'s root, so the direct line buys the
binary nothing it could not already name — and it is what makes the manifest's
reachability sentence wider than the facts.

<!-- fragment «manifest-dependencies» owner="one-call-plus-rendering" source="crates/grove-llm/Cargo.toml" lines="26-31" parent="manifest-thin-by-crate" -->
````toml

[dependencies]
anyhow = "1.0"
clap = { version = "4", features = ["derive"] }
grove-loop = { path = "../grove-loop" }
jj-workspace = { path = "../jj-workspace" }
````
<!-- /fragment -->

<a id="the-dev-dependencies"></a>
## Five dev-dependencies, and the test each exists for

The dev-dependencies are the only trace inside the corpus of the test
directory this book cites and never reproduces, and three of the five carry a
comment saying which test needs them and why. The table names the file behind
each comment, so a reader can open the evidence the manifest points at; its
rows are read from the test directory at this checkout.

| Crate | Why the manifest says it is here | Where it is used |
|---|---|---|
| `assert_cmd` | no comment: the ordinary way to spawn a built binary and assert on its streams and status | most of the directory's files, which drive `grove-llm` as a process |
| `keyed-launch` | the completion channel's own type, so `complete`'s round trip uses the framing the driver reads back | `complete.rs`, which imports `keyed_launch::Channel` |
| `libc` | `flock(2)` directly, so a fixture can hold the tree lock from outside the binary — the one thing no verb can be asked to do | `tree_lock.rs`, which locks the working-tree root itself and then runs a verb against it |
| `ordinal-fs-tree` | the store's `EntryName` trait, which the loop's task-name type implements, so a filename is read back through the seam production uses | `session_kind_guidance.rs`, the one file that imports the crate |
| `tempfile` | no comment: temporary working trees | most of the directory's files |

Two of those rows are the evidence for the orders this book is about. The
`libc` row is *Growing the tree*'s: a fixture that holds the lock from outside
is how the book knows what a verb does when the tree is already locked by
another process, which is the contention the first order is about. The
grove-side half of that self-deadlock risk — whether grove adds a blocking lock
of its own — is checked by
`no_production_lock_grove_takes_for_itself_ever_blocks`
(`crates/grove-llm/tests/tree_lock.rs`), which scans the production source for
every lock call and needs no `libc` to do it. It does not prove a verb never
opens the store twice; *Growing the tree* establishes that structurally by
reading the one handler that takes both openings. The `keyed-launch` row is
*Leaving the loop*'s: the driver
reads the channel back through that crate's framing, so a test of `complete`
that invented its own framing would prove nothing about the driver.
`ordinal-fs-tree` is taken with `default-features = false`, which drops that
crate's one default feature, `cli`, and with it the `clap` dependency behind
its demonstration binary: the tests want one trait, not a second command-line
tool.

<!-- fragment «manifest-dev-dependencies» owner="one-call-plus-rendering" source="crates/grove-llm/Cargo.toml" lines="32-44" parent="manifest-thin-by-crate" -->
````toml

[dev-dependencies]
assert_cmd = "2.0"
# The completion channel's own type, so the round trip through `complete` goes
# through the framing the driver reads back rather than one the test invented.
keyed-launch = { path = "../keyed-launch" }
# `flock(2)` directly, so the lock-contention fixtures hold the tree lock from
# outside the binary they are driving — the one thing no verb can be asked to do.
libc = "0.2"
# The store's `EntryName` trait, which `grove_loop::TaskName` implements: the
# session-kind tests read a filename back through the same seam production does.
ordinal-fs-tree = { path = "../ordinal-fs-tree", default-features = false }
tempfile = "3.10"
````
<!-- /fragment -->

<a id="the-release-block"></a>
## Lints, and what a release does here

The lint configuration is inherited for the same reason `version` is: one
workspace, one standard, and nothing crate-specific to add.

<!-- fragment «manifest-lints» owner="one-call-plus-rendering" source="crates/grove-llm/Cargo.toml" lines="45-47" parent="manifest-thin-by-crate" -->
````toml

[lints]
workspace = true
````
<!-- /fragment -->

The last block settles what the package block left open — what a release does
to a package whose version is the workspace's. The comment's answer is that
the release tool cuts the human binary's package and nothing else, and this
package ships inside that cut. `release = false` therefore takes away a
tag, a changelog section and a publish of its own; it does not freeze a
version, because there is no version here to freeze. The document the comment
cites, `docs/RELEASING.md`, is the author's evidence for that and is named
rather than linked.

<!-- fragment «manifest-release» owner="one-call-plus-rendering" source="crates/grove-llm/Cargo.toml" lines="48-54" parent="manifest-thin-by-crate" -->
````toml

# `cargo release` cuts *grove* (`crates/grove`) and nothing else. This binary
# takes `version.workspace = true` and ships inside that cut; `release = false`
# buys it no tag, no changelog section and no publish of its own, not a frozen
# version (`docs/RELEASING.md`, *One release, six packages, one tag*).
[package.metadata.release]
release = false
````
<!-- /fragment -->

<a id="the-library-root"></a>
## The library root, and why there is one

The library root is sixteen lines, fourteen of them documentation, and it is
reconstructed in two fragments along its two paragraphs. The first paragraph
restates the manifest's thesis from the library's side: the binary is three
lines over this library, and this library is a `clap` surface over
`grove_loop::verbs`, so a separate crate is what makes *thin* the compiler's
fact rather than review's.

Its last clause names two publishers, and the manifest's third fragment above
records that one of them is gone. *Something `grove-loop` or `grove` chose to
publish* was true while this package depended on both; the dependency table
now names `grove-loop` and `jj-workspace`, and no `use grove::` appears in the
module. The manifest's comment is the current fact and this doc comment is the
stale one; the page says so and reproduces the comment as written, under the
same rule as the reachability sentence — a comment is corpus, and a corpus
change is a defect leaf's.

<!-- fragment «library-root-thin» owner="one-call-plus-rendering" source="crates/grove-llm/src/lib.rs" lines="1-8" parent="library-root" -->
````rust
//! **`grove-llm`: the verbs the LLM driving a grove session invokes.**
//!
//! The binary is three lines over this library, and this library is a clap
//! surface over [`grove_loop::verbs`]. It is a separate crate from the loop it
//! drives so that *the binary is thin* is a fact the compiler holds rather than
//! a discipline review has to keep (`docs/specs/module-decomposition.md`,
//! decision 1): everything reachable from here is something `grove-loop` or
//! `grove` chose to publish.
````
<!-- /fragment -->

The second paragraph answers the question the human binary's package settled
the other way. That package has no library, because a library would give its
binary something to reach into. This one has a library, and the comment's
argument is that it costs the guarantee nothing: the code the binary must not
reimplement is in `grove-loop` either way, and a library that carries only the
`clap` surface publishes nothing a verb could misuse. What the library buys is
that this crate's own tests can import it — a `clap` command tree is a value a
test can walk, where a spawned process can only be asked what it prints — and
five files in the test directory do, one of them to unit-test the crate's one
pure function. *Reading the tree* reads that function and this is why it is
`pub`. The module declaration is the file's one line of code, and `cli` is the
only module: the library root and the module are the whole library.

<!-- fragment «library-root-target-and-module» owner="one-call-plus-rendering" source="crates/grove-llm/src/lib.rs" lines="9-16" parent="library-root" -->
````rust
//!
//! There **is** a library target as well as the binary, and it carries the CLI
//! rather than any logic. It exists so the verb surface can be inspected by this
//! crate's own tests — a clap command tree is not something a spawned process
//! can be asked about — and it costs the guarantee above nothing, because the
//! code the binary must not reimplement is in a different crate either way.

pub mod cli;
````
<!-- /fragment -->

The composite that reassembles the file is stated here so the source index can
name it as the root's one child.

<!-- fragment «library-root» owner="one-call-plus-rendering" source="crates/grove-llm/src/lib.rs" lines="1-16" parent="source-library-root" -->
<!-- insert «library-root-thin» -->
<!-- insert «library-root-target-and-module» -->
<!-- /fragment -->

<a id="the-entry-point"></a>
## Three lines

The entry point is one function that calls one function. `main` returns the
`anyhow` result that `cli::run` returns, so an error from any verb is printed
by the standard library's `main` handling with an `Error:` prefix and the
process exits non-zero; a verb that returns `Ok(())` exits zero, which *Reading
the tree* will show includes a verb that found nothing. There is no argument
handling, no setup and no exit-code logic here: the three lines are the whole
of what the binary target adds to the library.

<!-- fragment «entry-point» owner="one-call-plus-rendering" source="crates/grove-llm/src/main.rs" lines="1-3" parent="source-entry-point" -->
````rust
fn main() -> anyhow::Result<()> {
    grove_llm::cli::run()
}
````
<!-- /fragment -->

<a id="the-header"></a>
## The header: the audience, the thesis, the three orders

The module's header comment states the book's organizing claim, and the chapter owns its
thirty-four lines — the comment and the imports that follow it — in three
fragments. The first fragment states who the verbs are for, and the decision it
cites is the audience split this repository's `docs/ARCHITECTURE.md` records
under `cli-binary-split`: one binary for the human, whose bare invocation
selects nothing and runs the loop, and one for the LLM inside a session, whose
every verb is invoked deterministically by a process that has read `--help`.
The second sentence is a consequence for the grammar that *The grammar and the
openings* will show in the `Command` enum: the twelve verbs are flat, hyphenated
names with no subgroup, so one `grove-llm --help` lists every verb — which is
what a session that has lost its context reads to recover.

<!-- fragment «surface-header-audience» owner="one-call-plus-rendering" source="crates/grove-llm/src/cli.rs" lines="1-9" parent="surface-thesis-and-imports" -->
````rust
// The LLM-driven CLI surface — `grove-llm`. See cli-binary-split
// (`docs/ARCHITECTURE.md#cli-binary-split`) for the audience-split
// rationale: every verb here exists for the LLM driving a grove session to
// invoke deterministically, not for a human at a terminal.
//
// Verbs are flat (hyphenated) so a single `grove-llm --help` enumerates every
// verb the LLM might call — important for bootstrap-recovery if a session
// drops context.
//
````
<!-- /fragment -->

The second fragment is the thesis, and every chapter of this book is a longer
reading of one clause of it. *Every verb below is one `grove_loop::verbs::`
call plus rendering* is the claim the fragment graph makes checkable: a reader
holding the whole module can look for a tree walk, a filename rule or a kind
definition and find none, because they live in a different crate since the
leaf the comment names. The second paragraph is where this book parts from the
overview. A thin surface still has to get one thing right, and it is order:
which text is read before which lock, which check is asked before which
mutation, which admission precedes which signal. The comment counts *three
verbs*; the chapters find the first two orders hold in four verbs each — every
verb that writes a leaf — and the third in one, `complete`. Each order is
stated in the handler where it happens, each is pinned by a named test, and
*What order holds* tabulates all twelve verbs against the three.

<!-- fragment «surface-header-thin-and-order» owner="one-call-plus-rendering" source="crates/grove-llm/src/cli.rs" lines="10-23" parent="surface-thesis-and-imports" -->
````rust
// # This crate is thin, and the compiler is what holds it thin
//
// Every verb below is one `grove_loop::verbs::` call plus rendering. Nothing
// here walks a tree, spells a filename or decides what a kind is: those live in
// a **different crate** since `loop-crate-verbs-k21`, so *the binary is thin*
// stopped being a discipline held by review the moment `grove-llm` stopped being
// a `[[bin]]` target inside the library it drives
// (`docs/specs/module-decomposition.md`, decision 1).
//
// What is left here that is not rendering is the **order** three verbs depend
// on, and each is stated where it happens: read the operator's text with the
// type that owns it *before* taking a lock, ask the just-in-time presence rule
// *before* the mutation, and admit the session against the completion channel
// *before* writing to it.
````
<!-- /fragment -->

<a id="the-imports"></a>
## The imports: what the binary reaches

The import block is the evidence for the thesis, read the way the compiler
reads it: every name the module can use without a path is listed here, and
what is not listed is either reached by an explicit path or not reached at
all. Two `use` lines are the standard library's and two are the error and
grammar crates'. The three that matter name one module and fourteen items from
`grove_loop`, and one type from `jj_workspace`. The module is `verbs`, and it
is what the thesis means by *one call per verb*. Read as a count, the slogan
is the header's and not quite the file's: eight handlers make one `verbs` call,
and four make a second through a helper — `brief-chain` picks before it
chains, `leaf-insert` lints after it inserts, `leaf-decompose` reads a kind
before it decomposes, `complete` resolves its channel before it writes — and
the owning chapters name each. What holds without exception is the claim's
substance: no handler does anything a `verbs` call does not do for it, and the
book names those calls and explains none of them. The fourteen items are types
the loop publishes and the handlers pass in or match on, and the block names
every one of them before its owning chapter explains what the binary does with
it. The table states each family's minimum
meaning here so this page can be read without the later one, and the ledger in
the source index carries the same rows; the owning chapter is where each is
read in full.

| Symbol family | What it is, for this page | Owning chapter |
|---|---|---|
| `Reading`, `Tree`, `Writing`, `TreeWrite` | The two openings of a grove — a shared read and an exclusive write — each answering *vacant* as a value rather than an error. | The grammar and the openings |
| `SessionEpochGuard` | The guard `run` obtains when it admits this process against a live session epoch — present under a driver, absent for a manual command — alive through the verb, and consulted only by `complete`. | The grammar and the openings |
| `Workspace` | A resolved jj working tree; every verb but `complete` resolves it from the current directory, and the grove root is spelled from it in one place. | The grammar and the openings |
| `Outcome` | Live, retired or abandoned — the infix a filename carries, rendered as a stderr note so a dead end never looks live. | Reading the tree |
| `Reference` | A parsed spelling of a tree entry — key, handle or slug — read by its own type before any tree is opened. | Reading the tree |
| `Sought`, `Resolution` | `Sought` is a found-or-nothing answer; `Resolution` is what `resolve` found — the root, one entry, or an ambiguity. | Reading the tree |
| `Kind`, `Slug` | The grammar's own types for a `--kind` token and a slug; malformed text is refused by them, before any lock. | Growing the tree |
| `SessionConfig` | The launch configuration, loaded whole and asked whether one kind resolves to a template. | Growing the tree |
| `Handle` | A `<slug>-k<key>` handle, parsed leniently on the key and spoken canonically thereafter. | Leaving the loop |
| `Signalled` | Whether `complete` wrote the disposition to a channel or found no loop to signal. | Leaving the loop |

What the block is not evidence of is the five `grove_loop` items the module
reaches by path and never imports: the `VERSION` constant that the grammar's
attribute reads, `admit_ambient_session` that `run` calls once before
dispatch, the `read` and `write` functions the two openings, one handler and
one helper call, and the `verbs::Inserted` value one helper takes. Four are
`pub` at the crate's root and the fifth is `verbs`'s, so the count of what the
binary reaches is nineteen items and one module, and the block shows fourteen
of them. The one `jj_workspace` import is the type the manifest's second
dependency exists for, and the chapter that owns the function using it —
`worktree`, in *The grammar and the openings* — is where the dependence on jj
is stated. `std::io::Write` is imported for one `writeln!` in *Growing the
tree*, the site that decides which of the module's outputs may fail silently,
and the two path types are what every verb's argument and answer is spelled
in.

<!-- fragment «surface-imports» owner="one-call-plus-rendering" source="crates/grove-llm/src/cli.rs" lines="24-34" parent="surface-thesis-and-imports" -->
````rust

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use grove_loop::verbs::{self, Resolution, Signalled};
use grove_loop::{
    Handle, Kind, Outcome, Reading, Reference, SessionConfig, SessionEpochGuard, Slug, Sought,
    Tree, TreeWrite, Writing,
};
use jj_workspace::Workspace;
use std::io::Write;
use std::path::{Path, PathBuf};
````
<!-- /fragment -->

The composite that reassembles the chapter's share of the module — the header
and the imports, lines 1 to 34 — is stated here; the source index defers the
remaining 910 lines to the five chapters that own them.

<!-- fragment «surface-thesis-and-imports» owner="one-call-plus-rendering" source="crates/grove-llm/src/cli.rs" lines="1-34" parent="source-command-surface" -->
<!-- insert «surface-header-audience» -->
<!-- insert «surface-header-thin-and-order» -->
<!-- insert «surface-imports» -->
<!-- /fragment -->

<a id="one-session"></a>
## One session, end to end

This is the session the book carries. It appears at low resolution here — every
verb named, no handler shown — and chapters 2 to 6 each take the verb they own
at full resolution, on the same tree and the same names. The tree, the leaf,
the configuration and the driver's control files below are fixed here and
reused unchanged by every later page.

The starting tree is the overview's, one template richer: a Jujutsu workspace
holding a grove with one live leaf, a driver that has taken the lease over it,
and a configuration mapping two kinds to commands. The driver has launched a
session for that leaf and is watching for one file to appear.

```text
/work/atlas/
├── .jj/
│   └── grove/
│       ├── driver.lease                 held by the driver for as long as it runs
│       └── session.epoch                rewritten by the driver around this launch
├── .grove/
│   ├── BRIEF.md
│   └── 01-impl--rate-limit-k3.md      live: no DONE or ABANDONED infix
└── crates/
    └── gateway/

~/.config/grove/config.kdl
    impl        "claude --add-dir ${repo} ${prompt}"
    review-impl "claude --add-dir ${repo} ${prompt}"

the session's environment, set by the driver
    GROVE_SIGNAL_FILE=/work/atlas/.jj/grove/signal-3f9c2a7e5b1d4c8890aa61e0f27b4d13
    the file does not exist yet; its appearance is the signal
```

The session's mandate names the handle `rate-limit-k3`, and its shell is at
`/work/atlas`. Here is the session, verb by verb, with the values this tree
produces. Every verb begins the same way — because the driver's channel is in
the environment, admitted against the live
[session epoch](../../../CONTEXT.md#session-epoch), which binds this working
tree, the [driver lease](../../../CONTEXT.md#driver-lease) and that signal path
together — and then makes its call; the line under each verb is what comes
back, on the stream the binary chose for it.

```text
$ grove-llm resolve rate-limit-k3
  admitted, then verbs::resolve under the shared opening: one entry, live
  stdout   /work/atlas/.grove/01-impl--rate-limit-k3.md

$ grove-llm brief-chain /work/atlas/.grove/01-impl--rate-limit-k3.md
  admitted, then verbs::brief_chain under the shared opening
  stdout   /work/atlas/.grove/BRIEF.md

  … the session reads the brief and the leaf, and does the work …

$ grove-llm leaf-add . rate-limit --kind review-impl
  admitted; `review-impl` and `rate-limit` read by their own types; the
  configuration asked whether review-impl resolves to a template: it does;
  then verbs::leaf_add under the exclusive opening — one leaf, fresh key
  stdout   /work/atlas/.grove/02-review-impl--rate-limit-k4.md

$ grove-llm leaf-retire /work/atlas/.grove/01-impl--rate-limit-k3.md
  admitted, then verbs::leaf_retire under the exclusive opening: one rename
  stdout   /work/atlas/.grove/01-DONE-impl--rate-limit-k3.md
  stderr   leaf-retire: two steps remain:
             1. commit this session's work, including this rename
             2. run `grove-llm complete` as your last action

$ jj commit -m 'rate-limit-k3: …'
  jj's, not grove-llm's: the work, the new leaf and the rename, in one change

$ grove-llm complete
  admitted; the channel resolved from GROVE_SIGNAL_FILE and checked against
  the admitted epoch; then verbs::complete writes the relaunch flag to it
  stderr   grove complete: signalled; the loop will start the next task.
  -> the file exists; the driver sees it and ends this session
```

Five of those commands are grove's and one is jj's, and each grove line is the
subject of a later page. The two reading verbs are *Reading the tree*'s: the
shared opening, one call, a path on stdout. `leaf-add` is *Growing the tree*'s,
and the clause *read by their own types* before *the exclusive opening* is the
first order made visible; that chapter's second ending is this same argv with
a kind the configuration does not declare, refused before any lock. The
retirement is *Ending work*'s, and its stderr is why that chapter is named for
two steps. `complete` is *Leaving the loop*'s: the channel is the
[loop control channel](../../../CONTEXT.md#loop-control-channel), and the
check *against the admitted epoch* before the write is the third order. What
*admitted* means at every step is *The grammar and the openings*' premise.

The fresh key is `k4` because a key is the maximum over the whole tree plus
one, and `k3` was the maximum; the new leaf's position is `02` because
positions are per directory and the grove root held one child. Neither rule is
this binary's — both are the call's — and the page names them because the
printed path is the only thing a session sees, and a reader following the
trace should be able to predict it.

The trace above names the verbs; the figure below is the tree they leave
behind, because three of the five later chapters read a state they do not draw.
Two of the session's verbs mutate `.grove/`, so it has three states, and what
the reader is to take from the figure is which chapter is standing at which one
— *Reading the tree* opens on the first and closes on the third, which is why
its transcript jumps.

```text
.grove/ as the driver launched it            read by chapters 2, 3 and 4
├── BRIEF.md
└── 01-impl--rate-limit-k3.md

      $ grove-llm leaf-add . rate-limit --kind review-impl     chapter 4

.grove/ after the add                        read by chapter 4
├── BRIEF.md
├── 01-impl--rate-limit-k3.md
└── 02-review-impl--rate-limit-k4.md

      $ grove-llm leaf-retire …/01-impl--rate-limit-k3.md      chapter 5

.grove/ after the retire                     read by chapters 3 and 5
├── BRIEF.md
├── 01-DONE-impl--rate-limit-k3.md
└── 02-review-impl--rate-limit-k4.md
```

`complete`, the session's last verb, writes outside `.grove/` and leaves the
third state standing; it is the state the session commits. Two later chapters
draw a tree of their own and both go past this one: *Ending work* adds a node
the session never made, so that a prune has something to act on, and *Leaving
the loop* shows the whole grove terminal with a `finish` leaf the driver
materialised.

<a id="the-map"></a>
## Six families, three orders, seven chapters

Now that one session has been seen end to end, the map. The verbs fall into
six families by what they do to the tree, the chapters follow the order a
session meets them, and each chapter opens on the rule its family enforces.
The table says which page holds which family and which of the three orders,
if any, that page states; it is read from the header comment above and from
the structure the pages follow.

| The family | The rule it opens on | The order it states | Chapter |
|---|---|---|---:|
| the manifest, the library root, the entry point, the header | thin is held by the crate boundary, and order and rendering are what thin leaves behind | — | Orientation |
| the grammar, dispatch and the two openings | every verb is admitted before it is dispatched, and a grove that is not there is not a grove that is finished | — | The grammar and the openings |
| `pick`, `brief-chain`, `kind`, `resolve` | an absent answer is information, not an error | — | Reading the tree |
| `root-init`, `leaf-add`, `leaf-insert`, `leaf-decompose` | text before lock, presence before mutation | the first and the second | Growing the tree |
| `leaf-retire`, `leaf-prune` | the last grove verbs a session runs say on stderr what remains | — | Ending work |
| `finish-commit`, `complete` | admit against the channel before writing to it | the third | Leaving the loop |
| all twelve, in one table | what the compiler holds, what order holds, what tests hold | all three, applied back | What order holds |

The first half of the thesis is this chapter's, and it is now fully read: a
separate crate with one binary target and a library that carries only the
surface, two workspace dependencies of which one is reached through the other,
and a header that states thin as the compiler's fact and names the remaining
responsibility. The three orders are stated on three later pages, and the
grammar every verb
passes through before it reaches any of them is read next.

[Contents](README.md) | [Next: The grammar and the openings](02-the-grammar.md)
