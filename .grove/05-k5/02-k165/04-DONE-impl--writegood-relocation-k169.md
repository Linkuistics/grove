# writegood-relocation-k169

## Goal

Move `~/Development/AgenticWriting/Writegood` to
`~/Development/AgenticOrchestration/Writegood` by the steps of the parent
brief.

## Context

At planning, Writegood was colocated Git and jj with one jj workspace and an
empty working copy. Git `HEAD` was detached. Git listed no remote.

Writegood holds its own grove tree in `.grove/`, with retired leaves and
possibly live ones. This grove does not edit that tree. A Grove driver or a
harness session for that tree must not be running during the move.

It also holds `.venv`, `.pytest_cache`, `.ruff_cache`, `.writegood`,
`corpora`, `data` and `candidates`. Some of these can hold absolute paths.

## Done when

- The parent brief's Done when holds for Writegood. The tracked-file digest
  includes `.grove/`.
- The digest of every untracked and ignored file outside `.venv` and the
  tool caches is the same before and after the move. Writegood can hold data
  outside version control, so this check is necessary here.
- Each absolute path to the old location inside the project is listed. Each
  one is repaired, or the record states why it stays.
- The project's tests pass at the new path, after `uv sync` recreates the
  environment if the moved `.venv` fails.

## Notes

Writegood has no remote, so the local directory is its only copy. Make a
complete copy outside the project, such as a `tar` archive in the session
scratchpad, before the move. Delete that copy only after the "after" readings
agree. Ask the human before a deletion if any reading differs.

Read the project's `CONTEXT.md` and README before you run its checks.

## Decisions (running log)

- The untracked reading covers every file that Git does not track, outside
  `.git/`, `.jj/`, `.venv/`, `.pytest_cache/`, `.ruff_cache/` and
  `__pycache__/`. Before the move, that set is exactly the 2680 files of
  `.writegood/`. `candidates`, `corpora` and `data` are tracked.
- No file in the project names `AgenticWriting`, including `.jj/`, `.git/`,
  `.venv/` and `.writegood/`. A mutated pattern found nothing.
- Three tracked files name `~/Development/Writegood`, an earlier root. Two
  are recorded session captures in `tests/fixtures/`, and one is the
  measurement record `docs/measurements/render-token-floors.md`. They record
  past runs, so they stay unchanged.
- The `.venv` console scripts already named
  `~/Development/Writegood/.venv/bin/python` before the move. The tests run
  only after the move, so that the test run cannot change a "before" reading.
- The readings before the move, after the move and after the test runs are
  identical. The scratchpad archive held all 8029 entries, and this session
  deleted it after the readings agreed.
- `uv sync --frozen --reinstall` repaired the moved `.venv`. With it, one of
  688 tests fails. The test
  `test_safe_mode_on_a_guided_argv_is_refused` reads the installed
  `claude --version`, which is `2.1.318`. Writegood holds token floors only
  for `2.1.233` and `2.1.234`. With a stand-in `claude` that reports
  `2.1.234` and passes every other call to the real one, all 688 tests pass.
  The failure therefore comes from the environment, not from the move. The
  product is not changed here.
- The full old path occurs only in session transcripts and in this note. The
  rollout text for paused groves needs no entry for Writegood.
- `~/.claude.json`, `~/.claude/projects/` and `~/.codex/config.toml` name
  `~/Development/Writegood`, the earlier root. They do not name the path of
  this move, so they stay unchanged.
