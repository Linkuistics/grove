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
grove install [<repo>]            # materialise grove into <repo> (create-only)
grove update  [<repo>]            # refresh an existing install
grove uninstall [<repo>]          # remove grove (refuses if live groves exist; --force to override)
grove version                     # CLI version + installed content version per harness
grove status [<repo>]             # installed versions + per-grove summary in <repo>
grove list [<repo>]               # grove names in <repo>, one per line (scriptable)
```

`grove install` and `grove update` create a single path-scoped git commit covering the installed paths (default messages: `Install grove v<ver>` / `Update grove to v<ver>`). They refuse to proceed if the install-scope paths already have staged changes, but leave unrelated dirty state elsewhere untouched. Pass `--no-commit` to stage and commit yourself, or `--message <text>` (`-m`) to override the message.

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
