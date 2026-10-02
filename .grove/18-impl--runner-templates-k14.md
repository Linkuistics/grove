# runner-templates-k14

## Goal

The runner is a unit that takes a program and its arguments and knows no
configuration. Its template, vocabulary, inspection and conformance machinery
is deleted, and so is the last record of Grove configuration.

## Context

- `docs/specs/harness-selection-and-execution.md`, *The runner* under *Grove
  integration*, and the runner row of the test seams.
- `docs/adr/the-launched-child-is-a-job.md`: the contract that stays.
- Root brief, requirements 2 and 14.
- `direct-dispatch-k4`'s notes *Where the design lands in the code* (the
  *Runner* entry) and *Records this design left for the leaf that deletes the
  machinery*.

## Done when

- The template, vocabulary, inspection and conformance modules and the
  configuration half of the error type are deleted, with the tests of them.
- The runner's remaining tests build their argv with the public constructor.
  The job, terminal, channel, escalation and confinement cases hold as before.
- No record of Grove configuration remains: the two configuration ADRs, the
  modular-configuration specification and design, the configuration reference
  and its forms audit. Their citations are gone too.
- `docs/specs/module-decomposition.md` follows the code: decision 6 goes,
  decision 7 is the runner alone, and the surviving decisions keep their
  numbers.
- The three ADRs that mention configuration validation as a driver step are
  reworded. Their decisions stand.
- The glossary's **Grove configuration** cluster, **Kind routing** and **Review
  target diversity** are retired or reworded.
- The `keyed-launch` book and its structure specification describe what
  remains.
- `bash scripts/check.sh` passes.

## Notes

- The crate keeps its name.
- Nothing here moves the runner under dispatch or changes the `exec` handoff.
- The book loses its template chapters. That is a restructure and not a patch,
  so decompose at the book if the code and the book do not fit one session.
- `grove-loop`'s inline lease tests still build their argv with
  `keyed_launch::Templates`, from a `launch.kdl` they write: in
  `driver_lease.rs`, `driver_lease/observation.rs` and
  `driver_lease/witnesses.rs`. They need `Argv::new` when the templates go, and
  the `grove-loop` book reproduces all three files. `lifecycle-launch-k12` left
  them: they launch no session and read no Grove configuration.
- `grove-configuration-k13` deleted both configuration ADRs with
  `session_config.rs`, which held their last citation from live code. What it
  left, each still describing deleted Grove code: the `keyed-launch` book's
  chapter 5, which cites `session_config.rs` and its test as evidence;
  `docs/specs/module-decomposition.md`'s decisions 6 and 7, which name
  `SessionConfig` and the `grove config` commands; the rest of the glossary's
  **Grove configuration** cluster; and `docs/CONFIGURATION.md`,
  `docs/specs/modular-configuration.md`, its design directory and the forms
  audit, where only links into deleted files were removed.
- A book's line ranges and its source index are derived data. `k13` rebased two
  books with throwaway scripts: carry each fragment's range across a `difflib`
  diff of the root's old and new text, re-read every literal body from the
  file, then rewrite each index row from `walkthrough.toml` and the fragment
  headers. Only the prose and the roll-up figures needed hands.
