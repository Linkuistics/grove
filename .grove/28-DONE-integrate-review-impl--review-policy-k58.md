# review-policy-k58

**Integrates:** review-policy-k57

## Goal

Independently triage the committed findings in `review-policy-k57` against the
`review-policy-k35` implementation. Apply the findings that hold, explain each
disposition, and verify accepted changes through the appropriate existing
command seams and project tasks.

## Context

Read `review-policy-k57` from its task commit, found by the stable handle.
The reviewed producers are `review-selector-k36` and
`grove-review-adapter-k37`, committed as `cb5b12d9` and `d3e44d41`.
The report supplies source coordinates, failure scenarios and the evidence
limits of the inspection-only review. It does not establish that every
finding requires a fix.

The area spec and `docs/adr/a-review-carries-its-creator-reference.md` own
the contract. Keep the generic command free of Grove review semantics. The
next `creator-reference-k38` node already owns the methodology amendment and
real Grove lifecycle cases; reconcile the boundary with that work rather than
absorbing it into this leaf.

## Done when

- Every finding in `review-policy-k57` has an evidence-based disposition in
  this leaf's running log; rejected findings explain why.
- Accepted changes are implemented, their relevant executable checks pass,
  and required project checks and any applicable installed-smoke obligations
  are completed. The review's prior evidence is not a substitute for
  verification after fixes.
- Durable documentation and any handoff needed by `creator-reference-k38`
  agree with the final implementation.

## Decisions (running log)

**Evidence boundary.** Findings read from `review-policy-k57`'s own commit,
`daac05da`. Each was re-derived from the current source, which is the
reviewed `d3e44d41` plus only `.grove/` changes.

**F1 — real issue, fixed in the generic remedy.** `choice::context_breach`
takes a read whose `maxBytes` equals the budget as a read's own limit, and
`context::source_too_large` then advises a larger `maxBytes` "up to the
context budget", which the read already has. The same impossible remedy
reaches a default read under a budget below 64 KiB, whose limit is the
budget too (the `--context-bytes 1024` case in `tests/bounds.rs`). So the
defect is the generic diagnostic's, which the adapter makes ordinary. The
fix stays in the core and carries no Grove semantics. When a read's limit
already equals the budget, the remedy names `--context-bytes` and its
ceiling. At that ceiling it names only a smaller source. The bound, the
refusal code and the whole-file read are unchanged. The adapter cannot
repair it itself, because a breach the worker records is reported whatever
the policy does with the error.

**F2 — real issue, fixed by qualifying the promise.** The current
`TASK-FORMAT.md` still says no body records how a past session ran, and
`references/retire.md` says a waiting review needs no such record. The
README (lines 705–711 and 773–778) and `examples/grove-review.ts` (line 36)
nonetheless say a finishing session writes or removes the line. Two more
surfaces say it: the adapter's header claims `TASK-FORMAT.md` documents the
convention, and its `creator_line_missing` remedy tells the owner the
finishing session writes the line. The Grove-side guidance
(`docs/CONFIGURATION.md`, `docs/USAGE.md`, configure-grove's `dispatch.md`)
already names the run without saying who writes it, as the brief states.
Fix these four surfaces to the same neutral form: the line names the
dispatch run that finished the producer, and the owner writes whichever form
applies. Do not write the methodology here; `creator-methodology-k39` owns
it, and `creator-lifecycle-k40`'s documentation restores "who writes or
removes the line" once it ships. Hand that list of surfaces to k40.

**F3 — real issue, fixed in the regression guard.** An embedded example
composing through `harness-dispatch/examples/grove-review` is bundled with
the adapter and never calls the registration callback for it. So unless it
joins `bringsAdapter` it reports `null`, and the test, seeing no direct
`from "harness-dispatch/grove"`, expects the same `null`. The worker's
explicit set stays: deriving it at build time would need a bundler plugin
for a one-member set. The test's expectation becomes transitive over the
specifiers each example quotes, with its relative imports refused. Its
residue is a specifier built at run time, and the test's comment names it.
The control is a temporary embedded example that composes through
`grove-review` and is left out of `bringsAdapter`. The old test must pass it
and the new one must fail it, before the example is removed.

**F1 verified.** The new command-seam test
`a_read_that_already_takes_the_whole_budget_is_remedied_by_the_budget`
(`tests/bounds.rs`) covers a read below the budget, a `maxBytes` that follows
the budget, a default read under a 1 KiB budget, and a read at the 8 MiB
ceiling. The Grove seam's overflow case in
`a_task_file_that_cannot_be_read_refuses_naming_it` now asserts the remedy
too. Against the unpatched `context.rs` both fail. There the shipped
`grove-review` example's remedy was "read it with a larger maxBytes, up to
the context budget of 50000 bytes", the review's predicted scenario. Both pass
with the fix. The sibling remedies in `context_too_large` and the
over-budget `maxBytes` branch keep an achievable alternative at the ceiling,
so they stay as they are.

**F2 applied.** The README's form description and its missing-line remedy,
`examples/grove-review.ts`'s header, and the adapter's header and
`creator_line_missing` remedy now name the run the line refers to. They make
the owner the line's writer until Grove's sessions do it. The remedy also
says to name the finishing session, not an earlier attempt, since a run
reference is its writer's word. The adapter's `version` stays `"1"`: what it
reads and how are unchanged. The spec needs no change, since its notice lists
only delivered behavior and the methodology is not among it. The command-seam
assertions on the `creator_line_missing` remedy still hold.

**F3 verified.** The control was a temporary `grove-review-mine` example that
re-exports `groveReviewSelector`, registered in the embedded table but not
in `bringsAdapter`. The old test passed against it. The new test failed on it,
reporting null where a report was expected. With the example added to
`bringsAdapter` it passed. The example and the `main.ts` registration are
removed, and `main.ts` is byte-identical to the reviewed source. The guard
now carries two positive controls: the Grove example brings the adapter,
and a synthetic source composing only that example brings it too.

**Handoff.** The root brief's review-policy facts now say the dispatch side
holds the same interim. `creator-lifecycle-k40`'s body names the four
qualified statements, and the remedy's test assertions, for it to make
current when the methodology ships. The README's `source_too_large` guidance
for Grove already names `--context-bytes`, so it agrees with the fix.

**Verified.** `task check` passed after the last edit: all 12 principal
checks, including clippy at deny, fmt, the worker, probe and type builds,
every suite and the book check. The new and changed tests ran unfiltered.
The worker's sources changed, so the brief's per-target installed smoke was
owed. `task release:smoke` passed on `aarch64-apple-darwin`,
`aarch64-unknown-linux-gnu` and `x86_64-unknown-linux-gnu`, with the
glibc 2.17 and CPU-floor controls firing. Every archive's worker reports
build `648c690ce979…`, built from the final sources after the control example
was removed. The working-copy diff digest was the same before and after both
runs. No reviewer was spent: each code fix is covered by a command-seam test
seen to fail without it, F2 is wording checked against the methodology files,
and no redesign remains to externalise.
