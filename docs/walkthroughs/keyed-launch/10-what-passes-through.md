# What passes through
<!-- book-page id="what-passes-through" slice="assembly" order="10" -->
[Previous: How this is checked](09-how-checked.md) | [Contents](README.md)

<a id="assembly"></a>
## Nine openings, one question

<!-- rollup «owned-lines-total» -->
This chapter owns no production source. The 11 roots and 3,399 lines are
already reconstructed by the fragment graph the nine chapters before it built,
and the [source index](source-index.md) records that graph in full. What is left
is the thing no single chapter could state, because each one opened on its own
refusal and stopped at the boundary of the lines it owned.

Every chapter opened the same way: *what this stage must not add and must not
interpret is X*. Read one at a time, each of those is a local argument about a
few dozen lines — a manifest with no domain dependency, a slot table keyed by
position, a scan for a character a splitter would otherwise have obeyed. Read
together, they are nine answers to the question that this chapter applies:

> **Where does this layer learn what the value means?**

The answer this crate gives is *nowhere*, and that answer must be checked rather
than assumed. A layer that carries a value from a human's
file to an effect in the world has three chances to learn a meaning it was never
given, and the chapters divide across them unevenly. The table is the division,
and the third column is the one to read down: it is where a reader looking for
the same property in their own code should expect to find it missing.

| Ch | What the stage carries | The meaning available to be learned | Where it would enter |
|---:|---|---|---|
| 1 | the manifest, the library root, the two error types | what a key names; what a template's program does | the claim, not yet an arm |
| 2 | the vocabulary and the compiled shapes | what a slot name refers to | on the way in |
| 3 | reading and whole-document validation | which parts of a launch came from which file | on the way in |
| 4 | the template rules and their diagnostics | what a word does — a pipe, a redirect, a `#` | on the way through |
| 5 | resolution, expansion and the argv | what a substituted value contains | on the way through |
| 6 | the completion channel | what the token says | on the way out |
| 7 | the environment, the terminal, the spawn | what the child needs that the operator did not write | on the way out |
| 8 | the watch, the escalation and the launcher's signals | whether the child is done | on the way out |
| 9 | the conformance kit, and the channel's inline tests | what *correct* means | all three, from outside |

Two rows are not arms of the test.
[Chapter 1](01-orientation.md#understands-neither) states the claim and proves
nothing: the manifest buys three dependencies and no domain, the library root
says a key is an opaque string, and the two error types keep either half's caller
from handling the other's failures. That is where the reader is told what to
watch for. [Chapter 9](09-how-checked.md#checked-without-meaning) is the other
end — it holds a configuration to the crate's obligations from outside the
consumer, and every obligation it applies is about a document's shape rather than
about what any key in it is for. The seven rows between them are where the answer
is earned.

The division is also uneven in a way the crate cannot fix, and this page ends on
it. Two of the three arms are closed before any process exists — by what `load`
builds out of a complete document and by what `expand` refuses to do to a value
afterwards. The third runs while a child is alive, and one case in it has no
answer at this layer at all.

<a id="where-does-it-learn"></a>
## Where does it learn what the value means?

The test has three parts and one question. Take any layer in your own code that
carries a value from a human's file to an effect in the world — a configuration
to a process, a query to an engine, a route to a handler — and ask where it
learns what the value means. The three parts below are the three places such a
layer usually learns it anyway; each has a cost, and each has a shape you can
look for. What follows is each part answered against the chapters that proved it
here, with the carried example — grove's own two-line configuration, its four
slots and its channel variable — as the material.

The starting point is the same in all three: the two lines
[chapter 1 put on the page](01-orientation.md#the-launch-in-outline).

```text
impl "claude --model opus ${prompt}"
review-impl "codex exec --model gpt-5 ${prompt}"
```

A reader of this book knows what `claude`, `--model opus` and a mandate are. The
crate does not, and the three parts below are the three moments at which it could
have found out.

### 1 · On the way in — assembling one value out of more than one source

**The move.** The layer builds one effective value by combining pieces from
several places: a default, a file, an environment variable, a flag, a
site-specific overlay. To combine them it must know what the parts *are* —
which fields merge, which override, which concatenate — and that knowledge is a
domain model it acquired without anyone deciding it should have one.

**The cost.** Nobody can see the whole of a value in one place, and no single
author owns it. The file an operator reads is not the value the program runs,
and the difference is spread across a precedence rule nobody wrote down as a
document.

**What the crate makes explicit.** Flat templates remain whole replacements.
Named reuse adds two references: a route names a binding, and a binding names a
personal command definition. The local file may replace either target, while
primary policy still authorizes every key. [Chapter 3](03-two-documents.md#named-fold)
shows the fold and its provenance. It interprets configuration names, not harness
flags: the program and its arguments still belong to the command author.
`Templates::source` names only the template text's file; inspection carries the
route and binding sources as well. The generic grammar makes the precedence
visible without adding a model, vendor or approval-policy schema.

The second document exposes this arm's asymmetry. grove's overlay is a
project-local file a repository can ship, and the property that stands between
it and *a program the operator
never chose* is that the overlay may override a key and may not introduce one.
[Chapter 3's second ending](03-two-documents.md#the-second-ending) is that
refusal: a key only the overlay declares does not resolve, and the message names
the overlay it was found in and the primary that would have to declare it.

Two tests in `crates/keyed-launch/tests/templates.rs` hold the arm.
`an_overlay_replaces_a_whole_template_and_reports_its_own_path` loads a primary
declaring `one` and `two` and an overlay declaring `two`, then asserts that `one`
expands from the primary's words, `two` from the overlay's, and that
`templates.source` returns each key's own file — the whole-template replacement
and the single-owner property in one case.
`a_key_only_the_overlay_declares_does_not_resolve` loads a primary declaring only
`one` beside an overlay declaring `two`, and asserts three things: that
`source("two")` is `None`, that `require("two")` refuses with the overlay's path
and an instruction naming the primary, and that `expand` refuses through the same
sentence. A refusal that named only the key would have left an operator hunting
two files.

[Chapter 2](02-the-names.md#an-input-to-load) is the same arm one level down, and
it is the reason chapter 3's checks can run at all. The names a template is
written against arrive as a parameter of `Templates::load` rather than of
`Templates::expand`, so a document can be checked whole before anything is
spawned. That ordering is a position the crate takes and the chapter argues
rather than asserts; what matters here is its consequence, which is that the
vocabulary is one source with one author — the consumer — and no rule in the
crate consults anything ambient to decide what `prompt` is.

### 2 · On the way through — re-reading a value it has already read

**The move.** The layer reads a value, and then reads it again in a different
grammar: it splits on whitespace, interpolates a `$`, honours a quote, expands a
glob, or passes the whole line to something that will. Each second reading is a
grammar the value was not written in, applied by a layer that has no way to know
whether the author meant it.

**The cost.** These failures are silent and execute more or less than the
operator wrote. A value with a space becomes two
arguments. A `#` truncates the line and the program runs with fewer arguments
than the operator wrote. A `$(…)` becomes a command.

**What the crate does instead.** A template is split into words exactly once, at
load, by POSIX quoting rules, and never again.
[Chapter 4](04-template-law.md#words-not-shell) owns the split and every rule
that binds at it; [chapter 5](05-to-an-argv.md#whole-word-or-nothing) owns what
happens afterwards, which is nothing. No variable is expanded, no `$(…)` is run,
no glob is matched, no redirection or pipeline is honoured, no `~` is a home
directory, and no shell is anywhere in the path. The crate's own `${name}` is not
an exception: it resolves against the slot table the consumer declared, and
[a substitution may only ever stand for a whole word](04-template-law.md#a-whole-word),
so it can never move the boundary between one argument and the next.

The structural half of the answer is in the compiled shape, and
[chapter 2 read the line](02-the-names.md#a-word-and-a-role) that makes it
structural: a compiled word is either a literal or `CompiledWord::Slot(String)`
with a validated runtime name. Expansion pushes either the literal or that
name's native offered value, and there is no
line in it that can put two words together or take one apart. The value the
caller handed in is moved, not read.

Three tests in the same file hold the arm, and a fourth guards what the third of
them costs. They divide the way the chapters do.
`a_slot_value_is_one_argument_whatever_it_contains` expands
`wrapper --flag 'a b' ${prompt}` with the prompt `two words $(not a command)` and
asserts four words back, the last of them that string entire: the spaces did not
split it and the `$(…)` was not run. `shell_metacharacters_stay_literal` expands
`wrapper '|' '>' ${prompt}` and asserts `|` and `>` arrive as ordinary arguments
— nothing was handed to a shell, so a pipe is a word.

The third is the case where a dependency adds meaning even though the crate
declines to interpret the value itself.
[Chapter 4's comment-start scan](04-template-law.md#the-comment-start) exists
because `shell-words`, the splitter this crate depends on, *does* interpret an
unquoted `#` as starting a comment and drops the rest of the line — silently, and
legally, because that is what a shell does. A crate that merely refrained from
adding a meaning would have inherited that one from a dependency. So it scans the
line for the character before the split and refuses the template rather than
compiling an argv that means less than the operator wrote.
`an_unquoted_hash_is_refused_rather_than_truncating_the_argv` loads
`one "wrapper # ${prompt}"` and requires the refusal to say
`` `#` starts a comment in a command template ``, and
`quoted_and_midword_hashes_stay_literal` holds the other side, so the rule does
not cost an operator a `#tag` or a `mid#word` that no shell would have eaten
either.

That asymmetry is worth carrying away, because it is the part of this arm that
does not follow from restraint. **A layer inherits every re-reading its
dependencies perform.** Declining to interpret is a property of the whole path a
value takes, not of the code you wrote, and the only way to know which grammar
your dependency applies is to go and read what it does with the characters your
users are allowed to type.

### 3 · On the way out — inferring what came back, or adding what was not written

**The move.** The layer reaches a conclusion the value did not carry. It decides
from a status, a timeout or a silence that the work is done; or it adds to the
call something the operator did not write, because the addition looks like
helpfulness — a flag because the child is non-interactive, a variable because the
child seemed to want one, a directory because none was given.

**The cost.** A launcher that decides for itself that a child is done will
sometimes be wrong while looking exactly as if it were right; and a value the
operator cannot see in their own configuration file is a value they cannot change
by editing it.

**What the crate does instead**, in the *adding* direction, is
[chapter 7](07-the-job.md#nothing-else-added): `run` hands the child one
environment variable holding one path, and that is the whole of what it adds. The
argv is the one a template authored, the working directory is the caller's, and
the scrub list removes launch-control variables a nested launcher must not
inherit rather than installing any.
`a_scrubbed_variable_is_removed_from_an_inherited_environment` is the test, and
its shape is the claim: it scrubs `HOME` from a child that inherits an ordinary
environment, then asserts through the channel that `HOME` came back `<unset>`
while `PATH` came back equal to the launcher's own. A scrub that merely emptied a
variable, or an inheritance that quietly rebuilt one, fails a different assertion
in the same test. `arguments_reach_the_child_as_written` holds the other
direction over a whole trip: it expands `sh ${script} 'one two  three'`, asserts
the argv is two arguments with the second arriving with its interior spacing
intact, spawns it, and asserts the child reported that same string back. The
template authored it, and nothing between the file and the process re-read it.

The compile-time half of the same answer is chapter 5's, and it is the one place
in the book where a refusal is held by a type instead of by a rule. `Argv` has
private fields and a `pub(crate)` constructor, so
[nothing can hand `run` words a template did not author](05-to-an-argv.md#no-constructor):
`run` takes an `Argv` and nothing else that could name a program. Chapter 1
claimed that seam and chapter 5 proved it, and this is the answer it was being
saved for — the *adding* direction closed before the program is written rather
than checked after it runs.

In the *inferring* direction the answer is [chapter 6](06-the-channel.md#the-thesis).
The crate does not decide that a child is finished: it allocates a path, hands
that path to the child, and waits for the path to exist. `Channel::allocate`
writes nothing, which is why the file's later existence is unambiguous evidence
that something else wrote it, and why appearance alone can be the event. What the
child wrote is read back once and handed to the caller as an opaque `Token`. The
crate's only interest in the bytes is
[whether there are any](06-the-channel.md#three-ways-to-have-no-token): `read`
trims trailing framing and reports nothing for an empty file, which is a presence
test and not a semantic interpretation. `relaunch` is grove's word, and this book
has carried it through nine chapters without the crate ever having read it *as*
a word. The
string does occur once inside the corpus — `src/channel.rs` line 357, where the
inline module writes it to a channel and reads it back — and that occurrence is
the point rather than an exception to it: even the crate's own test uses the word
as a value to move, and would pass just as well with any other.

[Chapter 8](08-the-escalation.md#three-observables) is where the restriction
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

The three arms, collected. This is the table to carry to a layer of your own:
the second column is the move to look for, the third is what it costs when you
find it, and the fourth is one shape that closes the arm — and the third row is
the one the next section qualifies.

| The arm | The move | The cost | What this crate does instead |
|---|---|---|---|
| 1 · on the way in | one effective value assembled from a default, a file, a variable, a flag, an overlay — which needs to know which parts merge, override or concatenate | nobody can see the whole of a value in one place, and the precedence rule is a document nobody wrote | explicit binding and route targets resolve to a complete command, and the overlay cannot authorize a new key ([chapter 3](03-two-documents.md#never-assembled)) |
| 2 · on the way through | the value read a second time in a grammar it was not written in — a split, a `$`, a quote, a glob, or a shell asked to do the reading | the failures are silent, and the program runs more or less than the operator wrote | split into words exactly once, at load, and never again; a substitution is a whole word or nothing, and a compiled slot names one validated runtime value ([chapter 4](04-template-law.md#words-not-shell), [chapter 5](05-to-an-argv.md#whole-word-or-nothing)) |
| 3 · on the way out | a conclusion the value did not carry — that the work is done, or that the launch wants something the operator did not write | a launcher that decides a child is done is sometimes wrong while looking exactly right, and a value not in the file cannot be changed by editing it | one variable holding one path is the whole of what is added and `Argv` has no public constructor; the appearance of a file the crate never wrote is the only completion event ([chapter 6](06-the-channel.md#the-thesis), [chapter 7](07-the-job.md#nothing-else-added), [chapter 8](08-the-escalation.md#three-observables)) |

<a id="the-one-that-stays-open"></a>
## The one the crate cannot close

The third arm has a limitation that the first two do not.
[Chapter 8 states it in its second paragraph](08-the-escalation.md#the-launchers-job),
and [chapter 7 reproduces it inside `run`'s own doc comment](07-the-job.md#the-child-is-a-job),
where it is part of the function's stated contract and, as that section says
itself, a paragraph that belongs to chapter 8:
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
already lives, and refuse to let the layer guess. Chapter 8 does the first,
grove's instruction does the second, and the three observables do the third.

<a id="taking-the-test-away"></a>
## Taking the test to a layer of your own

The transferable result is the question, which applies to any layer that sits
between a file somebody edits and
something that happens. Four steps, and the third is where the answer usually
turns out to be *somewhere after all*.

1. **Name the value and the two ends of its trip.** Not *"the config"* — *"the
   string in `impl` becomes the argv of a process"*. A trip you cannot state as a
   start and an end has no *through* for anything to be learned in.
2. **Find every point where the value is read, and count the grammars.** One
   reading is a parse. Two readings in different grammars is where this arm's
   failures live, and the second reading is often not in your code: a splitter, a
   templating layer, a shell invoked for convenience, a serialiser that
   round-trips. The `#` in [chapter 4](04-template-law.md#the-comment-start) is a
   dependency's second reading, and it took a scan to refuse rather than a
   decision to abstain.
3. **Ask what your layer would have to know for each of its own rules to be
   correct.** This is the one that finds the meaning. A merge rule has to know
   which fields are lists. A retry has to know which operations are idempotent. A
   default has to know what the absence of a value meant. Each of those is a fact
   about the caller's domain living inside a layer that will not be told when the
   domain changes — and none of them looks like a domain model at the point where
   it is written.
4. **Ask what ends the trip, and who decides.** If the answer is *the layer
   decides* — from a status, a timeout, a silence — you have this book's third
   arm, and the question is whether the thing being decided is a fact the layer
   can observe or a conclusion it is drawing. `Channel::allocate` writing nothing
   is what turns the observation into a fact; the stall above is what is left when
   even that is not enough.

Run those four over `keyed-launch` and you get the table this page opened with.
**Almost all of it is closed at load**, before a process exists: the merge that
happens once, the substitution that is whole-word or nothing. Cheap checks, run
early, over a document that is complete — which is what makes them checkable by a
conformance kit a consumer can run in its own suite. And **the one answer above
that a type holds cannot decay**, which the others can: a rule is a line somebody
can edit, but `Argv`'s missing constructor fails a build. The crate's own launch
suite is that seam seen from the other side — it reaches `run` through
`Templates::expand` because there is no other route, and its module comment says
it does not want one. Where a property can be moved from a rule to a type, a
breach becomes a compile failure rather than a condition every caller must
remember to check.

The last question the test will not answer is whether the property justifies its
cost. That is a judgement about what the layer is for. A crate that understands
neither half of the pair it carries can serve a consumer with a different domain
unchanged,
and grove's entire presence in this book is one sentence — a session kind is a
key. The corpus does name grove, in four of its ten files and in four kinds of
place, and not one of them is a thing the code knows: the comment over the
release block, which is about how the package ships; `src/channel.rs`'s note on
whose driver leaves files in a control directory; a shell line in an example; and
`src/conformance.rs`'s claim to keep *reusable outside grove* true. Provenance,
packaging and illustration — never a branch, a constant or a name the crate acts
on. A layer that learned what a key meant would require domain-specific changes
for consumers with different keys.

<a id="the-closed-ledgers"></a>
## The closed ledgers

The book's two ledgers are complete, and the closure is mechanical rather than a
claim this chapter makes.

<!-- rollup «ownership-blocks» -->
<!-- rollup «source-roots» -->
<!-- rollup «ownership-blocks-owned-by» of="understands-neither" -->
<!-- rollup «ownership-blocks-not-owned-by» of="understands-neither" -->
**Ownership.** 27 top-level blocks over 11 source roots, every one
`resolved`. The table is the source index's
[ownership blocks](source-index.md#ownership-blocks), and chapter 1's session
owns 3 blocks, with 24 owned by the other chapters. The inspection
records extend the corpus under the same recursive rule. No `defer` directive remains anywhere in the
book, and none may: `F003` reports any defer at all in final mode, so *every
deferral has become an insertion* is a statement the validator refuses to let be
false rather than one this page asserts.

<!-- rollup «ownership-blocks» -->
<!-- rollup «source-roots» -->
27 blocks over 11 roots is the price of reading the crate
in its own conceptual order. Seven roots are owned whole by one chapter; the other
four split, and each split is the concept order disagreeing with the file's. The
figure is the original three split files in file order, and what it carries that a list of
ranges cannot is the interleaving: in two of the three, one chapter's block sits
*inside* another chapter's pair.

```text
src/templates.rs   982 lines, 8 blocks, chapters 2 3 4 5
      1-164   template-shapes
    165-278   templates-load
    279-460   resolution-and-expansion
    461-669   reading-and-whole-document-validation
    670-809   node-and-template-rules
    810-892   word-scanning
    893-972   diagnostics
    973-982   templates-keys

src/run.rs         672 lines, 4 blocks, chapters 7 8
      1–123   ch 7   ┐  the launch's shape
    124–243   ch 8   │  the watch and the latch, inside chapter 7's pair
    244–510   ch 7   ┘  the terminal and the spawn
    511–672   ch 8      supervise and escalate

src/channel.rs     404 lines, 2 blocks, chapters 6 9
      1–271   ch 6      the production code
    272–404   ch 9      the inline #[cfg(test)] module
```

`src/templates/named.rs` adds six blocks: capture and diagnostics, target folding
and projection in chapter 3, and named dollar scanning plus parameter instantiation
in chapter 4. Defaults resolve into the same literal words inspection explains
and expansion forwards.
`src/templates.rs` divides by what a reader needs when, and `src/run.rs` divides
by whose signal a line is about — which is why chapter 8's watch and latch sit
between chapter 7's launch shape and its spawn. `src/channel.rs`'s single
boundary is the `#[cfg(test)]` attribute on line 272, the only ownership boundary
in the book cut at a compilation condition rather than a conceptual one.

<!-- rollup «early-use-rows» -->
<!-- rollup «early-use-rows-declared» -->
<!-- rollup «early-use-rows-at» of="01-orientation.md#the-cast" -->
**Early use.** 12 rows, every one `explained`. 11 of them are the manifest's
`[[early-use]]` entries, which are the rows the book may not omit: 9 forced
by [chapter 1's cast](01-orientation.md#the-cast) naming nearly every public type
before its owner explains it, one by
[chapter 3](03-two-documents.md#both-documents) reaching the two rule checks that
`validate_document` drives and chapter 4 owns, and one by
[chapter 7](07-the-job.md#the-spawn) reaching the handler, the latch and the
supervisor that are `run`'s first and last acts and chapter 8's to explain. The
twelfth was added under the clause that requires a row before any additional
later-owned name is introduced: chapter 3's parse reaches `source_location`,
`format_location` and `render_diagnostics`, three of chapter 4's diagnostic
helpers, and carries the minimum statement until that chapter explains them
together. Each row turned `explained` in its owner's slice and in no other.

<!-- rollup «owned-lines-sequence» -->
<!-- rollup «source-owning-chapters» -->
**Owned source.** 292 + 316 + 936 + 441 + 240 + 271 + 390 + 282 + 231 = 3,399
lines across 9 chapters, and 0 for this one. The tenth row of that table exists
to be zero: a chapter that owns no source is the shape the structure brief chose
for the assembly, and the total is the 3,399 lines in the current declared corpus.

<!-- rollup «source-roots» -->
The [concept index](concept-index.md) and the [source index](source-index.md) are
the two lookup surfaces, and neither is part of the reading order. The source
index is the authoritative record of how the fragment graph reconstructs each of
the 11 files; the concept index is curated navigation into the arguments, and
makes no completeness claim.

<a id="final-verification"></a>
## Final verification

Three commands prove the book, and they prove different things. The first is the
only one that reads the corpus byte for byte.

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/keyed-launch --final --check all
valid: 11 files, 3172 resolved lines, 0 deferred lines, final=true
```

`--final` is what makes this different from every scoped run the drafting
sessions made. In scoped mode a later chapter's range may be reserved by a defer
and counted as deferred rather than resolved; in final mode a defer is an error,
every source root must expand to its complete file, and the page inventory must
match the manifest exactly. 3,399 resolved and 0 deferred is the whole corpus
reconstructed — including `src/channel.rs` lines 272 to 404, the inline
`#[cfg(test)] mod tests` that is corpus because a root is `src/**/*.rs` and the
specification's exception inventory carries no row for this book.

```console
$ bash scripts/check.sh
...
  6 book(s) checked, 0 failing
  ✓ book-check

check: all 8 principal checks pass
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
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/conformance_kit.rs
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/interrupt.rs
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/launch.rs
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/reraise.rs
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/templates.rs
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   Doc-tests keyed_launch
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Fifty-four tests and one documentation test, and the split across the two surfaces is
itself a fact this book explained. Nine are the inline `#[cfg(test)] mod tests`
inside `src/channel.rs`, which are corpus and are on
[chapter 9's page](09-how-checked.md#inside-the-root) like any other lines. The
*module* exists because `use super::*;` reaches seven items no integration test
can see, and one of the nine tests takes that up: only a module inside the file
can call `is_channel_name`, whose exactness chapter 6 argued. The other eight use
the public surface and would compile anywhere; they are in here because their
subject matter is. The other forty-five live in five files and 1,370 lines under
`crates/keyed-launch/tests/` — 66% of the corpus's own size — and are evidence
rather than corpus: named wherever a chapter adjudicates a claim, reproduced
nowhere. The single doctest is the conformance kit's usage example, which is
`no_run` because running it would need a configuration file that does not exist.

The seven tests the book's stated outcome names are all in that outer directory,
and they divide the way the arms do: two in `tests/templates.rs` for the way in,
three more in the same file for the way through, and two in `tests/launch.rs` for
the way out. This page named two others from the same directory —
`quoted_and_midword_hashes_stay_literal`, which covers the quoted and midword
counter-cases to the `#` rule, and
`arguments_reach_the_child_as_written`, which is the one trip it runs end to end
— for nine in all.
Nothing in the corpus holds them; they hold the corpus.

<!-- rollup «source-roots» -->
<!-- rollup «owned-lines-total» -->
<!-- rollup «chapters» -->
The book is complete: 11 roots, 3,399 lines, 10 chapters, two lookup surfaces,
zero deferred ranges. What it argued is that nine refusals are one design, and
what it leaves the reader with is the question — *where does this layer learn what
the value means?* — together with the one case where this crate's own answer runs
out, and the reason that case belongs to whoever knows what the child was asked
to do.

[Previous: How this is checked](09-how-checked.md) | [Contents](README.md)
