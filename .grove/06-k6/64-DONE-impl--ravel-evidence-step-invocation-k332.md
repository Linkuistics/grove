# ravel-evidence-step-invocation-k332

## Goal

Make Ravel's release task start its evidence step through `bash`, so that the
next cut does not stop at that step.

## Context

- ravel-two-publication-k313, its running log: the 2.0.0 cut stopped at the
  evidence step with "permission denied" and exit 127.
- Ravel `Taskfile.yml`, internal task `release`: the line
  `scripts/release-evidence.sh --channel {{.CHANNEL}}` starts the script
  directly.
- Git records `scripts/release-evidence.sh` with mode `100644`. Ravel's other
  scripts with that mode are started through `bash`. No other Taskfile line
  starts a script with that mode directly.
- `scripts/release-channel.test.sh` starts the script through `bash` in its
  fixture, so the lifecycle suite did not find the defect. Its text check
  near line 188 reads the Taskfile for
  `scripts/release-evidence.sh --channel versioned-2`.

## Done when

- The Taskfile line starts the script through `bash`, as the other lines for
  scripts with mode `100644` do. Changing the file mode is the other choice.
  Record the choice and its reason.
- A control makes the lifecycle suite fail when the evidence step cannot run
  as the Taskfile writes it. Watch the control fail on the current line
  before the repair.
- Every Ravel check passes, including `task release:lifecycle:test` without
  `HOMEBREW_NO_INSTALL_CLEANUP` in the environment.
- Ravel `main` holds the repair as its own described change and is pushed.
  The tag `v2.0.0` does not move.

## Notes

Promotion is not affected. The Taskfile starts `release-promote.sh` through
`bash`, and that script only reads the evidence file. Promotion reads the
public tag, the GitHub release and the tap, not Ravel `main`.

Work in a Ravel workspace other than the default one, or leave the default
workspace on an empty change above `main`. A Grove session inherits
`RAVEL_EXIT_FILE` and `GROVE_LAUNCH_DIR`. Remove both from the environment of
each Ravel task.

Do not cut a release. The repair reaches users with the next Ravel cut, after
promotion, through the unversioned channel.

## Decisions (running log)

- The Taskfile line starts the evidence script through `bash`. The file mode
  stays `100644`. Reason: the Taskfile already starts `release-promote.sh`,
  which also has mode `100644`, through `bash`. Chapter 11's recovery
  fragment "finish a release from the installation step" also starts the
  script directly. It receives the same `bash` prefix, because a resume that
  copies it would stop the same way.
- The work uses Ravel's default workspace, which already held an empty change
  above `main`. Its `target/` is warm for `task check`. The session leaves it
  on a new empty change above the repaired `main`.
- The control lists every Taskfile command whose first word is a `scripts/`
  path and requires each file to be executable. On the unrepaired line it
  failed with "the Taskfile starts scripts/release-evidence.sh by its path,
  but the file is not executable". The text check now expects
  `bash scripts/release-evidence.sh --channel versioned-2`. Mutant "task
  evidence by path" restores the old line. Mutant "task non-executable by
  path" removes `bash` from `release-check.sh lifecycle`, which only the new
  control reads.
- `task release:lifecycle:test` also failed in `release-doctor.test.sh`, for
  a reason older than this leaf. Release commit `ed6cedbf9a14` replaced the
  chapter 3 line `ravel @RAVEL_BOOK_VERSION@` with `ravel 2.0.0`. The
  fixture removed only the placeholder form, so it removed nothing. The
  suite fails at every commit since the cut. Done-when requires the suite,
  so a second described change on Ravel `main` repairs the fixture. The
  fixture now removes either form, with the pattern of
  `release-book-version.sh`. It fails if it removes nothing.
- `release-book-version.test.sh` failed next, for the same cause. It
  required the copied page to start with the placeholder. Its fixture now
  resets the version line to the placeholder before the scenario, and its
  missing-line control removes either form. The same second change holds
  both fixture repairs.
