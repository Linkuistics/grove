# plan-k1


## Goal

Cut, publish and install the next Grove release from the work already on
`main`. The human gave this goal and the cause of the empty tree in two
messages:

<!-- tc: off because these are the human's own words, quoted -->

> We were wanting to do a release

> the grove was finishing, so the grove files have been deleted

<!-- tc: on -->

## Context

The previous grove in this workspace delivered the gh-issue-22 work. Its finish
session removed that tree in the commit `finish-k24: remove completed grove task
tree`. The session then moved `main` to that commit and pushed it. It did not
reach the release step. The newest tag is still `v22.5.0`, and 67 commits sit on
`main` above it. The driver later found a rootless working tree and created this
tree, which is not a resumed finish.

- `docs/RELEASING.md` gives the release procedure and the recovery from a
  partial release.
- `CLAUDE.md`, section "Finish, integrate, and release", names the release tasks
  and forbids a second cut after a partial one.

## Done when

- `task release:minor` passed its final verification. The tag `v22.6.0`, the
  GitHub Release with three archives, the tap formula and the installed `grove`
  and `grove-llm` agree on 22.6.0.
- Issue 22 is closed with a comment that names v22.6.0.
- The root brief tells the finish session of this grove to make no second
  release.

## Notes

## Decisions (running log)

**No map and no grilling.** The work is one release of commits already on
`main`, and it fits one sitting. Fewer than three open questions depend on each
other, so the grilling threshold is not met. Checks on 2026-10-24 found no
partial cut: no `v22.6.0` tag, no release lock and no version change. The
release changes neither the tree grammar nor the session ending, so it needs no
cutover. The release doctor reported all prerequisites met.

**The release runs in this session.** The human chose to cut v22.6.0 now, with
`task release:minor` from the default workspace. The finish session of this
grove then removes the tree and makes no second release.

**Issue 22 closes after the release.** The human chose to close
Linkuistics/grove#22 with a comment that names v22.6.0.

**The session wrote the release notes by hand.** The release-notes writer that
the owner policy selects cannot start inside the Ravel sandbox. That failure
also stopped the release of the previous finish. `docs/RELEASING.md` names a
hand-written Unreleased section as the remedy, so the notes came from the same
evidence and followed `scripts/release-notes/SKILL.md`. Issue 23 records the
writer failure.
