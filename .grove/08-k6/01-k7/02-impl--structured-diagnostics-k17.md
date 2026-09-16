# structured-diagnostics-k17


## Goal
Expose stable structured diagnostics for captured flat configuration loading,
resolution, require and expansion, with real locations and actionable remedies.



## Context
Build on `flat-snapshots-k16` and the parent's reviewed Diagnostic/SourceSpan
contract. Do not build another loader or expose inspection stubs.

## Done when
- ConfigError::diagnostics returns the reviewed records and stable categories
  for every load, resolve, require and expand refusal.
- Retained source text and parsed spans supply real paths and UTF-8 byte ranges;
  unavailable locations remain absent. External unknown selections report
  `unknown_profile` with no fabricated source span.
- Independent structural errors aggregate across readable primary and overlay
  documents deterministically, without semantic cascades from malformed input.
- Public tests cover categories, paths, ranges, remedies, names and no-source
  cases, including conformance semantic failures. Existing legacy tests pass.
- Docs and source-exact books describe the delivered records; root checks pass.
- Audit and close the parent captured-configuration contract before retirement.

## Notes
This child owns the diagnostic conditions of the parent; captured inputs,
vocabulary reservation and conformance migration land in its predecessor.
