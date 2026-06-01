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
grove do <name>                   # the sole lifecycle entry verb: start a new grove or continue an existing one
grove takeover <name>             # orient on an unfamiliar grove without picking a task
grove retire <name>/<node-path>   # promote brief upward, mv node into done/
```

`grove do` is the **sole lifecycle entry verb**: it inspects the grove's state and dispatches — no grove by that name → create the worktree and open a bootstrap session; live worktree → continue; branch exists but worktree gone → re-attach and continue. The former `grove start` and `grove continue` are removed (`do` already covered both); on a brand-new grove `do` accepts `--start-point <ref>` to branch from somewhere other than origin's HEAD. The former `grove finish` is also removed: a grove is now finished **in-session** — when it has no live leaves left, the running loop proposes the complete finish cycle (delete `.grove/`, merge to the default branch, delete the branch and worktree). See the methodology's *Finish* step.

Each verb takes optional `--harness <name>` (repeatable for file-system verbs) and respects auto-detection from the repo's `.claude/` and `.codex/` directories. Session launchers stamp `.grove-stamps/<name>` only when needed for disambiguation in multi-harness repos. Worktrees live at `.grove-worktrees/<name>/`; the task tree itself is `.grove/` inside that worktree.

**Browse a repo's groves in the dashboard:**

```
grove tui [<repo>]                # launch the interactive dashboard
```

`grove tui` opens grove's dashboard — a master/detail navigator over the repo's groves with inbox capture. It runs on a grove-owned [zellij](https://zellij.dev) substrate (ADR-0015/0016): grove launches zellij with its own bundled config and a chrome-free layout, so the composite presents as a single binary, not "a zellij session". zellij runs in **locked mode**, passing every key straight through to the focused app; the one control seam is **`Ctrl-o`**, which toggles zellij's own controls on and off (remapped from zellij's default `Ctrl-g`). An installed `zellij` is required. Quitting zellij returns to your shell.

See `grove --help` for flag details. For end-to-end walkthroughs of each verb in context, see [`docs/workflows/`](docs/workflows/).
