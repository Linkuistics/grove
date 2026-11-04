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
