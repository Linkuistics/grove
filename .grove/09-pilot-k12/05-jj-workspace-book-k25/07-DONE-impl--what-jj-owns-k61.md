# what-jj-owns-k61

## Goal

Draft `07-what-jj-owns.md`, slice `assembly`, and close the book: green **final**
validation with no deferred holes, and a green `bash scripts/check.sh`. This is
the last commit of the draft stage.

## Context

- Responsibilities: [`docs/specs/jj-workspace-book-structure.md`](../../../docs/specs/jj-workspace-book-structure.md),
  *7 · What jj owns*. It owns **no** production source and is therefore
  final-only: it has no scoped prefix to prove, which follows from the slice
  owning no `[[block]]` rather than from any manifest field.
- It carries **no** worked-example section — six chapters do, and the seventh's
  job is synthesis (structure brief, *Worked examples*).
- What it must produce: one table assembling the six refusals into *what was
  subtracted, who owns it, and what the subtraction bought*; the test that
  separates a justified subtraction from an abdication — the underlying tool's
  guarantee must be at least as strong as the one the consumer needs, and
  checkable — applied to **all six**, including the one where the answer is least
  comfortable; the closed early-use and source ledgers; and the final
  verification commands with their results.
- The reader-facing outcome the whole book is for is stated here (decision 6 of
  `jj-workspace-structure-k17`): the reader can take a boundary in their own
  code, ask what the tool underneath already owns, and tell a justified
  subtraction from an abdication.

## Done when

- `07-what-jj-owns.md` exists; `README.md` lists all seven chapters as links with
  no plain-text future titles left; navigation's final-page form is on this page
  and `06-refusal.md` gains its Next.
- Every `[[early-use]]` row is `explained` and every ownership row is `resolved`.
- `cargo run --quiet -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/jj-workspace --final --check all` exits 0, reporting 4 roots,
  698 lines, zero deferred ranges and `final=true`.
- `bash scripts/check.sh` passes — **all** checks, including `book-check`. This
  is the child that closes the gate every earlier child left red.
- The draft stage record is complete: `## Provenance` lists all seven commits of
  the range, and `## Baseline` carries the final page inventory, the
  book-directory digest and the final-validation result.

## Notes

**This closes the draft stage and nothing more.** Developmental edit, technical
edit, copy edit, art and proof are `pilot-measure-k26`'s, one leaf and one commit
each. A synthesis chapter is a tempting place to smooth the six chapters ahead of
it; doing so would destroy the unedited baseline the pilot exists to measure
against.

## Decisions (running log)

**1 · The assembly table is placed first, ahead of everything, although the book's
own catalogue-after-example rule appears to forbid it.** The structure brief's
*Worked examples* section says no section earlier than a chapter's worked example
may primarily enumerate three or more public operations or refusal kinds. This
chapter has no worked example — the same section says so explicitly, and the task
file repeats it — so the rule has no anchor to be measured against here, and read
literally it would forbid the chapter's mandated deliverable outright: *§7 · What
jj owns* requires the six refusals assembled into one table. Taken as written the
two instructions are inconsistent for exactly one chapter of the book. Resolved in
favour of the requirement rather than the ordering rule, because the ordering rule
exists to stop a catalogue standing in for an explanation, and a synthesis chapter
whose entire subject is six explanations the reader has already had is the one
place that risk does not apply. Recorded rather than silently decided, since a
technical review checking the rule mechanically will land on this page.

**2 · The test is stated with two clauses — at least as strong, and checkable —
and refusal 4 is failed against the second.** The structure brief fixes the test's
wording and requires it applied to all six "including the one where the answer is
least comfortable", so the verdict was the session's to reach, not to choose. Four
candidates were weighed on evidence already on the pages. Refusal 3's selector
list and refusal 6's static remedy strings are the same copy-not-delegation shape,
and both were graded *passing on a margin* because their error direction is cheap
and their decay is visible. Refusal 5 was rejected as the answer despite carrying
the lossy-path hole, because that hole is at the *interface into* a delegation
whose guarantee is sound and measured — a different failure, and the page names it
as one. Refusal 4 is the answer: `JJ_OWNED_NAMES` is a compiled-in fork of jj's
layout that nothing checks against jj, it is demonstrably one name short on jj
0.44.0 (`jj-owned-names-k65`), and it is the row `CONTEXT-MAP.md` leans on to argue
about the whole crate. That last fact is what makes it the least comfortable rather
than merely the weakest. Rejected: grading all six as justified with caveats, which
would have made the test unfalsifiable on its first application — the one outcome
the structure brief's "including the one where the answer is least comfortable"
exists to prevent.

**3 · A fourth test-coverage gap was found and cut as a leaf rather than absorbed
or footnoted.** Applying the checkability clause to refusal 3 separated two things
that read alike: the removal *mechanism* is proved — deleting the loop fails
`resolution_ignores_repository_selection_and_temporary_directory_environment` —
while the *list* is not, and the test sets only three of the array's four names.
`GIT_INDEX_FILE` is never set. Unlike findings 3 to 6 of the stage record this
touches no frozen root, because `tests/` is evidence rather than a book root, so
it is not bound by the cross-book one-commit rule and is not blocked behind the
books. It is still not this session's: adding a test is a non-book change inside
the draft's commit range, which the preregistration's *Validity* rules out. Cut as
`env-selector-coverage-k68`, placed beside the other `jj-workspace` follow-ups.
Rejected: fixing it here, for the reason above. Rejected: recording it as prose
only — the fog-or-ticket test is whether the question can be phrased precisely
now, and one named variable in one named test is as precise as a leaf gets.

**4 · Two claims drafted onto the page were wrong and were corrected before the
gate was run, not after.** The first draft asserted "six integration tests" for the
gate and "twelve rows" for the early-use ledger. The gate's resolution tests number
ten, and the ledger carries ten rows rather than twelve — the manifest declares
five and the book's own ledger adds five more. Neither error is one the validator
can see: this chapter owns no source, so there is no `F008` byte comparison behind
any sentence on it, and both numbers would have shipped green. The test-count claim
was replaced with a structural statement naming the tree shapes the tests cover
rather than a count, and the ledger paragraph was rewritten to the real numbers and
to name `early-use-scope-k63` as the reason the split exists. Recorded because it
is the specific hazard of a source-owning-nothing chapter, and the technical edit
should know the class exists.
