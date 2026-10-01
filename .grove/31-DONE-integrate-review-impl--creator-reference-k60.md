# creator-reference-k60

**Integrates:** creator-reference-k59

## Goal

Independently triage the implementation review of `creator-reference-k38`,
apply the findings that are valid, and verify the resulting methodology,
lifecycle evidence and operator guidance before dependent implementation runs.

## Context

Read `creator-reference-k59`'s findings from its committed review artifact,
against the current source and the producer commits it names. The reviewed
producer is the whole `creator-reference-k38` node, including
`creator-methodology-k39` and `creator-lifecycle-k40`.

The contract is `docs/specs/harness-selection-and-execution.md`, especially
`#identity-and-creator`, `#grove-integration` and the Grove launch-boundary
row of `#test-seams`, with `docs/adr/a-review-carries-its-creator-reference.md`.
The brief chain records the agreed policy boundary and the owner-guidance
surfaces. The review was inspection-only; its source-level counterexamples
are not fresh test results.

## Done when

- Every finding has an evidence-based disposition in this task's running log,
  under the integration procedure's triage categories.
- Valid findings are integrated at a coherent, reviewable boundary, with the
  durable specification and ADR set reconciled where needed.
- Appropriate reusable checks and meaningful controls have been run against
  the resulting changes, with their evidence recorded. The repository remains
  releasable before `package-entry-resolution-k52` resumes.

## Notes

This is a triage charter, not an instruction to accept the review's findings.
Preserve the explicitly accepted attestation boundary and generic dispatch
policy ownership unless the evidence requires a separately chartered design
change. Follow the kind's integration procedure for any review or substantial
work it requires.

## Decisions (running log)

**The findings were read from the review's commit, `3a6b741faf1e`, and graded
against the source.** That is `creator-reference-k59`'s committed body. Each
disposition below names what was checked.

**F1 is a contract stated unclearly, and the contract is repaired.** The naming
section of `references/retire.md` says *the review leaf you cut for it* and
never says the cut comes first. `references/decompose.md` calls the cut the
**last act** of the session, and the section itself follows retirement. A
session that takes the three in that order names the reviews already there,
then cuts a review that never gets a line. The step settles a line *under*
`**Reviews:**`, so the dependency is real, and `creator-lifecycle-k40`'s log
records its eighth mutation refusing for exactly this reason. The owner of the
step states the order. `references/decompose.md` keeps its sentence: *last act*
is this corpus's idiom for cutting the next step once the work is done, which
the brief format, the editorial family and the pass series use too. The naming
section now quotes it and says what still follows the cut.

**F2 is a real issue, reproduced before it was fixed.** Five mutants of the
fake session each passed all four lifecycle cases: settling only the first
review found, searching the grove root only, naming only the outermost closed
node, rewriting terminal reviews, and matching the handle anywhere in a line.
A sixth, `finish` not walking the cascade, failed the decomposed case, so the
instrument reads dirty when it should. The subject's digest was the same
before and after each run. The spec says the fake producers follow the
documented convention, and nothing observed the convention's set: every live
review, anywhere in the tree, of every producer the cascade finishes, and no
other file.

**F3 is noise raised for want of context, with one valid residue.** The
single-owner records file a rule that binds session kinds into the spine and
the `grove-<kind>` skills. `configure-grove` binds no kind: its reader is an
operator diagnosing a refused review, who performs no finishing step. The
flagged sentences are third-person description of what a session leaves
behind, and the contract requires them there: the spec's `#grove-integration`
and `creator-lifecycle-k40`'s `Done when` both have configure-grove explain
who writes or removes the line. What they restate is the spec's
`#identity-and-creator`, as the configuration reference, the usage guide and
the dispatch README do by `creator-lifecycle-k40`'s recorded choice of one
home and three shorter forms. The instruction against copying a run ID is the
operator's own rule, since the operator is the one positioned to write a line
by hand. The cost of four accounts is drift, and it stays accepted:
`dispatch-documentation-k41` consolidates against the configuration
reference's section. The residue is the review's second observation, which is
correct. `conformance.sh` sweeps the spine and the kind skills, so its clean
run says nothing about `configure-grove`. The brief claimed more than that,
and is corrected. The sweep's scope is left alone: it is the register the
ownership record defines, and widening it is a methodology design change.

**F4 is a real issue in the case's stated provenance, and the statement is
repaired.** The first session of the direct-finish case calls `name_run` while
its leaf is live and exits without retiring. The procedure retires first, and
says a leaf left live writes nothing, so no session that follows it produces
that line. The removal and declaration assertions are sound and stay. What
changes is what the case and the spec say it is: a stale line planted by a
session that broke the order, which a finishing session must not trust. The
case is not rebuilt around a node that closed and was reopened, where the
procedure itself leaves an earlier finish's line. The spec says later work on
a finished producer changes no reference, and whether re-closing a node is
such work is a design question no finding raised.

**The rule is *cut before you retire*, and it has no recovery clause.** The
first wording said to cut a review before settling it, and to take the step
again for one cut later. This leaf's one in-session reviewer read that wording
with no conclusion supplied, and showed the second half was wrong. A leaf cut
inside a node keeps the node open. So a review cut after the cascade, beside
an inner node, reopens an outer node the session has already named its run on
and reported closed. Retirement is what starts the cascade, so cutting before
it leaves nothing for a later cut to invalidate. The paragraph now states the
rule, says the `**Reviews:**` line is written at the cut, and names both
consequences of cutting afterwards without licensing it.

**The reviewer's other points, classified.** Valid and fixed in the same
section: the trigger *cut later* missed a review cut early whose body was
written late; *last act* was misquoted as the work's where
`references/decompose.md` says the session's; *step* had three referents and
the paragraph leaned on the bullets after it, so it moved below them; its lead
read as an order to cut a review, and is now conditional; a search for a
handle also finds a longer handle it begins, so the text says the whole
handle; a variable set but empty is not a run, so the first branch says *set
to a run ID*; and a node is named at the last of its close's four steps, so
the lead says that instead of leaving it to the section's position ahead of
the check. Noise: *replacing any line already there* leaving a second line,
which misreads *any*; `**Reviews:**` written with a bare key, which
`TASK-FORMAT.md` does not show and the adapter refuses as malformed; and
*live* being defined in `TASK-FORMAT.md`, which the section points to. A visible
trade-off, already the spec's: a finishing session with no run removes the
owner's early declaration with the rest.

**The rewritten paragraph is unreviewed, and that is stated where the next
review will read it.** The allowance is spent, and the rewrite is the
substantive one. A `review-impl` leaf for one paragraph was weighed and not
cut. The executable half of the rule is covered by a seam: the fake session
that cuts after it finishes fails the new case. The wording half goes to the
documentation-acceptance review `dispatch-documentation-k41` already cuts, and
the brief names the three shapes for it to read.

**F2's fix is one case, whose subjects are the set's boundaries.**
`a_close_cascade_settles_every_live_review_of_each_producer_it_finishes_and_no_other`
plants a second live review of `parser-k1` inside another node, a done and an
abandoned review of it, and a live review of `parser-k17` whose prose ends a
line with `parser-k1`'s own line. The session that closes both nodes cuts the
inner node's review, at the root, before it finishes. The case asserts the
sorted set of every creator line in the tree as each session ended, each
file's bytes, and that both reviews of `parser-k1` launched from the closing
session's run. The whole set is what makes *and no other* checkable. A list of
files proves only the files someone thought to list.

**Each control was seen to fail, against one frozen subject.** Ten mutants of
`loop_driver.rs`, each an exact replacement that had to match once, each
restored to digest `446877d25184`. The five that passed everything before the
fix now fail the new case: the two partial searches at the launch boundary,
where the unsettled review refuses, and the other three on the set of lines.
Prefix-only and suffix-only handle matching fail it on the `parser-k17`
review. Cutting the inner node's review after finishing fails it with two
lines named where three are expected. `finish` not walking the cascade fails
it and the decomposed case. A session with no run keeping the line it found
fails the direct-finish case. The wording pins added to
`composition_guidance.rs` each failed against the parent revision's
`retire.md`, put back in place for one run, and pass on the amended file,
restored to the digest below.

**The spec carries the order and the ADR set is unchanged.**
`#identity-and-creator` now says a session cuts a review before it retires,
and why. The launch-boundary row of `#test-seams` gains the cascade case and
calls the direct-finish line stale and planted. The paragraph under the table
names the one fake that breaks the convention. The creator-reference record
decides who the creator is and where the reference lives. The order of two
steps inside one session clears no part of the ADR bar.

**`bash scripts/check.sh` passes, all twelve checks.** It ran once, after
every edit to a file it reads, and nothing it reads changed under it. The
`retire.md` it read has digest `ad574250b8e5`. Conformance and its control
suite pass over the amended skills, the Codex provisioning test finds the
installed spine byte-equal to the plugin's, and the six walkthrough books
validate.
