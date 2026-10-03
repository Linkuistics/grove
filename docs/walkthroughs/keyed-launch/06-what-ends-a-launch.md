# What ends a launch
<!-- book-page id="what-ends-a-launch" slice="assembly" order="6" -->
[Previous: How this is checked](05-how-checked.md) | [Contents](README.md) | [Next: Confined noninteractive jobs](07-confined-jobs.md)

<a id="assembly"></a>
## One question

<!-- rollup «owned-lines-total» -->
This chapter owns no production source. The book reconstructs 7 roots and
2,226 lines, including the confinement chapter that follows this assembly,
and the [source index](source-index.md) records that graph in full. What is left
is the thing no single chapter could state, because each one opened on its own
refusal and stopped at the boundary of the lines it owned.

Chapters 1 to 5 each opened on what its stage must not add and must not
interpret. Read one at a time, each of those is a local argument about a
few dozen lines — a manifest with no domain dependency, a path nothing has
written to, an environment with one variable added. Read together, they answer
the question this chapter applies:

> **What ends a launch, and who decides?**

The crate decides nothing. It does not know what the program does or what the
token says, so it has nothing to judge the work by, and a layer in that position
can only observe. It observes three things, and they are the only three ways a
launch ends: the child exits, the file the child was handed a path to appears,
or the launcher itself is signalled. The table is what each chapter's refusal
left for that answer.

| Ch | What the stage carries | What it must not add or interpret | What that leaves the ending |
|---:|---|---|---|
| 1 | the manifest, the library root, the error type and the argv | what the program does, and what the token says | nothing here can judge whether the work is done |
| 2 | the completion channel | the ending, and what the token says | a file the launcher never wrote, so its appearance is evidence that the child spoke |
| 3 | the environment, the terminal, the spawn | anything the caller did not write | a job that can be ended whole, handed this launch's path |
| 4 | the watch, the escalation, the launcher's signals and the end of the group | the ending: whether the child is done | three observables, an ending the launcher performs, and a group confirmed gone |
| 5 | the channel's inline tests | what a name refers to | the properties appearance rests on, held by tests that spawn nothing |

One ending reaches none of the three, and it has a section of its own below.

<a id="three-endings"></a>
## Three endings, and what breaks without each

Take each observable away in turn. What breaks is the reason it is there, and
the test named beside it is the one that holds it. The launch is the
interactive one chapters 1 to 5 cover. Chapter 7's mode changes what the
launcher sends, and not the three things it observes.

### The child exits

**The observable.** `exited` reports the child's exit, never a stop, before
anything else has ended the child, and leaves it unreaped until its group is
killed. The launch comes back `End::Exited`, with a token if the child wrote one and without if it did not.

**Without it.** A launcher that waited only to be told would wait for ever on a
child that crashed, or one that exited before it spoke.

**What the crate does not do with it** is read the status. An exit says the
child is gone and says nothing about its work:
[the token, never the exit status, says what the launch meant](04-the-escalation.md#the-poll).
`a_child_that_never_signals_ends_with_no_token` pins it in four assertions. A
child that exits 3 without writing anything comes back with `token` of `None`,
`end` of `End::Exited`, status code 3, and no file on the channel path. A
launcher that inferred completion from an exit, or made an empty token out of
an absent file, fails there.

### The file appears

**The observable.** `channel.appeared()`. Appearance starts the grace. If
the child is still running when the grace runs out, the launcher ends it and
the launch comes back `End::Escalated`.

**Without it.**
[An interactive child returns to its prompt when it finishes](02-the-channel.md#appearance-is-the-event)
and does not exit. A launcher with only the first observable waits for as long
as the child sits there. Or it guesses: a timeout, a quiet period, a pattern in
the output. Each guess is a conclusion about work the crate cannot see, and a
launcher that decides for itself that a child is done is sometimes wrong while
looking exactly as if it were right.

**What makes it an observation and not a guess** is work chapters 2 and 3 did.
`Channel::allocate` [writes nothing](02-the-channel.md#writes-nothing), so a
file at that path was put there by somebody other than the launcher. A caller
that allocates a channel for each launch gets a fresh path each time, so the
file belongs to this launch, which
`successive_launches_get_independent_channels` holds. And
[the scrub list](03-the-job.md#everything-one-launch-is) is where a caller
removes the launch-control variables its own environment carried, so a nested
child is not handed a live path belonging to somebody else's launch. A leaked
path is authority to end that launch.
`an_allocated_channel_names_a_path_that_does_not_yet_exist` holds the first, and
`a_scrubbed_variable_is_removed_from_an_inherited_environment` holds that a
scrub removes what was inherited.

What the child wrote is read back once, after the child is gone, and handed to
the caller as an opaque `Token`. The crate never compares the bytes with
anything. `read`
[refuses what is not a token](02-the-channel.md#three-ways-to-have-no-token) —
an empty file, one that is too large, a path that is not a regular file — and
trims trailing framing from the rest, which is a test of shape and not an
interpretation. `relaunch` is grove's word, and this book has carried it since
chapter 1 without the crate ever having read it *as* a word. The string does
occur inside the corpus, where the inline module in `src/channel.rs` writes it
to a channel it then discards. Even the crate's own test uses the word as a
value to move, and would pass just as well with any other.

### The launcher is signalled

**The observable.** `take_interrupt()` returns the number of a SIGTERM or SIGHUP,
or a SIGINT for a launch with no terminal, that the launcher's own handler
latched. The launcher sends the job the same
signal, reaps the child, and the launch comes back
`End::Interrupted { signal }`.

**Without it.** The signal's default disposition ends the launcher where it
stands, and its child is left on the terminal with no parent to end it. A
launcher that caught the signal, tidied up and exited 0 would do no better: it
would tell its own parent that the work finished.
[An exit code cannot say *was signalled*](04-the-escalation.md#dying-of-it), so
the number is carried and `reraise` lets the launcher die of the signal it was
sent.

The channel cannot express this ending, because an interrupt normally leaves no
token. `an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other`
in `tests/interrupt.rs` holds the report, and
`a_reraised_signal_reaches_the_parent_as_a_wait_status` in `tests/reraise.rs`
holds the death.

### Seeing an ending is not performing one

Two of the three are followed by an act, and the act is the launcher's because
[only the child's parent can perform it](03-the-job.md#the-two-waits). A child
at its prompt will not exit, and a child asked to end itself from inside a
sandbox may be denied silently. So the launcher signals: the grace, SIGTERM, the
kill grace, SIGKILL.

The signal goes to [the whole process group and then the child](04-the-escalation.md#the-whole-group),
which is why chapter 3 spawned the child as a job. A grandchild outside that
reach survives its parent, can hold a lock the launcher's caller is about to
wait on, and turns a reported ending into a stall.
`the_escalation_reaps_the_childs_descendants` holds it, with a bystander in
another group that must be left alone.

The group is ended after **every** ending, not only an escalated one, because a
child that exits on its own can leave a member running as surely as one that
was killed. [The end of the launch](04-the-escalation.md#the-group-ends) kills
what remains of the group while the exited child still reserves its ID, reaps
the child, and asks the system whether the group is gone. The answer comes back
beside the child's status as `Group`, so a caller never relaunches or publishes
beside a survivor. `a_term_ignoring_descendant_is_gone_before_the_launch_returns`
holds it across all three ways a child can leave a TERM-ignoring descendant
behind.

What comes back keeps the two facts apart.
[`End` says who acted and `token` says whether the child spoke](03-the-job.md#which-of-three-happened),
and neither is inferred from the other.
`a_child_that_signals_and_exits_inside_the_grace_is_never_touched` is the case
that separates them: a token came back, and the launch still ended `Exited`,
because nothing had to end it.

The three, collected. The third column is what the reader looks for in a layer
of their own.

| The ending | Where it is observed | Without it | Held by |
|---|---|---|---|
| the child exits | `child.exited` | a child that dies without speaking is waited on for ever | `a_child_that_never_signals_ends_with_no_token`, `an_unsignalled_child_runs_to_its_own_exit_untouched` |
| the file appears | `channel.appeared()` | an interactive child never ends, or the launcher guesses that it has | `a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace`, `a_child_that_signals_and_exits_inside_the_grace_is_never_touched` |
| the launcher is signalled | `take_interrupt()` | the launcher dies and leaves its child on the terminal, or exits 0 and reports work that was cut short | `an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other`, `a_reraised_signal_reaches_the_parent_as_a_wait_status` |

<a id="the-one-that-stays-open"></a>
## The ending no observable reaches

The three observables leave a gap.
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
layer that does not know what a session is cannot make one.

The general form applies to the reader's own layer: **a layer that learns nothing
cannot notice that nothing happened.** The cost of preserving that independence
is that failures detectable only through domain meaning must be detected from
outside. The response is to name the case, put the obligation where the meaning
already lives, and refuse to let the layer guess. Chapter 4 does the first,
grove's instruction does the second, and the three observables do the third.

<a id="taking-the-test-away"></a>
## Taking the test to a layer of your own

The transferable result is the question. It applies to any layer that starts
work it does not understand: a launcher, a job runner, a supervisor, a consumer
handing a message to a handler. Four steps.

1. **Name what your layer starts and what it can see of it.** Not *"the job"* —
   *"a process I am the parent of, a path I chose, and my own signals"*. What a
   layer can see bounds what it can know. Anything else it reports about the
   work is a conclusion.
2. **List every way the work ends, and mark each one observed or inferred.** An
   exit is observed. *Finished* is not, unless the work says so. A timeout, a
   quiet period and a status the layer liked the look of are inferences. Where
   the work has to say it is done, check that only the work can say it:
   `Channel::allocate` writing nothing is what makes a file's appearance a fact
   about this launch, and the scrub is what keeps a nested child from holding
   somebody else's path.
3. **Take each ending away and say what breaks.** If nothing breaks, it was not
   an ending. If what breaks is a stall, an orphan or a false report, that is why
   it is there, and it names the test that should hold it. The table above is
   this step run over `keyed-launch`.
4. **Find the ending that reaches none of them, and say whose it is.** A layer
   that does not know the work cannot add an observable for it. The obligation
   goes to the layer that knows what was asked. Then ask who performs each
   ending: if the work cannot end itself, the layer that started it must, and
   must reach everything it started.

The last question the test will not answer is whether the property justifies its
cost. That is a judgement about what the layer is for. A crate that does not
know what it launches can serve a caller with a different domain unchanged. The
corpus does name grove, in three kinds of place: the comment over the release
block, which is about how the package ships; `src/channel.rs`'s note on whose
driver leaves files in a control directory; and a shell line in a comment in
`src/run.rs`. Packaging, provenance and illustration. The crate never branches
on who its caller is.

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
src/run.rs       1,292 lines, 4 blocks, chapters 3 4
      1–168   ch 3   ┐  the launch’s shape
    169–357   ch 4   │  the watch, the latch and the entry signal state,
                     │  inside chapter 3’s pair
    358–957   ch 3   ┘  terminal, its lease, detached mode and spawn
   958–1292   ch 4      the end of the group, supervise and escalate

src/channel.rs     505 lines, 2 blocks, chapters 2 5
      1–319   ch 2      the production code
    320–505   ch 5      the inline #[cfg(test)] module
```

`src/run.rs` divides
by whose signal a line is about — which is why chapter 4's watch and latch sit
between chapter 3's launch shape and its spawn. `src/channel.rs`'s single
boundary is the `#[cfg(test)]` attribute on line 320, the only ownership boundary
in the book cut at a compilation condition rather than a conceptual one.

<!-- rollup «early-use-rows» -->
<!-- rollup «early-use-rows-declared» -->
<!-- rollup «early-use-rows-at» of="01-orientation.md#the-cast" -->
**Early use.** 5 rows, every one `explained`. 5 of them are the manifest's
`[[early-use]]` entries, which are the rows the book may not omit: 4 forced
by [chapter 1's cast](01-orientation.md#the-cast) naming the public surface
before its owners explain it, and one by
[chapter 3](03-the-job.md#the-spawn) reaching the handler, the latch, the
SIGCHLD repair, the launch mode and the supervisor that are `run`'s first and
last acts and chapter 4's to explain.
Each row turned `explained` in its owner's slice and in no other.

<!-- rollup «owned-lines-sequence» -->
<!-- rollup «source-owning-chapters» -->
**Owned source.** 212 + 319 + 768 + 524 + 186 + 217 = 2,226
lines across 6 source-owning chapters, and 0 for this one. A chapter that owns
no source is the shape the structure brief chose for the assembly, and the total
is the 2,226 lines in the current declared corpus.

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
valid: 7 files, 2226 resolved lines, 0 deferred lines, final=true
```

`--final` is what makes this different from every scoped run the drafting
sessions made. In scoped mode a later chapter's range may be reserved by a defer
and counted as deferred rather than resolved; in final mode a defer is an error,
every source root must expand to its complete file, and the page inventory must
match the manifest exactly. 2,226 resolved and 0 deferred is the whole corpus
reconstructed — including `src/channel.rs` lines 320 to 505, the inline
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
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/confinement.rs
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/interrupt.rs
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/job.rs
test result: ok. 9 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out
     Running tests/launch.rs
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/noninteractive.rs
test result: ok. 7 passed; 0 failed; 7 ignored; 0 measured; 0 filtered out
     Running tests/reraise.rs
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   Doc-tests keyed_launch
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Fifty-eight tests, and the split across the two surfaces is itself a fact this
book explained. Twelve of the fifteen unit tests are the inline
`#[cfg(test)] mod tests` inside `src/channel.rs`, which are corpus and are on
[chapter 5's page](05-how-checked.md#inside-the-root) like any other lines. The
*module* exists because `use super::*;` reaches seven items no integration test
can see, and one of the twelve tests takes that up: only a module inside the file
can call `is_channel_name`, whose exactness chapter 2 argued. The other three
are in `tests/internal/wait_events.rs`, which `src/run.rs` includes by path so
that they can reach the supervisor's private seam. The other forty-three live
in six files under `crates/keyed-launch/tests/` and are evidence rather than
corpus: named wherever a chapter adjudicates a claim, reproduced nowhere. The
eleven ignored tests, seven in `tests/noninteractive.rs` and four in
`tests/job.rs`, are the roles those files re-execute themselves as.

<!-- rollup «source-roots» -->
<!-- rollup «owned-lines-total» -->
<!-- rollup «chapters» -->
The book reconstructs 7 roots, 2,226 lines, 7 chapters, two lookup surfaces,
zero deferred ranges. What it leaves the reader with is the question — *what ends
a launch, and who decides?* — together with the one ending no observable
reaches, and the reason that ending belongs to whoever knows what the child was
asked to do.

[Previous: How this is checked](05-how-checked.md) | [Contents](README.md) | [Next: Confined noninteractive jobs](07-confined-jobs.md)
