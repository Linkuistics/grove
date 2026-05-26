# 020-plan-workflow-docs — brief

## Goal
Write the five **lifecycle walkthroughs** (`install`, `update`, `start`, `multi-step`, `finish`) plus an index, landing them under `docs/workflows/`. See `CONTEXT.md` for the term.

## Done when
- `docs/workflows/` exists with five per-verb walkthroughs and `docs/workflows/README.md` as the index.
- Each walkthrough shows every command, explains what happens, and shows what changed in the repo/worktree using `tree` (subset) and/or `git status` / `git log` snippets in fenced blocks.
- README.md and `docs/grove.md` carry a short pointer into `docs/workflows/`.
- No duplication of the methodology already in `content/SKILL.md` — walkthroughs show *how the CLI drives the methodology*, not what the methodology is.

## Decomposition
Numeric prefix encodes the natural reading order and the dependency order between the walkthroughs themselves (update assumes install ran; multi-step assumes start ran; finish closes the loop). The index is last so it can link real files instead of phantom paths.

- `010-install.md` — write `docs/workflows/install.md`.
- `020-update.md` — write `docs/workflows/update.md`.
- `030-start-a-grove.md` — write `docs/workflows/start.md`.
- `040-multi-step-grove.md` — write `docs/workflows/multi-step.md`.
- `050-finish.md` — write `docs/workflows/finish.md`.
- `060-workflows-index.md` — write `docs/workflows/README.md` and add the cross-link from `README.md` and `docs/grove.md` into `docs/workflows/`.

## Pointers
- ADRs to read: `docs/adr/0001-install-and-update-create-commits.md` (auto-commit behavior demonstrated by `010-install` and `020-update`).
- Glossary terms in play: **lifecycle walkthrough**, **install scope**, **path-scoped commit** (see `CONTEXT.md`).
- Existing surfaces to align with — do not duplicate:
  - `README.md` — the CLI surface (flags).
  - `docs/grove.md` — what grove is and why.
  - `content/SKILL.md` — the methodology agents read at runtime.

## Style decisions (from grilling on 2026-05-27)
These apply across all six child leaves; restate only deviations.

- **Style:** hybrid. Each step is a short paragraph of intent, then a fenced command block, then *optionally* a small fenced "what changed" block. Use the panel only where the change is the point of the step.
- **Evidence format:** mix `tree -L 2 <path>` (subset, only the relevant subtree), `git status`, and `git log --oneline -N` in fenced text blocks. No mermaid, no before/after ASCII trees.
- **Harness coverage:** Claude Code primary. Use `.claude/skills/grove/` paths in the main text. Where Codex differs, add a short callout block (path is `.codex/skills/grove/`, harness command differs, etc.). Do not write parallel Codex sections.
- **Demo target:** one fictional throwaway repo, `acme/orders-api`, used across every walkthrough. The running example grove for `start` / `multi-step` / `finish` is `add-rate-limiting`. Pinning these here prevents per-leaf invention drift.
- **Verification:** prose only. No executable walkthrough harness for now. If output drift becomes a real problem after a couple of CLI releases, revisit by adding a sibling leaf for a snapshot test scaffold — but do not pre-build it.
- **What's *not* in scope:** uninstall, status, list, version, takeover, retire — those are CLI commands but not part of the five flows the root brief enumerates.

## Notes
- Each child leaf is a **work task**, not a planning task. The shape is fixed by these decisions; the writing is the work.
- If a writing leaf discovers a missing CLI behavior or a contradicting prompt, do not paper over it in prose — surface it (raise an ADR or a sibling work task) before writing around it.
- README and `docs/grove.md` cross-links land in `060-workflows-index.md`, not in earlier leaves, so the planning-task commit stays scoped to tree growth.
