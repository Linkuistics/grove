# 060-backwards-compat-migration

**Kind:** work

## Goal

Let the new grove keep working on existing trees during the transition, and convert
them once: a **transitional dual-format reader** (old `NNN-slug/` directory format
+ new flat dotted-decimal) plus a one-time **`grove migrate`** verb (old → new).
Then old-format reading is **dropped** (ADR-0031, 030 D5/option ii).

## Context

Read **030 D5** (the transitional-bridge decision) and **050** (the new target
format) first. Rationale for *transitional*, not permanent, dual-format: grove
trees are ephemeral (each finishes and deletes its `.grove/`), so the old-format
population drains on its own — permanent dual-read would be a forever-tax. This
grove is itself an old `NNN-slug/` dogfood tree, so the bridge must keep *it*
working while the refactor proceeds.

## Done when

- `grove-llm pick` + the grow verbs read **both** formats (detect per-tree which
  format is in play; no flag).
- `grove migrate` converts an old `NNN-slug/` tree to the new flat dotted-decimal
  format in place (a reviewable git change), preserving order, done-ness, and
  brief-chain structure.
- A deprecation path is recorded for **dropping** old-format reading once trees
  convert (so the dual-read code does not become permanent).
- Verified against a **fixture** old-format tree — **not** this grove (per the
  rollout decision in the root BRIEF, this grove finishes on the old scheme).

## Notes

- Depends on 050 (the target scheme). Sequence after 050.
- Keep the reader's format-discrimination logic in `grove-llm` (and described in
  the skill prose), per the spike's distribution finding — nothing in the skill
  model prevents one skill reading two on-disk formats.
- **Do not migrate this grove.** Per the rollout decision (root BRIEF), this
  workstream runs to completion on the *old* installed grove; the new scheme ships
  in a new major release installed via homebrew. `grove migrate` + the dual-format
  reader are therefore built and verified against a fixture, and only exercised on
  real old trees (the user's other groves) *post-release*.
