# bounded-context-k22

## Goal

Give policy a bounded, measured context. Caller JSON arrives through
`--context`, and a `loadContext` callback reads sources through the SDK. Every
delivered byte is attributed and hashed, and it stays within documented limits
that refuse instead of truncating. Inspection shows exactly what selection saw.

## Context

The contract is the spec's `#bounded-context`, plus the caller-context shape in
`#policy-and-choice`. The request `cwd` is data. It is not the worker's runtime
cwd, which stays private. Policy imports keep module-relative resolution. SDK
reads resolve explicit relative paths against the request cwd.

## Done when

- `--context` accepts only a version-1 caller context of the specified shape,
  including `reviewedArtifact` with an ID and at most one creator form, `run`
  or `declared`. Unknown versions, unknown fields and executable fields refuse
  with their location.
- `loadContext(request, host)` returns the final serializable context, which the
  worker and Rust both validate and measure. The `select` callback receives
  that measured value.
- `host.readText(path, maxBytes)` and `host.readJson(path, maxBytes)` return the
  content with its canonical source name, byte count and SHA-256. Other
  operations are `host.diagnostic(text)` and the `host.signal` abort signal.
  `host.run` refuses as not yet supported.
- Each bound holds and each overflow refuses by name:
  - context 256 KiB by default, up to 8 MiB through `--context-bytes`;
  - one source read 64 KiB, capped at the context budget;
  - 256 sources;
  - a 1 MiB result/catalog message;
  - 256 KiB of diagnostics across both streams, drained within the bound, with
    excess ending evaluation with an output-limit error.
  None of these truncates. Policy cannot raise a ceiling after evaluation
  starts.
- Inspection reports the sources, digests, actual source bytes, final encoded
  bytes, effective limits and any bound errors. Policy diagnostics never
  corrupt `--json` output.
- Command-seam tests cover a missing required source and a failed loader. They
  cover each overflow at and just past its bound, with the boundary case seen
  to pass. They also show that an oversize prompt does not count against the
  context budget.
- The usage documentation covers computed policy, context and the limits. The
  spec's notice states what is delivered. The node brief's `Done when` holds.
