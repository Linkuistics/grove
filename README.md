# skills

Antony Blakey's coding standards, packaged as agent **skills** so they load
lazily — only when relevant to the file or task at hand — across Claude Code,
Codex, and other agents that support the [`SKILL.md`](https://agentskills.io)
open standard.

This repository is two things at once:

- a **Claude Code plugin marketplace** (`.claude-plugin/marketplace.json`), and
- the **canonical source** for the skill directories under
  `plugins/linkuistics/skills/`.

## What's here

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
| `grove` | by description; materialised per repo | methodology for long, multi-session workstreams — see `skills/grove/SKILL.md` |

Each skill's one-line `description` is the only standing context cost; the body
loads on demand. In Claude Code the `paths:` frontmatter makes language skills
auto-load deterministically by file type. Other harnesses ignore `paths:` and
fall back to the `description`.

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

## grove — materialised, not installed

`grove` is a workstream methodology. Unlike the coding-style skills, a project
consuming it for serious work does not install the plugin — it **materialises**
grove into its own repo so the methodology version is pinned by the project's
own git history:

    scripts/materialise-grove.sh <path-to-consuming-repo> [<ref>]

This copies `grove/` into the consuming repo's `.claude/skills/grove/` and
writes a `VERSION.md` provenance stamp. Updating is the same command again.

## Editing a skill

Edit the `SKILL.md` under `plugins/linkuistics/skills/<name>/` and commit.
Keep `description` sharp (key use case first) and the body concise — an invoked
skill stays in context for the rest of the session.
