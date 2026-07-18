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

## Result
All four findings fixed, plus both cheap Notes items. One decision made
explicit (B2's "does the base var survive a reroute" question):

- **B2** — `model_for` now takes a `rerouted` bool; on a reroute (`launch !=
  stamped`) only the harness-scoped var (`GROVE_<HARNESS>_<KIND>_MODEL`) can
  supply a model — the base `GROVE_<KIND>_MODEL` fallback is skipped entirely,
  never carrying a value meant for the stamped harness onto a different one.
  **Decision recorded**: the base var does *not* survive a reroute, full stop
  — with no scoped var set, a rerouted leaf launches with no `--model` at all
  (never a crash, never a foreign-harness value). The "no way to say pass no
  flag on this harness" compounding note is subsumed by this: since the base
  var never applies post-reroute, an unset or empty scoped var both already
  mean "no flag" there: nothing further to add. The non-rerouted (single-
  harness, the common case) path is untouched — scoped still beats base
  exactly as before.
- **B5** — `harness_bin` takes the same `rerouted` bool; the legacy unscoped
  `GROVE_HARNESS_BIN` now only applies when `!rerouted` (mirrors B2's
  decision). A reroute with no `GROVE_HARNESS_BIN_<NAME>` set falls straight
  to `exec_bin`. Also switched to the shared `env_nonempty` helper, so `""` is
  treated as unset here too (it was the one holdout env seam in the file).
- **B6** — `resolve_kind` now returns a three-way `KindPeek` (`Kind` /
  `Empty` / `Degraded`) instead of `Option<Kind>`, so `resolve_launch` no
  longer conflates "no live leaf" (silently fine — nothing to route) with "the
  peek itself failed" (genuinely unknown). On `Degraded` with an active
  per-kind harness override, `resolve_launch` now **bails loudly** rather than
  silently launching on the stamped harness — the same class of failure
  `unknown_review_harness_fails_loudly` already covers for a bad *value*, now
  also covered for a failed *peek*. Model-only configs (no harness override)
  still degrade gracefully, unchanged.
- **B7** — `resolve_launch` now runs *before* `load_prompt` in `run_loop`, so
  the prompt is read from the post-reroute launch harness, never the stamped
  one.
- **Notes item 1** — `launch_session` prints one diagnostic line per launch
  (`grove: launching <harness> (model: <model|default>)`), unconditionally —
  it has to fire on *every* launch, routed or not, or a var-name typo that
  produces zero routing effect would also produce zero diagnostic, defeating
  the point.
- **Notes item 2** — `validate_all_harness_overrides` sweeps all five
  `GROVE_<KIND>_HARNESS` vars (via a new `checked_harness_override` shared
  with `harness_override`) the moment any routing env is present, not just the
  picked leaf's kind — an off-kind typo now fails at the very next launch
  instead of hours later.

Tests: upgraded `review_leaf_reroutes_to_the_review_harness` (T4) to real
per-harness skill dirs under a scratch `$HOME` instead of a shared
`GROVE_SKILL_DIR`, asserting on which prompt copy was actually read — this is
what makes B7 observable at all (a shared dir makes both harnesses read the
same file by construction). Added five new tests:
`base_model_var_does_not_survive_a_reroute` (B2),
`unscoped_harness_bin_does_not_leak_across_a_reroute` +
`empty_string_harness_bin_is_treated_as_unset` (B5),
`degraded_kind_peek_refuses_to_silently_cancel_a_harness_override` (B6),
`an_off_kind_harness_override_typo_is_caught_immediately` (validate-all).
T3 (`unknown_review_harness_fails_loudly_on_the_continue_path`, added by
review-fix-tests-k16) needed no changes and still passes.

Mutation-verified all six live: reverted `src/loop_driver.rs` to the
pre-fix (`HEAD`) version with the new/upgraded tests in place and confirmed
all six failed for the stated reason (B2's foreign profile name reaching pi;
B5's wrapper leaking across the reroute and `""` being attempted as a literal
path; B6 launching on the stamped harness instead of erroring; B7 reading
codex's prompt for the pi launch; the off-kind typo passing silently) —
then restored the fix and confirmed green again.

`cargo test` clean both under the real dogfooded environment (this repo's own
`~/.zshenv` routing vars, `GROVE_REVIEW_HARNESS=pi` etc., loaded) and with
`GROVE_*` model/harness vars scrubbed — 282 unit tests plus every integration
binary, 17 in `loop_driver.rs` (up from 12), zero failures. `cargo fmt --check`
and `cargo clippy --all-targets --all-features -- -D warnings` both clean.
One focused commit.
