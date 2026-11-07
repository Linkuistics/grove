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

## Decisions (running log)

- The draft is `docs/owner-cutover/switch-plan-draft.json`. The script
  `docs/evaluations/owner-skill-delivery/switch_plan_draft.py` writes it
  from the real HOME, read only. It also writes the part table
  `docs/owner-cutover/switch-plan-parts.tsv`.
- `parts` holds 154 entries and 2 files. The entries are `grove` and
  `terminal-browser` in `.claude/skills`, 59 children of `.codex/skills`,
  48 children of `.agents/skills`, 44 dangling links in `.pi/agent/skills`
  and the dangling Pi extension link. Each skill root keeps
  `codebase-memory`. `.claude/skills` also keeps `synced`, and
  `.codex/skills` also keeps `.system`. The Pi extensions folder keeps five
  tool extensions.
- The cells of `.claude/settings.json` disable `grove`, `linkuistics` and
  `testanyware` of `linkuistics`, `superpowers` and `quint-llm-kit`. They
  enable `technical-communication-hooks@writing-hooks` and
  `guardrail@personal`. The earlier value of those two is `false`, the
  value after the hooks prelude and after the preparation of
  owner-pack-checkouts-k93. The Ravel cell goes from the off value to the
  `skills` value of `settings-switched.json`.
- `outside` names no root that a platform writes. `.claude/skills/synced`
  holds account-synced skills, and `.codex/skills/.system` holds Codex's
  system skills. A probe session can rewrite either, and step 14 would
  then count a failed probe. Those two roots therefore have path entries
  for `codebase-memory` only. Root manifests cover `.agents/skills`,
  `.pi/agent/skills`, `.pi/agent/extensions`, `.claude/agents` and
  `.codex/agents`. The path entry `.agents/.grove` keeps the snapshot store
  that the moved links name.
- `baseline` and the probes run programs from `<state>/prepared`, by paths
  relative to HOME, because each command runs in HOME. The instrument is
  `inventory.py`, which needs Python 3.11 or later for `tomllib`, so it
  runs under `$python`. owner-switch-plan-k254 copies it there.
- No probe program exists. The draft names the interface of
  `owner_switch_probes.py`: `resolve --expect switched|disabled` and
  `session --harness claude|codex --route ravel|native`. A new leaf before
  owner-switch-plan-k254 implements it.
- `accepted.trials` holds provisional commands that fail with a message.
  A step 4 that needs a rerun therefore stops until k254 names the trials of
  owner-staging-discovery-k249.
- Finding: Claude Code runs `caffeinate -i -t 300` as a child of the
  harness process. The design's kept processes do not name it, so step 2
  would always stop. The draft keeps it through `session.mcp`, the only
  plan field that keeps a child of the harness. The design should name it.
- `watch` matches the program token of each command, not any substring, so
  a path such as `.../grove ` in another argument does not match. It adds
  the applications Claude, ChatGPT, Herdr and Orca. ChatGPT holds Codex's
  `node_repl` MCP server. `OrcaSlicer` does not match.
- `jobs` runs `launchctl list` and `crontab -l`. `sfltool dumpbtm` waits for
  authorisation, so the listing cannot read login items without privileges.
  That is a limit of step 1.
- `check` copies each named entry from the real HOME into a disposable
  HOME and requires each copy's change identity. The dry run there reported
  `native` and listed each part with its inverse. Five controls failed as
  expected, and 18 pattern cases passed. Three broadened mutants of the
  patterns failed those cases.
- The probe program is a separate leaf, owner-switch-probes-k257, before
  owner-switch-plan-k254. It needs real harness sessions, which do not fit
  this leaf.
- Review: the draft earns a `review-impl` leaf, placed before the staging
  leaves. k254 copies the draft's judgements into the real plan. The
  rehearsal tests behaviour against the plan, not the plan against the
  design.
