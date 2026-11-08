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
