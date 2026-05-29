# grove

Hierarchical, self-extending workstream tool for AI agents. See [`docs/grove.md`](docs/grove.md) for what grove is and why, [`content/SKILL.md`](content/SKILL.md) for the methodology agents read at runtime; this README covers the CLI.

## Install

```
brew tap Linkuistics/taps
brew install grove
```

## Use

**Manage the materialised skill in a repo:**

```
grove install [<repo>]            # materialise grove into <repo> (idempotent: install / update / no-op)
grove uninstall [<repo>]          # remove grove (refuses if live groves exist; --force to override)
grove status [<repo>]             # cli/repo/worktree versions, drift, and per-grove summary in <repo>
```

`grove status` is the canonical visibility surface — it shows the CLI version, each harness's installed (`repo`) version, and every grove worktree's materialised version with drift markers. It supersedes the former `grove list` and `grove version`, both removed in v4.0.0; for the CLI version alone use `grove --version`.

`grove install` is **idempotent** (ADR-0008): not installed → install; same version → no-op; different version → update. It prints a per-harness outcome line (`installed @ X`, `already at X, no change`, or `updated X → Y`) so the result is always explicit — safe to rely on in CI and setup scripts. The former `grove update` is removed; re-run `grove install` to refresh. It creates a single path-scoped git commit covering the installed paths (default subject `Install grove v<ver>` for a fresh materialisation, `Update grove to v<ver>` when refreshing an existing one). It refuses to proceed if the install-scope paths already have staged changes, but leaves unrelated dirty state elsewhere untouched. Pass `--no-commit` to stage and commit yourself, or `--message <text>` (`-m`) to override the message.

**Drive a grove workstream:**

```
grove start <name>                # new grove: create worktree + launch harness
grove continue <name>             # resume: pick the next leaf, run the loop
grove do <name>                   # start or continue — use when you don't remember which
grove takeover <name>             # orient on an unfamiliar grove without picking a task
grove retire <name>/<node-path>   # promote brief upward, mv node into done/
grove finish <name>               # grove is done: merge + cleanup per project convention
```

Each verb takes optional `--harness <name>` (repeatable for file-system verbs) and respects auto-detection from the repo's `.claude/` and `.codex/` directories. Session launchers stamp `.grove-stamps/<name>` only when needed for disambiguation in multi-harness repos. Worktrees live at `.grove-worktrees/<name>/`; the task tree itself is `.grove/` inside that worktree.

See `grove --help` for flag details. For end-to-end walkthroughs of each verb in context, see [`docs/workflows/`](docs/workflows/).
