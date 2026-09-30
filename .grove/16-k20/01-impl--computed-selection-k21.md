# computed-selection-k21

## Goal

Replace the "`select` not yet supported" refusal with the computed form. An
asynchronous `select` chooses a configured candidate, or refuses, from the
versioned request. Explicit choices under `select` are accepted, refused or
caught as a mismatch.

## Context

The contract is the spec's `#policy-and-choice`. The request here carries
`schemaVersion`, `kind`, `cwd`, optional `taskFile`, `taskId` and
`explicitChoice`, and the effective `limits`. Caller and loaded context arrive
with `bounded-context-k22`. Until then, pass an absent context and refuse
`--context` explicitly.

## Done when

- A policy with both or neither of `routes` and `select` refuses. `select` may
  be synchronous or return a promise. `selected` requires a configured
  candidate ID and a nonblank reason. `refused` requires a code, message and
  remedy, and inspection and `run` report them with exit 3.
- Exceptions, a promise left unsettled once the worker's event loop drains, an
  unknown ID, abstention and malformed results refuse with distinct codes. A
  result cannot supply argv. Rust validates the result against the catalog
  snapshot.
- The whole-selection deadline from `selection-deadline-k44` ends a `select`
  that awaits a promise kept pending by a live timer. It also ends one that
  spins synchronously. Command-seam tests show each exiting 124 with no
  fake-harness marker and no surviving worker. Variants that finish within the
  bound, seen to select, are the positive controls.
- With `--choice`, `select` receives `explicitChoice`. Returning the same ID
  accepts it and returning `refused` refuses it. Any other ID is
  `explicit_choice_mismatch`, including one the policy calls a fallback.
- Inspection distinguishes computed selection from a route and from an explicit
  choice under `routes`, and it shows the policy's reason.
- The deterministic dynamic example is registered as an embedded specifier,
  with its declarations and readable source. A command-seam test selects
  through it.
- The per-target installed smoke gains the computed TypeScript case, and it
  passes on every target. The archive assertions include the new example.
