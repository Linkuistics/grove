# classify-grove-rows-name

## Goal

Make `bash scripts/check.sh` pass again. Its test
`grove-llm::removed_surface every_grove_name_in_the_tree_is_classified`
fails on the name `GROVE_ROWS` in `scripts/owner_switch_rehearsal.py`.

## Context

- owner-switch-dry-run-k271 added the constant `GROVE_ROWS` to the dry-run
  comparison of `scripts/owner_switch_rehearsal.py`.
- The test in `crates/grove-llm/tests/removed_surface.rs` finds each
  `GROVE_*` name in the tree. Each name must have a role in `ROLES`, or it
  counts as a reintroduced launch surface.
- `GROVE_ROWS` is a local Python constant, not an environment variable or
  a launch surface. staging-kind-delegate-trials-k276 found the failure on
  its parent commit. That leaf touched neither file.

## Done when

- The test passes, and `bash scripts/check.sh` passes.
- The fix is either a rename of the constant or a role in `ROLES`. A role
  must state why the name is no launch surface.

## Notes

A rename to a name without the `GROVE_` prefix is the smaller change. It
keeps the classifier's list for real environment names.

## Decisions (running log)

- Rename `GROVE_ROWS` to `KEG_ROWS_OF_GROVE`. The name follows the
  module's `FORMULA_OF_GROVE` convention. The classifier matches a name
  only at an identifier boundary, so a `GROVE` after `_` is no match.
  `ROLES` stays limited to real environment names.
