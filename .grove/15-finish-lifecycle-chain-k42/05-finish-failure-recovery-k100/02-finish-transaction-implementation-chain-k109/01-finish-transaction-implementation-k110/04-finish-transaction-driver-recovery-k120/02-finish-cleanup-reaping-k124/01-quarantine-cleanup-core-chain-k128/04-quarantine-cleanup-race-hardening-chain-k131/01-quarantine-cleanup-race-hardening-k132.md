# quarantine-cleanup-race-hardening-k132

**Kind:** impl

## Goal

Close the concurrent-substitution and marker-authority gaps surfaced by the
narrow review of `quarantine-cleanup-core-integrate-k130` without broadening
cleanup into lifecycle classification.

## Context

- `src/finish_cleanup/unix.rs::remove_directory_contents` stats a
  non-directory entry, then unlinks the current name without revalidating its
  identity. A regular file or symlink substituted between those operations can
  be deleted.
- `validate_marker` accepts quarantine and claimed byte strings containing `/`.
  Descriptor-relative opens then traverse intermediate components; `O_NOFOLLOW`
  protects only the final component.
- The quarantine/claimed existence probes are independent. Creating or moving a
  name between them can bypass the ambiguous-owner refusal or return
  `NothingToDispose` while an artifact still exists.
- `prepare_quarantine` checks `NAME_MAX` against an opened control directory but
  publishes its temporary marker through the path. Replacing an ancestor can
  move publication onto an unchecked filesystem.
- Preserve the cleanup outcome and actionable diagnostic seam established by
  `quarantine-cleanup-core-integrate-k130`.

## Done when

- Every non-directory unlink revalidates the current no-follow entry identity;
  deterministic substitution tests prove no replacement is deleted.
- Marker validation requires quarantine and claimed names to be single,
  non-empty path components before any descriptor-relative operation.
- Ambiguous-owner and nothing-to-dispose decisions are stable against
  deterministic races, or fail closed without deleting either candidate or the
  marker.
- Component-limit validation and marker publication are bound to the same
  no-follow control-directory object; a path replacement cannot redirect the
  marker.
- Unit and process tests cover each exact race and keep unrelated cleanup,
  lifecycle, and repository behavior green.

## Notes

This is the substantial redesign path required after the integration leaf spent
its one in-session reviewer. Do not absorb driver/lease reaping or Git-index
auxiliary cleanup; those remain `cleanup-driver-acceptance-k127` and
`auxiliary-cleanup-markers-k126`.
