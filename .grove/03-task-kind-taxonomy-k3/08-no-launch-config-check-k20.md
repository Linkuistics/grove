# no-launch-config-check-k20

**Kind:** impl

## Goal

Make `grove do --no-launch` actually report launch-readiness, now that a
half-configured environment is a hard launch failure rather than a benign
fall-through.

## Context

Surfaced by `config-sweep-k16` while verifying the migrated environment, and
externalized rather than absorbed: it changes launch-path behaviour.

`--no-launch` is documented as "report readiness but don't exec the harness", and
`docs/grove.md` sells it as "useful for inspection or scripting". But
`launch.rs::do_grove` returns at the `args.no_launch` guard **before** both
config checks:

- `loop_driver::preflight_check(harness)` — which resolves every configured
  `GROVE_<KIND>_HARNESS` / `GROVE_<FAMILY>_HARNESS`, so a typo or an
  uninstalled rerouted harness goes unreported.
- the per-leaf model resolution inside `loop_driver::run`, which under
  `required-model-vars-k18` is now a hard error.

Measured on the live environment: with `GROVE_CLAUDE_IMPL_MODEL` unset and an
`impl` leaf picked, `--no-launch` prints `grove: ready in <path> (no-launch)` and
exits **0**, while the very next real `grove do` exits non-zero with "no model is
configured for it on claude". "Ready" is the one thing it is not.

That is the same partial-configuration invisibility *model-per-task-kind* exists
to eliminate, reintroduced through the dry-run door.

## Done when

- `--no-launch` runs the config checks and exits non-zero when the next real
  launch would fail, naming the same vars the real launch would name.
- It still writes **no stamp** — the reason the guard sits where it does (`B3`:
  a documented dry run must never permanently rebind the grove). Moving
  `preflight_check` above the guard is safe on that count (it is side-effect
  free); confirm the same for the kind peek, which spawns `grove-llm kind`.
- The no-live-leaf case still succeeds: a grove ready to finish has no kind to
  require a model for (that exemption is already in `model_for`'s callers, not
  in the peek).
- A test asserts the previously-passing case now fails: unset one required var,
  run `--no-launch`, expect non-zero and the var named.

## Notes

Decide explicitly whether `--no-launch` should *also* peek the leaf, or only run
`preflight_check`. The peek is one subprocess and no side effect, so the cost is
trivial — but it makes `--no-launch` depend on `grove-llm` being resolvable,
which is a new failure mode for a flag whose job is inspection. Recommendation:
do both, and report the leaf and its kind in the readiness line so the output
becomes genuinely informative rather than merely non-committal.

Do not widen this into a general `grove doctor` verb — `do` is the sole
lifecycle entry verb (*do-is-sole-lifecycle-verb*), and this is a flag on it
telling the truth, not a new surface.
