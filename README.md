# grove

<!-- tc: pending because the conversion has not reached this part of the document -->

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
| Skill plugins | [`plugins/`](plugins/) | Grove's own methodology, and the record of the external packs that Grove's release installs. |

<!-- tc: on -->

Grove and its skills share a repository. [Ravel](https://github.com/Linkuistics/Ravel)
has releases of its own. Grove names the skill that each session must load. Homebrew
installs Grove's skills with Grove, and Ravel delivers them to each session. The
`grove` binary installs no skill.

## Install Grove

```sh
brew tap linkuistics/taps
brew trust linkuistics/taps
brew install grove
```

Homebrew does not trust a tap when `brew tap` adds it, and it refuses to load a
formula from a tap that is not trusted. Grove's formula loads its dependency
`ravel` from the same tap, so trust the tap before the first install. Homebrew's
[Tap Trust](https://docs.brew.sh/Tap-Trust#installing-from-a-tap) page gives the
rules.

`brew install grove` installs Grove and its skills, and `brew upgrade` keeps both current.
Ravel 2 delivers the skills, as [the skills](#skills) state. No project needs an
installation step. Homebrew installs these parts:

- The executables `grove` and `grove-llm`. `grove --version` reports the installed
  version.
- Grove's readable caller assets, at `libexec/ravel-policy`.
- Ravel, as a dependency with its own version.
- Core and the five domain packs, as [the skills](#skills) state.

A release archive holds the same files except Ravel. An archive user installs Ravel
separately.
Only a Homebrew installation links the skill sources into Ravel's default skill
directory, so an archive user names them in Ravel's settings.

<!-- tc: pending because the conversion has not reached this part of the document -->

Grove has no launch configuration of its own. Independent Ravel 2 reads your
`~/.config/ravel/config.rhai` and returns the selected command. Grove supports
stable Ravel 2.x from 2.0.0, supplied by the public unversioned `ravel` formula.
Copy the schema-4 sample into an absent file, read it and edit it:

```sh
mkdir -p ~/.config/ravel
cp "$(brew --prefix grove)/libexec/ravel-policy/sample/config.rhai" ~/.config/ravel/config.rhai
```

Preserve an existing policy. The sample launches codex with approvals off and
full access. Creator-aware examples require the complete caller directory tree
so their relative imports remain intact. A policy refusal gives a repair remedy;
rerunning `grove` continues. Ravel evaluates policy when a session starts; it has no inspection verb. Earlier config.kdl/.grove.kdl files are ignored.

`~/.config/ravel/settings.json` holds selection bounds and explicit policy
environment grants. Grove removes enclosing session authority before selection;
never grant lifecycle control names. `.ravel-choice` chooses among options the
policy offers. [Launch policy](docs/USAGE.md#usage-launch-policy) has the rest.

<!-- tc: on -->

Grove's methodology cites the craft skill `decision-records` for ADR discipline
and `codebase-design` for module design, structural simplicity and test seams.
Both skills ship in the `design` source of DesignSkills, which Grove's formula
installs.

<!-- tc: pending because the conversion has not reached this part of the document -->

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

## Skills

<!-- tc: on -->

Grove's formula installs six skill sources. Each is one Ravel source:

| Source | Repository | Content |
| --- | --- | --- |
| `grove-core` | This repository, [`plugins/grove`](plugins/grove/README.md) | Grove's methodology: the spine, the kinds of core and `configure-grove` |
| `requirements` | [RequirementsSkills](https://github.com/Linkuistics/RequirementsSkills) | The requirements kinds. The first leaf of a fresh grove is a `requirements` leaf |
| `design` | [DesignSkills](https://github.com/Linkuistics/DesignSkills) | The design kinds, and the craft skills `decision-records`, `codebase-design`, `cli-tool-design` and `model-led-development` |
| `engineering` | [EngineeringSkills](https://github.com/Linkuistics/EngineeringSkills) | The impl and prototype kinds, and the craft skills: six `coding-style-*` skills, `using-codebase-memory`, `simplify-project` and `doubt-driven-development` |
| `research` | [ResearchSkills](https://github.com/Linkuistics/ResearchSkills) | The research kinds |
| `writing` | [WritingSkills](https://github.com/Linkuistics/WritingSkills) | The editorial kinds `draft`, `copy-edit`, `art` and `proof`, and the craft skills `technical-communication` and `writing-code-walkthroughs` |

Grove's release takes each pack at the commit that Grove's pack check accepted. A
change to a pack therefore reaches users through a Grove release.

Homebrew links each source into Ravel's default skill directory,
`$(brew --prefix)/share/ravel/skills/`. Ravel 2.0.0 reads that directory when its
settings name no `sources`. It adds the location of each source to each launch
prompt, and the agent finds and reads its skills there. No harness needs a native
copy, and no setting is necessary.
[The directory route](docs/USAGE.md#usage-directory-route) gives the details.

The public `ravel` dependency supplies Ravel 2. `brew install grove` or
`brew upgrade` installs the compatible runtime and skills without a PATH
prefix. Configure `config.rhai` at schema 4 as [Install Grove](#install-grove)
states. [Launch policy](docs/USAGE.md#usage-launch-policy) covers configuration
and executable selection.

To disable directory delivery, set
`{"skills": {"schemaVersion": 1, "sources": []}}` in Ravel's optional
`settings.json`, with no mandatory file. Such a launch uses existing native
guidance, if available; Grove installs none. A session that cannot read its
required kind skill reports the missing guidance and force-stops.

Other formulae of the tap install more skill sources in the same way:

| Formula | Skills |
| --- | --- |
| `personal` | The owner's craft skills `machine-environment`, `using-jujutsu` and `authoring-conventions`, from [PersonalSkills](https://github.com/Linkuistics/PersonalSkills). The formula also holds the `guardrail` plugin for Claude Code |
| `testanyware` | The skill `using-testanyware`, with TestAnyware's executables |
| `superpowers` | The development workflow skills of [superpowers](https://github.com/obra/superpowers) |
| `quint-llm-kit` | The Quint specification skills of [quint-llm-kit](https://github.com/quint-co/quint-llm-kit) |

The always-on hooks of `technical-communication` and the `guardrail` hook are not
skills, so Ravel does not deliver them. Register them in each harness by the
commands of [the hook adapters](docs/USAGE.md#usage-hook-adapters). The caveats of
`brew info grove` print the same commands. On Codex, repeat the registration after
each upgrade of Grove.

No remote marketplace lists a pack, and Grove ships no manual installer. Grove's
Claude Code marketplace `linkuistics` is retired and lists no plugin. A user of that
marketplace does these steps in this order:

1. Install or upgrade Grove with Homebrew.
2. Use Ravel 2.0, which reads the skill sources with no settings.
3. Register the hook adapters.
4. Remove the marketplace with `/plugin marketplace remove linkuistics`. The removal
   uninstalls each plugin of the marketplace.

A refresh of the retired marketplace keeps each of its plugins installed and enabled,
but the plugin no longer loads, and no notice appears. Do the first three steps
before any refresh.

<!-- tc: pending because the conversion has not reached this part of the document -->

Grove releases up to 22.8 installed Codex skills automatically in
`~/.agents/skills` at startup. Later releases do not. They leave those links and
the snapshots under `~/.agents/.grove` unchanged, and the links keep working
while their targets exist. To remove the links, follow the
[optional cleanup procedure](docs/USAGE.md#usage-codex-cleanup). It keeps
`~/.agents/.grove`.

See [`plugins/README.md`](plugins/README.md) for the catalogue, the ownership
rules and the layout of each source.

## Documentation

- [Usage](docs/USAGE.md) — the bare `grove` lifecycle, the `grove-llm` verbs
  over the task tree, and the start-to-finish workflow.
- [Ravel](https://github.com/Linkuistics/Ravel) — independent process/router CLI;
  [Grove caller assets](assets/ravel-policy/README.md) — ordinary Rhai policies,
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
- [Skill plugin documentation](plugins/README.md).
- [Technical communication](https://github.com/Linkuistics/WritingSkills/blob/main/docs/TECHNICAL-COMMUNICATION.md)
  in WritingSkills — the always-on controlled-English skill: what it does, how
  to install its hooks, how to turn it off, and how to read its reply log.
