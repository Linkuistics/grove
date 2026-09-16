# Orientation
<!-- book-page id="orientation" slice="understands-neither" order="1" -->
[Contents](README.md) | [Next: The names a template is written against](02-the-names.md)

<a id="understands-neither"></a>
## Understands neither

`keyed-launch` sits between a human's configuration file and a running process.
A consumer hands it one file path, optionally a second, and a list of slot names;
it hands back a program and its arguments, spawns that program, and reports how
the launch ended.
Between those two points it learns nothing about the domain it is serving. A key
is an opaque string. A slot is a name the consumer declared. The words of a
template are the words the file holds.

That refusal is the whole of the crate's design, and it is what each of the nine
source-owning chapters opens on; the tenth applies the test they have proved.
This chapter opens on the narrowest form of it: **the crate understands neither
half of the pair it carries.** It does not know what a key names, and it does not
know what a template's program does. Three files declare that — the manifest,
which buys three dependencies and no domain; the library root, which states the
claim in its first paragraph and maps the rest of the crate; and the error
module, whose two types exist so that neither half's caller has to handle the
other's failures.

grove is the consumer this crate was extracted from, and its mapping onto the
crate is one sentence: **a session kind is a key.** That sentence is the
whole of grove's presence in this book. What a session is, what a kind means and
what a launch is *for* are the guide's, and the `README.md` points there once so
that no chapter has to.

**Where the `jj-workspace` book's chapters each open on something that crate
declines to own and name who owns it instead — `std`, jj, the consumer — this
crate names no other owner, because the meaning does not exist anywhere inside it
to be delegated: a key is an opaque string and stays one.** No later chapter
returns to the comparison.

<a id="the-package"></a>
## The package: three dependencies and no domain

The manifest is production source and this chapter reconstructs all forty-seven
lines of it, in five fragments that follow the file's own blocks. It is read
first because the crate's central claim is checkable there before any Rust is
read: a crate that understood what it was launching would need a dependency that
knew something about the domain, and this one buys three that know nothing.

<!-- fragment «manifest-three-dependencies» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="1-47" parent="source-crate-manifest" -->
<!-- insert «manifest-package-identity» -->
<!-- insert «manifest-dependencies» -->
<!-- insert «manifest-dev-dependencies» -->
<!-- insert «manifest-lints» -->
<!-- insert «manifest-release» -->
<!-- /fragment -->

The first fragment is the package block. Five of its seven fields are inherited
from the workspace root rather than stated here, so the package carries no
version, edition, licence or minimum toolchain of its own — a fact the release
block at the end of the file returns to. The two fields stated rather than
inherited are `name` and `description`, and the `description` is the spine of the
book compressed into one line: a configuration of key to complete command
template, validated whole and expanded into an argv. Every claim the rest of this
chapter makes is a reading of that sentence.

<!-- fragment «manifest-package-identity» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="1-10" parent="manifest-three-dependencies" -->
````toml
[package]
name = "keyed-launch"
version.workspace = true
edition.workspace = true
description = "A configuration of key → complete command template, validated whole and expanded into an argv"
license.workspace = true
repository.workspace = true
# Inherited from the workspace root, whose comment carries the evidence: the
# locked dependency graph cannot be parsed by a cargo below 1.85.
rust-version.workspace = true
````
<!-- /fragment -->

The second fragment is the chapter's evidence. Its comment enumerates the three
dependencies and gives the reason for each, and the enumeration is exhaustive:
`kdl` parses the configuration document, `shell-words` splits a template into
words, and `libc` supplies the system calls `std` does not expose. None of the
three knows what a key is for. The comment argues the third by naming the two
calls the *escalation* needs, `kill(2)` and `signal(2)`; the crate reaches
further into `libc` than that, for the process-group and terminal calls chapter 7
owns, and `crates/keyed-launch/src/run.rs` is the only source file that reaches
it at all. The comment also states what is *not* bought — an
error library — and records that `crates/jj-workspace` and
`crates/ordinal-fs-tree` state the same rule for theirs, which is the manifest's
own cross-reference to the error module this chapter reads last.

<!-- fragment «manifest-dependencies» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="11-26" parent="manifest-three-dependencies" -->
````toml

# **Three dependencies: two document formats this crate reads, and the syscalls
# it cannot reach from `std`.** `kdl` is the configuration language;
# `shell-words` splits a template into words the way a reader of the line
# expects, without ever handing it to a shell. `libc` is the escalation:
# `std::process::Child::kill` sends SIGKILL and nothing else, so a graduated
# SIGTERM-then-SIGKILL — and catching the launcher's own SIGTERM/SIGHUP to
# forward it — needs `kill(2)` and `signal(2)` directly. Errors are the crate's
# own opaque `ConfigError` and `LaunchError` implementing `std::error::Error`,
# so a consumer using `anyhow`, `thiserror` or nothing at all takes on none of
# these — the same rule `crates/jj-workspace` and `crates/ordinal-fs-tree` state
# for theirs.
[dependencies]
kdl = "4.7"
libc = "0.2"
shell-words = "1.1"
````
<!-- /fragment -->

Read against the crate's claim, the `libc` line is the one that carries an
argument rather than a fact. `std::process::Child::kill` sends SIGKILL and
nothing else, so a launcher that wants to ask a child to end before insisting
cannot express that through the standard library at all. Chapter 8 is where the
graduated ending is built out of `kill(2)` and `signal(2)`; the manifest is where
the need for them is declared. The two document formats are the other half of the
same discipline: the crate reads a KDL document and splits a line into words, and
it adds exactly one rule of its own on top of each, which chapters 3 and 4 own.

<a id="the-dev-dependency"></a>
## One dev-dependency, and the lints

The dev-dependencies table names one dependency. `tempfile` is what four of the
crate's five test files use to build a directory of configuration documents and
channel files per test, and it is the only thing the suite needs that the standard
library does not supply; the fifth, `tests/reraise.rs`, re-executes itself and
needs no directory at all. Unlike the crate's other siblings, a crate that
spawns processes and writes files might be expected to buy a process harness or
a fixture framework, and this one buys a temporary directory.

<!-- fragment «manifest-dev-dependencies» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="27-29" parent="manifest-three-dependencies" -->
````toml

[dev-dependencies]
tempfile = "3.10"
````
<!-- /fragment -->

The lints table inherits the workspace's lint configuration rather than declaring
its own. That is the mechanism by which this crate is held to the same clippy and
rustc settings as every other member. No `#![allow]` appears in the library root
this chapter reads next, and none appears anywhere in the crate.

<!-- fragment «manifest-lints» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="30-32" parent="manifest-three-dependencies" -->
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
release ships. One workspace, one release version.

<!-- fragment «manifest-release» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="33-47" parent="manifest-three-dependencies" -->
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

The comment's second paragraph states that this is an answered question rather
than an open one, and names where the answer lives: `docs/RELEASING.md`, under
*One release, seven packages, one tag*, which settles that no library member of
this workspace has a release lane of its own. That matters to a reader of this
book in one specific way. A crate whose publication was undecided would have an
argument for a stable, documented error taxonomy, because downstream users
outside this repository would be matching on it. This one ships inside grove's
cut, and the error module reads differently in that light.

<a id="the-map"></a>
## The map: seven sections, eight chapters

`src/lib.rs` is sixty-eight lines and fifty-two of them are the module's
documentation. It is the crate's own account of itself, and this book's chapter
order is that account's order. The book reads it whole here, in seven fragments:
six that follow the doc comment's own paragraph breaks, and one for the module
declarations and exports, which this chapter reads after the worked example.

<!-- fragment «library-root» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="1-85" parent="source-library-root" -->
<!-- insert «library-root-thesis» -->
<!-- insert «library-root-two-documents» -->
<!-- insert «library-root-vocabulary» -->
<!-- insert «library-root-to-a-child» -->
<!-- insert «library-root-job-and-out-of-band» -->
<!-- insert «library-root-conformance» -->
<!-- insert «library-root-modules-and-exports» -->
<!-- /fragment -->

The first fragment is the spine, and every chapter of this book is a reading of
its second sentence. The claim has two halves. The crate understands neither the
key nor the template: a consumer names one and a template names the other, and
nothing in these 2,275 lines interprets either. What the crate does own is
stated positively — a launch is one complete template string read whole out of
one file, never assembled from two, and every rule about a template is checked
before anything is spawned. Chapters 3 and 4 are those two clauses.

<!-- fragment «library-root-thesis» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="1-9" parent="library-root" -->
````rust
//! A configuration of **key → complete command template**, validated whole and
//! expanded into an argv.
//!
//! A consumer names a key; a template names a program. Nothing here understands
//! either: a key is an opaque string, a slot is a name the consumer declares,
//! and the words of a template are the words the file holds. What the crate owns
//! is that a launch is *one complete template string, read whole out of one
//! file* — never assembled from two — and that every rule about a template is
//! checked before anything is spawned.
````
<!-- /fragment -->

The second fragment is the first headed section and chapter 3's charter. It
states the asymmetry between the two documents `Templates::load` reads: a key
resolves only if the **primary** declares it, and an overlay that declares a key
the primary does not is refused by name rather than honoured. The last sentence
gives the reason a reader can carry to their own code — that rule is what keeps a
second source unable to introduce a program the operator never chose — and the
paragraph after it draws the crate's boundary at the filesystem: which two files
those are, and whether an overlay is admissible at all, are the consumer's
questions.

<!-- fragment «library-root-two-documents» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="10-22" parent="library-root" -->
````rust
//!
//! # Two documents, and what the second one may do
//!
//! [`Catalog::load`] captures a primary file and an optional overlay. **A key
//! resolves only if the primary declares it**: where the overlay also declares
//! it the overlay's template is the one used, whole; where only the overlay
//! declares it the key does not resolve, and the refusal names the key and the
//! primary file that must declare it. That is what keeps a second source unable
//! to introduce a program the operator never chose, and it is checked without
//! either document knowing what a key means.
//!
//! Which files those two are, and whether the overlay is admissible at all, are
//! the consumer's questions. This crate reads the paths it is handed.
````
<!-- /fragment -->

The third fragment is five lines and it is the one this book moves. In the file
it follows the two documents; in the book it precedes them, because the argument
it makes is what makes the two documents checkable at all. Every template rule is
a rule about slot *names*, so a loader that will not learn the names until
expansion can check none of them. Chapter 2 owns that argument and takes the
position the crate takes on it; chapter 3 then reads a `load` that has the names
in hand.

<!-- fragment «library-root-vocabulary» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="23-37" parent="library-root" -->
````rust
//! Catalog retains both original documents and their declarations. Resolving an
//! explicit [`Selection`] returns an owned [`Templates`] snapshot without source
//! I/O; expansion still works after the files change and the Catalog is dropped.
//! [`Templates::load`] delegates to that path with an empty selection.
//!
//! Only flat configuration is implemented: selection declarations are absent,
//! and selecting any profile is an error. Wrapper commands, profiles and
//! inspection are pending; no partial inspection API is exposed.
//! [`ConfigError::diagnostics`] exposes stable categories, source byte ranges
//! and remedies. Independent structural errors aggregate across both inputs.
//!
//! # The vocabulary is an input to `load`, not to `expand`
//!
//! Every template rule is a rule about slot *names*, so a loader that will not
//! learn the names until expansion can check none of them. See [`Vocabulary`].
````
<!-- /fragment -->

The fourth fragment states the crate's structural claim, which is the ground this
book's fifth chapter stands on. `Templates::expand` authors an `Argv`; `run`
consumes one; **the two halves meet only at `Argv`**, and neither module takes a
type or calls a function of the other. That is enforced rather than kept by
convention: `Argv` has no public constructor, so a launcher that builds its argv
some other way cannot construct one. The claim is about the code and not about
the prose: `run.rs` does name `Templates::expand` once, in the doc comment on
`Launch::argv`, and what that comment records is the authoring rule rather than a
dependency — nothing in `run` compiles against `templates`, and nothing in
`templates` compiles against `run`. This chapter states the claim; chapter 5
shows the two lines that make it true, and this chapter's last section reads the
error module that the claim is also visible in.

<!-- fragment «library-root-to-a-child» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="38-44" parent="library-root" -->
````rust
//!
//! # From a template to a running child
//!
//! [`Templates::expand`] authors an [`Argv`]; [`run`] spawns it — directly,
//! with no shell — and supervises the child until it ends. The two halves meet
//! only at `Argv`, and each is usable without the other: a launcher that builds
//! its argv some other way still cannot construct one, which is the point.
````
<!-- /fragment -->

The fifth fragment carries the two bold paragraphs that are chapters 7 and 8's
theses and chapter 6's. **The child is a job**: it is spawned into a process
group of its own and handed the launcher's controlling terminal, and the
paragraph gives three consequences of that rather than describing the mechanism.
**A launch ends out of band**: an interactive child returns to its prompt when it
finishes rather than exiting, so its own exit is not the event anyone is waiting
for, and the channel's *appearance* is. Those two sentences are the reason the
crate has a `Channel` at all, and chapter 6 is where the appearance rule is built.

<!-- fragment «library-root-job-and-out-of-band» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="45-57" parent="library-root" -->
````rust
//!
//! **The child is a job.** It is spawned into a process group of its own and
//! handed the launcher's controlling terminal, so a terminal signal reaches the
//! child rather than the launcher, the escalation can reap a grandchild the
//! child spawned, and a launcher's own ignored dispositions are not inherited
//! across the `exec` by every wrapper the template names. See [`run`].
//!
//! **A launch ends out of band.** An interactive child returns to its prompt
//! when it finishes rather than exiting, so its own exit is not the event
//! anyone is waiting for. [`Channel`] is: a fresh path per launch that the
//! child writes a [`Token`] to (through [`signal`]) when it is done, and whose
//! *appearance* starts the kill [`Escalation`] the child cannot perform on
//! itself. See [`Escalation`] for why that is the launcher's job.
````
<!-- /fragment -->

The sixth fragment is the last headed section and chapter 9's charter. It names
the one thing in the crate that is neither configuration nor launch: a kit that
holds a consumer's configuration to this crate's contract **from outside the
consumer's own suite**. The distinction it draws in that clause is the whole of
why the kit exists, and chapter 9 argues it.

<!-- fragment «library-root-conformance» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="58-67" parent="library-root" -->
````rust
//!
//! [`run_observed`] adds synchronous parent-side [`LaunchEvent`] notifications
//! at successful spawn and confirmed reap, including reap during wait-error
//! recovery. Notifications precede token reading and terminal recovery; failed
//! spawn emits none. [`run`] keeps the same interface without an observer.
//!
//! # Testing a consumer's configuration
//!
//! [`conformance::check`] holds a configuration to this crate's contract from
//! outside the consumer's own suite.
````
<!-- /fragment -->

Four of those seven sections carry a `#` heading and two are bold paragraphs; the
opening thesis has neither. The table below is where a reader checks that this
book's order is the crate's own rather than one imposed on it, and it is also
where the two places the book departs from the file are visible. The vocabulary
section is **moved**: third in the file, second in the book, for the reason the
third fragment gives. The out-of-band paragraph is **split**: it argues both why
there is a channel, which is chapter 6, and why ending the child is the
launcher's job, which is chapter 8.

| `src/lib.rs` section | Lines | Chapter |
|---|---:|---:|
| the opening thesis | 1–9 | 1 |
| *Two documents, and what the second one may do* | 11–30 | 3 |
| *The vocabulary is an input to `load`, not to `expand`* | 32–35 | 2 |
| *From a template to a running child* | 37–42 | 5, and the seam in 1 |
| **The child is a job** | 44–48 | 7 |
| **A launch ends out of band** | 50–54 | 6, and the escalation in 8 |
| *Testing a consumer's configuration* | 62–65 | 9 |

Chapter 4 is the one chapter with no paragraph of its own in the library root,
and that absence is itself informative: the template rules are the part of the
crate the module documentation does not argue, and `src/templates.rs` is 13 per
cent comment. Chapter 4 supplies the argument the source does not make.

<a id="the-launch-in-outline"></a>
## The launch in outline

The example this book carries is grove's own configuration, told strictly from
the crate's side. The reader knows exactly what these words mean; the point of
the example is watching the crate not care. It starts here at low resolution —
every call named, no handler shown — and each later chapter takes one step of it
apart. Its values are fixed here and reused by every chapter that follows.

The launch starts with two lines in a file the crate did not write and does not
own. This is the primary document — for grove, a personal `config.kdl` under the
operator's home directory — declaring two keys, each mapped to one complete
command template. The figure is the whole input to the trace below, and the two
`${prompt}` substitutions are the only syntax in it the crate will attach a rule
to. Those two lines are the whole document; what the crate does
with a `#` inside one of those quoted templates is chapter 4's, and it is not
what a reader of shell would expect.

```text
impl "claude --model opus ${prompt}"
review-impl "codex exec --model gpt-5 ${prompt}"
```

The consumer also supplies a vocabulary: the slot names its own templates are
written against, each with a cardinality. grove's is four slots, and the crate
learns nothing from them except their spelling and how often each may appear.
The table states what the loader will check, not what the values will be.

| Slot | Cardinality | What grove will put there |
|---|---|---|
| `prompt` | exactly once | the session's mandate |
| `session_name` | at most once | a label for the session |
| `worktree` | at most once | the working tree holding `.grove/` |
| `repo` | at most once | the main repository root |

The right-hand column is grove's, and it is the only column of the three this
crate never sees. A `Vocabulary` is a slice of `SlotRule`, and a `SlotRule` is a
name and a `Requirement` and nothing else, which chapter 2 reads.

With those two inputs the launch runs as five calls. The trace below is the
book's spine in one column: each step names the chapter that owns it, and no step
shows a line of the handler behind it.

```text
1  Templates::load(primary, None, vocabulary)          chapter 3
     the document is read and validated whole against the vocabulary,
     then two keys resolve. The second document is chapter 3's.

2  templates.expand("impl", [one Slot per declared name: prompt = <the mandate>,
                              session_name, worktree, repo])
     -> Argv ["claude", "--model", "opus", "<the mandate>"]   chapter 5
     a value is offered for every slot the vocabulary declares, not only the
     one this template mentions. Chapter 5 reads why.

3  Channel::allocate(Path::new("/work/atlas/.jj/grove"))       chapter 6
     -> /work/atlas/.jj/grove/signal-3f9c1d4a7b2e5086c1a4f70d93b6e281
     the path is drawn; nothing is written

4  run(Launch { argv, channel, channel_var: "GROVE_SIGNAL_FILE",
                scrub, cwd: Some("/work/atlas"), escalation })  chapters 7, 8
     child spawned in its own process group, holding the terminal,
     with GROVE_SIGNAL_FILE set to the path from step 3

5  the child writes "relaunch\n" to that path and returns to its prompt
     the file appears; the grace elapses; the child is signalled
     -> Ended { end: Signalled, status, elapsed, token: Some("relaunch") }
```

Four properties of that trace are worth naming now, because they are what the
later chapters prove and what the closing chapter tests. **Step 1 reads two
documents and assembles nothing**: a key resolves from one file, whole. **Step 2
produces a value the caller cannot construct any other way**, which is what makes
step 4's promise checkable. **Step 3 writes nothing**, so the file's later
existence is unambiguous evidence that something wrote it. And **step 5 is the
only thing that ends the launch**: the crate reaches no conclusion of its own
about whether the child finished its work.

The word `relaunch` in step 5 is grove's. This crate wrote the path, watched for
the file and read the line back, and at no point did it look at what the line
said. Chapter 6 is where that is a design decision with a stated reason rather
than an omission.

<a id="the-cast"></a>
## The cast

`run_observed` accepts the same `Launch` plus a synchronous callback. Its
`LaunchEvent` distinguishes successful spawn (Started) from confirmed reap
(Reaped); `run` supplies a no-op callback. Chapter 7 owns the API and chapter 8
shows why token appearance is not reap.

The final module declarations and exports put the public surface in one place.
This book reads them here rather than deferring each name to its own chapter, because the
list is short and the map above has already said which chapter owns what.

<!-- fragment «library-root-modules-and-exports» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="68-85" parent="library-root" -->
````rust

pub mod conformance;

mod argv;
mod channel;
mod error;
mod run;
mod templates;
mod vocabulary;

pub use argv::{Argv, Slot};
pub use channel::{signal, Channel, Token};
pub use error::{ConfigError, Diagnostic, LaunchError, Occurrence};
pub use run::{
    reraise, run, run_observed, take_interrupt, End, Ended, Escalation, Launch, LaunchEvent,
};
pub use templates::{Catalog, Selection, Source, SourceRole, SourceSpan, Templates};
pub use vocabulary::{Requirement, SlotRule, Vocabulary};
````
<!-- /fragment -->

Seven modules, one of them public. `conformance` is public because a consumer's
own test suite calls into it; the other six are private, and everything a
consumer touches from them is re-exported by the six `pub use` lines. That shape
is what makes the seam of the fourth fragment enforceable rather than
conventional. `argv` is a private module, so the only thing outside this crate
can see of it is what `pub use argv::{Argv, Slot}` publishes — the two types, and
none of the constructor. Chapter 5 reads the line that makes `Argv::new`
`pub(crate)` and counts its callers.

Every name in that block belongs to a later chapter except `ConfigError` and
`LaunchError`, which this chapter reads next. The table below states the minimum a
reader needs to follow this chapter, and nothing more; each row's owning chapter
is where the full account lives. Every row but the last is an **early use** — a
name this page must state a minimum for because its owner is ahead of it — and
the source index carries those seven verbatim as the book's early-use ledger. The
last row is this chapter's own and is not one.

| Names | Minimum statement | Chapter |
|---|---|---:|
| `Catalog`, `Selection`, `SourceRole`, `Source`, `SourceSpan` | Catalog owns captured documents and vocabulary; Selection supplies the explicit profile list and optional source origin; SourceRole, Source and SourceSpan identify that origin. | 2 |
| `Templates` | One loaded configuration: key to complete command template, compiled against a vocabulary and validated whole before anything is spawned. | 2 |
| `Vocabulary`, `SlotRule`, `Requirement` | The slot names a consumer's templates are written against, each with a cardinality; supplied at load, because every template rule is a rule about a slot's name. | 2 |
| `Argv`, `Slot` | `Argv` is a program and its arguments with no public constructor, authored only by `Templates::expand`; `Slot` is one name-and-value a caller offers to that call. | 5 |
| `Channel`, `Token`, `signal` | A fresh path per launch that allocation picks and writes nothing to; `signal` is what the child calls to make it appear, and `Token` is what the caller reads back. | 6 |
| `run`, `Launch`, `Ended`, `End`, `Escalation` | `run` spawns one `Launch` — argv, channel, scrub list, working directory and the two graces of an `Escalation` — and returns an `Ended` saying which of `End`'s three cases happened. | 7 |
| `reraise`, `take_interrupt` | The launcher's own two obligations for a termination signal: `take_interrupt` collects one that arrived between launches, and `reraise` is how a launcher dies of the same signal rather than reporting an exit code. | 8 |
| `conformance::check` | The kit that holds a consumer's configuration to this crate's contract from outside the consumer's own suite. | 9 |
| `ConfigError`, `Diagnostic`, `Occurrence`, `LaunchError` | structured configuration refusals and separate launch errors, read next | 1 |

The last row is the only one this chapter owns, and it is the last file this
chapter reads.

<a id="the-two-errors"></a>
## Two opaque errors, and why there are two

`ConfigError` retains structured Diagnostic records and a human rendering.
`LaunchError` remains a message-only error for channel and process failures.
Both implement Display, Debug and Error without an error-library dependency.

<!-- fragment «two-opaque-errors» owner="understands-neither" source="crates/keyed-launch/src/error.rs" lines="1-148" parent="source-error-types" -->
<!-- insert «error-import» -->
<!-- insert «error-config-type» -->
<!-- insert «error-config-traits» -->
<!-- insert «error-launch-type» -->
<!-- insert «error-launch-traits» -->
<!-- /fragment -->

The formatting import supports both error types. Source and SourceSpan come
from the captured configuration model, allowing diagnostics to identify files
and byte ranges without importing the launch half.

<!-- fragment «error-import» owner="understands-neither" source="crates/keyed-launch/src/error.rs" lines="1-1" parent="two-opaque-errors" -->
````rust
use std::fmt;
````
<!-- /fragment -->

Diagnostic owns a stable category, message, remedy, optional source and primary
span, related spans, affected names and an occurrence chain. ConfigError exposes
these through `diagnostics()` while keeping its construction private.
`from_diagnostics` derives the human rendering from the same records, so consumers
can choose structured access without parsing prose. `contextualize` supplies a
known source and key to runtime errors without inventing a source span.
Occurrence records identify each selected entry, including an unknown external
profile and its list index; absent selection origins stay absent. The binding,
command and parameter names are reserved for later modular resolution.
Chapter 4 explains how the flat validator supplies locations and categories.

<!-- fragment «error-config-type» owner="understands-neither" source="crates/keyed-launch/src/error.rs" lines="2-93" parent="two-opaque-errors" -->
````rust

use crate::templates::{Source, SourceSpan};

/// One selected/include occurrence, including an unresolved selection in an error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Occurrence {
    pub id: usize,
    pub profile: String,
    pub parent: Option<usize>,
    pub selection_index: usize,
    pub via: Option<SourceSpan>,
}

/// A stable machine-readable refusal with the locations and names available at
/// the failing operation. Byte ranges refer to the captured UTF-8 source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub category: String,
    pub message: String,
    pub source: Option<Source>,
    pub primary: Option<SourceSpan>,
    pub related: Vec<SourceSpan>,
    pub occurrence_chain: Vec<Occurrence>,
    pub key: Option<String>,
    pub binding: Option<String>,
    pub command: Option<String>,
    pub parameter: Option<String>,
    pub remedy: String,
}

impl Diagnostic {
    pub(crate) fn new(category: &str, message: impl Into<String>, remedy: &str) -> Self {
        Self {
            category: category.to_owned(),
            message: message.into(),
            source: None,
            primary: None,
            related: Vec::new(),
            occurrence_chain: Vec::new(),
            key: None,
            binding: None,
            command: None,
            parameter: None,
            remedy: remedy.to_owned(),
        }
    }
}

/// Reading, validation, resolution or expansion failed. Display is for humans;
/// [`Self::diagnostics`] exposes stable categories without parsing that prose.
/// The error remains opaque and implements `std::error::Error` without imposing
/// an error-handling dependency on its consumers.
pub struct ConfigError {
    message: String,
    diagnostics: Vec<Diagnostic>,
}

impl ConfigError {
    pub(crate) fn new(category: &str, message: impl Into<String>, remedy: &str) -> Self {
        Self::from_diagnostics(vec![Diagnostic::new(category, message, remedy)])
    }

    pub(crate) fn from_diagnostics(diagnostics: Vec<Diagnostic>) -> Self {
        let message = diagnostics
            .iter()
            .map(|d| format!("{}\n  {}", d.message, d.remedy))
            .collect::<Vec<_>>()
            .join("\n");
        Self {
            message,
            diagnostics,
        }
    }

    pub(crate) fn contextualize(mut self, source: Option<Source>, key: Option<&str>) -> Self {
        for diagnostic in &mut self.diagnostics {
            diagnostic.source.clone_from(&source);
            diagnostic.key = key.map(str::to_owned);
        }
        self
    }

    pub(crate) fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    /// Independent refusals in source-role, byte-position and key order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}
````
<!-- /fragment -->

Display and Debug both render the human report, including remedies. Debug
deliberately avoids a record dump in panic output and error chains; callers
wanting structured records use `diagnostics()` explicitly.

<!-- fragment «error-config-traits» owner="understands-neither" source="crates/keyed-launch/src/error.rs" lines="94-109" parent="two-opaque-errors" -->
````rust

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// The message, not a struct dump: a `{:?}` of this error is read by a human in
/// a panic or an `anyhow` chain, and a record dump would obscure the human report.
impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ConfigError {}
````
<!-- /fragment -->

The second type is where the seam this chapter opened on becomes visible in the
source. `LaunchError` covers allocating a channel, spawning a child and
supervising one, and its comment states that being a *separate* type is
deliberate: **a caller that only loads and expands a configuration never handles
a spawn failure, and one that only launches never handles a KDL parse error.** The
sentence that follows is the argument this chapter has been building toward —
two types keep the two halves of this crate usable apart, which is the whole
claim `Templates` and `run` make by not referring to each other.

<!-- fragment «error-launch-type» owner="understands-neither" source="crates/keyed-launch/src/error.rs" lines="110-133" parent="two-opaque-errors" -->
````rust

/// Everything that can go wrong allocating a channel, spawning a child, or
/// supervising one.
///
/// **Opaque for the same reason [`ConfigError`] is**, and deliberately a
/// *separate* type rather than a shared one: a caller that only loads and
/// expands a configuration never handles a spawn failure, and one that only
/// launches never handles a KDL parse error. Two types keep the two halves of
/// this crate usable apart — which is the whole claim `Templates` and `run`
/// make by not referring to each other.
///
/// Its obligation is the same: name what is wrong, name where, and name what
/// would fix it.
pub struct LaunchError {
    message: String,
}

impl LaunchError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
````
<!-- /fragment -->

The last fragment is `LaunchError`'s three trait implementations, and it closes
the file by pointing at the first type rather than repeating its argument. The
`Debug` comment is one line and defers to `ConfigError`'s, which is the same
economy the crate applies to its errors: state the reason once, in one place, and
name that place from everywhere else.

<!-- fragment «error-launch-traits» owner="understands-neither" source="crates/keyed-launch/src/error.rs" lines="134-148" parent="two-opaque-errors" -->
````rust

impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// The message, not a struct dump — see [`ConfigError`]'s `Debug` for why.
impl fmt::Debug for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for LaunchError {}
````
<!-- /fragment -->

Two types, then, and the count is the claim. One error type would have been
shorter and would have compiled identically; what it would have cost is the
property the next four chapters and the four after them rely on — that a reader
can follow the configuration half without meeting a spawn, and the launch half
without meeting a parse. The error module shows that property before any of the
code that keeps it.

Chapter 2 takes the first step into the configuration half, and it takes it at
the argument the library root stated in five lines: the vocabulary is an input to
`load`, not to `expand`.

[Contents](README.md) | [Next: The names a template is written against](02-the-names.md)
