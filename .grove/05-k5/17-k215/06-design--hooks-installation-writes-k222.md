# hooks-installation-writes-k222

## Goal

Decide how the writes of the hooks installation inside finish take part in
detection, or state an equivalent mechanism. Claude Code's own commands make
those writes, so the program's exchange cannot publish them.

## Context

- `native-settings-recovery-k221` found this case. Its running log holds the
  reasons. The owner design states it as the open question at the end of
  "The hooks installation inside finish".
- The hooks installation runs in step 3 of the switch, while no run is open.
  It runs `claude plugin marketplace add`, `claude plugin install` and
  `claude plugin disable` against the real HOME.
- `docs/homebrew-skill-delivery-evidence.md`, "The hook plugins across a keg
  change", measured two facts. `marketplace add` writes
  `extraKnownMarketplaces` in `settings.json`. `plugin install` sets the
  plugin's `enabledPlugins` cell to `true`, and no option installs a plugin
  disabled. `plugin disable` writes the same cell.
- `~/.claude/settings.json` holds cells of the switch set. "Concurrent
  writers" requires detection of each write by another program, and property
  1 of `docs/models/owner-switch.qnt` states the same rule.
- The exclusion keeps the kept processes. A kept MCP server, such as
  codebase-memory-mcp, can rewrite its hooks in that file.
- The model treats each command as one atomic write of one cell. Its list of
  what it does not establish names this gap.

## Done when

- The owner design states how each write of the hooks installation takes part
  in detection, or why an equivalent mechanism protects it. An accepted case
  states its evidence and its bound.
- A measurement in a scratch HOME supports each claim about how Claude Code
  writes `settings.json`. No measurement uses the real HOME.
- If a recovery rule changes, `docs/models/owner-switch.qnt` models it first.
  `task model:owner-switch` stays green, and a mutant instance shows each new
  mechanism.
- The test seams cover each new mechanism or accepted case.
- The views in `docs/design/harness-selection-and-execution/` agree.

## Notes

Compare at least these options before choosing one:

- **A held inode.** Before each command, the program records the file's
  digest and keeps a hard link to its inode in a settings record. This works
  only if Claude Code replaces the file by a rename. A write in place would
  defeat it, so measure the write method first.
- **Writes outside the switch set.** The commands write only files outside
  the switch set, and the program changes the `settings.json` cells by its
  own exchange. Check whether any documented option, such as `--scope`, keeps
  a command out of `settings.json` without changing what the plugin
  installation means.
- **An accepted case.** Only the exclusion protects the commands. That needs
  evidence about which kept processes can write `settings.json` while the
  commands run.

The review chain that follows this leaf covers this design together with
`native-settings-recovery-k221`. Do not cut a second review chain unless this
design changes the recovery beyond the hooks installation. Then record why in
the running log.
