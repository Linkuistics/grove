# select-contract-k16

**Reviews:** select-contract-k8

## Goal

Read `select-contract-k8`'s implementation of the harness-dispatch contract
adversarially, before four more leaves build on it: the front, the worker and
SDK, the four examples and the adapter, the migrated tests, and the dispatch
README.

## Context

- The producer's commit names `select-contract-k8`. Its running log holds each
  call it made and what the call was made against.
- `docs/specs/harness-selection-and-execution.md` is the contract. The producer
  held to *Command interface*, *Policy and the selected command*, *Bounded
  context*, *Supplied review policy*, *Records and later observations*,
  *Diagnostics and exits* and the command rows of *Agreed test seams and
  acceptance*. Owner settings, the deciding-agent stand-in, `init`, the sample
  and the choice file are later leaves' and are not gaps.
- The test files were migrated by six parallel agents inside the producer's
  session, one group of files each. The producer read their reports and
  spot-checked their files, and did not read every rewritten test.

## Done when

- Findings are recorded here, each with its evidence, and an integration leaf
  is cut only if one is worth acting on.

## Notes

The doubts the producer could not close for itself:

1. **Coverage lost in the test migration.** Tests whose subject went with the
   catalog were deleted, `tests/choice.rs` whole, and several were replaced by
   a successor. Check each deletion against the old file: was the whole subject
   gone, or did a surviving behaviour lose its only test? The cases to look at
   hardest are the ones where an old test covered two things, one of which
   survives.
2. **Tests that pass for the wrong reason.** A policy in a migrated test now
   builds its own argv, so a test can assert what its own fixture wrote and
   nothing the front did. Look for assertions that would still pass with the
   front's validation, resolution or recording removed.
3. **`selection.selectedBy` is `null` in a new run's launch document.** The
   specification names three values a new run lacks. The producer read the
   selection form as both `form` and `selectedBy`. Is `null` right, or does
   something that reads records depend on `"select"`?
4. **Inspection's `command` object drops `resolvedBy` and `pathEntry`.** They
   were reported under the old `executable` object. The text report still says
   how the program was found. Does any reader of the JSON lose something it
   cannot derive from `program` and `executable`?
5. **The 1 MiB result bound now bounds a prompt returned in `args`.** A prompt
   near its own 1 MiB bound refuses as `message_too_large` before exec. The
   producer left the bound as specified and documented the remedy. Is that a
   consequence the specification intends, given that it states the prompt's
   bound separately?
6. **The review example's rule against the specification.** The origins a
   creator may have are now the review entry's own keys, and an unlisted
   recorded origin and a misspelt declaration share one refusal,
   `creator_origin_unlisted`. Check the four checks in *Supplied review policy*
   against `worker/examples/review.ts` line by line, the gateway case included.
7. **`definePolicy` lost its type parameter.** That is what makes a `catalog` or
   `routes` field a type error. Does an owner's policy lose an inference it
   had a use for?
8. **The README's fragments.** Its whole-module TypeScript snippets were
   type-checked against the shipped declarations once, by hand. Its fragments,
   its JSON examples and its refusal table were not checked by anything. Read
   them against the code.
