# select-contract-k8

## Goal

harness-dispatch implements the specification's contract: `select` receives the
caller's parameters and the prompt and returns the command to run or a refusal.
The catalog, routes, slots and `--choice` are gone.

## Context

- `docs/specs/harness-selection-and-execution.md`: *Command interface*, *Policy
  and the selected command*, *Bounded context*, *Supplied review policy*,
  *Records and later observations*, *Diagnostics and exits*, and the command
  rows of *Agreed test seams and acceptance*.
- `direct-dispatch-k4`'s note *Where the design lands in the code*, the
  *harness-dispatch front* and *Worker and SDK* entries, and *Not to build*.

## Done when

- `--param`, the request's `prompt` and `params`, the two result variants,
  inspection's `command` object, inspection without a prompt, and the run
  record are as the specification states. A version-1 policy refuses.
- A run recorded under the catalog contract is still shown, observed and looked
  up, and a review still resolves its provider.
- The four examples, the Grove adapter and every type-check fixture are written
  to the contract. The dynamic example is deleted.
- The command-seam cases hold, apart from those the next two leaves own: owner
  settings, the deciding-agent stand-in, `init`, the sample and the choice
  file.
- Grove's own code is unchanged. Its tests that reach dispatch through an
  owner's command definition pass, with their policies rewritten.
- The dispatch README and `--help` describe this contract. The glossary terms
  whose mechanism goes, **Joint candidate** and **Selection provider** among
  them, are retired or reworded.
- The installed-layout smoke test and `bash scripts/check.sh` pass.

## Notes

- This is the largest leaf. Every policy, example and test changes shape
  together. If it proves bigger than one session, the seam is the policy schema
  version: accept version 2 beside version 1, move the examples and tests, then
  remove version 1. Decompose there and do not run long.
- Other documents change only as far as the checks require.
  `current-state-documents-k15` rewrites them.

## Decisions (running log)

**One session, not the version-2-beside-version-1 seam (2026-10-02).** The front
and worker changed shape in one pass and compiled, so there was never a point
where coexistence would have kept anything green that the single cut did not.
The test files are independent of each other, so their migration was split
across parallel agents inside this session.

**A new run writes `null` for `selection.selectedBy` as well as
`selection.form` (2026-10-02).** The specification names three values a new run
lacks: the candidate ID, the selection form and the explicit choice. The old
document recorded the form in two fields, `form` and `selectedBy`, one derived
from the other, so both are `null`. Chosen over writing `"select"`, which would
be a constant that says nothing. The labels stay under `candidate`, with
`candidate.id` `null`, as the design review's ruling 4 asks. `record show`'s
text leaves out the three catalog-only rows for a run that has none.

**Inspection's `command` holds exactly `program`, `args` and `executable`
(2026-10-02).** The old `executable` object's `resolvedBy` and `pathEntry` are
not carried into it: both follow from the program and the resolved path, and
the specification defines the object as those three. The text report still says
how the program was found, and the run record's own `executable` object is
unchanged. `proposedRunId` goes with the `runId` slot, since inspection reports
no run ID.

**The private protocol number stays 1 (2026-10-02).** The front refuses any
worker whose source digest differs from its own, so a worker of the old
conversation shape can never be paired with this front, and the tests that play
the front's side keep their frames.

**`definePolicy` takes and returns `Policy`, with no type parameter
(2026-10-02).** The generic form existed to keep a catalog's literal IDs. With
it, a literal carrying `catalog` or `routes` type-checked, because an excess
property is not checked against a type parameter's constraint. The plain form
refuses both at the type level, which the fixtures hold.

**The prompt marker is `<harness-dispatch inspect: no prompt was supplied>`,
which the SDK names `PROMPT_NOT_SUPPLIED` (2026-10-02).** The front sends it as
`request.prompt` and reports it as `prompt.marker`. A fixture policy compares
the two, so the Rust and TypeScript copies cannot drift unseen.

**A refused run's `inspect` report says the prompt is left out (2026-10-02).**
JSON carries it as `inspect.prompt`, and text as a second line under the
`inspect:` line, each saying that a policy which reads the prompt selects as it
did only when the same prompt is added.

**The examples' tables hold functions, not names (2026-10-02).** A route is
`(request) => command`, so a value reaches an argument because the entry puts
it there. `examples/static` exports `selectRoute`, which the other three reuse;
it is in an example and not the SDK, as the plan's *Not to build* asks. A review
entry maps each creator origin to a route. The origins an entry lists are its
keys, so the old pair of refusals, an origin the catalog lacks and an origin the
entry lacks, is one: `creator_origin_unlisted`.

**A context field named `catalog`, `candidate`, `candidates` or `routes` is an
ordinary unknown field (2026-10-02).** They were refused as executable names
because they were. `program`, `args`, `select` and the rest still say why.

**The catalog-contract fixture is a store 21.13.0 wrote (2026-10-02).**
`crates/harness-dispatch/tests/fixtures/catalog-contract/` holds the record
store the installed release produced, recorded while that release was still the
one installed, with the version-1 policy it evaluated. Chosen over planting a
hand-written launch document, which would test this release's idea of the old
shape and not the old shape.

**The result's 1 MiB message bound is left as specified, so a prompt near its
own bound cannot be returned in `args` (2026-10-02).** The prompt now travels
back in the result, and its JSON encoding can pass the fixed message bound
before the platform's argument limit is reached. The specification states both
bounds and nothing asks for the message bound to move, so the refusal's remedy
and the README say what to do: have the harness read a long prompt from a file
an argument names. The two tests that used a full 1 MiB prompt now measure the
prompt and the exec limit separately.

**Documents outside the dispatch README were changed only where a check reads
them (2026-10-02).** `docs/CONFIGURATION.md`'s `dispatch-deep` example passes a
literal `--param`, because Grove's launch-boundary suite holds every quoted
command definition to the one it launches, and the runtime-evidence account of
the installed smoke test follows the rewritten cases. A Grove command template
cannot place a substitution inside a word, so through an owner's command
definition only a literal parameter reaches a policy until Grove calls dispatch
itself. `docs/CONFIGURATION.md`'s dispatch section, `docs/USAGE.md`,
`CONTEXT-MAP.md` and the configure-grove skill still describe the catalog,
routes and `--choice`. `current-state-documents-k15` sweeps for exactly that.

**`record show` and inspection share one parameter row (2026-10-02).** A value
holding a space is quoted in both, so two parameters cannot run together.

**This leaf gets a review, `select-contract-k16`, ahead of `owner-settings-k9`
(2026-10-02).** It is the contract four more leaves build on and a breaking
release ships, and its tests were migrated by agents whose files the producer
spot-checked and did not read whole. The review's body carries the doubts this
session could not close. It is inserted before the next leaf so that a finding
is integrated before the settings and the sample are built on it. The
in-session reviewer was not spent. This session had no dispatch run
(`HARNESS_DISPATCH_RUN_ID` was unset), so the review carries no `**Creator:**`
line.
