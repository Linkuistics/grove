# Appearance is the event
<!-- book-page id="the-channel" slice="appearance-is-the-event" order="6" -->
[Previous: From a template to an argv](05-to-an-argv.md) | [Contents](README.md)

<a id="appearance-is-the-event"></a>
## Appearance is the event

Chapter 5 ended with an `Argv` and nowhere to send it. The configuration half of
the crate is closed: a key resolved from one file, a template was split into
words once at load, four values filled the vocabulary the consumer declared, and
four words came out — none of which the crate has an opinion about. This chapter
opens the launch half, and it owns one file. `src/channel.rs` is 404 lines; lines
1 to 271 are all of it except the inline `#[cfg(test)] mod tests` at the end,
which is corpus like everything else and is chapter 9's.

What this stage must not add and must not interpret is **the ending**. The crate
does not decide that a child is finished. It allocates a path, hands that path to
the child, and waits for the path to exist; whether the work was done, whether it
succeeded, and what should happen next are all read off a string this crate never
looks at. That is the third arm of the book's outcome — a layer learning what a
value means **on the way out**, by inferring what came back — and this is where
the crate declines to.

The reason a launch needs a channel at all is that the obvious signal is wrong
for the child this crate exists to launch. A batch program ends by exiting, and
its exit status is a perfectly good ending. An **interactive** program does not:
when it finishes the turn it was given, it returns to its prompt and waits for
another. Its process is still alive, its exit is still ahead of it, and nothing
about it has changed in a way a parent can observe. A launcher that waited for
that child to exit would wait for as long as the child was willing to sit there,
which is forever. So the ending has to be carried out of band — over a path the
launcher chose before the child started, on which the child says one thing and
then stops mattering.

The word *channel* is grove's before it is this crate's. Grove's glossary already
names the mechanism, as the
[loop control channel](../../../CONTEXT.md#loop-control-channel): the per-launch path
the driver watches while its harness child runs, whose **appearance alone** ends
the session. Everything else that entry says — the session epoch, the driver, the
reading of `Relaunch` against `Done`, the scrubbing of the variable from every
other spawn — is on grove's side of the line, and this crate has never heard of
any of it. What follows is the same mechanism from underneath, where it is a
directory, a name, a file that may or may not appear, and a string nobody reads.

One property makes the whole arrangement work, and it is the first thing the file
says about itself: **allocation picks a name and writes nothing**. The channel
file does not exist when the child starts. It comes into existence only if
something writes to it, and the only thing holding its path is the child. So the
file's later existence is unambiguous evidence that the child spoke — not a stale
file from a previous launch, not a placeholder the launcher created and forgot,
not a race between two launches sharing a directory. That is why `run` is allowed
to treat mere *appearance* as an event, and why chapter 8's supervisor can be a
poll on `exists()` rather than a protocol. Take the writing-nothing property away
and appearance means nothing; every other decision on this page rests on it.

<a id="a-path-and-nothing-else"></a>
## A path, and nothing else

This section takes step 3 of the five-call trace chapter 1 wrote and runs it to
full resolution, then runs the far end of it — the token coming back — which
chapter 1 wrote as step 5. Nothing between the two is this chapter's; the spawn
is chapter 7's and the watch is chapter 8's.

Grove's control directory for the worked example is the one chapter 1 fixed, and
it already exists — the driver made it long before any launch.

```text
/work/atlas/.jj/grove/
  driver.lease
```

The call is `Channel::allocate` on that directory, and its observable end is a
path that does not exist:

```text
Channel::allocate(Path::new("/work/atlas/.jj/grove"))
  -> Ok(Channel { path: "/work/atlas/.jj/grove/signal-3f9c1d4a7b2e5086c1a4f70d93b6e281" })

/work/atlas/.jj/grove/
  driver.lease                                        (unchanged)
```

The directory is byte-for-byte what it was. `allocate` read its metadata, drew
sixteen bytes from `/dev/urandom`, rendered them as thirty-two lowercase hex
characters, joined that to the prefix `signal-`, asked the filesystem whether
anything already stood at the result, and — told nothing did — returned the name.
The one observable effect of a successful allocation is that a `Channel` value
exists in the launcher's memory holding a `PathBuf`. `channel.path()` is that
`PathBuf`, and chapter 7 is where it becomes the value of `GROVE_SIGNAL_FILE` in
the child's environment.

The child now runs, and grove's harness ends its turn by writing one word and
returning to its prompt. That write is the other end of this file:

```text
signal(Path::new("/work/atlas/.jj/grove/signal-3f9c1d4a7b2e5086c1a4f70d93b6e281"), "relaunch")

/work/atlas/.jj/grove/
  driver.lease
  signal-3f9c1d4a7b2e5086c1a4f70d93b6e281            "relaunch\n"
```

And the launcher, holding the same `Channel`, reads it back:

```text
channel.read()
  -> Some(Token("relaunch"))
```

Three things in that round trip are worth naming before the source is read,
because the rest of the chapter is each of them read closely. The **file appeared
between the two figures above**, and that appearance is the whole event: a
launcher polling the directory learns everything it is going to learn from
`exists()`. The **newline is gone** from the value and still present in the file;
`signal` added it and `read` took it off, and neither call looked at what sat in
front of it. And the word is **`relaunch`**, which is grove's word and means
something specific to grove's driver — a fact established nowhere in these 271
lines. `crates/grove-loop/src/complete.rs` is where it acquires meaning, in a
function called `interpret` whose whole body is a comparison against grove's own
constant. The crate that carried the word from one process to another never
compared it with anything.

<a id="what-the-block-answers"></a>
## What the block answers

The 271 lines are a module thesis, three constants, one type with five methods, a
second type with two accessors, one free function and four private helpers. The
table collects what each answers and how each refuses, so the sections that
follow can be read one at a time; every refusal text is exact. The tests named
without a path are the inline module at the end of this same file — chapter 9
reproduces and explains them, against the fragments below — and the rest are in
`crates/keyed-launch/tests/launch.rs`.

| Item | Answers | Refuses with | Pinned by |
|---|---|---|---|
| `Channel::allocate` | which fresh path names this launch alone | `` cannot allocate a completion channel in `dir`: … `` ; `` …: it is not a directory `` ; `` cannot establish whether the drawn completion-channel path `p` is free: … `` ; `` could not allocate a fresh completion channel after 8 occupied random draws `` | `an_allocated_channel_names_a_path_that_does_not_yet_exist`, `successive_allocations_in_one_directory_never_collide`, `allocation_names_a_missing_directory_and_says_what_to_do` |
| `Channel::path` | the path to publish to the child | cannot fail | `the_channel_path_is_published_under_the_callers_chosen_variable_name` |
| `Channel::read` | the token this launch left, if any | `None` is an answer, not a refusal | `a_signalled_channel_reads_back_the_token_without_its_framing`, `an_empty_channel_file_is_not_an_empty_token`, `an_unsignalled_channel_reads_back_nothing` |
| `Channel::discard` | is this launch's path empty now | `` cannot remove the completion channel `p`: …; remove it by hand `` | `discarding_removes_the_file_and_succeeds_when_there_was_none` |
| `Channel::discard_abandoned` | are a previous launcher's channels gone | `` cannot list abandoned completion channels in `dir`: … `` ; `` could not remove N abandoned completion channel(s) …; remove them by hand `` | `abandoned_cleanup_removes_channels_and_leaves_every_other_entry_alone`, `abandoned_cleanup_names_the_directory_when_it_cannot_be_listed` |
| `Token` | what the launch said, as the caller's to read | — | `a_child_signals_through_the_published_path_and_the_token_comes_back` |
| `signal` | write this token to this path | `` cannot write the completion token to `p`: …; check that the channel was not removed early `` | `a_child_signals_through_the_published_path_and_the_token_comes_back` |
| `is_channel_name` | is this name exactly one of ours | — | `abandoned_cleanup_removes_channels_and_leaves_every_other_entry_alone` |
| `remove_if_present` | is this path empty now | it owns both removal wordings above | `discarding_removes_the_file_and_succeeds_when_there_was_none` |
| `draw_nonce` | sixteen bytes of OS randomness | `` cannot open the OS randomness source: … `` ; `` cannot read 16 bytes of OS randomness … `` | — |
| `hex` | those bytes as thirty-two lowercase characters | cannot fail | `an_allocated_channel_names_a_path_that_does_not_yet_exist` |

The composite below is the block as a whole. It is the file up to the
`#[cfg(test)]` attribute on line 272, and that boundary is the only one in this
book cut at a compilation condition rather than at a concept — the reason belongs
on chapter 9's page, where the module it separates is explained.

<!-- fragment «channel-production» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="1-271" parent="source-channel" -->
<!-- insert «channel-thesis» -->
<!-- insert «channel-prefix» -->
<!-- insert «channel-nonce-bytes» -->
<!-- insert «channel-retry-limit» -->
<!-- insert «channel-type» -->
<!-- insert «channel-allocate» -->
<!-- insert «channel-published-path» -->
<!-- insert «channel-read» -->
<!-- insert «channel-discard» -->
<!-- insert «channel-discard-abandoned» -->
<!-- insert «channel-token» -->
<!-- insert «channel-signal» -->
<!-- insert «channel-name-grammar» -->
<!-- insert «channel-remove-if-present» -->
<!-- insert «channel-draw-nonce» -->
<!-- insert «channel-hex» -->
<!-- /fragment -->

<a id="the-thesis"></a>
## What the file says it is

The module's own first sentence is the chapter title in the crate's words, and
the four imports under it are the whole of what the file needs.

<!-- fragment «channel-thesis» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="1-9" parent="channel-production" -->
````rust
//! The out-of-band completion channel: a fresh, collision-resistant path per
//! launch, naming that launch alone.

use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::error::LaunchError;
````
<!-- /fragment -->

Two words in that sentence are load-bearing and both are argued later in the
file. **Out-of-band** is the reason the channel exists at all: the band is the
child's own process — its exit status, its output, its lifetime — and the ending
this crate waits for travels beside it rather than through it.
**Collision-resistant** is a claim about the name, and the next three fragments
are the three numbers that make it one. *A fresh path per launch, naming that
launch alone* is what `successive_launches_get_independent_channels` pins from
the outside: two launches in one control directory, two different paths, and each
launch's channel holding only its own token.

The imports are worth one sentence because of what is missing from them. `fs`,
`File`, `Read`, `Path`, `PathBuf`, a `Write` trait for formatting, and this
crate's own `LaunchError` — nothing else. Chapter 1 read the manifest's rule for
this crate's dependencies: *two document formats this crate reads, and the
syscalls it cannot reach from `std`*. Neither clause reaches a channel. `kdl` and
`shell-words` belong to the configuration half the previous chapter closed, and
`libc` arrives in chapters 7 and 8 for the signals `std` cannot send. Everything
on this page is a file read, a file write, a directory listing and a remove, and
`std` has all four — which is why the file that owns the crate's one
cryptographic-sounding requirement adds no dependency to satisfy it. The one
place that shows is `draw_nonce`, at the end of the chapter.

<a id="the-name-grammar"></a>
## A name a human can recognise, in three numbers

The name of a channel file is a constant, a length and a bound, in that order.
None of the three is arbitrary and each is argued where it is declared, so the
prose here connects them rather than restating them.

<!-- fragment «channel-prefix» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="10-21" parent="channel-production" -->
````rust

/// The fixed leading text of every channel file name.
///
/// A prefix rather than a bare nonce so a human reading the directory can tell
/// what the file is, and so [`Channel::discard_abandoned`] has something to
/// recognise that no neighbouring file will accidentally match.
///
/// Named for [`signal`], the verb that writes one. It is also the spelling
/// grove's driver already leaves in its control directory, so a launcher moving
/// onto this crate still recognises the channels its predecessor abandoned
/// rather than orphaning them permanently.
const CHANNEL_PREFIX: &str = "signal-";
````
<!-- /fragment -->

The prefix carries three separate obligations, and the comment names all three.
It makes the file legible to a person who has just listed the directory; it gives
`discard_abandoned` something to recognise; and — the third clause, which is the
one a reader coming from grove will care about — it is the spelling grove's
driver already leaves behind. That last is a compatibility fact with an address:
grove's control directory holds files named `signal-…` written by driver versions
that predate this crate, and a launcher moving onto `keyed-launch` still
recognises them as abandoned channels rather than orphaning them permanently. It
is the only line in the file that knows any consumer exists, and it knows only
the consumer's *spelling*, not its meaning.

Note which way the naming runs. The constant is named for [`signal`], the free
function at the bottom of this file that writes one — not for the launcher that
allocates it, and not for what the token will say. The file is named after the
act that brings it into existence, which is the same choice as making appearance
the event.

<!-- fragment «channel-nonce-bytes» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="22-25" parent="channel-production" -->
````rust

/// Bytes of OS randomness in a channel name — 128 bits, rendered as 32
/// lowercase hex characters.
const NONCE_BYTES: usize = 16;
````
<!-- /fragment -->

Sixteen bytes is 128 bits, and the comment states the width and its rendering
together because the two travel as one grammar: the length this constant fixes
is the length `is_channel_name` checks and the length `hex` produces, and all
three read it from here. The bound below is the first thing 128 bits buys.

<!-- fragment «channel-retry-limit» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="26-32" parent="channel-production" -->
````rust

/// How many occupied random draws are tolerated before allocation refuses.
///
/// A bound rather than an unbounded retry: at 128 bits a single collision means
/// the randomness source is not random, and spinning on that forever turns a
/// diagnosable fault into a hang.
const DRAW_RETRY_LIMIT: usize = 8;
````
<!-- /fragment -->

What 128 bits buys is that the retry loop below is a diagnostic rather than a
liveness hazard. The comment on `DRAW_RETRY_LIMIT` states the
argument in full and it is worth reading as a piece of reasoning rather than a
tuning knob: at this width a collision is not an unlucky draw a retry should
absorb, it is evidence that the randomness source is broken. An unbounded retry
would respond to that evidence by spinning forever, and a launcher that hangs
looks — from the outside, and to the operator — exactly like a child that has not
finished. The bound converts the same fault into a sentence naming
`/dev/urandom`.

Eight is therefore not a probability estimate; nothing about the number eight
follows from 128 bits. It is a number small enough that a broken source is
reported promptly and large enough that no working source will ever reach it. The
cost of the choice is that a genuinely occupied directory — one already holding
every name the source produced, which cannot arise from randomness — fails rather
than searching, and that is the outcome the comment argues for.

<a id="writes-nothing"></a>
## The type that writes nothing

`Channel` is one `PathBuf`, private. Everything this chapter claims about
evidence follows from the paragraph above it.

<!-- fragment «channel-type» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="33-43" parent="channel-production" -->
````rust

/// The out-of-band completion signal for one launch.
///
/// **Allocation picks a name; it writes nothing.** The channel file comes into
/// existence only when something calls [`signal`] on its path — which is what
/// makes *appearance* the event [`run`](crate::run) watches for. A channel that
/// is never signalled is a path that never existed.
#[derive(Debug)]
pub struct Channel {
    path: PathBuf,
}
````
<!-- /fragment -->

*A channel that is never signalled is a path that never existed.* That sentence
is the chapter's through-line to chapters 7 and 8, and it is worth spelling out
what it buys, because the comment states the property without stating the
consequence. The launcher's supervisor decides that a child has spoken by testing
whether the path exists. That test is sound **only** because nothing else in the
system could have created the file: allocation did not, the launcher did not, and
a previous launch could not, because its name was drawn independently. If
`allocate` had created the file — an obvious design, and the one a reader
expecting `O_CREAT|O_EXCL` reservation semantics will reach for first — then
existence would be true from the moment of allocation and the supervisor would
need a second signal to distinguish *reserved* from *written*: a size check, a
lock, a rename, or a sentinel content. Each of those is a protocol, and a
protocol is something both processes have to agree on. Writing nothing is what
lets the two ends agree on nothing at all beyond a path.

The struct derives `Debug` and nothing else. It is deliberately not `Clone`: two
`Channel` values naming one path would make `discard`, which consumes `self`,
unable to keep its post-condition. And it is not `Copy`, `PartialEq` or
`Serialize` — a channel is a handle to a live launch, not a value to compare or
persist.

<a id="drawing-a-name"></a>
## Drawing a name

`allocate` is the file's first method — thirty-nine lines, second in length only
to `discard_abandoned` — and it does two distinguishable jobs: it checks the
caller's directory, and then it draws names until one is free.

<!-- fragment «channel-allocate» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="44-92" parent="channel-production" -->
````rust

impl Channel {
    /// Draw a fresh channel path inside `dir`, retrying an occupied name
    /// without touching whatever occupies it.
    ///
    /// `dir` must already exist. That is checked here rather than left to the
    /// child's first write, because the two failures land in very different
    /// places: checked, the caller is told which directory is missing before
    /// anything is spawned; unchecked, a child runs to completion and its
    /// signal silently fails to land, which reads as a launch that hung.
    pub fn allocate(dir: &Path) -> Result<Self, LaunchError> {
        let metadata = fs::metadata(dir).map_err(|error| {
            LaunchError::new(format!(
                "cannot allocate a completion channel in {}: {error}; create the directory, or \
                 pass one that exists",
                dir.display()
            ))
        })?;
        if !metadata.is_dir() {
            return Err(LaunchError::new(format!(
                "cannot allocate a completion channel in {}: it is not a directory; pass the \
                 directory the channel files should live in",
                dir.display()
            )));
        }

        for _ in 0..DRAW_RETRY_LIMIT {
            let path = dir.join(format!("{CHANNEL_PREFIX}{}", hex(draw_nonce()?)));
            match fs::symlink_metadata(&path) {
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    return Ok(Self { path })
                }
                Err(error) => {
                    return Err(LaunchError::new(format!(
                        "cannot establish whether the drawn completion-channel path {} is free: \
                         {error}; make the directory readable and retry",
                        path.display()
                    )))
                }
                Ok(_) => continue,
            }
        }
        Err(LaunchError::new(format!(
            "could not allocate a fresh completion channel after {DRAW_RETRY_LIMIT} occupied \
             random draws in {}; at 128 bits this means the OS randomness source is not random — \
             check /dev/urandom",
            dir.display()
        )))
    }
````
<!-- /fragment -->

The first job is argued in the comment, and the argument is about *where a
failure lands* rather than about correctness: both versions of the code refuse a
missing directory, and the difference is that one refuses before anything is
spawned and the other refuses inside a child nobody is reading the stderr of.
Unchecked, the sequence is a child that runs to completion, a `signal` that
fails, a file that never appears, and a launcher that waits out its grace and
escalates — which reads, to an operator, as a launch that hung.
`allocation_names_a_missing_directory_and_says_what_to_do` pins both halves of
the refusal: the directory's own path, and the imperative that ends it.

The two failures are separate arms because they are separate operator mistakes.
`fs::metadata` failing is *there is nothing here*; `!metadata.is_dir()` is *there
is something here and it is a file*. The second could have been folded into the
first by letting `dir.join(…)` fail later, and the cost would have been a message
naming a path inside a non-directory, which does not name the thing the operator
got wrong.

The second job is the loop, and it turns on one call the comments do not discuss.
Occupancy is tested with **`fs::symlink_metadata`**, not `fs::metadata`. Both
answer *is this name taken*, and for an ordinary file they answer identically —
`Ok`, which is the `continue` arm. They part company only where the entry is a
symlink, because `symlink_metadata` describes the link and `metadata` describes
whatever the link points at, and that difference reaches this loop in two ways.

A **dangling** symlink — one whose target does not exist — is `Ok` to
`symlink_metadata` and `NotFound` to `metadata`. Written the second way, the loop
would call such a name free and return it; the launcher would publish it to the
child, and whatever the child uses to write the token — `signal`'s `fs::write`,
or a shell redirection, which is what this crate's own launch tests use — would
follow the link and create the file at the target instead. The target need not be
anywhere dramatic: a *relative* link resolves against the same directory, so the
token can land on a neighbour the caller does keep there. Because the link is
dangling, the effect is creation rather than overwrite, and it fails outright if
the target's own parent is missing.

A **symlink loop**, or a target behind a directory the process cannot traverse,
is the other way and the sharper one, because it costs a launch rather than
misplacing a file. `symlink_metadata` returns `Ok` and the loop simply redraws.
`metadata` returns `FilesystemLoop` or a permission error — not `NotFound` — so it
falls into the `Err(error)` arm below, and a name that could have been abandoned
in favour of the next draw becomes a hard allocation failure instead. That arm is
right for what it is meant to catch, a directory whose entries cannot be
inspected at all; it is the wrong answer for one unusable name among 2^128.

**Read this as hygiene, not as a guarantee, and the crate offers no guarantee
here.** Nothing re-examines the path between the draw and the child's write;
`Path::exists`, which chapter 8 polls, and `read`'s `fs::read_to_string` both
follow links like any other reader. And for a symlink to be standing at a drawn
name in the first place, something would have to have predicted 128 bits — the
same broken-randomness condition `DRAW_RETRY_LIMIT`'s message already reports. So
the claim to take from this line is the modest one: `symlink_metadata` is the
strictly correct spelling of *is this name taken*, it costs nothing over the
alternative, and it is right in both cases where the two spellings differ.
**No test pins any of it.** The three inline tests cover the ordinary draw,
non-collision across two allocations, and the missing directory; the two
paragraphs above are properties of the `std` calls, stated here because the
source is silent on them, and they are the only claims on this page resting on
library behaviour rather than on a test in this repository.

`Ok(_) => continue` is the collision arm, and note what it does not do: it does
not read, stat, remove, or report whatever occupies the name. *Retrying an
occupied name without touching whatever occupies it* is the signature's promise,
and it is the same restraint `discard_abandoned` shows further down for the same
reason — the directory belongs to the consumer, and the names this crate writes
to are the ones it drew itself.

<a id="the-published-path"></a>
## The path the child is handed

Three lines of body and a two-line comment, and the comment is the interesting
part: *this is the value a launch publishes to the child under the caller's
chosen variable name*. Two facts are deferred by that sentence rather than
asserted. The publishing is chapter 7's — `run` sets the variable immediately
before the spawn — and **the variable's name is the caller's**, so this crate
does not know that grove calls it `GROVE_SIGNAL_FILE`. The book's worked example
uses that name because grove chose it; `crates/keyed-launch/tests/launch.rs`
publishes the same path under `TEST_CHANNEL` throughout, and nothing in these 271
lines can tell the difference.

<!-- fragment «channel-published-path» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="93-99" parent="channel-production" -->
````rust

    /// The path a child writes its token to. This is the value a launch
    /// publishes to the child under the caller's chosen variable name.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
````
<!-- /fragment -->

`the_channel_path_is_published_under_the_callers_chosen_variable_name` is the
test that closes the loop through both ends of the file at once: the child script
writes the value of its own channel variable *into* the channel, and the launcher
asserts that the token it reads back equals `channel.path()`. The two agree only
if the path this accessor returned is the path the child was handed. It is also
the plainest demonstration on this page that the token is not interpreted — the
token in that test is a filesystem path, and the crate treats it exactly as it
treats `relaunch`.

The borrow is a `&Path` rather than a clone because the caller wants to read it,
not own it, and `#[must_use]` is here for the same reason it is on `source` and
`keys` in chapter 5: the returned value is the only reason to call.

<a id="three-ways-to-have-no-token"></a>
## Three ways to have no token, and one answer for all of them

`read` is five lines of body under twelve of comment, and the comment is doing
the harder half of the work.

<!-- fragment «channel-read» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="100-118" parent="channel-production" -->
````rust

    /// The token a launch left here, if any.
    ///
    /// `None` covers *nothing was written*, *the file is unreadable*, and *the
    /// file is there but empty*: none of the three is a token, and the
    /// difference is not one a caller could act on differently — a launch that
    /// could not deliver its token did not deliver one.
    ///
    /// **An empty file is deliberately not an empty token.** The escalation
    /// fires on the channel's *appearance*, so a child killed between creating
    /// the file and writing to it leaves one behind; handing that back as
    /// `Some("")` would make a caller's own "anything unrecognised means keep
    /// going" rule fire on a launch that never said anything at all.
    #[must_use]
    pub fn read(&self) -> Option<Token> {
        let content = fs::read_to_string(&self.path).ok()?;
        let token = content.trim_end();
        (!token.is_empty()).then(|| Token(token.to_string()))
    }
````
<!-- /fragment -->

The signature is `Option<Token>`, and the first comment paragraph is an argument
for why that is not a lossy return type. Nothing was written, the file could not
be read, and the file is there but empty are three different situations in the
filesystem and one situation for a caller: **no token came back**. Collapsing
them is defensible only because no caller could act on the difference — a launch
that could not deliver its token did not deliver one — and the `.ok()?` on the
read is where the second of the three is collapsed into the first.

The second paragraph is the sharper claim and it is the one the inline module
tests hardest. An empty file is **not** an empty token. The reason is a
consequence of this chapter's own thesis reaching into chapter 8: because the
escalation fires on *appearance*, a child killed in the window between creating
the file and writing to it leaves a real, empty file behind. Handing that back as
`Some("")` would be a token by the type system's lights, and a caller whose rule
is *anything I do not recognise means keep going* would then act on a launch that
said nothing at all. Grove is exactly such a caller:
`crates/grove-loop/src/complete.rs`'s `interpret` compares the token against one
constant and treats every other present value as `Relaunch`. So `Some("")` would
relaunch a session that had been killed mid-write, which is the one outcome a
completion channel exists to prevent. `an_empty_channel_file_is_not_an_empty_token`
runs `""`, `"\n"` and `"  \n"` through it and expects `None` for all three.

The trimming is `trim_end`, and the exact choice matters twice. It removes
trailing whitespace generally rather than exactly the one newline `signal` added,
so a child that writes with a trailing blank line still reads back the same
token — the framing is undone even when the child added a little more of it than
it needed. And it leaves the **front** of the content alone: a token with leading
spaces comes back with them. That is the line where *framing, not interpretation*
stops being a slogan and becomes a decision, because trimming both ends would
have been one character shorter to write and would have meant this crate
normalising a value it does not understand.
`a_signalled_channel_reads_back_the_token_without_its_framing` asserts both sides
of it at once: the token reads back as `done`, and the file on disk still holds
`done\n`.

<a id="discarding"></a>
## Removing this launch's file

The launcher's own cleanup is three lines of body under six of comment, and the
comment carries two decisions: what the method does to the value it is called
on, and what it promises about the path.

<!-- fragment «channel-discard» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="119-128" parent="channel-production" -->
````rust

    /// Remove this launch's channel file, consuming the channel so nothing can
    /// read a path whose file is gone.
    ///
    /// A channel that was never signalled has no file, and discarding it is
    /// still success: the post-condition is *this path holds nothing*, not
    /// *this call removed something*.
    pub fn discard(self) -> Result<(), LaunchError> {
        remove_if_present(&self.path)
    }
````
<!-- /fragment -->

`discard` takes `self` by value, and the comment says why in its first clause:
consuming the channel is what stops anything reading a path whose file is gone.
That is the same reasoning as chapter 5's `Argv` having no public constructor,
one type earlier — a property enforced by the compiler costs nothing to maintain
and cannot be forgotten by a caller.

The second paragraph states a post-condition rather than an effect, and the
distinction is the whole of `remove_if_present` further down. *This path holds
nothing* is true whether or not this call removed something, so an unsignalled
launch — which leaves no file at all — discards successfully.
`discarding_removes_the_file_and_succeeds_when_there_was_none` runs both cases in
one test, and `successive_launches_get_independent_channels` closes the harder
version of the same claim from the outside: two launches, two channels, both
discarded, and a control directory with zero entries afterwards.

<a id="the-cleanup-that-must-not-overreach"></a>
## The cleanup that must not overreach

`discard_abandoned` is an associated function rather than a method because the
channels it removes belong to launchers that are gone; there is no `Channel`
value left to call it on.

<!-- fragment «channel-discard-abandoned» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="129-184" parent="channel-production" -->
````rust

    /// Remove every channel file in `dir` — the ones a previous launcher
    /// allocated and did not live to discard.
    ///
    /// **The name grammar is this crate's, so recognising an abandoned channel
    /// has to be too.** The alternative is a consumer open-coding
    /// `signal-<32 hex>` in its own cleanup, which is a second spelling of a
    /// rule that only one place should hold.
    ///
    /// Allocation does not depend on this: it draws fresh names and retries
    /// occupied ones. This is hygiene, and a caller that cannot afford to fail
    /// on it may report the error and carry on.
    pub fn discard_abandoned(dir: &Path) -> Result<(), LaunchError> {
        let entries = fs::read_dir(dir).map_err(|error| {
            LaunchError::new(format!(
                "cannot list abandoned completion channels in {}: {error}; make the directory \
                 readable and retry",
                dir.display()
            ))
        })?;

        // Every entry is attempted before anything is reported. Stopping at the
        // first failure would leave channels behind that this pass could have
        // removed, and the one unremovable file would hide the rest.
        let mut failures = Vec::new();
        for entry in entries {
            let path = match entry {
                Ok(entry) => entry.path(),
                Err(error) => {
                    failures.push(format!("reading a directory entry: {error}"));
                    continue;
                }
            };
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if !is_channel_name(name) {
                continue;
            }
            if let Err(error) = remove_if_present(&path) {
                failures.push(format!("{error}"));
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(LaunchError::new(format!(
                "could not remove {} abandoned completion channel(s) in {}; remove them by hand:\n- {}",
                failures.len(),
                dir.display(),
                failures.join("\n- ")
            )))
        }
    }
}
````
<!-- /fragment -->

The comment's first paragraph places the responsibility, and the placement is the
argument: **the name grammar is this crate's, so recognising an abandoned channel
has to be too.** The alternative is not that cleanup goes undone — it is that the
consumer open-codes `signal-<32 hex>` in its own housekeeping, and one rule then
lives in two places that must agree forever. Grove's own consumer is where that
division is visible: `crates/grove-loop/src/driver_lease.rs` calls
`keyed_launch::Channel::discard_abandoned(&lease.control_dir)` and its comment
says the split in one line — *the grammar of an abandoned channel is the runner's,
so the runner recognises them; the lease supplies only the directory*. The
consumer knows which directory and when; the crate knows which names.

The second paragraph is a licence rather than a description, and grove takes it.
Allocation draws fresh names and retries occupied ones, so nothing about a
successful launch depends on the directory being clean; that is why the comment
can say a caller unable to afford failure here may report and carry on. Grove's
lease does exactly that — it prints a warning naming the error and continues into
the launch — and the reason it can is stated in this comment rather than in
grove's.

The body has one structural decision and it is in the comment above the loop:
**every entry is attempted before anything is reported.** Stopping at the first
failure would leave removable channels behind and, worse, would let one
unremovable file hide the existence of the rest — an operator fixing the reported
file would run the cleanup again and meet the next one. So failures accumulate
into a `Vec` and the refusal is one message with a count and a bulleted list. The
two `continue` arms are the same restraint stated twice: a directory entry that
cannot be read is a failure worth reporting, while a name that is not valid UTF-8
is simply not one of ours and is skipped in silence, because a non-UTF-8 name
cannot be a channel name — the grammar is thirty-two ASCII hex characters after
an ASCII prefix.

`abandoned_cleanup_removes_channels_and_leaves_every_other_entry_alone` is the
test that pins the whole function, and it is built to fail in the four different
ways a looser rule would: an uppercase nonce, a short nonce, an unprefixed name,
and an ordinary neighbouring file called `driver.lease`. All four must survive;
only the exact name is removed.
`abandoned_cleanup_names_the_directory_when_it_cannot_be_listed` pins the other
refusal.

<a id="opaque-here-readable-there"></a>
## Opaque here, readable there

`Token` is a newtype over `String` with two accessors, and its comment states the
one property that makes the launch half of this crate possible.

<!-- fragment «channel-token» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="185-202" parent="channel-production" -->
````rust

/// What a launch left in its channel. **Opaque to this crate**: its appearance
/// ends the launch, and its content is the caller's to interpret, which is why
/// the content is readable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token(String);

impl Token {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}
````
<!-- /fragment -->

**Opaque to this crate; readable to its caller** is a single sentence holding two
decisions that pull in opposite directions, and the comment states the second as
a consequence of the first: the content is readable *because* the appearance,
not the content, is what ends the launch. Had the crate needed to know what the
token said, the type would have been an enum and the crate would have owned a
vocabulary of endings — which is the crate learning what a value means on the way
out, precisely what the book's third arm names. Had the crate instead refused to
expose the content at all, the launch would have ended with no information
crossing back, and every consumer would have needed a second channel of its own.
Wrapping the string is the middle: the crate carries it and does not read it.

Grove is the caller that reads it, and the division is legible in one file.
`crates/grove-loop/src/complete.rs` imports `keyed_launch::Token`, and
`interpret(token: Option<&Token>)` is eight lines: `None` stays `None`, a token
equal to grove's `DONE_TOKEN` becomes `Done`, and anything else becomes
`Relaunch`. Every word of that policy — including the deliberate reading of an
unrecognised value as `Relaunch`, which is what keeps an older binary's legacy
token working — is grove's, written in grove's crate, over a string this crate
handed across without comparison. The same file's `signal` wrapper is the other
end: it calls `keyed_launch::signal` with grove's own word.

The two accessors are the borrow and the move, and both are `#[must_use]` for the
same reason as `path`. `as_str` is what a comparison wants; `into_string` is what
a caller wants when the launch is over and the token is the only thing worth
keeping —`the_channel_path_is_published_under_the_callers_chosen_variable_name`
uses it, and so does any consumer storing the token past the `Ended` that carried
it. The derives are `Clone, Debug, PartialEq, Eq`: equality is here because
`Ended` holds an `Option<Token>` and callers and tests compare them —
`successive_launches_get_independent_channels` asserts two launches' tokens
differ — and there is no `Display`, because printing a token as though it were a
message would be the first step toward reading it.

<a id="the-other-end"></a>
## The other end of the channel

`signal` is the only item in the file that runs in the child rather than the
launcher, and its shape is decided by that fact.

<!-- fragment «channel-signal» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="203-221" parent="channel-production" -->
````rust

/// Write `token` to `path` — the one thing a launched child does to end itself.
///
/// A free function rather than a [`Channel`] method because the two ends are
/// different processes: the launcher holds the `Channel`, and the child holds
/// only the path it was handed under the launch's chosen variable name.
///
/// The file is line-framed — the token followed by a newline — so `cat` on it
/// reads as a line and [`Channel::read`] trims the framing back off. That is
/// framing, not interpretation: nothing here looks at what the token says.
pub fn signal(path: &Path, token: &str) -> Result<(), LaunchError> {
    fs::write(path, format!("{token}\n")).map_err(|error| {
        LaunchError::new(format!(
            "cannot write the completion token to {}: {error}; the launcher allocated this path \
             and its directory should exist — check that the channel was not removed early",
            path.display()
        ))
    })
}
````
<!-- /fragment -->

A **free function rather than a `Channel` method**, and the comment gives the
reason without hedging: the two ends are different processes. The launcher holds
the `Channel`; the child holds a path it read out of an environment variable, and
nothing it could do would reconstitute the launcher's value. Making this a method
would have required the child to construct a `Channel` from a path — a
constructor whose only purpose is to let a process pretend it allocated something
it did not, and one that would have made `Channel` a type two unrelated processes
both claim to own. As a free function the child's half of the protocol is exactly
what it should be: a path in, a string in, a file out.

The framing paragraph is the one to read against `read`, four sections above.
`signal` writes `{token}\n` and `read` trims the trailing whitespace back off, so
the file is a line and the value is not. *That is framing, not interpretation:
nothing here looks at what the token says* — and the `format!` is the proof, since
the only thing done to the token is concatenation. A launch whose token contains
a newline would produce a two-line file and read back only through `trim_end`;
nothing in the crate refuses it, because refusing it would be a rule about
content.

The refusal is the file write failing, and its second half is unusually specific
about whose fault it is: *the launcher allocated this path and its directory
should exist — check that the channel was not removed early*. That sentence is
addressed to a reader debugging a consumer, and it names the one thing that can
have gone wrong given everything `allocate` already checked. It is the mirror of
`allocate`'s directory check: the earlier call exists so that this one, running
inside a child whose stderr nobody may be reading, is not where a missing
directory is discovered.

<a id="exactly-this-name"></a>
## Exactly this name

`is_channel_name` is the grammar `discard_abandoned` recognises, in nine lines.

<!-- fragment «channel-name-grammar» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="222-236" parent="channel-production" -->
````rust

/// Exactly [`CHANNEL_PREFIX`] followed by 32 lowercase hex characters.
///
/// Deliberately exact. A looser rule would let this crate's cleanup delete a
/// neighbouring file that merely starts the same way, in a directory whose
/// other contents belong to the consumer.
fn is_channel_name(name: &str) -> bool {
    let Some(suffix) = name.strip_prefix(CHANNEL_PREFIX) else {
        return false;
    };
    suffix.len() == NONCE_BYTES * 2
        && suffix
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
````
<!-- /fragment -->

**Deliberately exact**, and the comment names the cost of the alternative in one
clause: a looser rule would let this crate's cleanup delete a neighbouring file
that merely starts the same way, in a directory whose other contents belong to
the consumer. That is not hypothetical — grove's control directory holds
`driver.lease` and `session.epoch` beside the channels, and both are files whose
loss would end the loop. A `starts_with("signal-")` test would be four
characters shorter and would put every neighbour at the mercy of a naming
coincidence.

The three conditions are prefix, length and alphabet, and each rules out one of
the ways a name can be nearly right. The length is written as `NONCE_BYTES * 2`
rather than as `32`, which is the coupling worth noticing: this predicate and the
`hex` function at the bottom of the file are two halves of one grammar, and both
are derived from the same constant, so a change to the nonce width cannot leave
the recogniser behind. The alphabet is lowercase only —
`byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)` — which matches exactly
what `hex`'s `{byte:02x}` produces and nothing else.

That lowercase restriction is the condition the inline test is most careful
about, and its comment explains a subtlety that is easy to get wrong when writing
such a test: the uppercase decoy uses a *different* nonce from the real channel,
because on a case-insensitive filesystem the same nonce in two cases would be one
file and the test would assert nothing. It is the sort of detail that makes the
difference between a test that pins the rule and a test that passes.

<a id="the-three-helpers"></a>
## Three helpers, and the one dependency they do not need

The last three functions are private, small, and each closes a question raised
earlier on the page. The first is the post-condition `discard` and
`discard_abandoned` both promise.

<!-- fragment «channel-remove-if-present» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="237-247" parent="channel-production" -->
````rust

fn remove_if_present(path: &Path) -> Result<(), LaunchError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(LaunchError::new(format!(
            "cannot remove the completion channel {}: {error}; remove it by hand",
            path.display()
        ))),
    }
}
````
<!-- /fragment -->

`NotFound` is mapped to `Ok(())` here and nowhere else, which is what makes *this
path holds nothing* the post-condition of both callers rather than a claim each
has to make separately. Every other error is a refusal naming the path and ending
in *remove it by hand* — the same imperative shape chapter 4 established for the
template diagnostics, and the reason `discard_abandoned`'s aggregate message can
simply interpolate `{error}` for each failure and still read as a list of
actionable sentences.

The second is where the crate's dependency rule meets its one requirement for
randomness.

<!-- fragment «channel-draw-nonce» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="248-261" parent="channel-production" -->
````rust

fn draw_nonce() -> Result<[u8; NONCE_BYTES], LaunchError> {
    let mut source = File::open("/dev/urandom").map_err(|error| {
        LaunchError::new(format!("cannot open the OS randomness source: {error}"))
    })?;
    let mut nonce = [0_u8; NONCE_BYTES];
    source.read_exact(&mut nonce).map_err(|error| {
        LaunchError::new(format!(
            "cannot read {} bytes of OS randomness for a completion-channel name: {error}",
            NONCE_BYTES
        ))
    })?;
    Ok(nonce)
}
````
<!-- /fragment -->

Sixteen bytes are read from `/dev/urandom` with `File::open` and `read_exact` —
`std` and nothing else. This is the concrete instance of the manifest rule
chapter 1 read: *two document formats this crate reads, and the syscalls it
cannot reach from `std`*. A randomness crate would have been a fourth dependency
satisfying neither clause, and the four lines above are what the crate does
instead. `read_exact` rather than `read` is the correct call and not merely the
convenient one: a short read would otherwise yield a nonce with predictable
trailing zeros and no error at all, and the error arm is where the operator is
told which of *open* and *read* failed and how many bytes were wanted.

Neither refusal is reachable in this repository's tests — a machine without
`/dev/urandom` is not a case the suite constructs — so unlike every other
diagnostic on this page these two are pinned by nothing, and are here because
`DRAW_RETRY_LIMIT`'s message promises the operator a `/dev/urandom` to check.

The third is the rendering half of the grammar.

<!-- fragment «channel-hex» owner="appearance-is-the-event" source="crates/keyed-launch/src/channel.rs" lines="262-271" parent="channel-production" -->
````rust

fn hex(nonce: [u8; NONCE_BYTES]) -> String {
    let mut rendered = String::with_capacity(NONCE_BYTES * 2);
    for byte in nonce {
        // Infallible: `String`'s `Write` never errors, and the format is fixed.
        let _ = write!(&mut rendered, "{byte:02x}");
    }
    rendered
}

````
<!-- /fragment -->

`hex` is the function `is_channel_name` recognises the output of, and the two are
written to be read together: `{byte:02x}` is zero-padded, two characters, and
lowercase, which is precisely the alphabet and the length the predicate accepts.
The capacity is `NONCE_BYTES * 2` from the same constant that sets the
predicate's length, so all three of the grammar's numbers trace to the one
declaration at the top of the file.

The comment on the discarded result is the last line of the block and it is
doing real work: `write!` to a `String` returns a `Result` because it goes
through `fmt::Write`, and that `Result` is infallible here for two independent
reasons — `String`'s implementation never errors, and the format string holds no
user input that could fail to render. Naming both is what makes `let _ =` a
decision rather than a suppressed warning, and it is why this file needs the
`use std::fmt::Write as _` import that the thesis fragment opened with.

That is the whole channel. A directory the caller owns, a name drawn once and
written by nobody, a file that appears only if a child put it there, and a string
that crosses two processes without either end of this crate reading it. The
launch has a path to end on and an `Argv` to run, and neither has yet met the
other. Chapter 7 is where they do: `run` takes both, publishes the path under
the caller's chosen variable, and spawns the child with nothing added that the
operator did not write.

[Previous: From a template to an argv](05-to-an-argv.md) | [Contents](README.md)
