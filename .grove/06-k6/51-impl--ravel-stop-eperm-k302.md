# ravel-stop-eperm-k302

## Goal

Find out why the accepted Ravel 2.0 build ended one staging session with
`supervision_failed`. Repair Ravel before its release cut if the cause is in
Ravel. A real Grove session that ends this way stops the Grove loop.

## Context

- core-candidate-restaging-k300 ran the staging case `stale-copy` on Codex
  0.167.0 with `gpt-6.1-sol` at low effort. The record is
  `~/.local/state/owner-staging/trials/stale-copy-codex-2`. The agent ran
  `ravel end-session staging-ok` as its last event.
- Ravel then exited 5. `ravel.stderr` in that record holds this message:

  ```text
  required job stop failed; stop attempt 1 target -32130 signal 15:
  Operation not permitted (os error 1); stop attempt 3 target -32130
  signal 9: Operation not permitted (os error 1); and reaped
  ```
- The build is the accepted 2.0 build at
  `~/.local/state/owner-staging/ravel-2.0`. Its build ID is
  `58d479ed058f…`, from `Ravel.skill-directory-delivery` at `ff14cba826b0`.
- No other staging record holds `supervision_failed`. The same case passed
  with Ravel exit 0 in `stale-copy-codex-1`. The session started Codex's MCP
  servers, which include `codebase-memory-mcp` and `npm exec
  @upstash/context7-mcp`.
- One hypothesis is that macOS `killpg` returns `EPERM` for a group whose
  members have all exited but are not yet reaped. Ravel would then treat a
  stopped group as a failed stop. No measurement supports this hypothesis yet.

## Done when

- A measurement or a reproduction names the cause.
- A Ravel cause has a repair and a test on the 2.0 line before the release
  cut. Otherwise, the record states why Ravel's report is accurate and what
  the owner must do.
- The evidence of the 2.0 line records the result.
