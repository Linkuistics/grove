# What jj owns
<!-- book-page id="what-jj-owns" slice="assembly" order="7" -->
[Previous: Refusal](06-refusal.md) | [Contents](README.md)

<a id="assembly"></a>
## Six subtractions, assembled

This chapter owns no production source. The four roots and 752 lines are already
reconstructed by the fragment graph the six chapters before it built, and the
[source index](source-index.md) records that graph in full. What is left is the
thing no single chapter could state: the six refusals are one design, and the
design has a test.

Each chapter opened on something the crate declines to own and named who owns it
instead. Read one at a time, each of those is a local argument about a few dozen
lines. Read together, they are one claim — *jj already owns version control, so
this crate is the small residue jj does not own* — and the table below is that
claim in the only form that can be argued with. The third column is what the
first two are for: a subtraction that bought nothing is not modesty, it is a
hole.

| # | What was subtracted | Who owns it instead | What the subtraction bought |
|---:|---|---|---|
| 1 | Every dependency | `std` spawns a process, reads a directory, and defines `Error` | A crate a consumer takes without inheriting a build, and a refusal a consumer can put in `anyhow` without this crate having chosen `anyhow` |
| 2 | A repository abstraction, and any second lane behind it | jj, which *is* the version control system here | Resolution as a precondition rather than a dispatch: no trait, no enum, and no branch on which VCS owns the tree anywhere in 752 lines |
| 3 | Ambient repository selection | The child process's working directory, with jj's four Git selectors removed | One property proved once at one seam, instead of a hygiene checklist repeated at every call site |
| 4 | A vocabulary for its consumer | The consumer, which passes its namespace in as an ordinary string | One crate that serves any consumer, and a boundary the reader can watch `"grove"` cross rather than being told about |
| 5 | Transactions, and history added by a read | jj's snapshot and its operation log | 155 lines where six mechanisms would have been, and a change id that still names the work after a rewrite |
| 6 | Any remedy of its own | whoever the repair belongs to — jj in two of the eleven messages, and the filesystem, the environment or this crate's own vocabulary in seven more, and no one in the last two | Messages that are true whoever is calling, because none of them is a statement about one consumer's policy |

Two things about the table are worth saying before the test is applied to it.

**The rows are not independent, and three of them are one chain rather than three
choices.** Refusal 2 subtracts the abstraction, which is why there is nothing for
refusal 3's selectors to be configured *through*; and refusal 3's seam is what
makes refusal 5's commit reproducible, because a commit whose repository was
chosen by an inherited variable is not a commit anyone can reason about. Each of
the three is a separate decision, and the second and third were only available
because the first had been taken. The through-line runs `resolve` → the seam →
`commit`, and chapter 5 traced it. A table of six flattens that, which is what a
table is for and also what it costs.

**Refusal 4 is the odd row, and it is odd in the direction that matters.** Five
rows subtract something. The fourth *adds* — one directory, reserved inside
`.jj/` — and then declines to say what it is for. It is the only place in the
crate where a line of code exists because a consumer needed it, and it is the row
the test below has the most trouble with.

<a id="the-test"></a>
## The test: at least as strong, and checkable

Every row above is the same move: *we do not do X, because the thing underneath
already does X*. That move has a good version and a bad version, and they are
indistinguishable from the inside. The good version is a subtraction. The bad
version is an abdication — the work was not done, the layer below was named as
the reason, and nobody checked whether the layer below agreed.

The test that separates them has two clauses, and both are required:

> **A subtraction is justified when the guarantee the tool underneath already
> makes is at least as strong as the guarantee your consumer needs, and when that
> guarantee is checkable from where you stand.** If either clause fails, the same
> code is an abdication.

The clauses catch different failures, which is why neither is sufficient alone.

**The strength clause catches the guarantee that is nearly right.** The layer
below does something adjacent to what the consumer needs, the two are described
by the same word, and the difference only appears in the case nobody constructed.
"jj records the commit" and "the commit contains the paths I named" are both true
sentences about `jj commit`, and they are not the same guarantee; a caller who
needs the second and is given the first will not find out at the call site. The
failure is silent by construction, because the code that would have caught it is
the code that was subtracted.

**The checkability clause catches the delegation that is really a copy.** Here
the guarantee genuinely is strong enough, and the code does not actually ask for
it. It restates what the tool is believed to own — as a constant, a list, a
string, an assumption — and that restatement is a fork of the tool's behaviour
that no longer tracks it. A copy is correct on the day it is written and decays
without any commit touching it, so nothing in the change history marks the moment
it became wrong. This is the clause that costs a maintainer, and it is the one
that fails below.

Two clarifications, because the test is loose without them.

**Checkable means checkable *from where you stand*, not checkable in principle.**
Jujutsu's documentation is a check; so is a test that fails when the guarantee
moves; so is a value read back out of the tool at run time. A sentence in a
comment asserting what jj does is not, however accurate, because nothing goes red
when jj changes. The useful question is the negative one: **if the tool's
guarantee changed tomorrow, what here would go red?** If the honest answer is
*nothing*, the delegation is a copy no matter how correct it currently is.

**And the direction of the error is part of the answer.** A guarantee that is
wrong in the cheap direction and a guarantee that is wrong in the expensive one
score differently under the same clause, and the crate already reasons this way
in one place. Chapter 4's reserved-name list carries a four-line argument that
over-reserving costs a consumer a different word while under-reserving costs a
collision — the crate naming the direction of its own error before anyone asked
it to. Pricing the direction does not repair a failed clause. It does say what a
failure will cost, which is what decides whether it is worth fixing now.

<a id="applying-the-test"></a>
## The test applied to all six

Both clauses, run over every row of the assembly table. This is the verdict and
not the argument: each row is taken on its own below, in order, and the six
sections that follow are where the two clause columns are shown rather than
asserted.

| # | Guarantee the consumer needs | Where the guarantee comes from | Strong enough? | Checkable? | Verdict |
|---:|---|---|---|---|---|
| 1 | Start a child process with a controlled environment; read and create a directory | `std::process`, `std::fs`, `std::error::Error` | Yes — these are the operations, not approximations of them | Yes, by the compiler: an empty `[dependencies]` table cannot silently acquire an entry | Justified |
| 2 | This tree is a jj workspace, and here are its root and its main repo | jj's own on-disk shape, and `jj workspace root` | Yes for shape; the gate deliberately does not promise health | Yes — the shape is jj's documented one, and the resolution tests exercise it against native, colocated, nested-Git, symlinked and secondary trees | Justified, with the boundary stated |
| 3 | The repository is chosen by the directory, not by what the calling process inherited | jj's documented selection rule, with four Git variables removed | Yes, *given* the four names are the four that matter | Partly — the mechanism is proved by a test; the completeness of the list is not | Justified, on the same unchecked list as row 4 |
| 4 | The reserved directory is not one Jujutsu owns | A three-element array in this crate | Yes if the array is right | **No** — the array is a copy of jj's contents, and on jj 0.44.0 it was one name short of a colocated workspace's until a human noticed | **Abdication, by the second clause** |
| 5 | The work is durably recorded, and recoverable if the wrong thing lands | jj's snapshot and operation log | Yes, and it is the one row settled by a measurement rather than an argument | Yes for the delegation; **no** at the interface into it | Justified, with a hole at the call |
| 6 | A stop that says what is wrong and names a repair that exists | jj's `undo`, `op log` and `git init` commands in two of the eleven messages, and elsewhere the filesystem, the environment, this crate's own rules, or nothing at all | Yes where a repair is named — the four jj commands are jj's and they do repair — but narrower than the row reads, and row 6 below states how much | Weakly: the remedies are static strings, and one URL in them has already moved | Justified, and the weakest check of the five |

### 1 · Every dependency

The cheapest row, and the only one where the guarantee cannot drift. `std` spawns
the child, reads the directory, and supplies the `Error` trait that lets a
`Refusal` sit inside `anyhow` without this crate ever having named `anyhow`.
Chapter 6 showed what the empty table cost here in full: a hundred-line `Display`
written by hand where `thiserror` would have written it, kept because the messages
are multi-paragraph text that belongs in a body rather than in an attribute, and
because a derived error publishes the variants the whole type exists to hide.

Both clauses pass, and the second passes in the strongest available form: the
check is the compiler. A dependency cannot appear here without a line appearing
in `Cargo.toml`, and that line is inside the corpus this book reconstructs byte
for byte. The row is listed first because it sets the ceiling — this is what a
subtraction looks like when nothing can decay — and every later row is measured
against how far short of it that row falls.

### 2 · A repository abstraction, and any second lane

The guarantee the consumer needs is that jj commands run here will address this
tree. The gate delivers a narrower one on purpose: it walks up for a `.jj/`
directory, canonicalises what it finds, and asks jj itself where the default
workspace is only when the `.jj/repo` entry is a pointer file rather than the
repository. Chapter 2 was explicit that this asks *is there a pointer*, not *is
the repository intact*, and named a test —
`resolution_does_not_validate_the_repository_behind_the_workspace` — that pins
the narrower promise so it cannot quietly widen.

That is a real gap between what is needed and what is guaranteed, and it does not
fail the strength clause, because the remainder is not dropped. A corrupt
repository passes the gate and is refused by the first jj invocation that touches
it, as a `CommandFailed` carrying jj's own stderr. The guarantee is delivered in
two places rather than one, and the second place says something the gate could
not have said as well. A design that made the gate probe repository health would
be slower on every resolution, would duplicate a check jj already performs, and
would still have to handle a repository that broke between the probe and the
command.

> **The consumer's half.** What grove needs from this row is one sentence, and it
> is a sentence about grove rather than about jj: a grove has a
> [stated VCS](../../../CONTEXT.md#stated-vcs), stated once by the driver, and no
> session re-derives it. `docs/adr/jj-is-the-only-lane.md` is where that decision
> is recorded and where the alternatives it closed off are argued. The crate does
> not know any of this. It has no lane to choose because there is only one, and
> the reason there is only one lives in grove's documents, not in these 752
> lines — which is exactly why the crate can be read without them.

### 3 · Ambient repository selection

The guarantee needed is that nothing lying around in the calling process's
environment can redirect a jj invocation to a different repository. jj's own
selection rule supplies it: jj walks up from its working directory, and the
overrides are Git's variables. The seam removes four of them —
`GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR` and `GIT_INDEX_FILE` — from every
child it builds, and fixes the working directory explicitly rather than relying
on the parent's.

The strength clause passes cleanly. The checkability clause is where this row is
interesting, because it half-passes in a way that is easy to mistake for passing.
**The mechanism is checked**: `resolution_ignores_repository_selection_and_temporary_directory_environment`
sets all four names to point at a second, foreign workspace, resolves from a
subdirectory of the intended one, asserts that the intended workspace came back
and that nothing was created in the foreign one, and then snapshots the intended
tree through `is_tracked` and requires the exported Git index to have landed in
the intended repository. That last assertion is the one that discriminates. If the removal loop were
deleted, that test would fail. **The list is not checked**: it is a four-element
array literal, and nothing anywhere goes red if jj — or Git beneath it — begins
reading a fifth name.

And the mechanism is checked by exactly one of its four members. Deleting
`GIT_INDEX_FILE` from the array turns the test red; deleting `GIT_DIR`,
`GIT_WORK_TREE` or `GIT_COMMON_DIR` — one at a time, against jj 0.45.1 — leaves
it green. The honest reading of three green mutations is *this test does not
detect them*, which is weaker than *jj ignores them*; the stronger claim was
checked separately and separately narrowed, and
[chapter 3](03-subprocess-seam.md#the-selectors) states exactly how far it goes —
the four subcommands this crate runs leave a foreign repository untouched under
all three. Three of the four names are therefore removed on an argument about a
class of child, and the fourth on a check. That is not a hole to close: the array
guards against a Git-aware child, and a test can only ever reach the children this
crate actually spawns.

This is the copy failure at its cheapest, and it is worth naming here precisely
because the verdict is still *justified*. The direction of error is the good one:
removing a variable jj does not read costs nothing, and the set of Git variables
that select a repository is small, old and slow-moving. The row passes on a
margin rather than on a check, and the margin is the argument.

### 4 · A vocabulary for its consumer

This row is the chapter's difficulty and has its own section below.

### 5 · Transactions, and history added by a read

The only row where a number settles the question. The consumer needs its work
durably recorded and recoverable if the wrong thing lands, and jj supplies both:
it snapshots the working copy before each command, and its operation log keeps
every previous state reachable. The strength clause therefore passes on a claim
about jj that a reader can verify in jj's own documentation, rather than on a
claim about this crate they would have to take on faith — and chapter 5 listed the
six mechanisms this makes unnecessary, from a rollback proof to a quarantine area.

The one place the crate lets jj snapshot is `is_tracked`, and chapter 5 did not
argue that exception, it measured it:
`asking_twice_over_an_unchanged_working_copy_records_no_operation` counts entries
in the operation log around two probes and shows an unchanged tree records none.
That is the checkability clause satisfied in its strongest form after the
compiler's — a test that goes red if jj's behaviour moves under the crate.

**And then the row failed at the interface into the delegation — which is where
this book's test earned its keep.** `commit` builds a `root:` fileset per path,
and jj's guarantee for a fileset is about the files it matches, not about the
paths the caller meant. A fileset that matches nothing makes `jj file list` print
nothing and exit 0, and makes `jj commit` take an *empty* commit and exit 0 —
measured on jj 0.44.0, and recorded in chapter 5's account of the path algebra.
`relative` used to render each path component with `to_string_lossy`, so a path
whose bytes were not valid UTF-8 became a fileset naming a file that does not
exist: the delegation was sound and the argument handed across it was silently
wrong, with `is_tracked` answering `false` about a tracked file, or a `Commit`
naming an empty change, and no refusal anywhere.

That hole was found by applying this row's test rather than by a failing test, and
it has since been closed — the render loop refuses a path it cannot express, and
chapter 5 reads the refusal. Two things about the repair are worth keeping. The
first is where the argument for it came from: not from this crate's taste for
refusing, but from jj's own changelog, which says a path whose name is not valid
UTF-8 *"can't be tracked"* and is skipped with a warning. Question 2 of the four
below — *find where the tool underneath states its counterpart, and read it
there* — is what turned a plausible fix into a settled one. The second is that
nothing in the suite had gone red, and nothing would have: the failure was an
argument, not a guarantee.

The test as stated is about the guarantee, and that was about the call. That is
not a reason to widen the test — it is the reason to apply it twice. **A
delegation is only as strong as the argument you pass into it**, and the argument
is code you wrote rather than a guarantee the tool made.

> **The consumer's half.** grove commits at a
> [task commit boundary](../../../CONTEXT.md#task-commit-boundary): one task is one
> focused commit, and sealing it is what makes the next session start from a clean
> change. That is the whole of what the path-scoped commit is *for*, and none of it
> is in the crate. What the crate would have needed in order to have transactions
> is a definition of what a unit of work is, which is grove's word and not jj's —
> so subtracting the transaction and subtracting the vocabulary are, at this row,
> the same subtraction.

### 6 · Any remedy of its own

The needed guarantee is that a stop names a repair that exists and that running it
helps. jj supplies it: `jj git init`, `jj git init --colocate`, `jj undo` and
`jj op log` are jj's commands and they do what the messages say. Chapter 6's
distinction is the sharp one here — every remedy in the file is a statement true
whoever is calling, and none is a statement about a consumer's policy, which the
crate has no way to know. Its opposite appears zero times, across all eleven
kinds. But the guarantee this row claims from jj is narrower than that: **only
two of the eleven messages name a jj command** — `NotAWorkspace` and
`CommitNotRecorded`, which are where all four of the commands above come from.
The other nine point at the filesystem, at the environment, or at this crate's
own vocabulary, or name no remedy at all
([*Refusal*](06-refusal.md#no-remedy-of-its-own) takes them one at a time). So
the row is met by the tool underneath only where the tool underneath has
something to offer, and `PathNotText` — *rename it*, because jj will not track
such a path — is the case that shows that boundary rather than an exception to
it.

The checkability clause passes weakest here. The remedies are string literals, so
nothing goes red if a jj command is renamed, and the file has already supplied the
proof that this decays: the `NotRunnable` remedy printed an installation URL whose
host began answering `301 Moved Permanently`, and re-pointing it took a leaf of
its own (`jj-docs-url-k64`, chapter 6) rather than a failing test. That is the
copy failure again, in the one part of the crate whose entire content is quoted
from another tool. It is justified rather than an abdication because the substance
is right and the decay is visible to a reader the moment they follow the link —
but *visible to a reader who follows it* is a much weaker check than *red in CI*,
and the row should be read as passing on substance while conceding the mechanism.

The [guide's account of undoing a mistake](../../USAGE.md#undoing-a-mistake) is
where a grove user meets the `CommitNotRecorded` remedy in practice, and it is
worth noticing that the crate's message and the guide's section were written
independently and say the same thing. That is not a check either. It is two copies
of jj's behaviour agreeing, which is what two copies do until one of them stops.

<a id="the-uncomfortable-one"></a>
## The one where the answer is least comfortable

Refusal 4 fails the test, and it fails the clause that is hardest to notice.

The crate's one addition is a control directory inside `.jj/`, and the guarantee a
consumer needs from it is exactly one sentence: *the directory you have been given
is not one Jujutsu owns*. The row reads like a delegation — jj owns `.jj/`, the
crate reserves a corner of it and refuses to hand over anything jj has claimed —
and it is not one. Nothing in `control_dir` or `validated_namespace` asks jj what
it owns. The answer is a three-element array, `["repo", "working_copy",
".gitignore"]`, compiled into this crate, and that array is a fork of jj's
on-disk layout that tracks it only when someone notices that it has stopped.

It is not a hypothetical fork, and the proof is that it has already drifted once.
On jj 0.44.0 the array was one name short of what a stock `jj git init` produces,
and [*Reserving `grove`*](04-namespace.md#worked-reservation) traced both endings
that gap had: a refusal about the wrong thing in a colocated workspace, and, in
one that is not, a call that succeeded and handed the consumer a directory
sitting on a name jj would want later. What matters at this row is only which
check produced them. Neither ending came from the guard that exists to catch a
name jj owns — one came from the filesystem refusing an entry that was already
there, and the other from nothing at all. The third name is in the array now, so
both endings are gone; what is not gone is the mechanism that let them happen. It
was still the array that was asked and not jj, and a reader who takes the repair
as the answer has mistaken a corrected copy for a delegation.

What makes this the least comfortable row is not that a list is out of date. Lists
go out of date. It is that **the fourth refusal is the one `CONTEXT-MAP.md` uses
to argue about the whole crate** — the claim that `jj-workspace` is deliberately
not a bounded context of its own, because every term in it is Jujutsu's and the
one thing it adds is a namespace it will not name. That argument is exactly right
about the *naming*. The crate genuinely refuses to know what `"grove"` means; the
reader watched the string be passed in. But underneath the naming, the crate did
not delegate the ownership question at all. It answered it from memory, and then
answered it from memory again when the first answer went stale.

Three things keep this a finding rather than a condemnation, and each is checkable
on the page.

**The crate priced the error before it happened.** The four-line comment above
`JJ_OWNED_NAMES` says the list is wrong cheaply in one direction and expensively
in the other. It is the only place in the crate that reasons about the cost of its
own staleness, and it is the reason the failure is a bad refusal message rather
than a consumer writing into `.jj/repo`.

**The alternative is not obviously better.** Asking jj at run time means another
subprocess per reservation with no jj command that answers *what do you own*;
reading `.jj/`'s contents means treating whatever is currently there as owned,
which would make a stale editor file a reserved name. The comparison is chapter
4's, and the list won it on the direction-of-error argument. The test's verdict is
that winning that comparison does not make the result a delegation.

**And the repair is evidence for the finding rather than against it.**
`jj-owned-names-k65` carried it, placed after every crate book because a byte of a
frozen root cannot move while a book that quotes it is being written: the source
change, the page and a green validator run over the book landed in one commit or
would not have landed at all. What repaired the list was a human reading jj's
`.jj/` while drafting chapter 4 — the same instrument that found it, on a
schedule nobody controls. Nothing went red, because in that direction there is
nothing that could: no test in the suite asserts that the array covers `.jj/`.
The repair did leave a check behind, and only in the cheap direction —
`the_git_ignore_jujutsu_writes_is_refused` opens by asserting its colocated
fixture really holds a `.jj/.gitignore`, so the suite now goes red if jj *stops*
writing the entry, and stays silent if jj *adds* a fourth. That is the asymmetry
the constant's own comment describes, turned into the one place the suite can
observe it. It is the second clause failing with its consequence paid rather than
predicted.

The honest summary is the one the test produces without special pleading. Five
rows are subtractions. The fourth is an abdication with a good excuse, a priced
error, and one round of that error already paid — and it is the row the crate's
own architectural argument leans on hardest.

<a id="taking-the-test-away"></a>
## Taking the test to a boundary of your own

The crate is 752 lines and it is not the point. The point is the move it makes six
times, which is the most common move in any code that sits on top of a real tool,
and which is almost never examined because it looks like restraint.

Four questions, in this order, and the third is the one that does the work.

1. **State the guarantee your consumer needs as a sentence that could be false.**
   Not *"we rely on the database"* — *"a row this call returns was committed
   before the call started"*. A guarantee you cannot phrase as a falsifiable
   sentence cannot be compared with anything, and the comparison is the test.
2. **Find where the tool underneath states its counterpart, and read it there.**
   Its documentation, its changelog, its own tests. Not your memory of it, and not
   a comment in your codebase written by someone else's memory of it — the second
   is worse than the first because it looks like a citation.
3. **Ask what in your code would go red if that guarantee changed tomorrow.** A
   compile error is the strongest answer, a failing test the next, a run-time read
   from the tool after that. *Nothing* means you have a copy, and the copy is
   correct today for reasons that will not be recorded anywhere when it stops
   being.
4. **Price the direction of the error.** Being wrong the cheap way and being wrong
   the expensive way are different findings, and knowing which you have is what
   decides whether it is fixed now or scheduled.

Run those four over this crate and you get the table above: rows 1, 2 and 5 answer
question 3 with a compiler or a test, row 6 answers it with a URL a reader might
follow, row 3 answers it with a test that reaches one of the four names it lists,
and row 4 answers it with nothing at all. Rows 3 and 4 are the pair to compare,
and they differ on question 4 as well as on question 3 — one is a slow-moving list
of Git variables where over-removal is free, the other is a fast-moving directory
listing where under-reservation is a collision — and that difference is the whole
distance between *passes on a margin* and *fails*.

The one thing the test will not tell you is whether the subtraction was worth
making. That is a question about the third column of the assembly table, and it is
answered by what the code looks like: 752 lines, four files, no dependencies, and
six chapters that could each be read on their own. A justified subtraction leaves
something small enough to hold in one reading. An abdication leaves the same thing
and moves the missing work into whoever calls you — which, when it is spelled out
as a three-name array, is not visible from either side of the boundary until
someone stands where this chapter stands and asks the question directly.

<a id="the-closed-ledgers"></a>
## The closed ledgers

The book's two ledgers are complete, and the closure is mechanical rather than a
claim this chapter makes. A third account closes here too, and it is the one the
validator does not keep.

<!-- rollup «ownership-blocks» -->
<!-- rollup «source-roots» -->
<!-- rollup «ownership-blocks-owned-by» of="no-dependencies" -->
<!-- rollup «ownership-blocks-not-owned-by» of="no-dependencies" -->
**Ownership.** 11 top-level blocks over 4 source roots, every one of them
`resolved`. Chapter 1 created the whole ownership table at the start, with its own
2 blocks resolved and the other 9 reserved by `defer` directives; each later
chapter replaced its own defers with inserts and turned its own rows. No `defer`
directive remains anywhere in the book, and none may: `F003` reports any defer at
all in final mode, so *"every deferral has become an insertion"* is a statement the
validator refuses to let be false rather than one this page asserts.

<!-- rollup «early-use-rows» -->
**Early use.** 12 rows, every one `explained`. The structure brief fixed eight
of them in advance as the minimum ledger: six forced by chapter 1 tracing the
carried operation through every layer before any of those layers had been read —
`Workspace`, `Refusal`, `Commit`, `control_dir`, `main_repo_of` and `is_tracked`
— and two forced by the narrative order putting the gate ahead of the seam it
calls. `main_repo_of` reached that six late: the brief prescribed the chapter 1
trace that forces it without giving it a row of its own, the developmental edit
added the row to the manifest and to this ledger, and
`jj-workspace-brief-early-use-drift-k215` added the matching row to the brief.
Four more were added under the clause that requires a row before any additional
later-owned name is introduced: three for the chapters that first name a refusal
constructor they do not own, and one for `Refusal::path_not_text`, which arrived
with the refusal itself at `lossy-path-rendering-k66`. Each row turned
`explained` in its owner's slice and in no other.

<!-- rollup «early-use-rows-declared» -->
<!-- rollup «early-use-rows» -->
The manifest declares 8 of the 12 and the book's own ledger carries the
other four. That split was six and five when this chapter was first written, and
the reason was a defect rather than a preference: a manifest `[[early-use]]` row
had to have its first-use anchor in a page present in the snapshot, which a scoped
prefix cannot satisfy for a row whose first use is on a later chapter, so the two
rows first used in chapter 2 could not be declared at all. `early-use-scope-k63`
scoped the anchor check to the proved prefix and moved them into the manifest. The
four that remain ledger-only are first used in chapters 3, 4 and 5, and nothing now
stops them being declared either — they are a choice rather than a workaround.

<!-- rollup «owned-lines-sequence» -->
<!-- rollup «source-owning-chapters» -->
**Owned source.** 98 + 92 + 81 + 58 + 155 + 268 = 752 lines across 6 chapters,
and 0 for this one. The seventh row of that table exists to be zero: a chapter
that owns no source is the shape the structure brief chose for the assembly. The
total is fifty-four lines above the 698 the root brief froze, and the fifty-four
belong to two leaves: forty to `lossy-path-rendering-k66`, sixteen in `lib.rs` and
twenty-four in `refusal.rs`, and fourteen more to `duplicated-cause-k67`, all of
them in `refusal.rs`. The freeze is not a promise that the corpus never moves — it
is the rule that a source change and every page it invalidates land in one commit,
re-proved by the validator, which is what both leaves did.

**Evidence.** Seven claims in this book are held by no test, and each chapter said
so where it made one — *stated here as unasserted rather than left for a reader to
assume covered*. What no chapter could do is say how many there are, because each
saw only its own. This is the third question of the test above turned on the book
itself: what in it would go red if the crate's behaviour moved?

| # | The claim | Where it is stated | What holds it instead of a test |
|---:|---|---|---|
| 1 | jj is absent, and `NotRunnable` is the refusal | [ch. 3](03-subprocess-seam.md#worked-invocation) | nothing: a fixture that removed `jj` from `PATH` could not build its own tree |
| 2 | jj's output is not text, and `OutputNotText` is the refusal | [ch. 3](03-subprocess-seam.md#worked-invocation) | nothing: no jj command can be asked to emit non-UTF-8 on stdout |
| 3 | `GIT_DIR`, `GIT_WORK_TREE` and `GIT_COMMON_DIR` are removed from every child | [ch. 3](03-subprocess-seam.md#the-selectors) | an argument about a class of child this crate never spawns; the suite stays green with any one of them deleted, which is a measure of the suite's reach and not of jj's behaviour |
| 4 | `\` and `\0` are refused as namespaces | [ch. 4](04-namespace.md#the-validation) | measured on jj 0.44.0; the suite reaches that guard through `/` only |
| 5 | `control_dir(".gitignore")` is refused in a native workspace too, where nothing would have collided | [ch. 4](04-namespace.md#worked-reservation) | reading the guard: it consults an array and never the tree; `the_git_ignore_jujutsu_writes_is_refused` stands in the colocated shape only |
| 6 | A path containing `"` or `\` survives into the fileset unchanged | [ch. 5](05-scope-and-commit.md#the-path-algebra) | jj's documented string-literal syntax, and reading the loop |
| 7 | The workspace root is refused as a scope inside itself | [ch. 5](05-scope-and-commit.md#the-path-algebra) | a reading of the code, and nothing else — the weakest row in the table |

The fourth column is the one to read down, because the rows are not equally weak.
Row 4 rests on a measurement taken against jj 0.44.0 and written into the page
beside the code, which is a check a later reader can repeat against a later jj.
Row 6 rests on jj's published syntax. Row 3 is the row whose fourth column is
easiest to over-read: the mutation *can* be written and was run, and it leaves the
suite green — which measures how far this suite reaches, not what jj does. Those
three names guard a Git-aware child, a test can only reach the children this crate
spawns, and so the row rests on the class argument with a measurement bounding it
rather than supporting it. Row 5 rests on a property inherited by construction
rather than tested per case: a guard that reads an array and never the workspace
it is standing in. Rows 1, 2 and 7 rest on nothing repeatable at all.

**This table was nine rows, and the two that left it are the more instructive
half.** Both were `lossy-path-rendering-k66`'s: *a canonicalised parent that does
strip resolves a deleted path*, and *a non-UTF-8 path becomes a fileset matching
nothing, with no refusal*. The first is now
`a_deletion_reached_through_a_symlinked_ancestor_is_committable` and the second
`a_path_whose_name_is_not_valid_utf8_is_refused_rather_than_rendered_lossily`, and
each was checked by mutation before it was credited: removing the clause the test
claims to hold fails that test and no other in the suite. That is what a row
leaving this table for the right reason looks like — not a paragraph asserting
coverage, but an assertion that has been seen to fail.

The two departures are not the same kind of departure, either, and the difference
is worth keeping. The first row was a *gap in the tests* over correct code. The
second was a gap in the code, and the row was honest about it — *the end-to-end
case was not constructed* — so closing it took a source change and not a test. Row 3
is the second row to change hands rather than close: `env-selector-coverage-k68`
took the claim that stood there — that `GIT_INDEX_FILE` is removed and no test
sets it — and closed it, by setting all four names and requiring the index jj
exports for the intended tree to land in the intended repository. What took its
place is the residue that repair exposed: the removal loop's *other* three names,
which the row it replaced had recorded as the ones the test *set* — a description
two other pages had already rounded up into *tested*. Row 5 changed hands
the same way: `jj-owned-names-k65` retired the claim that
stood there — that `control_dir(".gitignore")` collides in a colocated tree and
succeeds in a native one — by putting the name in the list, and what took its
place is the narrower claim the repair itself created. A row leaving this table
by being fixed is the ordinary case; a row leaving it by being replaced with its
own successor is what a hand-maintained list looks like from here.

That is the honest total: this book proves 752 lines byte for byte, and argues
seven claims that nothing goes red on. Chapter 1's row of the verdict table set
the ceiling — a check the compiler performs — and every row here
falls short of it. Saying by how much is what the assembly owed a reader who has
just been taught to ask.

<!-- rollup «source-roots» -->
The [concept index](concept-index.md) and the [source index](source-index.md) are
the two lookup surfaces, and neither is part of the reading order. The source
index is the authoritative record of how the fragment graph reconstructs each of
the 4 files; the concept index is curated navigation into the arguments, and
makes no completeness claim.

<a id="final-verification"></a>
## Final verification

Three commands prove the book, and they prove different things. The first is the
only one that reads the corpus byte for byte.

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/jj-workspace --final --check all
valid: 4 files, 752 resolved lines, 0 deferred lines, final=true
```

`--final` is what makes this different from every scoped run the drafting
sessions made. In scoped mode a later chapter's range may be reserved by a defer
and counted as deferred rather than resolved; in final mode a defer is an error,
every source root must expand to its complete file, and the page inventory must
match the manifest exactly. 752 resolved and 0 deferred is the whole corpus
reconstructed.

```console
$ bash scripts/check.sh
...
=== book-check
  book-check docs/walkthroughs/grove-llm
valid: 4 files, 1017 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/grove-loop
valid: 13 files, 10557 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/jj-workspace
valid: 4 files, 752 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/keyed-launch
valid: 9 files, 2073 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/ordinal-fs-tree
valid: 17 files, 8720 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/overview
valid: 3 files, 204 resolved lines, 0 deferred lines, final=true
  6 book(s) checked, 0 failing
  ✓ book-check

check: all 8 principal checks pass
```

The umbrella. It runs `book-check --final --check all` over every book root under
`docs/walkthroughs/` **by discovery** rather than from a list, which is why this
book has been inside the gate since chapter 1 created its directory and why every
drafting session but this one left the script red on `book-check` alone. It also
runs the repository's own tests, and two of those cover this book without naming
it: `every_repository_markdown_reference_resolves` sweeps every Markdown file in
the repository, and `crates/grove/tests/reference_navigation.rs` fails a book root
that has no row in the *Documentation ownership* table of `docs/ARCHITECTURE.md`.

```console
$ cargo test --locked -p jj-workspace
     Running tests/environment.rs
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/workspace.rs
test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The crate's own suite, which is the evidence this book cites and does not
reproduce. Thirty-two tests, all of them integration tests: the crate carries no
unit tests and no documentation tests, because its public surface is the three
types and six functions [*Orientation*](01-orientation.md#public-surface) listed
— `Workspace`, `Commit` and `Refusal`, with `resolve` the only constructor — and
every claim worth making about it is a claim about a real workspace on disk. It
is outside the corpus by design — chapter 1 said so — and every claim in the six
chapters that names a test names one that runs here.

<!-- rollup «source-roots» -->
<!-- rollup «owned-lines-total» -->
<!-- rollup «chapters» -->
The book is complete: 4 roots, 752 lines, 7 chapters, two lookup surfaces,
zero deferred ranges. What it argued is that six refusals are one design, and what
it leaves the reader with is the test that tells a subtraction from an abdication
— including, on the crate's own fourth refusal, the answer nobody drafting it
would have preferred.

[Previous: Refusal](06-refusal.md) | [Contents](README.md)
