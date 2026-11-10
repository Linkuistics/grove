# staging-mcp-trials-k286

## Goal

Prove on each harness in the staging HOME that the Quint and
codebase-memory skills work with their MCP servers through Ravel delivery.

## Context

- The parent brief holds the trial list and the staging layout.
- The staging Claude Code entry of `codebase-memory-mcp` sets
  `CBM_CACHE_DIR` to the account daemon's cache folder, and it connects.
  The server refuses a client with another cache folder.
- The staging Codex entry of `codebase-memory-mcp` passes `CBM_CACHE_DIR`
  and `CBM_RUNTIME_DIR` through `env_vars`. The staging launcher sends an
  allow-listed environment that holds neither name. Measure whether the
  Codex server connects before a trial depends on it.
- `quint-kb` runs from the Quint clone. It connected on Claude Code in the
  isolation trial.

## Done when

- On each harness, an agent uses `quint-lang` or `quint-modeling` from the
  `quint` source with the `quint-kb` MCP server.
- On each harness, an agent uses `using-codebase-memory` from its source
  with `codebase-memory-mcp`.
- The evidence document records each trial, its versions and its result,
  and each change to the staging configuration with its reason.
