# integrated-acceptance-checks-k126

## Goal

Run the applicable Ravel checks, Grove core conformance and Grove's full
`bash scripts/check.sh` against the integrated subjects.

## Context

- The check decisions of `stage-evidence-reconciliation-k125` in the record.
- `docs/RELEASING.md`, the Ravel Taskfile and the Grove Taskfile.

## Done when

- Each check passes, or its failure becomes repair work.
- The record names each subject by commit and build identity, and the command
  and the result of each check.

## Notes

Do not publish a release, install `ravel@2` or change the owner's settings.

The "Check decisions" table of the record names each check, its subjects and
the reason to run it. Its "An observation for k126" section describes a
timeout test that failed on the 1.1 keg under load in k125.
