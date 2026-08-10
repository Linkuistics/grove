# finish-witness-identity-k140

**Kind:** impl

## Goal

Make witness and evacuated-tree creation/recovery descriptor-relative and
reject substitutions or foreign root entries without moving external bytes.

## Context

- Consume the authoritative task-root handle delivered by
  `finish-task-root-identity-k139`.
- Manifest field/content validation belongs to `finish-manifest-integrity-k141`.

## Done when

- Red tests cover witness/original-tree symlinks or replacements, foreign
  entries beside a ready witness, and destination collisions while preserving
  every external byte.
- Witness, recovery tree, and transaction files are created/opened beneath
  no-follow directory handles and their identities remain carried into later
  transitions.

## Notes

Preserve raw Unix filenames and reuse the existing no-replace filesystem seam.
