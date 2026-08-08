# grove

Grove is a hierarchical, self-extending workstream tool for AI coding agents.
It keeps a long project in a small, version-controlled task tree and launches
one fresh, appropriately configured agent session at a time. The repository
also contains a separately installed collection of agent skills.

## What's in this repository

| Product | Source | Purpose |
|---|---|---|
| Grove | [`src/`](src/), [`content/`](content/) | Rust CLI plus the methodology embedded in it. |
| Skill plugins | [`plugins/`](plugins/) | Linkuistics coding/design skills and the Testanyware GUI-testing skill. |

The products share a repository but have separate installation paths. Grove
provisions only its own embedded methodology; it does not install the skill
plugins.

## Install Grove

```sh
brew tap Linkuistics/taps
brew install grove
```

There is no per-project installation step. The first `grove do` provisions the
embedded Grove methodology to the selected agent harness's personal skill
directory. `grove --version` reports the installed binary version.

Grove's methodology uses two Linkuistics skills: `decision-records` for ADR
discipline and `codebase-design` for testable module seams. Install the
Linkuistics plugin separately using the instructions below.

## Install the skill plugins

For Claude Code:

```text
/plugin marketplace add Linkuistics/grove
/plugin install linkuistics@linkuistics
/plugin install testanyware@linkuistics
```

For Codex, Gemini CLI, and Pi, clone this repository and run:

```sh
./plugins/install.sh
```

That script installs the portable Linkuistics skills by symlink. Testanyware is
currently distributed through the Claude Code marketplace only. See
[`plugins/README.md`](plugins/README.md) for the complete plugin catalogue and
installation behavior.

## Documentation

- [Usage](docs/USAGE.md) — commands and the start-to-finish workflow.
- [Configuration](docs/CONFIGURATION.md) — harnesses, `.grove-stamps/`, task
  routing, models, and diagnostic overrides.
- [Architecture](docs/ARCHITECTURE.md) — runtime flow, task-tree model, module
  seams, VCS behavior, and current design constraints.
- [Releasing](docs/RELEASING.md) — cutting a version, publishing release
  archives, and updating the Homebrew tap.
- [Grove vocabulary](CONTEXT.md) and the [context map](CONTEXT-MAP.md).
- [Runtime agent methodology](content/SKILL.md) and its adjacent format guides.
- [Skill plugin documentation](plugins/README.md) and
  [provenance](plugins/linkuistics/PROVENANCE.md).
