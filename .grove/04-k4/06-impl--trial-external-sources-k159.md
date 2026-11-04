# trial-external-sources-k159

## Goal

Let the harness trial runner take a source from an external checkout at a
named commit. Let it record that commit in each run.

## Context

- `docs/specs/external-pack-contract.md`, the proof-shape section.
- `scripts/harness_trial.py`: `settings_for`, `load_case`, `prepare` and the
  `clones` mechanism. Today every source path is the prepared Grove checkout
  plus the source's `plugin` field.
- `scripts/test_harness_trial.py` and `task trial:controls`.
- `docs/common-loading-acceptance-record.md`, "The runner" and "Research
  recipes".

## Done when

- A case can name an external source by repository, commit and source
  directory, with its roots. The runner makes a plain Git checkout of that
  commit outside every working copy, with the isolated Git configuration of
  the recipes.
- The run record names the external commit, the checkout path and the
  `resourceDigest` of the source.
- `moveAway` works for an external source, so a case can expect
  `skill_source_missing` before any worker.
- A case can name a working copy as a `local` source for the working-copy
  proof. The record names the working-copy commit and whether it was clean.
- Unit tests and `task trial:controls` cover the new fields, including a
  refusal of an unknown commit. Existing cases keep their result.
- The record's "The runner" section describes the new fields.

## Notes

This change spends no subscription usage. The real runs are in
research-directory-trials-k160.
