# grove-rhai-review-assets-k96

## Goal

Port the review adapter, the two review examples and the provider-evidence
wrapper to Rhai. Keep receipt bytes and provider-safety outcomes identical
across runtimes.

## Context

- The "Runtime parity" section of the delivery specification, including the
  one accepted receipt difference.
- `assets/ravel-policy/grove.ts`, `provenance.ts`, `examples/review.ts` and
  `examples/grove-review.ts`, and their cases in the asset checks.
- `docs/grove-provider-evidence.md` owns the receipt and context contract.
- The case table and the Rhai static example of grove-rhai-static-assets-k95.

## Done when

- `grove.rhai`, `provenance.rhai`, `examples/review.rhai` and
  `examples/grove-review.rhai` sit beside their counterparts. Each names its
  runtime and schema. The TypeScript files keep their installed paths.
- The Rhai wrapper writes the receipt bytes of the TypeScript wrapper. It
  keeps the constant member order and joins members that `json_encode` wrote.
- A receipt that the wrapper of one runtime records passes the review loader
  of the other runtime. The case table checks both directions.
- The Rhai helpers test the ECMAScript whitespace set explicitly for blank
  labels. Label, receipt and command checks give the same outcome in both
  runtimes.
- A receipt that escapes an unpaired surrogate passes the TypeScript loader
  and refuses as `creator_line_malformed` in Rhai. Its case keeps the reason.
- `ravel:review` and `ravel:provenance` pass against each build. That includes
  actual Grove retirement and retry sessions with fake agents.
- `docs/grove-provider-evidence.md` holds the runtime-neutral definition of a
  blank label. The assets README describes the Rhai review composition.

## Notes

The typecheck files stay TypeScript only. Controls: a Rhai receipt with a
changed member order must fail the byte comparison. A Rhai loader that trims
with Rhai's own `trim` must fail a U+FEFF or U+0085 case. Watch each fail once.

This port carries provider-diversity enforcement. Earn a `review-impl` unless
the cross-runtime receipt and safety cases cover every changed branch.

grove-ravel-runtime-fixtures-k94 left the six cases of
`crates/grove/tests/creator_lifecycle.rs` behind the guard
`review_assets_unported`. Under the 2.0 runtime they report that and run
nothing. Give them Rhai review policies and remove the guard, so that
`ravel:provenance` runs them against each build.

grove-rhai-static-assets-k95 leaves three handoffs. A Rhai script that composes
the routes of `grove-static.rhai` selects through its `select_route`. Rhai
finds the function of a closure only while a function of its module runs. The
review and provenance tests of `ravel_policy_assets.rs` carry the guard
`review_assets_unported`, which this leaf removes. Under 2.0 the case table runs the
Rhai static example and sample without a wrapper. Wrap them in the Rhai
provenance wrapper, as the table wraps the TypeScript set.
