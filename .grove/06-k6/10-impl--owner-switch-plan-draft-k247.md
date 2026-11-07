# owner-switch-plan-draft-k247

## Goal

Draft the real switch plan for the owner's HOME, with every field of the
plan format. The program's loader and its dry run must accept the draft in
a disposable layout.

## Context

- The docstring of `scripts/owner_switch.py` states the plan format,
  schema `owner-switch-plan/1`. Each field is required. Each path is
  relative to HOME, and its parent folder resolves inside HOME.
- Order 4 of the cutover contract calls this draft "the residue plan". The
  sections "Classes and dispositions", "Native residue" and "Name collisions
  and stale copies" of `docs/specs/owner-skill-delivery.md` state each
  entry's disposition.
- The section "Owner-pack entries of the baseline" of
  `docs/repository-moves-record.md` maps each baseline line to its source or
  its route. The section "Re-inventory before the cutover" of
  `docs/owner-skill-delivery-evidence.md` states the differences since the
  baseline.
- owner-settings-drafts-k246 supplies the off value and the switched value
  for the cells of `.config/ravel/settings.json`.
- The parent brief's "Owner-delivery handoff" assigns items to this plan.
  owner-switch-gates-k241 and owner-switch-program-k243 added the rules
  for gates, MCP patterns and the staging paths.

## Done when

- `parts` holds each entry that the switch moves and each cell that it
  changes. The entries cover both Codex user roots' Grove snapshot links,
  the stale copies, the dangling links and the quint links. The cells cover
  each plugin of `enabledPlugins` that the switch disables or enables, and
  the `skills` key of the Ravel settings. Each entry carries the change
  identity that `owner_switch.py identity` prints. A committed table maps
  each part to its inventory row.
- `outside` names the native residue that must stay unchanged. It names no
  plugin version directory, because each Claude Code session writes
  `.in_use/<pid>` there.
- `session.mcp` matches each of the owner's MCP server processes and each
  launcher chain, such as `npx` and the `node` server that it launches. A
  committed listing of the owner's real process tree shows each match.
- `state` is `.local/state/owner-switch`. `manual` names the maintained
  manual. `baseline` names the inventory instrument. Each list of steps 4 to
  7 names `["$python", "-I", "$program", "gate", "<step>"]`.
- `probes`, `native_probe` and `mac_check` hold the commands that the
  sections "The switch inside finish" and "Rollback" describe.
- `accepted` holds provisional values from the moves record, and the record
  says so. owner-switch-plan-k254 replaces them with the staging evidence.
- In a disposable layout built from the draft, the program admits the plan,
  and `run --dry-run` lists each part and its inverse. That evidence is
  committed.

## Notes

The draft does not go into the real `<state>/prepared` folder. Only
owner-switch-plan-k254 places the completed plan there.

The plan is load-bearing: finish runs it against the owner's HOME. Decide at
the end whether it earns a `review-impl` leaf.
