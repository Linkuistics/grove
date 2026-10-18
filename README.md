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
| Grove caller policies | [`assets/ravel-policy/`](assets/ravel-policy/) | Readable owner-editable policies and adapters for independent Ravel. |
| Skill plugins | [`plugins/`](plugins/) | Grove's own methodology, the Linkuistics coding/design skills, and the Testanyware GUI-testing skill. |

Grove and its skills share a release snapshot; [Ravel](https://github.com/Linkuistics/Ravel) is independently released. Bare `grove` installs
and repairs the bundled Codex-compatible skills before launching a session.
Claude Code uses the plugin marketplace with auto-update enabled.

## Install Grove

```sh
brew tap Linkuistics/taps
brew install grove
```

There is no per-project installation step. `grove --version` reports the
installed binary version. Homebrew installs independently versioned Ravel as a
runtime dependency. Grove archives ship only Grove executables and its readable
caller assets at `libexec/ravel-policy`; archive users install Ravel separately.
The first bare `grove` run provisions Codex skills when `~/.codex` is a directory
or `CODEX_HOME` is nonempty. Other harnesses need the setup below.

Grove has no launch configuration of its own. Independent Ravel reads your
owner policy at `~/.config/ravel/policy.ts` and returns the selected command.
Copy the sample into an absent owner policy, read it and edit it:

```sh
mkdir -p ~/.config/ravel
cp "$(brew --prefix grove)/libexec/ravel-policy/sample/policy.ts" ~/.config/ravel/policy.ts
```

Preserve an existing policy. The sample launches codex with approvals off and
full access. Creator-aware examples require the complete caller directory tree
so their relative imports remain intact. A policy refusal gives a repair remedy;
rerunning `grove` continues. Ravel evaluates policy when a session starts; it has no inspection verb. Earlier config.kdl/.grove.kdl files are ignored.

`~/.config/ravel/settings.json` holds selection bounds and explicit policy
environment grants. Grove removes enclosing session authority before selection;
never grant lifecycle control names. `.ravel-choice` chooses among options the
policy offers. [Launch policy](docs/USAGE.md#usage-launch-policy) has the rest.

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
- [Ravel](https://github.com/Linkuistics/Ravel) — independent process/router CLI;
  [Grove caller assets](assets/ravel-policy/README.md) — ordinary SDK policies,
  task conventions and creator evidence.

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
- [Technical communication](docs/TECHNICAL-COMMUNICATION.md) — the always-on
  controlled-English skill: what it does, how to install its hooks, how to turn it
  off, and how to read its reply log.
