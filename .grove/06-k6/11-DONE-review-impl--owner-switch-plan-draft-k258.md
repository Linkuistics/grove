# owner-switch-plan-draft-k258

**Reviews:** owner-switch-plan-draft-k247

## Goal

Read the owner's switch plan draft adversarially against the owner design.
Find each entry, cell, pattern or command that is wrong for the real HOME.
A wrong item lets the switch change the HOME wrongly, miss a reader or a
writer, or stop a valid switch. Produce findings, not fixes.

## Context

- The subjects are `docs/owner-cutover/switch-plan-draft.json`,
  `docs/owner-cutover/switch-plan-parts.tsv`, the script
  `docs/evaluations/owner-skill-delivery/switch_plan_draft.py` and its two
  tasks `owner:switch-plan:draft` and `owner:switch-plan:check`.
- The section "Owner switch plan draft" of
  `docs/owner-skill-delivery-evidence.md` states each choice. The producer's
  task file holds the running decisions.
- `docs/specs/owner-skill-delivery.md` owns each rule. The relevant sections
  are "Classes and dispositions", "Native residue", "Name collisions and
  stale copies", "Resource lifetime", "The switch inside finish",
  "Concurrent writers" and "Rollback". The docstring of
  `scripts/owner_switch.py` owns the plan format.
- The reference inventory `docs/owner-skill-inventory-reference.tsv` and the
  section "The handoff to the owner cutover" of
  `docs/repository-moves-record.md` hold each entry's class and route.

## Done when

- Each finding names the defect, a concrete state of the owner's HOME or
  process tree that shows it, and the rule that it breaks.
- The findings cover these doubts:
  - The draft names no root manifest for `.claude/skills` or
    `.codex/skills`, because a platform writes `synced` and `.system`. Does
    that leave a new native copy of a superseded skill undetected?
  - The draft keeps `caffeinate` through `session.mcp`. Is that sound
    against "Concurrent writers", and can another child of the harness
    process need the same treatment?
  - Do the watch and job patterns cover each program and job of step 1?
    The job listing cannot read login items without privileges.
  - The earlier values of the hooks plugin and `guardrail` are `false`, but
    both keys are absent in the real HOME today. Is each earlier value the
    value that the HOME holds before step 9?
  - `outside` names `.agents/.grove`, two `codebase-memory` copies and five
    root manifests. Can a probe session or a kept MCP server change one of
    them, so that step 14 runs the inverse after a valid switch?
  - `baseline`, `probes` and `native_probe` run files of `<state>/prepared`
    that the plan's digest does not cover. Does that weaken step 8 or a
    continuation?
- No finding is a fix. A review that finds nothing worth acting on cuts no
  integration leaf.

## Notes

The real HOME is read only for this review. Use the check task in a scratch
folder. Do not write into `.local/state/owner-switch`.

## Decisions (running log)

- The reviewed producer is commit `9490761a9099`. The review is inspection
  only: it reads the committed check record and source, and runs no check,
  test, build, lint or format task. The real HOME and the real switch state
  are read only.
- The absent plugin keys are not defects in the earlier values. Cutover
  order 6 installs `guardrail` disabled, and step 3 installs and disables
  the writing hooks plugin before step 9.
- Actionable findings concern new children of the omitted discovery roots,
  external command identity, job enumeration, the non-server exemption and
  the inventory's classification of enabled `guardrail`. The findings are
  recorded in the section "Owner switch plan draft review" of
  `docs/owner-skill-delivery-evidence.md`.
- No false reversal from ordinary reads of the eight `outside` items is
  established. Their identities omit timestamps and do not follow links.
  Installer writes can change them; the current design explicitly treats
  those changes as failed probes. No update or real probe was run here.
