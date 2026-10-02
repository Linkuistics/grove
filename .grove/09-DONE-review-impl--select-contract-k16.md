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

## Findings

### F1 — P2: inspection can report a different executable from the one resolved

`crates/harness-dispatch/src/inspect.rs:107` builds the newly executable
`command.executable` contract with `choice.executable.path.to_string_lossy()`.
`program::resolve` accepts PATH as `OsStr` and joins each entry without a UTF-8
check (`src/program.rs:130`), while `run` executes the original `PathBuf`
(`src/run.rs:100`). For example, on a supported Linux target a UTF-8 program name
found under a PATH directory containing byte `0xff` resolves successfully, but
inspection replaces that byte with U+FFFD. Executing the JSON path then fails to find the selected
file, or executes a different file if that replacement spelling exists.

The specification's *Command interface* requires a caller that launches the
command itself to execute `command.executable` without resolving again. The
old report's lossy descriptive path cannot satisfy this new operational
contract. Preserve the resolved path exactly or refuse an unrepresentable
executable before emitting a successful proposal. The paired integration should
cover a non-UTF-8 PATH directory and, if needed, a non-UTF-8 cwd for relative
resolution. This is a source-derived counterexample; this inspection-only
review did not run it.

## Rulings on the producer's doubts

1. **The inspected deletions remove catalog behavior, with surviving checks
   retained.** All five tests in deleted `tests/choice.rs` concern the removed
   `--choice` rule. The dynamic-example deletions concern an example the leaf
   explicitly removes. `tests/run.rs`'s deleted missing-slot test concerns
   expansion, while optional input absence is checked by the rewritten
   task-file case and `tests/select.rs`'s recorded request. The old unknown-choice
   loader test is replaced with a malformed-policy/valid-policy pair that still
   checks validation before loading context. Exception, unsettled promise,
   abstention and malformed-result cases survive in `tests/select.rs`.
2. **Fixture-built argv is the intended new boundary.** `tests/select.rs` and
   `tests/run.rs` compare the launched fake harness's arguments with the returned
   command, and `tests/params.rs` checks caller data through selection, launch,
   inspection and records. The front's missing/unexecutable program and NUL
   cases still assert refusals before a record or harness appears. The old slot
   expansion assertions need no substitute, because policy now builds argv.
   No additional actionable tautology was identified in those inspected cases.
3. **`selectedBy: null` is consistent.** The specification's *Records and later
   observations* removes selection-form evidence. `record::launch` nulls both
   fields that described it; text export omits null rows, and lookup reads labels
   rather than either field. `tests/records.rs` asserts the new document and
   separately checks export of the old route and explicit-choice documents.
4. **The new object need not preserve the old resolution annotations.** The
   specified `command` fields are `program`, `args` and `executable`; resolution
   explanations remain in text and in launch records. The producer's claim
   that the original `pathEntry` spelling follows from the other two fields is
   too strong for relative PATH entries, but that spelling is not required by
   the specified execution interface. F1 addresses its material path defect.
5. **The result bound is an explicit limit.** Both independent limits are in
   *Bounded context*. Returning the entire near-limit prompt leaves no room for
   the rest of its result; the producer documents that consequence and adds the
   `tests/bounds.rs` refusal case. Raising the fixed message limit would change
   the design, rather than repair an implementation deviation.
6. **The review rule implements the specified checks.** It refuses a missing
   lookup or launch failure, an exact-origin non-member, and a mapped command
   with the creator's provider label. The rewritten `tests/review.rs` gateway
   case preserves the former explicit-choice test's surviving same-origin
   behavior, including retries and a distinct program/model/argv. Provider
   labels remain owner assertions, as the specification explicitly accepts.
7. **The nongeneric `definePolicy` is appropriate to this contract.** It retains
   contextual callback typing against `Policy`; the literal candidate-ID
   inference it once preserved has no corresponding contract field now.
   `worker/typecheck/refused-shapes.ts` checks catalog/routes rejection, missing
   result fields, obsolete slots and the narrower selection host. Erasing a
   literal policy version is not a required capability.
8. **The inspected README fragments follow the implementation.** Request and
   result forms, flattened lookup labels, null record fields, marker behavior
   and review refusal names agree with the source. Its 120-second timeout ceiling
   describes this increment; owner settings and the raised ceiling belong to
   `owner-settings-k9`. Grove configuration wording outside this leaf's contract
   remains explicitly assigned to later leaves.

## Verification and limits

This review inspected producer commit `086fd972` against its parent, the current
front/worker/SDK/examples, the contract sections named by the task, targeted
migrated tests and the old deleted tests. It is a task-directed source review,
not a complete audit of every test body or runtime platform. No test, build,
lint, format or smoke command was run, as the review procedure requires.
The producer's retired task and commit description contain no fresh full-check
transcript; the runtime-evidence document includes historical release results,
which do not establish that this producer's exact commit passed. Integration
owns executing the counterexample and the required post-fix checks.

## Decisions (running log)

**F1 is actionable (2026-10-02).** The new `command` object is an execution
input for the next standalone-selection leaf, so a successful proposal must
not substitute a path. This is worth integrating before the next producer.

**Review evidence uses current source (2026-10-02).** Tier 2 graph discovery
found the right subsystem, but coverage metadata reports generation
`2026-10-02T02:27:48Z` and changed metadata for the producer's source files;
the graph still carries the catalog contract. The examples, scripts and fixture
directory are excluded. Current source and commit `086fd972`'s diff therefore
provide the evidence, with direct reads for stale or excluded material. No
test, build, lint or format command runs in this review.

**The remaining producer doubts do not earn fixes (2026-10-02).** The rulings
above distinguish removed behavior, surviving contract checks and explicit
design costs. They do not treat source inspection as a passing test run.
