# ravel-caller-commands-k38


## Goal
Provide Grove's shared public Ravel command construction, independently verified
against the installed two-verb pair before atomic activation.



## Context
Use k26's resolved canonical executable as an input, k32's SessionResult and
k36's InstalledRavel fixture. Ravel 0.1.0 admits its real front inside confinement;
the public contract is Ravel's docs/session-cli.md and
docs/specs/session-execution.md. Do not grant policy/worker assets to the harness.

## Done when
- Shared lifecycle/confined argv builders use only documented start-session
  flags, preserving the supplied prompt and optional task data as literal values.
- Ending instructions quote the supplied executable as one shell word and pass
  exactly grove-continue. No executable lookup or re-canonicalization occurs.
- Confined runtime grants resolve relative to the caller before staging cwd.
  Real installed Ravel tests prove prompt/task round trips, stream separation,
  quoted paths, independent prefix selection and confined read grants with an
  observed denial control. No models, host runtime or personal configuration.
- A reusable Taskfile check, saved complete logs and matching input digests cover
  the new boundary. Update the affected exact-source book and run relevant
  nextest/format/clippy checks. Both actual consumers remain on dispatch.

## Notes

## Implementation plan

1. Add behavioral tests for literal shell quoting and lifecycle/confined public
   inputs through copied installed Ravel. Observe the absent API fail.
2. Add session_command to grove-loop: lifecycle, confined and end_session helpers
   using the already selected executable and existing CONTINUE_RESULT.
3. Run the installed tests, including missing runtime grant and mutated command
   controls, then restore and rerun. Add task ravel:commands.
4. Reconcile grove-loop's fragment/root/index/rollups, freeze all inputs, run the
   focused task and relevant default suites/format/clippy/book checks; compare
   digests. Record the handoff, retire only k38, and seal through jj.

## Decisions (running log)

- Expose lifecycle/confined builders and end_session in grove-loop, where both
  consumers can share the frozen result convention. The supplied Ravel path is
  already resolved; only standalone runtime-file grants are canonicalized.
  Owner policy/settings remain outside the argv. Ravel 0.1.0 itself admits the
  real front to the confined harness, so no worker or policy grant is added.
- Observe public selection inputs at their native SDK fields taskFile/taskId.
  Lifecycle Selection supplies an absolute task path; Ravel resolves a relative
  CLI task-file against its cwd. The prompt is passed byte for byte. Both
  installed routes execute the exact quoted ending command from a separate
  prefix on a bin-only PATH.
- The confined test supplies its credential relative to the caller directory
  before staging; without a grant it observes runtime-denied. Removing --confine
  makes that same dirty control read both the credential and policy and fail.
  Restoring confinement is measured again in the final check. These executable
  controls discharge the small command-construction claim; no in-session
  reviewer is needed, and the later atomic activation remains live.
- Change the lifecycle fixture's lookup PATH after selecting the copied real
  pair: only an executable decoy remains there. Both argv and ending still name
  the original canonical installation. Mutating lifecycle argv to bare ravel
  reaches the decoy and fails; restore it before the final frozen measurement.

## Verification and handoff

`grove_loop::session_command` supplies three shared helpers:

- `lifecycle(&Path, &Selection, &str, &Path) -> Argv` carries kind, literal
  prompt, exit directory and the selection's task file/handle.
- `confined(&Path, &str, &str, &Path, &[PathBuf]) -> Result<Argv, Error>` adds
  confinement and canonical runtime-file grants before the staged cwd.
- `end_session(&Path) -> Result<String, Error>` quotes the selected UTF-8 path
  and passes exactly `end-session grove-continue`.

All take the already selected executable and perform no lookup or second
canonicalization of it. k39 must use the same selected path for all three and
replace the old actual caller argv/report readers through the existing stdout
capture/epoch/authority seams. The module's staged-caller note and its source
book's corresponding prose change with that activation.

Ravel is the installed 0.1.0 pair. Canonical source SHA-256:

- `/opt/homebrew/Cellar/ravel/0.1.0/bin/ravel`:
  `1e7f61777da30f69fdf6df8ef132fd50cb5a7420cc71bef65cf46e6565e6b87b`
- `/opt/homebrew/Cellar/ravel/0.1.0/libexec/ravel/ravel-policy`:
  `c6d63fc215c15cd86d8752cc972b5111a71cff9b3b1ae622ef4556c25ac88c18`

Complete logs/instruments are under `target/ravel-caller-commands-k38/`:

- `task ravel:commands RAVEL=/opt/homebrew/bin/ravel`: all four shell/installed
  checks pass (`commands-final.log`). Both real worker routes use a separate
  quoted prefix; lifecycle's PATH contains only an executable decoy, and the
  confined route has only the pair's bin directory. Managed output never enters
  the held result. The ungranted credential control observes denial; the granted
  relative-file case completes while the policy stays unreadable.
- `PATH=/Users/antony/Development/Ravel/target/tools:$PATH task test:grove test:grove-loop`:
  147 caller and 387 loop nextest cases pass, with three and five explicit
  installed/role skips respectively (`runtime-final.log`). Loop doctests have no
  executable cases. The bundled prerequisite used verified Bun 1.4.2.
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets` and
  grove-loop's `book-check --final --check all` pass (`format.log`, `clippy.log`,
  `book-final.log`). The book reconstructs all 17 roots and 14,383 declared lines,
  with no deferred source. The library insertion, new module, indexes and
  chapter/README rollups were reconciled together.
- `red.log` observes the absent API; `confine-mutation.log` observes credential
  and policy exposure without --confine; `path-mutation.log` observes the decoy
  execute after lifecycle argv is changed to bare ravel. Both mutations were
  restored before the final frozen checks.
- `measure.py`, `before.json`, `after.json`, `digest-comparison.json`: all 2,016
  individually measured repository inputs, installed source pair, selected
  tools, built worker assets and original 22.1.0 keg files match. This terminal
  handoff prose and retirement follow the comparison.

All first-child criteria hold. k39 remains live for the atomic activation; no
inherited switch ancestor closes. Both actual consumers, active prompts and
installed commands retain the dispatch contract. No installation or release ran.
This session ends through its original `harness-dispatch exit` channel.
