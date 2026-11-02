# claude-code-trials-k104 — brief

## Goal

Measure the common loading route in actual Claude Code sessions, with native
copies absent. Leave a reusable trial runner for the Codex node.

## Done when

- A runner launches real Claude Code sessions through a disposable Grove tree
  and the 2.0 build, in the sandbox home of Ravel's configuration
  specification. Fake-harness tests prove its isolation and recording.
- Claude Code discovers a craft skill from activity alone and reads it before
  the governed action. Mandatory, kind and supporting reads keep their order.
- Every explicit request row, every discovery problem, continuation, both
  compaction cases, one real delegate and the native-only missing-kind case
  have their own record.
- The record states each gap. No gap passes as a result.

## Decomposition

1. `claude-trial-runner-k107`: the runner, its fake-harness tests and one real
   smoke launch. It credits no acceptance case.
2. `claude-discovery-trial-k108`: implicit discovery, read order, source provenance,
   a resolution without a network and one confined run.
3. `claude-explicit-routes-k109`: name requests, loader outcomes, the native-only
   restriction and a concrete path.
4. `claude-discovery-problems-k110`: ambiguity, missing guidance, a stale native
   copy and a user-invoked skill.
5. `claude-continuation-trial-k111`: continuation, with retained reads after source
   and settings changes.
6. `claude-compaction-trials-k112`: loss of the path, then loss of the whole
   preamble.
7. `claude-delegate-trial-k113`: one real delegate, its reads and its inherited
   ending authority.
8. `claude-missing-kind-k114`: the native-only missing-kind case and, separately,
   the unreadable confined case.

Leaves 2 to 8 depend only on the runner. Each records its own results.

## Notes

The owner signs in to Claude Code with a claude.ai account. On macOS the
sandbox home links only `Library/Keychains`. Refuse the five Claude Code
location variables that Ravel's configuration specification names, before any
launch of the harness.
