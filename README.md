# grove

Hierarchical, self-extending workstream tool for AI agents. See [`docs/grove.md`](docs/grove.md) for what grove is and why, [`content/SKILL.md`](content/SKILL.md) for the methodology agents read at runtime; this README covers the CLI.

## What's in this repo

Two components, shipped by two paths. They live together because they change in lockstep — see [`docs/adr/skills-monorepo.md`](docs/adr/skills-monorepo.md); the skill plugins were formerly the separate `Linkuistics/skills` repo, grafted in with its history.

| | What it is | How you install it |
|---|---|---|
| **grove** — [`src/`](src/), [`content/`](content/) | the CLI plus the workstream methodology it embeds and provisions to your agent | `brew install grove` — see [Install grove](#install-grove) below |
| **skill plugins** — [`plugins/`](plugins/) | `linkuistics` (coding standards) and `testanyware` (GUI testing in VMs) | Claude Code marketplace, or [`install.sh`](install.sh) for codex/gemini — see [`plugins/README.md`](plugins/README.md) |

**Which do you want?** If you are here to drive long multi-session workstreams, you want grove — keep reading. If you are here for the coding-style, design, ADR, jj or GUI-testing skills, you want the plugins: go to [`plugins/README.md`](plugins/README.md), which lists every skill and what triggers it. Installing one does not install the other; the `grove` binary provisions grove's own methodology and nothing else.

The repo's two bounded contexts and how they relate are mapped in [`CONTEXT-MAP.md`](CONTEXT-MAP.md), over [`CONTEXT.md`](CONTEXT.md) (grove) and [`plugins/CONTEXT.md`](plugins/CONTEXT.md) (skills).

## Install grove

(For the skill plugins instead, see [`plugins/README.md`](plugins/README.md).)

```
brew tap Linkuistics/taps
brew install grove
```

That is the whole installation of grove itself. The `grove` binary embeds its full methodology and provisions it to your **personal** skill dir (`~/.claude/skills/grove/`) on the first `grove do` — idempotent against a content-hash stamp, re-provisioned whenever the binary changes (self-extension-core-and-methodology / task-tree-scheme). There is no per-repo install step and nothing to keep in sync; `grove --version` reports the binary's version.

## Prerequisite

grove's methodology defers two bodies of guidance to the **`linkuistics` plugin**: decision-record philosophy, format and the when-to-write test to `linkuistics:decision-records`, and test-seam judgement to `linkuistics:codebase-design`. Grove's bundled `ADR-FORMAT.md` and `SPEC-FORMAT.md` keep only grove's own placement and recording conventions, so a session raising an ADR or sketching a spec's seams consults those skills.

Install the plugin alongside grove. It is hosted in this repo under [`plugins/linkuistics/`](plugins/linkuistics/) but is **still a separate install** — the `grove` binary provisions grove's methodology and nothing else. See [`plugins/README.md`](plugins/README.md) for the two commands. The dependency is documentation-level (grove does not enforce it at install time); everything else grove needs is embedded in the binary.

## Use

**Drive a grove workstream:**

```
grove do                   # the sole lifecycle entry verb, run from inside your working tree
grove retire <node-path>   # promote a finished node's brief upward (its leaves stay marked done in place)
```

`grove do` is **argument-less** and run from inside a working tree you provide — git or jj-enabled — and grove never creates, integrates, or tears down working trees, branches, or bookmarks, and reads no branch and no bookmark anywhere (VCS topology is entirely yours: plain git/gh or jj, or a dedicated worktree manager such as [worktrunk](https://github.com/max-sixty/worktrunk)). It is the **sole lifecycle entry verb**: it inspects the grove's state on disk and dispatches — no `.grove/` yet → open a bootstrap session; a live tree → continue. The former `grove start` and `grove continue` are removed (`do` already covered both). The former `grove finish` is also removed: a grove is now finished **in-session** — when it has no live leaves left, the running loop proposes the complete finish cycle (promote durable content out of the briefs, delete `.grove/` in one commit, signal the loop to stop). See the methodology's *Finish* step.

Each verb takes optional `--harness <name>` and respects auto-detection from the repo's harness directories (`.claude/`, `.codex/`, `.pi/`). Session launchers stamp `.grove-stamps/<name>` (keyed by the working tree's basename) whenever `--harness` is passed explicitly, and also when needed for disambiguation in multi-harness repos — a single-harness repo relying on auto-detection stays stamp-free. The grove's name is the working tree's own directory basename; the task tree itself is `.grove/` inside it.

See `grove --help` for flag details. For end-to-end walkthroughs of each verb in context, see [`docs/workflows/`](docs/workflows/).

## Configuration

`grove do` picks each task session's model by the **kind** of the leaf it's
about to launch, so planning can run on a stronger reasoning model and
mechanical work on a cheaper/faster one:

```
GROVE_PLANNING_MODEL=opus GROVE_WORK_MODEL=sonnet grove do
```

| Variable | Applies to |
|---|---|
| `GROVE_PLANNING_MODEL` | planning leaves (grilling, design) |
| `GROVE_RESEARCH_MODEL` | research leaves (citation-disciplined surveys) |
| `GROVE_PROTOTYPE_MODEL` | prototype leaves (a cheap throwaway artifact) |
| `GROVE_WORK_MODEL` | work leaves (code, docs, tests) |
| `GROVE_REVIEW_MODEL` | review leaves (fresh-context adversarial read) |

The loop passes the matching value via Claude Code's native `--model` at each launch. Two things to know:

- **Unset ⇒ inherit your own default.** Leave a variable unset and grove passes no `--model` for that kind — the session runs on your own default (`ANTHROPIC_MODEL` or your Claude Code settings). So grove is a no-op until you opt in, and it never clobbers a default you already have; setting only some of the five is fine (an unconfigured kind still inherits).
- **The launch model is a default, not a lock.** An in-session `/model` switch outranks `--model` for that one session (native Claude Code). Whether it persists into the *next* task depends on the switched kind's env var: **set** ⇒ the driver passes `--model` again on the next launch and the override does not persist; **unset** ⇒ interactive `/model` saves as your own default, so the override *does* persist into every subsequent unconfigured session.

See [`docs/adr/model-per-task-kind.md`](docs/adr/model-per-task-kind.md) for the rationale (why native `--model` rather than a router/proxy).
