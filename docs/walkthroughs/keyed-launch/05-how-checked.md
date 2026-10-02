# How this is checked
<!-- book-page id="how-checked" slice="checked-without-meaning" order="5" -->
[Previous: The watch and the escalation](04-the-escalation.md) | [Contents](README.md) | [Next: What passes through](06-what-passes-through.md)

<a id="checked-without-meaning"></a>
## Checked without meaning

This chapter owns the last 165 lines of `src/channel.rs`: an inline test module
that holds a filename to a grammar and never asks what the name refers to. It
is the only source this chapter owns.

The rest of the crate's evidence is outside the corpus. The integration tests
under `crates/keyed-launch/tests/` link the crate as an external library and
launch real children: `launch.rs` for the job, the channel, the environment and
the escalation, `interrupt.rs` and `reraise.rs` for the launcher's own signals,
`noninteractive.rs` for a child with no terminal, and `confinement.rs` for the
filesystem policy. Every one of them builds its command with `Argv::new`. The
book cites them where they adjudicate a claim and reproduces none of them.

<!-- fragment «channel-inline-tests» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="289-453" parent="source-channel" -->
<!-- insert «channel-tests-module» -->
<!-- insert «channel-tests-allocate» -->
<!-- insert «channel-tests-read» -->
<!-- insert «channel-tests-discard» -->
<!-- insert «channel-tests-cleanup» -->
<!-- /fragment -->

<a id="inside-the-root"></a>
## What a module inside the root reaches

The rest of this chapter is 165 lines that are neither production code nor
outside the corpus. They are the end of `src/channel.rs`, compiled only under
`cfg(test)`, and they are the only tests in the crate that can see a private
item.

<!-- fragment «channel-tests-module» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="289-291" parent="channel-inline-tests" -->
````rust
#[cfg(test)]
mod tests {
    use super::*;
````
<!-- /fragment -->

`use super::*;` is what the module is for. An integration test under
`crates/keyed-launch/tests/` links the crate as an external library and sees the
public surface. A module
inside the file sees everything the file declares, and for `src/channel.rs` the
difference is seven items: the constants `CHANNEL_PREFIX`, `NONCE_BYTES` and
`DRAW_RETRY_LIMIT`, and the private functions `is_channel_name`,
`remove_if_present`, `draw_nonce` and `hex`. One of the seven is the reason the
module exists. Chapter 2 argued `is_channel_name`'s **exactness** at length — prefix, length and
lowercase alphabet as three conditions, each ruling out one way a name can be
nearly right, against a looser rule that would let this crate's cleanup delete a
neighbouring file — and the only test that can call it directly is in this
module.

That is why the book cuts `src/channel.rs` at line 289 and gives the two pieces
to different chapters, and it is the only ownership boundary in the book taken at
a compilation condition rather than at a concept. The module's *subject matter*
is chapter 2's, since every function it exercises is above line 289; its
*subject* is assurance, which is this chapter's. Explaining the tests here, with
chapter 2's fragments behind the reader, costs that chapter one forward reference
and puts the evidence for one claim beside the evidence for every other claim
about the same crate.

The eleven tests divide by the method each exercises, which is how the four
sections below take them. The table is the map: what each test holds, and where
in chapter 2 the rule it holds was argued.

| Test | What it holds | Argued at |
|---|---|---|
| `an_allocated_channel_names_a_path_that_does_not_yet_exist` | allocation picks a name and creates nothing, and the name it picks satisfies the grammar | [Drawing a name](02-the-channel.md#drawing-a-name), [Exactly this name](02-the-channel.md#exactly-this-name) |
| `successive_allocations_in_one_directory_never_collide` | a nonce is drawn per call, not per directory or per process | [Drawing a name](02-the-channel.md#drawing-a-name) |
| `allocation_names_a_missing_directory_and_says_what_to_do` | the directory is checked before anything is spawned, and the refusal names the fix | [Drawing a name](02-the-channel.md#drawing-a-name) |
| `a_signalled_channel_reads_back_the_token_without_its_framing` | `signal` frames the token with a newline and `read` trims the framing back off | [The other end of the channel](02-the-channel.md#the-other-end), [Three ways to have no token](02-the-channel.md#three-ways-to-have-no-token) |
| `an_empty_channel_file_is_not_an_empty_token` | an empty or whitespace-only file reads as `None`, never as `Some("")` | [Three ways to have no token](02-the-channel.md#three-ways-to-have-no-token) |
| `an_unsignalled_channel_reads_back_nothing` | a path nothing ever wrote to reads as `None` | [Three ways to have no token](02-the-channel.md#three-ways-to-have-no-token) |
| `completion_rejects_links_and_oversized_tokens` | symlinks, FIFO and content beyond 4,096 bytes yield no token | [Bounded channel read](02-the-channel.md#three-ways-to-have-no-token) |
| `completion_stays_in_its_original_directory` | replacing the parent pathname cannot redirect the token read | [Held channel directory](02-the-channel.md#writes-nothing) |
| `discarding_removes_the_file_and_succeeds_when_there_was_none` | the post-condition is *this path holds nothing*, both when there was a file and when there was not | [Removing this launch's file](02-the-channel.md#discarding), [Three helpers](02-the-channel.md#the-three-helpers) |
| `abandoned_cleanup_removes_channels_and_leaves_every_other_entry_alone` | cleanup removes exactly the names the grammar accepts and nothing else in the directory | [The cleanup that must not overreach](02-the-channel.md#the-cleanup-that-must-not-overreach), [Exactly this name](02-the-channel.md#exactly-this-name) |
| `abandoned_cleanup_names_the_directory_when_it_cannot_be_listed` | a cleanup that cannot list its directory refuses and names it | [The cleanup that must not overreach](02-the-channel.md#the-cleanup-that-must-not-overreach) |

Nothing in the module spawns a process, and that is the other half of the
boundary. All eleven build a `tempfile::tempdir()` and work on the filesystem, so
the module's whole reach is the channel's file-facing side. The process-facing
side — that the path is published to a child under the caller's chosen variable
name, and that a child writing to it ends the launch — needs a real child and
lives in `crates/keyed-launch/tests/launch.rs`. The two surfaces are
complementary rather than alternative: one reaches what is private, the other
reaches what needs an operating system.

<a id="a-name-and-not-a-file"></a>
## Three tests on a name that is not yet a file

The first three tests are `Channel::allocate` and nothing else. Each hands it a
directory and reads one thing back: the path it drew, the second path it drew,
and the refusal when the directory is not there.

<!-- fragment «channel-tests-allocate» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="292-329" parent="channel-inline-tests" -->
````rust

    #[test]
    fn an_allocated_channel_names_a_path_that_does_not_yet_exist() {
        let dir = tempfile::tempdir().unwrap();

        let channel = Channel::allocate(dir.path()).unwrap();

        assert!(
            !channel.path().exists(),
            "allocation must pick a name, not create a file — appearance is the event"
        );
        assert_eq!(channel.path().parent(), Some(dir.path()));
        assert!(is_channel_name(
            channel.path().file_name().unwrap().to_str().unwrap()
        ));
    }

    #[test]
    fn successive_allocations_in_one_directory_never_collide() {
        let dir = tempfile::tempdir().unwrap();

        let first = Channel::allocate(dir.path()).unwrap();
        let second = Channel::allocate(dir.path()).unwrap();

        assert_ne!(first.path(), second.path());
    }

    #[test]
    fn allocation_names_a_missing_directory_and_says_what_to_do() {
        let dir = tempfile::tempdir().unwrap();
        let absent = dir.path().join("never-created");

        let error = Channel::allocate(&absent).unwrap_err();

        let message = error.to_string();
        assert!(message.contains(&absent.display().to_string()), "{message}");
        assert!(message.contains("create the directory"), "{message}");
    }
````
<!-- /fragment -->

The first test states three parts of chapter 2's central claim in order: the path
does not exist, its parent is the directory it was
given, and its file name satisfies the grammar. The first carries the only
assertion message in the crate that states the spine as a sentence —
*allocation must pick a name, not create a file — appearance is the event* — and
that phrase occurs exactly once in the whole of `src/`.

The third assertion is the direct call to `is_channel_name`, and it is the reason
this module sits inside the root rather than under `tests/`. It carries the only
assertion anywhere on `hex`'s output — every test that allocates runs `hex`, and
this is the one that checks what it produced: the name satisfies the grammar only
if sixteen drawn bytes came back as thirty-two lowercase hex characters, which is
the half of the grammar `hex` owns and the reason chapter 2's table credits that
function to this test.

The second test is narrower than its name. Nothing occupies a name while it runs,
so `symlink_metadata` returns `NotFound` on the first draw both times and the
retry `continue` arm is never entered. What it pins is that a nonce is drawn per
call: an `allocate` that cached one per directory or per process would fail here.
The loop around the draw runs on every allocation in the crate; what no test
reaches is its `Ok(_) => continue` arm and the refusal that follows the loop.

The third asserts on the message with two `contains` rather than on the whole
string — the absent path, and *create the directory*. That is chapter 1's refusal
obligation checked in the shape it was stated, name where and name the fix, while
leaving the sentence editable. The refusal's second clause, *or pass one that
exists*, is deliberately not pinned.

Three of `allocate`'s four refusals are named by no test in the crate: a
directory argument that exists but is not a directory, a drawn path whose
existence cannot be established, and eight consecutive occupied draws. The first
two need a filesystem state a test would have to manufacture, and the third needs
`/dev/urandom` to repeat itself.

<a id="not-a-token"></a>
## Three tests on what is not a token

The next three are `Channel::read`, and between them they are chapter 2's
distinction between a file and a token: one write that produces a token, and two
files that do not.

<!-- fragment «channel-tests-read» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="330-368" parent="channel-inline-tests" -->
````rust

    #[test]
    fn a_signalled_channel_reads_back_the_token_without_its_framing() {
        let dir = tempfile::tempdir().unwrap();
        let channel = Channel::allocate(dir.path()).unwrap();

        signal(channel.path(), "done").unwrap();

        assert_eq!(channel.read().unwrap().as_str(), "done");
        assert_eq!(
            std::fs::read_to_string(channel.path()).unwrap(),
            "done\n",
            "the file itself stays line-framed"
        );
    }

    /// The channel's *appearance* is what starts an escalation, so a child
    /// killed between creating the file and writing to it leaves an empty one.
    /// That is not a token, and reporting it as `Some("")` would let a caller's
    /// "anything unrecognised means keep going" rule fire on a launch that said
    /// nothing at all.
    #[test]
    fn an_empty_channel_file_is_not_an_empty_token() {
        let dir = tempfile::tempdir().unwrap();
        let channel = Channel::allocate(dir.path()).unwrap();

        for content in ["", "\n", "  \n"] {
            std::fs::write(channel.path(), content).unwrap();
            assert_eq!(channel.read(), None, "{content:?} is not a token");
        }
    }

    #[test]
    fn an_unsignalled_channel_reads_back_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let channel = Channel::allocate(dir.path()).unwrap();

        assert_eq!(channel.read(), None);
    }
````
<!-- /fragment -->

The first of the three asserts both ends of one write: `channel.read()` gives
`done`, and `std::fs::read_to_string` on the same path gives `done\n`. It is the
only test in the module that reads the channel file directly rather than through
the type, and it has to be, because the claim is the *difference* between the two
readings and no single accessor can show it. The framing is `signal`'s and the
trimming is `read`'s, and neither call looked at what sat between them.

The second is the only test in the module carrying a doc comment, and the comment
is an argument rather than a description: the channel's appearance is what starts
an escalation, so a child killed between creating the file and writing to it
leaves an empty one behind, and reporting that as `Some("")` would let a caller's
*anything unrecognised means keep going* rule fire on a launch that said nothing
at all. That is chapter 2's `read` comment restated where the case is exercised.
The loop over `["", "\n", "  \n"]` is what makes it three cases rather than one,
and the third is the one that matters: it exercises `trim_end` past whitespace
that is not a newline, and a `read` that stripped only a trailing `\n` would
return `Some("  ")` and fail there.

The third test completes a set. Chapter 2 named
three ways to have no token — nothing was written, the file cannot be read, and
the file is there but empty — and this module reaches the first and the third.
The unsignalled test exercises the `.ok()?` arm through `NotFound`, collapsing
the read failure into an absent token. No test supplies the middle premise: an
existing file whose read returns `Err`.

<a id="a-post-condition"></a>
## A post-condition, tested from both of its sides

Two adversarial read tests precede the discard test in source order. They
establish that a channel cannot turn a symlink, oversized file, FIFO or
replaced parent pathname into an accepted completion. The discard test then
checks its post-condition with both a signalled and an untouched channel.

<!-- fragment «channel-tests-discard» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="369-414" parent="channel-inline-tests" -->
<!-- insert «channel-untrusted-entry-test» -->
<!-- insert «channel-parent-identity-test» -->
<!-- insert «channel-discard-postcondition-test» -->
<!-- /fragment -->

<a id="channel-untrusted-entry-test"></a>
### Refuse hostile channel entries

The first test substitutes a symlink, then a 4,097-byte file, then a FIFO at
one channel name. Each read must return no token; the FIFO assertion also
exercises the nonblocking open. These fixtures hold the supervisor's bounded
read contract without running a harness.

<!-- fragment «channel-untrusted-entry-test» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="369-387" parent="channel-tests-discard" -->
````rust

    #[test]
    fn completion_rejects_links_and_oversized_tokens() {
        let dir = tempfile::tempdir().unwrap();
        let channel = Channel::allocate(dir.path()).unwrap();
        let outside = dir.path().join("outside");
        fs::write(&outside, "done").unwrap();
        std::os::unix::fs::symlink(&outside, channel.path()).unwrap();
        assert_eq!(channel.read(), None, "completion must not follow a link");
        fs::remove_file(channel.path()).unwrap();
        fs::write(channel.path(), vec![b'x'; 4097]).unwrap();
        assert_eq!(channel.read(), None, "completion must have a bounded size");
        fs::remove_file(channel.path()).unwrap();
        let path = std::ffi::CString::new(channel.path().as_os_str().as_encoded_bytes()).unwrap();
        // SAFETY: valid NUL-terminated path in a private test directory.
        assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
        assert_eq!(channel.read(), None, "completion must not block on a FIFO");
    }

````
<!-- /fragment -->

<a id="channel-parent-identity-test"></a>
### Keep the allocated directory identity

The second test places a valid-looking token in a different directory, renames
the original channel directory, and installs a symlink under its old pathname.
`Channel::read` still returns no token because its held descriptor names the
original directory. This distinguishes stable directory identity from merely
checking the final filename for symlinks.

<!-- fragment «channel-parent-identity-test» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="388-401" parent="channel-tests-discard" -->
````rust
    #[test]
    fn completion_stays_in_its_original_directory() {
        let dir = tempfile::tempdir().unwrap();
        let control = dir.path().join("control");
        fs::create_dir(&control).unwrap();
        let channel = Channel::allocate(&control).unwrap();
        let outside = dir.path().join("outside");
        fs::create_dir(&outside).unwrap();
        fs::write(outside.join(channel.path().file_name().unwrap()), "done").unwrap();
        fs::rename(&control, dir.path().join("original")).unwrap();
        std::os::unix::fs::symlink(outside, &control).unwrap();
        assert_eq!(channel.read(), None);
    }

````
<!-- /fragment -->

<a id="channel-discard-postcondition-test"></a>
### Discard a present or absent file

The discard test signals one channel and leaves another untouched, then
consumes both. Both paths must be absent afterwards. The two inputs establish
that discard promises the same final state whether or not completion ever
created a file.

<!-- fragment «channel-discard-postcondition-test» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="402-414" parent="channel-tests-discard" -->
````rust
    #[test]
    fn discarding_removes_the_file_and_succeeds_when_there_was_none() {
        let dir = tempfile::tempdir().unwrap();
        let signalled = Channel::allocate(dir.path()).unwrap();
        signal(signalled.path(), "relaunch").unwrap();
        let signalled_path = signalled.path().to_path_buf();
        let untouched = Channel::allocate(dir.path()).unwrap();

        signalled.discard().unwrap();
        untouched.discard().unwrap();

        assert!(!signalled_path.exists());
    }
````
<!-- /fragment -->

`signalled` has a file, written through `signal`; `untouched` never had one.
Both are discarded, and both must succeed. Only one of the two halves is an
`assert!`, and the other is the `.unwrap()` on `untouched.discard()`: a `discard`
that returned `Err` for a path with no file would panic there and the test would
fail with the error's own message. It is a real assertion in a form a reader
scanning for `assert!` lines will not count, and the module leans on that form
more than once: `signalled.discard()` above it carries the other half the same
way, the cleanup call in the next section carries *a clean pass returns `Ok`*,
and the two tests that expect a refusal put the claim *this call fails* in an
`unwrap_err` with the macros beside it checking only what the message says.
Counting `assert!` lines is not a way to count what this module holds.

The `to_path_buf()` on the line before the calls is a consequence of the
signature. `discard` takes `self`, so after the call the channel is gone and
cannot be asked for its path; the test copies the path first. A test written
around a signature is evidence about the signature — here, that consuming the
channel is what stops anything reading a path whose file has just been removed.

What the test does not reach is `remove_if_present`'s error arm. Both halves take
the `Ok(())` and `NotFound` branches, and the refusal ending in *remove it by
hand* is named by no test in the crate.

<a id="the-decoys"></a>
## Four decoys, each wrong in a different way

The last two tests are `Channel::discard_abandoned`, which is the only caller of
`is_channel_name` in production code. The first builds a directory the grammar
has to sort correctly; the second takes away the directory.

<!-- fragment «channel-tests-cleanup» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="415-453" parent="channel-inline-tests" -->
````rust

    #[test]
    fn abandoned_cleanup_removes_channels_and_leaves_every_other_entry_alone() {
        let dir = tempfile::tempdir().unwrap();
        let channel = dir.path().join("signal-0123456789abcdef0123456789abcdef");
        // Each of these fails the grammar in a different way, and each is a
        // file a consumer could legitimately keep in the same directory.
        // A distinct nonce, not the same one in another case: on a
        // case-insensitive filesystem the two names would be one file, and the
        // test would be asserting nothing.
        let uppercase = dir.path().join("signal-FEDCBA9876543210FEDCBA9876543210");
        let short = dir.path().join("signal-0123456789abcdef");
        let unprefixed = dir.path().join("0123456789abcdef0123456789abcdef");
        let neighbour = dir.path().join("driver.lease");
        for path in [&channel, &uppercase, &short, &unprefixed, &neighbour] {
            std::fs::write(path, "x").unwrap();
        }

        Channel::discard_abandoned(dir.path()).unwrap();

        assert!(!channel.exists(), "an exact channel name must be removed");
        for path in [&uppercase, &short, &unprefixed, &neighbour] {
            assert!(path.exists(), "{} must survive cleanup", path.display());
        }
    }

    #[test]
    fn abandoned_cleanup_names_the_directory_when_it_cannot_be_listed() {
        let dir = tempfile::tempdir().unwrap();
        let absent = dir.path().join("never-created");

        let error = Channel::discard_abandoned(&absent).unwrap_err();

        assert!(
            error.to_string().contains(&absent.display().to_string()),
            "{error}"
        );
    }
}
````
<!-- /fragment -->

The first is the module's largest test and the one that runs the grammar through
its caller. Five files go into one directory: one name the grammar accepts and
four it must reject. The table is the fixture read against
[chapter 2's three conditions](02-the-channel.md#exactly-this-name); take from it
that each decoy is a file a consumer could legitimately keep, and that the three
conditions refuse one at a time — the prefix doing it twice, once for a name
whose suffix would otherwise pass and once for a file that is not a channel name
in any respect.

| The file the fixture writes | Where it stops being a channel name | After the cleanup |
|---|---|---|
| `signal-0123456789abcdef0123456789abcdef` | nowhere — prefix, thirty-two characters, lowercase hex | removed |
| `signal-FEDCBA9876543210FEDCBA9876543210` | the alphabet: `hex` emits lowercase and the predicate accepts nothing else | kept |
| `signal-0123456789abcdef` | the length: sixteen characters where `NONCE_BYTES * 2` requires thirty-two | kept |
| `0123456789abcdef0123456789abcdef` | the prefix, with a suffix that would otherwise pass | kept |
| `driver.lease` | the prefix, and it is the file chapter 2 named as the cost of a loose rule | kept |

One assertion covers the removal, and a loop asserts each of the four survivors
in turn, every failure message naming the path that should have been left alone.

The two-sided shape is what makes it a check rather than an observation. A
cleanup that removed everything fails on the first survivor; a cleanup that
removed nothing fails on the first assertion. No fixture that merely reported
what it was told could produce both results.

The comment inside it is about the fixture rather than about the code, which is
why it sits in the test and not on `is_channel_name`: the uppercase decoy uses a
*different* nonce, because on a case-insensitive filesystem the same nonce in two
cases would be one file and the test would be asserting nothing. Chapter 2 read
that argument at the function it protects; what this page adds is that the
decision it records is a decision about how to write a test, and that it belongs
to the test's author rather than to the grammar's.

The second test covers the one refusal `discard_abandoned` reaches before it
removes anything: `read_dir` fails on a directory that is not there, and the
error names it. The aggregate refusal at the other end — *could not remove N
abandoned completion channel(s) … remove them by hand* — needs a directory that
lists and an entry that will not delete, and is named by no test. Line 453 is the
module's closing brace and the last byte of `src/channel.rs`.

The module holds a filename to a grammar of prefix, length and alphabet, and to
nothing about what the name refers to. There is no point in it at which the
crate could have learned what a value means.

Chapter 6 owns no source. Chapter 7 reads the last root: a child with no
terminal, and the same child under a filesystem policy.

[Previous: The watch and the escalation](04-the-escalation.md) | [Contents](README.md) | [Next: What passes through](06-what-passes-through.md)
