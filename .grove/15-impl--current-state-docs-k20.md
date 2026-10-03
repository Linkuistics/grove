# current-state-docs-k20

## Goal

Make the documents a reader meets describe the result as current state. These
are the documents no test ties to a source change, so the implementing leaves
did not have to move them (P2):

- `docs/ARCHITECTURE.md`: *Process ownership*, the harness-dispatch section,
  and the runner.
- `docs/USAGE.md` and its coverage inventory,
  `docs/specs/user-guide-coverage.md`. `complete` is gone and
  `record-teardown` arrives; the guide covers how a session ends its run and
  `grove run` through dispatch.
- The dispatch README, `crates/harness-dispatch/README.md`:
  - `run` supervises;
  - `exit` and the run flags (`--exit-dir`, `--ending-file`, `--confine`,
    `--runtime-read`);
  - the run ending and the end observation;
  - how a session ends its run;
  - never granting `GROVE_LAUNCH_DIR`.
- This repository's `CLAUDE.md`: the finish sequence's last step and the
  section on invoking `grove-llm` (the cargo guard, now on the new variables).
- `docs/RELEASING.md`'s mentions of the signal channel, such as the `env -u
  GROVE_SIGNAL_FILE` probe for live groves.
- Any other repository document that still describes the `exec` handoff,
  `complete`, `GROVE_SIGNAL_FILE`, the token or Grove-side confinement as
  current.

## Context

- `confined-run-k23`'s committed review, F6, points to decision 7 of
  `docs/specs/module-decomposition.md`. Reconcile its runner API listing with
  the shipped code as part of this document sweep; the integration leaf fixes
  the book-structure ledger, leaving the full module contract to this leaf.

- The spec's *Grove integration* closing paragraphs list what Grove's usage
  guide, the configure-grove skill and the dispatch README must explain, and
  each invocation they quote must be one `--help` carries or a suite makes.
- `docs/ARCHITECTURE.md`'s documentation-ownership table says which document
  owns which subject.
- The specs, ADRs, glossary and visual document were reworked by design.
  Change them only where implementation proved them wrong, and record why in
  the running log.

## Done when

- An enumerate-then-classify sweep (`references/execute.md`), with controls
  seen to fail, covers the repository's Markdown outside `.grove/`. It shows
  every remaining mention of the retired surface is a historical record or
  states the retirement.
- The user-guide coverage test and the repository's link and reference tests
  pass.
- `bash scripts/check.sh` passes.

## Notes

- **`CLAUDE.md` is read by this grove's own finish**, which runs under the v22
  driver and prompt (P5 e). Word the finish sequence's last step so it holds
  under both versions, for example by ending as the launch prompt directs. Or
  agree a wording with the note `major-release` writes for the finish. Never
  leave the finish told to run a verb that cannot end its run.
- `AGENTS.md` is a symlink to `CLAUDE.md`, so a single edit reaches Codex too.
