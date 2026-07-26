# required-model-vars-k18

**Kind:** work

## Goal

Invert *model-per-task-kind*'s unset-is-fine rule: a picked leaf whose kind
resolves **no** model var must make `grove do` fail loudly, rather than launch
the session with no `--model` and let it inherit the user's own default.

## Context

- `docs/specs/task-kind-taxonomy.md` — the written contract, in particular
  *Model selection is required* and the exemption list. Implement from the spec,
  not from this file.
- ADR *model-per-task-kind* — already reworked to carry the inverted rule and
  its consequences. Do not re-argue it here.
- The lattice this stands on lands in `family-fallback-k14`, which must be done
  first: "required" is defined against the four-key resolution order, and the
  ~9-var figure only holds once the two family vars exist.
- `src/loop_driver.rs` — `resolve_launch` (the opt-out short-circuit that has to
  go), `model_for`, `KindPeek`.

## Done when

- A leaf whose kind resolves no model var fails with an error naming the kind
  **and** the vars that would satisfy it (exact-kind and family, harness-scoped
  and unscoped), rather than launching.
- The kind peek runs unconditionally on the `continue` path — `resolve_launch`'s
  `!any_harness_override_env() && !any_model_env(stamped)` short-circuit is gone.
- `KindPeek::Empty` still launches (no leaf ⇒ no requirement); `KindPeek::Degraded`
  now bails **always**, not only under a configured harness override.
- A harness whose `model_args` is empty is exempt.
- Harness absence is still fine — no `GROVE_<KIND>_HARNESS` means the stamped
  harness, and the stamp is not a default.

## Notes

**Most of this leaf is the test blast radius, so size it accordingly.**
`tests/loop_driver.rs` has 38 tests; roughly 13 set no model env at all and will
now error before reaching their assertions — each needs a model env added as
scaffolding, not a change of intent. Two more are direct tests of the rule being
inverted and must be **rewritten, not deleted**:
`loop_omits_model_flag_when_env_unset` and
`loop_omits_model_flag_when_env_is_empty_string` should become tests that the
absence (and the empty string) now *fails loudly*, preserving the empty-string
`env_nonempty` semantics they were guarding.

No new test seams — this lands on `tests/loop_driver.rs`, the existing
whole-`grove do` seam (fake harness binary on `GROVE_HARNESS_BIN_<NAME>`, assert
on recorded argv and on stderr + exit code).
