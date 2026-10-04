# grove

Grove is a hierarchical, self-extending workstream tool for AI coding agents.
It keeps a long project in a small, version-controlled task tree and launches
one fresh agent session at a time, with the harness, model and effort your own
policy selects for that task. The repository
also contains the agent skills used by those sessions.

## What's in this repository

| Product | Source | Purpose |
|---|---|---|
| Grove | [`crates/`](crates/) | The Rust workspace: two thin binaries over five library crates, the loop that launches one session per task among them. |
| harness-dispatch | [`crates/harness-dispatch/`](crates/harness-dispatch/) | A separate command in the same workspace, installed with Grove and usable without it: it selects and runs an agent harness from its owner's TypeScript policy. Grove launches every session through it. |
| Skill plugins | [`plugins/`](plugins/) | Grove's own methodology, the Linkuistics coding/design skills, and the Testanyware GUI-testing skill. |

The products share a repository and a release snapshot. Bare `grove` installs
and repairs the bundled Codex-compatible skills before launching a session.
Claude Code uses the plugin marketplace with auto-update enabled.

## Install Grove

```sh
brew tap Linkuistics/taps
brew install grove
```

There is no per-project installation step. `grove --version` reports the
installed binary version. The same installation carries `harness-dispatch`, a
separate command that selects and runs an agent harness from its owner's
TypeScript policy ([its README](crates/harness-dispatch/README.md)). The first bare `grove` run provisions Codex skills when
`~/.codex` is a directory or `CODEX_HOME` is set to a nonempty value. Claude Code and other
harnesses need the setup below.

Grove has no launch configuration of its own. It launches every session by
running `harness-dispatch`, and your policy there,
`~/.config/harness-dispatch/policy.ts`, returns the harness command for each
session kind. Install the sample policy once, read it, and edit it:

```sh
harness-dispatch init
cd /work/parser && harness-dispatch inspect --kind impl
```

The sample launches `codex` with approvals off and full access, so read it
before the first launch. `inspect` reports what a kind would launch, and
launches nothing; run it in the working-tree root, where Grove runs it. A kind your policy does not route is refused when its leaf
launches, with a remedy, and rerunning `grove` continues. A `config.kdl` or
`.grove.kdl` from an earlier release is never read.

Two more files are yours. `~/.config/harness-dispatch/settings.json` sets what
applies to every launch: the selection time bound and the environment variables
granted to the policy among it. Never grant `GROVE_LAUNCH_DIR` there, because
a policy holding it could act on the Grove tree or record a teardown for
the session it is selecting for. A `.harness-dispatch-choice` file in a working-tree root picks, for that checkout,
among the options your policy offers.
[Launch policy](docs/USAGE.md#usage-launch-policy) in the usage guide has the
rest.

Grove's methodology uses Linkuistics' `decision-records` for ADR discipline and
`codebase-design` for module design, structural simplicity, and test seams.
Codex receives those skills automatically; install the Linkuistics plugin for
Claude Code using the instructions below.

## Browse a task tree

Run `grove view` for the current directory's `.grove`, or
`grove view /path/to/worktree` for another tree. The browser is permanently
read-only and needs no jj workspace, launch policy or installed skills.
It never searches parent directories. Files render as formatted Markdown and
refresh automatically every 500 ms; `r` requests an immediate refresh.
Tab switches between tree navigation and file scrolling;
use arrows or hjkl, Enter/Space to fold branches, Home/End to jump,
PageUp/PageDown to read files, Left/Right to scroll wide code and tables,
`?` for key help, and q or Ctrl-c to quit. Resize reflows prose while retaining
the reading location; returning to a file restores its saved position.
Selection follows permanent keys across renames, moves and retirement. Replacing
the root starts a fresh view. Reading positions follow unchanged source passages
through edits, including on revisit; deleted passages fall back to nearby surviving
text. Identical passages use source context and proximity to choose consistently.
While a writer holds the tree, reads show WAITING and retry without blocking input.
Quit, handled termination signals and errors restore the terminal; unwinding
panics restore it before printing their diagnostic.
See [Viewing a tree](docs/USAGE.md#usage-viewing-tree) for details and recovery.

## Install the skill plugins

For Claude Code:

```text
/plugin marketplace add Linkuistics/grove
/plugin install grove@linkuistics
/plugin install linkuistics@linkuistics
/plugin install testanyware@linkuistics
```

Enable marketplace auto-update (`/plugin` → Marketplaces → Enable auto-update)
so Claude Code keeps all three plugins current. `grove@linkuistics` supplies
Grove's methodology; see [`plugins/grove/README.md`](plugins/grove/README.md).

For Codex, bare `grove` automatically installs every bundled compatible skill
from Grove, Linkuistics and Testanyware into `~/.agents/skills`. It detects
`~/.codex` or an explicit `CODEX_HOME`, uses the snapshot embedded in the binary,
and repairs missing or outdated Grove-managed installations before launch. No
checkout or network access is needed. Foreign entries with the same name are
preserved and reported as an error; move those entries aside before retrying.

For Gemini CLI and Pi, clone this repository and run:

```sh
./plugins/install.sh
```

That script also remains available for checkout-based Codex installations. Both
routes filter on each skill's `harnesses:` declaration, so Claude-only
`guardrail` is excluded from Codex. See [`plugins/README.md`](plugins/README.md)
for the catalogue, ownership rules and installation paths.

## Documentation

- [Usage](docs/USAGE.md) — the bare `grove` lifecycle, the `grove-llm` verbs
  over the task tree, and the start-to-finish workflow.
- [harness-dispatch](crates/harness-dispatch/README.md) — selection policies,
  inspection, run records and refusals, and
  [how Grove calls it](crates/harness-dispatch/README.md#called-from-grove).
- [Architecture](docs/ARCHITECTURE.md) — the design decisions, constraints and
  measurement records behind the runtime, the task tree and the VCS seam.
- [System overview](docs/walkthroughs/overview/README.md) — the `grove` binary
  read page by page, with the runtime flow, the two command surfaces and the
  module map its one call reaches; the other code walkthroughs sit beside it
  under `docs/walkthroughs/`.
- [Releasing](docs/RELEASING.md) — cutting a version, publishing release
  archives, and updating the Homebrew tap.
- [Grove vocabulary](CONTEXT.md) and the [context map](CONTEXT-MAP.md).
- [Runtime agent methodology](plugins/grove/skills/grove/SKILL.md) and its
  adjacent format guides.
- [Skill plugin documentation](plugins/README.md) and
  [provenance](plugins/linkuistics/PROVENANCE.md).
