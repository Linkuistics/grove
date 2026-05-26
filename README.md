# grove

Hierarchical, self-extending workstream tool for AI agents. See `content/SKILL.md` for the methodology; this README covers the CLI.

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
grove init [<repo>]               # first-time setup: install + CONTEXT.md + docs/adr/
grove version                     # CLI version + installed content version per harness
grove status [<repo>]             # installed versions + per-grove summary in <repo>
grove list [<repo>]               # grove names in <repo>, one per line (scriptable)
```

**Drive a grove workstream:**

```
grove start <name>                # new grove: create worktree + launch harness
grove continue <name>             # resume: pick the next leaf, run the loop
grove takeover <name>             # orient on an unfamiliar grove without picking a task
grove retire <name>/<node-path>   # promote brief upward, mv node into done/
grove finish <name>               # grove is done: merge + cleanup per project convention
```

Each verb takes optional `--harness <name>` (repeatable for file-system verbs) and respects auto-detection from the repo's `.claude/` and `.codex/` directories. Session launchers stamp `groves/<name>/.harness` only when needed for disambiguation in multi-harness repos.

See `grove --help` for flag details.
