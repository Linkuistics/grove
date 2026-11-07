# owner-switch-simplification-k260

## Goal

Redesign the owner skill switch for the owner's real situation. This grove
is the only grove that runs, and the owner is the only user of Grove and
Ravel. Remove each mechanism that only defends against other actors. Keep
each rule that a single owner still needs.

## Context

The human's words on 2026-11-08: "you are the only grove running, and I am
the only user of grove and ravel. You should use that information to
simplify the switch over".

The human then chose all four simplifications that
owner-switch-plan-draft-k259 offered. Each line is the offered direction,
not a settled design:

- **Process and job exclusion.** An owner checklist replaces the automatic
  listing of processes, jobs and login items. That listing has patterns,
  helpers and launchd parsing. The program checks only that no other Claude Code,
  Codex, Grove or Ravel process runs.
- **Concurrent-writer detection.** One backup, a plain write and a final
  comparison replace four mechanisms. They are the atomic exchange, the kept
  and flagged copies, the merge entry and the held links around Claude Code
  commands.
- **Recovery state machine.** One backup and one restore command replace
  the resumable states, the program copies in backups, the
  pending-activation records and the Quint model. On any failure, the
  program restores everything and stops.
- **Rehearsal admission.** One dry run of the real plan against a
  disposable copy of HOME replaces the digest-bound rehearsal records that
  admit a plan.

The current design and its implementation:

- `docs/specs/owner-skill-delivery.md`: "The switch inside finish",
  "Concurrent writers", "Resuming the switch", "The rehearsed plan",
  "Rollback", "The recovery model", "Purge", "The native settings inside
  finish", "The hooks installation inside finish", "Claude Code's own
  writes", "Cutover contract" and "Test seams".
- `docs/adr/finish-promotes-after-the-grove-release.md`.
- `scripts/owner_switch.py`, `scripts/owner_switch_rehearsal.py`,
  `docs/models/owner-switch.qnt` with `task model:owner-switch`, the
  switch row of `mac-check`, and
  `docs/evaluations/owner-skill-delivery/switch_plan_draft.py` with its
  draft `docs/owner-cutover/switch-plan-draft.json`.
- The design diagrams `owner-cutover.mmd` and `owner-switch-states.mmd`
  in `docs/design/harness-selection-and-execution/`.

The review owner-switch-plan-draft-k258 found five defects in the draft.
The section "Owner switch plan draft integration" of
`docs/owner-skill-delivery-evidence.md` records their dispositions. Two
findings dissolve with an automatic listing: D2, the job labels, and D4,
the `caffeinate` exemption. Three stay relevant to any design:

- D3: a new native copy of a superseded skill can appear in
  `~/.claude/skills` or `~/.codex/skills`. A platform rewrites their
  folders `synced` and `.system`, so a comparison must skip only those
  folders and still see each sibling.
- D5: the inventory instrument classifies an enabled `guardrail@personal`
  as `UNCLASSIFIED`. The switch enables that plugin, so the instrument
  reports a valid switched state as an anomaly.
- D1: a dry run certifies a plan only when it identifies each file that the
  plan runs, other than the program, by its content.

k259 measured these facts while it prototyped repairs that it then
discarded:

- During a turn, Claude Code runs `caffeinate -i -t 300` as a child of its
  harness process. That child writes no file.
- A probe session can rewrite `~/.claude/skills/synced` and
  `~/.codex/skills/.system`.
- `launchctl print gui/<uid>/<label>` shows the program arguments of each
  loaded job. All 577 jobs took about 2 seconds.
- launchd reads the account's own home, whatever HOME names. A job listing
  that reads `~/Library/LaunchAgents` through HOME misses the owner's jobs
  in a disposable layout.

## Done when

- The owner design states the simplified switch. For each of the four
  mechanisms, it states what remains, what goes, and why the single-owner
  facts make the removal safe.
- The design separates the owner's checklist from the program's checks. A
  hazard that the facts do not remove stays covered. One example is the
  finish session's own Claude Code process, which writes
  `~/.claude/settings.json`. Other examples are the owner's own tools,
  such as `mise dot watch`, herdr and the codebase-memory updater.
- Each rule that does not depend on other actors stays, or the design
  states why it goes. These are the rules:
  - Each name has one active copy.
  - The final session's guidance stays readable through native removal.
  - A failed probe restores native delivery.
  - No upgrade removes a running driver's binaries.
- D1, D3 and D5 are resolved in the design, or the design drops each one
  with its reason.
- The design names each artifact that the change removes or shrinks. The
  candidates are the program, the rehearsal, the model, the `mac-check`
  states, the draft generator, the diagrams and the evidence. It states the consequence for
  each live leaf from owner-staging-setup-k248 to
  owner-switch-rehearsal-k255, and for the parent brief's "Owner-delivery
  handoff". owner-switch-simplification-k262 plans from that statement.
- The spec set and the decision records are current. Search the claim, not
  a file list.

## Notes

This leaf changes the design only. The implementation follows the leaves
that owner-switch-simplification-k262 plans.

The real HOME is read only. The cutover's ordering inside finish stays,
unless the design changes it on its merits and states why.
