# grove

Hierarchical, self-extending workstream tool for AI agents. See [`docs/grove.md`](docs/grove.md) for what grove is and why, [`content/SKILL.md`](content/SKILL.md) for the methodology agents read at runtime; this README covers the CLI.

## Install

```
brew tap Linkuistics/taps
brew install grove
```

That is the whole installation. The `grove` binary embeds its full methodology and provisions it to your **personal** skill dir (`~/.claude/skills/grove/`) on the first `grove do` — idempotent against a content-hash stamp, re-provisioned whenever the binary changes (ADR-0031/0034). There is no per-repo install step and nothing to keep in sync; `grove --version` reports the binary's version.

## Use

**Drive a grove workstream:**

```
grove do <name>                   # the sole lifecycle entry verb: start a new grove or continue an existing one
grove retire <name>/<node-path>   # promote a finished node's brief upward (its leaves stay marked done in place)
```

`grove do` is the **sole lifecycle entry verb**: it inspects the grove's state and dispatches — no grove by that name → create the worktree and open a bootstrap session; live worktree → continue; branch exists but worktree gone → re-attach and continue. The former `grove start` and `grove continue` are removed (`do` already covered both); on a brand-new grove `do` accepts `--start-point <ref>` to branch from somewhere other than origin's HEAD. The former `grove finish` is also removed: a grove is now finished **in-session** — when it has no live leaves left, the running loop proposes the complete finish cycle (delete `.grove/`, merge to the default branch, delete the branch and worktree). See the methodology's *Finish* step.

Each verb takes optional `--harness <name>` and respects auto-detection from the repo's `.claude/` and `.codex/` directories. Session launchers stamp `.grove-stamps/<name>` only when needed for disambiguation in multi-harness repos. Worktrees live at `.grove-worktrees/<name>/`; the task tree itself is `.grove/` inside that worktree.

See `grove --help` for flag details. For end-to-end walkthroughs of each verb in context, see [`docs/workflows/`](docs/workflows/).
