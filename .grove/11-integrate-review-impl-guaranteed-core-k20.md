# guaranteed-core-k20

**Integrates:** guaranteed-core-k19

## Goal

Triage `guaranteed-core-k19`'s findings and apply the ones that hold. This leaf
owns every fix and all post-fix verification — `cargo fmt`, `cargo clippy
--all-targets`, `cargo test` — none of which the review was allowed to run.

## Why it sits here

Adjacent to its review, ahead of `mandate-machinery-k10`. A review's findings are
anchored to a commit and to `path:line` coordinates, and any intervening edit to a
cited file moves them **silently** — nothing errors, the finding simply points
somewhere slightly wrong. `mandate-machinery-k10` edits exactly the files this
review reads.

## Notes

A finding that turns out to be a design disagreement rather than a defect belongs
in the record it disagrees with — `docs/adr/skill-delivers-the-methodology.md` has
a `## Considered options` section written to be argued against, and the spec's
requirements are the other home. Neither is appended to; both are current-state
sets reworked in place.
