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
