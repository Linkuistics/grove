# owner-switch-process-inspection-k267

## Goal

Replace the switch program's process and job patterns with the narrow
recogniser of Claude Code, Codex, Grove and Ravel. The owner's checklist
covers every other writer.

## Context

- The sections "The owner's checklist" and "The narrow process inspection"
  of `docs/specs/owner-skill-delivery.md` own the contract. "Test seams"
  lists the recognition controls.
- `scripts/owner_switch.py` holds the old gate: `process_gate`, `matches`,
  `descendants` and the plan's `session.mcp` and job patterns. The draft
  generator `docs/evaluations/owner-skill-delivery/switch_plan_draft.py`
  writes those patterns into `docs/owner-cutover/switch-plan-draft.json`.
- The section "Owner switch plan draft" of
  `docs/owner-skill-delivery-evidence.md` records the measured process
  forms, such as the Codex app-server daemon, Node launchers and
  `caffeinate`.

## Done when

- The program recognises the four tools by executable basename, canonical
  path and parsed arguments. It covers native executables, versioned
  packaged binaries, Node launchers with their entry scripts, and Codex
  app-server and daemon subcommands.
- A tool name inside an unrelated argument does not match. An unknown
  launcher form or unreadable executable identity requires review.
- The program admits only itself and the operator's exact harness and
  driver ancestors. It grants no descendant exemption. Another descendant
  harness and an existing app-server refuse the attempt. A `caffeinate`
  child needs no exemption.
- The plan has no process regex, MCP pattern or job pattern. The draft
  generator writes none, and `task owner:switch-plan:check` passes on the
  regenerated draft.
- The old pattern code, its fixture controls and its launchd parsing are
  gone. Recognition fixtures cover each case of "Test seams".
- The program signals no process. The evidence document records which
  checklist duties no fixture can establish.

## Notes

Probe-owned processes appear during probes. owner-switch-probes-k257 supplies
their ownership evidence. Give the inspection a seam that admits only recorded
probe-owned processes, and test it with a fixture.

`task owner:switch-plan:draft` reads the real HOME only. Do not change the
real HOME, launchd or the process table.

## Decisions (running log)

- Measured forms on 2026-11-08 (reading only): Claude Code runs as
  `~/.local/share/claude/versions/<version>` with argv[0] `claude`. The same
  binary runs as `ugrep` for Claude Code's search. Codex daemons run
  `.../app-server-daemon/releases/<version>/bin/codex` with `app-server daemon
  pid-update-loop` or `app-server --listen`. Ravel and Grove run from their
  Cellar paths. Node MCP servers rewrite their title, for example
  `npm exec @playwright/mcp@latest`.
- The recogniser reads the executable with `proc_pidpath` and the argument
  vector with `sysctl` `KERN_PROCARGS2`, not the joined `ps` text. The
  executable basename (`claude`, `codex`, `grove`, `grove-llm`, `ravel`)
  recognises a tool. So does Claude Code's versioned path
  `.../claude/versions/<version>`. A Node process is a tool when its title
  is `claude` or `codex`. It is also a tool when its entry script resolves
  into `@anthropic-ai/claude-code` or `@openai/codex`, or has the stem
  `claude` or `codex`. Comparison is case-sensitive, so the desktop application `Claude`
  stays a checklist obligation.
- Review, not recognition: a tool name in the program position without a
  tool executable. The program position is argv[0], the launch path, or the
  script of a shell, Python, Perl or Ruby interpreter. A process of the owner whose executable path and
  argument vector are both unreadable also requires review.
- 17 of the owner's processes had an unreadable `proc_pidpath`. Each was a
  zombie or ran a binary that an upgrade deleted. For such a process the
  launch path in `KERN_PROCARGS2` is the identity evidence, and the report
  marks it. So two deleted Codex daemons are still recognised as Codex, and a
  deleted watchman is not a review item. Zombies are skipped.
- Processes of other accounts: libproc gives no creation time or arguments
  without privilege. The inspection recognises them by executable path only
  and never sends them to review. The evidence records this limit.
- Admission: the program and its exact ancestors (from `ps` parent IDs).
  No descendant exemption. Probe-owned processes are admitted only when the
  PID, the creation time (`PROC_PIDTBSDINFO`, seconds and microseconds) and the
  canonical executable all equal a record. owner-switch-probes-k257 supplies
  those records and the inspection after probe cleanup.
- Seam: a new entry `processes [--home HOME] [--owned FILE] [--json]` runs the
  inspection and changes nothing. The switch's gate calls the same function.
  `OWNER_SWITCH_REHEARSAL_SCOPE` keeps its k266 meaning for every tool, and
  the program still refuses it with the account's HOME.
- `native-off` keeps its contract: it refuses only other Ravel processes,
  now through the shared recogniser.
- `validate_plan` refuses a plan that still holds `session`, `watch` or
  `jobs`, so a stale draft cannot pass.
- In-session review (the leaf's one allowance) changed these decisions:
  - Reversed: an unreadable executable that its launch path does not
    identify as a tool needs review, as the contract states. The launch path
    still recognises a deleted tool binary. The real table then sends 11
    processes with deleted binaries to review. The owner ends them.
  - Codex helpers `codex-exec-server` and `codex-code-mode-host`: each
    basename with the prefix `codex-` is Codex.
  - Every executable in `.../claude/versions/` is Claude Code, so a
    prerelease name is covered.
  - A title with spaces gives its first word, so a Node server whose title
    ends in a path is not read as a tool.
  - A relative entry script resolves against the process's own folder
    (`PROC_PIDVNODEPATHINFO`). `bun` is a Node runner. Shell and Python
    option values are skipped.
  - Ancestors: the program admits its chain up to the first harness and
    the drivers above it. A second harness ancestor is another session and
    is refused, as are tools above it.
  - A probe-owned record also names the process's HOME, read from the
    environment in `KERN_PROCARGS2`. Only inspections during probes pass
    records.
  - A process that becomes a zombie or exits during the inspection does not
    count. A failed `ps` is a stop.
- Visible trade-offs, recorded in the evidence:
  - The operator's transient children, such as Claude Code's `ugrep` or a
    status-line command, count as other processes.
  - An empty argv[0] cannot be told from the kernel's padding.
  - The inspection recognises other accounts' processes by executable only.
  - An unlisted Node option that takes a value can hide the entry script.
