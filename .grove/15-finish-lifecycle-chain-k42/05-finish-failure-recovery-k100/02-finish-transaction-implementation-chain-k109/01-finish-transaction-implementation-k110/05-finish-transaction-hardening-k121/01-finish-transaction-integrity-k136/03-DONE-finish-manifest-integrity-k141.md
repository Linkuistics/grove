# finish-manifest-integrity-k141

**Kind:** impl

## Goal

Reject malformed, mismatched, or tampered ready/manifest state and evacuated
entry sets before repository classification.

## Context

- Consume the validated task-root and witness handles from the preceding two
  leaves.
- Repository topology classification remains behind `repo::finish_commit` and
  is not duplicated here.

## Done when

- Red tests cover marker/manifest symlinks, versions and identity mismatch,
  duplicate or traversal-shaped entries, content/mode/link-target tampering,
  missing entries, and foreign evacuated entries.
- Recovery validates the exact canonical entry set through directory handles
  before asking the repository seam for a disposition.

## Notes

Preserve the existing length-delimited SHA-256 format unless a failing case
demonstrates an ambiguity.
