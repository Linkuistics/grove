# keyed-launch — book structure brief

## Status and provenance

This is the structure brief for the book at `docs/walkthroughs/keyed-launch/`,
which does not exist yet and which this document is written ahead of, in the
form [`walkthrough-books.md`](walkthrough-books.md) requires under *The
structure brief*. It settles what that specification deliberately does not: the
chapter sequence, the mapping of that sequence onto the corpus, each chapter's
worked example, the early uses the order forces, and what the book does not
cover.

**This document is authored, not recovered, and it precedes its book.** Every
decision below was settled in the `keyed-launch-structure-k34` interview and is
recorded, with its rejected alternatives, in that leaf's decision log. It follows
[`jj-workspace-book-structure.md`](jj-workspace-book-structure.md),
[`overview-book-structure.md`](overview-book-structure.md) and
[`grove-llm-book-structure.md`](grove-llm-book-structure.md), the three briefs
elicited before their books, and is uniform with them.

**This brief is the human contract; the manifest is its machine-readable form.**
The chapter sequence and ownership mapping below are what
`docs/walkthroughs/keyed-launch/walkthrough.toml` records as its `[[page]]` and
`[[block]]` groups. Where the two disagree, that is a defect in one of them, not
a licence to prefer either.

**What is different about this corpus.** Nine roots and 2,073 lines, the largest
book but one, and the first whose structural problem is neither one big file nor
many small ones but an **imbalance in how well the source explains itself**:
`src/run.rs` is 53% comment prose and argues nearly every claim it makes in
situ, while `src/templates.rs` — the biggest root at 670 lines — is 13%, and it
is where every rule the decision records state actually binds. It is also the
first book whose corpus contains an inline `#[cfg(test)]` module *inside a root*
(`src/channel.rs` lines 272–404): the corpus exception inventory in
[`walkthrough-books.md`](walkthrough-books.md) carries no `keyed-launch` row, so
those 133 lines are owned, reconstructed and explained like any other. And it is
the first book with a **residue obligation**: `docs/ARCHITECTURE.md` line 1188
carries the one `residue(grove-loop, keyed-launch)` marker in the document.

## Audience and intended outcome

The audience is settled by decision 7 of `plan-k1` and is not re-opened here: a
reader who knows Rust and Jujutsu and has driven a grove, for whom grove's
vocabulary is linked to the glossary and never re-taught, and whose entry point
to the system is [`USAGE.md`](../USAGE.md). This reader has written a
`config.kdl`, has had grove refuse a malformed one, and has watched a session end
without ever seeing what ended it.

**The intended outcome is the pass-through test.** At the end the reader can take
any layer in their own code that carries a value from a human's file to an effect
in the world — a configuration to a process, a query to an engine, a route to a
handler — and ask *where does this layer learn what the value means?* The right
answer is nowhere, and the reader can name the three places such a layer usually
learns it anyway, what each costs, and the test that catches it:

- **On the way in** — assembling one value out of more than one source. The cost
  is that nobody can see the whole of it in one place and no single author owns
  it. Held by `a_key_only_the_overlay_declares_does_not_resolve` and
  `an_overlay_replaces_a_whole_template_and_reports_its_own_path` in
  `crates/keyed-launch/tests/templates.rs`.
- **On the way through** — re-reading a value it has already read. The cost is
  that a value with a space becomes two arguments, a `#` truncates the line, and
  a `$(…)` becomes a command. Held by
  `a_slot_value_is_one_argument_whatever_it_contains`,
  `shell_metacharacters_stay_literal` and
  `an_unquoted_hash_is_refused_rather_than_truncating_the_argv` in the same file.
- **On the way out** — inferring what came back, or adding to the launch what the
  operator did not write. The cost is a launcher that decides for itself that a
  child is done, or hands it an environment it never asked for. Held by
  `a_child_that_never_signals_ends_with_no_token` and
  `a_scrubbed_variable_is_removed_from_an_inherited_environment` in
  `crates/keyed-launch/tests/launch.rs`.

All three are provable inside this book's 2,073 lines. The closing page states
the test and applies it to all nine source-owning chapters.

**This is not `jj-workspace`'s spine wearing a different hat, and the book says
so once.** That book's every chapter opens on something the crate declines to
own *and names who owns it instead* — `std`, jj, the consumer. It is a book about
delegation, and each refusal has an address. This crate names no other owner,
because the meaning does not exist anywhere inside it to be delegated: a key is
an opaque string and stays one. Chapter 1 states the difference in a sentence and
no later chapter returns to it.

Three candidate outcomes were rejected. **The out-of-band completion test** — name
the three ways a launch can end and what breaks when one is missing — is sharper
than the chosen outcome and is genuinely transferable, but reaches only
`src/run.rs` and `src/channel.rs`; chapters 2–5 are 762 lines, 37% of the corpus,
and the book would have a stated outcome its first half does not serve. It
survives as chapter 8's thesis rather than the book's. **Take it as a dependency**
is ruled out by fact — [`RELEASING.md`](../RELEASING.md), *One release, six
packages, one tag*, settles that this package ships inside grove's cut with
`release = false` and no lane of its own, which `Cargo.toml` lines 41–45 call
settled rather than open. `jj-workspace`'s brief rejected the identical outcome on
the identical fact. **The maintainer outcome** — add a fifth slot, or a second
overlay, without breaking anything — follows for free from source-exactness, as
all three precedents found for theirs.

## The spine: the words are the words the file holds

**Every chapter opens on the one thing this stage must not add and must not
interpret.** The spine is recovered from the source rather than imposed on it.
`src/lib.rs` states it in its first paragraph, lines 4–9: *a consumer names a
key; a template names a program. Nothing here understands either: a key is an
opaque string, a slot is a name the consumer declares, and the words of a
template are the words the file holds.* `Cargo.toml`'s `description` field is the
same sentence compressed to one line. `docs/ARCHITECTURE.md` line 166 states it
from grove's side — *the whole of that is `crates/keyed-launch`, which has never
heard of a session* — and [`CONTEXT-MAP.md`](../../CONTEXT-MAP.md) lines 46–53
records the refusal of the word **session** as the naming decision that keeps the
crate from being a bounded context of its own.

| The stage | What it must not add or interpret | Chapter |
|---|---|---:|
| the manifest, the library root, the two error types | three dependencies and no domain; two opaque errors so neither half's caller handles the other's | 1 |
| the vocabulary and the compiled shapes | a rule is about a slot's *name*; the crate never learns what the name means | 2 |
| reading and validating two documents | a launch is one complete string read whole out of one file, never assembled from two | 3 |
| the template rules and their diagnostics | a word is a word; no shell ever sees the line, and a `#` is refused rather than obeyed | 4 |
| resolution, expansion and the argv | substitution is whole-word or nothing; the values fill the vocabulary, not this template | 5 |
| the completion channel | appearance is the event; the token's content is the caller's to read | 6 |
| the environment, the terminal, the spawn | no argument, no flag, no variable the operator did not write | 7 |
| the watch, the escalation and the launcher's own signals | it cannot know the child is done — only that the child said so | 8 |
| the conformance kit | a configuration is held to a contract without either side knowing what a key is for | 9 |

Three alternatives were rejected. **The two halves meet only at `Argv`**
(`src/lib.rs` lines 31–34) is the crate's true structural claim and is compiler-
enforced rather than tested — `Argv` has no public constructor — but it is one
boundary, so it splits the book in two rather than giving nine chapters nine
rules; it is stated in chapter 1 and proved in chapter 5, where the seam actually
is. **Checked whole before anything is spawned** is the configuration half's
thesis, and `src/run.rs` — 29% of the corpus — validates nothing; it is
chapter 3's. **A launch ends out of band** is the mirror of it, the launch half's
thesis and chapter 6's, leaving `src/templates.rs`'s 32% unserved.

## Chapter sequence

Ten pages, nine of which own source. `README.md`, `concept-index.md` and
`source-index.md` are the contents page and the two lookup surfaces; they are not
chapters and not alternate explanatory paths.

| Order | File | Page ID | Title | Slice |
|---:|---|---|---|---|
| 1 | `01-orientation.md` | `orientation` | Orientation | `understands-neither` |
| 2 | `02-the-names.md` | `the-names` | The names a template is written against | `rules-about-names` |
| 3 | `03-two-documents.md` | `two-documents` | Two documents, neither one assembled | `never-assembled` |
| 4 | `04-template-law.md` | `template-law` | What a template must be | `words-not-shell` |
| 5 | `05-to-an-argv.md` | `to-an-argv` | From a template to an argv | `whole-word-or-nothing` |
| 6 | `06-the-channel.md` | `the-channel` | Appearance is the event | `appearance-is-the-event` |
| 7 | `07-the-job.md` | `the-job` | The child is a job | `nothing-else-added` |
| 8 | `08-the-escalation.md` | `the-escalation` | The watch and the escalation | `the-launchers-job` |
| 9 | `09-how-checked.md` | `how-checked` | How this is checked | `checked-without-meaning` |
| 10 | `10-what-passes-through.md` | `what-passes-through` | What passes through | `assembly` |

`assembly` owns no production source and is therefore final-only: it has no
scoped prefix to prove. Both elicited precedents close the same way —
`jj-workspace`'s `07-what-jj-owns` and `grove-llm`'s `07-what-order-holds` each
carry slice `assembly` and own nothing — and each states its transferable test
there. Keeping chapter 9's 237 lines under the closing page would give it a
scoped prefix to prove as well as a book-wide argument to make.

**Slice IDs are named for the rule each chapter carries**, so the slice list
reads as the book's spine, and no slice token equals any page ID. That difference
is by construction, and it is the property the specification's separate slice
domain exists for: a presentation-level page rename must not become a
fragment-ownership, ledger, manifest and CLI migration. Slice IDs carry no Grove
task key, for the reason recorded in `jj-workspace-structure-k17`'s decision 8,
applied here rather than re-elicited.

**The pages follow the crate's own conceptual order, which is `src/lib.rs`'s
section order, not the module list.** `lib.rs`'s headed sections are *Two
documents, and what the second one may do*, *The vocabulary is an input to `load`,
not to `expand`*, *From a template to a running child*, the two bold paragraphs on
the job and the out-of-band ending, and *Testing a consumer's configuration* —
chapters 3, 2, 5, 7, 8 and 9 respectively. The order the book takes is that order
with the vocabulary moved ahead of the two documents, because the vocabulary is
what makes the documents checkable and `lib.rs` says so.

**Chapters 3, 4 and 5 divide one 670-line file by concept rather than by its own
order**, at the price of eight ownership blocks in that root. `templates.rs` is
ordered by Rust convention — the types, then the public `impl`, then the free
validation functions, then the diagnostic helpers — so `load` sits 130 lines from
the functions it calls and `expand` sits between them. Chapter 3 gathers `load`
with the reading and whole-document validation it drives; chapter 4 gathers every
per-node and per-template rule with the diagnostics they produce; chapter 5
gathers resolution and expansion with the `Argv` they author. `grove-llm`'s book
paid twenty-two blocks in one root for the same reason.

**Chapters 7 and 8 divide `src/run.rs` by whose signal it is.** Chapter 7 owns the
launch's shape and everything done *to the child* — its process group, its
terminal, its dispositions, its environment, the spawn. Chapter 8 owns everything
about *endings*: the supervisor's state machine, the escalation, and the
launcher's own SIGTERM latch, which is an ending the channel cannot express. The
cost is recorded in *Early uses the order forces*: `run` calls three of chapter
8's items.

## Concept and seam responsibilities

### `README.md` — reader contract

State the audience, scope, exclusions, source-authority rule, exact-fragment
claim, canonical page order, lookup paths, and the distinction between scoped and
final completeness. Explain how to recognise a fragment definition, insertion,
source root and deferred hole without duplicating the full grammar. State the
pass-through outcome. Cite [`USAGE.md`](../USAGE.md) at `usage-running-grove` —
the one guide link the contract permits and requires from this page, and the
section whose line 92 states this crate's eager-validation property from the
reader's side: *a missing or malformed `config.kdl` leaves your working tree
byte-identical*. State the book's boundary: it explains one crate, and it stops
at the words in the file and at the process it spawned.

### 1 · Orientation — understands neither

Owns `Cargo.toml`, `src/lib.rs` and `src/error.rs` whole. Responsible for: what
`keyed-launch` is and what it refuses to be — the crate that owns a launch's
shape and none of its meaning, with grove's mapping onto it stated **once**, in
one sentence, as *a session kind is a key*, and never used again; the three
dependencies and what each is for, read as the evidence of the claim — `kdl` and
`shell-words` are two document formats it reads, `libc` is the syscalls it cannot
reach from `std`, and there is nothing else; the manifest's opaque-error rule and
its statement that `jj-workspace` and `ordinal-fs-tree` hold the same one; `lib.rs`
as the book's map, its six headed theses named against the chapters that own
them; **the two halves and the seam** — that `Templates::expand` authors an
`Argv`, that `run` consumes one, that neither module names the other, and that
this is compile-enforced rather than kept by convention, stated here as the ground
chapter 5 stands on; the two error types read as the seam's evidence, since
`error.rs` lines 47–52 give *keeping the two halves usable apart* as the reason
there are two rather than one; the opacity argument — that a variant list would be
a second interface and every new diagnostic a breaking change — and the obligation
that replaces it: name what is wrong, name where, name what fixes it; and
`release = false` as an answered question rather than an open one.

This chapter states, in one sentence, how this book's spine differs from
`jj-workspace`'s. It carries the worked example at low resolution.

### 2 · The names a template is written against — rules about names

Owns `src/vocabulary.rs` whole and `src/templates.rs` lines 1–91. Responsible
for: why the vocabulary is an input to `load` and not to `expand`, which is the
chapter's whole argument and the one place the book takes a position a reader
could disagree with — every template rule is a rule about slot *names*, so a
loader that will not learn the names until expansion can check none of them, and
a vocabulary supplied per call would make every rule just-in-time; `Requirement`
as a cardinality with two cases and the two messages it owns; that `SlotRule`
names a slot bare and `${name}` is the spelling, and that the crate never learns
what the name means; and then the shapes the two later chapters compile into —
`SlotSpec` as the owned table, `Template` carrying **the file it was read from**
per key rather than once per configuration, `Word` as literal-or-slot-by-index,
`DocumentRole` and its statement that the rules do *not* differ by role,
`SourceLocation`, `ValidationDiagnostic` and `NodeValidation`. The per-key
`source` is the chapter's second argued claim: after an overlay resolves there is
no single answer, so a diagnostic naming a file has to name the one that actually
supplied the failing key.

### 3 · Two documents, neither one assembled — never assembled

Owns `src/templates.rs` lines 92–145 and 276–414. Responsible for:
`Templates::load` as the crate's one entry point for a configuration, and its
three promises — a key resolves only if the **primary** declares it, both
documents are validated whole against the vocabulary, and the load is
all-or-nothing in both halves; why an unreadable, unparseable or invalid overlay
fails the load rather than falling back to the very policy its owner was moving
away from; `compile_vocabulary`'s duplicate-name refusal and its reason — a
duplicated slot is counted twice against its own cardinality rule and takes
whichever value arrived first, *a consumer bug that looks like a template bug for
as long as it goes unnamed*; the asymmetry between `read_primary` and
`read_overlay`, which is the whole of what *the overlay is optional* means at the
filesystem; and `validate_document`'s aggregation — every diagnostic in both
documents, with locations, in one refusal, so one edit fixes the file rather than
uncovering the next problem.

This chapter carries the **second ending**: a `.grove.kdl` that declares a key
the personal file does not, refused by name, with the refusal naming the file
that must declare it.

### 4 · What a template must be — words not shell

Owns `src/templates.rs` lines 415–534, 535–617 and 618–660. Responsible for:
every rule a template must satisfy, each with the line that enforces it and the
diagnostic it produces — the node shape, the duplicate key reporting *every*
declaration location, word zero being a literal executable, a substitution
occupying a whole word, a declared slot's cardinality, and unmatched quotes;
the unquoted `#`, which is the chapter's sharpest case, because `shell-words`
treats it as a comment start and would silently truncate the argv, so the crate
scans for it and refuses rather than accepting a line that means less than it
says; `ShellWordScanState` as the reason that scan cannot be a `contains`;
`whole_substitution` as the mechanism of whole-word substitution and the place
`${a}${b}` is refused; and `render_diagnostics`, `format_location` and
`source_location` as the machinery behind *name what is wrong, name where, name
what fixes it* — the obligation chapter 1 stated and this chapter is the only one
that discharges.

### 5 · From a template to an argv — whole word or nothing

Owns `src/templates.rs` lines 146–275 and 661–670, and `src/argv.rs` whole.
Responsible for: `source`, and why `None` for a key the primary does not declare
whatever the overlay says; `require` as the obligation a consumer discharges
*before* it commits to a key, stated once so the refusal's wording has one owner;
`expand`'s single remaining obligation and why it is stated over the *vocabulary*
rather than over this template's own words — so a consumer cannot have a call
that works for one key and fails for its neighbour purely because the two
templates mention different optional slots; `match_values` as that check;
`unresolved` and its two wordings, which is where the untracked-delta rule is
actually kept; `keys` as the conformance kit's one window and nothing more; and
`Argv` — no constructor, no shell, `words()` for the callers that want the whole
launch as one list.

**The seam is proved here.** `Argv::new` is `pub(crate)` and `Templates::expand`
is its only caller, so *nothing reaches a spawn that a template did not author*
is a fact about the types. Chapter 1 stated it; this chapter shows the two lines
that make it true.

### 6 · Appearance is the event — the channel

Owns `src/channel.rs` lines 1–271. Responsible for: what the channel is and why a
launch needs one at all — an interactive child returns to its prompt when it
finishes rather than exiting, so its own exit is not the event anyone is waiting
for; **allocation picks a name and writes nothing**, which is what makes
*appearance* the event; the name grammar — a recognisable prefix plus 128 bits of
randomness — and why the prefix is named for `signal` and matches what grove's
driver already leaves behind; `DRAW_RETRY_LIMIT` as a bound rather than an
unbounded retry, because at 128 bits a collision means the randomness source is
not random; why `allocate` checks the directory rather than leaving it to the
child's first write; `Token` as opaque to this crate and readable to its caller,
and the line framing as framing rather than interpretation; `signal` as a free
function because the two ends are different processes; and `discard_abandoned`,
whose exactness is the point — a looser rule would let this crate's cleanup delete
a neighbouring file in a directory whose other contents belong to the consumer.

This is the chapter that cites [`CONTEXT.md`](../../CONTEXT.md) at
`loop-control-channel`, at its first use of *channel* for the thing grove's
glossary already names.

### 7 · The child is a job — nothing else added

Owns `src/run.rs` lines 1–123 and 244–448. Responsible for: `Escalation`'s two
waits and what each is for; `Launch` as *everything one launch is*, every field
the caller's, and `run`'s promise that nothing else is added — no argument, no
flag, no variable; the scrub list as the caller's obligation discharged here, and
why an environment is inherited rather than addressed, so a nested launcher would
otherwise hand a child a live channel path belonging to somebody else's launch;
`cwd` and why `None` is rarely what a launcher wants; `Ended` and `End`'s three
cases, and the distinction the chapter must make carefully — `Signalled` is
narrower than *a token appeared*, because a child that signals and exits inside
its own grace was never touched and comes back `Exited` with a token;
`DEFAULT_DISPOSITION_IN_CHILD` and its argument that **only an ignored disposition
survives `execve`**, so the list is about a launcher's own ignores rather than its
handlers; `Terminal::open` and why `/dev/tty` rather than stdin is the gate that
needs no flag; the spawn itself — the child's own process group, the terminal
handed over, and why a new *session* was rejected; and `POLL_INTERVAL` as not a
knob.

`End::Interrupted` is defined here and produced only in chapter 8; the chapter
names it and defers.

### 8 · The watch and the escalation — the launcher's job

Owns `src/run.rs` lines 124–243 and 449–607. Responsible for: `Watch` as the
supervisor's state machine; `watch`'s three observables and the honest statement
that they are the only three ways a launch ends — **a child that finishes its work
and never signals reaches none of them**, and the launch stalls rather than ends,
which the source names as a real failure mode with no cheap fix and the book must
not soften; why ending an interactive child is the *launcher's* job — it is the
child's parent, outside whatever sandbox the child runs under, and a child asked
to end itself may be denied silently; the escalation addressed to `-pgid` as well
as the pid, and what that reaps; `supervise` taking the terminal back only from
the job this launch owned, and why returning while the terminal belonged to a dead
group would be a SIGTTOU stop rather than an error anybody could read; `kill`'s
deliberately ignored failure as *the shell's `kill … 2>/dev/null`, written down*;
and then the launcher's own signals — `INTERRUPTED_BY` as process-global because a
disposition is, latched because the launch on which the child finally exits still
has to report it, carrying the *number* because a launcher that re-raises SIGTERM
for a SIGHUP has told its parent the wrong thing, and cleared immediately before
each spawn because **a latch that outlives its launch is a loaded gun**;
`take_interrupt` for the signal that arrives between launches; `reraise` and why
an exit code cannot express *was signalled* at all; and SIGINT's deliberate
absence from the handler.

### 9 · How this is checked — checked without meaning

Owns `src/channel.rs` lines 272–404 and `src/conformance.rs` whole. Responsible
for: the conformance kit as the **cross-crate seam** — a consumer's own suite can
only assert that its configuration works with its build, and the kit is what
holds a configuration to this crate's contract from outside the consumer; the
three obligations `check` applies in order, and why the third is not a second
spelling of `load` — expansion is the only place the compiled words are walked;
**why an empty document fails**, which is the chapter's best case for the spine: a
kit that only reports violations reads identically when handed nothing to check,
so a configuration declaring no keys is a failure in its own right, not because an
empty file is malformed but because a suite of must-hold claims cannot otherwise
detect that it did not run; and the nine inline channel tests, read as what a
`#[cfg(test)]` module inside a root buys that an integration test cannot — reaching
`is_channel_name`, a private function whose exactness chapter 6 argued and only
this module can pin.

The chapter states, once, why these nine tests are in the corpus and the 1,319
lines under `crates/keyed-launch/tests/` are not: a root is `src/**/*.rs`, and the
corpus exception inventory carries no `keyed-launch` row.

### 10 · What passes through — assembly

Owns no source. Responsible for: stating the pass-through test in the form under
*Audience and intended outcome*, and applying it to all nine source-owning
chapters — for each of the three places a layer learns a meaning it was never
given, which chapters proved the crate does not, and by which test. It closes on
the one thing the crate cannot do anything about: a child that finishes and never
signals, which no observable here can distinguish from one still working, and
which is the caller's to close at the layer that instructs the child.

## The mapping onto the corpus

### Top-level ownership blocks

| Root | Lines | What the block is | Chapter |
|---|---:|---|---:|
| `Cargo.toml` | 1–47 | whole | 1 |
| `src/lib.rs` | 1–68 | whole | 1 |
| `src/error.rs` | 1–81 | whole | 1 |
| `src/vocabulary.rs` | 1–44 | whole | 2 |
| `src/templates.rs` | 1–91 | imports and the seven types | 2 |
| `src/templates.rs` | 92–145 | the `impl Templates` opening and `load` | 3 |
| `src/templates.rs` | 146–275 | `source`, `require`, `expand`, `match_values`, `declared_slots`, `unresolved` | 5 |
| `src/templates.rs` | 276–414 | `compile_vocabulary`, `read_primary`, `read_overlay`, `parse_and_validate`, `validate_document` | 3 |
| `src/templates.rs` | 415–534 | `validate_node`, `validate_template` | 4 |
| `src/templates.rs` | 535–617 | `ShellWordScanState`, `contains_shell_comment_start`, `parse_template_word`, `whole_substitution` | 4 |
| `src/templates.rs` | 618–660 | `at_node`, `at_template`, `render_diagnostics`, `format_location`, `source_location` | 4 |
| `src/templates.rs` | 661–670 | `keys` | 5 |
| `src/argv.rs` | 1–48 | whole | 5 |
| `src/channel.rs` | 1–271 | header, imports, the three constants, `Channel`, `Token`, `signal`, four helpers | 6 |
| `src/channel.rs` | 272–404 | the inline `#[cfg(test)] mod tests` | 9 |
| `src/run.rs` | 1–123 | header, imports, `POLL_INTERVAL`, `Escalation`, `Launch`, `Ended`, `End` | 7 |
| `src/run.rs` | 124–243 | `Watch`, `INTERRUPTED_BY`, `take_interrupt`, `reraise`, `on_terminate`, `install_termination_handler` | 8 |
| `src/run.rs` | 244–448 | `DEFAULT_DISPOSITION_IN_CHILD`, `Terminal`, `own_group`, `run` | 7 |
| `src/run.rs` | 449–607 | `supervise`, `watch`, `kill` | 8 |
| `src/conformance.rs` | 1–104 | whole | 9 |

### Where a file's concerns split across chapters

Three roots split, and each split is the concept order disagreeing with the
file's own.

**`src/templates.rs` splits four ways across chapters 2, 3, 4 and 5**, in eight
blocks. The file is types, then the public `impl`, then the free validation
functions, then the diagnostic helpers; the book is shapes, then loading, then
the rules, then expansion. The two interleave: chapter 3 owns lines 92–145 and
276–414, and chapter 5 owns the 146–275 that sits between them.

**`src/run.rs` splits two ways across chapters 7 and 8**, in four blocks, by
whose signal it is. Chapter 7 owns 1–123 and 244–448; chapter 8 owns the 124–243
between them, and 449–607 after.

**`src/channel.rs` splits at line 272**, the `#[cfg(test)]` attribute, with
production in chapter 6 and the inline module in chapter 9. This is the only
split in the book made at a `cfg` boundary rather than a conceptual one, and the
reason is that the module's subject is *assurance*, which is chapter 9's, while
its subject matter is chapter 6's — so chapter 9 explains the tests against
chapter 6's fragments, which are behind it.

### Owned-source totals

| Chapter | Lines | Share |
|---:|---:|---:|
| 1 · Orientation | 196 | 9% |
| 2 · The names | 135 | 7% |
| 3 · Two documents | 193 | 9% |
| 4 · Template law | 246 | 12% |
| 5 · To an argv | 188 | 9% |
| 6 · The channel | 271 | 13% |
| 7 · The job | 328 | 16% |
| 8 · The escalation | 279 | 13% |
| 9 · How this is checked | 237 | 11% |
| 10 · What passes through | 0 | — |
| **total** | **2,073** | **100%** |

Per root: `Cargo.toml` 47, `src/lib.rs` 68, `src/vocabulary.rs` 44,
`src/argv.rs` 48, `src/error.rs` 81, `src/conformance.rs` 104,
`src/channel.rs` 404, `src/run.rs` 607, `src/templates.rs` 670.

## What each chapter's prose owes

Measured before deciding: **33% of the corpus is comment prose** — 693 of 2,073
lines — but it is not spread evenly, and the unevenness is what the book has to
answer. `src/lib.rs` is 76%, `Cargo.toml` 53% and `src/run.rs` 53% (324 of 607
lines), and in `run.rs` that prose is *argument*: the escalation, the child
dispositions, the interrupt latch and the terminal each carry a full case in
situ. `src/templates.rs` is **13%** — 89 lines over 670 — and it is where every
rule the decision records state actually binds. Three things are what each
chapter's prose owes, and a technical review checks for them:

1. **Adjudicate the claim.** For every argued claim, name the behaviour it rests
   on, the test that proves it, and the alternative rejected with what it would
   have cost. A doc comment cannot cite an integration test in another file, and
   this crate's evidence is 1,319 lines against a 2,073-line corpus.
2. **Carry the through-line.** Show where a decision in one root is only
   explicable by a decision in another: the vocabulary being an input to `load`
   is *why* `Templates` keeps an owned `SlotSpec` table, which is *why*
   `Word::Slot` is an index rather than a name, which is *why* expansion's only
   obligation is over the vocabulary and not this template; `Channel::allocate`
   writing nothing is *why* `watch` can treat appearance as an event at all; and
   the two error types are *why* the two halves can be described without either
   naming the other. No single comment holds any of those.
3. **Carry the load where the source does not, and only there.** The instruction
   differs by chapter and a reviewer checks the right one. **Chapters 3, 4 and 5
   supply the argument**: for each template rule, the line that enforces it, the
   diagnostic it produces, and the record clause it keeps — because the source
   states almost none of that, and the rules are the reason the records exist.
   **Chapters 7 and 8 do not restate**: the comments already argue, the fragment
   graph quotes them verbatim on the page, and prose that paraphrases an argument
   the reader has just read in the source is the failure mode this obligation
   exists to name. There, the prose connects the arguments across items and names
   the test, and does nothing else.

The third is this book's own. The risk it closes is the reverse of `grove-llm`'s:
not paraphrase across three documents describing the same verbs, but a book that
pads where the source is strong and thins where it is silent. Rejected: **hold the
record to the code** — for each clause of decision 7 and the three ADRs, the line
that keeps it and the test that catches its breach — which is a real obligation
but is roughly half-discharged already by `run.rs`'s own comments, and which the
first two obligations reach for the half that is not; **argue the seam end to
end**, which is one argument made once rather than a per-chapter obligation a
reviewer can check page by page, and which is therefore chapter 1's responsibility
and chapter 5's proof instead; and **no third obligation**, which leaves the
measured 13%/53% asymmetry unaddressed in a corpus where it is the dominant fact.

## Worked examples

**The carried example is grove's own configuration, told strictly from the
crate's side.** The reader knows exactly what the words mean and watches the
crate not care, which is the spine made visible; an invented consumer would leave
nothing for the crate to refuse. The starting values are real and published:

- the primary document, two keys, from [`CONFIGURATION.md`](../CONFIGURATION.md):
  `impl "claude --model opus ${prompt}"` and
  `review-impl "codex exec --model gpt-5 ${prompt}"`;
- an overlay that declares `impl` and nothing else;
- the vocabulary `prompt` (`ExactlyOnce`) with `session_name`, `worktree` and
  `repo` (`AtMostOnce`), which is grove's real four-slot set;
- the channel variable `GROVE_SIGNAL_FILE`.

| Chapter | Anchor | Starts at | Observable end |
|---:|---|---|---|
| 1 | `#the-launch-in-outline` | the two lines on disk | a running child and a token, named but not traced |
| 2 | `#the-four-slots` | the vocabulary value | `${prompt}` is required and why that is checkable only here |
| 3 | `#both-documents` | `load` with a primary and an overlay | `impl` resolves from the overlay, `review-impl` from the primary, each naming its own file |
| 4 | `#the-rules-on-one-line` | the `impl` template as text | the compiled `Vec<Word>`; and the same line with an unquoted `#`, refused with its location |
| 5 | `#four-words` | `expand("impl", …)` | `["claude", "--model", "opus", "<the prompt>"]` — the prompt one argument however many spaces it holds |
| 6 | `#a-path-and-nothing-else` | `Channel::allocate` in the control directory | a path that does not exist, and a token read back after `signal` |
| 7 | `#the-spawn` | `run` with that argv and channel | the child in its own group, holding the terminal, with `GROVE_SIGNAL_FILE` set and the scrub applied |
| 8 | `#the-two-graces` | the token appearing | grace → SIGTERM → kill-grace → SIGKILL; and the launcher's own SIGTERM as `End::Interrupted` |
| 9 | `#checking-the-same-file` | `conformance::check` over that primary | an empty document failing, and a violated rule reported |
| 10 | `#where-does-it-learn` | the three parts of the test | each part answered against the chapters that proved it |

**The second ending is the overlay-only refusal**, carried in chapter 3: an
overlay declaring a key the primary does not, where the key does not resolve and
the refusal names the key and the primary file that must declare it. It is the
second ending because it is the property that stands between an untracked file a
project ships and a program its operator never chose, and
`a_key_only_the_overlay_declares_does_not_resolve` pins it. Rejected: a refusal
as the *carried* example, which both precedents found makes the happy path the
aside; and the crate's own test fixtures as the carry, which are real and pinned
but use three different vocabularies — `prompt`/`label`, `script`, and the kit's —
because the halves are separately usable, so no single thread runs through them.

Concrete values for the prompt string, the paths and the token are the
orientation slice's to fix, as all three precedents left them.

## Early uses the order forces

Two rows, both the price of concept order over file order, and both recorded in
the book's early-use ledger.

| Term | First used | Owned by | Why the order forces it |
|---|---:|---:|---|
| `validate_node`, `validate_template` | 3 | 4 | `validate_document` drives both, and it belongs with `load` |
| `install_termination_handler`, `INTERRUPTED_BY`, `supervise` | 7 | 8 | `run` installs the handler, clears the latch and calls the supervisor as its first and last acts |

Chapter 1 additionally names nearly every public type before its owner explains
it — `Vocabulary`, `Templates`, `Argv`, `Channel`, `Token`, `signal`,
`Escalation`, `run`, `conformance::check` — because `src/lib.rs` is the crate's
own map and reproducing it is what chapter 1 is for. That is one block naming the
cast at low resolution, not a per-term forward reference, and it is the same
shape `jj-workspace`'s and `grove-llm`'s orientation chapters took.

## Outbound links

```toml
[guide]
path    = "docs/USAGE.md"
anchors = ["usage-running-grove", "usage-session-lifecycle"]

[[glossary]]
path    = "CONTEXT.md"
anchors = ["loop-control-channel"]
```

Three anchors, **all of which exist today** as explicit `<a id="…"></a>` lines
preceding their headings, so `book-check`'s `M201` is green from the first slice
and no work is owed outside the book on their account. `grove-llm`'s book
reserved two anchors that did not yet exist and its book leaf carried the
promotions; this one carries none.

`usage-running-grove` is the `README.md`'s required guide citation, because
[`USAGE.md`](../USAGE.md) line 92 — *full configuration validation precedes every
one of those tree mutations, so a missing or malformed `config.kdl` leaves your
working tree byte-identical* — is the one place the guide and this book state the
same fact, from the two sides. `usage-session-lifecycle` says what a launch is
*for*, once, so no chapter has to. `loop-control-channel` is cited at chapter 6's
first use of *channel*.

Rejected: `usage-review-composition`, a false friend whose *escalation* is the
methodology's review escalation and not the kill escalation; `usage-driver-lease`
and the glossary's `session-epoch` and `driver-lease`, which describe what grove
does with a channel path rather than anything this crate knows; a promoted
`session-kind` anchor, which chapter 1's one-sentence mapping does not need badly
enough to buy an edit outside the book; and `[guide] omitted`, which
[`CONTEXT-MAP.md`](../../CONTEXT-MAP.md) lines 46–47 rule out — it holds
`keyed-launch` as **not** a context of its own, explicitly unlike
`ordinal-fs-tree`, which is the only book granted that exemption and is granted it
on exactly that ground.

**The records this crate is governed by are named and never cited.** Decision 7
of [`module-decomposition.md`](module-decomposition.md),
[*complete session configuration*](../adr/complete-session-configuration.md),
[*the untracked configuration delta*](../adr/untracked-configuration-delta.md)
and [*the launched child is a job*](../adr/the-launched-child-is-a-job.md) are
where this crate's decisions live, and none of them is a permitted link target
from a book page — the contract closes a book's local targets to its own pages,
its own roots, the guide and the glossary. The book states what each record
settles, in prose, at the chapter that keeps it, and links none of them. That is
the contract working as designed rather than a gap: a book is self-contained for
its claims.

## The book's row in the ownership table

`docs/ARCHITECTURE.md`'s *Documentation ownership* table has no `keyed-launch`
row today, and `every_book_root_has_a_documentation_ownership_row`
(`crates/grove/tests/reference_navigation.rs`) is red until it does. Adding it is
`keyed-launch-book-k35`'s, as each precedent book's row was its own book leaf's.
The row follows the four existing ones in form:

```text
| `keyed-launch` source, read page by page | [`walkthroughs/keyed-launch/README.md`](walkthroughs/keyed-launch/README.md) — the code walkthrough: a book whose every chapter opens on what the crate must not add and must not interpret, and whose fragments reconstruct every byte of the crate's frozen corpus |
```

## What this book absorbs

`docs/ARCHITECTURE.md` line 1188 carries `residue(grove-loop, keyed-launch)` over
the passage describing the watch and the escalation and what the driver does
without a signal. Chapters 6, 7 and 8 make the descriptive half of it redundant.

**The deletion is joint and is not this book's to make.** The marker names two
books, so `architecture-residue-k75` can act on that passage only once the
`grove-loop` book has also landed; and the sandbox ground and the
does-not-infer-`done` decision stay in `ARCHITECTURE.md` either way, as the marker
itself says. This book neither edits nor cites that document. No other residue
marker names `keyed-launch`.

## What the book deliberately does not cover

### KDL, and shell-word splitting, as formats

`kdl` 4.7 parses the document and `shell-words` splits the line. The book explains
what the crate asks of each and what it adds on top — the node shape it requires,
the comment-start scan it performs because `shell-words` would silently truncate —
and never the crates themselves. A reader wanting KDL's grammar is reading the
wrong book.

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
question. That it is closed rather than unexamined is worth one sentence in
chapter 7 and no more.

### Everything grove does with a launch

What a session is, what a kind means, the driver lease, the session epoch, the
task tree, the methodology. Chapter 1 states the mapping — a session kind is a
key — in one sentence, and the book's whole spine is that the crate does not know
the rest. A book that explains grove's sessions has documented the wrong crate.
[`USAGE.md`](../USAGE.md) at `usage-session-lifecycle` is where that account
lives, and the `README.md` points there once.

### `docs/CONFIGURATION.md`'s account of the schema

That document is the human's reference for writing a `config.kdl`: what the file
must contain, which slots exist, what a delta may do. This book explains the code
that reads it. The two describe the same rules from opposite sides and neither
is a substitute; the book does not restate the schema and does not link the
document, which the contract does not permit as a target.

### The `tests/` directory

`crates/keyed-launch/tests/` is 1,319 lines across five files — 64% of the
corpus's own size — and every one of them is evidence rather than a root. The book
names a test whenever it adjudicates a claim, and reproduces none of them. The
nine tests it *does* reproduce are the inline module inside `src/channel.rs`,
which are corpus because a root is `src/**/*.rs`.

### The four decision records

Named at the chapters that keep them, never cited, for the link-contract reason
under *Outbound links*. The book does not reproduce their reasoning: it shows the
lines that keep them and says which record each line answers to.

## Known in advance: what this book does not adjudicate

**Decision 7 of [`module-decomposition.md`](module-decomposition.md) carries a
stale interface sketch, and this book cannot fix it or cite it.** The sketch omits
`Launch::cwd`, `End::Interrupted`'s payload, `Templates::require` and
`Templates::keys`, `Channel::discard_abandoned`, `Argv::words`, and the free
functions `take_interrupt` and `reraise` — six drifts, of which the last is
substantive: those two functions are the whole of a looping launcher's obligation
for a signal arriving between launches, and a sketch without them describes a
crate that cannot report that ending.

This is not a corpus defect. The stale text is in a specification, not in a source
root, so the freeze is not engaged, and the book will reproduce the crate's real
surface without ever contradicting a document it may not link. Neither chapter 1
nor chapter 5 adjudicates it at a fragment, because there is no fragment to
adjudicate it against. It is `runner-sketch-drift-k106`, cut at this leaf and
placed ahead of `architecture-residue-k75`, and the book does not wait on it.

**No claim inside the corpus was found stale.** The manifest's cross-reference to
`crates/jj-workspace` and `crates/ordinal-fs-tree` holds, `release = false` reads
correctly against [`RELEASING.md`](../RELEASING.md), and every doc comment checked
against the code it sits on agreed with it. Unlike `grove-llm`'s chapter 1 and
chapter 2, no chapter here carries an adjudication obligation.

## What this brief does not settle

The prose itself, the figures, the concept-index and source-index entries, the
fragment identifiers, the exact block decomposition *inside* each top-level
ownership block, and the concrete values the orientation slice fixes for the
carried example. Those are the book's, through the pipeline. This brief settles
the shape.
