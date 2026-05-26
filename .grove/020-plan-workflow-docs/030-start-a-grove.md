# 030-start-a-grove

**Kind:** work

## Goal
Write `docs/workflows/start.md` — the lifecycle walkthrough for `grove start <name>` in `acme/orders-api`, demonstrating worktree creation, branching, and the harness bootstrap session. The running example grove is `add-rate-limiting`.

## Context
- Assumes the install walkthrough is the prior state; the user has `grove` installed and `acme/orders-api` has been materialised.
- `content/SKILL.md` describes the methodology — do not re-explain it here. The walkthrough is about *what the CLI does* when you type `grove start <name>`: branch creation, worktree placement at `.grove-worktrees/<name>/`, session rename to `<repo>: <name> grove`, prompt selection from `prompts/start.md`.
- `docs/grove.md` "Driving a grove" already names the worktree path convention; cite it once, don't repeat the rationale.

## Done when
- `docs/workflows/start.md` exists.
- Walks at minimum: starting state (orders-api on default branch, no `.grove-worktrees/`), the `grove start add-rate-limiting` command, what's created on disk (`tree -L 2 .grove-worktrees/`), the new branch (`git branch -a` subset), and a sentence on the harness session that was just exec'd.
- Shows the resulting `.grove/BRIEF.md` shape that the bootstrap session is expected to write (one or two leaves, terse) — link to `BRIEF-FORMAT.md`, do not re-author it.
- Covers two variations as short subsections:
  - `--start-point <ref>` (branch from somewhere other than origin's default HEAD).
  - `--no-launch` (set up the worktree but skip the harness exec — useful for inspection or scripting).
- Brief note on the multi-harness case: when both `.claude/` and `.codex/` are present, `--harness` selects, and the `.grove-stamps/<name>` one-liner binds the grove to its harness.
- Short Codex-equivalent callout: the harness command is different but the worktree shape is identical.

## Notes
- The bootstrap session itself produces a *separate* commit (the first commit on the grove branch) — make sure the walkthrough doesn't conflate "what `grove start` did" (worktree + branch) with "what the bootstrap session does after exec" (writes `BRIEF.md`, possibly `CONTEXT.md`).
- If the running example grove name needs to be plausible, `add-rate-limiting` was chosen in the planning brief (`BRIEF.md`). Stick with it across `030`/`040`/`050` for continuity.
