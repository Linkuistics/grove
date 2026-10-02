# What passes through
<!-- book-page id="what-passes-through" slice="assembly" order="6" -->
[Previous: How this is checked](05-how-checked.md) | [Contents](README.md) | [Next: Confined noninteractive jobs](07-confined-jobs.md)

<a id="assembly"></a>
## One question

<!-- rollup «owned-lines-total» -->
This chapter owns no production source. The book reconstructs 7 roots and
1,688 lines, including the confinement chapter that follows this assembly,
and the [source index](source-index.md) records that graph in full. What is left
is the thing no single chapter could state, because each one opened on its own
refusal and stopped at the boundary of the lines it owned.

Every chapter opened the same way: *what this stage must not add and must not
interpret is X*. Read one at a time, each of those is a local argument about a
few dozen lines — a manifest with no domain dependency, a path nothing has
written to, an environment with one variable added. Read
together, they are answers to the question that this chapter applies:

> **Where does this layer learn what the value means?**

The answer this crate gives is *nowhere*, and that answer must be checked rather
than assumed. A layer that carries a value to an effect in the world has three
chances to learn a meaning it was never given: on the way in, by composing a value from several sources; on the way
through, by reading a value a second time in another grammar; and on the way
out, by inferring what came back or adding what was not written. This crate
used to resolve a key to a command template, and the first two were closed
there. That half is deleted. A command now arrives as an `Argv` its caller
built, so the crate composes nothing and reads no word twice, and what is left
to check is the way out. The table is where each chapter stands.

| Ch | What the stage carries | The meaning available to be learned | Where it would enter |
|---:|---|---|---|
| 1 | the manifest, the library root, the error type and the argv | what the program does | the claim, and a type that holds it |
| 2 | the completion channel | what the token says | on the way out |
| 3 | the environment, the terminal, the spawn | what the child needs that the caller did not write | on the way out |
| 4 | the watch, the escalation and the launcher's signals | whether the child is done | on the way out |
| 5 | the channel's inline tests | what *correct* means | from outside |

[Chapter 1](01-orientation.md#understands-neither) states the claim: the
manifest buys one dependency and no domain, the library root says every word of
the command is the caller's, and `Argv` holds whatever was put into it.
[Chapter 5](05-how-checked.md#checked-without-meaning) is the other end — it
holds a filename to a grammar, and nothing it checks is about what a name is
for. The three rows between them are where the answer is earned.

One case on the way out has no answer at this layer at all, and this page ends
on it.

<a id="where-does-it-learn"></a>
## Where does it learn what the value means?

Take any layer in your own code that carries a value to an effect in the world
— a command to a process, a query to an engine, a route to a handler — and ask
where it learns what the value means. What follows is the one part of the test
this crate still answers, against the chapters that proved it, with the carried
example — one of grove's launches and its channel variable — as the material.

### 3 · On the way out — inferring what came back, or adding what was not written

**The move.** The layer reaches a conclusion the value did not carry. It decides
from a status, a timeout or a silence that the work is done; or it adds to the
call something the operator did not write, because the addition looks like
helpfulness — a flag because the child is non-interactive, a variable because the
child seemed to want one, a directory because none was given.

**The cost.** A launcher that decides for itself that a child is done will
sometimes be wrong while looking exactly as if it were right; and a value the
caller cannot see in the argv it built is a value it cannot change by building a
different one.

**What the crate does instead**, in the *adding* direction, is
[chapter 3](03-the-job.md#nothing-else-added): `run` hands the child one
environment variable holding one path, and that is the whole of what it adds. The
argv is the one the caller built, the working directory is the caller's, and
the scrub list removes launch-control variables a nested launcher must not
inherit rather than installing any.
`a_scrubbed_variable_is_removed_from_an_inherited_environment` is the test, and
its shape is the claim: it scrubs `HOME` from a child that inherits an ordinary
environment, then asserts through the channel that `HOME` came back `<unset>`
while `PATH` came back equal to the launcher's own. A scrub that merely emptied a
variable, or an inheritance that quietly rebuilt one, fails a different assertion
in the same test. `a_caller_built_argv_is_spawned_whole_and_directly` holds the
other direction over a whole trip: five words a shell would re-read go into an
`Argv`, and the child reports the same five back.

The compile-time half of the same answer is chapter 1's, and it is the one place
in the book where a refusal is held by a type instead of by a rule. `Argv` has
private fields and no method that changes one, so
[what was put into an `Argv` is what `run` spawns](01-orientation.md#the-seam-type):
`run` takes an `Argv` and nothing else that could name a program. The *adding*
direction is closed before the program is written rather than checked after it
runs. The type says nothing about who authored the words. The caller that built
the `Argv` answers for them.

In the *inferring* direction the answer is [chapter 2](02-the-channel.md#the-thesis).
The crate does not decide that a child is finished: it allocates a path, hands
that path to the child, and waits for the path to exist. `Channel::allocate`
writes nothing, which is why the file's later existence is unambiguous evidence
that something else wrote it, and why appearance alone can be the event. What the
child wrote is read back once and handed to the caller as an opaque `Token`. The
crate's only interest in the bytes is
[whether there are any](02-the-channel.md#three-ways-to-have-no-token): `read`
trims trailing framing and reports nothing for an empty file, which is a presence
test and not a semantic interpretation. `relaunch` is grove's word, and this book
has carried it through every chapter without the crate ever having read it *as*
a word. The
string does occur once inside the corpus — `src/channel.rs` line 357, where the
inline module writes it to a channel and reads it back — and that occurrence is
the point rather than an exception to it: even the crate's own test uses the word
as a value to move, and would pass just as well with any other.

[Chapter 4](04-the-escalation.md#three-observables) is where the restriction
becomes a state machine. Supervision polls three things, and they are the only
three ways a launch ends: the child exits, the token's file appears, or the
launcher itself is signalled. Nothing else is counted as an ending — not a
timeout, not a quiet period, not an exit status the crate liked the look of.
`a_child_that_never_signals_ends_with_no_token` is the test that pins the
negative case, and it pins it in four assertions: a child that exits 3 without
writing anything comes back with `token` of `None`, `end` of `End::Exited`,
status code 3, and no file on the channel path. A launcher that inferred
completion from a clean exit, or that manufactured an empty token from an absent
file, fails there.

The arm, collected. The second column is the move to look for, the third is
what it costs when you find it, and the fourth is one shape that closes it —
which the next section qualifies.

| The arm | The move | The cost | What this crate does instead |
|---|---|---|---|
| on the way out | a conclusion the value did not carry — that the work is done, or that the launch wants something the caller did not write | a launcher that decides a child is done is sometimes wrong while looking exactly right, and a value not in the argv cannot be changed by building a different one | one variable holding one path is the whole of what is added and nothing can change an `Argv` once it is built; the appearance of a file the crate never wrote is the only completion event ([chapter 2](02-the-channel.md#the-thesis), [chapter 3](03-the-job.md#nothing-else-added), [chapter 4](04-the-escalation.md#three-observables)) |

<a id="the-one-that-stays-open"></a>
## The one the crate cannot close

The way out has a limitation.
[Chapter 4 states it in its second paragraph](04-the-escalation.md#the-launchers-job),
and [chapter 3 reproduces it inside `run`'s own doc comment](03-the-job.md#the-child-is-a-job),
where it is part of the function's stated contract and, as that section says
itself, a paragraph that belongs to chapter 4:
**an interactive child that finishes its turn and never signals reaches none of
the three observables.** It returns to its prompt rather than exiting, so the
launch does not end. It stalls.

The reason it cannot be fixed here is the reason the rest of the page has been
arguing for. Nothing this crate can observe distinguishes a child that forgot to
signal from one still working. Both are the same process, alive, holding the same
terminal, at the same point in the launcher's poll; the three things supervision
looks at are identical in the two cases. Anything that would tell them apart is a
reading of what the child is *doing* — which is a meaning, and the crate has
none. A second completion observable does not solve it either; it trades a stall
for a wrong kill, and a wrong kill is the more expensive error, because the stall
is visible to the operator sitting at the terminal and the kill destroys work
that was in progress.

So it is the caller's to close, at the layer that instructs the child. That layer
is the one that knows what it asked for, and it has instruments this one does
not: it wrote the mandate, so it can require the child to signal as part of the
instruction, and it can decide what an unsignalled ending means for the work it
had in hand. grove closes it exactly there — the instruction its sessions carry
is what makes signalling the child's last act, and the
[user guide's account of the session lifecycle](../../USAGE.md#usage-session-lifecycle)
is where that obligation is written down for an operator. None of that is in this
crate. Keeping the obligation outside preserves the crate's domain independence:
an obligation on the child is a statement about what the child is for, and a
layer that has never heard of sessions cannot make one.

The general form applies to the reader's own layer: **a layer that learns nothing
cannot notice that nothing happened.** The cost of preserving that independence
is that failures detectable only through domain meaning must be detected from
outside. The response is to name the case, put the obligation where the meaning
already lives, and refuse to let the layer guess. Chapter 4 does the first,
grove's instruction does the second, and the three observables do the third.

<a id="taking-the-test-away"></a>
## Taking the test to a layer of your own

The transferable result is the question, which applies to any layer that sits
between a value somebody wrote and
something that happens. Four steps, and the third is where the answer usually
turns out to be *somewhere after all*.

1. **Name the value and the two ends of its trip.** Not *"the command"* — *"the
   words in this `Argv` become the argv of a process"*. A trip you cannot state as a
   start and an end has no *through* for anything to be learned in.
2. **Find every point where the value is read, and count the grammars.** One
   reading is a parse. Two readings in different grammars is where this arm's
   failures live, and the second reading is often not in your code: a splitter, a
   templating layer, a shell invoked for convenience, a serialiser that
   round-trips. This crate has one reading and no grammar: `Argv` holds words
   and `run` spawns them.
3. **Ask what your layer would have to know for each of its own rules to be
   correct.** This is the one that finds the meaning. A merge rule has to know
   which fields are lists. A retry has to know which operations are idempotent. A
   default has to know what the absence of a value meant. Each of those is a fact
   about the caller's domain living inside a layer that will not be told when the
   domain changes — and none of them looks like a domain model at the point where
   it is written.
4. **Ask what ends the trip, and who decides.** If the answer is *the layer
   decides* — from a status, a timeout, a silence — you have this book's
   arm, and the question is whether the thing being decided is a fact the layer
   can observe or a conclusion it is drawing. `Channel::allocate` writing nothing
   is what turns the observation into a fact; the stall above is what is left when
   even that is not enough.

Run those four over `keyed-launch` and you get the table this page opened with.
**The one answer above that a type holds cannot decay**, which the others can:
a rule is a line somebody can edit, but a second way to hand `run` a program
would have to be a new field on `Launch`, and that fails every caller's build.
Where a property can be moved from a rule to a type, a breach becomes a compile
failure rather than a condition every caller must remember to check.

The last question the test will not answer is whether the property justifies its
cost. That is a judgement about what the layer is for. A crate that does not
know what it launches can serve a caller with a different domain unchanged. The
corpus does name grove, in three kinds of place: the comment over the release
block, which is about how the package ships; `src/channel.rs`'s note on whose
driver leaves files in a control directory; and a shell line in a comment in
`src/run.rs`. Packaging, provenance and illustration — never a branch, a
constant or a name the crate acts on.

<a id="the-closed-ledgers"></a>
## The closed ledgers

The book's two ledgers are complete, and the closure is mechanical rather than a
claim this chapter makes.

<!-- rollup «ownership-blocks» -->
<!-- rollup «source-roots» -->
<!-- rollup «ownership-blocks-owned-by» of="understands-neither" -->
<!-- rollup «ownership-blocks-not-owned-by» of="understands-neither" -->
**Ownership.** 11 top-level blocks over 7 source roots, every one
`resolved`. The table is the source index's
[ownership blocks](source-index.md#ownership-blocks), and chapter 1's slice
owns 4 blocks, with 7 owned by the other chapters. No `defer` directive remains anywhere in the
book, and none may: `F003` reports any defer at all in final mode, so *every
deferral has become an insertion* is a statement the validator refuses to let be
false rather than one this page asserts.

<!-- rollup «ownership-blocks» -->
<!-- rollup «source-roots» -->
11 blocks over 7 roots is the price of reading the crate
in its own conceptual order. Five roots are owned whole by one chapter; the other
two split, and each split is the concept order disagreeing with the file's. In
`src/run.rs`, one chapter's block sits *inside* another chapter's pair.

```text
src/run.rs         852 lines, 4 blocks, chapters 3 4
      1–124   ch 3   ┐  the launch’s shape
    125–244   ch 4   │  the watch and the latch, inside chapter 3’s pair
    245–678   ch 3   ┘  terminal, detached mode and spawn
    679–852   ch 4      supervise and escalate

src/channel.rs     453 lines, 2 blocks, chapters 2 5
      1–288   ch 2      the production code
    289–453   ch 5      the inline #[cfg(test)] module
```

`src/run.rs` divides
by whose signal a line is about — which is why chapter 4's watch and latch sit
between chapter 3's launch shape and its spawn. `src/channel.rs`'s single
boundary is the `#[cfg(test)]` attribute on line 289, the only ownership boundary
in the book cut at a compilation condition rather than a conceptual one.

<!-- rollup «early-use-rows» -->
<!-- rollup «early-use-rows-declared» -->
<!-- rollup «early-use-rows-at» of="01-orientation.md#the-cast" -->
**Early use.** 5 rows, every one `explained`. 5 of them are the manifest's
`[[early-use]]` entries, which are the rows the book may not omit: 4 forced
by [chapter 1's cast](01-orientation.md#the-cast) naming the public surface
before its owners explain it, and one by
[chapter 3](03-the-job.md#the-spawn) reaching the handler, the latch and the
supervisor that are `run`'s first and last acts and chapter 4's to explain.
Each row turned `explained` in its owner's slice and in no other.

<!-- rollup «owned-lines-sequence» -->
<!-- rollup «source-owning-chapters» -->
**Owned source.** 166 + 288 + 558 + 294 + 165 + 217 = 1,688
lines across 6 source-owning chapters, and 0 for this one. A chapter that owns
no source is the shape the structure brief chose for the assembly, and the total
is the 1,688 lines in the current declared corpus.

<!-- rollup «source-roots» -->
The [concept index](concept-index.md) and the [source index](source-index.md) are
the two lookup surfaces, and neither is part of the reading order. The source
index is the authoritative record of how the fragment graph reconstructs each of
the 7 files; the concept index is curated navigation into the arguments, and
makes no completeness claim.

<a id="final-verification"></a>
## Final verification

Three commands prove the book, and they prove different things. The first is the
only one that reads the corpus byte for byte.

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/keyed-launch --final --check all
valid: 7 files, 1688 resolved lines, 0 deferred lines, final=true
```

`--final` is what makes this different from every scoped run the drafting
sessions made. In scoped mode a later chapter's range may be reserved by a defer
and counted as deferred rather than resolved; in final mode a defer is an error,
every source root must expand to its complete file, and the page inventory must
match the manifest exactly. 1,688 resolved and 0 deferred is the whole corpus
reconstructed — including `src/channel.rs` lines 289 to 453, the inline
`#[cfg(test)] mod tests` that is corpus because a root is `src/**/*.rs` and the
specification's exception inventory carries no row for this book.

```console
$ bash scripts/check.sh
...
  6 book(s) checked, 0 failing
  ✓ book-check

check: all 12 principal checks pass
```

The umbrella. It runs `book-check --final --check all` over every book root under
`docs/walkthroughs/` **by discovery** rather than from a list, which is why this
book has been included in the gate since chapter 1 created its directory, and why
every drafting session but this one left the script red on `book-check` alone: a
scoped prefix deliberately leaves later blocks deferred, and the gate only ever
runs `--final`. The script also runs the repository's own tests, and two of those
cover this book without naming it — `every_repository_markdown_reference_resolves`
sweeps every Markdown file in the repository, and
`every_book_root_has_a_documentation_ownership_row` fails a book root with no row
in the *Documentation ownership* table of `docs/ARCHITECTURE.md`. That row was the
one obligation this book owed outside its own directory, and chapter 1's session
added it with the rest of the scaffolding.

```console
$ cargo test --locked -p keyed-launch
     Running unittests src/lib.rs
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/confinement.rs
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/interrupt.rs
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/launch.rs
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/noninteractive.rs
test result: ok. 5 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out
     Running tests/reraise.rs
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   Doc-tests keyed_launch
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Thirty-nine tests, and the split across the two surfaces is itself a fact this
book explained. Eleven of the twelve unit tests are the inline
`#[cfg(test)] mod tests` inside `src/channel.rs`, which are corpus and are on
[chapter 5's page](05-how-checked.md#inside-the-root) like any other lines. The
*module* exists because `use super::*;` reaches seven items no integration test
can see, and one of the eleven tests takes that up: only a module inside the file
can call `is_channel_name`, whose exactness chapter 2 argued. The twelfth is in
`tests/internal/wait_events.rs`, which `src/run.rs` includes by path so that it
can reach the supervisor's private seam. The other twenty-seven live in five
files under `crates/keyed-launch/tests/` and are evidence rather than corpus:
named wherever a chapter adjudicates a claim, reproduced nowhere. The six
ignored tests in `tests/noninteractive.rs` are the child roles that file
re-executes itself as.

<!-- rollup «source-roots» -->
<!-- rollup «owned-lines-total» -->
<!-- rollup «chapters» -->
The book reconstructs 7 roots, 1,688 lines, 7 chapters, two lookup surfaces,
zero deferred ranges. What it leaves the reader with is the question — *where
does this layer learn what the value means?* — together with the one case where
this crate's own answer runs out, and the reason that case belongs to whoever
knows what the child was asked to do.

[Previous: How this is checked](05-how-checked.md) | [Contents](README.md) | [Next: Confined noninteractive jobs](07-confined-jobs.md)
