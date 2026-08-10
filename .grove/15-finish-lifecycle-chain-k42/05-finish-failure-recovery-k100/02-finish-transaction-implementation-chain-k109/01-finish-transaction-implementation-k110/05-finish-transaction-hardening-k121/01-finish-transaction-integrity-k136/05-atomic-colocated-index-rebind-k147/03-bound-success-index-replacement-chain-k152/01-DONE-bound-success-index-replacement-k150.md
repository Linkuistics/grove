# bound-success-index-replacement-k150

**Kind:** impl

## Goal

Bind the intended filtered-index replacement before recovery is allowed to
adopt its inode as the success artifact.

## Context

- A bare `rebinding=true` flag authorizes any regular file at the artifact name,
  so post-intent substitution can be activated or deleted as if Grove made it.
- The transition needs enough pre-published identity or provenance to validate
  both the pre-replacement and post-replacement states.

## Done when

- Recovery adopts only the exact Grove-produced replacement and rejects a
  different regular file or symlink.
- Both old and replacement artifact bytes remain protected at interruption and
  substitution boundaries.
- Unit tests prove genuine transition recovery and fail-closed substitution.

## Notes

Build on `recoverable-marker-replacement-k149`; do not infer ownership from a
deterministic filename or from transition phase alone.
