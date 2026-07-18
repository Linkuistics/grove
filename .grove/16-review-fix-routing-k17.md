# review-fix-routing-k17

**Kind:** work

## Goal
Close the four routing defects that let the trial silently run the wrong
harness or the wrong model — the failure mode the whole grove exists to
prevent ("K3 reviews everywhere").

## Context
Findings B2, B5, B6, B7 in `.grove/14-DONE-branch-review-k14.md`.

- **B2** `src/loop_driver.rs:225` falls back to the harness-agnostic
  `GROVE_<KIND>_MODEL` and applies it to the *post-override* harness, so
  `GROVE_REVIEW_MODEL=sol-high` + `GROVE_REVIEW_HARNESS=pi` launches
  `pi --model sol-high`. The doc comment at `:220-221` asserts the opposite.
  Decide the semantics: does the base var survive a reroute at all? There is
  also no way to express "on this harness, for this kind, pass no flag" —
  `""` falls through rather than suppressing.
- **B5** `src/loop_driver.rs:199-201` — unscoped `GROVE_HARNESS_BIN` applies
  to every harness, so a reroute execs the wrong binary with the right
  harness's flags. Also the only env seam in the file treating `""` as set.
- **B6** `src/loop_driver.rs:284-287` — a degraded kind peek returns the
  stamped harness, silently cancelling the reroute; the diagnostics mention
  only the model, never the harness.
- **B7** `src/loop_driver.rs:109` loads the prompt from the stamped harness
  before `:114` resolves the launching one. Contract says launching.

## Done when
- B2/B5/B6/B7 are fixed or explicitly accepted with the reason recorded here.
- Tests exist that would go red under each defect — in particular a reroute
  test that does **not** set a single `GROVE_SKILL_DIR` (T4: that fixture
  makes B7 structurally invisible), and a review-path unknown-harness test
  (T3: the current one takes the start-path shortcut).
- `cargo test`, `fmt`, `clippy` clean.
- One focused commit.

## Notes
- Consider a one-line launch diagnostic naming the resolved
  `(harness, model)`. The review flagged that the trial's central invariant is
  currently unobservable at runtime, and that a typo in a var *name*
  (`GROVE_REVIEWS_HARNESS`) would produce zero output for a whole month. Cheap
  insurance for a month-long A/B whose entire output is a judgement call.
- Also cheap and in-scope: `harness_override` validates only the picked kind's
  var, so a typo'd value fails hours later. `any_harness_override_env` already
  sweeps all five suffixes — validating all five at startup is nearly free.
- Deliberately out of scope unless they fall out naturally: `KIND_SUFFIXES`
  drift, env-name sanitisation, routing env inherited by nested sessions. All
  three are recorded under "decision" in k14.
