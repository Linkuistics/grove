# 030-update-skill-and-glossary

**Kind:** work

## Goal

Update the grove methodology so it teaches the inbox convention as a
first-class part of the loop, not as bolted-on tooling. Two artifacts
to update: `content/SKILL.md` (the methodology source bundled into
harnesses, including this very worktree's `.claude/skills/grove/`) and
the bundled glossary template, so future repos inherit the Inbox / Seed
/ Drain / `grove-inboxes branch` entries by default.

## Context

- `content/SKILL.md` in the main repo — the source of truth for the
  methodology; this is materialised into each repo's
  `.claude/skills/grove/SKILL.md` by `grove install` / `grove update`.
- `.claude/skills/grove/SKILL.md` (in this worktree) — the materialised
  copy; useful for reference while editing the source. Do not edit the
  materialised copy directly.
- `CONTEXT.md` (this worktree) — the four glossary entries
  (`Inbox`, `Seed`, `Drain`, `grove-inboxes branch`) added during the
  020 grilling. The bundled glossary template should pick these up so
  new repos start with them present (or pointed to).
- The two ADRs from sibling leaves 010 and 020.

## Done when

- `content/SKILL.md` "The loop" section names **drain** as part of
  Bootstrap (the receiving grove's session reads its inbox alongside
  glossary, ADRs, and brief chain) and as part of the per-task flow on
  every `grove start` and `grove continue`.
- `content/SKILL.md` "Artifacts" table gains a row for the
  `grove-inboxes` branch (cross-grove inbox files), keeping the table's
  format and tone.
- `content/SKILL.md` notes that the convention is CLI-mediated: the LLM
  uses `grove inbox <name>` (or whatever 040 settles on) rather than
  writing files directly on the inbox branch.
- The bundled glossary template (wherever `CONTEXT.md` is seeded from
  during `grove install`, if such a template exists in `content/`) gains
  Inbox / Seed / Drain / `grove-inboxes branch` entries. If no such
  template exists, the methodology doc points at this grove's
  `CONTEXT.md` as the worked example.
- The two ADRs (0002, 0003) are cited from the relevant SKILL.md spot
  so a session reading the methodology can find the decision record.

## Notes

- Do not edit `.claude/skills/grove/SKILL.md` in this worktree directly.
  The materialised copy will be refreshed by `grove update` once the
  source changes.
- The SKILL.md spine (the "seven constraints") must remain on one page
  (constraint 7). Add to existing sections rather than introducing a
  whole new top-level section if at all possible — the drain step fits
  naturally inside Bootstrap.
- The artifact table addition is a single row; resist the urge to
  expand `Inbox`/`Seed` into their own rows (they live on the branch,
  not as separate artifact classes).
