# computed-policy-k20 — brief

## Goal

Let an owner compute a selection. `select` may be asynchronous, and it sees the
versioned request and a measured context assembled from caller JSON and a
`loadContext` callback, all within the documented bounds. Inspection shows each
delivered source, its hash and size, and the effective limits. Explicit choices
are policed under `select`.

## Done when

- A policy exports exactly one of `routes` or `select`. `select(request,
  context, host)` may return a promise. It returns either `selected` with a
  candidate ID and a nonblank reason, or `refused` with a code, message and
  remedy. Exceptions, an unresolved promise, an unknown ID, abstention and
  malformed results refuse. Rust validates the result and the returned catalog
  snapshot, and a result cannot add executable words.
- Under `select`, an explicit choice is visible to context assembly and to
  selection. The policy must explicitly accept or refuse it. Any other returned
  ID is `explicit_choice_mismatch`, whatever reason the policy gives.
- `--context PATH` accepts a version-1 caller context, read as data: summary,
  acceptance criteria, facts, attributed assessments, sources and
  `reviewedArtifact` with its creator forms. Unknown versions and fields, and
  executable fields, are refused with their location. Empty collections stay
  distinct from unknown facts.
- `loadContext(request, host)` and the SDK operations `readText`, `readJson`,
  `diagnostic` and `signal` behave as specified. Reads resolve against the
  request cwd, and each source is measured, SHA-256 hashed and attributed. A
  failed required read or loader fails the whole selection.
- The spec's resource bounds hold, except the timeout, the lock wait and the
  prompt budget, which other increments own. The bounds are context bytes
  (`--context-bytes`, up to 8 MiB), per-source reads, the source count, the
  result/catalog message size, and the combined diagnostics bound. Each
  overflow refuses and is never truncated.
- Inspection reports the context sources, hashes, actual source bytes, final
  encoded bytes, the effective limits and the explicit choice. It reports that
  selection was computed, with the policy's own reason.
- The dynamic example is a deterministic callback registered under its
  `harness-dispatch/examples/…` specifier. The installed smoke gains the computed
  TypeScript case, and it passes on every target.

## Decomposition

1. `computed-selection-k21`: `select`, its result contract, explicit-choice
   policing, the dynamic example and the computed smoke case.
2. `bounded-context-k22`: caller context, `loadContext`, the SDK reads and
   diagnostics, measurement, the resource bounds and context inspection.

## Pointers

- Spec sections: `#policy-and-choice`, `#bounded-context` and `#command-interface`
  (`--context`, `--context-bytes`, the JSON schema rules).
- Run lookup (`host.run`) is `run-lookup-k26`'s. Refuse it explicitly until
  then. The whole-selection timeout is `selection-cancellation-k28`'s.
