# corpus-disposition-k5

**Kind:** planning

## Goal
Classify all 35 of grove's `docs/adr/` ADRs into a **keep / delete / merge**
disposition, and get **explicit human approval** at the required checkpoint
**before** any delete or merge is executed. This leaf produces the *plan*; the
execution is `corpus-rework-k6`.

## Context
Mandate: **`docs/specs/2026-07-04-adr-minimum-coherent-set-design.md` — Part 3**
(Method steps 1–3 and the required Checkpoint).
- Establish grove's *current* architecture from `README.md`, `content/SKILL.md`,
  `CONTEXT.md`, and the live ADRs (0028–0035 and any others still describing
  current state) before classifying.
- Classify each ADR: **keep** (describes current state → survives, will be
  renamed slug-only and edited to be self-contained/current-state), **delete**
  (superseded / dead → git holds it), or **merge** (a live lesson folded into a
  surviving ADR). Expect the 0013–0030 TUI tower to collapse dramatically.

## Done when
- A disposition table covering **all 35 ADRs** (number, current title, verdict,
  target slug for keeps, merge-target for merges, one-line rationale) is produced.
- The table is **presented to the human and approved** at the checkpoint — no
  delete/merge decided without approval; a live constraint must not be silently
  dropped.
- The approved table is **persisted** where `corpus-rework-k6` can consume it
  (e.g. a companion under `docs/specs/`, or committed alongside this leaf).

## Notes
- **Planning leaf** — the deliverable is the approved plan, not the rework. No
  file `git mv` / delete / merge happens here.
- **Required human checkpoint** — halt and wait for approval; a headless run with
  no human present reports the proposed table and stops.
- Gates `corpus-rework-k6` and `citation-reconcile-k7`. If classification proves
  larger than one session, `leaf-decompose` this leaf (first child only).
