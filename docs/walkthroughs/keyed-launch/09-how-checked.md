# How this is checked
<!-- book-page id="how-checked" slice="checked-without-meaning" order="9" -->
[Previous: The watch and the escalation](08-the-escalation.md) | [Contents](README.md) | [Next: What passes through](10-what-passes-through.md)

<a id="checked-without-meaning"></a>
## Checked without meaning

Chapter 8 closed the launch half, and with it the crate. A configuration became
an argv, an argv became a child, and a launcher waited for one of three things
without ever deciding for itself that the child was finished. Both halves are
complete, and neither has yet been held to a contract from outside itself.

What this stage must not add and must not interpret is **the standard**.
Checking is the activity that seems most obviously to require knowing what
*correct* means, and a conformance kit is therefore where a layer like this one
is most likely to acquire a domain. This one does not. It holds a consumer's
configuration to the obligations this crate states, and every one of those
obligations is about the shape of a document rather than about what any key in it
is for.

The chapter owns 237 lines in two roots, and they are one chapter because their
subject is the same. `src/conformance.rs` is the kit whole, 104 lines.
`src/channel.rs` lines 272–404 are the inline `#[cfg(test)] mod tests` chapter 6
stopped at — 133 lines and nine tests. The two check in opposite directions: the
kit holds a **consumer's document** to this crate's contract from outside the
consumer, and the module holds **this crate's own private grammar** to its rules
from inside the file that states them. Neither learns a meaning on the way.

**The inline module is corpus and `crates/keyed-launch/tests/` is not.** A root
of this book is every `src/**/*.rs` file of the crate plus the crate's own
`Cargo.toml`, and `docs/specs/walkthrough-books.md`'s corpus exception inventory
carries no `keyed-launch` row. So those 133 lines are owned, reconstructed and
explained exactly like production source, while the five files and 1,319 lines
under `crates/keyed-launch/tests/` — 64% of the corpus's own size — are evidence:
named wherever a chapter adjudicates a claim, reproduced nowhere. The rule is
about where a file sits, not about what it contains, and this is the one page in
the book where the difference is visible.

The weight of the prose below is set by where the source argues and where it does
not. `src/conformance.rs` is 33 comment lines in 104 — twenty-five of them one module
comment under a heading of its own — and it argues its own design; the test module
is 10 in 133 and argues almost nothing. So the
first half of this page reads the kit's argument and adjudicates it, and the
second half supplies the argument the tests do not make — what each one actually
holds, which of chapter 6's claims it reaches, and which it leaves unreached.

<a id="checking-the-same-file"></a>
## Checking the same file

The kit's whole input is a path and a vocabulary, and this book already has both.
The path is the operator's personal `~/.config/grove/config.kdl` holding the two
lines chapter 1 fixed; the vocabulary is chapter 2's four slots. This section
runs `conformance::check` over that file three times — once as chapter 1 wrote
it, once with both keys commented out, and once with the substitution dropped
from the first template — and the observable end of each run is an `Outcome`.

```text
impl "claude --model opus ${prompt}"
review-impl "codex exec --model gpt-5 ${prompt}"
```

That is the document the first run is handed: the same two lines chapters 3, 4
and 5 loaded, validated and expanded, checked here by a caller that is not grove
and has no values of its own.

```text
config      = the operator's ~/.config/grove/config.kdl
GROVE_SLOTS = the four SlotRules of chapter 2's table

conformance::check(config, Vocabulary { slots: &GROVE_SLOTS })
  -> Outcome { failures: [] }

outcome.passed() -> true
```

Three obligations ran to produce that empty list. The document loaded whole
against the four slots; it declared at least one key; and every key it declares
was expanded, with one placeholder value per slot the vocabulary names. The two
argvs below are what the third obligation built and immediately dropped — its
evidence rather than its product, since the kit spawns nothing.

```text
impl        -> ["claude", "--model", "opus", "<prompt>"]
review-impl -> ["codex", "exec", "--model", "gpt-5", "<prompt>"]
```

`<prompt>` is the whole of what the kit knows about a value: the slot's own name
in angle brackets. Values for `session_name`, `worktree` and `repo` were offered
as well, and neither template mentions them, because expansion's obligation is
stated over the vocabulary rather than over the template in front of it —
chapter 5's rule, met here by a caller with nothing to fill it from.

Now the first failure. The operator comments both lines out, or ships a
`config.kdl` that never declared anything:

```text
// impl "claude --model opus ${prompt}"
// review-impl "codex exec --model gpt-5 ${prompt}"
```

```console
~/.config/grove/config.kdl declares no keys, so nothing in this kit was exercised. A configuration that checks nothing passes every check.
```

That is `outcome.failures` printed the way every caller prints it, and it is the
whole of the list. The document is well-formed KDL, declares nothing, and
violates no rule; a kit that only reported violations would call it conforming.
The message's second sentence is the reason the first one is a failure at all,
and it states the chapter's connection to the book's central claim.

And the second failure. The operator drops the substitution from `impl`, leaving
a line that still reads as a complete command:

```text
impl "claude --model opus"
review-impl "codex exec --model gpt-5 ${prompt}"
```

```console
invalid configuration at ~/.config/grove/config.kdl:
  - ~/.config/grove/config.kdl:1:1: key `impl`: command template must contain `${prompt}` exactly once
```

Those two lines are chapter 2's refusal, verbatim and unwrapped: the kit did not
re-word it, re-classify it, or add to it. They are the whole of the list again,
and for a different reason — the load failed, `check` returned carrying its
message, and the second and third obligations were never reached. That early
return belongs to the first obligation alone: the other two record a failure and
let the function run on, so the order the three are applied in decides an outcome
here and nowhere else in the kit.

One fact about all three runs is invisible in the figures and decides what a
green result means. `check` loads with **no overlay**: the path it is handed is
the only file it reads, and `the_kit_reads_only_the_file_it_is_given` in
`crates/keyed-launch/tests/conformance_kit.rs` pins that by writing a second
document into the same directory and asserting the outcome is unchanged. Which
files take part in a configuration is the consumer's question, not this crate's.
For a grove operator that means a green run over a personal `config.kdl` is a
statement about that document alone, and says nothing about an overlay beside it.

<a id="what-the-blocks-answer"></a>
## What the two blocks answer

The kit is one public function over one public type, and its three obligations
are what the first half of this page reads one at a time. The table collects
them with what each reports and what holds it, so the sections below can be read
in any order. Tests named without a path are in
`crates/keyed-launch/tests/conformance_kit.rs`.

| # | Obligation | Decided by | What a failure carries | Pinned by |
|---:|---|---|---|---|
| 1 | the document loads whole against the vocabulary | `Templates::load` | the load's own refusal, unchanged | `a_violated_template_rule_is_reported_as_a_failure`, `a_missing_file_is_a_failure_rather_than_a_panic`, `the_kit_and_grove_refuse_the_same_document` (`crates/grove-loop/tests/session_config.rs`) |
| 2 | it declares at least one key | `Templates::keys` | `` `p` declares no keys, so nothing in this kit was exercised. A configuration that checks nothing passes every check. `` | `a_document_that_exercises_nothing_does_not_pass` |
| 3 | every key expands to an argv with a program | `Templates::expand` | ``key `k` does not expand: … `` ; ``key `k` expands to an empty program`` | — |

The empty cell in the last row is a finding rather than an omission, and
[Walking the compiled words](09-how-checked.md#walking-the-compiled-words) is
where it is established: no test reaches either of the third obligation's failures, and on
today's validation rules no consumer's document could produce one.

The first composite is the kit whole — a module thesis, one single-field type
with one method, and one function. It is also the crate's only public module.

<!-- fragment «conformance» owner="checked-without-meaning" source="crates/keyed-launch/src/conformance.rs" lines="1-104" parent="source-conformance" -->
<!-- insert «conformance-thesis» -->
<!-- insert «conformance-outcome» -->
<!-- insert «conformance-check-and-placeholders» -->
<!-- insert «conformance-load» -->
<!-- insert «conformance-no-keys» -->
<!-- insert «conformance-values» -->
<!-- insert «conformance-expands» -->
<!-- /fragment -->

The second is the inline module that closes `src/channel.rs`. Chapter 6 owns
every byte above line 272 and named this boundary as the only one in the book cut
at a compilation condition rather than at a concept;
[What a module inside the root reaches](09-how-checked.md#inside-the-root)
states the reason.

<!-- fragment «channel-inline-tests» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="272-404" parent="source-channel" -->
<!-- insert «channel-tests-module» -->
<!-- insert «channel-tests-allocate» -->
<!-- insert «channel-tests-read» -->
<!-- insert «channel-tests-discard» -->
<!-- insert «channel-tests-cleanup» -->
<!-- /fragment -->

<a id="from-outside-the-consumer"></a>
## The seam a consumer's own suite cannot cross

Twenty-five of `src/conformance.rs`'s 104 lines are its module comment, and it
makes three separate cases: what the kit is for, what calling it looks like, and why a
document that declares nothing is a failure. It is reproduced whole with the four
imports that follow it, because each of the three arguments is made here and
nowhere else in the crate, and because the third is the one claim on this page
that the code cannot make on its own — an empty `failures` list is the same value
whether the kit checked two keys or none.

<!-- fragment «conformance-thesis» owner="checked-without-meaning" source="crates/keyed-launch/src/conformance.rs" lines="1-31" parent="conformance" -->
````rust
//! The conformance kit: hand it a configuration file and the slot vocabulary it
//! was written against, and learn which of this crate's obligations it violates.
//!
//! This is the **cross-crate seam**. A consumer's own suite can only assert that
//! *its* configuration works with *its* build; the kit is what holds a
//! configuration to the contract this crate states, from outside the consumer,
//! which is what keeps *reusable outside grove* true without a second
//! repository.
//!
//! ```no_run
//! use keyed_launch::{conformance, Requirement, SlotRule, Vocabulary};
//! let outcome = conformance::check(
//!     std::path::Path::new("config.kdl"),
//!     Vocabulary { slots: &[SlotRule { name: "prompt", requirement: Requirement::ExactlyOnce }] },
//! );
//! assert!(outcome.passed(), "{}", outcome.failures.join("\n"));
//! ```
//!
//! # Why an empty document fails
//!
//! A kit that only reports violations reads exactly the same when it is handed
//! nothing to check: no keys, no violations, conforming. A configuration
//! declaring no keys is therefore a failure in its own right — not because an
//! empty file is malformed, but because a suite of must-hold claims cannot
//! otherwise detect that it did not run.

use std::ffi::OsString;
use std::path::Path;

use crate::templates::Templates;
use crate::vocabulary::Vocabulary;
````
<!-- /fragment -->

**The cross-crate seam is the first claim, and it is checkable.** A consumer's
own suite can assert that *its* configuration works with *its* build, which is a
statement about one pairing; what it cannot do is hold that configuration to the
contract this crate states, because that contract is not written down anywhere
the consumer's suite can read it. The kit is the contract in executable form, and
`crates/grove-loop/tests/session_config.rs` is where the seam is actually
crossed. Two tests there call the kit from the other side of the workspace:
`a_grove_configuration_conforms_to_the_runners_own_kit` writes a four-key
document whose first template uses all four of grove's slots and asserts the
outcome passes, and
`the_kit_and_grove_refuse_the_same_document` writes a template naming an
undeclared slot and asserts the same substring — ``unknown substitution
`${settings}` `` — in the kit's failure and in grove's own load error. One
contract, checked from two sides, under a heading in that file naming it as test
seam 3 of `docs/specs/module-decomposition.md`.

*Which is what keeps* reusable outside grove *true without a second repository*
is the clause that names the alternative. The ordinary evidence that a crate is
consumable outside the repository that grew it is a second consumer; this crate
has none and will not acquire one, because `Cargo.toml` settles that it ships
inside grove's release with no publication lane of its own (chapter 1). The kit
is the cheaper form of the same claim and is weaker in one exact way worth
stating: it establishes that a configuration conforms, not that a second consumer
could build against the crate at all.

**The example is the crate's only doctest, and it is `no_run`.** `cargo test`
compiles it against the public surface and does not execute it, which is the
right setting for an example naming a `config.kdl` that does not exist. What it
proves is the spelling: that `conformance::check` takes a `&Path` and a
`Vocabulary` built out of `SlotRule`s, that all four names are reachable from the
crate root, and that `Outcome::passed` and `Outcome::failures` are both public.
It is not the only thing that would break if one of those names moved —
`crates/keyed-launch/tests/conformance_kit.rs` imports the identical four — but it
is the only one a consumer reads before writing any code of their own, which is
the surface the kit exists to be.

`conformance` is also the crate's **only public module**. `argv`, `channel`,
`error`, `run`, `templates` and `vocabulary` are all private, with eighteen names
re-exported at the crate root, so a consumer writes `keyed_launch::Templates` and
never `keyed_launch::templates::Templates`. The kit is deliberately not flattened
that way. Keeping `conformance::` in front of `check` is what separates a call a
consumer's production code makes from one only its tests make.

**Why an empty document fails** is the third argument, and it is the one the rest
of the crate depends on nowhere else. A kit that only reports violations cannot
distinguish *checked, and found nothing wrong* from *checked nothing*: both
produce an empty list. Chapter 5 stated the minimum of this at `keys`, the
accessor that exists for it. The full form is that the kit is a suite of
must-hold claims, and a suite that ran no claims is not a passing suite but an
unrun one — and the only evidence available to a layer that never learns what a
key is for is that the document declared something to check. The comment is
careful about what the failure is *not*: an empty file is well-formed KDL and
loads without complaint, so the refusal is about the kit's ability to report
rather than about the document's form.

The alternative it rules out is the one a kit with a domain would take: assert
that particular keys are present. A kit requiring `impl` and `review-impl` would
catch the commented-out document immediately and would give a better message for
it. It would also be the crate learning what a key is for — the one thing the
whole book says it does not do — and it would be unusable by any consumer whose
key set differs from grove's. Counting is what is left when naming is
unavailable, and one is the only threshold a counter with no domain can defend.

<a id="a-list-of-sentences"></a>
## A list of sentences, and one question over it

The result type is declared before the function that builds it, and it is one
field and one method. It exposes the failure strings, and `passed()` derives its
answer from whether that list is empty.

<!-- fragment «conformance-outcome» owner="checked-without-meaning" source="crates/keyed-launch/src/conformance.rs" lines="32-43" parent="conformance" -->
````rust

/// What the kit found. Empty [`failures`](Self::failures) is conformance.
pub struct Outcome {
    pub failures: Vec<String>,
}

impl Outcome {
    #[must_use]
    pub fn passed(&self) -> bool {
        self.failures.is_empty()
    }
}
````
<!-- /fragment -->

`failures` is a public field, and the crate's public types divide on exactly that
question. Six hide their fields behind accessors — `Templates`, `Argv`,
`Channel`, `Token`, `ConfigError` and `LaunchError` — and seven do not:
`Vocabulary`, `SlotRule`, `Slot`, `Escalation`, `Launch`, `Ended` and `Outcome`.
The line is not data against behaviour and it is not input against output, since
`Launch` is an input and `Ended` an output and both are plain records. It is
whether a public field would let a caller **fabricate** a value this crate was
supposed to have authored. A `Templates` is a validated document, an `Argv` is
words a template chose, a `Channel` is a path allocation drew, a `Token` is a
string a child wrote, and the two error types are this crate's own diagnosis —
each is a value whose provenance is the guarantee, and a public field would hand
that guarantee away. An `Outcome` guarantees nothing of the kind: its strings are
a report about somebody else's file, and a caller who built a false one would
only be misleading itself.

`Vec<String>` rather than `ConfigError` is chapter 1's argument about the error
types applied one layer out. The failures arrive from two different places — a
`ConfigError`'s rendered message, and the kit's own sentence — and their only use
is being read by whoever has to fix the file. A variant list would be a second
interface for a value nobody matches on. Every use of the field in the repository
reads it as text and none takes it apart: an assertion's condition either scans
the list with `failures.iter().any(|failure| failure.contains(…))` or indexes
`failures[0].contains(…)`, and its message either joins the list with
`failures.join("\n")` or prints the whole vector with `"{:?}"`. Four shapes, and
all four want sentences.

`passed` is the one question asked over the list, and both it and `check` are
`#[must_use]`. That pairing is what makes the kit a value rather than an action.
`check` has no effect to call it for — it reads a file and returns — so calling it
and discarding the result is always a mistake, and the attribute says so at the
call site.

**Today the list holds at most one entry**, and the reason is worth stating
because the plural shape suggests otherwise. A load failure returns immediately,
so obligation 1 contributes one string and nothing after it runs. The empty-key
rule and the expansion loop are mutually exclusive, because the loop iterates the
list the rule found empty. And the loop cannot contribute anything at all, for
the reason
[Walking the compiled words](09-how-checked.md#walking-the-compiled-words) gives.
`failures[0]` is indexed by two tests —
`a_missing_file_is_a_failure_rather_than_a_panic` here and
`the_kit_and_grove_refuse_the_same_document` in `crates/grove-loop` — and both
are load failures, where the early return is what guarantees the index. The
plural is what lets an obligation be added later that co-occurs with another; it
is not a description of what the kit produces now.

<a id="three-obligations"></a>
## Three obligations, and the values they are checked with

`check` states its contract before it does anything and then builds the only
values it will ever supply. The doc comment is where the three obligations are
named and ordered, and the ten lines under the signature are where the kit
decides what a slot value looks like when nobody has one.

<!-- fragment «conformance-check-and-placeholders» owner="checked-without-meaning" source="crates/keyed-launch/src/conformance.rs" lines="44-63" parent="conformance" -->
````rust

/// Hold a consumer's configuration to this crate's contract.
///
/// Three obligations, in order: the document loads whole against `vocabulary`;
/// it declares at least one key; and every key it declares expands to an argv
/// with a program, given one placeholder value per declared slot. The third is
/// what stops the kit from being a second spelling of `load` — expansion is the
/// only place the compiled words are walked.
#[must_use]
pub fn check(config: &Path, vocabulary: Vocabulary<'_>) -> Outcome {
    let placeholders: Vec<(String, OsString)> = vocabulary
        .slots
        .iter()
        .map(|slot| {
            (
                slot.name.to_owned(),
                OsString::from(format!("<{}>", slot.name)),
            )
        })
        .collect();
````
<!-- /fragment -->

The signature is `Templates::load`'s two mandatory arguments and nothing else: a
path and a vocabulary. That is the kit's entire input surface, and it is why the
kit can be run by a consumer that has never constructed a `Slot`, a `Channel` or
a `Launch`.

The placeholders are one per slot the vocabulary declares, each the slot's own
name in angle brackets — `<prompt>`, `<session_name>`, `<worktree>`, `<repo>`.
The brackets are not a form this crate enforces or recognises anywhere; they are
chosen so that a value surfacing in a diagnostic reads as a placeholder rather
than as a plausible path. `OsString` is not a choice at all: `Slot::value` is an
`&OsStr`, because chapter 5's expansion writes values straight into an argv and
an argv's words are OS strings.

The two-pass shape — owned `(String, OsString)` pairs here, borrowed `Slot`s
further down the function — is a lifetime consequence rather than a style. A
`Slot<'a>` holds a `&'a str` and a `&'a OsStr` and owns neither, so something has
to own the generated names and values for as long as the slice of `Slot`s
exists. Building the `Slot`s directly from `vocabulary.slots` would give the
names a long enough life and leave the values with nowhere to live, since each is
a `String` this function has just formatted.

<a id="the-whole-document-first"></a>
## The whole document, or nothing to check

The first obligation is one call and one early return. Everything chapters 3
and 4 proved about a configuration — its syntax, its node shape, its duplicate
keys, every template rule and both cardinalities — is what the kit inherits by
making it.

<!-- fragment «conformance-load» owner="checked-without-meaning" source="crates/keyed-launch/src/conformance.rs" lines="64-72" parent="conformance" -->
````rust

    let templates = match Templates::load(config, None, vocabulary) {
        Ok(templates) => templates,
        Err(error) => {
            return Outcome {
                failures: vec![error.to_string()],
            }
        }
    };
````
<!-- /fragment -->

`None` in the overlay position is the kit's one deliberate narrowing. A
consumer's real load may pass a second document; the kit checks the file it was
handed and does not look beside it, because *which* files take part is a question
about the consumer's own policy rather than about this crate's contract.
`the_kit_reads_only_the_file_it_is_given` writes a `.grove.kdl` next to the
configuration under test and asserts the outcome is unchanged, which pins the
absence instead of leaving it to be inferred from the argument.

`error.to_string()` is what makes the failure identical to the one a consumer's
own load would print. `ConfigError`'s `Display` writes the message and nothing
around it — chapter 1 read the two implementations that guarantee it — so the
string that lands in `failures` is byte-for-byte the refusal chapters 2 and 4
assembled. `the_kit_and_grove_refuse_the_same_document` is the assertion that
depends on it: it looks for the same substring in the kit's failure and in
grove's own load error, and a kit that wrapped, prefixed or re-classified the
message would fail it.

The whole of the first obligation is nine lines because the checking belongs to
somebody else. A kit that re-implemented the rules would be a second statement of
them and the two would drift; this one cannot, because a rule the loader stops
enforcing is a rule the kit stops reporting in the same commit.

<a id="declares-no-keys"></a>
## The failure that has to be its own

The second obligation is the one the kit could not get from `load`, and the only
one whose sentence the kit writes itself.

<!-- fragment «conformance-no-keys» owner="checked-without-meaning" source="crates/keyed-launch/src/conformance.rs" lines="73-82" parent="conformance" -->
````rust

    let mut failures = Vec::new();
    let keys: Vec<String> = templates.keys().into_iter().map(str::to_owned).collect();
    if keys.is_empty() {
        failures.push(format!(
            "{} declares no keys, so nothing in this kit was exercised. A configuration \
             that checks nothing passes every check.",
            config.display()
        ));
    }
````
<!-- /fragment -->

`keys` is chapter 5's one window into a loaded configuration, and this is the
call it exists for. It returns the keys the **primary** declares, in name order;
an overlay-only key is not among them, which for this caller is moot, since it
passed no overlay.

The message names the file and then explains itself, and that second sentence —
*A configuration that checks nothing passes every check* — is the module comment's
argument restated where the failure is produced. It is also the only refusal in
the crate that ends in a justification. Chapter 1 read the obligation
`ConfigError` carries — name what is wrong, name where, and name what fixes it —
and the crate discharges that third clause two ways: imperatively, as in
*create the directory* and *remove it by hand*, or by supplying what the fix
needs, as in *declared slots: …*. This message does neither. It names what is
wrong and where, and then argues that the failure is a failure. The difference is
in who reads it. `ConfigError`'s messages are read by an operator looking at a
file they did not expect to be
wrong; this one is read in a test runner's output by the author of the document,
for whom *declare a key* is not the missing information.

`a_document_that_exercises_nothing_does_not_pass` is the test, and its own doc
comment states the argument a third time. What it asserts is narrower than the
argument: a document of `// nothing at all` does not pass, and some failure
contains the substring `declares no keys`. It does not assert the second
sentence, the file name, or that the list holds exactly one entry — which is the
right shape, because the claim under test is that an empty document is refused,
not how the refusal is worded.

<a id="walking-the-compiled-words"></a>
## Walking the compiled words

The third obligation needs values, and the values are the placeholders from the
top of the function borrowed into the shape `expand` accepts.

<!-- fragment «conformance-values» owner="checked-without-meaning" source="crates/keyed-launch/src/conformance.rs" lines="83-90" parent="conformance" -->
````rust

    let values: Vec<crate::Slot<'_>> = placeholders
        .iter()
        .map(|(name, value)| crate::Slot {
            name,
            value: value.as_os_str(),
        })
        .collect();
````
<!-- /fragment -->

This is the second half of the two-pass shape. Each `Slot` borrows a name and a
value out of `placeholders`, which is still alive and outlives the call below it;
`value.as_os_str()` is the owned `OsString` handed back as the `&OsStr` a `Slot`
holds.

With those, the loop. It is the whole of the third obligation and the last thing
in the file.

<!-- fragment «conformance-expands» owner="checked-without-meaning" source="crates/keyed-launch/src/conformance.rs" lines="91-104" parent="conformance" -->
````rust

    for key in &keys {
        match templates.expand(key, &values) {
            Ok(argv) => {
                if argv.program().is_empty() {
                    failures.push(format!("key `{key}` expands to an empty program"));
                }
            }
            Err(error) => failures.push(format!("key `{key}` does not expand: {error}")),
        }
    }

    Outcome { failures }
}
````
<!-- /fragment -->

**Expansion is the only place the compiled words are walked.** `load` walks the
words of the *source* — a `&str` split by `shell_words`,
each word classified into a `Word` — and then stores the `Vec<Word>` and never
reads it again. Nothing between the load and the spawn touches the compiled form
except `Templates::expand`, which turns it back into `OsString`s and hands the
result to `Argv`. So this loop is the only check anywhere that the compilation
step round-trips: that a template which validated as text can still be turned
back into a program and its arguments.

**And on today's validation rules it cannot report a failure.** For any document
that reaches this loop, neither of the two strings above can be produced, and the
argument is short enough to give completely. `load` returns an error if
`validate_document` collected any diagnostic at all, so a template that survives
has none. A template with no words, or whose word zero is not a non-empty
`Word::Literal`, produces a diagnostic — chapter 4's two word-zero rules — so
after a successful load every stored template begins with a non-empty literal and
`argv.program()` cannot be empty. Every `Word::Slot` index came from a `position`
in the same slot table `Templates` still holds, so no index is out of range. And
`match_values` refuses on exactly three conditions: an offered name the vocabulary
does not declare, a name offered twice, and a declared slot with no value. The
placeholder list is built from `vocabulary.slots` itself, one entry each, so it
satisfies all three by construction — and `compile_vocabulary` has already
refused a vocabulary that declares a name twice.

That is what the empty cell in this chapter's table records. The two failure
strings at the bottom of `src/conformance.rs` name a defect in **this crate** — a
compilation that stopped round-tripping, or a validation rule that stopped
covering word zero — rather than a defect in any consumer's document. No test
reaches either, and none could be written without breaking the crate first.

The obligation is still not a second spelling of `load`, and the distinction is
worth being exact about: it is redundant as a *report about the document* and it
is not redundant as *coverage of the path a document takes*. A consumer that runs
the kit expands every key its configuration declares, with a full set of values,
on every run — which its own suite would otherwise do only for the keys it
happens to launch in a test. Dropping the third obligation would leave `check` a
load and a key count, saying nothing about the half of the configuration path a
document actually flows through at launch time. What it costs is one loop and the
two `Argv` values it discards.

The kit's boundary is the configuration half, and it stops here. There is no
conformance surface over `run`, `Channel` or the escalation, and the reason is
not that the launch half needs one less: that half's contract is with the
operating system rather than with a document, so a kit over it would need a
program to spawn, a terminal to hand over and a machine to be slow on.
`crates/keyed-launch/tests/launch.rs` is exactly that, at 616 lines, and it is a
test suite rather than a kit because it cannot be handed to a consumer to run
against the consumer's own material.

<a id="inside-the-root"></a>
## What a module inside the root reaches

The rest of this chapter is 133 lines that are neither production code nor
outside the corpus. They are the end of `src/channel.rs`, compiled only under
`cfg(test)`, and they are the only tests in the crate that can see a private
item.

<!-- fragment «channel-tests-module» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="272-274" parent="channel-inline-tests" -->
````rust
#[cfg(test)]
mod tests {
    use super::*;
````
<!-- /fragment -->

`use super::*;` is what the module is for. An integration test under
`crates/keyed-launch/tests/` links the crate as an external library and sees the
public surface — eighteen re-exported names and one public module. A module
inside the file sees everything the file declares, and for `src/channel.rs` the
difference is seven items: the constants `CHANNEL_PREFIX`, `NONCE_BYTES` and
`DRAW_RETRY_LIMIT`, and the private functions `is_channel_name`,
`remove_if_present`, `draw_nonce` and `hex`. One of the seven is the reason the
module exists. Chapter 6 argued `is_channel_name`'s **exactness** at length — prefix, length and
lowercase alphabet as three conditions, each ruling out one way a name can be
nearly right, against a looser rule that would let this crate's cleanup delete a
neighbouring file — and the only test that can call it directly is in this
module.

That is why the book cuts `src/channel.rs` at line 272 and gives the two pieces
to different chapters, and it is the only ownership boundary in the book taken at
a compilation condition rather than at a concept. The module's *subject matter*
is chapter 6's, since every function it exercises is above line 272; its
*subject* is assurance, which is this chapter's. Explaining the tests here, with
chapter 6's fragments behind the reader, costs that chapter one forward reference
and puts the evidence for one claim beside the evidence for every other claim
about the same crate.

The nine tests divide by the method each exercises, which is how the four
sections below take them. The table is the map: what each test holds, and where
in chapter 6 the rule it holds was argued.

| Test | What it holds | Argued at |
|---|---|---|
| `an_allocated_channel_names_a_path_that_does_not_yet_exist` | allocation picks a name and creates nothing, and the name it picks satisfies the grammar | [Drawing a name](06-the-channel.md#drawing-a-name), [Exactly this name](06-the-channel.md#exactly-this-name) |
| `successive_allocations_in_one_directory_never_collide` | a nonce is drawn per call, not per directory or per process | [Drawing a name](06-the-channel.md#drawing-a-name) |
| `allocation_names_a_missing_directory_and_says_what_to_do` | the directory is checked before anything is spawned, and the refusal names the fix | [Drawing a name](06-the-channel.md#drawing-a-name) |
| `a_signalled_channel_reads_back_the_token_without_its_framing` | `signal` frames the token with a newline and `read` trims the framing back off | [The other end of the channel](06-the-channel.md#the-other-end), [Three ways to have no token](06-the-channel.md#three-ways-to-have-no-token) |
| `an_empty_channel_file_is_not_an_empty_token` | an empty or whitespace-only file reads as `None`, never as `Some("")` | [Three ways to have no token](06-the-channel.md#three-ways-to-have-no-token) |
| `an_unsignalled_channel_reads_back_nothing` | a path nothing ever wrote to reads as `None` | [Three ways to have no token](06-the-channel.md#three-ways-to-have-no-token) |
| `discarding_removes_the_file_and_succeeds_when_there_was_none` | the post-condition is *this path holds nothing*, both when there was a file and when there was not | [Removing this launch's file](06-the-channel.md#discarding), [Three helpers](06-the-channel.md#the-three-helpers) |
| `abandoned_cleanup_removes_channels_and_leaves_every_other_entry_alone` | cleanup removes exactly the names the grammar accepts and nothing else in the directory | [The cleanup that must not overreach](06-the-channel.md#the-cleanup-that-must-not-overreach), [Exactly this name](06-the-channel.md#exactly-this-name) |
| `abandoned_cleanup_names_the_directory_when_it_cannot_be_listed` | a cleanup that cannot list its directory refuses and names it | [The cleanup that must not overreach](06-the-channel.md#the-cleanup-that-must-not-overreach) |

Nothing in the module spawns a process, and that is the other half of the
boundary. All nine build a `tempfile::tempdir()` and work on the filesystem, so
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

<!-- fragment «channel-tests-allocate» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="275-312" parent="channel-inline-tests" -->
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

The first test states three parts of chapter 6's central claim in order: the path
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
the half of the grammar `hex` owns and the reason chapter 6's table credits that
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

The next three are `Channel::read`, and between them they are chapter 6's
distinction between a file and a token: one write that produces a token, and two
files that do not.

<!-- fragment «channel-tests-read» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="313-351" parent="channel-inline-tests" -->
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
at all. That is chapter 6's `read` comment restated where the case is exercised.
The loop over `["", "\n", "  \n"]` is what makes it three cases rather than one,
and the third is the one that matters: it exercises `trim_end` past whitespace
that is not a newline, and a `read` that stripped only a trailing `\n` would
return `Some("  ")` and fail there.

The third test completes a set. Chapter 6 named
three ways to have no token — nothing was written, the file cannot be read, and
the file is there but empty — and this module reaches the first and the third.
The unsignalled test exercises the `.ok()?` arm through `NotFound`, collapsing
the read failure into an absent token. No test supplies the middle premise: an
existing file whose read returns `Err`.

<a id="a-post-condition"></a>
## A post-condition, tested from both of its sides

One test covers `Channel::discard`, and it covers both halves of what chapter 6
called the post-condition: *this path holds nothing*, rather than *this call
removed something*. It needs two channels to do it.

<!-- fragment «channel-tests-discard» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="352-365" parent="channel-inline-tests" -->
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

<!-- fragment «channel-tests-cleanup» owner="checked-without-meaning" source="crates/keyed-launch/src/channel.rs" lines="366-404" parent="channel-inline-tests" -->
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
its caller. Five files go into one directory: one name the grammar accepts, and
four it must reject — a suffix in uppercase hex, a suffix of sixteen characters
instead of thirty-two, a nonce with no prefix, and `driver.lease`, which is not a
channel name in any respect and is the file chapter 6 named as the cost of a
loose rule. One assertion covers the removal, and a loop asserts each of the
four survivors in turn, every failure message naming the path that should have
been left alone.

The two-sided shape is what makes it a check rather than an observation. A
cleanup that removed everything fails on the first survivor; a cleanup that
removed nothing fails on the first assertion. No fixture that merely reported
what it was told could produce both results.

The comment inside it is about the fixture rather than about the code, which is
why it sits in the test and not on `is_channel_name`: the uppercase decoy uses a
*different* nonce, because on a case-insensitive filesystem the same nonce in two
cases would be one file and the test would be asserting nothing. Chapter 6 read
that argument at the function it protects; what this page adds is that the
decision it records is a decision about how to write a test, and that it belongs
to the test's author rather than to the grammar's.

The second test covers the one refusal `discard_abandoned` reaches before it
removes anything: `read_dir` fails on a directory that is not there, and the
error names it. The aggregate refusal at the other end — *could not remove N
abandoned completion channel(s) … remove them by hand* — needs a directory that
lists and an entry that will not delete, and is named by no test. Line 404 is the
module's closing brace and the last byte of `src/channel.rs`.

With it, every byte of the crate is on a page: nine roots, 2,073 lines, nine
chapters that own source, and nothing deferred.

The two halves of this chapter check in opposite directions and neither
interprets a value. The kit holds a consumer's document to obligations about
form and count — it loads, it declares something, its keys expand — and to
nothing about what any key names. The module holds a filename to a grammar of
prefix, length and alphabet, and to nothing about what the name refers to. In
neither is there a point at which the crate could have learned what a value means
and declined to; there was nothing there to decline, which is what the eight
chapters before this one have been showing from the other side.

What remains is to say that once, as a test a reader can carry into their own
code. Chapter 10 owns no source and is where it is stated: where a layer learns
what its values mean, what it costs when the answer is anywhere at all, and which
of these nine chapters proved that here the answer is nowhere.

[Previous: The watch and the escalation](08-the-escalation.md) | [Contents](README.md) | [Next: What passes through](10-what-passes-through.md)
