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
