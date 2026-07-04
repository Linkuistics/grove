# citation-reconcile-k7

**Kind:** work

## Goal
Reconcile **every** ADR citation across the repo to the final survivor slugs from
`corpus-rework-k6`, and convert the in-prose numbered `ADR-NNNN` citations to
slug/title form. A merge/delete that leaves a dangling `ADR-NNNN` is a defect —
this leaf makes the whole repo internally consistent.

## Context
Mandate: **`docs/specs/2026-07-04-adr-minimum-coherent-set-design.md` — Part 3
step 5**, plus the Part-2 citation list.
- Reconcile citations across `content/`, `docs/`, `docs/research/`,
  `docs/workflows/`, and any BRIEFs.
- **Re-grep `ADR-[0-9]` across the tree** — do **not** trust the spec's snapshot
  of "6 known citations" (`TASK-FORMAT.md`, `SKILL.md` ×4, `prompts/continue.md`);
  the corpus rework may have changed which slugs exist and which files cite them.
- Includes the citation edits **deferred** from `grove-adr-note-k3` and
  `grove-process-prose-k4`.

## Done when
- `grep -rE 'ADR-[0-9]'` across the repo returns no citation that fails to
  resolve to a surviving slug (allowing only intentional historical references,
  if any, called out explicitly).
- Every ADR reference points to a current survivor slug/title.
- The deferred citation edits from leaves k3 and k4 are completed.
- Committed in `grove`.

## Notes
- **Must run last** — after `corpus-rework-k6` so the survivor slugs are final.
  Running it earlier would reconcile against slugs that later change.
