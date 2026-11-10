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

## Decisions (running log)

- A direct MCP handshake on 2026-11-10 ran `codebase-memory-mcp` 0.10.6 with
  the staging launcher's allow-listed environment. Without `CBM_CACHE_DIR`,
  the server refused: "the active account daemon uses a different cache
  directory". With `CBM_CACHE_DIR` set to the daemon's folder, it answered
  `list_projects`.
- A new case field `mcp`, `{server, source, skill}`, adds one verdict. The
  agent reads the skill from its source, then makes at least one call to a
  tool of the server whose result is no error. The verdict needs no edit,
  because the edit of `requires` does not show MCP use. A call through the
  `codebase-memory-mcp` CLI is no MCP call.
- `harness_trial.py` maps a Codex `McpToolCall` item to the tool name
  `mcp__<server>__<tool>`, the form of Claude Code. Its error is the item's
  status or the result's `isError`.
- The Codex case of `codebase-memory-mcp` runs first with the unchanged
  staging entry. That run measures whether Codex connects the server, and it
  is the negative control of the verdict on real records.
- `codebase-memory-mcp-codex-1` failed as the probe predicted. The agent
  wrote that its tools held no `codebase-memory-mcp` tool. The staging Codex
  log of 2026-11-09 holds the server's refusal with the same requested cache
  digest. `staging_home.py join-cbm`, which `build` now also runs, replaced
  the entry's `env_vars` line with `env = { CBM_CACHE_DIR = ... }`. The
  staging launcher passes neither forwarded name, so `env_vars` forwarded
  nothing. The staging `config.toml` went from `1faeb55d…` to `e4f41d05…`.
