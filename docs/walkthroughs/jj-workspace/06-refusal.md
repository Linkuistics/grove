# Refusal
<!-- book-page id="refusal" slice="no-remedy-of-its-own" order="6" -->
[Previous: Scope and commit](05-scope-and-commit.md) | [Contents](README.md) | [Next: What jj owns](07-what-jj-owns.md)

<a id="no-remedy-of-its-own"></a>
## It speaks for jj, and never for you

Every operation in the five chapters before this one returns
`Result<_, Refusal>`, and every one of those chapters printed a refusal's message
without reading the type behind it. This chapter reads it. Two hundred and thirty
lines, one public struct, one private enum, and two trait implementations — the
largest single ownership block in the book, and the last one it has.

The sixth refusal is what the file declines to say. **The crate has no consumer
to speak for**, so it never tells a caller what to do about a stop; it names what
jj offers and stops there. The distinction is checkable on any line of the file.
*Make the tree jj-enabled and rerun: `jj git init --colocate`* is a statement
about jj's offer, and it is true whoever is calling. *Abort the session and
report to the operator* would be a statement about one consumer's policy, and the
crate has no way to know whether it is true. The first shape is in this file ten
times. The second shape is in it zero times.

That is a narrower claim than *the crate is polite about errors*, and it is the
one this chapter has to defend, because the file does contain remedies. Two of
them are two-line command listings, and one of those two is the most-read output
this crate produces. The defence is that each remedy is a jj command whose effect
is the same for every caller, offered rather than performed: **the crate runs no
recovery of its own**, and the one refusal that could plausibly have run one says
so in its own last line.

Most of the file's non-comment bytes are that user-facing text, sitting inside
string literals. The fragment graph therefore puts the messages on the page
verbatim without the prose having to quote them, and what the prose owes is the
part a message cannot state about itself: why the type is opaque, what a consumer
gets in place of matchable variants, and which alternative was rejected at each
of those choices.

<a id="the-premise"></a>
## The premise: a cause chain, and a repair that belongs to jj

Two behaviours outside this crate carry the chapter, and a reader who disputes
its design is disputing one of them rather than disputing the file.

**`Error::source` is the standard library's cause chain, one link at a time.**
`fn source(&self) -> Option<&(dyn Error + 'static)>` returns the immediate cause
of an error and nothing further; a consumer that wants the whole chain walks it
by calling `source` on each link in turn
([`std::error::Error::source`](https://doc.rust-lang.org/std/error/trait.Error.html#method.source)).
The chain is therefore made of `Error` values. Text that an error merely quotes
is not part of it, and that distinction decides how much structure this crate's
refusals actually hand a consumer — it is measured in the worked example below
rather than assumed here.

**jj's operation log is the transaction record, and `jj undo` reverses the last
operation in it.** [*Scope and commit*](05-scope-and-commit.md#the-premise) stated
that as the reason the crate implements no transaction; this chapter needs the
second half of it, which is that the repair is a command a person runs. The crate
can name that command in a message. It cannot run it on a caller's behalf without
deciding, for that caller, that undoing is what they wanted.

One consequence of the first premise is worth stating before the code, because it
is the whole of what the opacity costs. A consumer that cannot match on a refusal
can still hand it to a general error type: `anyhow::Error` accepts any `E` that is
`Error + Send + Sync + 'static` through its blanket `From` — checked against the
version this workspace locks, `anyhow` 1.0.102, whose `impl<E> From<E> for Error
where E: StdError + Send + Sync + 'static` is in `src/error.rs` of that release
([anyhow](https://docs.rs/anyhow/1.0.102/anyhow/struct.Error.html)). `Refusal`
carries only `PathBuf`, `String`, `io::Error` and a boxed `Refusal`, so it
satisfies those bounds without a line of the file saying so, and grove converts a
refusal that way at `crates/grove-loop/src/session_config.rs:194`. Opacity does
not cost a consumer its error plumbing; it costs it the ability to branch.

<a id="worked-refusal"></a>
## Worked example: the same attempt, declining

This is the attempt [*Scope and commit*](05-scope-and-commit.md#worked-commit)
traced, with one thing changed about the tree and nothing changed about the call.
The values are the same because the point is the ending, not the scenario.

```text
/work/atlas/                                    the workspace root, canonical
├── .jj/
│   ├── repo/
│   ├── working_copy/
│   └── grove/                                  reserved in chapter 4
├── .grove/
│   ├── BRIEF.md
│   └── 01-DONE-impl--rate-limit-k3.md          renamed: the commit is about this
├── crates/
│   └── gateway/
│       └── src/
│           └── main.rs                         edited, and out of scope
└── unreadable/                                 mode 000, and named by nobody
```

`unreadable/` is the one addition, and it is the same fixture
[*The subprocess seam*](03-subprocess-seam.md#worked-invocation) used: jj
snapshots the whole working copy before it commits any part of it, so a directory
it cannot read fails the command regardless of the fileset the caller named. The
call is byte-for-byte chapter 5's.

```text
workspace.commit(
    &[Path::new(".grove/01-DONE-impl--rate-limit-k3.md")],
    "rate-limit-k3: refuse a request over the burst ceiling",
)

  paths.is_empty()                       -> false
  fileset(…)                             -> "root:\".grove/01-DONE-impl--rate-limit-k3.md\""

  jj::output("/work/atlas", ["commit", "-m", "rate-limit-k3: …", "root:\"…\""])
    status 255, stderr: Internal error: Failed to snapshot the working copy …
    -> Err(Refusal::command_failed("jj commit -m … root:\"…\"",
                                   "/work/atlas",
                                   jj's stderr, trimmed))

  map_err(|cause| Refusal::commit_not_recorded("/work/atlas", cause))
    -> Err(Refusal(CommitNotRecorded {
             root: "/work/atlas",
             cause: Box::new(Refusal(CommandFailed { … })),
         }))
```

The second `jj::output` — the change-id read — is never reached, so the caller
receives one refusal rather than two, and it is the wrapping one. What a consumer
prints is the outer refusal's `Display`, and this is the real output, taken
against jj 0.44.0 through this crate with the tree above and paths shortened to
`/work/atlas`:

```text
the commit did not land in /work/atlas: `jj commit -m rate-limit-k3: refuse a request over the burst ceiling root:".grove/01-DONE-impl--rate-limit-k3.md"` failed in /work/atlas: Internal error: Failed to snapshot the working copy
Caused by:
1: Failed to read directory /work/atlas/unreadable
2: Permission denied (os error 13)

Jujutsu snapshots the working copy before every command and its operation log is the transaction record, so the state before this attempt is still reachable:
      jj undo                    # reverse the snapshot this attempt recorded
      jj op log                  # inspect the operations first, if `jj undo` is not the one

Nothing here runs a recovery of its own.
```

Three claims about that output are checkable and two of them are surprising.

**The `Caused by:` block belongs to jj, not to Rust.** Lines two to four are
inside the `stderr` string `CommandFailed` carries, and they arrived as text on a
file descriptor. Nothing in this crate produced them and nothing in it can walk
them.

**The `source()` chain is exactly one link long.** Walking it from the refusal
the caller holds gives the `CommandFailed` refusal and then `None` — measured
rather than read off the type, by walking `source` to exhaustion on the value
produced above:

```text
e.source()            -> Some(`jj commit -m … root:"…"` failed in /work/atlas: …)
    .source()         -> None
```

So the structure a consumer receives is one wrapper and one wrapped refusal. Two
of jj's own causes are visible in the message and neither is reachable as a
value, because `command_failed` takes jj's stderr as a `&str` rather than as an
error.

**The cause is both interpolated and exposed, so a consumer that prints the chain
prints it twice.** `CommitNotRecorded`'s message contains `{cause}` and its
`source()` returns the same refusal. `anyhow`'s alternate `Display` appends every
link of the chain after the top message, so `{:#}` over this value ends the
remedy paragraph and then repeats the whole `CommandFailed` message after a
colon:

```text
… Nothing here runs a recovery of its own.: `jj commit -m … root:"…"` failed in /work/atlas: Internal error: …
```

That is the measured cost of putting the cause in the message, and the reason it
is paid is in the same measurement: a consumer that prints only `{}` — which is
what a message written for a person is for — would otherwise be shown *the commit
did not land in /work/atlas* with no statement of why. The file chooses the
consumer that prints one string over the consumer that walks a chain. Both are
served; only one is served without redundancy.

The redundancy is reachable from grove rather than hypothetical, and it is
recorded here as an outstanding observable in the way chapters 4 and 5 recorded
theirs. `grove-llm`'s `main` returns `anyhow::Result<()>`
(`crates/grove-llm/src/main.rs:1`), so a refusal that reaches the top is printed
with `anyhow`'s `Debug`: the top message, then `Caused by:`, then a chain whose
one link is text the top message already contains. Four of the ten kinds
interpolate a cause they also return from `source()`, so four of them print it
twice. The corpus is frozen for this book and the fix moves line boundaries
inside arms this chapter quotes, so it must land in one commit with this page.

> **The consumer's half.** grove never names this type. `grove-loop` re-exports
> `Commit` and `Workspace` from this crate and not `Refusal`
> (`crates/grove-loop/src/lib.rs:81`), and its three call sites each do one of the
> three things an opaque error allows. `session_config.rs:194` converts it —
> `Workspace::resolve(worktree).map_err(anyhow::Error::from)` — and lets it print
> as it stands. `crates/grove-llm/src/cli.rs:452` adds the sentence the crate
> could not have written, `.context("cannot commit the finished grove")`, so an
> operator reads what grove was attempting and then what jj said about it.
> `session_config.rs:354` discards it: `let Ok(workspace) = Workspace::resolve(
> directory) else { return Ok(false) }`, because that caller's question is whether
> a configuration delta is tracked, and a directory that is not a workspace
> answers `false` rather than failing. Convert, contextualise, or discard. None of
> the three requires a variant, and grove matches on no refusal this crate
> produces.

<a id="the-opaque-type"></a>
## One opaque value over a private case analysis

The whole file is one ownership block, and it is the only block in the book that
is an entire source root. Its parts are declared here and read in the sections
that follow, in the order the file writes them.

<!-- fragment «refusal-source» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="1-230" parent="source-refusal" -->
<!-- insert «refusal-module-thesis» -->
<!-- insert «refusal-imports» -->
<!-- insert «refusal-opaque-type» -->
<!-- insert «refusal-kind-open» -->
<!-- insert «refusal-kind-gate» -->
<!-- insert «refusal-kind-namespace» -->
<!-- insert «refusal-kind-scope» -->
<!-- insert «refusal-kind-seam» -->
<!-- insert «refusal-kind-commit» -->
<!-- insert «refusal-constructors-gate» -->
<!-- insert «refusal-constructors-namespace» -->
<!-- insert «refusal-constructors-scope» -->
<!-- insert «refusal-constructors-seam» -->
<!-- insert «refusal-constructors-commit» -->
<!-- insert «refusal-display-open» -->
<!-- insert «refusal-display-gate» -->
<!-- insert «refusal-display-namespace» -->
<!-- insert «refusal-display-scope» -->
<!-- insert «refusal-display-seam» -->
<!-- insert «refusal-display-commit» -->
<!-- insert «refusal-error-source-caused» -->
<!-- insert «refusal-error-source-uncaused» -->
<!-- /fragment -->

The first thirteen lines are the module's own statement of the refusal, and they
are the only place in the crate where the word *refusal* is defined rather than
used. The paragraph a reader should weigh is the second: an error that only
reports detection is described as unfinished, and the reason given is that a
caller synthesising its own remedy would have to know jj — which is the thing
taking this crate was meant to stop.

<!-- fragment «refusal-module-thesis» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="1-13" parent="refusal-source" -->
````rust
//! The one error type, and the reason it is a *refusal* rather than an error.
//!
//! Every operation in this crate either answers the question or declines to
//! act, and a decline is a value the caller can print at a human: it names what
//! is wrong, where, and the command that fixes it. That last part is the whole
//! of why the type exists — an error that only reports detection is unfinished,
//! and a caller that has to synthesise the remedy has to know jj, which is
//! exactly what taking this crate was meant to stop.
//!
//! The remedies named here are **jj's**. This crate has no consumer to speak
//! for, so it never says what the caller should do about the refusal; it says
//! what jj offers.

````
<!-- /fragment -->

The last paragraph is the boundary, stated as a limit rather than as a courtesy:
*The remedies named here are jj's.* The sentence after it is the one that makes
the limit checkable — the crate has no consumer to speak for, so it never says
what the caller should do; it says what jj offers. Every message later in the file
can be held against that sentence, and this chapter holds all ten against it.

The type itself is eleven lines, eight of which argue for the three that declare
it. A newtype over a private enum is the smallest construction in Rust that
publishes a value while publishing nothing about its shape, and the comment says
what that buys rather than what it is.

<!-- fragment «refusal-opaque-type» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="19-29" parent="refusal-source" -->
````rust
/// Why an operation declined to act.
///
/// Opaque on purpose. A consumer that matched on the shape of a refusal would
/// be encoding this crate's internal case analysis into its own control flow,
/// and every one of these cases is a *stop*: there is nothing to branch on and
/// nothing to recover from in code. What a consumer needs is the message, which
/// [`Display`](fmt::Display) gives it, and a cause chain, which
/// [`Error::source`] gives it.
#[derive(Debug)]
pub struct Refusal(Kind);

````
<!-- /fragment -->

The argument has two steps and only the second is specific to this crate. The
first is general: a consumer that matches on the shape of a refusal encodes the
producer's internal case analysis into its own control flow, and every later split
or merge of a case becomes a breaking change. The second is the one that makes
opacity right *here* — **every one of these cases is a stop.** There is nothing to
branch on because there is no case in which the caller can do something else and
continue; the workspace is not a workspace, or jj is not installed, or the commit
is not there. A matchable enum would be an interface for recovery in a crate that
offers none.

The rejected alternative is not hypothetical, and it is in this repository.
`ordinal_fs_tree::Refusal` (`crates/ordinal-fs-tree/src/plan.rs:255`) is a public
enum whose named variants carry the library's own values — a key, an ordinal —
and `grove-loop` names two of them, `Refusal::KeysExhausted` and
`Refusal::DestinationOccupied`, in `task_grow.rs` and `tree_lifecycle.rs`. It
names them in doc comments and in a test's assertion message rather than in a
`match`, and `docs/ARCHITECTURE.md`'s *How an `ordinal-fs-tree` refusal reaches an
operator* is why: grove resolves and classifies its target *before* it calls the
library, so a refusal that arrives has already been ruled out upstream, and the
section is about which of the library's variants may reach a person unaltered.
Two crates in one workspace, opposite choices, and the difference is the one this
comment states. That library's refusals are *algebraic*: they distinguish cases a
caller could act on, which is what makes publishing them worth a version-bump
cost. This crate's are stops. Copying the enum here would have bought a consumer
nothing to do and cost this crate the freedom to split `Namespace` from
`ControlDir`, or to add an eleventh case, without a version bump.

What the consumer gets instead is stated in the last sentence of the comment and
is exactly two things: the message, from `Display`, and the cause chain, from
`Error::source`. The worked example above measured both. The rest of this chapter
reads how they are produced.

<a id="the-case-analysis"></a>
## Ten kinds, in nearly the order the reader met them

The private enum is the case analysis the public type refuses to publish. Its
variants are ordinary Rust with no derive but `Debug`, and the `Debug` is what
makes a refusal usable in a test assertion and in `unwrap`'s panic message
without any of it reaching a consumer's control flow.

<!-- fragment «refusal-kind-open» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="30-31" parent="refusal-source" -->
````rust
#[derive(Debug)]
enum Kind {
````
<!-- /fragment -->

Ten variants follow, and their order in the file is close to the order this book
met them: the gate's two, the namespace's two, scope's two, the seam's three, and
the commit's one. That is not a coincidence and it is not a plan either — it is
the order the crate's own `lib.rs` introduces the operations, and this file was
written alongside it. The one displacement is the seam's three, which the reader
met in chapter 3 and the file lists fourth of the five groups: the enum follows
`lib.rs`'s operations, and the seam is the file those operations call rather
than one of them. The five sections below take the variants in that order and
read each one with the constructor that builds it and the `Display` arm that
renders it, because those three lines are one decision split across three
implementations, and reading them apart is what turns a design into a catalogue.

What no single group's section shows is the shape of the whole case analysis — which kinds carry
a cause, and which shape of remedy each message ends on — so it is set out once
here and argued in place below:

| Kind | Group | Carries | `source()` | The remedy its message names |
|---|---|---|---|---|
| `NotAWorkspace` | gate | `searched_from` | none | `jj git init --colocate` and `jj git init`, both, unconditionally |
| `UnresolvablePath` | gate | `path`, `io::Error` | the `io::Error` | a diagnosis and no command: a broken symlink, or a directory removed underneath the process |
| `Namespace` | namespace | `namespace`, `reason` | none | fixed text: a namespace is one plain directory name |
| `ControlDir` | namespace | `path`, `io::Error` | the `io::Error` | check the permissions on the workspace's `.jj` directory |
| `OutsideWorkspace` | scope | `path`, `root` | none | resolve the workspace that contains the path and ask that one |
| `NotScoped` | scope | `reason` | none | name the paths the operation is about |
| `NotRunnable` | seam | `command`, `io::Error` | the `io::Error` | install jj, at a hard-coded URL |
| `CommandFailed` | seam | `command`, `directory`, `stderr` | none | none: everything after the colon is jj's own stderr |
| `OutputNotText` | seam | `command` | none | none: there is no action a person takes |
| `CommitNotRecorded` | commit | `root`, `Box<Refusal>` | the boxed `Refusal` | `jj undo` and `jj op log`, followed by a disclaimer |

The fourth column is read again under [*What `source()` gives a consumer in place
of variants*](#the-cause-chain), where its four causes and six absences are one
exhaustive `match` written out rather than defaulted. The fifth is where this
chapter's thesis is checkable row by row: every entry in it is a statement about
jj's offer, and none is a statement about a consumer's policy.

<a id="the-gates-two"></a>
## The gate's two, and a remedy stated unconditionally

[*The gate*](02-the-gate.md#worked-resolution) ended on both of these. The walk
finds no `.jj/` anywhere above the caller's path, or it finds one and cannot make
the path canonical.

<!-- fragment «refusal-kind-gate» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="32-35" parent="refusal-source" -->
````rust
    /// No `.jj/` at or above the path — the precondition gate's refusal.
    NotAWorkspace { searched_from: PathBuf },
    /// A `.jj/` was found but the path it sits on could not be made canonical.
    UnresolvablePath { path: PathBuf, cause: io::Error },
````
<!-- /fragment -->

Each variant carries only what its message needs. `NotAWorkspace` carries
`searched_from` — the path the walk started at, not the path it ended at, because
the message tells a person where to look and *at and above* is the whole answer.
`UnresolvablePath` carries the path and the `io::Error` that refused it, and that
`io::Error` is one of the four values in this file that become a `source()` link.

The constructors are the crate's only way to build a refusal, and they are
`pub(crate)`: nothing outside this crate can construct one, so a `Refusal` a
consumer holds was produced by an operation that actually declined.

<!-- fragment «refusal-constructors-gate» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="58-70" parent="refusal-source" -->
````rust
impl Refusal {
    pub(crate) fn not_a_workspace(searched_from: &Path) -> Self {
        Self(Kind::NotAWorkspace {
            searched_from: searched_from.to_path_buf(),
        })
    }

    pub(crate) fn unresolvable_path(path: &Path, cause: io::Error) -> Self {
        Self(Kind::UnresolvablePath {
            path: path.to_path_buf(),
            cause,
        })
    }
````
<!-- /fragment -->

Both take `&Path` and own it immediately. The alternative — borrowing, and giving
`Refusal` a lifetime — would have made the error type unable to outlive the call
that produced it, which is the opposite of what an error is for: a consumer
returns it up the stack, boxes it, or converts it into `anyhow::Error`, and every
one of those requires `'static`. The allocation is on the failure path only, and
the failure path is a stop.

`Display` is one `match` over the ten kinds, and every arm is a single `write!`.
There is no shared prefix, no severity, no code and no wrapping helper, so an arm
can be read as the complete text of its message.

<!-- fragment «refusal-display-open» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="128-130" parent="refusal-source" -->
````rust
impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
````
<!-- /fragment -->

The gate's arm is the longest in the file and carries the file's only comment
about a message. It is worth reading as an argument about defaults: the crate
knows the tree is not a jj workspace, and it does not know whether it is a Git
repository, so it states both initialisation commands and lets the person choose.

<!-- fragment «refusal-display-gate» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="131-152" parent="refusal-source" -->
````rust
            // The gate's refusal, and the one a consumer's user is most likely
            // to meet. Both remedies are stated unconditionally rather than
            // chosen by probing for a `.git`: a message that guesses which one
            // applies can guess wrong, and the pair is two lines.
            Kind::NotAWorkspace { searched_from } => write!(
                f,
                "not a Jujutsu working tree\n  \
                 looked for a `.jj` directory at and above: {}\n\n\
                 Make the tree jj-enabled and rerun:\n      \
                 jj git init --colocate     # an existing Git repository, history kept\n      \
                 jj git init                # no repository here yet\n\n\
                 Nothing was created or changed.",
                searched_from.display()
            ),
            Kind::UnresolvablePath { path, cause } => write!(
                f,
                "a `.jj` directory was found at {} but the path could not be resolved: {cause}\n\n\
                 A workspace is identified by its canonical path, so aliases reach one \
                 workspace; check the path for a broken symlink or a directory that has been \
                 removed underneath this process.",
                path.display()
            ),
````
<!-- /fragment -->

**The alternative was to probe for a `.git` directory and name one command.** The
comment rejects it in one clause — *a message that guesses which one applies can
guess wrong, and the pair is two lines* — and the cost of guessing wrong is
asymmetric in a way the clause implies rather than states. Suggesting
`jj git init` in a tree that does have Git history discards nothing but produces a
repository with no history in it, and the person then has two. Suggesting
`jj git init --colocate` in a tree with no repository fails. Two lines and a
trailing comment on each avoid both, and the crate spends no filesystem call to
earn them.

The last line, *Nothing was created or changed*, is a claim about the whole call
rather than about the message, and chapter 2 already showed the test that proves
it —`a_refused_tree_is_left_exactly_as_it_was` compares the tree's entries either
side of the refusal. It is repeated in this file because a person reading the
message has no access to that test, and a gate that says nothing about its own
effects leaves the reader to check the tree by hand.

`UnresolvablePath`'s arm, immediately below, is the file's shortest remedy and
the only one that is a diagnosis rather than a command: *check the path for a
broken symlink or a directory that has been removed underneath this process*.
There is no jj command for that, so none is offered. The first clause of the
message states why the crate cares — a workspace is identified by its canonical
path, so aliases reach one workspace — which is
[*The gate*](02-the-gate.md#the-value-and-the-gate)'s decision arriving in the
place a person meets it.

<a id="the-namespaces-two"></a>
## The namespace's two, and a message whose second half never varies

[*The namespace it will not name*](04-namespace.md#worked-reservation) refused
four reservations and created one directory. Two variants carry all of it.

<!-- fragment «refusal-kind-namespace» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="36-39" parent="refusal-source" -->
````rust
    /// The namespace a consumer asked for cannot be given to it.
    Namespace { namespace: String, reason: String },
    /// The control directory could not be created or proved writable.
    ControlDir { path: PathBuf, cause: io::Error },
````
<!-- /fragment -->

`Namespace` carries the name and a `reason: String`, and that `String` is the one
piece of message text in this file assembled somewhere else — `validated_namespace`
in `lib.rs` supplies *it is empty*, *it is a path rather than one directory name*,
*it names a directory other than itself* or *Jujutsu owns that name inside `.jj`*.
`ControlDir` carries the path and an `io::Error`, and is the second of the four
kinds with a `source()`.

The constructors show the split: one takes `impl Into<String>` for a reason the
caller composes, the other takes an `io::Error` the filesystem composed.

<!-- fragment «refusal-constructors-namespace» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="71-84" parent="refusal-source" -->
````rust

    pub(crate) fn namespace(namespace: &str, reason: impl Into<String>) -> Self {
        Self(Kind::Namespace {
            namespace: namespace.to_owned(),
            reason: reason.into(),
        })
    }

    pub(crate) fn control_dir(path: &Path, cause: io::Error) -> Self {
        Self(Kind::ControlDir {
            path: path.to_path_buf(),
            cause,
        })
    }
````
<!-- /fragment -->

`impl Into<String>` rather than `&str` is worth one sentence, because it is the
only generic parameter in the file. It accepts a `&'static str` for the four fixed
reasons without allocating at the call site and a `String` for a composed one
without a second copy, and `not_scoped` takes it for the same reason. The crate
composes none today; the parameter costs nothing and keeps that from being an API
change.

The two arms differ in where their remedy comes from, and that is the whole of
what this section adds to chapter 4.

<!-- fragment «refusal-display-namespace» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="153-165" parent="refusal-source" -->
````rust
            Kind::Namespace { namespace, reason } => write!(
                f,
                "cannot reserve the control namespace `{namespace}`: {reason}\n\n\
                 A namespace is one plain directory name, owned by the consumer that asks for \
                 it and kept apart from Jujutsu's own.",
            ),
            Kind::ControlDir { path, cause } => write!(
                f,
                "the control directory {} is not usable: {cause}\n\n\
                 It must exist and be writable before anything can coordinate through it. \
                 Check the permissions on the workspace's `.jj` directory.",
                path.display()
            ),
````
<!-- /fragment -->

**`Namespace`'s remedy is fixed text and its variable half is the reason.** The
second paragraph — *a namespace is one plain directory name, owned by the consumer
that asks for it and kept apart from Jujutsu's own* — is the same for all four
rejections, because it is a statement of the rule rather than of the violation.
The message therefore reads as *what you asked for, why it was refused, what the
rule is*, and only the middle third is computed.

**`ControlDir`'s remedy names permissions, and chapter 4 recorded the case where
that is the wrong thing to say.** `control_dir(".gitignore")` reaches this arm
rather than the one above it, because `.gitignore` is a name jj writes inside
`.jj/` of a colocated workspace and the crate's reserved list does not hold it,
so the refusal a consumer sees suggests checking permissions on a directory that
already exists — measured on jj 0.44.0 and recorded in
[*The reserved list*](04-namespace.md#the-reserved-list). The message is not wrong
about what it observed; `create_dir_all` did refuse. It is wrong about what to do,
and that is the shape of defect this arm can have: the remedy is chosen by which
constructor was reached, so a gap in the validation upstream becomes a misleading
remedy here.

<a id="scopes-two"></a>
## Scope's two, and reasons written for the condition

[*Scope and commit*](05-scope-and-commit.md#the-path-algebra) produced both of
these from the path algebra, and one of them from two different places in it.

<!-- fragment «refusal-kind-scope» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="40-43" parent="refusal-source" -->
````rust
    /// A path was handed to a workspace that does not contain it.
    OutsideWorkspace { path: PathBuf, root: PathBuf },
    /// An operation whose point is a narrow scope was given none.
    NotScoped { reason: String },
````
<!-- /fragment -->

`OutsideWorkspace` carries the path and the root it was compared against, and the
root is in the message because *not inside the workspace* is unfalsifiable without
it — a caller reading the refusal is usually holding a path it believes is inside
one. `NotScoped` carries a reason and no path at all, because there is nothing to
name: the refusal is that nothing was named.

<!-- fragment «refusal-constructors-scope» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="85-97" parent="refusal-source" -->
````rust

    pub(crate) fn outside_workspace(path: &Path, root: &Path) -> Self {
        Self(Kind::OutsideWorkspace {
            path: path.to_path_buf(),
            root: root.to_path_buf(),
        })
    }

    pub(crate) fn not_scoped(reason: impl Into<String>) -> Self {
        Self(Kind::NotScoped {
            reason: reason.into(),
        })
    }
````
<!-- /fragment -->

`outside_workspace` is called three times from one branch of `relative`, for a
path with no parent, a path with no file name, and a canonical parent still
outside the root. That is one condition with three detections rather than three
conditions, and this constructor is where the decision to treat them alike is
made visible: they produce the same refusal because a caller's next action is the
same for all three.

The two arms are the file's plainest, and both end in a sentence that says what
the crate will not do instead.

<!-- fragment «refusal-display-scope» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="166-179" parent="refusal-source" -->
````rust
            Kind::OutsideWorkspace { path, root } => write!(
                f,
                "{} is not inside the Jujutsu workspace rooted at {}\n\n\
                 A workspace answers only for its own files. Resolve the workspace that \
                 contains the path and ask that one.",
                path.display(),
                root.display()
            ),
            Kind::NotScoped { reason } => write!(
                f,
                "a path-scoped operation was given no scope: {reason}\n\n\
                 Name the paths the operation is about. Widening it to the whole working copy \
                 is not the fallback, because the scope is what the caller asked for.",
            ),
````
<!-- /fragment -->

*A workspace answers only for its own files. Resolve the workspace that contains
the path and ask that one.* is a remedy that names no command, and there is no jj
command it could name — resolving another workspace is a call into this crate.
That is the one place in the file where the remedy is an operation of the crate's
own, and it is stated as a direction rather than as an offer to perform it.

*Widening it to the whole working copy is not the fallback, because the scope is
what the caller asked for* is the same refusal chapter 5 argued, written where a
person reads it. The reason string that precedes it is written for the condition —
`"no paths were named"`, `"the workspace root is not a scope inside itself"` — and
neither guesses at the caller's mistake. A message that said *did you mean to pass
the files you changed?* would be speaking for a consumer this crate does not know.

<a id="the-seams-three"></a>
## The seam's three: could not start, ran and declined, said something unreadable

[*The subprocess seam*](03-subprocess-seam.md#the-two-endings) built all three and
printed two of them. They are grouped in the enum because they are produced at one
place — `jj.rs` — and because the distinction between the first two is the seam's
reason for existing.

<!-- fragment «refusal-kind-seam» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="44-53" parent="refusal-source" -->
````rust
    /// `jj` could not be run at all.
    NotRunnable { command: String, cause: io::Error },
    /// `jj` ran and failed.
    CommandFailed {
        command: String,
        directory: PathBuf,
        stderr: String,
    },
    /// `jj` produced bytes that are not text.
    OutputNotText { command: String },
````
<!-- /fragment -->

`NotRunnable` carries the rendered command and the `io::Error` that stopped it
starting. `CommandFailed` is the file's only struct-form variant with three
fields, and it carries no `io::Error` at all: the command started, so the failure
is jj's rather than the operating system's, and what jj has to say is its stderr.
`OutputNotText` carries the command and nothing else, because there is nothing
useful to carry — bytes that are not UTF-8 cannot be put in a message.

<!-- fragment «refusal-constructors-seam» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="98-118" parent="refusal-source" -->
````rust

    pub(crate) fn not_runnable(command: &str, cause: io::Error) -> Self {
        Self(Kind::NotRunnable {
            command: command.to_owned(),
            cause,
        })
    }

    pub(crate) fn command_failed(command: &str, directory: &Path, stderr: &str) -> Self {
        Self(Kind::CommandFailed {
            command: command.to_owned(),
            directory: directory.to_path_buf(),
            stderr: stderr.trim().to_owned(),
        })
    }

    pub(crate) fn output_not_text(command: &str) -> Self {
        Self(Kind::OutputNotText {
            command: command.to_owned(),
        })
    }
````
<!-- /fragment -->

`command_failed` is the only constructor that transforms its input:
`stderr.trim().to_owned()`. jj's stderr ends in a newline and often begins with a
blank line, and the message embeds it mid-sentence, so trimming is the difference
between one line and a paragraph with a hole in it. Nothing else is done to it —
no truncation, no first-line extraction, no filtering of jj's hints — because the
crate cannot tell which part of jj's output a person needs.

The three arms are the file's only place where a remedy is not jj's to run, and
one of the three has no remedy at all.

<!-- fragment «refusal-display-seam» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="180-194" parent="refusal-source" -->
````rust
            Kind::NotRunnable { command, cause } => write!(
                f,
                "could not run `{command}`: {cause}\n\n\
                 Jujutsu drives this workspace, so its binary has to be on `PATH`. Install it \
                 (https://jj-vcs.github.io/jj/latest/install-and-setup/) and rerun."
            ),
            Kind::CommandFailed {
                command,
                directory,
                stderr,
            } => write!(f, "`{command}` failed in {}: {stderr}", directory.display()),
            Kind::OutputNotText { command } => write!(
                f,
                "`{command}` produced output that is not text, and its answer cannot be read"
            ),
````
<!-- /fragment -->

**`NotRunnable`'s remedy is installation, and it is the only arm that hard-codes a
URL.** That URL is a defect the book records rather than fixes: on jj 0.44.0
`https://jj-vcs.github.io/jj/latest/install-and-setup/` answers `301 Moved
Permanently` to `docs.jj-vcs.dev`, so the crate prints a redirect. The corpus is
frozen for this book, and the fix moves a line boundary inside the fragment above,
so it must land in one commit with this page; it is carried as `jj-docs-url-k64`
and is the reason chapter 2 links jj's glossary at the current host instead of
propagating this one. A reader following the printed link still arrives, which is
why this is a defect and not a break.

**`CommandFailed`'s arm has no remedy paragraph, and that is the correct shape.**
It is the only arm that renders on one line, and everything after the colon is
jj's. The crate does not know why jj declined — the same variant carries a
snapshot failure, a conflicted revision and a syntax error in a fileset — so any
sentence it appended would be a guess. The blank-line-and-remedy shape of every
other arm is absent exactly where the crate has nothing to add, which is more
informative than a generic *check the command and try again*.

**`OutputNotText` says what happened and stops.** *its answer cannot be read* is
the honest end of that sentence: there is no action a person takes, because a jj
command that emits non-UTF-8 on stdout is a fault in something other than the
caller. Chapter 3 recorded that neither this refusal nor `NotRunnable` is reached
by any test in the crate's suite, for a fixture reason rather than an oversight,
and neither message is asserted on anywhere.

<a id="the-commit-that-did-not-land"></a>
## The one refusal about state

Nine of the ten kinds say something about a command: it could not run, it was
refused, its argument was wrong. The tenth says something about the tree the
caller is standing in.

<!-- fragment «refusal-kind-commit» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="54-57" parent="refusal-source" -->
````rust
    /// A commit was attempted and did not land.
    CommitNotRecorded { root: PathBuf, cause: Box<Refusal> },
}

````
<!-- /fragment -->

`CommitNotRecorded` is the only variant whose cause is a `Refusal` rather than an
`io::Error`, and the `Box` is what makes that possible — a variant holding a
`Refusal` by value would make `Kind` infinitely sized. It carries `root` rather
than the paths that were being committed, because the remedy is about the
workspace: `jj undo` reverses an operation in a repository, not a file.

<!-- fragment «refusal-constructors-commit» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="119-127" parent="refusal-source" -->
````rust

    pub(crate) fn commit_not_recorded(root: &Path, cause: Refusal) -> Self {
        Self(Kind::CommitNotRecorded {
            root: root.to_path_buf(),
            cause: Box::new(cause),
        })
    }
}

````
<!-- /fragment -->

`commit_not_recorded` takes its cause **by value** where every other constructor
takes a borrow, and that is the wrapping relationship made explicit in the
signature: the seam's refusal is consumed and does not survive independently. The
one call site is `commit`'s `map_err`
([*Scope and commit*](05-scope-and-commit.md#the-commit)), and the refusal from
the change-id read one line below it is deliberately **not** wrapped — which is
how a consumer tells *there is no commit* from *the commit landed and could not be
named*, since it cannot tell them apart by matching.

The arm is the second of the two that carry a command listing, and it is the only
one whose subject is what is true now rather than what was attempted.

<!-- fragment «refusal-display-commit» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="195-214" parent="refusal-source" -->
````rust
            // The only refusal that has to say something about *state* rather
            // than about a command: the caller asked for a commit and does not
            // have one, so the working copy is holding whatever it prepared.
            // jj owns the repair and this names it; the crate runs none of it.
            Kind::CommitNotRecorded { root, cause } => write!(
                f,
                "the commit did not land in {}: {cause}\n\n\
                 Jujutsu snapshots the working copy before every command and its operation log \
                 is the transaction record, so the state before this attempt is still \
                 reachable:\n      \
                 jj undo                    # reverse the snapshot this attempt recorded\n      \
                 jj op log                  # inspect the operations first, if `jj undo` is \
                 not the one\n\n\
                 Nothing here runs a recovery of its own.",
                root.display()
            ),
        }
    }
}

````
<!-- /fragment -->

The comment above it states the reason in the terms this chapter opened on: the
caller asked for a commit and does not have one, so the working copy is holding
whatever it prepared. That is a fact about state, and it is why the remedy cannot
be *rerun the command* — rerunning is what a caller would do with a refusal about
a command, and here it may not be what the person wants.

**Two commands, in a deliberate order, and neither is run.** `jj undo` is offered
first because it is the common repair and it is one word; `jj op log` is offered
second, with the condition that makes it the right one — *inspect the operations
first, if `jj undo` is not the one*. The listing is the same two-command shape as
the gate's, and for the same reason: the crate knows the class of repair and not
which member of it applies, so it states both rather than choosing. The user
guide's [*Undoing a mistake*](../../USAGE.md#undoing-a-mistake) is the same pair
in the same order for grove as a whole, which is what a consumer's operator reads
when a session's commit does not land.

The last line, *Nothing here runs a recovery of its own*, is this chapter's thesis
as one sentence, and it is the only line in the file that disclaims an action
rather than describing one. It is there because this is the arm where a reader is
most likely to assume otherwise: a crate that knows the exact repair, has the
workspace root in hand and already spawns jj could plainly run `jj undo` itself.
It does not, because undoing is a decision about the caller's work — the operation
this attempt recorded may be a snapshot the caller wants kept — and the crate has
no consumer to make that decision for.
`a_commit_that_cannot_land_names_the_operation_log_repair`
(`crates/jj-workspace/tests/workspace.rs`) asserts all three parts: that the
commit is absent, that both commands are named, and that the disclaimer is
present.

<a id="the-cause-chain"></a>
## What `source()` gives a consumer in place of variants

The last sixteen lines are the second half of the interface. `Display` gives a
consumer the message; `Error::source` gives it the one thing the message cannot —
a value it can walk.

<!-- fragment «refusal-error-source-caused» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="215-221" parent="refusal-source" -->
````rust
impl Error for Refusal {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.0 {
            Kind::UnresolvablePath { cause, .. }
            | Kind::ControlDir { cause, .. }
            | Kind::NotRunnable { cause, .. } => Some(cause),
            Kind::CommitNotRecorded { cause, .. } => Some(cause.as_ref()),
````
<!-- /fragment -->

Four kinds have a cause and the `match` groups them by what that cause is. Three
hold an `io::Error` and return it directly; `CommitNotRecorded` holds a boxed
`Refusal` and returns `cause.as_ref()`, which is where a consumer walking the
chain crosses from this crate's error into this crate's error again. That is the
only chain the crate produces with a `Refusal` at both ends, and the worked
example measured its depth: one link, then `None`.

The remaining six return `None`, and the exhaustive list is what makes that a
decision rather than a default.

<!-- fragment «refusal-error-source-uncaused» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="222-230" parent="refusal-source" -->
````rust
            Kind::NotAWorkspace { .. }
            | Kind::Namespace { .. }
            | Kind::OutsideWorkspace { .. }
            | Kind::NotScoped { .. }
            | Kind::CommandFailed { .. }
            | Kind::OutputNotText { .. } => None,
        }
    }
}
````
<!-- /fragment -->

A catch-all `_ => None` would compile and would be shorter by five lines. Writing
every kind out means a new variant fails to compile until someone decides which
group it belongs to, and *has this kind a cause* is exactly the question a new
variant's author should be made to answer. The five-line cost buys one
non-negotiable review question.

`CommandFailed` is in this list, and it is the one whose absence from the chain is
worth stating plainly. jj's own `Caused by: 1: … 2: …` lines are in the message,
as the worked example showed, and they are text. A consumer walking `source()`
from a failed commit takes one link, reaches the `CommandFailed` refusal, and
stops; jj's own two causes below it exist only as bytes in a string. Nothing here is lossy — the text is complete — but a
consumer that wanted to classify the underlying `io::ErrorKind` of jj's own
failure cannot, because the process boundary already turned it into a message.
That is a consequence of the subprocess seam rather than of this file, and it is
the price of the boundary chapter 3 argued for.

<a id="no-error-crate"></a>
## Four `use` lines, and the dependency table they keep empty

[*Orientation*](01-orientation.md#package-contract) opened the book on an empty
`[dependencies]` table and said the error type was the place that claim would be
easiest to break. This is that place, and the imports are the check.

<!-- fragment «refusal-imports» owner="no-remedy-of-its-own" source="crates/jj-workspace/src/refusal.rs" lines="14-18" parent="refusal-source" -->
````rust
use std::error::Error;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

````
<!-- /fragment -->

`std::error::Error` is the trait, `std::fmt` is `Display`, `std::io` is the cause
type three kinds hold, and `std::path` is what the rest carry. There is no
`thiserror` derive, no `anyhow` in the library, no `miette` diagnostic and no
error-code enum from a shared crate. The manual `Display` implementation is
roughly eighty lines that `thiserror` would have written from attributes, and that
is the measurable cost of the empty table.

The cost is paid for two reasons and only one of them is the dependency count.
**A derived `Display` puts the message text in an attribute on the variant**, and
these messages are multi-paragraph, contain a blank line, and in two cases contain
an indented command listing — text that a `#[error("…")]` attribute can hold but
that no reader would go looking for there. The messages are the deliverable of
this file, and writing them in the body puts them where they are edited. **A
derived error also publishes the variants**, or at least makes keeping them private
awkward, and the opacity argued at the top of this chapter is the design. A
`thiserror` enum that is then wrapped in a newtype to hide it is the same eighty
lines with an extra type.

What this file inherits from the empty table, in return, is that a consumer
depends on `std` to handle a `Refusal`. The `Error` implementation above is the
standard library's trait, so `Box<dyn Error>`, `anyhow`, `eyre` and a plain
`{}` all work on it, and none of them was chosen by this crate on a consumer's
behalf. That is the dependency argument and the consumer-vocabulary argument
reaching the same line from opposite directions.

<a id="what-this-chapter-settled"></a>
## What this chapter settled

Two hundred and thirty lines, and about a hundred and thirty of them are text a
person reads. The type is one field wide and publishes nothing: ten kinds behind a
newtype, every one of them a stop, so a consumer branches on none of them and
loses nothing by it. The alternative is in this repository and is the right choice
there — `ordinal_fs_tree::Refusal` is a public enum whose variants grove names,
because its refusals distinguish cases a caller could act on rather than stops.

What a consumer gets instead was measured rather than described. `Display` gives
the message, and the messages are structured the same way throughout: what is
wrong, where, a blank line, and what jj offers — with the remedy paragraph absent
in exactly the two arms that have nothing to add, `CommandFailed`, whose remedy is
jj's own stderr, and `OutputNotText`, which has none. `Error::source` gives a chain that is
one link deep at its deepest, and the worked example walked it to `None`. The
cause is in the message as well as in the chain, so a consumer that prints the
chain prints it twice; that redundancy is the measured price of serving the
consumer that prints one string, which is the consumer these messages are written
for.

The remedies are jj's throughout, and the two that are command listings state both
members of a pair rather than probing to choose one. The gate names
`jj git init --colocate` and `jj git init` because guessing which applies can
guess wrong and the pair costs two lines. `CommitNotRecorded` names `jj undo` and
`jj op log` for the same reason, and then disclaims running either — the one line
in the file that says what the crate will not do, placed in the one arm where a
reader would most reasonably expect it to act.

And the empty dependency table survived the file that was most likely to break it.
Four `use` lines, all `std`, an eighty-line `Display` written by hand rather than
derived, and an error a consumer can put in `anyhow` without this crate having
depended on `anyhow` to make that true.

One chapter remains. Every refusal in the crate has now been read at the site that
creates it, and the last chapter does not read any source: it assembles the six
subtractions into one table, states the test that separates a justified subtraction
from an abdication, and applies it to all six — including the one where the answer
is least comfortable.

[Previous: Scope and commit](05-scope-and-commit.md) | [Contents](README.md) | [Next: What jj owns](07-what-jj-owns.md)
