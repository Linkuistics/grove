# 020-docs-start-flow

**Kind:** work

## Goal
Make the brand-new-grove start legible in prose: rewrite the content-free
`start.md` launcher prompt to name the actual procedure, and add a "Starting a
new grove" section to the methodology so the loop no longer assumes `.grove/`
already exists. After this leaf, an LLM dropped into a fresh grove has an
explicit, do-able path and never improvises scaffolding.

## Context
- Depends on leaf 010: `grove-llm root-init` and its exact behavior/`--help`
  must already exist so the docs describe shipped reality, not a plan.
- `content/prompts/start.md` — today its entire content is "Start a new grove —
  use the grove skill's start-a-new-grove flow", a pointer to a flow that does
  not exist. This is the primary confusion (root brief evidence item 1).
- `content/SKILL.md` — documents the loop (Pick → Bootstrap → … → Finish), every
  step of which assumes `.grove/` exists. Needs an explicit fresh-grove entry
  point that hands off into the normal loop.
- Memory/feedback constraint: launcher prompts stay small — `content/prompts/*`
  should refer to files, not inline large content. Keep `start.md` a thin,
  concrete pointer (name the verb + the immediate next step), not a tutorial.
- ADR-0011 (from leaf 010) — cross-link it from the SKILL.md section.

## Done when
- `content/prompts/start.md` names the procedure concretely: run `grove-llm
  root-init` (scaffolds root brief + first planning leaf), then enter the normal
  loop (`pick` → grill the planning leaf → grow the tree). Stays small.
- `content/SKILL.md` has a short "Starting a new grove" passage so the empty-
  `.grove/` case is no longer an undocumented gap; it explains the chicken-and-
  egg (`pick` errors before a root exists) and points at `root-init` as the
  resolution, then hands off to the existing loop.
- The fix is verified against the methodology as materialised into a worktree's
  skill (the prompts/SKILL the LLM actually reads), not only the `content/`
  source — confirm they stay in sync via the normal install/materialise path.
- No dangling references to a "start-a-new-grove flow" that has no home.

## Notes
- This leaf closes the loop on the irony that produced this grove: the session
  that wrote these docs is the one that hit the confusion. Quoting the
  reproduced failure (brief evidence items 1–4) keeps the rationale concrete.
- Glossary entry for the verb lands in leaf 010; here just reference it.
