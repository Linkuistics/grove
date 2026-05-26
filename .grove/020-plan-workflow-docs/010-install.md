# 010-install

**Kind:** work

## Goal
Write `docs/workflows/install.md` — the lifecycle walkthrough for `grove install` against the demo repo `acme/orders-api` from scratch, demonstrating the new path-scoped auto-commit behavior (ADR-0001).

## Context
- ADR-0001 specifies the auto-commit behavior this walkthrough must demonstrate (default-on commit, `--no-commit`, `--message`, pre-existing-staged-hunks refusal, no empty commit on no-op, fail-loud on hook failure).
- The previous-leaf implementation (`done/010-implement-path-scoped-commit.md`) shipped this; the actual CLI is what to drive.
- `README.md`'s "Use" section already paraphrases the auto-commit. The walkthrough goes deeper: the actual session a user runs.

## Done when
- `docs/workflows/install.md` exists.
- Walks at minimum: the bare `grove install` in `acme/orders-api`, the resulting commit (shown via `git log --oneline -1` + `git show --stat HEAD`), the on-disk layout (subset `tree .claude/skills/grove/`), and a brief note that `grove install --help` enumerates the flags.
- Covers each of these variations as short subsections, each with the command + what changes (or doesn't):
  - `--no-commit` (and the printed follow-up `git add … && git commit …`).
  - `--message "…"` overriding the default.
  - Re-running `grove install` on an already-installed repo (should fail or be a no-op — verify against the actual CLI before writing the prose).
  - Pre-existing staged hunks on install-scope paths (the refusal message).
- Short Codex-equivalent callout where `.claude/skills/grove/` becomes `.codex/skills/grove/`.
- No duplication of the methodology (`content/SKILL.md`) — this is about the CLI, not about what gets written.

## Notes
- Demo repo: `acme/orders-api`. Walkthrough should open by showing a clean `git status` in that repo, then run `grove install`.
- Output examples must reflect what the CLI actually prints. Run the commands in a scratch repo (or read `src/install.rs`) before pasting — do not invent stdout.
- The "what changed" panels should be selective: show the new `.claude/skills/grove/` subtree once (the first install step), then for later steps prefer `git log --oneline` / `git status` over re-showing the tree.
