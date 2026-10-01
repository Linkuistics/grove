# dispatch-documentation-k72

**Integrates:** dispatch-documentation-k71


## Goal

Triage the documentation-acceptance review of `dispatch-documentation-k41`
and apply the findings that hold, preserving the delivered first-release
contract.



## Context

Read `dispatch-documentation-k71` from its committed review artifact. It owns
the findings, per-clause dispositions and executed activation evidence; this
leaf does not transcribe them into its charter. The root brief and the area
specification remain the contract. Use `grove-integrate-review-impl` for the
triage and all post-fix verification.

## Done when

- The review has been triaged, with each disposition recorded in this task.
- Accepted repairs and appropriate verification are complete, and any scope
  judgment requiring the human has been handled explicitly.

## Notes

## Decisions (running log)

**The findings were read from the review's commit.** `dispatch-documentation-k71`
is change `ouuwvxoo`, commit `f6ce247f39de`. Its four findings and its
context-map judgement were each checked against the documents and the source,
and F1 and F2 were run again before anything changed.

**How F1 and F2 were run.** An isolated `HOME` and `TMPDIR`, `env -i`, the
checkout's `target/debug` binaries with the worker and examples under
`target/libexec/harness-dispatch`, two fake wrappers and a `brew` stand-in whose
prefix is `target`. The front, the worker and the two installed examples were
digested before and after, and did not change; the worker reported build
`721aab0848f6`. The installed `grove-static.ts`, `grove-review.ts` and
`review.ts` are byte-identical to the source.

**F1 holds: a real issue, repaired in the guidance.** The copy block of
`docs/CONFIGURATION.md` was run verbatim, the copy's two programs were changed
to the fake wrappers, and `inspect --kind impl` chose `lead-high` on the
owner's wrapper. The documented `grove-review` re-export then refused
`program_not_found`, exit 127, naming `my-codex-wrapper`. A copy of
`grove-review.ts` with its imports unchanged refused the same way.
`grove-review.ts` takes `catalog` and `routes` from the installed
`harness-dispatch/examples/grove-static`, so neither path reads the owner's
copy. The example's default behaviour is consistent, so no source changes.

**F1's repair, as run.** Move the edited starter to `grove-static.ts` beside
the policy, copy `grove-review.ts` to `policy.ts`, and change that copy's one
import of the starter to `./grove-static.ts`. Run that way, `impl` chose the
owner's lead, `review-impl` chose the owner's reviewer for a creator declared
`openai` and the owner's lead for one declared `anthropic`, a same-origin
`--choice` refused with exit 3, a `run` launched the fake wrapper with the
prompt as one argument, and bare `grove` launched the reviewer from a review
leaf. A candidate ID renamed in the owner's catalog refused the affected review
as `reviewer_unknown`, naming the entry, and left `impl` working. So the
guidance says the copy's `reviews` table follows a renamed ID or provider. The
rule and the adapter stay the installation's, by name, and the guidance says
that too. The README's `groveReviewSelector` recipe is linked as the route for
a catalog that shares nothing with the starter's.

**F2 holds: a contract stated unclearly.** A review leaf with `**Reviews:**` and
no `**Creator:**` was launched by bare `grove` under the repaired policy.
harness-dispatch refused `creator_line_missing`. Grove printed `status exit
status: 3` and `loop stopped`, left the leaf live, and itself exited 0.
`docs/USAGE.md` says Grove stops the loop "with harness-dispatch's exit
status", which reads as Grove's own. The configuration reference and
configure-grove already say Grove *reports* that status. The repair states
Grove's own exit beside the reported one in all three, and changes no
transcript and no driver behaviour.

**F3 holds: a contract stated unclearly.** `worker/src/main.ts` imports
`../grove/index.ts` and the Grove review example, registers both in `embedded`,
and names both in `bringsAdapter`. It calls nothing in either. So "the core
never imports it" is false of the policy host, and true only of the front. The
behaviour the sentence was reaching for holds: the adapter's reader runs only
when a policy imports it, which k70's
`grove::without_the_adapter_a_review_kind_and_its_marker_lines_mean_nothing_to_the_front`
observes. The claim was swept by phrase and by subject, not by the review's
three citations. Five sentences state it: the spec's package-boundary
paragraph twice, the architecture's adapter paragraph, the glossary's *Grove
adapter*, and the README's "ordinary dispatch never loads". The three ADRs and
the design directory's package diagram do not state it.

**One statement of F3 is left, as a visible trade-off.** The header comment of
`worker/grove/index.ts` says the adapter is "never part of ordinary dispatch"
and can move "unchanged". Both are true of the module. The file is in the
worker's digest set, so rewording a comment changes the build identity and
would retire k67's three-target smoke run for a sentence that is not wrong.

**F3's wording.** Each of the five sentences now says what the source shows.
The front has none of the adapter's code. The policy host imports it only to
embed it, register its specifier and report its version, and never calls its
reader. So it reads a task file only when a policy imports it, directly or
through the Grove review example. The extraction sentence stays, and gains its
cost: the move also takes the adapter, and the example that composes it, out of
the host's embedded set.

**F4 holds: a real issue.** The CHANGELOG said a dispatch policy's adapter
reads the `**Reviews:**` and `**Integrates:**` lines. The adapter's two markers
are `**Reviews:**` and `**Creator:**`, and `markerLines` collects no other. No
file under `crates/harness-dispatch` contains `Integrates`. The same search
finds it in `crates/grove-llm/tests`, and finds `Reviews` in the package, so
the empty result is not a broken search. The sentence now names the two lines
the adapter reads and says nothing reads `**Integrates:**`.

**A test now holds F1's repair.**
`grove::a_copy_of_the_grove_example_takes_an_owners_edited_starter_only_when_its_import_names_that_file`
performs the documented edit on the shipped sources. Its control is the same
copy with its import as shipped, which must refuse `program_not_found` naming
the installation's wrapper. Then, with the import pointed at the owner's file,
`impl` and `review-impl` resolve the owner's wrappers, a same-origin `--choice`
refuses, and a run launches with the owner's model. It also requires the
example to import the starter on exactly one line, the line the configuration
reference quotes, so a change to that line fails here. It was seen to fail both
ways: with the import left as shipped, at the first report, and with
`my-codex-wrapper` on `PATH`, at the control. The test reads only the package's
own files, so the package gains no reference to Grove's documents. It is a
test file, so no worker source changed and the build is still `721aab0848f6`.

**The revised activation was run as written.** The fenced blocks of *Activating
it* were extracted from `docs/CONFIGURATION.md` in order and run in a second
fresh `HOME`: the first copy block, then the new `mv` and `cp` block, then the
change from the first quoted import line to the second, which matched exactly
one whole line of the copy. `inspect` for `impl` and `review-impl` and a `run`
of `review-impl` then used the owner's wrappers.

**The context-map judgement went to the human, who chose the explicit
exception.** The review did not make it a finding and reserved it. It holds as
a criticism of the reasoning: the map defines a context as a language boundary,
tabulates the words the two sides use differently, and then withheld the
declaration because the package has no glossary, which is what a declaration
would produce. Three outcomes were put on 2026-10-01: state the exception,
declare a fourth context in a new leaf, or leave the paragraph. The
recommendation was the exception, on this evidence: the spec and README already
define the package's words, so a glossary would be a third statement of them;
the command-only entries in `CONTEXT.md` are where a Grove owner looks; and no
test or behaviour depends on the declaration. The human chose it. `CONTEXT-MAP.md`
now says the boundary exists and the collision table holds it, that the
declaration is withheld on purpose, that the command-only glossary entries are
part of the exception, and that an extraction ends it. No glossary entry moved,
and the map still counts three contexts.

**The review's other clauses needed nothing.** Its table gives a reason for no
finding on inspection, launch-time validation, selection ownership, the creator
conventions, outcome entry, the floors and agreement with `--help`, and it
found the rewritten paragraph of `references/retire.md` sound for all three
cases. None of those was reopened here.

**The retired sentences are gone from the documents.** One search for the four
retired phrasings found nothing outside `.grove/`. With the same flags it found
the sentence left in `worker/grove/index.ts`, and it still found "core never
imports" in the review's own task file, where the record of the claim belongs.

**`task check` passed on the finished tree.** It ran after the last edit to a
checked file: all 12 principal checks, with every suite's result line read and
none failing, and the new test's own line passing in `tests/grove.rs`. The
working copy's diff had the same digest before and after the run. The worker
and its probes rebuilt to `721aab0848f6…`, the build k67's smoke run stands
for. Only this entry and the retirement came after the run, and both are under
`.grove/`.

**No review is cut for this integration.** F2, F3 and F4 are rewordings to what
the source and a run show. F1's guidance is held by the test above and was run
as written. The context map carries the human's own decision. The session
spent no in-session reviewer.
