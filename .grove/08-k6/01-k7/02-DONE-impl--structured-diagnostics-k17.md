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

## Decisions (running log)

- Keep the existing flat validator and scanner. Add Diagnostic records at the
  refusal sites, retaining human Display output and the reviewed Source types.
  Runtime/vocabulary failures carry no invented span; runtime failures retain
  the affected key and winning source when known.
- Read and validate both explicit documents, then aggregate by primary/overlay
  and byte position. Structural/read/syntax errors suppress template-semantic
  reports until structure is valid. Malformed nodes do not compile templates.

- The single in-session doubt review found two actionable gaps: only the first
  unknown selection was reported and overlay-only refusals discarded available
  declaration spans. Red/green public tests now cover every selected occurrence
  (including repeats and its index) and retained related overlay spans after
  source removal. Both fixes are conclusively covered at the public seam; no
  second reviewer is needed.
- The old mixed structural/semantic aggregation test assumed one-stage reports.
  The reviewed contract requires structure first. Keep its semantic assertions
  against structurally valid input and test staged suppression separately.
- KDL 4.7.1 is locked. Its installed upstream `node.rs` documents parsed spans
  and mutation caveats; captured documents are never mutated. docs.rs could not
  be fetched by the web tool; the checked dependency source and UTF-8 slice tests
  supply the API evidence, with the versioned URL beside the call.

## Implementation plan

1. Add public-seam tests in `crates/keyed-launch/tests/diagnostics.rs` for source
   read, syntax, shape, duplicate, invalid template, vocabulary, selection,
   require and expansion refusals, with exact UTF-8 slices and remedies.
2. Extend `error.rs` and exports with Diagnostic/Occurrence; annotate validator
   locations with byte ranges and convert its reports without parsing messages.
   Collect both document results in Catalog load and retain contextual names.
3. Run keyed-launch tests, update all affected source-exact book fragments,
   ledgers and current-implementation notices, then run `bash scripts/check.sh`.
4. Audit captured-configuration-k7, retire, commit and seal, then signal.

## Validation notes

The first root check passed seven checks and exposed one Grove adapter test
with the old mixed structural/semantic expectation. The corrected public test
first asserts structural diagnostics, repairs those declarations, then checks
the remaining template error against the same local source. All 1,790 versioned
inputs had unchanged before/after SHA-256 digests during that run. Its books
passed; a final prose audit also corrected stale descriptions of the retained
span map and source-position ordering before the final run.

The final principal-check run passed formatting, shellcheck, clippy, plugin
installation, both conformance checks and all six final book validations.
Workspace testing exposed one intermittent failure in the unchanged TUI
`witnessed_launch_identity_change_is_compared_even_when_tree_capture_fails`
fixture, at its initial locked process-observation assertion. That exact test
passed in isolation without edits, then `cargo test --locked --workspace`
passed in full, including that fixture and all configuration tests. The same
1,790 versioned inputs retained identical SHA-256 digests across the principal
run and the complete workspace rerun. The intermittent fixture remains an
observed testing limitation; this leaf changes no TUI or observation code.

## Parent contract audit

`captured-configuration-k7` is satisfied. The predecessor's public Catalog tests
cover captured source ownership, absent flat selections, deterministic keys,
primary authority, convenience-loader equivalence, native argument bytes,
non-Grove vocabulary and conformance after input removal. This leaf supplies
structured load/resolve/require/expand refusals, actual UTF-8 spans, related
duplicate and overlay-only declarations, every external selected occurrence,
remedies and structural-first cross-document aggregation. Both vocabularies
reserve `param.`. Conformance rejects empty admitted sets and reports resolution
failures without rereading. Retained source bytes and parsed documents remain
available to the next provenance leaf; no inspection stub was added. The user
reference, interface specs and source-exact books match this delivered boundary.
The existing ADRs already describe the agreed diagnostics contract, so no new
decision record is warranted.
