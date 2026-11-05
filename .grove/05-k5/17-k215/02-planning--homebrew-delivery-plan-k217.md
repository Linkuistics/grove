# homebrew-delivery-plan-k217

## Goal

Cut the implementation leaves of `homebrew-skill-delivery-k215` from the
settled design.

## Context

- The parent brief states the outcomes.
- `homebrew-delivery-design-k216`, and any review chain that it earned,
  settle the design.

## Done when

- Each outcome in the parent brief's "Done when" belongs to a leaf with a
  concrete, checkable result.
- The leaves cover each of these subjects:
  - Grove's release archive and formula.
  - Grove's record of pack commits, and `check:packs`.
  - The removal of the marketplace's pack entries and of the installer's
    skill links.
  - The formulae of personal, superpowers and quint-llm-kit, and
    TestAnyware's formula.
  - The marketplace files and the documented commands of the hook adapters.
  - The README changes in the six packs and in TestAnyware, and Grove's
    guidance.
- Each leaf leaves Grove's `bash scripts/check.sh` green. Each pack change
  passes the checks of its own pack.

## Notes

A change to a pack's README is a new pack commit on its public remote. Grove's
record then names the commit that Grove's pack check accepts.

`discovery-rollout-k6` owns the owner's cutover. Put a leaf there only when
the design gives a step to rollout.
