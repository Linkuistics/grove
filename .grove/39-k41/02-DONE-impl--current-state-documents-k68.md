# current-state-documents-k68

## Goal

Rewrite the design-tense artifacts of harness-dispatch as current state: the
spec, the visual design README and its viewer text, the runtime evidence, the
ADRs and the glossary.

## Context

`acceptance-walk-k67` ran first. It found a passing instrument for every
acceptance case and every seam clause, so the spec may describe the whole
command as current state and nothing stays hedged. Its running log names the
test for each clause, which is the place to look when a sentence of the
notice claims that a suite shows something.

That log also lists where a test holds its clause more narrowly than the
spec's `#test-seams` table words it, and `seam-controls-k70` closes the ones
worth closing. Do not reword the table to match the narrower tests: the
table is the contract, and k70 moves the tests.

What k67 saw while sizing this leaf, as a starting list and not a complete one.
Sweep each document whole, since a list of patterns is complete only as far as
the list.

- The spec opens with a notice of over a hundred lines, from "This is the
  first-release design" to the sentence about the visual document. It lists
  what is delivered, increment by increment. Before deleting it, check each
  sentence for a contract fact the body below does not state. Much of it
  describes what a test suite shows, which `#test-seams` already carries.
- The spec's body still speaks as a design in places. `#grove-integration`
  opens with an instruction, "Add optional whole-argument slots", and ends
  "Usage and configure-grove must explain". `#execution-contract` has "The
  implementation must therefore record". `#delivery` has "Use Bun 1.4.2
  initially", "Each existing archive gains" and "These are
  implementation/release acceptance requirements, not results of this design".
  `#out-of-scope` says "this increment". `#records-and-outcomes` has "a later
  increment supplies".
- Some sentences are history, not hedges: "The `Worker` class in the table
  above was reported here until it was seen to fire", "which is why
  package.json autoloading stayed off until the worker moved" and "The move
  gave that one up to close a reach that was open as shipped". Decide for each
  whether the reason it gives is still needed as current state. The runtime
  evidence is where what was seen belongs.
- The visual design README calls itself "a design for independent review, not
  implemented product behavior", lists "Current changes" from review k9, and
  ends by saying Linux execution, packaging and cancellation tests "belong to
  implementation and release". `diagrams.json`'s `intro` says "nothing here is
  implemented" and names k9 and k10. Check the `.mmd` captions and
  `index.html` too.
- The runtime evidence says, under *Source grounding*, "The three new
  lifecycle slots are proposed, not existing API". Its earlier sections are
  dated observations, which stay as what was seen.
- `docs/adr/harness-selection-is-owned-by-policy.md` has "initially ships" and
  two "follow-up work" sentences. Read all three dispatch ADRs whole:
  that one, `policy-evaluation-precedes-process-replacement.md` and
  `a-review-carries-its-creator-reference.md`. `ADR-FORMAT.md` governs how a
  set is reworked.
- The glossary's dispatch entries, from *Review target diversity* to
  *Delivered context* in `CONTEXT.md`, already read as current state. Confirm
  that, and change nothing that is already right.

No behavior changes here. Tests pin some of these words: the conformance rows,
the composition-guidance pins, and
`the_documented_command_definition_for_dispatch_is_the_one_launched_here`.

## Done when

- The spec carries no "not implemented" notice and no design-era hedge. It
  describes the delivered command as current state, and every anchor another
  document links to still resolves.
- The visual design README, the viewer's text and the runtime evidence are
  current.
- The three ADRs and the glossary entries carry no design-tense wording.
- `task check` passes.

## Notes

## Decisions (running log)

**How the sweep was made.** Each document was read whole first: the spec, the
three ADRs, the visual README, `diagrams.json`, the four `.mmd` sources,
`index.html`, the runtime evidence and the glossary's dispatch entries. Then
every line holding a tense or hedge word was listed and classified, kept or
changed. The word list is in no sense the sweep: it is the second pass over a
document already read, and it found the seam table's row labels, which the
reading had passed over.

**The notice's sentences, sorted three ways.** Each sentence of the spec's
notice was one of three things. Most stated a fact the body already states.
Many said what a suite shows, which `#test-seams` carries. A few stated a
contract fact found nowhere else, and those moved into the body before the
notice went:

- The examples by specifier. The body named only `grove-review`, yet it calls
  the registered list part of the worker's versioned protocol. *Policy and
  joint choice* now names the three starters and points to the two review
  examples, and *Supplied review policy* opens with
  `harness-dispatch/examples/review`. The names were read from the `embedded`
  table in `worker/src/main.ts`.
- A dispatched kind and a direct-harness kind coexist in one configuration.
- A refused delegated launch leaves its leaf live.
- What the Grove-side documents explain, and that the command they quote is
  the one `--help` carries and the launch-boundary suite launches. This
  replaced "Usage and configure-grove must explain".
- macOS arm64 runs the installed smoke natively, which the floor table, being
  Linux's, did not say.

**History sentences, each decided.** "The `Worker` class in the table above
was reported here until it was seen to fire" is gone: it gives no reason a
reader needs, and the runtime evidence's *A VM started later* has what was
seen. The other two carried a reason that is still the reason, so they stay as
conditionals with the history taken out: a worker that did not move *would*
open each `package.json` above it, "which is why package.json autoloading
depends on the move"; and the lost dotenv control "is the price of the move,
which closes a wider reach".

**"As shipped" meant a development build.** The spec and the worker ADR both
said the TMPDIR reach "was open as shipped" or "in the shipped build". No
release ever carried it: "shipped" was the grove's word for the non-probe
build. Both now say the reach is open *without the move*, which is true of the
`unmoved` probe today and claims nothing about a release.

**The seam table's clauses are untouched.** Only its row labels changed:
"New command" to "The command", and "Existing Grove launch boundary" to
"Grove launch boundary". Nothing outside DONE leaves of this grove cites the
old labels.

**"Every field that a later increment supplies … `null` until then."** Every
field is supplied now. The launch document in `src/record.rs` writes each one,
and a field is `null` where the run has no value for it: no task identity, no
context, no creator, no adapter. The spec says that. Two module comments,
in `src/store.rs` and `src/record.rs`, restated the old sentence and now
restate the new one. That is the whole of what changed in source, and both
edits keep their line counts. The package source was not otherwise swept for
design-tense comments. A search of it for "later increment", "not
implemented", "follow-up" and "not yet" found only those two, and a search by
a phrase list is complete only as far as the list.

**The worker ADR's evidence paragraph is rewritten, not trimmed.** It read as
a chronology: "A later control found", "It could not be on while the worker
stayed", "That was accepted". It now gives each control with the class that
fires without it, then the move, its one cost and the alias limit, in the
present tense. Every fact is kept. One sentence says the observations are one
host and one Bun version, and points to the runtime evidence.

**The creator ADR states three rules, not an amendment.** The rules now read
as the plugin's `references/retire.md` and `TASK-FORMAT.md` state them. "Still"
and "rescoped" compared them with rules that no longer exist in that form. The
two rejected registration designs are argued in the present tense. Their
citations of `harness-selection-and-execution-k5` and `-k8` stay: fifteen
other ADRs in the set cite a leaf by handle, so it is the set's convention.

**The policy-ownership ADR.** "Initially ships", both "follow-up work"
sentences, "must allow later extraction", "For the first release" and "Both
inspection surfaces must make the boundary clear" are current state. The last
now says which documents state the boundary. The rejected option "Create a
separate repository immediately" is "Keep the executable in a repository of
its own", with the same reopen condition.

**The viewer has no discussion.** `diagrams.json`'s `discussion` block was the
design review's change list: "Repaired", "New", "Corrected", "Chosen by the
human; reviewed". A delivered command has no current discussion, and
`index.html` treats the block as optional, so it is removed and the viewer is
unchanged. Its "Current discussion" and "Updated" strings are generic chrome
that renders only when a manifest has the block. The README's "Current
changes" list went with it, and a table of the five views replaced it. The
options comparison stays, as the creator ADR's considered options drawn.

**Diagram sources.** The review-finding tags are gone: `(F3)` and `(F4)` in
`creator.mmd`, and `(first design)` and `(k5 F1, F2)` in
`creator-options.mmd`. Two labels were wrong as current state, and are
corrected. `handoff.mmd` said "private cwd … stdin closed", where the worker
has a private *start* directory and null stdin. `creator-flow.mmd` gave the
declaration remedy for every refusal, where a same-origin candidate refuses
with another remedy.

**Runtime evidence: dated observations stay.** Only sentences that still spoke
of work to come changed, each to where that work landed. "Production
acceptance must reproduce these cases" now points to *Ambient authority* and
the spec's seams. "Remain unmeasured" is the probe's own limit, in the past
tense. "No Linux runtime smoke test was performed in this design leaf" is
gone, since *Installed smoke* records three targets. *Source grounding* is
dated as the pre-implementation reading it was, and each finding is followed
by what the source says today. I read `SLOTS` in
`crates/grove-loop/src/session_config.rs`, `LOOP_CONTROL_ENV` in
`loop_driver.rs`, and the three release scripts for `harness-dispatch`. Three
table headers in those dated sections said "as shipped" of a build with all
four switches off, which is no longer the shipped one. They say "as shipped
then".

**The glossary needed nothing.** *Review target diversity* to *Delivered
context* already read as current state, and no tense word in them is a hedge.

**The research survey's bridge is in scope, and the survey is not.**
`docs/research/grove-model-effort-routing.md` opens with *Adopted boundary*,
the section `ADR-FORMAT.md` asks a survey to carry for the records its
findings landed in. It restated the policy-ownership ADR in the tense that
ADR just lost: "initially shipped", "remains follow-up work", "No automatic
dispatcher is implemented". Leaving it would have left the two disagreeing, so
the header and that section are current. Everything below them is research
dated 29 September, proposals included, and stays as written. The header says
so.

**Two briefs changed for the leaves still to run.** The root brief's common
obligation told every implementation leaf to narrow the spec's notice, which
no longer exists. It now says the spec is current state and a leaf that
changes behavior changes the spec's sentence with it. The node brief's second
item says what `usage-agreement-k69` builds on: three sentences here are
claims about k69's documents.

**The viewer was rendered, with a control.** The server the design session
left on port 8769 accepts connections and answers nothing, and 8765 to 8771
are all taken on this host. It is not this leaf's to stop, so
`task design:harness-selection PORT=64930` served the directory. All five
views rendered one SVG each, with no error element and the discussion panel
hidden and empty. The same probe against a scratch copy with `creator.mmd`
broken reported that view's parse error and no SVG for it, while the page
still loaded and the other four rendered. So a check that the page loads
would not have seen a broken diagram, and this one does.

**In-page links have their own check.** The repository's reference test
resolves relative links and says bare fragments are outside its scope. Most of
the spec's in-page links went with the notice, so a script resolved every
`](#…)` in each edited document against that document's own anchors. A copy
of the spec with one fragment misspelt failed it.

**No in-session reviewer.** This node's review is scheduled:
`usage-agreement-k69` cuts the documentation-acceptance review of
`dispatch-documentation-k41`, so this leaf spends none.

**`task check` passes on the finished tree.** It ran once, after the last
edit. An earlier run was stopped unread when a sentence still needed changing,
since a check whose subject moves under it has measured nothing. All 12
principal checks pass, and no test result line reports a failure. Ten tests
are ignored, the number `acceptance-walk-k67` reported. The three tests that
read documentation by name each appear as `ok`:
`every_repository_markdown_reference_resolves`,
`the_documented_command_definition_for_dispatch_is_the_one_launched_here` and
`the_creator_reference_amendment_is_stated_where_each_rule_lives`. A name that
does not exist is found nowhere in the log, so the lookup is not vacuous. A
digest over every tracked file outside `.grove/` was the same before and
after. The worker is still build `721aab0848f6…`: no worker source, installed
layout or native dependency changed, so `task release:smoke` was not rerun and
k67's run of it stands.
