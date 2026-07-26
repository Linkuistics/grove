# family-fallback-k14

**Kind:** work

## Goal

Make routing resolve a **family** before falling back further, so one variable
states a policy that covers all five kinds of a family.
`GROVE_REVIEW_IMPL_HARNESS` beats `GROVE_REVIEW_HARNESS`; likewise for the model
vars.

## Context

- `src/loop_driver.rs` — `harness_override` / `checked_harness_override` (the
  harness axis) and `model_for` (the model axis, which **already** implements
  "specific beats general" on the *harness* dimension:
  `GROVE_<HARNESS>_<KIND>_MODEL` beats `GROVE_<KIND>_MODEL`). This leaf adds the
  same precedence on the *kind* dimension. Read `model_for` first — the shape to
  copy is already there.
- `validate_all_harness_overrides` sweeps every suffix so that a typo in a var
  for a kind that is *not* today's leaf still fails loudly. Family vars must join
  that sweep, or a misspelled family var silently does nothing indefinitely.
- `preflight_check` resolves every harness a launch might need. Family vars name
  harnesses too, so they must be pre-flighted the same way.
- The spec from `taxonomy-spec-k12` states the precedence; this leaf implements
  it.

## Done when

- Exactly two families exist — `review-*` and `integrate-review-*`. The other
  seven kinds have no family and resolve as they do today.
- For both the harness var and the model var, precedence is stated once and
  tested at each step: full kind beats family beats the existing base.
- **`integrate-review-impl` resolves to the `integrate-review` family, never to
  `review`.** The two family names overlap as strings; longest-match wins, and
  there is a test that fails under naive prefix matching.
- Family vars are covered by `validate_all_harness_overrides` and
  `preflight_check`.
- A rerouted launch still refuses to inherit an unscoped value — the family
  fallback must not open a hole in that rule.

## Notes

The **rerouted** invariant is what is most likely to break here. Today a launch
whose harness differs from the stamped one deliberately gets *no* base var,
because that var was written with some other harness in mind (a codex profile
name is meaningless to pi). A family fallback runs along a different axis and
must compose with that rule rather than route around it: falling back
`review-impl` → `review` is fine; falling back across harnesses on a reroute is
not.

Do not generalise families beyond the two. A configurable family map, or
families for producers, is speculative structure with no case behind it
(constraint 4) — and every extra family multiplies the precedence table this
leaf has to keep testable.
