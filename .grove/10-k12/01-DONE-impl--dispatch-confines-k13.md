# dispatch-confines-k13

## Goal

`harness-dispatch run --confine` runs the harness it supervises under mandatory
filesystem confinement, with no unconfined fallback: Seatbelt on macOS,
bubblewrap on Linux. Selection runs first and outside it.

- **What the harness may write.** The cwd, which must not be `/`; dispatch's
  private run directory, which holds the harness's TMPDIR; and the exit
  channel's directory.
- **What it may read.** Installed system runtime resources, the selected
  executable, dispatch's own executable, and each `--runtime-read FILE`. A
  runtime read must be a regular file and is granted read-only. It is accepted
  only with `--confine`.
- **The refusal.** Before selection, dispatch compares canonical paths. A
  granted path that is a protected path, contains one or lies inside one
  refuses with exit 2, naming both. The granted paths are the cwd, the exit
  directory, the private run directory and each runtime read. The protected
  paths are the selected policy entry's directory, the owner settings file and
  the state directory. The two executables are exempt.
- **The environment.** HOME, USER, LOGNAME, PATH, LANG and the LC_* values;
  TMPDIR, TMP and TEMP set to the private temporary directory; and the three
  reserved values. Nothing else of dispatch's reaches the harness.
- **Noninteractive running.** No terminal, a POSIX session of the harness's
  own, and null stdin. Output goes to dispatch's own stdout and stderr.
  `argv[0]` is the resolved path. Cancellation kills the group at once.
- **The run record** notes the confinement and its runtime grants. A run
  recorded before supervision has no such field and reads as unconfined.

A missing backend, an unusable grant, a cwd of `/` or a grant that reaches a
protected path refuses before selection. Failing to establish confinement
launches nothing.

## Context

- The spec's *Confinement*, the `run` inputs table under *Command interface*,
  and *Records and later observations* (the confinement field).
  `harness-wrapper-k5`'s I2 and `harness-wrapper-k2`'s W13.
- The backends already exist in `crates/keyed-launch/src/confinement.rs`.
  Today `run_confined` writes to a log file the caller owns. The confined
  launch decision 7 states writes to the launcher's own output instead.

## Done when

- Seam 1: `--confine` refuses before selection with exit 2 in each of these
  cases:
  - the cwd holds the policy entry;
  - the exit directory holds the state directory;
  - a runtime read names the settings file;
  - the cwd reaches the policy's directory through a symlinked alias.

  A cwd beside those paths is admitted.
- A real-sandbox case through dispatch's command:
  - the harness receives a recorded run ID;
  - it cannot read the policy, the owner settings or the record store;
  - it cannot write outside its cwd and dispatch's run directory;
  - the file confined and run is the one dispatch resolved.
- Seam 2: a confined run's record notes its confinement and grants, and a run
  recorded before supervision is shown, observed and looked up as unconfined.
- `grove run` is unchanged and its suite passes.
- The `keyed-launch` book follows (P2), and `## Unreleased` records
  `--confine` and `--runtime-read`.
- `bash scripts/check.sh` passes.

## Notes

- Keep the runner's file-output confined launch beside the new form.
  `standalone-through-dispatch` removes it with its last caller, which is
  Grove's `grove run`.
- The protected paths come from what this invocation resolved: the policy entry
  it admitted, the settings file it read or would read from HOME, and the state
  directory it would record in. A runtime read is a literal grant, so a
  symlink to the settings file must be caught by its canonical path.

## Implementation plan

- Add command-seam tests for pre-selection refusals, canonical aliases, real
  sandbox grants, the scrubbed environment, and record compatibility; watch
  the new cases fail before implementing.
- Separate selection preparation from evaluation so confinement validates the
  exact admitted policy, settings location and state directory once.
- Extend keyed-launch's existing backend construction to multiple writable
  directories and add an observed confined launch with inherited output. Keep
  the file-output form for Grove's existing caller.
- Connect dispatch's flags, private scratch directory, minimal environment and
  immutable confinement record; run the focused suites.
- Update the keyed-launch book and Unreleased notes, run `task check`, retire
  this leaf and seal its jj change, then signal with installed v22 grove-llm.

## Decisions (running log)

- K1: Follow the already approved area specification and confined-run plan;
  no design or authorization is reopened. Grove's standalone caller stays
  unchanged in this increment.
- K2: Prepare selection once, then validate confinement before worker startup.
  This avoids resolving a different policy or settings directory for the
  refusal and the selection. Missing protected paths are resolved through
  existing ancestors, so first-run settings/store locations remain protected.
- K3: Reuse the runner's native sandbox backends for both output forms. The
  new form accepts multiple writable directories, inherits stdout/stderr,
  clears the environment before explicit grants, and retains observed spawn
  for dispatch's signal-mask handoff.
- K4: The fresh reviewer found three actionable boundary cases: implicit
  system runtime grants exposing owner paths, a runtime file beneath a writable
  root remaining writable on Seatbelt, and non-UTF-8 runtime paths panicking
  during record serialization. The command seam reproduced the first two;
  APFS does not admit the third filename, so that regression also runs on
  filesystems that admit native byte names. Refuse overlap with the backend's
  shared system-read inventory, enforce literal read-only grants and protect
  their ancestors from renaming, and refuse unrecordable paths before selection.
  These fixes have executable seams; no second reviewer is needed.
- K5: Preserve older launch JSON exactly as stored. A missing confinement field
  means unconfined in the text export and passes the same readable check used
  by show, observe and policy lookup; only new runs write the field.

## Validation

- `task check` passed all 12 principal checks, including the complete workspace
  tests and all six final book validations. The keyed-launch book resolves all
  2,365 source lines with none deferred. All 1,923 tracked input files and their
  hashes remained unchanged throughout the run.
- All eight dispatch confinement tests passed on macOS, exercising Seatbelt
  through the real command. Linux's bubblewrap backend was not exercised on this
  host; APFS cannot express the non-UTF-8 filename regression's subject.
- The graph CLI refused access because another pre-coordination or unverified
  generation was active. Source inspection supplied the evidence instead;
  graph coverage could not be established.
- The confined-run node remains open for `standalone-through-dispatch-k14`.
  This leaf has no pending review leaf and closes no ancestor.
