# The guaranteed core
<!-- book-page id="the-core" slice="too-late-to-say-later" order="19" -->
[Previous: Which files take part](18-which-files.md) | [Contents](README.md) | [Next: The loop](20-the-loop.md)

<a id="too-late-to-say-later"></a>
## The rule: a sentence rides `${prompt}` only if its failure mode is one the skill cannot repair

Chapter 18 read the file that decides *which* document names the program a launch
runs. This one reads what that launch is handed. `${prompt}` is the required slot
of the four chapter 18 enumerated, and this 245-line module is the whole of what
goes in it.

> **A sentence rides `${prompt}` only if its failure mode is one the skill cannot
> repair — because by the time the skill could speak, the moment has passed.**
> Three driver-authored parts and no methodology: an imperative naming the
> `grove-<kind>` skill, the runtime facts, and grove's own signalling contract.

The interesting word in that rule is **fact**, and the module closes on it rather
than leaving it to taste. A driver fact is a *launch-varying value the
methodology cannot know at authoring time*; its static meaning, and every
normative consequence of it, stay in the skill. Without that closure the test
admits anything a reader thinks important, and importance is unbounded — which is
why the [glossary entry](../../../CONTEXT.md#guaranteed-core) lists importance,
frequency and brevity as the three criteria it exists to refuse.

The closure is what makes the module small, and the smallness is a result rather
than a target. Every sentence that would have explained what a value *means* went
to the skill; what is left states values and stops.

The carried example reaches the point where a selected leaf becomes a session.

```text
  the walk has chosen a leaf              kind `impl`, handle `plan-k1`
  the lease has admitted the driver       chapters 16, 17
  the configuration has resolved          chapter 18 — `${prompt}` is required

  compose(Mandate { handle, kind, workspace, version })

    part 1  load `grove-impl`, bare and namespaced   -> the first action
    part 2  handle, stated VCS, version              -> three launch-varying values
    part 3  the signalling contract                  -> the last action

  -> one String, ~1.9 KiB, and the driver never looks inside it again
```

Three parts in the session's own timeline order — what to do first, what is true
now, how to end — and nothing between them but a blank line.

This is the outcome's third question in its most direct form. *On the way out — the
policy: what does this layer choose that nothing beneath it could have
defaulted?* Chapter 18 answered it with four slot **names**. This chapter answers
it with the contents of the one slot that is required, and the cost the outcome
names — *a chosen value must be stated where a reader can find it, and the layer
must not restate what the layer above owns* — is visible here as a rule with a
test beside it rather than as a principle. The pin is
`the_runtime_facts_restate_no_rule_the_skill_owns`.

<a id="what-the-instruments-see"></a>
## What the instruments see here, and the one thing they do not

**171 of the 245 lines are comment prose**, counting lines whose first non-space
characters are `//`. That is 69.8%, and counting all thirteen roots whole under
the same rule it is the **second densest in the crate**, behind `driver.rs` at
73.7% and ahead of `complete.rs` at 64.6% — an enumeration rather than a reading
of the brief's table, which splits the five roots that carry inline tests and so
ranks a different set of things. The structure brief's table says 69% here; the
count is given here rather than the percentage, because the rule behind the
brief's figures is not stated: chapter 15 gave its three roots' counts rather
than their percentages, and chapter 16 is the page that records why — two of the
brief's figures, `verbs.rs` and `driver.rs`, come out a point away under this
rule and a rounding difference is not a defect in either. Here the figure rounds
to 70 and truncates to 69, and nothing turns on which.

**All 171 are `///` or `//!`. The root has no plain `//` comment, and no
`#[cfg(test)]` module.** Both of the blind spots this book has been working
around are therefore absent, as they were in chapter 18 — which is worth stating
plainly, because it was forecast otherwise. Chapter 18 ends its own instrument
section saying *the next two chapters go back to the general case*, and the
finding it handed forward puts it more strongly still: **chapters 19 and 20 are
not the exception this root was.** The reasoning behind that was about
`loop_driver.rs`, whose inline test module `cargo doc` cannot see at all, and it
still holds for chapter 20. It does not hold here, and only the measurement
settles which.

So the reading here means something in both directions. `cargo doc --no-deps
--document-private-items -p grove-loop` reports thirty warnings over the crate and
names this file in exactly **one** of them — the unresolved intra-doc link at line
28. That the instrument reaches the rest is a measurement rather than an
assumption: planting a broken link inside a `///` docblock in this file takes the
crate to thirty-one and names the new line, so the silence over the other 170
comment lines is silence about links that resolve.

**One thing it still cannot see, and this root has none of them.** Chapter 18's
last finding was that `cargo doc` says nothing at all about a Markdown link with
an explicit URL target. This block contains no such link: every citation in it is
a backticked path or a record named in prose, which is this file's convention
throughout. So the two broken addresses this chapter does adjudicate are of two
other kinds — one the instrument reports, and one no instrument in this repository
looks for at all.

**The block holds no test, and that is why it takes *do not restate*.** There is
no `#[cfg(test)]` module and no `#[test]` function in the 245 lines, so the book's
*supply the claim* obligation has nothing here to attach to; chapter 19 is not on
the structure brief's list for it. At 69% comment prose the instruction is the
third one — the comments already argue, the fragment graph quotes them verbatim,
and prose here connects arguments, names the test, and stops.

**The evidence is sixteen tests, and the brief pins four of them.**
`crates/grove-loop/tests/prompt.rs` is 614 lines and sixteen `#[test]` functions;
it is `tests/`, so it is evidence and not a root, cited by name here and
reproduced nowhere. The count is taken from the file rather than from any prose
list of it. Chapter 17's block turned out to hold eighteen tests where the
structure brief named nine, and the discipline that correction left is to count
the block before writing a word.

<a id="the-file-that-fails"></a>
## The file the whole book's baseline is made of

Chapters through this book have measured coverage by mutating the crate in a copy
of the workspace and diffing the failures against an unmutated control of the same
copy, and **seven of them — chapters 11 to 17 — cite the same environmental
baseline**: 558 tests, 547 passed, eleven failed before any mutation, in
`crates/grove-loop/tests/prompt.rs`. That file is this chapter's evidence, so
this is the chapter that owes the reason.

The reason is `Mandate`'s third field. The suite scaffolds no fixture tree — its
own comment says a resolved workspace is *a value the prompt only renders* — and
resolves one from the repository root instead, unwrapping the result on the
stated ground that the repository is a jj workspace. In a copy it is not, so
every test that composes a prompt dies in the fixture rather than in an
assertion, with `Refusal(NotAWorkspace { … })`. Nothing about composition is
under test in a copy; the tests are refused entry to it. That is the same *state
unknown is not absence* shape chapter 18 read in the delta search, arriving from
the other side: here the fixture is entitled to fail closed, because a prompt
that cannot state the version control has nothing to state.

**Ten of the eleven have that cause, and the eleventh has another** — which
chapter 11 recorded exactly, and which the summary carried forward since has not.
Ten of this file's sixteen tests reach `compose`: nine through it or
`compose_with` directly, and one through `signalling_contract()`, which composes
an `impl` prompt and slices from `**Signal.**`. Those ten are the ten that die in
the fixture. The eleventh, `the_namespace_is_the_shipped_plugin_entrys_declared_name`,
composes nothing: it reads `.claude-plugin/marketplace.json`, which the copy
recipe does not carry. Chapter 10, whose copy was scoped differently — 626 tests
over three crates — reports **ten** rather than eleven for the same reason, and
the two numbers have never disagreed about anything but what the copy contained.

The distinction is worth more than the arithmetic, which is why losing it
matters. A control taken at a flat eleven records the marketplace test as
*already failing*, and a mutation that broke it — anything touching `PLUGIN`,
which that test reads five times — would then be invisible, its newly-failing set
empty against a baseline that had already written the test off. A control wrong
in that direction hides an observer, which is the same failure mode the `cargo
build -p grove --bins` step exists to prevent at seventeen. Copy
`.claude-plugin/` with the rest and the baseline is a clean ten, all one cause;
the other six tests read the shipped plugin, its marketplace entry or the spine,
compose nothing, and pass wherever the repository is copied whole.

<a id="the-block-declared"></a>
## The block, declared

The 245 lines are one composite whose children are the file's own items in file
order. The order is already the argument: the header states the rule, the two
names it needs, then the three parts **in the order they appear in the prompt**,
then the values one launch varies, then the composition that joins them, and last
the one derived value the module computes for itself.

<!-- fragment «the-prompt-core» owner="too-late-to-say-later" source="crates/grove-loop/src/prompt.rs" lines="1-245" parent="source-prompt" -->
<!-- insert «core-header» -->
<!-- insert «core-imports» -->
<!-- insert «core-plugin» -->
<!-- insert «core-skill-name» -->
<!-- insert «core-load-instruction» -->
<!-- insert «core-runtime-facts» -->
<!-- insert «core-signalling-contract» -->
<!-- insert «core-mandate» -->
<!-- insert «core-compose» -->
<!-- insert «core-stated-vcs» -->
<!-- /fragment -->

<a id="the-header-and-the-closure"></a>
## The header, and the word the test is closed on

Forty lines, and they are the chapter. The module states its rule, closes it,
says what the closure costs, and names the one coupling it cannot close.

<!-- fragment «core-header» owner="too-late-to-say-later" source="crates/grove-loop/src/prompt.rs" lines="1-41" parent="the-prompt-core" -->
````rust
//! The **guaranteed core** — the whole of `${prompt}`.
//!
//! Three driver-authored parts and no methodology: an imperative naming the
//! `grove-<kind>` skill, the runtime facts, and grove's own signalling contract
//! (`docs/specs/module-decomposition.md`, decisions 9 and 10). What earns a
//! place here is settled by the **too-late test**: a sentence rides `${prompt}`
//! only if its failure mode is one the skill cannot repair, because by the time
//! the skill could speak the moment has passed. Three shapes pass — a fact only
//! the driver holds, the instruction to open the skill, and the mechanism the
//! session's last action drives — and the test is closed on the word *fact*:
//!
//! > A driver fact is a **launch-varying value** that the methodology cannot
//! > know at authoring time. Its static meaning, and every normative consequence
//! > of it, stay in the skill.
//!
//! That closure is why the runtime facts below are bare values. *This selection
//! is authoritative*, *do not probe for the version control* — both are rules
//! with counterparts in the shipped spine (`plugins/grove/skills/grove/SKILL.md`,
//! *What the driver settled before your session*), and restating either here
//! would be the second source this design exists to avoid.
//!
//! **The prose drift surface is zero bytes, and it is now zero by construction
//! rather than by assertion.** This module reads nothing: the whole template is
//! driver prose, so there is no embedded content for a Rust literal to drift
//! from and no per-kind content decision to keep in step. That is what
//! `prompt-names-the-kind-k18` deleted — `reference_file`'s nineteen-to-ten
//! `match`, `ending_file`'s nineteen-to-two one, the `${locations}` slot, and
//! with all three this module's dependency on [`crate::methodology`]. The driver
//! now interprets a kind **nowhere**: it renders the kind's own label into a
//! skill name and stops.
//!
//! **One coupling remains, and it is not closeable from here.** The prompt names
//! a `grove-<kind>` skill, and that skill has to exist in the installed plugin.
//! Nothing in this workspace can check the *installed* set — a plugin is
//! installed per machine, by a marketplace the binary does not read — so the
//! assertion available is over the shipped set, and it lives in
//! `tests/prompt.rs`. The methodology's own answer to the residue is stated
//! where the cost falls: a kind exists **iff** a skill of that name exists
//! (`plugins/grove/skills/grove/SKILL.md`), and grove states the version it is
//! so a session can see which plugin it needs.

````
<!-- /fragment -->

Read the block quotation at lines 12 to 14 as the load-bearing sentence. Every
other statement in the file follows from it, and two of them are worth checking
rather than reading.

**"This module reads nothing" is structural, and it is stronger than it looks.**
There is no `fs`, no `include_str!`, no path and no I/O anywhere in the 245
lines — and, enumerated, there is also **not one conditional**. The words `if`,
`else` and `match` occur twice each and **all six are prose**: `if` and `else` in
comment text and inside the prompt's own wording, and the two `match`es only in
the header sentence naming the two this module no longer has. A module with no
branch
cannot make a per-kind content decision, which is the property the header claims
and the shape of `compose` at the end of the chapter confirms.

**The two rules it names as deliberately absent are in the spine, and a test
holds them there.** *This selection is authoritative* and *do not probe for the
version control* both have counterparts under *What the driver settled before
your session* in `plugins/grove/skills/grove/SKILL.md`, where the spine states
that the mandate is authoritative and that the stated VCS is definitive.
`the_spine_carries_the_two_rules_the_core_sheds` is the assertion, and it is one
of the six tests in the file that never compose anything — which is why it is
also one that passes in a workspace copy with no jj repository in it.

**What this block cites, it cites correctly on all but one count — and only the
enumeration shows which.** Counted across all 245 lines, the block carries
**eighteen** citations to a document or an anchored record: seven to
`docs/specs/module-decomposition.md`, naming four distinct decisions (4, 5, 9 and
10); three to `plugins/grove/skills/grove/SKILL.md`; two each to
`plugins/grove/README.md`, `crates/grove-loop/tests/prompt.rs` and
`docs/ARCHITECTURE.md` anchors; one to `docs/research/wording-micro-test.md`; and
one to a methodology rule id. Two grove task keys — `prompt-names-the-kind-k18`
and `open-kind-k20` — date changes rather than cite arguments, and one intra-doc
link points at a module; both fall outside that count and are read below.
**Seventeen of the eighteen resolve and support the sentence that cites them** —
decision 9 carries `Mandate`'s four fields and `compose`'s signature as
written source, and its own paragraph on the provisioning gap is nearly this
header's wording. The one that does not is at line 234, and the last section of
this chapter reads it. That census is worth taking because chapter 18 found two
addresses in its block that did not resolve and chapter 16 found a name that
resolved to nothing, and because this file cites more heavily than either.

**The intra-doc link is a separate defect, and `cargo doc` is the instrument that
finds it.**
`[`crate::methodology`]` is one of the crate's five unresolved intra-doc links
and the only one in this block. The prose around it is true: a `methodology`
module is exactly what `prompt-names-the-kind-k18` deleted, along with the two
`match`es and the `${locations}` slot the same sentence lists. So the sentence is
right and the link is broken *because* the sentence is right — rustdoc renders the
text literally and warns, and nothing in `scripts/check.sh` runs `cargo doc`, so
nothing fails. `unresolved-doc-links-k151` holds the repair for all five and is
deferred behind this book, because fixing this line before this page existed would
put corrected bytes in the source ahead of the page that has to reproduce them.

**The coupling the header ends on is the one it cannot close, and the honesty is
the point.** The prompt names a `grove-<kind>` skill; whether that skill is
*installed* is a property of a machine and a marketplace the binary never reads.
What a `cargo test` can reach is the *shipped* set — this working tree's own
`plugins/grove` — and that is where the assertion lives:
`every_kind_names_a_skill_the_plugin_ships` walks the plugin's own skill
directories, asserts that every one but the bare `grove` spine is a
`grove-<kind>`, and then checks that the name `skill_name` renders for each of
those kinds is a directory that exists. It composes nothing — the coupling is
between the renderer and the directory listing, not between a prompt and a file —
and `the_skill_set_claim_fails_on_a_kind_the_plugin_does_not_ship` is its
control, so the sweep is shown capable of failing. Neither test can say anything about an
installed set, and the header does not pretend otherwise: it points at the
methodology's own answer — a kind exists **iff** a skill of that name exists — and
at grove stating its version so a session can see which plugin it needs, which is
decision 10's inversion arriving one paragraph early.

<a id="what-it-imports"></a>
## Three types, and no runner

Two `use` lines, three types, and the module's whole outward dependence is in
them.

<!-- fragment «core-imports» owner="too-late-to-say-later" source="crates/grove-loop/src/prompt.rs" lines="42-45" parent="the-prompt-core" -->
````rust
use crate::{Handle, Kind};

use jj_workspace::Workspace;

````
<!-- /fragment -->

Two of the three are chapter 3's, and the third is another crate's. What is
absent is the striking part: `keyed_launch` appears nowhere in this module,
though the string this module builds exists only to be substituted into one of
that crate's templates. Chapter 18's imports named six `keyed_launch` types on one
line because that module hands the runner a vocabulary and receives an `Argv`.
This one hands the runner nothing — it returns a `String` to its caller, and
chapter 20's `session_prompt` is what puts it in a slot.

<a id="one-name-twice"></a>
## One name, and why it is stated once

The first of the module's two names, and the only `const` here that is not one
of the prompt's three parts.

<!-- fragment «core-plugin» owner="too-late-to-say-later" source="crates/grove-loop/src/prompt.rs" lines="46-54" parent="the-prompt-core" -->
````rust
/// The plugin the kind skills ship in, and the namespace a harness that
/// namespaces plugin components registers them under.
///
/// Stated once here because it appears twice in one sentence — bare and
/// namespaced — and the two spellings must name the same target.
/// `plugins/grove/README.md`, *The token a prompt names*, is the decision;
/// `tests/prompt.rs` holds this against the shipped plugin's own manifest.
pub const PLUGIN: &str = "grove";

````
<!-- /fragment -->

The justification is a count, and the count is checkable at `compose`: `PLUGIN`
is reached twice on the way to the load instruction — once through `skill_name`,
which builds `grove-<kind>`, and once as the `{plugin}` substitution that spells
the namespace. Two reads of one constant in one rendered sentence is exactly the
case a named constant is for, and the comment states the reason as *the two
spellings must name the same target* rather than as tidiness.

**`pub`, and reached from nowhere in production.** Enumerated across the
workspace, `PLUGIN` and `skill_name` have no consumer outside this file except
`crates/grove-loop/tests/prompt.rs`, which reaches them nine and two times; and
`lib.rs` re-exports only `compose` and `Mandate` from this module, not these two.
What makes the tests' access legal is `pub mod prompt` in the library root. This
is chapter 18's `DELTA_FILE_NAME` shape — public surface with no production
caller — arriving for a different reason: there the `pub` bought an intra-doc link
from the module's own header, here it buys an out-of-process suite the ability to
name the plugin without hard-coding it.

That matters more than it sounds, because one of those tests,
`the_namespace_is_the_shipped_plugin_entrys_declared_name`, reads
`.claude-plugin/marketplace.json` and asserts the registered plugin's declared
`name` is this same constant. The namespaced spelling every prompt writes is
`<name>:grove-<kind>`, so if the marketplace entry declared a different name the
namespaced half of every load instruction would address nothing — and no
composition test would notice, because composition would still render the
constant it was given.

<a id="a-rendering-not-an-interpretation"></a>
## A rendering, not an interpretation

The second name, and the only function in the file besides `compose` and the
value it computes at the end.

<!-- fragment «core-skill-name» owner="too-late-to-say-later" source="crates/grove-loop/src/prompt.rs" lines="55-65" parent="the-prompt-core" -->
````rust
/// The skill a session of `kind` is told to load: the plugin's per-kind skill,
/// named for the kind's own label.
///
/// **This is a rendering, not an interpretation.** Nothing here branches on
/// which kind it is, which is the whole of what decision 5 asked for: grove
/// names a kind only where grove writes the leaf, and the last two places that
/// *interpreted* one were the two `match`es this replaces.
pub fn skill_name(kind: &Kind) -> String {
    format!("{PLUGIN}-{}", kind.label())
}

````
<!-- /fragment -->

Three lines of body, and the doc comment above them is longer than the function
because the argument is about what the function does *not* do. Decision 5 of
`docs/specs/module-decomposition.md` is titled *Grove names a kind only where
grove writes the leaf*, and this is the last place in the driver that had to be
changed for that to be true.

The claim generalises, and it holds under enumeration. `label()` is reached six
times in `loop_driver.rs` — once to resolve which configuration file a template
came from, once to expand the template, three times as a diagnostic's argument,
and once on `Kind::finish()` rather than on a selection — and none of the six is
a branch. Together with this module's zero conditionals, *the driver interprets a
kind nowhere* is a statement about code that can be checked rather than a
statement of intent. The crate does still know that two kind tokens are reserved,
but that is chapter 4's grammar refusing a name, not the driver reading one.

<a id="part-one"></a>
## Part 1 — the load instruction, and the element that was measured

Part 1 is thirty-two lines of doc comment over a nine-line constant — the third
longest argument in the file, behind the module header's forty and part 3's
thirty-four — because it is the one part of the prompt whose wording was tested
rather than reasoned.

<!-- fragment «core-load-instruction» owner="too-late-to-say-later" source="crates/grove-loop/src/prompt.rs" lines="66-107" parent="the-prompt-core" -->
````rust
/// **Part 1 — the load instruction.** First, because it is the first action.
///
/// The wording descends from `docs/research/wording-micro-test.md`, *The winning
/// wording*, and carries the two elements of it that survive the plugin:
///
/// * **One imperative naming one target.** The driver resolved the kind before
///   the session existed, so the session performs no selection and has nothing
///   to defer. This is the element the test measured as load-bearing: the
///   control opened `SKILL.md` in 9 of 10 sessions and reached its kind's
///   *procedure* only after starting work, in every session of both arms. The
///   test's imperative named two targets because the methodology was one skill
///   with a routing table; the plugin gives each kind its own skill, so one
///   target is now the whole procedure and the property is preserved rather than
///   weakened.
/// * **An ordering clause that enumerates the tempting alternatives.** The
///   enumeration is what makes an ordering clause bite; "read this first"
///   without it is advice.
///
/// **Both spellings of the one target, in one imperative** — bare, and
/// namespaced for a harness that namespaces plugin components. That is
/// `plugins/grove/README.md`'s decision, taken so grove branches on no harness
/// and grows no registry to branch on; a session reads one line and uses
/// whichever spelling its harness offers. Naming two *spellings* is not naming
/// two *targets*: there is still nothing to select.
///
/// **The third measured element is gone with the thing it described.** The
/// provisioned directories rode this instruction by absolute path, and no
/// directory is provisioned any more. The gap is recorded rather than argued
/// away (`docs/specs/module-decomposition.md`, decision 9): a harness with a
/// skill-loading affordance is unaffected, one without loses its fallback, and
/// the reopen condition is a session that cannot reach the methodology by the
/// affordance alone.
const LOAD_INSTRUCTION: &str = "\
**Load the `{skill}` skill now** — on Claude Code, where plugin skills are
namespaced, that is `{plugin}:{skill}`. Your kind is `{kind}`; Grove resolved
that before this session existed, so there is no selection for you to make.

Do this **first**, before anything else you might reach for: before reading the
task file, before running any `grove-llm` verb, before looking at `.grove/`,
before inspecting the working tree, and before answering a question.
";

````
<!-- /fragment -->

The provenance is unusual for this crate: the wording was **measured**, not
argued, and `docs/research/wording-micro-test.md` is the record. Its finding is
sharper than *sessions did not read the skill*. The control opened `SKILL.md` in
9 of 10 sessions and every control session in both arms eventually reached its
kind's reference file; what failed, in **every** session, was reading the
*procedure* before starting work — four of them bundling the read into the same
shell call as a `grove-llm` verb or a `.grove/` read. So the defect was deferral,
not omission, and one imperative naming one target is what removes the thing to
defer.

Two of the three measured elements survive in the 456 bytes above, and the
comment is careful about what changed under the third. The micro-test's winning
wording named *two* targets — the skill and its reference file — because the
methodology was then one skill with a routing table; the plugin gives each kind
its own skill, so one target is now the whole procedure. That is the property
preserved under a smaller instruction, and the comment says so rather than
claiming the test measured what is written here.

The element that left is visible by comparing the two texts. The winning wording
carried a third paragraph naming the provisioned skill directories by absolute
path, as a fallback for a harness with no skill-loading affordance. Nothing is
provisioned any more, so the paragraph went with the thing it described, and the
gap is **recorded** — a harness with the affordance is unaffected, one without has
no fallback, and the reopen condition is a session that cannot reach the
methodology by the affordance alone. That sentence is decision 9's own, nearly
word for word, which is the file's habit: where a record already argues a cost, the
comment cites and does not re-argue.

`the_load_instruction_says_the_kind_was_already_resolved` is the test, and it
asserts on the rendered instruction per shipped kind rather than on the template.

<a id="part-two"></a>
## Part 2 — three values, and the tail that left

Part 2 is the shortest of the three, and the closure on *fact* is why.

<!-- fragment «core-runtime-facts» owner="too-late-to-say-later" source="crates/grove-loop/src/prompt.rs" lines="108-125" parent="the-prompt-core" -->
````rust
/// **Part 2 — the runtime facts.** Three launch-varying values and no normative
/// tail; see this module's header for why the tail left.
///
/// The version is the third, and it is what
/// `docs/specs/module-decomposition.md`'s decision 10 inverts the compatibility
/// check onto: the machinery states what it is, and the methodology decides
/// whether that is good enough. A value in the prompt needs no command to
/// succeed and cannot fail — where a verb would need the CLI on `PATH` and would
/// fire only if the session thought to run it, which is exactly the deferred
/// read the micro-test measured.
const RUNTIME_FACTS: &str = "\
Grove mandate: the leaf selected for this session is `{handle}`.

Version control: {stated_vcs}.

Grove version: {version}.
";

````
<!-- /fragment -->

The whole template is 124 bytes and three substitutions, which is what the
closure on *fact* buys: a launch-varying value each, and not one sentence saying
what any of them means. The handle is chapter 3's projection, the stated VCS is
computed at the bottom of this file, and the version is the workspace's own.

The third is where a design decision is visible in a line of text. Decision 10 —
*Grove publishes its version in the prompt* — inverts a compatibility check:
rather than the methodology asking the machinery whether it is new enough, the
machinery states what it is and the methodology decides. The comment gives the
mechanical reason for choosing a value over a verb, and it is the micro-test's
finding again in another costume: a verb would need the CLI on `PATH` and would
fire only if the session thought to run it, which is the deferred read the test
measured. A value in the prompt needs no command to succeed and cannot fail.

`the_runtime_facts_restate_no_rule_the_skill_owns` is the structure brief's pin
for this chapter and for the outcome's third question, and
`the_prompt_publishes_the_release_version_the_version_flag_renders` holds the
version against what `grove --version` prints — the fallback the decision keeps,
explicitly not as the mechanism.

<a id="part-three"></a>
## Part 3 — one text for every kind

Part 3 is thirty-four lines of argument over an eighteen-line constant, and
every paragraph of the argument is about something the text deliberately does
not say.

<!-- fragment «core-signalling-contract» owner="too-late-to-say-later" source="crates/grove-loop/src/prompt.rs" lines="126-178" parent="the-prompt-core" -->
````rust
/// **Part 3 — grove's own signalling contract.** Last, because it describes the
/// last action.
///
/// **One text for every kind**, where two embedded files used to serve
/// them, and the trade is deliberate rather than an oversight. What the driver
/// holds — that it is watching a named file, that writing it ends the session,
/// that not writing it stops the loop — is a fact about the process tree no
/// skill can discover and none can repair once the moment has passed, so it
/// rides the one channel a session cannot skip. Which ending a kind takes is a
/// rule about that kind, and the shipped spine already assigns it: *whether it
/// passes the done flag when it signals … is inline in that kind's own
/// `grove-<kind>` skill* (`plugins/grove/skills/grove/SKILL.md`).
///
/// **The skill's ending is the sentence's object, and the default is
/// subordinate to it.** That is load-bearing rather than stylistic. The
/// eighteen-and-one split this replaces meant a `finish` prompt never carried
/// *run `grove-llm complete`* as an imperative — the instruction that, taken by
/// the one session that may have just deleted the task tree, relaunches the loop
/// onto a torn-down grove, and whose stated precondition (*the task is retired
/// and committed*) a completed teardown satisfies exactly. A default stated as
/// the main verb would put that imperative back in every prompt and rest
/// its repair on the skill being read, which is the argument this module's
/// too-late test rejects. Stated as *ordinarily*, subordinate to the kind's own
/// ending, no prompt ends on a bare imperative for the wrong action.
///
/// **The residue that remains, stated rather than argued away.** A `finish`
/// session whose skill is missing or unread still meets *ordinarily it is
/// `grove-llm complete`* and nothing contradicting it, where the old prompt
/// alone was fail-safe for that kind whatever was installed. Decision 10 already
/// accepts that a session can be launched pointing at a skill that is not
/// installed, so the two residues compound. What is bought is that grove
/// interprets no kind; what is paid is that `finish`'s safety now depends on
/// `grove-finish` being present, like every other rule. The reopen condition is a
/// `finish` session observed signalling `complete` after a teardown.
const SIGNALLING_CONTRACT: &str = "\
**Signal.** Your kind's skill states how this session ends, and that ending is
your **last action — then do nothing else**. Ordinarily it is **`grove-llm
complete`**, once the task is retired and committed (and any parent-chain
cascade is settled and included). That is how the self-driving loop ends this
session and starts the next task with fresh context: the verb only writes the
relaunch flag to a signal file (`GROVE_SIGNAL_FILE`) and returns. Ending the
session is the **loop driver's** job, not the verb's — the driver launched this
session and is watching for the signal file while it runs, so it applies grace →
SIGTERM → kill-grace → SIGKILL to its own child once the file appears
(driver-side watcher: the driver can always signal its child, unlike an in-agent
self-kill, which some harness sandboxes silently deny). Run outside a `grove`
loop (no `GROVE_SIGNAL_FILE`) the verb is a safe no-op that just tells you to
exit manually. A session that ends *without* signalling stops the loop instead —
a crash or a Ctrl-C is one such ending, and for at least one kind it is a stated
one — so the signal is what separates a task you finished from a session that
died.
";

````
<!-- /fragment -->

At 1,175 bytes this is the largest of the three parts — roughly 62% of a composed
prompt — and it is also the only one that does not vary at all. That inversion is
the design: what the driver holds is a fact about the **process tree**, and no
skill can discover it or repair it once the moment has passed, so it rides the one
channel a session cannot skip. Which ending a kind takes is a rule about that
kind, and the spine already assigns it.

**The grammar of the last sentence is load-bearing, and the comment says why.**
The kind's own ending is the object; *ordinarily it is `grove-llm complete`* is
subordinate to it. Stated the other way round, every prompt would end on a bare
imperative to run `complete` — including a `finish` session's, whose skill may
have just deleted the task tree, and for which that imperative relaunches the loop
onto a grove that no longer exists. `no_prompt_states_the_stop_flag` guards the
adjacent case: `--done` is an ending only `finish` takes, and no prompt names it.

**The residue is stated, not argued away, and it compounds with decision 10's.**
A `finish` session whose skill is missing or unread meets the ordinary default and
nothing contradicting it, where the eighteen-and-one split this replaces was
fail-safe for that kind whatever was installed. What is bought is that grove
interprets no kind; what is paid is that `finish`'s safety now depends on
`grove-finish` being present, like every other rule. The reopen condition is
written down: a `finish` session observed signalling `complete` after a teardown.
`the_finish_skill_carries_the_three_endings_the_prompt_no_longer_routes` is the
test that the skill really does carry what the prompt stopped carrying — and it is
one of the six that need no jj tree, because it reads the shipped skill and
composes nothing.

**What the contract's own test establishes, and what it would still pass under.**
`the_signalling_contract_states_the_mechanism_and_defers_the_ending` slices part 3
out of a composed `impl` prompt and requires three clauses in it: that the kind's
skill states the ending, that the signal is the last action, and that a session
ending without signalling stops the loop. Its failure message claims that nothing
else in the corpus states these, so this is the only check there is — and that
uniqueness claim holds under enumeration: each of the three clauses occurs in
`src/prompt.rs` and in this test and nowhere else in `crates/`. What it would
still pass under is a fourth sentence: three `contains` calls cannot see an
addition, and `every_kind_gets_the_same_signalling_contract` cannot either,
because it compares kinds against each other rather than against a fixed text. The
test that closes that hole is at the composition below.

**One count in the evidence has gone stale, and the campaign that made it stale is
this one.** `no_prompt_states_the_stop_flag`'s doc comment says a prompt naming
`--done` would hand the flag to *the eighteen kinds it is not an ending for*.
Enumerated today, `plugins/grove/skills/` holds twenty-four directories, twenty-three
of them `grove-<kind>`, so the flag would reach **twenty-two**. Nineteen is the count
this module's own header records twice — the nineteen-to-ten reference map and the
nineteen-to-two ending map that `prompt-names-the-kind-k18` deleted — so the comment
was right when it was written, and the four kinds between then and now are the size
of the editorial pipeline that has been installed since. The test itself is
unaffected: it iterates `shipped_kinds()`, which reads the directory, so the
assertion has been covering twenty-three kinds while its prose said eighteen.
`stop-flag-kind-count-k173` holds the correction, and the file is `tests/` — evidence,
not a root — so no ledger and no page moves with it.

<a id="what-one-launch-varies"></a>
## Everything one launch varies

Four fields, and the doc comment on each names the owner of the thing rather
than explaining it.

<!-- fragment «core-mandate» owner="too-late-to-say-later" source="crates/grove-loop/src/prompt.rs" lines="179-201" parent="the-prompt-core" -->
````rust
/// Everything one launch varies, and the whole of what composition reads.
///
/// The shape is `docs/specs/module-decomposition.md`'s, decision 9. `kind` is
/// borrowed rather than owned: it became a validated token holding a `String` at
/// `open-kind-k20`, and a mandate is read once and dropped.
pub struct Mandate<'a> {
    /// The selected leaf's stable handle. Held as the owning type rather than as
    /// a rendered string: a handle is a projection of a name and the name's
    /// owner does the projecting (`docs/specs/module-decomposition.md`,
    /// decision 4). This module composes text and spells no grammar of its own.
    pub handle: &'a Handle,
    /// The selected leaf's kind — rendered into a skill name, never interpreted.
    pub kind: &'a Kind,
    /// The resolved jj workspace the session will run in. The **value** the
    /// prompt states the version control from, so no session ever detects it
    /// (`docs/ARCHITECTURE.md#symmetric-vcs-rule`).
    pub workspace: &'a Workspace,
    /// Grove's published release version — the workspace's single version, which
    /// orders and means something to a human. `grove --version` renders the same
    /// value and remains as a fallback, not as the mechanism.
    pub version: &'a str,
}

````
<!-- /fragment -->

The four fields are decision 9's, and unusually the record carries them as
written source rather than as prose: the `Mandate` struct in
`docs/specs/module-decomposition.md` has these four fields in this order, beside
`pub fn compose(mandate: &Mandate<'_>) -> String`. So *the shape is the record's*
is checkable by reading the record.

Each field's doc comment names the owner of the thing rather than describing it,
which is what *do not restate* looks like when the module writes it itself. The
handle is held as the owning type because a handle is a projection of a name and
the name's owner does the projecting; the workspace is here as a **value**, not as
a capability to detect anything; the version is the workspace's single release
version, chosen because it orders and means something to a human. And `kind` is
borrowed rather than owned for a reason that dates a change: it became a validated
token holding a `String` at `open-kind-k20`, and a mandate is read once and
dropped.

<a id="composition"></a>
## Composition, and the deletion that made it infallible

The function the whole module exists for, and the only place its three parts
meet.

<!-- fragment «core-compose» owner="too-late-to-say-later" source="crates/grove-loop/src/prompt.rs" lines="202-222" parent="the-prompt-core" -->
````rust
/// Compose the whole of `${prompt}` for one launch.
///
/// Three parts in the session's own timeline order — load instruction, runtime
/// facts, signalling contract — joined by a blank line, for every kind.
///
/// **Infallible**, which is the shape of the deletion rather than a convenience:
/// composition used to read the embed and could fail on a path the corpus did
/// not carry. It reads nothing now, so there is no launch left to fail.
pub fn compose(mandate: &Mandate<'_>) -> String {
    let load = LOAD_INSTRUCTION
        .replace("{skill}", &skill_name(mandate.kind))
        .replace("{plugin}", PLUGIN)
        .replace("{kind}", mandate.kind.label());
    let facts = RUNTIME_FACTS
        .replace("{handle}", &mandate.handle.to_string())
        .replace("{stated_vcs}", &stated_vcs(mandate.workspace))
        .replace("{version}", mandate.version);

    [load, facts, SIGNALLING_CONTRACT.to_string()].join("\n")
}

````
<!-- /fragment -->

Twelve lines, three substitutions on the load instruction, three on the runtime
facts, and the contract joined verbatim. **Infallible is checkable and it is
absolute**: enumerated, the whole 245-line root contains no `Result`, no `?`, no
`unwrap`, no `expect`, no `panic!` and no `bail!`. There is no error type here and
no failure path to test, which is why this chapter has no refusal arms to
attribute and no mutation to run over them — the two instruments Part V has leaned
on hardest have nothing to bite on, and that absence is the deletion's shape
rather than a gap in the evidence.

`the_prompt_is_three_parts_in_the_sessions_own_timeline_order` is the structure
brief's first pin and the one test in the file that could catch an addition. It
does not sweep for substrings: it reassembles the prompt from its parts and
compares the whole, so a driver-authored sentence added anywhere — an
introduction, a footer, a restated rule — fails it even though every `contains`
assertion in the file would still pass. That is the *and nothing else* claim,
which is the half of the rule that a positive check cannot make.

`no_placeholder_survives_composition` is its complement from underneath: a
substitution that stopped matching would leave `{handle}` or `{skill}` in the
output, and a prompt with a literal brace in it would state a fact no one
resolved.

The sizes are worth a paragraph, because the module is judged against a number
and because what moves the number is instructive. Composed for all twenty-three
kinds the plugin ships, with the suite's own fixture handle and version, a prompt
runs 1,843 to 1,921 bytes against a workspace root of
`/Users/antony/Development/grove` — 45.0% to 46.9% of the suite's 4 KiB alarm.
**The spread across kinds is exactly 78 bytes and does not depend on the root
path**: the kind's label is substituted three times per prompt, twice inside the
skill name and once on its own, and the shipped labels run from three characters
to twenty-nine. What *does* move the absolute figure is the workspace root, which
the stated VCS renders in full — checked out at a longer path the same
twenty-three prompts run 1,894 to 1,972. So the one launch-varying value that can
push this module toward its alarm is not a kind at all but where the repository
happens to live. Either way the reading is *well over half of it free*, which is
what the alarm's own comment claims; and the alarm
lives in the suite rather than the build deliberately, because it measures a
judgement against an admittedly arbitrary number and failing a contributor's
build on that is a gate this design declines to erect.
`every_kinds_prompt_stays_under_the_size_alarm` is the sweep and
`the_size_alarm_fires_on_an_oversized_prompt` is its control.

<a id="the-value-and-the-clause"></a>
## The value, and the clause that went to the skill

The last item, and the only value this module computes rather than receives.

<!-- fragment «core-stated-vcs» owner="too-late-to-say-later" source="crates/grove-loop/src/prompt.rs" lines="223-245" parent="the-prompt-core" -->
````rust
/// The **value** that states this working tree's VCS to the session.
///
/// The fact is the driver's: the workspace resolved before this session existed.
/// Only the session re-derived it, and re-derived it badly — a harness banner
/// computed from `.git` alone reads a jj workspace as no repository at all, and
/// detection carried as skill instructions is skippable, so a session that never
/// loaded them commits with Git in a jj tree and bypasses the operation log.
///
/// **Two elements, and the third one left**: identity and the resolved root, and
/// no *do not probe for it*. That clause is a normative consequence of a value,
/// and the closed fact test hands every such consequence to the skill — the
/// spine's `skill-stated-vcs-is-definitive` states it, and stating it here again
/// would be the second source the core exists to avoid.
///
/// Still deliberately **not** the commit-boundary commands — those live in the
/// methodology's Commit step, and a copy here would drift across the build
/// boundary (`docs/ARCHITECTURE.md#the-boundary-is-a-build-not-a-commit`).
fn stated_vcs(workspace: &Workspace) -> String {
    format!(
        "this working tree is jj-enabled (jj workspace root: `{}`)",
        workspace.root().display()
    )
}
````
<!-- /fragment -->

Six lines of body under seventeen of argument, and the argument is the book's
spine in miniature: the fact is the driver's, and only the session was
re-deriving it — badly. A harness banner computed from `.git` alone reads a jj
workspace as no repository at all, and detection carried as skill instructions is
skippable, so a session that never loaded them commits with Git in a jj tree and
bypasses the operation log. `docs/ARCHITECTURE.md#symmetric-vcs-rule` argues
exactly that, and the section's own residue marker names this sentence's subject.

**Two elements, and the third one left — and the third is where the closure on
*fact* does its work.** The [rendered value](../../../CONTEXT.md#stated-vcs)
carries identity and the resolved root — *this working tree is jj-enabled*, and
the workspace root. What it does not carry
is *do not probe for it*, because that is a normative consequence of a value, and
the closed test hands every such consequence to the skill. The clause is not lost;
it is one rung up, and the spine's *do not re-derive which lane this working tree
is on* is where a session meets it.

**The record naming that rule does not resolve, and this is the one address in
the block no instrument checks.** The comment attributes the shed clause to
`skill-stated-vcs-is-definitive`. The methodology's rule inventory is
`plugins/grove/conformance/rules.tsv`, 169 rows whose ids are unique across the
file, and the rule is there under the id **`stated-vcs-is-definitive`**, owned by
`grove/SKILL.md` — no id in the file begins with `skill-`. So the claim is true
twice over: the rule exists, and the spine's own text states it. Only the
identifier is wrong, by one prefix. This is chapter 16's `probe_lease_holder`
class met again — a name in a comment resolving to nothing — and it is invisible
to every instrument this book has used: it is not an intra-doc link, so `cargo
doc` is silent; it is not a Markdown link, so no link sweep sees it; and it is not
an `ADR <slug>` citation, so `every_adr_citation_names_a_decision_record` does not
read it either. It was found by enumerating the block's backticked tokens and
resolving each one, which is the only procedure that finds this class.
`prompt-rule-id-prefix-k174` holds the fix, deferred behind this book with the
unresolved link at line 28.

**And one clause the module keeps out on a different ground.** The
commit-boundary commands are *not* here, and the reason cited is
`docs/ARCHITECTURE.md#the-boundary-is-a-build-not-a-commit`: a running driver is
the build already in memory and never re-execs, so a change to the prompt reaches
no session in the same loop. A copy of the methodology's Commit step embedded in
this file would drift across exactly that boundary, and the section argues it in
those terms. That citation holds.

<a id="what-could-not-move-here"></a>
## What could not move

The book's three questions, asked of the smallest root in Part V. That ranking
is an enumeration rather than an impression, and it is worth writing down because
the unit is the trap that caught chapter 16: Part V's roots run `driver_lease.rs`
1,383, `loop_driver.rs` 615, `session_config.rs` 358 and `prompt.rs` **245**,
and its owned blocks run 819, 615, 564, 358 and 245. `prompt.rs` is last in both
lists, and because chapters 18 and 19 each own a whole unsplit root there is no
unit under which that changes.

**On the way in — the names.** Almost nothing, and the almost is the interesting
part. This module spells no grammar: it takes a `Handle` and a `Kind` already
parsed, renders the handle through the type that owns it, and turns a kind's label
into a skill name with a `format!`. The one name it does own is `PLUGIN`, and it
owns it because the same token has to appear twice in one sentence — bare inside
the skill name and again as the namespace — and the two spellings must land on the
same target. The cost is the outcome's: a chosen value stated where a reader can
find it, and here the reader who most needs it is a marketplace manifest in
another directory, held in step by one test and nothing else.

**On the way through — the preconditions.** None. Enumerated across the crate's
twelve Rust roots — counting the lines that match `Result`, `?`, `bail!`,
`unwrap`, `expect` or `panic!` once `//` comments are stripped — `prompt.rs` is
the **only one with no fallible construct at all**, and it is a zero rather than
a minimum: the next lowest root carries four, and the two heaviest carry 257
apiece. The precondition work happened upstream — the lease admitted the driver,
the walk chose the leaf, the configuration resolved a template — and composition
is what happens once none of it can fail. The outcome's second cost, *the check
must run against the same snapshot the operation then plans from*, is discharged
here by there being nothing left to check: `loop_driver.rs` reads the selection
once and does not recompute it before the spawn, so the mandate the prompt states
is the same value every other consumer of that read was given.

**On the way out — the policy.** Chapter 18 answered this question with four
names a runner could not have guessed; this chapter answers the same question
one layer in, and its answer is a **test** rather than a value. Grove chooses what
a session is told at the moment it can still be told anything, and the too-late
test is what bounds that choice: a sentence earns the channel only when its
failure mode is one the skill cannot repair. Closing the test on *fact* is what
stops it from growing — a driver fact is a launch-varying value, and every
normative consequence of a value stays in the skill — and the visible price is
paid in three places on this page: a rule shed to the spine at `stated_vcs`, an
ending deferred to the kind's skill in part 3, and a compatibility check inverted
into a bare version string in part 2. Each of those is a sentence grove could have
written into every prompt and chose not to.

**And the thing this chapter is really for.** The other four chapters of Part V
are about a runner's authority — who holds the lease, which calls it admits, which
file names the program. This one is about a **channel**, and about the only
question a channel poses: what has to be said now because it cannot be said later.
Nothing beneath this layer could answer it. `keyed-launch` expands a template and
knows nothing about sessions; `jj-workspace` resolves a workspace and knows nothing
about what the answer is for; the store has no word for a kind at all. The
too-late test could not move because it is a judgement about a methodology the
crate deliberately does not contain — and the module's answer to owning a judgement
it cannot check is to write the judgement down, close it on a definition, and keep
245 lines of prose arguing why each sentence that is not there has somewhere better
to be.

[Previous: Which files take part](18-which-files.md) | [Contents](README.md) | [Next: The loop](20-the-loop.md)
