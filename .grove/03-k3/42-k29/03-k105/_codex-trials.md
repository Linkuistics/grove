# codex-trials-k105 — brief

## Goal

Measure the common loading route in actual Codex sessions, with native copies
absent. Reuse the runner of claude-code-trials-k104 with a Codex adapter.

## Done when

- The runner launches real Codex sessions in the sandbox home of Ravel's
  configuration specification. Fake-harness tests prove the Codex adapter's
  isolation and recording.
- Codex discovers a craft skill from activity alone and reads it before the
  governed action. Mandatory, kind and supporting reads keep their order.
- Every explicit request row, every discovery problem, continuation, both
  compaction cases, one real delegate and the native-only missing-kind case
  have their own record.
- The record states each gap, and each row that does not apply to Codex with
  its evidence. No gap passes as a result.

## Decomposition

The children mirror the Claude Code node, case for case. The first child adds
the adapter. The other children run the trial groups and depend only on it.

1. `codex-trial-adapter-k115` mirrors `claude-trial-runner-k107`.
2. `codex-discovery-trial-k116` mirrors `claude-discovery-trial-k108`.
3. `codex-explicit-routes-k117` mirrors `claude-explicit-routes-k109`.
4. `codex-discovery-problems-k118` mirrors `claude-discovery-problems-k110`.
5. `codex-continuation-trial-k119` mirrors `claude-continuation-trial-k111`.
6. `codex-compaction-trials-k120` mirrors `claude-compaction-trials-k112`.
7. `codex-delegate-trial-k121` mirrors `claude-delegate-trial-k113`.
8. `codex-missing-kind-k122` mirrors `claude-missing-kind-k114`.

## Notes

The sandbox `.codex` holds only a link to the real `auth.json`. Each run sets
`CODEX_HOME` to the sandbox `.codex`. Refuse an inherited `CODEX_HOME` in the
real home, also an empty value, before any launch of the harness.

Codex 0.166.0 has its own sandbox modes. Record the mode of each case, and
whether it runs inside Ravel confinement or in its place.
