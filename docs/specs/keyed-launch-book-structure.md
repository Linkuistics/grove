# keyed-launch — book structure brief

## Status and provenance

This is the structure brief for the book at `docs/walkthroughs/keyed-launch/`.

**This document is authored, not recovered.** It was first settled in the
`keyed-launch-structure-k34` interview, for a ten-page book of a crate that
also resolved a key to a command template. That half of the crate is deleted.
`book-thesis-k24` restated the brief for the seven pages that remain, and its
decision log records what changed and why. It follows
[`jj-workspace-book-structure.md`](jj-workspace-book-structure.md),
[`overview-book-structure.md`](overview-book-structure.md) and
[`grove-llm-book-structure.md`](grove-llm-book-structure.md) and is uniform
with them.

**The intended outcome changed without a second interview.** The reasoning is
under *Audience and intended outcome*, and it is the interview's own. The human
can overrule it.

**This brief is the human contract; the manifest is its machine-readable form.**
The chapter sequence and ownership mapping below are what
`docs/walkthroughs/keyed-launch/walkthrough.toml` records as its `[[page]]` and
`[[block]]` groups. Where the two disagree, that is a defect in one of them, not
a licence to prefer either.

**What distinguishes this corpus.** Seven roots and 2,010 lines, of which
`src/run.rs` is 1,171: more than half the corpus is one file, and two chapters
divide it.
`src/channel.rs` contains an inline `#[cfg(test)]` module inside a root, at
lines 289–453. The corpus exception inventory in
[`walkthrough-books.md`](walkthrough-books.md) carries no `keyed-launch` row, so
those 165 lines are owned, reconstructed and explained like any other. And
`src/confinement.rs` arrived after the interview, with a chapter of its own
that follows the assembly.

## Audience and intended outcome

The audience is settled by decision 7 of `plan-k1` and is not re-opened here: a
reader who knows Rust and Jujutsu and has driven a grove, for whom grove's
vocabulary is linked to the glossary and never re-taught, and whose entry point
to the system is [`USAGE.md`](../USAGE.md). This reader has watched a session
end without ever seeing what ended it.

**The intended outcome is the ending test.** At the end the reader can take any
layer in their own code that starts work it does not understand — a launcher, a
job runner, a supervisor — and ask *what ends it, and who decides?* A layer
that does not know what the work is cannot decide that it is done. It can only
observe. The reader can name the three things this crate observes, what breaks
without each, and the test that holds it:

- **The child exits.** Without it, a child that dies without speaking is waited
  on for ever. The crate does not read the status as meaning. Held by
  `a_child_that_never_signals_ends_with_no_token` in
  `crates/keyed-launch/tests/launch.rs`.
- **The file appears.** Without it, an interactive child that returns to its
  prompt never ends, or the launcher guesses that it has. Held by
  `a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace` and
  `a_child_that_signals_and_exits_inside_the_grace_is_never_touched` in the same
  file.
- **The launcher is signalled.** Without it, the launcher dies and leaves its
  child on the terminal, or exits 0 and reports work that was cut short. Held by
  `an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` in
  `crates/keyed-launch/tests/interrupt.rs`.

The reader can also name the ending none of the three reaches: an interactive
child that finishes its turn and never signals. The launch stalls, and the
obligation belongs to the layer that instructs the child.

All three are exercised by tests the book names. Chapter 6 states the test and
applies it to chapters 1 to 5.

Three candidate outcomes were rejected. **The pass-through test** — ask *where
does this layer learn what the value means?* and name the three places a layer
learns it anyway — was the interview's choice. Two of its three places were the
template half's: composing a value on the way in, and re-reading it on the way
through. The corpus no longer shows either. The interview had called the ending
test *sharper than the chosen outcome and genuinely transferable* and set it
aside on one ground, that it left the template chapters unserved. With those
chapters gone the ground is gone. What the pass-through test still held, that
nothing is added to a launch, is the spine's and is kept there. **Take it as a
dependency** is ruled out by fact: [`RELEASING.md`](../RELEASING.md), *One
release, eight packages, one tag*, settles that this package ships inside
grove's cut with `release = false` and no lane of its own. **The maintainer
outcome** — change the escalation, or add an entry point, without breaking
anything — follows for free from source-exactness, as each precedent found for
theirs.

## The spine: every word, every name and every path is the caller's

**Chapters 1 to 5 each open on the one thing their stage must not add and must
not interpret.** The spine is recovered from the source rather than imposed on
it. `src/lib.rs` states it in its opening lines: *Nothing here chooses a
program, reads a configuration or understands what the child is for: every word
of the command, every variable name and every path is the caller's.*
[`CONTEXT-MAP.md`](../../CONTEXT-MAP.md) records the refusal of the word
**session** as the naming decision that keeps the crate from being a bounded
context of its own.

| The stage | What it must not add or interpret | Chapter |
|---|---|---:|
| the manifest, the library root, the error type and the argv | what the program does and what the token says: one dependency, no domain, and a command held exactly as its caller built it | 1 |
| the completion channel | appearance is the event; the token's content is the caller's to read | 2 |
| the environment, the terminal, the spawn | no argument, no flag, no variable the caller did not write | 3 |
| the watch, the escalation and the launcher's own signals | it cannot know the child is done — only that the child said so | 4 |
| the channel's inline tests | a filename is held to a grammar without anyone asking what it names | 5 |
| the noninteractive mode and the confinement policy | no program looked up for the caller, and no retry without confinement | 7 |

Chapter 7 keeps its row in its body and does not open on it. It was written
with `src/confinement.rs`, after the interview.

**The spine and the outcome are different things.** The spine is what each
chapter refuses. The outcome is what the refusals leave: a crate that knows
nothing about the work has three things to observe and nothing to infer from.

One alternative was rejected. **A launch ends out of band** is the outcome, and
as a spine it would give chapters 1 and 3 nothing to open on: the manifest and
the environment are about what is not added, and they earn the ending rather
than describe it.

## Chapter sequence

Ten pages: seven chapters, six of which own source, with `README.md`,
`concept-index.md` and `source-index.md` as the contents page and the two
lookup surfaces. The lookup pages are not chapters and not alternate
explanatory paths.

| Order | File | Page ID | Title | Slice |
|---:|---|---|---|---|
| 1 | `01-orientation.md` | `orientation` | Orientation | `understands-neither` |
| 2 | `02-the-channel.md` | `the-channel` | Appearance is the event | `appearance-is-the-event` |
| 3 | `03-the-job.md` | `the-job` | The child is a job | `nothing-else-added` |
| 4 | `04-the-escalation.md` | `the-escalation` | The watch and the escalation | `the-launchers-job` |
| 5 | `05-how-checked.md` | `how-checked` | How this is checked | `checked-without-meaning` |
| 6 | `06-what-ends-a-launch.md` | `what-ends-a-launch` | What ends a launch | `assembly` |
| 7 | `07-confined-jobs.md` | `confined-jobs` | Confined noninteractive jobs | `confined-jobs` |

`assembly` owns no production source and is therefore final-only: it has no
scoped prefix to prove. Both elicited precedents close the same way —
`jj-workspace`'s `07-what-jj-owns` and `grove-llm`'s `07-what-order-holds` each
carry slice `assembly` and own nothing — and each states its transferable test
there.

**Chapter 7 follows the assembly.** The assembly applies the test to chapters 1
to 5, which are the interactive launch the interview covered. Chapter 7 adds a
process mode and a filesystem policy over the same supervisor, so it changes no
row of the assembly's table. This brief records that order and does not argue
for it.

**Slice IDs are named for the rule each chapter carries**, so the slice list
reads as the book's spine. `understands-neither` is chapter 1's rule as that
chapter now states it: the crate understands neither what the program does nor
what the token says. The one slice token that equals its page ID is
`confined-jobs`. Slice IDs carry no Grove task key, for the reason recorded in
`jj-workspace-structure-k17`'s decision 8, applied here rather than re-elicited.

**The pages follow the launch.** An argv exists, a path is drawn, a child is
spawned, and the launcher waits and acts. That is also the order of the
four-step trace chapter 1 carries, so each of chapters 2 to 4 takes one step of
it apart.

**Chapters 3 and 4 divide `src/run.rs` by whose signal it is**, at the price of
four ownership blocks in that root. Chapter 3 owns the launch's shape and
everything done *to the child*: its process group, its terminal, its
dispositions, its environment, the spawn. Chapter 4 owns the watch and what follows
it: the supervisor's state machine, the escalation, and the launcher's
own SIGTERM latch, which is an ending the channel cannot express. The cost is
recorded in *Early uses the order forces*: the spawn uses three of chapter 4's
items.

## Concept and seam responsibilities

### `README.md` — reader contract

State the audience, scope, exclusions, source-authority rule, exact-fragment
claim, canonical page order, lookup paths, and the distinction between scoped and
final completeness. Explain how to recognise a fragment definition, insertion,
source root and deferred hole without duplicating the full grammar. State the
ending test. Cite [`USAGE.md`](../USAGE.md) at `usage-running-grove`, the one
guide link the contract permits and requires from this page. State the book's
boundary: it explains one crate, and it stops at the argv it was handed and the
process it spawned.

### 1 · Orientation — understands neither

Owns `Cargo.toml`, `src/lib.rs`, `src/error.rs` and `src/argv.rs` whole.
Responsible for: what `keyed-launch` is and what it refuses to be — the crate
that owns how a child is spawned, how its end is learned and how it is stopped,
and none of what the child is for; that the crate keeps the name it took when
it also resolved a key to a command template; the book's question,
stated the way the `README.md` and chapter 6 state it; the one dependency, read
as the evidence of the claim — `libc` is the syscalls the crate cannot reach
from `std`, and there is nothing else; `release = false` as an answered
question rather than an open one; `lib.rs` as the book's map, its paragraphs
named against the chapters that own them; the one opaque error — that a variant
list would be a second interface and every new diagnostic a breaking change —
and the obligation that replaces it: name what is wrong, name where, name what
fixes it; and `Argv`, the type a command arrives in, with private fields, no
method that changes one, and a public constructor that checks nothing, so that
what was put in is what is spawned and the caller answers for the words.

It carries the worked example at low resolution, and the cast of public names
with the chapter that owns each.

### 2 · Appearance is the event — the channel

Owns `src/channel.rs` lines 1–288. Responsible for: what the channel is and why a
launch needs one at all — an interactive child returns to its prompt when it
finishes rather than exiting, so its own exit is not the event anyone is waiting
for; **allocation picks a name and writes nothing**, which is what makes
*appearance* the event; the name grammar — a recognisable prefix plus 128 bits of
randomness — and why the prefix is named for `signal` and matches what grove's
driver already leaves behind; `DRAW_RETRY_LIMIT` as a bound rather than an
unbounded retry; why `allocate` checks the directory rather than leaving it to the
child's first write; `Token` as opaque to this crate and readable to its caller,
and the line framing as framing rather than interpretation; `signal` as a free
function because the two ends are different processes; and `discard_abandoned`,
whose exactness is the point — a looser rule would let this crate's cleanup delete
a neighbouring file in a directory whose other contents belong to the consumer.

This is the chapter that cites [`CONTEXT.md`](../../CONTEXT.md) at
`loop-control-channel`, at its first use of *channel* for the thing grove's
glossary already names.

### 3 · The child is a job — nothing else added

Owns `src/run.rs` lines 1–146 and 333–837. Responsible for: `Escalation`'s two
waits and what each is for; `Launch` as *everything one launch is*, every field
the caller's, and `run`'s promise that nothing else is added — no argument, no
flag, no variable; the scrub list as the caller's obligation discharged here, and
why an environment is inherited rather than addressed, so a nested launcher would
otherwise hand a child a live channel path belonging to somebody else's launch;
`cwd` and why `None` is rarely what a launcher wants; `Ended`, `Group` as the
survivor report that sits beside the child's status rather than replacing it,
and `End`'s three cases, and the distinction the chapter must make carefully — `Signalled` is
narrower than *a token appeared*, because a child that signals and exits inside
its own grace was never touched and comes back `Exited` with a token;
`DEFAULT_DISPOSITION_IN_CHILD` and its argument that **only an ignored disposition
survives `execve`**; `Terminal::open` and why `/dev/tty` rather than stdin is the
gate that needs no flag; the terminal's attributes saved and restored, and the
lease that decides whether a launch held the foreground and from which group it
takes the terminal back — the child's own after an exit, any but the launcher's
and the session leader's after a death by signal; the spawn itself — the child's own process group, the
terminal handed over, and why a new *session* was rejected for the interactive
launch; the observed entry point and its two events; and `POLL_INTERVAL` as not
a knob.

The block also holds the detached mode's setup and its descriptor bound,
because they establish the process the watch owns.
`End::Interrupted` is defined here and produced only in chapter 4; the chapter
names it and defers.

### 4 · The watch and the escalation — the launcher's job

Owns `src/run.rs` lines 147–332 and 838–1171. Responsible for: `Watch` as the
supervisor's state machine; `watch`'s three observables and the honest statement
that they are the only three ways a launch ends — **a child that finishes its work
and never signals reaches none of them**, and the launch stalls rather than ends,
which the source names as a real failure mode with no cheap fix and the book must
not soften; why ending an interactive child is the *launcher's* job — it is the
child's parent, outside whatever sandbox the child runs under, and a child asked
to end itself may be denied silently; the escalation addressed to `-pgid` as well
as the pid, and what that reaps; the end of every launch in its contracted
order — the exit observed without reaping, never a stop, the group killed twice
while the zombie child still reserves its ID, the reap, the terminal returned,
and the group confirmed gone on ESRCH alone or reported as `Group::Present` —
with the measured platform facts behind each step; the launch `Mode` and the
cancellation it selects, forwarding with a kill-grace or killing a confined
child at once; `kill`'s deliberately ignored failure as *the shell's
`kill … 2>/dev/null`, written down*; and then the launcher's own signals —
`INTERRUPTED_BY` as process-global because a disposition is, latched because the
launch on which the child finally exits still has to report it, carrying the
*number* because a launcher that re-raises SIGTERM for a SIGHUP has told its
parent the wrong thing, and cleared immediately before each spawn;
`take_interrupt` for the signal that arrives between launches; `reraise` and why
an exit code cannot express *was signalled* at all; SIGINT caught only for a
launch with no terminal; no handler over a disposition the launcher ignores; and
the repair for an inherited ignored SIGCHLD, latched once per process, never a
handler.

### 5 · How this is checked — checked without meaning

Owns `src/channel.rs` lines 289–453. Responsible for: the eleven inline channel
tests, read as what a `#[cfg(test)]` module inside a root buys that an
integration test cannot — reaching `is_channel_name`, a private function whose
exactness chapter 2 argued and only this module can pin; and what each test
holds, against the section of chapter 2 that argued it.

The chapter states, once, why these tests are in the corpus and the files under
`crates/keyed-launch/tests/` are not: a root is `src/**/*.rs`, and the corpus
exception inventory carries no `keyed-launch` row.

### 6 · What ends a launch — assembly

Owns no source. Responsible for: stating the ending test in the form under
*Audience and intended outcome*, and applying it to chapters 1 to 5 — for each
of the three observables, what breaks without it, which chapters built it, and
by which test; that seeing an ending is not performing one, so the escalation is
the launcher's and reaches the whole job; the one ending no observable reaches,
which is the caller's to close at the layer that instructs the child; and the
test as four steps a reader can run over a layer of their own. It closes the
ledgers and records the final verification.

### 7 · Confined noninteractive jobs — confined jobs

Owns `src/confinement.rs` whole. Responsible for: the noninteractive process
mode and what it changes — no terminal, stdin at EOF, output to a caller-owned
file, a new session; `Confinement` as a mandatory boundary with one writable
directory and explicit read grants; the executable resolved from an absolute
path and never looked up; the macOS and Linux backends and the refusal on any
other platform; that neither failure path retries without confinement; and
`regular_file_at`, the one read of a result that the policy leaves safe.

It carries a worked example of its own, and it points to chapters 3 and 4 for
spawning, cancellation and reaping.

## The mapping onto the corpus

### Top-level ownership blocks

| Root | Lines | What the block is | Chapter |
|---|---:|---|---:|
| `Cargo.toml` | 1–42 | manifest | 1 |
| `src/lib.rs` | 1–50 | the crate's account of itself, and its exports | 1 |
| `src/error.rs` | 1–38 | the one opaque error | 1 |
| `src/argv.rs` | 1–39 | the type a command arrives in | 1 |
| `src/channel.rs` | 1–288 | completion channel | 2 |
| `src/channel.rs` | 289–453 | inline test module | 5 |
| `src/run.rs` | 1–146 | the launch's shape | 3 |
| `src/run.rs` | 147–332 | the watch state, the launcher's signals and its entry state | 4 |
| `src/run.rs` | 333–837 | terminal, its lease, detached mode and spawn | 3 |
| `src/run.rs` | 838–1171 | the end of the group, supervise and escalate | 4 |
| `src/confinement.rs` | 1–217 | confinement policy | 7 |

### Where a file's concerns split across chapters

Two roots split. `src/run.rs` supplies the launch to chapter 3 and supervision
to chapter 4, and chapter 4's first block sits between chapter 3's two.

**`src/channel.rs` splits at line 289**, the `#[cfg(test)]` attribute, with
production in chapter 2 and the inline module in chapter 5. This is the only
split in the book made at a `cfg` boundary rather than a conceptual one, and the
reason is that the module's subject is *assurance*, which is chapter 5's, while
its subject matter is chapter 2's — so chapter 5 explains the tests against
chapter 2's fragments, which are behind it.

### Measurements

| Chapter | Lines | Share |
|---:|---:|---:|
| 1 · Orientation | 169 | 8% |
| 2 · The channel | 288 | 14% |
| 3 · The job | 651 | 32% |
| 4 · The escalation | 520 | 26% |
| 5 · How this is checked | 165 | 8% |
| 6 · What ends a launch | 0 | — |
| 7 · Confined jobs | 217 | 11% |
| **total** | **2,010** | **100%** |

Shares are rounded, so they need not sum to 100. Per root: `Cargo.toml` 42,
`src/lib.rs` 50, `src/error.rs` 38, `src/argv.rs` 39, `src/channel.rs` 453,
`src/run.rs` 1,171, `src/confinement.rs` 217.

## What each chapter's prose owes

Measured: **35% of the corpus is comment prose**, 705 of 2,010 lines, counting
every line whose first non-blank characters open a comment, and it is not spread
evenly. `src/lib.rs` is 70% and `Cargo.toml` 52%. `src/run.rs` is 44% (510 of
1,171 lines), and there the prose is *argument*: the escalation, the child
dispositions, the interrupt latch, the terminal and the end of the group each
carry a full case in situ. `src/confinement.rs` is 10%. Three things are what each chapter's prose
owes, and a technical review checks for them:

1. **Adjudicate the claim.** For every argued claim, name the behaviour it rests
   on, the test that proves it, and the alternative rejected with what it would
   have cost. A doc comment rarely names the test that holds it, and this
   crate's evidence is 2,799 lines in seven files against a 2,010-line corpus.
2. **Carry the through-line.** Show where a decision in one place rests on a
   decision in another: `Channel::allocate` writing nothing is *why* `watch` can
   treat appearance as an event at all; the scrub list is *why* a nested child
   cannot end somebody else's launch; and the process group the spawn creates
   is *why* `kill` can address the whole job. The source states each where it
   is made, and the book connects the two ends.
3. **Carry the load where the source does not, and only there.** **Chapters 3
   and 4 do not restate**: the comments already argue, the fragment graph quotes
   them verbatim on the page, and prose that paraphrases an argument the reader
   has just read in the source is the failure mode this obligation exists to
   name. There, the prose connects the arguments across items and names the
   test, and does nothing else. **Chapter 7 supplies the argument**, because
   `src/confinement.rs` states almost none of its own.

## Worked examples

**The carried example is one of grove's launches, told strictly from the crate's
side.** The reader knows exactly what the words mean and watches the crate not
care, which is the spine made visible. It starts from an argv the caller built:

- `Argv::new("claude", ["--model", "opus", "<the mandate>"])`, the last argument
  a long prompt with spaces and newlines in it;
- the control directory `/work/atlas/.jj/grove`;
- the channel variable `GROVE_SIGNAL_FILE`, and a scrub list that names it;
- the working directory `/work/atlas`, and an escalation of two seconds and
  five.

| Chapter | Anchor | Starts at | Observable end |
|---:|---|---|---|
| 1 | `#the-launch-in-outline` | an argv the caller built | a running child and a token, named but not traced |
| 2 | `#a-path-and-nothing-else` | `Channel::allocate` in the control directory | a path that does not exist, and a token read back after `signal` |
| 3 | `#the-spawn` | `run` with that argv and channel | the child in its own group, holding the terminal, with `GROVE_SIGNAL_FILE` set and the scrub applied |
| 4 | `#the-two-graces` | the token appearing | grace → SIGTERM → kill-grace → SIGKILL; and the launcher's own SIGTERM as `End::Interrupted` |

Chapters 5 and 6 carry no step of it. Chapter 5's tests work on a temporary
directory and spawn nothing, and chapter 6 argues from the crate's own tests.

Rejected: the crate's own test fixtures as the carry. They are real and pinned,
but each test builds and discards its own script, so no single thread runs
through them.

**The second ending is the launcher's own signal**, carried in chapter 4, where
the same launch runs twice. It is the second ending because it is the one the
channel cannot express, and
`an_interrupt_is_reported_against_the_launch_it_arrives_in_and_no_other` pins it.

**Chapter 7 has its own example**, at `#worked-confined-job`: a confined
`/bin/sh` job that copies a staged file, signals and exits, and whose result the
caller reads through a held directory. The carried example is an interactive
launch and cannot show a launch with no terminal.

## Early uses the order forces

Five rows, all in the manifest's `[[early-use]]` entries and the book's
early-use ledger.

| Term | First used | Owned by | Why the order forces it |
|---|---:|---:|---|
| `Channel`, `Token`, `signal` | 1 | 2 | chapter 1's cast names the public surface |
| `run`, `run_observed`, `LaunchEvent`, `Launch`, `Ended`, `End`, `Escalation` | 1 | 3 | the same |
| `reraise`, `take_interrupt` | 1 | 4 | the same |
| `run_noninteractive`, `run_confined`, `Confinement`, `regular_file_at` | 1 | 7 | the same |
| `install_termination_handler`, `INTERRUPTED_BY`, `supervise` | 3 | 4 | the spawn installs the handler, clears the latch and calls the supervisor |

Chapter 1 names every public type before its owner explains it because
`src/lib.rs` is the crate's own map and reproducing it is what chapter 1 is for.
That is one block naming the cast at low resolution, and it is the same shape
`jj-workspace`'s and `grove-llm`'s orientation chapters took.

## Outbound links

```toml
[guide]
path    = "docs/USAGE.md"
anchors = ["usage-running-grove", "usage-session-lifecycle"]

[[glossary]]
path    = "CONTEXT.md"
anchors = ["loop-control-channel"]
```

Three anchors, all of which exist as explicit `<a id="…"></a>` lines preceding
their headings.

`usage-running-grove` is the `README.md`'s required guide citation and the
reader's entry point. `usage-session-lifecycle` says what a launch is *for*,
so no chapter has to, and it is where chapter 6 sends the obligation the
crate cannot hold. `loop-control-channel` is cited at chapter 2's first use of
*channel*.

Rejected: `usage-review-composition`, a false friend whose *escalation* is the
methodology's review escalation and not the kill escalation; `usage-driver-lease`
and the glossary's `session-epoch` and `driver-lease`, which describe what grove
does with a channel path rather than anything this crate knows; and
`[guide] omitted`, which [`CONTEXT-MAP.md`](../../CONTEXT-MAP.md) rules out — it
holds `keyed-launch` as **not** a context of its own, unlike `ordinal-fs-tree`,
the one book granted that exemption.

**The records this crate is governed by are named and never cited.** Decision 7
of [`module-decomposition.md`](module-decomposition.md)
and [*the launched child is a job*](../adr/the-launched-child-is-a-job.md) are
where this crate's decisions live, and neither of them is a permitted link target
from a book page — the contract closes a book's local targets to its own pages,
its own roots, the guide and the glossary. The book states what each record
settles, in prose, at the chapter that keeps it, and links none of them.

## The book's row in the ownership table

`docs/ARCHITECTURE.md`'s *Documentation ownership* table carries the book's row,
and `every_book_root_has_a_documentation_ownership_row`
(`crates/grove/tests/reference_navigation.rs`) holds it there.

## What the book deliberately does not cover

### Rust, libc, and the system calls by name

`signal(2)`, `setpgid(2)`, `tcsetpgrp(2)`, `execve(2)` and `getpgrp(2)` are used
and their *consequences* are argued at length, because the consequences are the
crate's design. Their signatures, their error sets and their portability are the
operating system's documentation. The `unsafe` blocks are explained by their
`SAFETY` comments, which the fragments reproduce, and the book adjudicates the
argument each comment makes rather than teaching what `unsafe` is.

### Non-Unix platforms

The crate is Unix-only by construction — `std::os::unix::process::CommandExt`,
process groups, `/dev/tty` — and no chapter treats portability as an open
question.

### Everything grove does with a launch

What a session is, what a kind means, the driver lease, the session epoch, the
task tree, the methodology, and how grove chooses the command it hands over.
Chapter 1 says that grove builds one argv for each session and another for a
standalone invocation, and the book's whole spine is that the crate does not
know the rest. [`USAGE.md`](../USAGE.md) at `usage-session-lifecycle` is where
that account lives, and the `README.md` points there once.

### The `tests/` directory

`crates/keyed-launch/tests/` is 2,799 lines across seven files, one of them a
module `src/run.rs` includes by path, and every one of them is evidence rather
than a root. The book names a test whenever it
adjudicates a claim, and reproduces none of them. The eleven tests it *does*
reproduce are the inline module inside `src/channel.rs`, which are corpus
because a root is `src/**/*.rs`.

### The two decision records

Named at the chapters that keep them, never cited, for the link-contract reason
under *Outbound links*. The book does not reproduce their reasoning: it shows the
lines that keep them and says which record each line answers to.

## What this brief does not settle

The prose itself, the figures, the concept-index and source-index entries, the
fragment identifiers, the exact block decomposition *inside* each top-level
ownership block, and the concrete values chapter 1 fixes for the carried
example. Those are the book's. This brief settles the shape.
