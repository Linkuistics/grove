# local-config-kdl-k7

**Integrates:** local-config-kdl-k6

## Goal

Fix both configuration-delta review findings, add focused regression coverage,
reconcile any comments or documentation the fixes make stale, and verify the
result through the project checks.

## Context

The implementation reviewed in `local-config-kdl-k6` otherwise satisfies the
settled two-source, whole-template design. Preserve the personal file's mandatory
completeness, first-candidate search order, per-kind whole-template replacement,
candidate-owning VCS lookup, and fail-closed behavior.

Findings, carried verbatim from `local-config-kdl-k6`:

1. **High — the Git trackedness check trusts an inherited alternate index.**
   `src/repo.rs:493` builds the security-sensitive `git ls-files` probe and
   `src/repo.rs:501` applies only `anchor_git_worktree_environment`; that helper
   removes `GIT_DIR` and `GIT_COMMON_DIR` and replaces `GIT_WORK_TREE`, but it
   does not remove `GIT_INDEX_FILE` (`src/repo.rs:350`). The repository's own
   internal-child contract already classifies `GIT_INDEX_FILE` as a repository
   selector and removes it in `scrub_internal_child_env`
   (`src/launch.rs:105-112`), but this new subprocess never calls that helper.
   Concrete failure: let the real checkout index track `.grove.kdl`, launch
   Grove with `GIT_INDEX_FILE` naming a valid empty alternate index, and
   `git ls-files -- .grove.kdl` exits successfully with empty stdout. Grove then
   accepts and executes the repository-tracked delta, defeating the seam's
   security guarantee. The same omission also passes the ambient Grove control
   environment to both new VCS subprocesses, contrary to the shared
   internal-child rule.
2. **Medium — delta discovery turns metadata failures into absence.**
   `src/session_config.rs:201-204` implements the first-candidate search with
   `symlink_metadata(candidate).is_ok()`. Every error, not only `NotFound`, is
   therefore treated as “no delta”: a permission or I/O error at the worktree
   candidate silently selects a repository-root delta, and the same error at
   the repository candidate silently falls back to the personal file. That
   contradicts both search precedence and the explicit rule that an unreadable
   delta fails closed (`docs/CONFIGURATION.md:150-157`). Discovery needs to
   distinguish `NotFound` from every other metadata error and return the latter
   with the candidate path.

## Done when

- Both concrete failures above are covered by tests that fail against
  `local-config-kdl-k3` and pass after the fixes.
- Every VCS subprocess introduced for trackedness follows the shared internal
  child environment contract, then applies any lane-specific anchoring.
- Delta discovery treats only `NotFound` as absence and reports every other
  metadata error against the candidate whose state could not be established.
- The full project verification named by the implementation workflow is green.

## Notes

Findings own the fixes; the review deliberately changed no production or test
code and ran no test, build, lint, or format command.
