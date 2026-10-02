# standalone-selection-k11

## Goal

`grove run KIND` selects its command with `harness-dispatch inspect --json` and
the runner launches the reported file under confinement. No Grove configuration
is read for it.

## Context

- `docs/specs/harness-selection-and-execution.md`, *Grove integration*:
  *Finding it*, *A standalone invocation*, *A refusal* and *The runner*; and
  the confinement and runner rows of the test seams.
- `docs/specs/standalone-invocations.md`, *Selection*.
- Root brief, requirements 6, 10 and 14.
- `direct-dispatch-k4`'s note *Where the design lands in the code*, the *Runner*
  and *Grove* entries.

## Done when

- Grove finds `harness-dispatch` beside its own executable, from its own real
  path, and reports a missing one with that path.
- `grove run KIND` stages, selects outside the sandbox and launches inside it,
  as the specification states. A refused, cancelled or timed-out selection
  launches nothing and publishes nothing.
- The runner's argv has a public constructor. A confined launch takes an
  absolute program path, refuses any other, and does no PATH lookup of its own.
- The cases of the `grove run` confinement seam hold, and a `config.kdl` or
  `.grove.kdl` on disk changes nothing about `grove run`.
- The release tooling's tests that run `grove run release-notes` work from a
  policy.
- The overview and `keyed-launch` books are valid for what changed.
- `bash scripts/check.sh` passes.

## Notes

- Lifecycle sessions still launch from configuration after this leaf.
  `lifecycle-launch-k12` moves them, and reuses the lookup and the constructor
  this leaf adds.
- A standalone invocation gets no run record and no launcher on `run`.
