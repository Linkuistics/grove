# skills

This repository ships two things: **grove** — a methodology skill for long, multi-session workstreams — and a suite of **coding-style skills** for common languages and tools. Both are packaged as agent skills for Claude Code, Codex, and other agents that support the [`SKILL.md`](https://agentskills.io) open standard, via a Claude Code plugin marketplace (`.claude-plugin/marketplace.json`) and the skill source under `plugins/linkuistics/skills/`.

## grove

LLM-driven work that spans many sessions and many months can't be planned exhaustively upfront — some steps are themselves planning steps whose output is more steps. Each session starts fresh with no memory of prior sessions, so without a forcing function the project's vocabulary drifts: session 1 coins a term; session 7 reinvents it under a different name. grove solves both: a git-tracked tree of task files (one task per session) that grows lazily as understanding deepens, anchored by a living glossary read every session. It builds on Matt Pocock's `grill-with-docs` (bundled) and Domain-Driven Design's Ubiquitous Language and bounded-context partitioning — neither invented here. See [docs/grove.md](docs/grove.md) for the full problem/solution treatment.

grove is consumed by **materialisation**, not plugin installation — each project pins its own grove version by committing the files into its git history. To materialise grove into a target repo:

```
scripts/materialise-grove.sh <target-repo> [<ref>]
```

See [docs/grove.md](docs/grove.md) for install, update, and usage prompts.

## Coding-style skills

Antony Blakey's coding standards, packaged as agent **skills** so they load lazily — only when relevant to the file or task at hand — across Claude Code, Codex, and other agents that support the [`SKILL.md`](https://agentskills.io) open standard.

| Skill | Loads when | Notes |
|-------|-----------|-------|
| `coding-style` | any file (`paths: "**/*"`) | universal principles — TDD, naming, simplicity |
| `coding-style-rust` | `*.rs` | extends `coding-style` |
| `coding-style-python` | `*.py` | |
| `coding-style-elixir` | `*.ex`, `*.exs` | |
| `coding-style-bash` | `*.sh`, `*.bash` | |
| `coding-style-swift` | `*.swift` | |
| `coding-style-typescript` | `*.ts`, `*.tsx` | |
| `cli-tool-design` | by description | checklist in `SKILL.md`, audit detail in `references/` |

Each skill's one-line `description` is the only standing context cost; the body loads on demand. In Claude Code the `paths:` frontmatter makes language skills auto-load deterministically by file type. Other harnesses ignore `paths:` and fall back to the `description`.

## Install — Claude Code

```
/plugin marketplace add Linkuistics/skills
/plugin install linkuistics@linkuistics
```

Enable auto-update for the marketplace (`/plugin` → Marketplaces → Enable
auto-update) so every Claude Code startup pulls the latest skills.

## Install — Codex, Gemini CLI, other SKILL.md harnesses

```
git clone https://github.com/Linkuistics/skills.git
cd skills
./install.sh
```

`install.sh` symlinks each skill directory into `~/.codex/skills/`,
`~/.gemini/skills/`, etc. (only for harnesses that are installed). Update with
`git pull` — the symlinks mean the content refreshes in place.

## Updating / versioning

The plugin uses **commit-SHA versioning**: `plugin.json` deliberately has no
`version` field, so Claude Code treats every new commit as an update. Push a
change and consumers with auto-update enabled pick it up on next startup; no
version bump required.

If you later want controlled releases instead, add a `version` field to
`plugin.json` and bump it per [semver](https://semver.org) — Claude Code will
then only ship updates when that field changes.

## Editing a skill

Edit the `SKILL.md` under `plugins/linkuistics/skills/<name>/` and commit.
Keep `description` sharp (key use case first) and the body concise — an invoked
skill stays in context for the rest of the session.
