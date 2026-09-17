# modular-input-k11

## Goal

Reject flat and mixed configuration through both public loaders, preserving
modular and empty-document behavior.

## Done when

- Both sources reject unsupported top-level declarations, including grammar-word
  keys, with source spans and an actionable modular remedy.
- Compatibility-only success tests are removed or converted; retained inspection
  and consumer assertions continue to exercise modular input.
- Supported-language docs and changed source-exact projections match rejection.
- `bash scripts/check.sh` passes.

## Decisions (running log)

- Decomposed modular-loader-k10 at the public input boundary: reject old input
  first, then delete its unreachable internal representations in modular-cleanup-k12.
  The parser, resolver, public records, two renderers and two books exceed a
  focused session when changed together.
- Use an ordinary structural rejection before flat validation, rather than keep
  recognizing old templates for special migration diagnostics. The next child
  removes the unreachable compatibility parser and resolver paths.
- The rejection matrix covers both mixed-document orders and both sources through
  Catalog and Templates, with exact declaration spans. Existing modular tests
  retain parameter safety, source authority, snapshot lifetime and inspection.
- Migrated JSON inspection fixtures preserve supported assignment variants,
  complete reference tables, filtering and no-write assertions. Removed only
  dedicated flat/reset success cases; their behavior is no longer supported.
- The single narrow adversarial reviewer found no actionable issue in the input
  boundary or test migrations. The larger representation removal and final forms
  audit remain the next child's work, so no review leaf is needed for this gate.
- Updated the reference, usage introduction, modular spec, ADRs and glossary for
  rejection; the source-exact book reconstructs the new boundary and explicitly
  labels its later compatibility paths unreachable pending cleanup. Final book
  narrative/example reconciliation remains with modular-cleanup-k12.

## Validation

- The new public-loader rejection test failed on accepted flat input before the
  boundary was added, then passed. The final matrix includes both mixed orders,
  both source roles, grammar-word keys, exact spans and unchanged source bytes.
- Focused keyed-launch and existing Grove configuration inspection/SessionConfig
  suites passed. The narrow reviewer found no actionable issue.
- `bash scripts/check.sh` passed all eight principal checks, including the locked
  workspace suite and all six final source-exact walkthrough validations.
  No warnings or failures were reported by the final run.
- SHA-256 digests for all tracked non-`.grove/` files matched before and after
  the final check, covering sources/tests, fixtures, manifests, scripts, skills
  and documentation. An earlier check was stopped to remove an unused test
  import; only the restarted run is completion evidence.
- This retires modular-input-k11 only. modular-cleanup-k12 remains live, so
  modular-loader-k10 and its parent chain do not close in this session.
