# sample-policy-k10

## Goal

`harness-dispatch init` installs a sample policy that is the owner's pinned
configuration converted, and a supported helper lets one checkout choose among
the selections that policy offers.

## Context

- `docs/specs/harness-selection-and-execution.md`: *The sample policy and the
  choice file*, `init` under *Command interface*, *Delivery and release*, and
  the sample row of the test seams.
- Root brief, requirements 11, 12 and 13.
- `parity-fixture-k7` recorded the fixture the sample is compared against.

## Done when

- The SDK's choice-file helper behaves as the specification states and is
  tested as delivered: an offered name selects, an unoffered name refuses
  naming the file, the name and the names offered, and an absent file leaves
  the default.
- For every kind under every selection it offers, the installed sample selects
  the program and arguments in the parity fixture. Its default is `claude-led`
  with `codex-sol`. It refuses a caller that did not pass a parameter the
  selected command needs.
- `init` writes the sample to the personal default path, refuses when anything
  is already there, and reports that the sample runs `codex` with approvals
  off. `run` and `inspect` with no policy refuse as `policy_missing`, naming
  `init`.
- The sample ships in the release archive and the installed layout, and the
  installed-layout smoke test installs it with `init` and inspects it.
- The dispatch README covers installing the sample and the choice file.
- `bash scripts/check.sh` passes.

## Notes

- The front carries the sample's text, so `init` needs no worker.
- No table helper goes into the SDK. The sample's tables are its own code.
