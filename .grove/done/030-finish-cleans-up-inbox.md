# 030-finish-cleans-up-inbox

**Kind:** work

## Goal
Make the Complete finish cycle clean up the finished grove's `grove-meta` inbox
so a finished grove stops masquerading as a Seed in `grove status` / the TUI.
Today the finish cycle removes the worktree and branch but orphans
`inboxes/<name>/`, and by the glossary's own definition a finished grove's
leftover inbox *is* a Seed — indistinguishable from a not-yet-started one.

## Context
- The symmetric finish-side counterpart to evidence item 4 (start side): startup
  mis-signals a newborn grove as *done*; finish mis-signals a dead grove as a
  *seed*. Both are lifecycle-state legibility bugs on the seed/done axis — the
  reason this grove was broadened to cover both ends.
- CONTEXT.md — the **Seed**, **Inbox**, **Drain**, **grove-meta branch**, and
  **Complete finish cycle** entries. Note Seed explicitly includes "already
  finished"; this leaf changes that.
- ADR-0010 — in-session finish cycle (the five steps; this adds inbox cleanup).
- ADR-0005 — grove-meta sync semantics (fetch/push, non-ff refuse-and-instruct):
  the inbox lives on grove-meta, so cleanup is a grove-meta write and must follow
  the same best-effort-push / soft-on-offline rules.
- ADR-0007 — `grove status` / TUI as the canonical visibility surface: this is
  where the "finished grove shows as seed" symptom is observed.
- `grove meta` / `grove-llm inbox-*` verbs — the existing surfaces for grove-meta
  reads/writes; the LLM must not touch grove-meta git plumbing directly.

## Design decision to settle first (resolve before implementing; grill if needed)
**What does finish do when the inbox is NOT empty?** Drain runs only at `grove
do`, so other groves may have added observations since this grove's last start.
Blindly deleting `inboxes/<name>/` could lose un-triaged observations.
- Option A (recommended): finish drains/triages pending observations first; if
  any remain unresolved, **refuse and instruct** (mirroring ADR-0005's non-ff
  refusal) rather than silently delete; once clear, remove `inboxes/<name>/`
  entirely (including `.gitkeep`) as a new finish step.
- Option B: delete only when already empty; if pending, warn and leave the inbox
  (it legitimately still has work — but it keeps showing as a seed).
- Option C: don't delete — write a tombstone so status/TUI render "finished"
  distinctly from "seed" (preserves history, changes rendering not data).
Pick one (recommend A), record rationale in the running log / ADR.

## Done when
- The finish cycle removes (or tombstones, per the decision) the finished grove's
  `inboxes/<name>/` on grove-meta, so `grove status` / TUI no longer list a
  finished grove as a Seed.
- Non-empty-inbox case handled per the settled decision (no silent data loss).
- grove-meta write follows ADR-0005 sync rules (best-effort push, soft-on-offline).
- CONTEXT.md updated: the **Seed** definition no longer says a *finished* grove's
  inbox is a seed (or defines the tombstone), and **Complete finish cycle** gains
  the cleanup step. ADR-0010 updated (or a new ADR) to document the added step.
- A test asserts a finished grove does not appear as a seed.

## Notes
- Independent of leaves 010/020 (start-side); ordered last only by sequence, no
  dependency. Could be executed before them if preferred.
