# usage-agreement-k69

## Goal

Place harness-dispatch in the architecture document, bring the usage documents
into agreement with one another and with `--help`, and write the CHANGELOG for
the first release. Then cut the documentation-acceptance review, as the last
act before retiring.

## Context

`acceptance-walk-k67` found a passing instrument for every acceptance case,
and its running log names each one. `current-state-documents-k68` made the
spec current. The documents here must claim no more than those two
established.

What k67 saw while sizing this leaf, as a starting list and not a complete one.

- `docs/ARCHITECTURE.md` names harness-dispatch once, under *Task kinds and
  composition*, as the reader of the `**Reviews:**` and `**Creator:**` lines.
  It places the package nowhere. *Repository products* still says Homebrew
  installs `grove` and `grove-llm`. *Documentation ownership* has no row for
  the dispatch README or its spec. *Main module seams* defers to the overview
  book's package map, and no book under `docs/walkthroughs/` names the crate.
  *Verification* does not mention the worker build, the probe builds or the
  installed smoke. Place the package and its boundary: what it depends on,
  what it must not, and why an extraction can move it. The boundary's
  decision is `docs/adr/harness-selection-is-owned-by-policy.md`.
- `CONTEXT-MAP.md` argues, crate by crate, why each is or is not a bounded
  context. It says nothing of harness-dispatch, whose terms the grove glossary
  already holds. Decide whether it owes a paragraph.
- The Grove-side guidance lives in four places, as the root brief records
  under *Grove dispatch closed*: `docs/CONFIGURATION.md#harness-dispatch`,
  `docs/USAGE.md#if-a-dispatched-launch-refuses`, configure-grove's
  `references/dispatch.md`, and the dispatch README's *Called from Grove*.
  The creator line's owner is `docs/CONFIGURATION.md#a-reviews-creator-line`.
  Consolidate against those and do not write a fifth.
- Both help texts are a subject: `harness-dispatch --help` with each
  subcommand's, and `grove --help` where it quotes the dispatch command. A
  document that describes a flag, a default, a bound or an exit code must
  agree with the help text and with the spec's tables. Enumerate the flags
  from the help output, then find each in the documents.
- `docs/RELEASING.md` lists the installed layout near "the compiled policy
  worker". Check it against the spec's `#delivery`, which also puts the Grove
  adapter's declarations in `grove/`.
- The CHANGELOG's Unreleased section was written increment by increment. Its
  first `harness-dispatch` entry still says "This first step delivers
  `harness-dispatch inspect --kind K` for a static `routes` policy". Rewrite
  the section so it describes the first release of harness-dispatch as a
  whole, the Grove slots and the methodology amendment. The file's own header
  says how entries are written, and that the `## Unreleased` heading must stay
  exactly as it is.

A new documented form of the dispatch command needs a launched case in
`the_documented_command_definition_for_dispatch_is_the_one_launched_here`
(`crates/grove/tests/loop_driver.rs`), which pins every quoted command.

## Done when

- `docs/ARCHITECTURE.md` places the package and its boundary.
- The dispatch README, `docs/USAGE.md`, `docs/CONFIGURATION.md` and
  configure-grove agree with one another and with `--help`.
- The CHANGELOG's Unreleased section describes the first release of
  `harness-dispatch`, the Grove slots and the methodology amendment.
- `task check` passes on the tree the review will read. If the worker's
  source, the installed layout or a native dependency changed since k67's
  smoke run, `task release:smoke` passes again on all three targets.
- Before retiring, cut the review at the grove root:
  `grove-llm leaf-add . dispatch-documentation --kind review-impl`, with
  `**Reviews:** dispatch-documentation-k41`. Retiring this leaf closes the
  node, so the review names the node. Its body asks for a review of the
  documentation against the requirements' acceptance cases: activation, both
  inspection surfaces, the `**Creator:**` conventions and their remedies,
  later outcome entry, launch-time validation guidance and the stated floors.
  Point it at `acceptance-walk-k67`'s running log, which names the test for
  each clause and lists what only a review can judge, under *What a test
  cannot hold*. The review's own brief chain is the root alone, so it will
  not find that log unless its body names it.
  The root brief gives it one more subject, where it records
  `creator-reference-k60`: k60 rewrote the last paragraph of the retirement
  step, nothing has reviewed the rewrite, and this review reads it for a leaf
  that cuts its own review, a node close and a multi-level close.

## Notes

## Decisions (running log)

**How the agreement was checked.** The subject was read whole first: the spec,
the dispatch README, the configuration reference's dispatch section, the usage
guide's three dispatch passages, all four configure-grove files, the release
procedure, the context map, the policy-ownership ADR and the overview book's
package map. The help texts were captured from a binary built from this
working copy: `harness-dispatch --help` and `-h`, `inspect`, `run`, `record`,
`record observe`, `record show`, and `grove --help` with `config` and `run`.
Each flag was then looked for in the documents, by name, with its default and
range.

**Grove's help quotes no dispatch command.** `grove --help`, `grove config
--help` and `grove run --help` do not mention harness-dispatch, and no Rust
source Grove ships names it. The search was `grep -rli 'harness.dispatch'` over
the `src` of every Grove crate. The same command finds `keyed_launch` there,
and finds the pattern in `crates/grove/tests` and `crates/harness-dispatch/src`,
so it reads those trees. The one mention on Grove's side is the methodology's
`references/retire.md`, which names `HARNESS_DISPATCH_RUN_ID`. So "both help
texts" in `the_documented_command_definition_for_dispatch_is_the_one_launched_here`
are `harness-dispatch --help` and `run --help`, and the architecture document
may say Grove does not know the command.

**The package gets its own architecture section.** `docs/ARCHITECTURE.md` has
*The harness-dispatch package* after *Session configuration*. It states the
boundary as three sides: what the package depends on, what it must not, and
what Grove must not. It lists the four places data crosses, and what an
extraction takes and what refers to the directory from outside. It restates no
contract the spec owns. *Repository products*, *Documentation ownership*,
*Main module seams* and *Verification* each gained their part.

**The overview book's package map was wrong, so it is corrected.** *Main
module seams* defers to that map, which said the workspace has eight packages,
seven sharing the version, and that archives hold only `grove` and
`grove-llm`. It now lists `harness-dispatch`, states no count, and says no
Grove package depends on it or it on them. A book may link only its own pages,
its guide and its glossary (`M201`), so the page names `docs/ARCHITECTURE.md`
as a path. `book-check` passes on that book. Other "six packages" counts in the
book-structure specs are dated measurements of what left the architecture
document, and stay.

**`CONTEXT-MAP.md` owes a paragraph, and harness-dispatch is not declared a
fourth context.** The map argues every other crate in or out, so its silence
read as nothing to argue. There is something. By the map's own test, a
language boundary, this crate comes closest of any to a context: *routes*,
*selection*, *policy*, *run* and *context* each mean something else on Grove's
side. The runner avoided that by its naming and this package did not. What it
lacks is a glossary of its own. Its words are defined in the spec and the
README, and the grove glossary holds seven of them. Declaring a context would
mean moving four of those entries to a new glossary, which is a restructuring
of a file other artifacts link into, after two design reviews accepted where
the entries sit. That is more than this leaf's charter, so the map says the
crate is not declared a context, why, and that a glossary of its own, which an
extraction would force, is what would change that. It gained a row in the
terms table, a relationship entry with the seven shared words, and a line
under *Choosing a context*. I made this call without the human, and the review
is asked to attack it.

**What the flag walk found.** Every flag of `inspect` and `run` is in the
README's *Inputs* table with the help text's default and range, except
`--json`, which the README described only under *Inspect*, *Run* and
*Refusals*. It has a row now. The `record` flags are in *Run records* and
*Observations*. The exit results in `--help` agree with the README's refusal
table and the spec's `#diagnostics`. The help text is a summary: it gives 2 as
"malformed command line" and 5 as "worker or protocol failure", where the table
also files an invalid prompt under 2 and two internal failures under 5. No
document and no help text disagreed on a flag, default, bound or exit code.

**Disagreements found and closed.**

- The README opened with "This release delivers inspection and running of a
  static `routes` policy or a computed `select`" and said three starter
  policies ship, where *Starter examples* lists five. The opening now gives the
  four commands as a table and the rest in two paragraphs, with every fact it
  had.
- `docs/RELEASING.md`'s installed layout lacked `libexec/harness-dispatch/grove/`,
  which `archive_manifest` and the spec's `#delivery` both carry. It also gave
  the smoke test two of its four cases. The README's account of
  `installed-smoke.sh` gave one.
- The spec's `#delivery` says the release procedure states that each Bun
  upgrade rechecks the kernel range. Only the README did. The procedure now
  says so, and names what else an upgrade re-pins, from the comment in
  `dispatch.sh`.
- The usage guide's `grove --help` transcript lacked the `run` command. It is
  the built binary's output now. Its `grove 21.0.0` stays: a version in a
  transcript is stale at every release.
- The spec's `#grove-integration` said the usage guide explains activation and
  warns against granting `GROVE_SIGNAL_FILE`. It does neither: it shows both
  inspection surfaces, the boundary and a refused launch, and links the
  reference. The sentence now says which document does what. Adding the
  schema-level warning to the usage guide would have put configuration there,
  which that guide's opening says it does not carry.
- The root README called the workspace "two thin binaries over five library
  crates". It has a row for harness-dispatch and a documentation link.

**Activation was run as written.** With the one-line re-export of
`harness-dispatch/examples/grove-static` as the personal policy,
`harness-dispatch inspect --kind impl` refuses with `program_not_found`, exit
127, naming `my-codex-wrapper`. With a wrapper of that name on `PATH` it
reports `lead-high`. A copy of `examples/grove-static.ts` from beside the
worker works unchanged as `policy.ts`, and so does a copy of
`grove-review.ts`, which reports the adapter. So the configuration reference
now gives the copy as a command, says what the first inspection refuses with,
and gives the Homebrew prefix, `$(brew --prefix grove)`. I checked that prefix
on this host for the installed 21.12.0, which predates the command. The
formula template installs `libexec/harness-dispatch` under it. The README's
*Install* says where the worker, SDK, adapter and examples sit.

**Two additions to configure-grove, so that it agrees with the review
example.** `SKILL.md` requires every review to differ in provider from its
producer, and `model-selection.md` tells a router to exclude every recorded
producing provider. The shipped example compares one, the original creator's.
`references/dispatch.md` now says so under *Verify*, and that the rest is the
operator's to check. It also says the command never falls back, and that a
`select` returning its own table's candidate is the policy's choice, refused
against an explicit choice. That is where `model-selection.md`'s "fall back to
declared policy" lands under dispatch. No new command form is quoted, so the
launched-command pin needs no new case.

**The configuration reference's inspection passage says what the help says.**
*Inspecting both halves* said only that inspection launches nothing. It now
says inspection evaluates trusted TypeScript, which may have effects, and is a
proposal. `inspect --help` and configure-grove already said both.

**The CHANGELOG's Unreleased section is organised by subject.** It had one
entry per increment, and no entry at all for `run`, records, observations,
`select`, context, bounds, cancellation or the review policy. Every change
since v21.12.0 is this grove's, read from the commit subjects, so the section
is the whole release. It now has `harness-dispatch` bullets by subject, the
Grove slots, the dispatched command, the methodology amendment, which is kept
as written, configure-grove, the documentation, and the release tooling. Three
things in the old entries were statements about builds that never shipped:
"this first step delivers", the start directory "now created owner-only", and
"before, only a package laid out with an `index.js` loaded". They are gone.
The changes against v21.12.0 that an upgrading owner meets stay as changes:
the archive's `bin/` layout, the AppleDouble files, and what the checks and
the release now need. The emulator versions and the guest-base detail are in
`docs/RELEASING.md` and the runtime evidence, and left the CHANGELOG.

**Second passes, each with a control.** A script resolved every relative link
and fragment in the eleven edited documents against real anchors: 182
fragment links, no problem. The same check over a copy with three planted
faults reported all three. A word sweep of the README and of
`references/dispatch.md` for increment wording found only current-state uses,
such as "a record this release cannot read". The same pattern finds its words
in the CHANGELOG header and the research survey.

**No in-session reviewer.** This leaf cuts the node's review, so its review is
scheduled and it spends none.

**`task check` passes on the finished tree.** It ran once, after the last
edit to a tracked file outside `.grove/`, with `GROVE_SIGNAL_FILE` removed from
its environment. All 12 principal checks pass. Of 126 test result lines none
reports a failure, and ten tests are ignored, the number `acceptance-walk-k67`
reported. The tests that read documents by name each appear as `ok`:
`every_repository_markdown_reference_resolves`,
`user_documentation_references_resolve`,
`every_book_root_has_a_documentation_ownership_row`,
`the_documented_command_definition_for_dispatch_is_the_one_launched_here` and
`the_creator_reference_amendment_is_stated_where_each_rule_lives`. A name that
does not exist is found nowhere in the log. A digest over all 1920 tracked
files outside `.grove/` was the same before and after.

**The installed smoke was not rerun.** `dispatch.sh build-id` still prints
`721aab0848f6…`, the build `acceptance-walk-k67` smoke-tested on all three
targets. No worker source, archive manifest entry or native dependency
changed. The root `README.md` an archive carries changed in content only.

**The review is `dispatch-documentation-k71`, cut before retiring.** It was
appended at the grove root, as this leaf's brief directs, so it runs after the
two flake leaves and `seam-controls-k70`. Its body says k70 may have moved
test names since k67's log. Its `**Reviews:**` line names
`dispatch-documentation-k41`. This session has no `HARNESS_DISPATCH_RUN_ID`,
so it wrote no `**Creator:**` line. A search of `.grove/` for the whole
handles `usage-agreement-k69` and `dispatch-documentation-k41` found one live
review naming either, k71 itself, and no `**Creator:**` line to remove.

**Closing `dispatch-documentation-k41`.** Its brief's conditions hold: the
spec is current state (k68), the architecture document places the package and
the usage documents agree (this leaf), the walk is in k67's log, `task check`
passes on the integrated tree, k67's smoke run stands for this worker, the
CHANGELOG describes the release, and the review is cut. What later leaves
build on went to the root brief, under *Documentation closed*.
