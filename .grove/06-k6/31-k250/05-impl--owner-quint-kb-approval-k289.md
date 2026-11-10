# owner-quint-kb-approval-k289

## Goal

Get the owner's decision on the real Codex `quint-kb` entry, and apply
that decision. Codex sessions that Ravel starts for the owner cannot call
`quint-kb` today.

## Context

- "MCP servers in staging" of `docs/owner-skill-delivery-evidence.md`
  holds the measurement. In `quint-mcp-codex-1`, Codex 0.167.0 denied each
  `quint-kb` call with "MCP tool call requires approval, but approval
  policy is never".
- The owner's policy starts Codex with `--ask-for-approval never`. The
  real `config.toml` approves each of eight `codebase-memory-mcp` tools and
  no `quint-kb` tool. Native delivery has the same gap, so the switch does
  not cause it.
- In staging, `default_tools_approval_mode = "approve"` on the `quint-kb`
  entry repaired the case. `staging_home.py mcp-entries` makes that change
  in a staging HOME only, and the docstring cites the Codex source.
- "Native residue" of `docs/specs/owner-skill-delivery.md` lists the
  `quint-kb` server as unchanged by the switch. "Cutover contract" requires
  a repair or an explicit owner scope decision for a failed case.

## Done when

- The owner chose one of three options, and the task file records the
  choice. The options are: approve each `quint-kb` tool through the
  server-wide key, approve only named tools through `approval_mode`, or
  keep the gap as an accepted scope.
- After an approval, a real Codex session that Ravel starts calls a
  `quint-kb` tool with no error. The real `config.toml` changes only in
  the `quint-kb` entry, and its digest before and after is recorded.
- The evidence document records the decision and any real-HOME
  measurement.

## Notes

The real `config.toml` is outside the switch set. After an approval, ask
the owner where the entry is recorded: in the switch plan, in the manual or
nowhere. A record lets a rebuilt machine keep the entry.
