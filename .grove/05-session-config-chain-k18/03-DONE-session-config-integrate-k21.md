# session-config-integrate-k21

**Kind:** integrate-review-impl
**Integrates:** session-config-review-k20

## Goal

Apply the verified findings from `session-config-review-k20` while preserving the reviewed artifact's contract.

## Context

- Read `session-config-review-k20` and verify every finding against the binding
  spec before changing the reviewed module.
- Preserve the expand-only boundary: do not wire configuration into the driver
  or delete legacy routing in this integration step.

## Done when

- Every finding is classified as unclear contract, real issue, visible
  trade-off, or noise, with the disposition recorded in this task file.
- Verified issues are fixed at the configuration seam with regression tests;
  rejected findings have concrete evidence.
- `cargo fmt --check` and `cargo test --locked` pass.

## Dispositions

- **E1 — real issue, fixed.** `shell_words::split` treats a delimiter-state
  `#` as a comment and returned a plausible truncated argv. Configuration now
  rejects that form before splitting, while quoted, escaped, and mid-word
  hashes remain literal. The grammar clarification is recorded in
  `docs/specs/config-driven-sessions.md`; focused tests cover both rejection and
  the literal forms.
- **E2 — real issue, fixed.** Node validation now uses KDL's string accessor,
  which accepts both ordinary and raw strings. The existing non-string failure
  remains covered by the aggregate schema test, and a raw-string template is
  loaded and expanded through the public seam.
- **E3 — real issue, fixed.** Every template diagnostic is now stamped with its
  session kind at the common diagnostic helper; the aggregate test verifies the
  kind and source location together.
- **E4 — real issue, fixed.** Empty word zero is diagnosed only by the per-word
  validation when a parsed word exists. A focused regression test proves one
  defect produces one diagnostic.
- **E5 — real issue, fixed.** A whole substitution now has exactly one closing
  brace, so concatenated substitutions reach the accurate complete-word
  diagnostic instead of being mislabeled as one unknown name.
- **E6 — visible transition trade-off.** `REQUIRED_KINDS` must temporarily
  differ from the legacy `leaf::Kind` set during the expand half of the
  migration. A source comment makes the required convergence explicit;
  `session-kind-migration-k27` and `lifecycle-cutover-k39` already own the
  filename-kind cutover where one source of truth becomes possible.
- **E7 — visible trade-off, no change.** `Vec<OsString>` matches the binding
  interface, which leaves executable resolution and spawning to the caller.
  The code graph shows no production caller yet, so a public wrapper type would
  add an interface before it has a second use or an observed misuse; the single
  direct-exec caller arrives in `lifecycle-cutover-k39`.
- **E8 — real issue, already externalized.** The pre-existing MSRV mismatch is
  owned by `msrv-claim-k74`; this integration does not absorb it.

## Notes

Substantial driver or tree redesign is new work inside `session-config-chain-k18`,
not scope to absorb here.
