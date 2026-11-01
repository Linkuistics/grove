# Owner skill delivery evidence

This document records the inputs to the
[owner skill delivery design](specs/owner-skill-delivery.md). The design
session only read the machine. It changed no installation, setting, plugin
registry or file outside this checkout, and it signalled no other session.

## Method

On 2026-11-02, the session walked every skill discovery root of Claude Code,
Codex and Pi. It also read both Claude Code plugin registries and each
installed plugin's components. The walk covered user hooks, agent definitions,
MCP entries, the owner policy and the tool installations that skills need.

[`evaluations/owner-skill-delivery/inventory.py`](evaluations/owner-skill-delivery/inventory.py)
is the instrument. It writes [the baseline](owner-skill-inventory.tsv), one
row for each entry. Each row names the entry's harness, location, form,
frontmatter name, tree digest, file count, class, source owner, capabilities,
disposition and a note. The tree digest is the first 16 hexadecimal digits of
a SHA-256 over sorted relative paths, link targets and file digests. Rows
carry `~` for HOME. They record names only, never hook bodies or configuration
values.

| Artifact | SHA-256 |
| --- | --- |
| `docs/owner-skill-inventory.tsv` | `c9af548bdf8d616437e9f46dfa89dac6f97ac23dfb204c107734f85ab5aba4f0` |
| `docs/evaluations/owner-skill-delivery/inventory.py` | `6138bacfcd22f3aca25c61533207df4ccb3dfae792fb9ae8d5f17e10d6219792` |

The saved instrument reproduced the committed baseline byte for byte.

### Controls

A clean read from a broken instrument looks the same as a clean read from a
working one. Three controls separate the two cases:

- **Mutation control.** A scratch root held an unknown skill directory and a
  dangling link. The instrument reported one `UNCLASSIFIED` row and one
  dangling row for that root.
- **Clean run.** Without the scratch root, no row said `UNCLASSIFIED` or
  `UNASSIGNED`.
- **Positive control.** The instrument found `using-testanyware` in all four
  places that hold it: the Claude Code plugin, both Codex user roots and the
  dangling Pi link.

## Inputs

| Subject | Observation |
| --- | --- |
| Harnesses | Claude Code 2.1.316 and Codex CLI 0.166.0. Pi's settings record changelog version 0.84.2. |
| Grove and Ravel | Installed Grove 22.8.0 and Ravel 1.1.0. The driver's ending executable is `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`. |
| Linkuistics plugins | Installed version `238c3a839ab4`, the marketplace clone's commit `238c3a839ab4b30b101fbe3cc5212d66091df28a`. The marketplace follows `Linkuistics/grove` with automatic updates. |
| Superpowers | Plugin 7.0.0 at `bb92a77741419a4ab5f06e711a283343f1ada0c3`, from `https://github.com/obra/superpowers.git`. Fifteen skills and one `SessionStart` hook. |
| Quint | Plugin 1.0.0 at `cc75369f741af7d490936f82002c2d28e3b3d78d`. The clone `~/.quint-llm-kit/repo` is at the same commit, with a modified `package-lock.json`. `quint` 0.33.1. |
| TestAnyware | Homebrew `linkuistics/taps/testanyware` 2.7.0 and `/Applications/TestAnyware.app` 2.7.0. `testanyware doctor --json` passed its installation, configuration, directory and local-network checks. Its login-item check failed, which does not affect a skill read. |
| codebase-memory | `codebase-memory-mcp` 0.10.6. Its installer wrote skill copies, agent definitions, hooks and MCP entries in both harnesses. |
| Owner policy | `~/.config/ravel/policy.ts` routes each kind to `claude` or `codex`. `finish` goes to Claude Code with `claude-opus-5-5`. No route selects Pi. |
| Ravel settings | `~/.config/ravel` holds only `policy.ts`. No `settings.json` exists. |
| Ravel contracts | The historical directory contract `docs/specs/skill-discovery.md` at `b93bb927`. Ravel main's `docs/specs/configuration-evaluation.md` at `24a54523`. |

Ravel main keeps settings at `$HOME/.config/ravel/settings.json` for 2.0.0.
Its keys are `timeoutMs`, `contextBytes`, `configEnv`, `harnesses` and
`budgets`, and an unknown key refuses. The historical directory contract adds a
`skills` key and states that a 1.1 launch refuses shared settings that hold it.

## Discovery roots

| Harness | Root | Content at the baseline |
| --- | --- | --- |
| Claude Code | Plugin cache under `~/.claude/plugins/cache` | Enabled and disabled marketplace plugins, each at an immutable versioned path |
| Claude Code | `~/.claude/skills` | The tool's `codebase-memory` copy, a stale `grove` copy, account-synced skills and a dangling `terminal-browser` link |
| Claude Code | Built in | Skills that the harness supplies. The session's own skill listing is the only evidence for them. |
| Codex | `~/.codex/skills` | Grove-snapshot links, older superpowers copies, the tool's `codebase-memory` copy, a dangling `terminal-browser` link and system skills |
| Codex | `~/.agents/skills` | The same Grove-snapshot links, Quint links, the tool's `codebase-memory` copy and a dangling `terminal-browser` link |
| Codex | Bundled and runtime marketplaces | Platform plugin skills, most disabled in `config.toml` |
| Pi | `~/.pi/agent/skills` and `extensions` | Links to `~/Development/grove`, which no longer exists, and tool extensions |

The Codex executable names the roots `codex-home-skills`, `user-agent-skills`,
`system-skills` and a plugin cache. Codex therefore reads both user roots. Its
handling of one name in both roots was not measured.

## Baseline summary

The baseline has 375 entry rows.

| Class | Rows |
| --- | ---: |
| Owner pack | 137 |
| Platform-managed | 62 |
| Tool-managed | 49 |
| Dangling | 48 |
| Disabled third-party | 26 |
| Selected third-party pack | 24 |
| Stale copy | 15 |
| Capability-only | 14 |

These defects exist before any move:

- Each Pi skill link and the Pi technical-communication extension link dangle.
  They point at `~/Development/grove`, which the earlier checkout move left
  behind.
- `terminal-browser` links dangle in `~/.claude/skills`, `~/.codex/skills` and
  `~/.agents/skills`. The cask is absent from Homebrew.
- Both Codex user roots hold links to the same Grove snapshot entries.
- `~/.codex/skills` holds superpowers copies without provenance. Ten of them
  differ from superpowers 7.0.0, and four match it.
- `~/.claude/skills/grove` is an earlier Grove copy with retired signal files.
  Its name collides with the plugin's `grove` skill.
- Codex's snapshot copies of `technical-communication` and
  `machine-environment` differ from the Claude Code plugin's bytes.
- `guardrail` has no Codex link, as its `harnesses: [claude-code]` metadata
  directs.

## Capabilities beside the guidance

- The `linkuistics` Claude Code plugin carries the technical-communication
  hooks for six events. The same skill holds a Codex hooks plugin, but the
  owner's Codex configuration names no marketplace that offers it. Codex
  sessions therefore run without that always-on layer.
- `guardrail` installs a `PreToolUse` hook from its frontmatter and responds
  to `/guardrail`. Both need native Claude Code discovery.
- `authoring-conventions` and `guardrail` set `disable-model-invocation: true`.
- The superpowers `SessionStart` hook injects `using-superpowers` into each
  Claude Code session.
- Claude Code reads MCP entries for `quint-kb` and `codebase-memory-mcp`.
  Codex reads `node_repl`, `context7`, `openaiDeveloperDocs`, `quint-kb`,
  `computer-use` and `codebase-memory-mcp`.
- User hooks in both harnesses belong to Orca, herdr and codebase-memory-mcp.
  Each hook script names its installer.

## Machine contract

mise tracks `~/.claude/settings.json`, `~/.claude/hooks`, `~/.codex/config.toml`,
`~/.codex/hooks.json`, `~/.codex/AGENTS.md` and `~/.config/ravel` in place. It
does not track the skill roots, the plugin registries or `~/.pi`. The
maintained manual for this area is the Obsidian `Configuration/AI Dev Tooling`
note and its maintenance script. That note still names `~/Development/grove` as
the live Grove source.

## Not measured

- Claude Code's treatment of a root-level `SKILL.md` inside a skill-free
  plugin.
- The effect of disabling a plugin on a Claude Code session that is already
  running.
- Claude Code's sweep rule for cached plugin versions.
- Whether Claude Code under the owner policy reads snapshot paths outside its
  working directory without a permission prompt.
- Codex's handling of one skill name in both user roots.
- Pi's startup with a dangling extension link.
- Built-in Claude Code skills on disk. The baseline lists them from the
  session's skill listing.

The staging trials in the design's cutover contract own each of these
measurements before the switch.
