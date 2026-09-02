# Proving a negative
<!-- book-page id="proving-a-negative" slice="closure-proved" order="4" -->
[Previous: Three steps](03-three-steps.md) | [Contents](README.md) | [Next: What the call reaches](05-what-the-call-reaches.md)

<a id="closure-proved"></a>
## The closure tests

The grammar selects nothing and `run` does three things with it, and both of
those facts are now read. What holds them is the question this chapter answers,
and the answer is a test module: the 84 lines from `#[cfg(test)]` at line 54 to
the closing brace of `crates/grove/src/cli.rs` at line 137, 41% of the corpus,
which is the largest block in the book and the one a reader who wanted the
system rather than the technique can skip. This chapter supplies two of the
three mechanisms in the book's promised outcome. Of the three mechanisms that
can hold an entry point thin, *Orientation* read the one the compiler holds;
the other two are a property a test asserts and a convention a test checks,
and both are asserted here, in the file the grammar lives in.

The actor on this page is the test module. Its input is the clap model of `Cli`
— the `Command` value the derive's `command()` factory hands back, not the
rendered help text and not the parsed value — and its output is two
assertions: that the model lists no subcommand and no argument of this crate's,
and that everything the model lists carries a description. The invariant the
first assertion establishes is a closure property: the human command surface
has *nothing* to select, so it is closed against every verb and flag that could
be added, not only the ones anyone has thought to reject. A negative of that
kind cannot be proved by trying argument vectors — *The surface* showed one
refused, and a second one would prove only that two were — and this module
does not try any. It reads the model and asserts what is absent from it.

The page reads the module in the order of its argument rather than the order
of the file: the module head, which is where the manifest's decision to have no
library becomes visible; the closure test, which is mechanism 2; the reason
both tests read the model and not the text; the described-option test, which is
mechanism 3, and the walk it calls; why that walk exists in two packages and
what the alternatives would have cost; the empty-description case; and then
the worked example, which grows the grammar by one flag and shows what each
assertion prints. The last section closes the three mechanisms.

<a id="the-block"></a>
## Lines 54 to 137, in eleven fragments

The block is the whole test module, and the composite below is it in file
order. It is partitioned along the file's own seams — the four-line head, the
walk's doc comment split at its three paragraphs, the walk's two loops, the
convention test's doc comment and body, and the closure test's doc comment and
its body split at its two assertions — so that each piece can be read beside
the claim it carries. The blank line before each doc comment leads the fragment
that follows it, as everywhere else in this book, and the two closing braces at
lines 136 and 137 end the last fragment.

<!-- fragment «surface-closure-tests» owner="closure-proved" source="crates/grove/src/cli.rs" lines="54-137" parent="source-command-surface" -->
<!-- insert «tests-module-opening» -->
<!-- insert «undescribed-doc-purpose» -->
<!-- insert «undescribed-doc-twice» -->
<!-- insert «undescribed-doc-empty» -->
<!-- insert «undescribed-arguments» -->
<!-- insert «undescribed-subcommands» -->
<!-- insert «describes-test-doc» -->
<!-- insert «describes-test» -->
<!-- insert «closure-test-doc» -->
<!-- insert «closure-test-subcommands» -->
<!-- insert «closure-test-arguments» -->
<!-- /fragment -->

<a id="the-module"></a>
## A test module inside the binary

The head is four lines, and the first import is the one that matters. The
module is compiled only under `cargo test`, it is a child of `cli`, and it
names two things: `Cli` from its parent, and `CommandFactory`, the `clap` trait
the derive at line 8 implemented for `Cli` and whose `command()` method hands
back the model. `use super::Cli` is the line that could not be written outside
this binary target. *Orientation* read the manifest's third comment — there is no `[lib]`, so
`cli.rs` is a module of one binary target and its clap model is reachable only
from inside that target — and this is the consequence: the tests that hold the
grammar closed are a `mod tests` at the bottom of the file that declares it,
not a file under `tests/`, because an integration test would need a library to
import `Cli` from and the package deliberately has none. In the invocation the
book carries this module plays no part; it is compiled out of the binary a
human runs, and its role is to fail the test run of any later commit that
gives `grove` something to select.

<!-- fragment «tests-module-opening» owner="closure-proved" source="crates/grove/src/cli.rs" lines="54-57" parent="surface-closure-tests" -->
````rust
#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;
````
<!-- /fragment -->

<a id="a-closure-property"></a>
## A property, not a list

The second test in the file is read first, because it is the mechanism the
chapter is named for. Its doc comment states the choice: the assertion is a
closure property — *the human CLI has nothing to select* — rather than a list
of rejected verbs. The five names it says the property subsumes were the
human binary's own surface once. `grove do`, `grove migrate` and
`grove retire` were subcommands, `--harness` and `--no-launch` were flags of
the first and third of them, and all five were removed in one release, `17.0.0`,
with the launch routing that `--harness` selected. A test written as their rejection would be
five argument vectors, each asserted to fail, and it would say nothing about
the sixth. The property says the same thing about every vector at once and
names none of them, which is the sense in which it *fails on the next flag
too*: a flag added tomorrow is inside the assertion's scope on the day it is
added, with no edit to the test. In the worked example below this paragraph's
claim is what is exercised: the flag added there was not one of the five, and
the test fails on it anyway.

<!-- fragment «closure-test-doc» owner="closure-proved" source="crates/grove/src/cli.rs" lines="112-117" parent="surface-closure-tests" -->
````rust

    /// Stated as a closure property rather than as a list of rejected verbs: the
    /// human CLI has *nothing* to select. That subsumes `do` / `migrate` /
    /// `retire` / `--harness` / `--no-launch` without naming them, and it fails
    /// on the next flag too — which a list of five rejected argument vectors
    /// would not.
````
<!-- /fragment -->

The body asserts the property in two halves, subcommands first. `Cli::command()`
is the model; `get_subcommands()` iterates the subcommands declared on it, and
the test collects their names and asserts the list is empty. The message names
what a subcommand would contradict — bare `grove` is the whole human lifecycle,
start, resume and finish — and prints the offending names, so a failure reads
as a sentence rather than as a boolean. Nothing in this half filters anything,
and the reason is asymmetric with the second half: clap adds a `help`
subcommand of its own only to a command that already has subcommands, so a
model with none never acquires one, built or not, while the two flags the
second half filters are added to every command clap builds.

<!-- fragment «closure-test-subcommands» owner="closure-proved" source="crates/grove/src/cli.rs" lines="118-125" parent="surface-closure-tests" -->
````rust
    #[test]
    fn the_human_command_surface_has_nothing_left_to_select() {
        let command = Cli::command();
        let subcommands: Vec<&str> = command.get_subcommands().map(|s| s.get_name()).collect();
        assert!(
            subcommands.is_empty(),
            "bare `grove` is the whole human lifecycle; it has subcommands: {subcommands:?}"
        );
````
<!-- /fragment -->

The second half is the arguments. `get_arguments()` iterates every argument on
the model, the test takes each one's id — the derive names an argument after
the field that declares it — and drops the two ids `help` and `version` before
asserting that nothing remains. Those two are the ids clap gives the `--help`
and `--version` flags it adds to a command it builds — the second only when
the command has a version to print, which this one does — and *The surface*
gave the reason for the filter: the flags are the parser's, and the property is
about what this crate declared. The doc comment does not state one difference
between the declared and built models. The model `Cli::command()` returns is
the *declared* model, and clap adds its two flags only when that model is built
— which `parse` and the paths that render help do. This test never asks for a
built model. At this checkout the filter therefore removes nothing: the
unbuilt model of an empty struct lists no argument at all, and the assertion holds on an empty
list. The filter makes the assertion mean the same thing whether
or not the model has been built before it is inspected; the worked example
measures both. The assertion's panic message states the book's central rule —
*launch policy has one home and it is not the command line* — followed by the
ids that contradict it. The two closing braces end the test and the module.

<!-- fragment «closure-test-arguments» owner="closure-proved" source="crates/grove/src/cli.rs" lines="126-137" parent="surface-closure-tests" -->
````rust
        let arguments: Vec<String> = command
            .get_arguments()
            .map(|argument| argument.get_id().to_string())
            .filter(|id| id != "help" && id != "version")
            .collect();
        assert!(
            arguments.is_empty(),
            "launch policy has one home and it is not the command line; `grove` \
             accepts: {arguments:?}"
        );
    }
}
````
<!-- /fragment -->

That is mechanism 2 in full: one function, no argument vector, and an
assertion whose scope is every flag that does not exist yet. It is held by
`cargo test` rather than by the compiler, which is a weaker holder than
mechanism 1 in one exact way — a commit that deletes the test compiles — and
a stronger one in another: the compiler can hold what `main.rs` reaches, and
has no way to hold what `Cli` declares.

<a id="the-model-not-the-text"></a>
## The model, not the text

Both tests read the model and neither reads what `--help` prints, and the
reason is the same for both. A help-output check would run the binary with
`--help` and scan the output for an option row with nothing after it. That
scanner has to reproduce clap's rendering: two
layouts, a short row and the long-help layout that one multi-paragraph
description switches the whole command into, plus the wrapping of each. A
parser that reproduces a renderer is more likely to be wrong than the thing it
checks, and can report a clean result when it fails to recognise an option.
Walking the `Command` asks the same question where it is a fact rather than a
rendering: an argument either has a help string or it does not. The closure
test has the same choice and makes it for the same reason. Scraping the usage
line for an argument placeholder would test the renderer; asking the model
whether it declares any tests the grammar.

This argument is not made in this file. The doc comment of the described-option
test, read next, says it is asserted against the model *for the reasons
`crates/grove-llm/tests/help_surfaces.rs` sets out at length*, and that file's
module documentation is where the two-layouts-and-wrapping argument lives. It
is evidence for this page rather than corpus, and the paragraph above is the
whole of what a reader needs from it.

<a id="every-option-described"></a>
## Every option the binary lists is described

The third mechanism is a convention — every option the binary lists carries a
doc comment — and unlike a closure property a convention is only held while
something checks it. The doc comment records the one time it was not. A
subcommand of the human binary, `grove retire`, shipped a `--no-launch` flag
with no doc comment at all, and clap rendered it as a padded blank row beside
two described options: the flag's name, the column of spaces where the
description goes, and nothing. It compiled, no test looked at the surface it
was on, and it was found by a human running every help surface and reading
them. The handle in parentheses is a grove task handle, and the other copy adds
a second beside the sentence about the discovery; the changelog's entry for
release `16.3.0` records the description being written, and the two test
files are the only places the handles appear. The subcommand and the flag are gone
now, so at this checkout the convention test guards a surface that lists
nothing and passes with an empty list; it becomes non-vacuous when a flag is added,
and the worked example shows it.

<!-- fragment «describes-test-doc» owner="closure-proved" source="crates/grove/src/cli.rs" lines="96-101" parent="surface-closure-tests" -->
````rust

    /// `grove retire --no-launch` shipped with **no doc comment at all** and
    /// rendered as a padded blank row beside two described options
    /// (retire-no-launch-help-k21). Asserted against clap's own model rather than
    /// the rendered text, for the reasons
    /// `crates/grove-llm/tests/help_surfaces.rs` sets out at length.
````
<!-- /fragment -->

The test is three statements. It creates an empty list, hands the model, the
path prefix `grove` and the list to `undescribed`, and asserts the list came
back empty. The path is a string rather than a `Command` because the walk
prefixes every finding with the command path it was found under — `grove`, or
`grove retire` in the shape that failed — so that a finding names a row a
reader can locate in generated help. The message says what a non-empty list means on
the surface a human sees: *these render as blank rows in a generated help
surface*, then one finding per line, indented. In the worked example this
is the assertion whose message names the grown flag.

<!-- fragment «describes-test» owner="closure-proved" source="crates/grove/src/cli.rs" lines="102-111" parent="surface-closure-tests" -->
````rust
    #[test]
    fn the_human_facing_binary_describes_every_option_it_lists() {
        let mut out = Vec::new();
        undescribed(&Cli::command(), "grove", &mut out);
        assert!(
            out.is_empty(),
            "these render as blank rows in a generated help surface:\n  {}",
            out.join("\n  ")
        );
    }
````
<!-- /fragment -->

<a id="the-walk"></a>
## The walk

`undescribed` is the helper both halves of the convention rest on, and its doc
comment's first paragraph is its contract: collect every `<command path> ::
<thing>` in the subtree of `cmd` that appears in a help listing with no
description behind it. The three parameters are the model to walk, the path
prefix findings are reported under, and the list findings are pushed to; the
function returns nothing, and its whole effect is on that list. Rust does not
require it to be declared before the tests that call it; the file puts the
contract first. In the worked example the list comes back holding one string,
and this paragraph is the contract that string satisfies.

<!-- fragment «undescribed-doc-purpose» owner="closure-proved" source="crates/grove/src/cli.rs" lines="58-61" parent="surface-closure-tests" -->
````rust

    /// Collect every `<command path> :: <thing>` in `cmd`'s subtree that appears
    /// in a help listing with no description behind it.
    ///
````
<!-- /fragment -->

The first loop is over arguments. For each one the walk reads both help
strings clap can hold — the short help, which the derive fills from a doc
comment's first paragraph, and the long help, which it fills only for a
comment with more than one paragraph unless an attribute asks for it — and
counts the argument as described if either is present *and* non-empty after
trimming; an argument with neither string yields nothing to `any`, which is
then false. An undescribed argument is pushed as `<path> :: argument `<id>``,
where the id is the field's name. This loop is the one the worked example's
flag reaches: it is the only argument on the model, and it is pushed here.

<!-- fragment «undescribed-arguments» owner="closure-proved" source="crates/grove/src/cli.rs" lines="75-84" parent="surface-closure-tests" -->
````rust
    fn undescribed(cmd: &clap::Command, path: &str, out: &mut Vec<String>) {
        for arg in cmd.get_arguments() {
            let described = [arg.get_help(), arg.get_long_help()]
                .into_iter()
                .flatten()
                .any(|help| !help.to_string().trim().is_empty());
            if !described {
                out.push(format!("{path} :: argument `{}`", arg.get_id()));
            }
        }
````
<!-- /fragment -->

The second loop is over subcommands, and it is the same test applied one level
up: a subcommand's `about` and `long_about` are what a parent's help lists
beside its name, and a subcommand with neither renders the identical blank row
in the parent's listing that an undescribed argument renders in its own. So a
subcommand is checked for its description, pushed as `<path> :: subcommand
`<name>`` when it has none, and then walked recursively under the extended
path, so that its own arguments and subcommands are reported as
`grove retire :: …` rather than as `grove :: …`. The property this makes the
walk assert is *this help lists nothing it does not describe*; a walk over
arguments alone would pass on the same defect with a different name. On this
crate's `Cli` the second loop has nothing to iterate — in the worked example
it iterates nothing, since the grown flag is an argument — and the closure
test above is what keeps it that way; in the other copy, read next, it walks
the agent binary's twelve verbs, the surface *The surface* catalogued.

<!-- fragment «undescribed-subcommands» owner="closure-proved" source="crates/grove/src/cli.rs" lines="85-95" parent="surface-closure-tests" -->
````rust
        for sub in cmd.get_subcommands() {
            let described = [sub.get_about(), sub.get_long_about()]
                .into_iter()
                .flatten()
                .any(|about| !about.to_string().trim().is_empty());
            if !described {
                out.push(format!("{path} :: subcommand `{}`", sub.get_name()));
            }
            undescribed(sub, &format!("{path} {}", sub.get_name()), out);
        }
    }
````
<!-- /fragment -->

<a id="twice"></a>
## The walk exists twice, and what the alternatives would have cost

The second paragraph of the walk's doc comment is the one that argues, and it
argues about a duplication the reader can verify. The same twenty-one-line
function, with the same signature and the same two loops, is in
`crates/grove-llm/tests/help_surfaces.rs`, where it walks the agent binary's
twelve verbs; the two bodies differ in the names of two closure parameters and
in one level of indentation, and in nothing else. The doc comments differ, since
only this copy carries the paragraph read below. The comment explains why the
first mechanism causes the helper to be duplicated. A clap model is reachable
only from the package that declares it. This package is a binary target with no
library, so the copy over there cannot import this `Cli` and the copy here
cannot be imported by anything. Each package therefore carries its own model walk.

The comment names the two alternatives and what each would have cost, and the
table holds them beside the choice that was made so the trade can be read as
one relation rather than three sentences.

| Where the walk could live | What it would cost | Why that cost was refused |
|---|---|---|
| A shared test crate, depended on by both binaries' tests | A package whose whole content is this walk and the assertion around it — the comment's *thirty lines* is a round figure; the other copy's walk and its ten-line assertion helper are thirty-one | A separate package for one helper has no additional responsibility |
| A `[lib]` on this package, so an integration test could reach `Cli` | The package would contain a library of its own, and *the binary is thin* would become a claim about that library's contents as well as `main.rs`'s | That is the property `docs/specs/module-decomposition.md`'s decision 1 made this a crate to keep, and *Orientation* read what it does and does not hold: a package with one binary target and no library has no place inside its own boundary for logic to accumulate |
| **One copy per package** (chosen) | Twenty-one lines twice, and a fix to one copy that must be made to the other by hand | The duplication is visible and the comment names the other copy by path; neither alternative's cost is visible anywhere |

That last cell states the same package-boundary decision from the test's side.
*Orientation* read the manifest's refusal of a `[lib]` and said it was *why* the
closure tests are a `mod tests` inside the binary; this comment is the same
decision seen from the test's side, choosing a duplicated function over the library that would have
removed the duplication. The decision record the comment cites is evidence for
the author — the fact the reader needs is on the page, and it is that the
walk is duplicated because the boundary is real. The fragment is prose and
changes no value; its role in the worked example is that the copy the example
runs is this one, and the copy over the boundary would have to be run
separately to see the same flag on the other binary.

<!-- fragment «undescribed-doc-twice» owner="closure-proved" source="crates/grove/src/cli.rs" lines="62-71" parent="surface-closure-tests" -->
````rust
    /// **This walk exists twice**, here and in
    /// `crates/grove-llm/tests/help_surfaces.rs`, and that is the cost of the two
    /// binaries being two packages: a clap model is reachable only from the
    /// package that declares it, and this one is a binary target with no library
    /// to import from an integration test. The alternative was a shared test
    /// crate for thirty lines, or a `[lib]` on this package that exists only so a
    /// test can reach it — which would give the binary a library to reach into,
    /// and that is the property `docs/specs/module-decomposition.md`'s decision 1
    /// made this a crate to keep.
    ///
````
<!-- /fragment -->

<a id="an-empty-description"></a>
## An empty description is a missing one

The third paragraph is about one line of the walk — `.trim().is_empty()` — and
it is where an argument rests on a detail of what clap stores rather than on
this crate's code. An argument can be given a description
explicitly, with `#[arg(help = "")]`, and clap stores what it is given: an
empty string is present, `get_help()` returns it, and a walk that tested only
for presence would count the argument as described. What the renderer does
with it is print the name and then nothing, which is byte for byte the padded
blank row a missing doc comment produces. The check exists to reject that row,
so it tests the row's cause — a description that is empty after trimming —
rather than the mechanism that produced it. The worked example measures the
case: the empty string and the missing comment fail the same assertion with
the same message.

<!-- fragment «undescribed-doc-empty» owner="closure-proved" source="crates/grove/src/cli.rs" lines="72-74" parent="surface-closure-tests" -->
````rust
    /// A description counts only if it is non-empty after trimming: clap treats
    /// `#[arg(help = "")]` as present, and an empty string renders exactly like
    /// the missing doc comment this check exists to reject.
````
<!-- /fragment -->

<a id="worked-assertion"></a>
## Worked example: one flag, two assertions

The invocation the book carries is `grove` typed bare in
`/work/atlas/crates/gateway/src/`, and this chapter's example is what guards
it: the argv can never grow, and the
tests are what say so. To show them saying it, the grammar is given one flag —
`--harness`, the same name *The surface* showed as refused — and both tests are
run against it. Everything below was measured by copying the crate to a
scratch directory outside the repository, editing line 19 there, and running
`cargo test`; the assertion output is quoted as printed, and the repository's
own bytes were not touched. The struct that replaced the empty one is the
shape the convention test was written against, a flag with no doc comment:

```text
pub struct Cli {
    #[arg(long)]
    harness: Option<String>,
}
```

Run against that struct, both tests fail. Below is what `cargo test` prints
for them, with four things that say nothing about the assertions elided: the
thread ids in the two `panicked at` lines, the `RUST_BACKTRACE` note, the
closing list of the two failed names, and the rerun hint. The line numbers in
the two `panicked at` lines are the grown file's, which is three lines longer
than the corpus: line 109 there is the `assert!` at line 106 above, and 134 is
the one at 131.

```console
$ cargo test -p grove --bin grove
running 2 tests
test cli::tests::the_human_facing_binary_describes_every_option_it_lists ... FAILED
test cli::tests::the_human_command_surface_has_nothing_left_to_select ... FAILED

failures:

---- cli::tests::the_human_facing_binary_describes_every_option_it_lists stdout ----

thread 'cli::tests::the_human_facing_binary_describes_every_option_it_lists' panicked at crates/grove/src/cli.rs:109:9:
these render as blank rows in a generated help surface:
  grove :: argument `harness`

---- cli::tests::the_human_command_surface_has_nothing_left_to_select stdout ----

thread 'cli::tests::the_human_command_surface_has_nothing_left_to_select' panicked at crates/grove/src/cli.rs:134:9:
launch policy has one home and it is not the command line; `grove` accepts: ["harness"]

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Each message is the value its test collected, printed by the assertion that
found it non-empty. `undescribed` collected one string, `grove :: argument
`harness``: the walk's first loop met one argument, read its short help and
its long help, found neither, and pushed the path prefix, the word *argument*
and the field's id. `get_arguments` reported one id, `harness`, and the filter
at line 129 let it through — nothing in it is `help` or `version` — so the
assertion at line 131 printed it as the list `["harness"]`. The table is the
same model read two ways, and it is where the filter's inert-at-this-checkout
claim above is measured rather than asserted: the first row is what the test
inspects, and the second is what the same struct's model reports once
`build()` has been called on it.

| Model | `get_subcommands()` | `get_arguments()` ids | After the `help`/`version` filter |
|---|---|---|---|
| `Cli::command()`, as the tests read it | none | `harness` | `harness` |
| The same, after `build()` | none | `harness`, `help`, `version` | `harness` |

The filter changes nothing in the first row and removes exactly two ids in the
second, and the assertion fails identically on both. That is what the filter is
for: the property is about what the crate declared, and it reads the same
whether or not clap has added its own two flags to the model yet. The two
built-in flags also carry descriptions — `Print help` and `Print version` — so
the convention test would pass them on a built model too.

The blank row the convention test talks about is not visible in a test run, so
the grown binary was built and asked for its help. This is the row shape `--no-launch`
once rendered under `grove retire`, reproduced under `grove` with a different
name and with a value placeholder that a boolean flag does not carry; the
trailing spaces after the placeholder are in the output.

```console
$ grove --help
Grove: hierarchical workstream tool for AI agents

Usage: grove [OPTIONS]

Options:
      --harness <HARNESS>  
  -h, --help               Print help
  -V, --version            Print version
```

Two things changed against *The surface*'s transcript, and both are what the
tests exist to notice before a human does: the usage line now carries
`[OPTIONS]`, because there is something to select, and the option list has a
row with a name, a placeholder and a description column of spaces — the two
rows that were already there have moved their descriptions right to share
that column. `--help` and `--version` are described, because clap describes
what it adds.

The same flag was then measured in the two other shapes the chapter has
argued about — with an explicitly empty description, and with a doc comment —
and the table holds the three results together, because the point of running
three is the relation between them and not any one row.

| The flag, as declared | `describes_every_option` | `nothing_left_to_select` | `--help` renders |
|---|---|---|---|
| `#[arg(long)]` and no doc comment | fails: `grove :: argument `harness`` | fails: `accepts: ["harness"]` | the padded blank row above |
| `#[arg(long, help = "")]` | fails, with the identical message | fails, with the identical message | byte for byte the same row |
| `/// The harness to launch every session with.` and `#[arg(long)]` | passes | fails: `accepts: ["harness"]` | the row, with the description after it and without the comment's trailing period, which the derive drops |

The second row is the empty-description case measured: the empty string and
the missing comment are indistinguishable on every surface — the test's
message, the exit, and the rendered row — which is what `.trim().is_empty()`
buys and what a presence check would have missed. The third row is the two
mechanisms shown independent. A described flag satisfies the convention and
still violates the property, and the property is the one that matters for the
outcome: a described `--harness` duplicates the configuration file's ownership
of launch policy just as an undescribed one does. Mechanism 3 checks that what
the surface lists is described; mechanism 2 checks that it lists nothing. On this
binary the second makes the first vacuous, and the first is kept anyway,
because deliberately relaxing the second to admit a flag makes a blank row
possible again.

<a id="three-mechanisms-complete"></a>
## Three mechanisms, complete

*Orientation* opened a table whose three mechanism rows named where each
would be read, and read the first; this page has read the other two, and the
table can now be closed. Each mechanism is stated
with what holds it, what it can and cannot hold, and where in these 204 lines
it is proved.

| # | Mechanism | Held by | What it cannot hold | Proved at |
|---:|---|---|---|---|
| 1 | The entry point can reach only what the library publishes | the compiler, at the crate boundary | What the crate declares on its own surface | `Cargo.toml`, no `[lib]`, one target — *Orientation* |
| 2 | The human surface has nothing left to select | `the_human_command_surface_has_nothing_left_to_select`, against the model | A commit that deletes the test | `cli.rs` 112–137 — this page |
| 3 | Everything the surface lists is described | `the_human_facing_binary_describes_every_option_it_lists`, against the model | A description that is present, non-empty and wrong | `cli.rs` 58–111 — this page |

The transferable test is the book's stated outcome, and it can be applied now.
Given an entry point of your own, ask what is left for an argument to select,
and then ask which of the three holds the answer: whether the binary is a
package that can reach only what its library publishes, whether a test asserts
the surface closed against everything rather than against a list, and whether
a test walks the model for the convention a human would otherwise check by
reading every help surface. An entry point held by none of the three is thin
today and free to stop being thin tomorrow. *What the call reaches* names the
modules behind `grove_loop::run` and applies this test back across the map,
and it is where the book ends.

[Previous: Three steps](03-three-steps.md) | [Contents](README.md) | [Next: What the call reaches](05-what-the-call-reaches.md)
