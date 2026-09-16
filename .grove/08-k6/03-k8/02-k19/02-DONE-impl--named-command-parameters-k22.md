# named-command-parameters-k22

## Goal

Complete named-command-reuse-k19 with declared parameters/defaults and safe
whole-word or embedded parameter substitution.

## Context

Build on named-command-targets-k21. The parent and modular specification own the
full contract; parameter assignment/removal patches still belong to k20.

## Done when

- Parameter declarations/defaults validate structure, names and duplicates. Required
  parameters are demanded only for admitted routes, even when unused in template text.
- Named templates compile parameter fragments before substitution, retaining exact
  word counts, opaque adversarial/empty values and native runtime strings. Reject
  active NUL, unknown/unterminated references and executable substitutions.
- Parameter defaults and multi-origin words appear in inspection; histories and
  reference replacements remain correct and Catalog/convenience loading agree.
- Public generic tests and Grove fake-launch/mutation acceptance demonstrate shared
  defaults, edited defaults, errors before use and all applicable parent criteria.
- Shared/route param/unset patches and profile/selection syntax still fail explicitly.
- All affected documentation/books describe the delivered subset and root checks pass.

## Notes

This is the final child of k19: audit its entire brief, including the existing
reference behavior, before closing. k20 continues to own override maps, removals,
missing-target patches and reset histories for route parameter maps.

## Decisions (running log)

- Keep parameter fragments internal to named compilation; substitute defaults
  only after shell-word splitting and schema validation. Public CompiledWord
  stays literal-or-runtime-slot, shared by inspection and expansion.
- Retain every parameter declaration span, with default assignments in
  ParameterDefault histories. Require values on admitted routes (including
  unused declarations); compile active bindings without demanding required values.

## Implementation sequence

1. Add public-seam tests for declarations, safe fragments, validation scopes and
   multi-origin inspection; observe failure before extending named.rs.
2. Extend declaration capture, compile fragments, instantiate defaults and
   publish parameter provenance without changing the public expansion seam.
3. Exercise exact fake-launch argv, edited defaults and refusal before mutation;
   update current-boundary docs and source-exact walkthrough fragments.
4. Run focused tests and scripts/check.sh, audit k19, retire and commit.

- The public fragment/compiler tests failed against the prior unsupported-syntax
  boundary and now pass. The focused fresh-context review reported no findings
  across capture, scanning, validation scopes, replacements and provenance.
- Only keyed-launch's source-exact book owns changed production bytes; Grove
  acceptance changes are integration-test evidence outside book corpora. No
  corpus exceptions or public API variants were added.
- The first principal run exposed result_large_err in the internal compiler.
  Box the diagnostic at that private boundary; focused clippy passes. The failed
  run was stopped before edits, and all tracked input digests remained unchanged.

## Verification

`bash scripts/check.sh` passes all eight principal checks, including workspace
unit/integration tests and final reconstruction of all six walkthrough books.
The keyed-launch book reconstructs 11 files and 3,399 lines with no deferred
ranges. The inventory and SHA-256 digests of all 1,799 tracked files matched
before and after the final run (sources, tests, manifests, docs/books, plugin
checks and task artifacts included).

The preceding full run encountered the unchanged TUI observation fixture's
Running assertion; its isolated test and complete seven-test unit suite passed,
then the unchanged full principal run passed, including that exact test. No
TUI changes were made. The earlier compiler lint correction was storage-only.

The k19 audit is complete: public tests cover schemas, default reuse/reload,
opaque embedded and whole-word values, escapes, runtime native strings/NUL,
required unused declarations, active/dormant scopes, target replacements and
source-backed inspection. Grove fake launches and leaf mutation cover exact
argv and refusal before use. Existing flat and target-reuse tests remain green.
The implementation follows the existing ADRs without changing their decisions;
k20 retains parameter patches and k9 retains profiles/selections.
