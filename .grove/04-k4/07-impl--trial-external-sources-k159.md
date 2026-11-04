# trial-external-sources-k159

## Goal

Let the harness trial runner take a source from an external checkout at a
named commit. Let it record that commit in each run.

## Context

- `docs/specs/external-pack-contract.md`, "Proof shape" and "Checkout
  boundary".
- `scripts/harness_trial.py`: `settings_for`, `load_case`, `prepare` and the
  `clones` mechanism. Today every source path is the prepared Grove checkout
  plus the source's `plugin` field.
- `scripts/test_harness_trial.py` and `task trial:controls`.
- `docs/common-loading-acceptance-record.md`, "The runner" and "Research
  recipes".

## Done when

- A case source record can carry the `external` object of the contract's
  "Proof shape", with `repository` and `commit`. Its `plugin` then names the
  source directory inside that checkout.
- The runner makes a plain Git checkout of that commit under the trial's
  `sources/`, with the isolated Git configuration of the recipes. It refuses
  before any launch for an unknown commit, a different `HEAD` or a checkout
  that is not clean.
- The run record names the repository, the commit, the checkout path, the Git
  tree ID of the source directory and the `resourceDigest` of the source.
- `moveAway` works for an external source, so a case can expect
  `skill_source_missing` before any worker.
- The run option `--working-copy <id>=<dir>` replaces one source's checkout
  with a working copy. The record names its path, its Jujutsu change and
  commit, and whether the working-copy change is empty.
- Unit tests and `task trial:controls` cover the new fields, including the
  refusal of an unknown commit. Existing cases keep their result.
- The record's "The runner" section describes the new fields.

## Notes

This change spends no subscription usage. The real runs are in
research-directory-trials-k160.
