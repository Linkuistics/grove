# Orientation
<!-- book-page id="orientation" slice="understands-neither" order="1" -->
[Contents](README.md) | [Next: Appearance is the event](02-the-channel.md)

<a id="understands-neither"></a>
## Understands neither

`keyed-launch` sits between a command its caller built and a running process.
The caller hands it a program and its arguments, a directory for one file, and
the name of one environment variable. The crate spawns the program, watches for
the file, stops the child when the file appears, and reports how the launch
ended.

Between those two points it learns nothing about what it is serving. It does not
know what the program does, and it does not know what the child's completion
token says. So it cannot decide that the child's work is done. It can only
observe, and this book is for one question a reader can take to a layer of
their own: *what ends a launch, and who decides?* The crate observes three
things. Chapters 2 to 4 build them, and chapter 6 says what breaks without each
and names the one ending none of them reaches.

Four files declare what the crate does not know: the manifest, which buys one dependency and
no domain; the library root, which states the claim in its opening lines and
maps the rest of the crate; the error module, whose one type is a message; and
`src/argv.rs`, the type a command arrives in.

The crate keeps the name it took when it also resolved a key to a command
template. That half is gone. The crate reads no configuration, and nothing in it
chooses what a caller launches.

Grove is the consumer this crate was extracted from. Its loop builds one argv
for each session and its standalone command builds another, and both hand the
result here. What a session is and what a launch is *for* are the guide's, and
the `README.md` points there so that no chapter has to explain them.

<a id="the-package"></a>
## The package: one dependency and no domain

The manifest is production source and this chapter reconstructs all forty-two
lines of it, in five fragments that follow the file's own blocks. It is read
first because the crate's central claim is checkable there before any Rust is
read: a crate that understood what it was launching would need a dependency that
knew something about the domain, and this one buys a single dependency that
knows nothing.

<!-- fragment «manifest-one-dependency» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="1-42" parent="source-crate-manifest" -->
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
inherited are `name` and `description`, and the `description` is the crate in
one line: a program and its arguments, spawned as a job and supervised until it
ends.

<!-- fragment «manifest-package-identity» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="1-10" parent="manifest-one-dependency" -->
````toml
[package]
name = "keyed-launch"
version.workspace = true
edition.workspace = true
description = "A program and its arguments, spawned as a job and supervised until it ends"
license.workspace = true
repository.workspace = true
# Inherited from the workspace root, whose comment carries the evidence: the
# locked dependency graph cannot be parsed by a cargo below 1.85.
rust-version.workspace = true
````
<!-- /fragment -->

The second fragment is the chapter's evidence. `libc` supplies the system calls
`std` does not expose, and it is the only dependency. The comment argues it by
naming the two calls the *escalation* needs, `kill(2)` and `signal(2)`; the crate
reaches further into `libc` than that, for the process-group and terminal calls
chapter 3 owns and the confinement calls chapter 7 owns. The comment also states
what is *not* bought — an error library — and records that
`crates/jj-workspace` and `crates/ordinal-fs-tree` state the same rule for
theirs.

<!-- fragment «manifest-dependencies» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="11-21" parent="manifest-one-dependency" -->
````toml

# **One dependency: the syscalls this crate cannot reach from `std`.** `libc`
# is the escalation: `std::process::Child::kill` sends SIGKILL and nothing
# else, so a graduated SIGTERM-then-SIGKILL — and catching the launcher's own
# SIGTERM/SIGHUP to forward it — needs `kill(2)` and `signal(2)` directly. The
# error is the crate's own opaque `LaunchError` implementing
# `std::error::Error`, so a consumer using `anyhow`, `thiserror` or nothing at
# all takes on none of these — the same rule `crates/jj-workspace` and
# `crates/ordinal-fs-tree` state for theirs.
[dependencies]
libc = "0.2"
````
<!-- /fragment -->

The `libc` line carries an argument rather than a fact.
`std::process::Child::kill` sends SIGKILL and nothing else, so a launcher that
wants to ask a child to end before insisting cannot express that through the
standard library at all. Chapter 4 is where the graduated ending is built out of
`kill(2)` and `signal(2)`; the manifest is where the need for them is declared.

<a id="the-dev-dependency"></a>
## One dev-dependency, and the lints

The dev-dependencies table names one dependency. `tempfile` is what four of the
crate's five test files use to build a directory of scripts and channel files
per test, and it is the only thing the suite needs that the standard library
does not supply; the fifth, `tests/reraise.rs`, re-executes itself and needs no
directory at all. A crate that spawns processes and writes files might be
expected to buy a process harness or a fixture framework, and this one buys a
temporary directory.

<!-- fragment «manifest-dev-dependencies» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="22-24" parent="manifest-one-dependency" -->
````toml

[dev-dependencies]
tempfile = "3.10"
````
<!-- /fragment -->

The lints table inherits the workspace's lint configuration rather than declaring
its own. That is the mechanism by which this crate is held to the same clippy and
rustc settings as every other member. No `#![allow]` appears in the library root
this chapter reads next, and none appears anywhere in the crate.

<!-- fragment «manifest-lints» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="25-27" parent="manifest-one-dependency" -->
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

<!-- fragment «manifest-release» owner="understands-neither" source="crates/keyed-launch/Cargo.toml" lines="28-42" parent="manifest-one-dependency" -->
````toml

# `cargo release` cuts *grove* (`crates/grove`), and `release.toml` configures
# that cut. `release = false` here means no tag, no changelog section and no
# publish of its own. It does **not** mean a frozen version: this crate takes
# `version.workspace = true`, so a cut moves it with every other crate the
# release ships — one workspace, one release version
# (`docs/specs/module-decomposition.md`, decision 1).
#
# **This crate is not published on its own, and that is settled**
# (`docs/RELEASING.md`, *One release, eight packages, one tag*): it ships inside
# grove's cut, wearing grove's version, and no library member has a release lane
# of its own. Removing this line does not reopen the question — it corrupts the
# cut, which was measured rather than assumed.
[package.metadata.release]
release = false
````
<!-- /fragment -->

The comment's second paragraph states that this is an answered question rather
than an open one, and names where the answer lives: `docs/RELEASING.md`, under
*One release, eight packages, one tag*, which settles that no library member of
this workspace has a release lane of its own. That matters to a reader of this
book in one specific way. A crate whose publication was undecided would have an
argument for a stable, documented error taxonomy, because downstream users
outside this repository would be matching on it. This one ships inside grove's
cut, and the error module reads differently in that light.

<a id="the-map"></a>
## The map: the library root

`src/lib.rs` is forty-seven lines and thirty-two of them are the module's
documentation. It is the crate's own account of itself, and this book's chapter
order is that account's order. The book reads it whole here, in five fragments:
four that follow the doc comment's own paragraph breaks, and one for the module
declarations and exports, which this chapter reads after the worked example.

<!-- fragment «library-root» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="1-52" parent="source-library-root" -->
<!-- insert «library-root-thesis» -->
<!-- insert «library-root-to-a-child» -->
<!-- insert «library-root-job-and-out-of-band» -->
<!-- insert «library-root-observed» -->
<!-- insert «library-root-modules-and-exports» -->
<!-- /fragment -->

The first fragment is the spine. The claim has two halves. What the crate does
not own is stated by enumeration: it chooses no program, reads no configuration
and does not understand what the child is for, and every word of the command,
every variable name and every path is the caller's. What it does own is stated
positively: how the child is spawned, how its end is learned, and how it is
stopped. Chapter 3 takes the first, chapter 2 the second and chapter 4 the
third.

<!-- fragment «library-root-thesis» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="1-8" parent="library-root" -->
````rust
//! A program and its arguments, spawned as a **job** and supervised until it
//! ends.
//!
//! The caller builds an [`Argv`] and this crate launches it. Nothing here
//! chooses a program, reads a configuration or understands what the child is
//! for: every word of the command, every variable name and every path is the
//! caller's. What the crate owns is how the child is spawned, how its end is
//! learned, and how it is stopped.
````
<!-- /fragment -->

The second fragment is the crate's one headed section. `Argv::new` takes a
program and its arguments, each string one whole word, and `run` spawns them
directly, with no shell. This chapter's last section reads that type.

<!-- fragment «library-root-to-a-child» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="9-14" parent="library-root" -->
````rust
//!
//! # From an argv to a running child
//!
//! [`Argv::new`] takes a program and its arguments, each string one whole
//! word; [`run`] spawns them directly, with no shell, and supervises the
//! child until it ends.
````
<!-- /fragment -->

The third fragment carries the two bold paragraphs that are chapters 3 and 4's
theses and chapter 2's. **The child is a job**: it is spawned into a process
group of its own and handed the launcher's controlling terminal, and the
paragraph gives three consequences of that rather than describing the mechanism,
then the promise that closes every launch: the whole group ends with it, the
terminal comes back in the modes it was lent in, and a group that survives is
reported as `Group::Present` beside the child's status.
**A launch ends out of band**: an interactive child returns to its prompt when it
finishes rather than exiting, so its own exit is not the event anyone is waiting
for, and the channel's *appearance* is. Those two sentences are the reason the
crate has a `Channel` at all, and chapter 2 is where the appearance rule is built.

<!-- fragment «library-root-job-and-out-of-band» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="15-30" parent="library-root" -->
````rust
//!
//! **The child is a job.** It is spawned into a process group of its own and
//! handed the launcher's controlling terminal, so a terminal signal reaches the
//! child rather than the launcher, the escalation can reap a grandchild the
//! child spawned, and a launcher's own ignored dispositions are not inherited
//! across the `exec` by every wrapper the command names. Whatever ends the
//! child, its whole group ends with the launch, and the terminal comes back
//! with the modes it was handed over in. A group that survives is reported as
//! [`Group::Present`] beside the child's status. See [`run`].
//!
//! **A launch ends out of band.** An interactive child returns to its prompt
//! when it finishes rather than exiting, so its own exit is not the event
//! anyone is waiting for. [`Channel`] is: a fresh path per launch that the
//! child writes a [`Token`] to (through [`signal`]) when it is done, and whose
//! *appearance* starts the kill [`Escalation`] the child cannot perform on
//! itself. See [`Escalation`] for why that is the launcher's job.
````
<!-- /fragment -->

The fourth fragment names the observed entry point. `run_observed` reports
successful spawn and confirmed reap to its caller as they happen, and `run` is
the same launch without an observer. Chapter 3 owns the interface and chapter 4
shows why a token appearing is not a reap.

<!-- fragment «library-root-observed» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="31-35" parent="library-root" -->
````rust
//!
//! [`run_observed`] adds synchronous parent-side [`LaunchEvent`] notifications
//! at successful spawn and confirmed reap, including reap during wait-error
//! recovery. Notifications precede token reading and terminal recovery; failed
//! spawn emits none. [`run`] keeps the same interface without an observer.
````
<!-- /fragment -->

The table below is where a reader checks that this book's order follows the
crate's own account. The out-of-band paragraph is **split**: it argues both why
there is a channel, which is chapter 2, and why ending the child is the
launcher's job, which is chapter 4.

| `src/lib.rs` section | Lines | Chapter |
|---|---:|---:|
| the opening thesis | 1–8 | 1 |
| *From an argv to a running child* | 10–14 | 1 |
| **The child is a job** | 16–23 | 3, and the end of the group in 4 |
| **A launch ends out of band** | 25–30 | 2, and the escalation in 4 |
| `run_observed` | 32–35 | 3 |

The library root says nothing about a child with no terminal or about
confinement. Chapter 7 reads both.

<a id="the-launch-in-outline"></a>
## The launch in outline

The example this book carries is one of grove's launches, told strictly from the
crate's side. The reader knows what these words mean; the point of the example
is watching the crate not care. It starts here at low resolution — every call
named, no handler shown — and each later chapter takes one step of it apart. Its
values are fixed here and reused by every chapter that follows.

The launch starts with a command the crate did not choose. The caller holds a
program and three arguments, the last of which is a long prompt with spaces and
newlines in it. The trace below is the book's spine in one column: each step
names the chapter that owns it, and no step shows a line of the handler behind
it.

```text
1  Argv::new("claude", ["--model", "opus", "<the mandate>"])    chapter 1
     -> Argv ["claude", "--model", "opus", "<the mandate>"]
     four words go in and the same four words are held

2  Channel::allocate(Path::new("/work/atlas/.jj/grove"))       chapter 2
     -> /work/atlas/.jj/grove/signal-3f9c1d4a7b2e5086c1a4f70d93b6e281
     the path is drawn; nothing is written

3  run(Launch { argv, channel, channel_var: "GROVE_SIGNAL_FILE",
                scrub, cwd: Some("/work/atlas"), escalation })  chapters 3, 4
     child spawned in its own process group, holding the terminal,
     with GROVE_SIGNAL_FILE set to the path from step 2

4  the child writes "relaunch\n" to that path and returns to its prompt
     the file appears; the grace elapses; the child is signalled
     -> Ended { end: Signalled, status, elapsed, token: Some("relaunch") }
```

Three properties of that trace are worth naming now, because they are what the
later chapters prove. **Step 1 produces the one kind of value step 3 accepts**:
`run` takes an `Argv` and nothing else that could name a program. **Step 2
writes nothing**, so the file's later existence is unambiguous evidence that
something wrote it. And **step 4 is the only thing that ends the launch**: the
crate reaches no conclusion of its own about whether the child finished its
work.

The word `relaunch` in step 4 is grove's. This crate wrote the path, watched for
the file and read the line back, and at no point did it look at what the line
said. Chapter 2 is where that is a design decision with a stated reason rather
than an omission.

<a id="the-cast"></a>
## The cast

The final module declarations and exports put the public surface in one place.
This book reads them here rather than deferring each name to its own chapter,
because the list is short and the map above has already said which chapter owns
what.

<!-- fragment «library-root-modules-and-exports» owner="understands-neither" source="crates/keyed-launch/src/lib.rs" lines="36-52" parent="library-root" -->
````rust

mod argv;
mod channel;
mod confinement;
mod error;
mod run;

pub use argv::Argv;
pub use channel::{signal, Channel, Token};
pub use confinement::{
    confinement_available, confinement_system_reads, regular_file_at, FilesystemGrants,
};
pub use error::LaunchError;
pub use run::{
    reraise, run, run_confined_observed, run_noninteractive, run_observed, take_interrupt, End,
    Ended, EntrySignals, Escalation, Group, Launch, LaunchEvent, NoninteractiveLaunch,
};
````
<!-- /fragment -->

Five modules, all private. Everything a consumer touches is re-exported by the
five `pub use` declarations. `argv` is a private module, so all that anything
outside this crate can see of it is the one type `pub use argv::Argv` publishes,
with its fields private and its constructor public.

Every name in that block belongs to a later chapter except `LaunchError` and
`Argv`, which this chapter reads next. The table below states the minimum a
reader needs to follow this chapter, and nothing more; each row's owning chapter
is where the full account lives. Every row but the last is an **early use** — a
name this page must state a minimum for because its owner is ahead of it — and
the source index carries the required statements as the book's early-use ledger.
The last row is this chapter's own and is not one.

| Names | Minimum statement | Chapter |
|---|---|---:|
| `Channel`, `Token`, `signal` | A fresh path per launch that allocation picks and writes nothing to; `signal` is what the child calls to make it appear, and `Token` is what the caller reads back. | 2 |
| `run`, `run_observed`, `LaunchEvent`, `Launch`, `EntrySignals`, `Ended`, `End`, `Group`, `Escalation` | `run_observed` reports successful spawn and confirmed reap synchronously; `run` uses a no-op observer. Each spawns one `Launch` — argv, channel, scrub list, granted values, a transparent caller's `EntrySignals` when there is one, working directory and the two graces of an `Escalation` — and returns an `Ended` saying which of `End`'s three cases happened, whether the channel appeared, and whether the child's `Group` was confirmed gone. | 3 |
| `reraise`, `take_interrupt` | The launcher's own two obligations for a termination signal: `take_interrupt` collects one that arrived between launches, and `reraise` is how a launcher dies of the same signal rather than reporting an exit code. | 4 |
| `run_noninteractive`, `run_confined_observed`, `NoninteractiveLaunch`, `FilesystemGrants`, `confinement_available`, `confinement_system_reads`, `regular_file_at` | Detached launches with file or inherited output, mandatory filesystem grants, backend availability and system-read inventory, and stable reads of artifacts through held directories. | 7 |
| `LaunchError`, `Argv` | the one error type and the type a command arrives in, read next | 1 |

<a id="one-opaque-error"></a>
## One opaque error

`LaunchError` is a message. It covers allocating a channel, spawning a child
and supervising one, and it implements `Display`, `Debug` and `Error` without an
error-library dependency.

<!-- fragment «one-opaque-error» owner="understands-neither" source="crates/keyed-launch/src/error.rs" lines="1-56" parent="source-error-type" -->
<!-- insert «error-import» -->
<!-- insert «error-launch-type» -->
<!-- insert «error-launch-traits» -->
<!-- /fragment -->

The file opens with its one import, the formatting traits both implementations
below are written against.

<!-- fragment «error-import» owner="understands-neither" source="crates/keyed-launch/src/error.rs" lines="1-1" parent="one-opaque-error" -->
````rust
use std::fmt;
````
<!-- /fragment -->

The type's comment states the opacity and the obligation that replaces a
variant list. A variant list would be a second interface, and every new refusal
a breaking change. What the type owes instead is a message that names what is
wrong, names where, and names what would fix it. The constructor is
`pub(crate)`, so only this crate writes one, and every message in chapters 2 to
4 and 7 can be read against that obligation.

One fact rides beside the message, and only one. When the child could not be
spawned, the error keeps the system's error number, and `raw_os_error` answers
it; every other failure answers `None`. A caller that reports a failed start by
its cause needs the number rather than the words: `harness-dispatch` exits 127
for `ENOENT`, as a shell does for a missing program, and 126 for any other
cause. `Some` therefore also says that nothing was started. `with_errno` is how
the spawn in chapter 3 attaches it, and it is `pub(crate)` like the
constructor.

<!-- fragment «error-launch-type» owner="understands-neither" source="crates/keyed-launch/src/error.rs" lines="2-40" parent="one-opaque-error" -->
````rust

/// Everything that can go wrong allocating a channel, spawning a child, or
/// supervising one.
///
/// **Opaque**: the message is the interface, and the type implements
/// `std::error::Error` without imposing an error-handling dependency on its
/// consumers.
///
/// Its obligation is to name what is wrong, name where, and name what would
/// fix it.
pub struct LaunchError {
    message: String,
    errno: Option<i32>,
}

impl LaunchError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            errno: None,
        }
    }

    pub(crate) fn with_errno(mut self, errno: Option<i32>) -> Self {
        self.errno = errno;
        self
    }

    /// The system's error number when the child could not be spawned, and
    /// `None` for every other failure.
    ///
    /// A caller that reports a failed start by its cause, as a shell's 127 for
    /// a missing program and 126 for any other, needs the number and not the
    /// message. Nothing was started when this is `Some`.
    #[must_use]
    pub fn raw_os_error(&self) -> Option<i32> {
        self.errno
    }
}
````
<!-- /fragment -->

`Display` and `Debug` both render the message. `Debug` deliberately avoids a
struct dump, because a `{:?}` of this error is read by a human in a panic or an
error chain.

<!-- fragment «error-launch-traits» owner="understands-neither" source="crates/keyed-launch/src/error.rs" lines="41-56" parent="one-opaque-error" -->
````rust

impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// The message, not a struct dump: a `{:?}` of this error is read by a human in
/// a panic or an `anyhow` chain, and a record dump would obscure the human report.
impl fmt::Debug for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for LaunchError {}
````
<!-- /fragment -->

<a id="the-argv"></a>
## The type a command arrives in

`src/argv.rs` is fifty-three lines and one type. It is the only way a program
reaches `run`.

<!-- fragment «argv» owner="understands-neither" source="crates/keyed-launch/src/argv.rs" lines="1-64" parent="source-argv" -->
<!-- insert «argv-type» -->
<!-- insert «argv-public-constructor» -->
<!-- insert «argv-program-and-args» -->
<!-- insert «argv-words» -->
<!-- /fragment -->

`Argv` has three private fields, and its doc comment says what holds for every
value of the type. The third, `arg0`, is unset unless a caller asks for it.

<!-- fragment «argv-type» owner="understands-neither" source="crates/keyed-launch/src/argv.rs" lines="1-12" parent="argv" -->
````rust
use std::ffi::{OsStr, OsString};

/// A program and its arguments, in order, ready to spawn.
///
/// There is no shell and no second reading of any word: each is spawned as it
/// is given.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Argv {
    program: OsString,
    args: Vec<OsString>,
    arg0: Option<OsString>,
}
````
<!-- /fragment -->

*There is no shell and no second reading of any word.* That is a promise about
what happens to the words, and it says nothing about where they came from. The
fields are `OsString` and not `String`, which is the one choice in this file
that needs a reason. An argument to a process is a byte string on Unix and need
not be UTF-8; a path very often is not. A crate that took `String` here would
refuse to launch on a filename it had no business having an opinion about.

<!-- fragment «argv-public-constructor» owner="understands-neither" source="crates/keyed-launch/src/argv.rs" lines="13-36" parent="argv" -->
````rust

impl Argv {
    /// A program and arguments the caller built, each string one whole word.
    #[must_use]
    pub fn new(program: OsString, args: Vec<OsString>) -> Self {
        Self {
            program,
            args,
            arg0: None,
        }
    }

    /// The same launch with `arg0` as the child's `argv[0]` in place of the
    /// program. The program is still what is spawned.
    ///
    /// For a caller that resolved a name to a path itself: it spawns the path,
    /// so that nothing is looked up a second time, and the child still sees
    /// the name it was chosen by. A confined launch runs the path it is given,
    /// and its child's `argv[0]` is that path whatever this says.
    #[must_use]
    pub fn with_arg0(mut self, arg0: OsString) -> Self {
        self.arg0 = Some(arg0);
        self
    }
````
<!-- /fragment -->

`Argv::new` is public, and it checks nothing: a program and a list of arguments
go in, and the same program and arguments are what the value holds. Grove calls
it in two places. `crates/grove-loop/src/loop_driver.rs` builds the
`harness-dispatch run` invocation for a lifecycle session, and
`crates/grove/src/standalone.rs` builds a noninteractive `harness-dispatch run --confine` invocation from its canonical
sibling path, kind, prompt, runtime reads and ending-file path. The third caller is
`harness-dispatch run` itself, which builds the harness's command from what its
owner's policy selected.

That third caller is why `with_arg0` exists. Dispatch has already resolved the
program the policy named to an absolute path, and spawns that path, so that
nothing is looked up a second time under another directory or `PATH`. The
harness is still to see, as its `argv[0]`, the program as the policy wrote it.
`with_arg0` sets that word and leaves the program alone. A confined launch runs
the path through the sandbox's own launcher, which gives its child the path as
`argv[0]`, so the doc comment says the word has no effect there.

So the type does not say who authored the words. What it carries is narrower.
The fields are private and no method changes one, and `run` in chapter 3 takes
an `Argv` and nothing else that could name a program. Whatever was put in is
what is spawned, each string one argument, and the caller answers for the
words. `a_caller_built_argv_is_spawned_whole_and_directly` in
`crates/keyed-launch/tests/launch.rs` pins it with five words a shell would
re-read: a value with spaces, text shaped like a variable, quotes, a newline
and an empty string each reach the child as one argument.

The three accessors are `run`'s whole view of the value.

<!-- fragment «argv-program-and-args» owner="understands-neither" source="crates/keyed-launch/src/argv.rs" lines="37-53" parent="argv" -->
````rust

    #[must_use]
    pub fn program(&self) -> &OsStr {
        &self.program
    }

    /// The child's `argv[0]`: the program, unless [`Argv::with_arg0`] named
    /// another.
    #[must_use]
    pub fn arg0(&self) -> &OsStr {
        self.arg0.as_deref().unwrap_or(&self.program)
    }

    #[must_use]
    pub fn args(&self) -> &[OsString] {
        &self.args
    }
````
<!-- /fragment -->

`program` and `args` are separated because that is the shape a spawn wants:
`Command::new` takes the program and `args` takes the rest, and word zero is
special to the operating system rather than to this crate. `arg0` is that word
zero: the one `with_arg0` named, or the program. All three return borrows and
all three are `#[must_use]`. Fourteen functions in the crate carry the
attribute. Thirteen return a value that is the only reason to call them; the
fourteenth, `take_interrupt` in `src/run.rs`, is not — it *clears* a latch as it
reads it, and chapter 4 is where that difference matters.

The last method is the same value in the other shape.

<!-- fragment «argv-words» owner="understands-neither" source="crates/keyed-launch/src/argv.rs" lines="54-64" parent="argv" -->
````rust

    /// The whole launch as one word list, program first — the shape a
    /// `Command`-building consumer and a diagnostic both want.
    #[must_use]
    pub fn words(&self) -> Vec<OsString> {
        let mut words = Vec::with_capacity(self.args.len() + 1);
        words.push(self.program.clone());
        words.extend(self.args.iter().cloned());
        words
    }
}
````
<!-- /fragment -->

`words` is a program and its arguments as one list, program first, and it
allocates: it clones every `OsString` rather than borrowing, because there is no
contiguous slice in the struct to borrow. No production code in this workspace
calls it. Its one caller is a test in `crates/grove-loop/src/loop_driver.rs`
that wraps a launch in `env`. The code that spawns passes the `Argv` itself
into `run`, so no intermediate word list exists for a later edit to change.

An argv now exists and has nowhere to go. Chapter 2 allocates the path whose
*appearance* will be the only thing that ends the launch, and chapters 3 and 4
spawn the child and watch for that appearance.

[Contents](README.md) | [Next: Appearance is the event](02-the-channel.md)
