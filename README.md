# grove

Grove is a hierarchical, self-extending workstream tool for AI coding agents.
It keeps a long project in a small, version-controlled task tree and launches
one fresh, appropriately configured agent session at a time. The repository
also contains a separately installed collection of agent skills.

## What's in this repository

| Product | Source | Purpose |
|---|---|---|
| Grove | [`crates/`](crates/) | The Rust workspace: two thin binaries over four library crates, the loop that launches one session per task among them. |
| Skill plugins | [`plugins/`](plugins/) | Grove's own methodology, the Linkuistics coding/design skills, and the Testanyware GUI-testing skill. |

The products share a repository but have separate installation paths, and Grove
installs none of them. The methodology used to travel inside the binary and be
swept into each harness's personal skill directory on every launch; it is now
the `grove` plugin, installed the way the other two are.

## Install Grove

```sh
brew tap Linkuistics/taps
brew install grove
```

There is no per-project installation step, and no per-machine one beyond the
configuration below: `grove --version` reports the installed binary version.
**Install the `grove` plugin as well** — the binary no longer carries the
methodology, so a session whose harness cannot load that skill has nothing to
read.

Grove needs one personal configuration file, `~/.config/grove/config.kdl`, giving
each of its nineteen session kinds a complete command template. Grove will not
start without it — see [Configuration](docs/CONFIGURATION.md).

Grove's methodology uses two Linkuistics skills: `decision-records` for ADR
discipline and `codebase-design` for testable module seams. Install the
Linkuistics plugin separately using the instructions below.

## Install the skill plugins

For Claude Code:

```text
/plugin marketplace add Linkuistics/grove
/plugin install grove@linkuistics
/plugin install linkuistics@linkuistics
/plugin install testanyware@linkuistics
```

`grove@linkuistics` is grove's own methodology as skills, and it is **required**:
it replaced the binary's embedded `content/`, which no build carries any more.
See [`plugins/grove/README.md`](plugins/grove/README.md).

For Codex, Gemini CLI, and Pi, clone this repository and run:

```sh
./plugins/install.sh
```

That script installs, by symlink, every bundled skill whose `harnesses:`
frontmatter key declares it installable there — every bundled plugin is scanned, and a
skill that cannot work off Claude Code (`guardrail`, whose mechanism is a Claude
Code hook) is skipped and reported rather than linked. See
[`plugins/README.md`](plugins/README.md) for the complete plugin catalogue and
installation behavior.

## Documentation

- [Usage](docs/USAGE.md) — the bare `grove` lifecycle and the start-to-finish
  workflow.
- [Configuration](docs/CONFIGURATION.md) — the personal KDL file, its nineteen
  session kinds, command-template grammar, and diagnostics.
- [Architecture](docs/ARCHITECTURE.md) — runtime flow, task-tree model, module
  seams, VCS behavior, and current design constraints.
- [Releasing](docs/RELEASING.md) — cutting a version, publishing release
  archives, and updating the Homebrew tap.
- [Grove vocabulary](CONTEXT.md) and the [context map](CONTEXT-MAP.md).
- [Runtime agent methodology](plugins/grove/skills/grove/SKILL.md) and its
  adjacent format guides.
- [Skill plugin documentation](plugins/README.md) and
  [provenance](plugins/linkuistics/PROVENANCE.md).
