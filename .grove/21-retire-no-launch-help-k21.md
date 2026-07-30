# retire-no-launch-help-k21

**Kind:** impl

## Goal

Give `grove retire --no-launch` a description, and decide whether its dry run
should run the checks `grove do --no-launch` runs.

## Context

`RetireArgs::no_launch` (`src/cli.rs`) carries **no doc comment at all**, so
`grove retire --help` renders it as a padded blank row beside two described
options. Found by `retire-help-node-path-k20` executing *every* generated help
surface and scanning for undescribed options: this is the only one in the whole
CLI — both binaries, every subcommand.

Writing the description is not purely cosmetic, because the two dry runs are not
the same thing. `grove do --no-launch` is documented as *reporting readiness* and
deliberately resolves everything a launch would fail on — pre-flight plus the
kind/model peek, both moved above the early return by `no-launch-config-check-k20`
(a leaf of an earlier grove; see `src/launch.rs`), precisely because a dry run
that skipped them printed `ready` on exactly the half-configured environments the
checks exist to expose. `launch::retire`'s dry run does neither: it resolves the
harness, prints `would exec <bin> for retire (no-launch)` and returns before
loading the prompt. So either the description says honestly that retire's dry run
only reports the harness it would exec, or the same argument that moved `do`'s
checks applies here too and the behaviour changes. Answer that first; the help
text follows from it.

## Done when

- `grove retire --no-launch` renders a description that matches what it actually
  does, executed and read — not inferred from the source.
- The `do` vs `retire` asymmetry is either **fixed** (retire's dry run runs the
  same checks) or **stated** as deliberate, with the reason recorded where the
  next reader meets it.
- No generated help surface has an undescribed option — the scan
  `retire-help-node-path-k20` ran, re-run.
- `cargo test` passes.

## Notes
