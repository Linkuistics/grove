# 050-remove-project-skill-mirrors

**Kind:** work

## Goal

Now that the global skill is live (040), remove the project-local skill mirrors so
no stale project copy can silently shadow the global one, and document the
precedence rule.

## Context

- `.claude/skills/grove/` is a tracked mirror of `content/` in **three** places,
  each a reviewable change on its own branch:
  1. this worktree (`refactor-grove-to-be-an-archon-workflow`),
  2. the main checkout,
  3. the `grove-general-improvements` worktree.
- Claude Code skill precedence: **enterprise > personal > project**. A leftover
  *project* copy silently shadows the *personal* global skill the binary now
  provisions (`~/.claude/skills/grove/`) — hence removal.
- `content/` stays canonical (it is what the binary embeds, 010).

## Done when

- `.claude/skills/grove/` is deleted from this worktree (committed on this branch),
  the main checkout, and the `grove-general-improvements` worktree.
- The precedence note (enterprise > personal > project; `content/` is canonical, the
  binary provisions the personal global skill) is documented so a future
  contributor does not re-add a shadowing mirror — `CONTEXT.md` or a short
  `docs/` note.

## Notes

- **Ordering**: this MUST follow 040 (global skill live). Removing a mirror before
  the global exists leaves a session skill-less (ADR-0034 risk note).
- By this leaf the world is new-format, so it is executed by the **new** binary on
  the **migrated** tree.
- This retires the long-standing "edit `content/` and the `.claude/skills/grove/`
  mirror together" workflow — there is no more mirror to keep in sync.
