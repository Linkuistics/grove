# grove

Hierarchical, self-extending workstream tool for AI agents. See [`docs/grove.md`](docs/grove.md) for what grove is and why, [`content/SKILL.md`](content/SKILL.md) for the methodology agents read at runtime; this README covers the CLI.

## Install

```
brew tap Linkuistics/taps
brew install grove
```

That is the whole installation. The `grove` binary embeds its full methodology and provisions it to your **personal** skill dir (`~/.claude/skills/grove/`) on the first `grove do` — idempotent against a content-hash stamp, re-provisioned whenever the binary changes (self-extension-core-and-methodology / task-tree-scheme). There is no per-repo install step and nothing to keep in sync; `grove --version` reports the binary's version.

## Prerequisite

grove's ADR guidance defers to the **`linkuistics:decision-records`** skill (from the `linkuistics` plugin) for the decision-record philosophy, format, and when-to-write test — grove's bundled `ADR-FORMAT.md` keeps only grove's own placement conventions. A grove session that raises or reworks an ADR consults that skill, so install the `linkuistics` plugin alongside grove. The dependency is documentation-level (grove does not enforce it at install time); everything else grove needs is embedded in the binary.

## Use

**Drive a grove workstream:**

```
grove do <name>                   # the sole lifecycle entry verb: start a new grove or continue an existing one
grove retire <name>/<node-path>   # promote a finished node's brief upward (its leaves stay marked done in place)
```

`grove do` is the **sole lifecycle entry verb**: it inspects the grove's state and dispatches — no grove by that name → create the worktree and open a bootstrap session; live worktree → continue; branch exists but worktree gone → re-attach and continue. The former `grove start` and `grove continue` are removed (`do` already covered both); on a brand-new grove `do` accepts `--start-point <ref>` to branch from somewhere other than origin's HEAD. The former `grove finish` is also removed: a grove is now finished **in-session** — when it has no live leaves left, the running loop proposes the complete finish cycle (delete `.grove/`, merge to the default branch, delete the branch and worktree). See the methodology's *Finish* step.

Each verb takes optional `--harness <name>` and respects auto-detection from the repo's `.claude/` and `.codex/` directories. Session launchers stamp `.grove-stamps/<name>` only when needed for disambiguation in multi-harness repos. Worktrees live at `.grove-worktrees/<name>/`; the task tree itself is `.grove/` inside that worktree.

See `grove --help` for flag details. For end-to-end walkthroughs of each verb in context, see [`docs/workflows/`](docs/workflows/).

## Configuration

`grove do` picks each task session's model by the **kind** of the leaf it's
about to launch, so planning can run on a stronger reasoning model and
mechanical work on a cheaper/faster one:

```
GROVE_PLANNING_MODEL=opus GROVE_WORK_MODEL=sonnet grove do <name>
```

| Variable | Applies to |
|---|---|
| `GROVE_PLANNING_MODEL` | planning leaves (grilling, design) |
| `GROVE_WORK_MODEL` | work leaves (code, docs, tests) |

The loop passes the matching value via Claude Code's native `--model` at each launch. Two things to know:

- **Unset ⇒ inherit your own default.** Leave a variable unset and grove passes no `--model` for that kind — the session runs on your own default (`ANTHROPIC_MODEL` or your Claude Code settings). So grove is a no-op until you opt in, and it never clobbers a default you already have; setting only one variable is fine (the other kind still inherits).
- **The launch model is a default, not a lock.** An in-session `/model` switch overrides it (native Claude Code, higher priority than `--model`) but **does not persist across relaunch** — each task is a fresh session the loop launches on its kind's default, so a mid-session `/model` change applies to that one session only.

See [`docs/adr/model-per-task-kind.md`](docs/adr/model-per-task-kind.md) for the rationale (why native `--model` rather than a router/proxy).
