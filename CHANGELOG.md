# Changelog

## Unreleased

### Breaking

- **`grove start`, `grove continue`, and `grove finish` removed; `grove do` is the sole lifecycle entry verb.** `grove do` already subsumed start/continue (no grove by that name → create the worktree and open a bootstrap session; live worktree → continue; branch present but worktree gone → re-attach and continue), so both were strictly redundant (ADR-0009). `grove finish` is removed too: finishing a grove is now an **in-session** step — when the grove has no live leaves left, the running loop proposes the complete finish cycle (delete `.grove/`, merge to the default branch, delete the branch and worktree; the step-level design lands in a later grove leaf). Migration: replace `grove start <name>` / `grove continue <name>` / `grove finish <name>` with `grove do <name>`. The `--start-point <ref>` flag, formerly on `grove start`, now lives on `grove do` and applies on the new-grove path. Trade-off: there is no longer a way to force-finish a grove that still has live leaves — retire or clear the leaves first.

## v4.0.0

### Breaking

- **`grove list` removed.** Its output (grove names, one per line) is a subset of `grove status`, now the canonical visibility surface (ADR-0007). Migration: parse `grove status` instead of `grove list`.
- **`grove version` removed.** Its output (CLI version + per-harness installed version) is subsumed by `grove status`. Migration: use `grove --version` for the CLI version alone, or `grove status` for the full cli/repo/worktree picture.
- **`grove update` removed; `grove install` is now idempotent** (ADR-0008). One verb converges on the bundled version from any starting state: not installed → install; same version → no-op (no empty commit); different version → update. It always prints a per-harness outcome line — `installed @ X`, `already at X, no change`, or `updated X → Y` — making the result explicit and safe to rely on in CI/setup scripts. There is no `--update` / `--force` flag and no deprecated `grove update` alias. Migration: replace `grove update` with `grove install` (add `--version <tag>` to pin). The default commit subject is still `Install grove v<ver>` for a fresh install and `Update grove to v<ver>` when refreshing an existing one. The stored `VERSION.md` stamp is now canonical (no leading `v`); the git fetch ref is unchanged.

## v2.2.0

- `**Retire.**` doctrine in `content/SKILL.md` is now imperative and procedural: after committing a task, the session mvs the just-finished leaf into `.grove/done/` (mechanical, no ask), then walks the parent chain. If a node has no live leaves left, the session **asks the user** before retiring it — the confirmation gives them a moment to add a follow-up leaf — then promotes any still-relevant brief content upward and `mv`s the node into `.grove/done/`. The cascade recurses through ancestors until a node still has live leaves or the grove root is reached. The inner-loop mermaid graph and the `multi-step.md` walkthrough are updated to match.

## v2.1.0

- `grove install` and `grove update` now produce a single path-scoped git commit covering every targeted harness path (per ADR-0001). `--no-commit` opts out and prints the staging command; `-m`/`--message` overrides the default message. Pre-flight refuses if install-scope paths already have staged hunks; unrelated dirty state elsewhere is left alone. Hook failures leave the materialisation in place and print a follow-up `git commit -- <paths>`. Multi-harness invocations produce one combined commit; no-op materialisations skip the commit.
- New per-flow walkthroughs under `docs/workflows/` (install, update, start, multi-step, finish) with an index; README and `docs/grove.md` cross-link to them.
- Documentation clarifies that `grove continue` is a session launcher and notes that up-arrow history recall surfaces the last continue prompt.

## v2.0.0

Breaking on-disk layout change. Every storage location is now dot-prefixed and the per-grove namespace is gone where it was redundant.

- Task tree: `groves/<name>/` → `.grove/` (inside the grove's worktree). One worktree = one grove, so the name no longer needs to namespace the task tree.
- Worktree: `worktrees/<name>-grove/` on branch `<name>-grove` → `.grove-worktrees/<name>/` on branch `<name>`.
- Harness stamp: `groves/<name>/.harness` → `.grove-stamps/<name>`.
- `grove finish` now explicitly deletes `.grove/` in a focused commit before merging, so the default branch never carries any grove's local state. The history of completed groves lives in git's commit graph, not in retained `done/` directories.
- `grove uninstall`'s "live groves" check is now "any worktree exists in `.grove-worktrees/`" — simpler and authoritative.

Migration: existing groves on v1.x layout need manual relocation (`mv groves/<name> .grove-worktrees/<name>-grove/.grove`, then rebranch, then refresh content with `grove update`). New repos pick up the new layout automatically.

## v1.0.1

- Relicense from MIT to Apache-2.0 (matches sibling Linkuistics projects); add the missing LICENSE file at repo root.
- Add `docs/grove.md` — project-level intro covering the methodology rationale and the CLI's workstream verbs.

## v1.0.0

- Initial public release of the grove CLI.
- Lifecycle verbs: `install`, `update`, `uninstall`, `version`, `status`, `list`.
- Launcher verbs: `start`, `continue`, `takeover`, `retire`, `finish`.
- Multi-harness support with auto-detection of `.claude/` and `.codex/`; `.harness` stamp used as a per-grove disambiguator.
- Release pipeline producing macOS arm64 and Linux x86_64/arm64 binaries.
