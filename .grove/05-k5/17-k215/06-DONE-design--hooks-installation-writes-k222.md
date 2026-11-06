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

## Decisions (running log)

**The case (2026-11-06).** The open question at the end of "The hooks
installation inside finish" is unchanged since `2d1df7c400a`. The model's list
of what it does not establish names the same gap.

**How Claude Code writes `settings.json` (2026-11-06).** Scratch measurements
with Claude Code 2.1.319 settled the write method. Each of `marketplace add`,
`plugin install` and `plugin disable` writes the file once. It writes
`settings.json.tmp.<pid>.<hex>` beside the file and renames it onto the path.
A hard link to the earlier inode keeps the earlier content. The rename keeps
the mode and drops each ACL and each extended attribute except
`com.apple.provenance`. A command rewrites the file even when no value
changes. A deny-delete ACL makes the command fail with no write and no
fallback.

**The interval (2026-11-06).** An outside writer raced each command at 46
delays, once by rename and once in place. A command reads the file 30 to 45
ms before its rename. A rename by another program inside that interval is
lost. A write in place inside it lands in the earlier inode. Writes before the
read or after the rename stay in the live file.

**The options (2026-11-06).** Six designs were compared:

- **A held link.** Before each command, a settings record holds a hard link
  to the live inode and its digest. Chosen, with the disabling as an exchange
  and one accepted case.
- **`--scope project` or `--scope local`.** Rejected. The command writes the
  project's settings and records a `projectPath`, and the plugin is disabled
  in every other directory.
- **A private configuration directory.** Rejected. `installed_plugins.json`
  records an `installPath` inside it.
- **Cells written first by the program.** Rejected. Each command rewrites the
  file with unchanged content.
- **A link at the path during each command.** Rejected. Claude Code writes
  through the link to its target, so the harness's own rename races the
  command on that target.
- **The accepted case alone.** Rejected. A writer in place, or a descriptor,
  would lose its write with no report.

**The disabling (2026-11-06).** `plugin disable` changes only the plugin's
`enabledPlugins` cell, and no file under `~/.claude/plugins`. The program
therefore writes `false` by the exchange of step 9, after a settings record.
Detection covers it, and the resumption of an enabled plugin runs no Claude
Code command. `plugin install --marketplace` renames the file twice, so the
registration stays a separate command.

**The accepted case (2026-11-06).** A program that replaces `settings.json`
by a rename after the hold and before the command's rename can lose a write.
Grove's and Ravel 1.1.0's sources name no `.claude` path.
codebase-memory-mcp 0.10.6 wrote nothing in 20 seconds of server mode. The
finish session's own Claude Code process writes user settings by the same
rename after a settings action. The real file's modification time is
2026-10-11, a read-only observation. The bound is two commands, only while
the plugin is absent, each with an interval of 30 to 45 ms.

**A side effect (2026-11-06).** A scratch run of `codebase-memory-mcp
install` stopped every running codebase-memory-mcp server on the machine,
this session's included. It wrote only inside the scratch HOME. The evidence
script runs only the server mode.

**The read after the hold (2026-11-06).** The program takes each held link
first and then reads the plugin's state. Another program can write a third
value into the cell before the link. The state then stops at item 4, and no
command replaces that value with an unflagged held copy.

**The run of record of the measurement (2026-11-06).**
`docs/evaluations/hooks-installation-writes/settings-writes.sh` ran once on
an idle machine. Its subjects kept their digests. Each command read the file
30 to 38 ms before its rename. The Homebrew evidence holds the table.

**The model (2026-11-06).** The command reads the file and later renames onto
the path, under a held link that the next entry erases while its inode is
still at the path. Outside writes to part `HOOKS` land in place or by a
rename. The disabling is an exchange. P1 gains the accepted case. The new
world `hooks_commands` runs 12,000 samples, and the mutant `m_hi_no_hold`
violates only P1. `task model:owner-switch` passed all 241 claims in 6
minutes 8 seconds, with the same digests before and after. The rarest
witness was that mutant's, in 13 of 4,000 traces.

**The review chain (2026-11-06).** No second chain. The design changes the
recovery only inside the hooks installation: its held links, their erasure
in step 3 and the disabling by exchange. `native-settings-recovery-k223`
already reviews this leaf. Its notes now carry this leaf's specific doubts.
