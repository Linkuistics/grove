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

codex-trial-adapter-k115 delivered `task trial:codex OUT=… CASE=…`, which runs
any case in Codex through its own tree, with `--harness codex`. "The Codex
adapter" in `docs/common-loading-acceptance-record.md` states the route, the
`workspace-write` sandbox and its reason, the residue and the helper links.
The fields `continuation`, `compaction`, `expectDelegate` and `systemNote`
refuse for Codex until k119, k120 and k121 add their mechanisms. The delegate
leaf also adds Codex delegations, which the extraction does not yet link.

codex-discovery-trial-k116 recorded "discovery in Codex", which passes, and
"discovery-confined in Codex", which is a gap. Codex exits before its session
under Ravel confinement. The confined environment holds no `CODEX_HOME`, and
the sandbox denies writes in `$HOME/.codex`. The confined case of
codex-missing-kind-k122 probably meets the same gap.
