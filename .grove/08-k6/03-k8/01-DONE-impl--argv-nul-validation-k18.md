# argv-nul-validation-k18

## Goal

Reject NUL in configuration templates and runtime slot values before Argv is
constructed, while preserving native strings and exact argument boundaries.

## Context

The parent owns the full reusable-command contract. This slice strengthens the
existing flat Catalog/Templates path and the validation seam named commands will
reuse. `param.` vocabulary reservation already exists.

## Done when

- Both sources eagerly reject NUL-bearing template words with invalid_template
  diagnostics, real source spans and a useful remedy, even when overridden.
- Expansion rejects NUL-bearing offered runtime values with invalid_value,
  template source and key context, and no invented source range. All declared
  runtime values are checked, including an unused optional slot.
- Public-seam tests prove Catalog/convenience agreement, aggregate diagnostics,
  empty and opaque argument preservation, and native non-Unicode values.
- Crate/reference docs and affected exact-source walkthroughs describe the
  boundary and pass the root brief's checks. Wrapper syntax stays unsupported.

## Decisions (running log)

Check runtime values while matching the complete vocabulary, so validity does
not depend on whether one route happens to mention an optional slot. Preserve
OsStr values unchanged; validation must not convert them through Unicode.

## Implementation plan

1. Add public-seam regression tests and observe NUL cases fail.
2. Validate template text and native runtime values at their existing boundaries.
3. Update the affected source fragments, explanatory prose and derived indices.
4. Run focused tests, then the full repository check with inputs fixed; retire
   and seal this child while leaving the remaining node work live.

## Verification

The three NUL regressions failed before the implementation because loading or
expansion returned success. All 27 template tests passed after it, including
non-Unicode native strings and empty/opaque arguments. `bash scripts/check.sh`
passed all eight principal checks, including workspace tests and final checks
of all six books. SHA-256 digests of all 1,795 tracked file inputs (production,
tests, manifests, scripts and documentation) were unchanged across that run.

One bounded adversarial reviewer found no implementation defect and one
actionable prose omission: the runtime refusal table still listed three cases.
The table and error precedence explanation now include NUL. After that prose
fix, final keyed-launch book validation passed again: 10 files, 2,753 resolved
lines, no deferred lines; input digests remained unchanged during the check.
No second reviewer was needed for this mechanical correction.

The parent remains live through `named-command-reuse-k19` and
`parameter-overlays-k20`; wrapper syntax and profile composition are not claimed
as delivered by this child. No ADR decision changed: this enforces the existing
modular specification's argument validity contract.
