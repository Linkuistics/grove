# run-observations-k25

## Goal

Let any observer attach later evidence to a run and read it back. That covers
execution confirmation, exit, duration, usage, acceptance, findings, repair and
human work. Unknown and unobserved values stay explicit, and the launch fields
stay immutable.

## Context

The contract is the envelope and the measurement rules in the spec's
`#records-and-outcomes`. The package validates shape and association, not
external truth. Analytics, scoring aggregation and any learned-selector
benchmark are out of scope.

## Done when

- `record observe --run R --file F` validates a version-1 envelope. The
  envelope holds `schemaVersion`, `observationId`, `runId`, `source`,
  `observedAt`, `evidence`, optional `supersedes` and `measurements`, and
  validation refuses unknown versions and fields with their location. The
  observation is appended atomically.
- Each measurement has `state` equal to `observed`, `unknown` or `unobserved`.
  `observed` requires a typed value, and quantities require a unit. The other
  states carry no numeric value.
- The supported fields cover execution confirmation, exit or signal, duration,
  input, output and total usage with units, acceptance, missed defects, false
  findings, downstream repair, human-work measures and evidence links.
  Findings take stable IDs and may reference later repair observations.
- Routing probability fields keep `choiceProbability` distinct from
  `successProbability`. A success estimate must name its calibration data or
  version, or be labelled uncalibrated.
- A repeated identical ID and content is idempotent, and a conflicting repeat
  refuses. A correction names what it supersedes, and both are retained. No
  import changes launch fields.
- `record show` presents unsupplied fields as unobserved. A run with an
  execution confirmation is distinguishable from an attempt alone.
- Command-seam tests round-trip observe and show. They cover idempotency,
  conflict, supersession, unit and state errors, and observations after the
  task tree that named the run has been deleted.
- Help and the usage documentation carry an observation example.
