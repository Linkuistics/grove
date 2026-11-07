# Skill plugins

<!-- tc: pending because the conversion has not reached this part of the document -->

<!-- tc: on -->

One plugin lives here: `grove`. It shares this repository with the Grove CLI,
because the two change together
([architecture](../docs/ARCHITECTURE.md#skills-monorepo)). Homebrew installs it
with Grove as the Ravel source `grove-core`, and Ravel delivers it to each
session. The Grove binary embeds and installs no skill.

The five domain packs `requirements`, `design`, `engineering`, `research` and
`writing` live in their own repositories. Grove's pack record,
[`packs.json`](packs.json), names the commit of each pack that Grove's
`task check:packs` accepted. Grove's formula installs each pack at that commit,
beside `grove-core`. The `personal`, `testanyware`, `superpowers` and
`quint-llm-kit` formulae install the other sources. Each source is a
directory in Ravel's default skill directory,
`$(brew --prefix)/share/ravel/skills/<source-id>`.

No remote marketplace lists a plugin. The Claude Code marketplace
[`../.claude-plugin/marketplace.json`](../.claude-plugin/marketplace.json) is
retired, and it lists none.

<!-- tc: pending because the conversion has not reached this part of the document -->

The vocabulary of authoring, packaging, triggering and installing a skill is in
[`CONTEXT.md`](CONTEXT.md).

## `grove` — the methodology, as skills

Grove's own methodology: one shared `grove` spine skill, with one
`grove-<kind>` skill per session kind beside it. The plugin tree is the
authoritative copy, and Grove's release copies it into the keg. Its own
[`grove/README.md`](grove/README.md) carries the fatness rule, the migration
ledger, and the dependency-free conformance runner that asserts delivery over the
shipped skill set.

Grove skills declare `harnesses: [any]`. The key stays as metadata, and no
installer reads it.

The `configure-grove` operator skill installs, inspects and edits the owner's
Ravel policy, which selects the harness, model and effort for each
session kind, with its owner settings and per-checkout choice file. It is
available without a task tree and is not a lifecycle kind.

## `requirements` — the requirements kinds, from RequirementsSkills

<!-- tc: on -->

The requirements kinds `requirements`, `review-requirements` and
`integrate-review-requirements` ship in their own repository,
[RequirementsSkills](https://github.com/Linkuistics/RequirementsSkills). The
skill `grove-requirements` holds its interview procedure, `grilling.md`. This
repository holds no copy of those skills. A fresh grove starts with a
`requirements` leaf, so a user who starts a grove needs this plugin.

The review and integration kinds are families across packs. Their procedures,
`references/review.md` and `references/integrate-review.md`, stay in the
spine. The two family members of the pack load `grove` and read those files
there.

Grove's pack record names the source directory `plugins/requirements` and one
RequirementsSkills commit. Grove's `task check:packs` checks that commit alone
and with this tree's core. Grove's formula installs the source directory as
`requirements`.

<!-- tc: pending because the conversion has not reached this part of the document -->

## `design` — the design kinds and craft skills, from DesignSkills

<!-- tc: on -->

The design kinds `design`, `review-design` and `integrate-review-design` ship
in their own repository,
[DesignSkills](https://github.com/Linkuistics/DesignSkills). The skill
`grove-design` holds its visual design procedure and its diagram viewer. This
repository holds no copy of those skills.

The plugin `design` also holds four craft skills in its `craft/` root:

| Skill | Loads when | Notes |
|-------|-----------|-------|
| `cli-tool-design` | by description | checklist in `SKILL.md`, audit detail in `references/` |
| `codebase-design` | by description | deep modules, structural simplicity and testable seams by Ousterhout, Hickey and Feathers. Its worked examples are in `references/` |
| `decision-records` | by description | ADRs as a minimum coherent set that describes the design's current state, with identity by slug |
| `model-led-development` | by description | routes a specification question to the instrument that answers it, then lets the checked model lead the implementation. The routing table and the log format are in `references/` |

The ADR and spec formats stay in the spine, as `ADR-FORMAT.md` and
`SPEC-FORMAT.md`. `decision-records` and `codebase-design` deepen those
formats, and the formats bind without them.

The review and integration kinds are families across packs, as for
`requirements`. The two family members of the pack load `grove` and read their
family files in the spine.

Grove's pack record names the source directory `plugins/design` and one
DesignSkills commit. Grove's `task check:packs` checks that commit alone and
with this tree's core. Conformance reads only `skills/`, and the pack's own
`task check` checks the craft skills. Grove's formula installs the source
directory as `design`.

<!-- tc: pending because the conversion has not reached this part of the document -->

## `engineering` — the engineering kinds and craft skills, from EngineeringSkills

<!-- tc: on -->

The engineering kinds `impl`, `review-impl`, `integrate-review-impl`,
`prototype`, `review-prototype` and `integrate-review-prototype` ship in their
own repository,
[EngineeringSkills](https://github.com/Linkuistics/EngineeringSkills). The
skill `grove-impl` holds its licence file in `LICENSES/`. This repository holds
no copy of those skills.

The plugin `engineering` also holds nine craft skills in its `craft/` root:

| Skill | Loads when | Notes |
|-------|-----------|-------|
| `coding-style-rust` | `*.rs` | |
| `coding-style-python` | `*.py` | |
| `coding-style-elixir` | `*.ex`, `*.exs` | |
| `coding-style-bash` | `*.sh`, `*.bash` | |
| `coding-style-swift` | `*.swift` | |
| `coding-style-typescript` | `*.ts`, `*.tsx` | |
| `doubt-driven-development` | by description | in-flight adversarial verification. A fresh-context reviewer tries to disprove a non-trivial decision before it stands |
| `simplify-project` | by description | contract-preserving repository simplification. It consolidates current-state documents, removes proven-dead artifacts and reconciles every consumer |
| `using-codebase-memory` | by description | queries a codebase knowledge graph from any shell through the `codebase-memory-mcp` command line. The silent failure modes are in `references/` |

The six language skills are the whole of the coding-standard coverage. No
language-neutral coding skill exists. A language without a skill here, such as
Go, Java or C, therefore triggers no coding standard, and no test-first rule.
A new language needs a new skill. The design rules in `codebase-design` apply
to every language, and the `design` plugin of DesignSkills ships that skill.

In Claude Code, the `paths:` frontmatter loads each language skill by file
type. Other harnesses ignore `paths:` and use the `description`.

The review and integration kinds are families across packs, as for
`requirements`. The four family members of the pack load `grove` and read
their family files in the spine.

Grove's pack record names the source directory `plugins/engineering` and one
EngineeringSkills commit. Grove's `task check:packs` checks that commit alone
and with this tree's core. Conformance reads only `skills/`. The pack's own
`task check` checks the craft skills, and its doubt check compares
`doubt-driven-development` with Grove's text. Grove's formula installs the
source directory as `engineering`.

<!-- tc: pending because the conversion has not reached this part of the document -->

## `research` — the research kinds, from ResearchSkills

<!-- tc: on -->

The research kinds `research-a`, `research-b` and `combine-research` ship in
their own repository,
[ResearchSkills](https://github.com/Linkuistics/ResearchSkills). Its plugin
`research` also holds the `research-family` skill, the procedure that the two
survey halves share. This repository holds no copy of those skills.

Grove's pack record names the source directory `plugins/research` and one
ResearchSkills commit. Grove's `task check:packs` checks that commit alone and
with this tree's core. Grove's formula installs the source directory as
`research`. [`docs/specs/external-pack-contract.md`](../docs/specs/external-pack-contract.md)
states the contract that the pack follows.

## `writing` — the editorial kinds and writing craft skills, from WritingSkills

The editorial kinds `draft`, `copy-edit`, `art` and `proof` ship in their own
repository, [WritingSkills](https://github.com/Linkuistics/WritingSkills). Its
plugin `writing` also holds the `editorial-family` skill, the procedure that
the four stages share. This repository holds no copy of those skills.

The plugin `writing` also holds two craft skills in its `craft/` root:

| Skill | Loads when | Notes |
|-------|-----------|-------|
| `technical-communication` | always on, and by description | the model writes replies, documents and prompts in controlled English modelled on ASD-STE100. Hooks put it in front of every session on Claude Code, Codex and Pi, and one off file turns them off |
| `writing-code-walkthroughs` | by description | concept-ordered, source-exact code walkthroughs for Markdown books and subsystem guides |

One skill is **always on**: `technical-communication`. A harness entry gives
its instruction to every session, so it costs context on every turn. One off
file therefore turns all of its layers off.

Ravel delivers the skill, but it installs no hook. Grove's keg therefore holds
the marketplace file `writing-hooks` for Claude Code and for Codex. Each file
names the skill directory `craft/technical-communication` of the keg's
`writing` source as its plugin root. On Claude Code, the plugin is
`technical-communication-hooks@writing-hooks`. On Codex, it is
`technical-communication@writing-hooks`, and the human trusts its hooks after
each installation or upgrade of Grove.

Pi gets an extension by one link command, and Gemini CLI gets the skill only.
[The hook adapters](../docs/USAGE.md#usage-hook-adapters) of the usage guide
give each command. The
[user document](https://github.com/Linkuistics/WritingSkills/blob/main/docs/TECHNICAL-COMMUNICATION.md)
in WritingSkills gives the switch and the reply log.

Grove's pack record names the source directory `plugins/writing` and one
WritingSkills commit. Grove's `task check:packs` checks that commit alone and
with this tree's core. Conformance reads only `skills/`. The pack's own
`task check` checks the craft skills and counts the hook declarations of each
installation. Grove's formula installs the source directory as `writing`.

<!-- tc: pending because the conversion has not reached this part of the document -->

## `personal` and `guardrail` — personal skills, from PersonalSkills

<!-- tc: on -->

The owner's personal skills ship in their own repository,
[PersonalSkills](https://github.com/Linkuistics/PersonalSkills). This
repository holds no copy of those skills.

The plugin `personal` holds three craft skills in its `craft/` root, and it has
no `skills/` root:

| Skill | Loads when | Notes |
|-------|-----------|-------|
| `machine-environment` | project setup, development commands, toolchains or persistent configuration on Antony's Mac | mise selects project tools, Taskfiles keep workflows, mise bootstrap owns machine setup, and the Obsidian manual supplies current details. It declares `assumes-personal-setup: true` |
| `using-jujutsu` | by description | drives version control through Jujutsu in a jj-enabled repository, and through Git elsewhere. The command surface and the Git-to-jj mapping are in `references/` |
| `authoring-conventions` | by hand (`/authoring-conventions`, user-invoked) | the house `SKILL.md` conventions, a thin delta over the `writing-skills` skill of superpowers |

The plugin `guardrail` holds only the user-invoked skill `guardrail`. The skill
turns on a session-scoped `PreToolUse` hook, which asks for confirmation before
a destructive shell command or an edit outside the project. It declares
`harnesses: [claude-code]`, so it is a native plugin, and no Ravel root reaches
it.

The `personal` formula pins one PersonalSkills commit. It installs
`plugins/personal` as the Ravel source `personal`, and `plugins/guardrail` as
the native plugin `libexec/plugins/guardrail`. Its keg holds the Claude Code
marketplace file `personal`, so the human registers `guardrail@personal` once
from the keg. Grove's `task check:packs` reads neither directory, because
Grove's pack record names the five domain packs only. The pack's own
`task check` checks the skills and the hook script.

<!-- tc: pending because the conversion has not reached this part of the document -->

## `testanyware` — GUI testing in isolated VMs

<!-- tc: on -->

The skill `using-testanyware` tells an agent to drive a GUI app inside an
isolated macOS, Linux or Windows VM through the `testanyware` command. The app
then does not touch the host. The plugin is not in this repository. It is
`plugins/testanyware` in the
[TestAnyware](https://github.com/Linkuistics/TestAnyware) repository. The
`testanyware` formula installs it with TestAnyware's executables, as the Ravel
source `testanyware`. TestAnyware's README states the source directory and its
checks.

<!-- tc: pending because the conversion has not reached this part of the document -->

## Install

<!-- tc: on -->

```sh
brew install linkuistics/taps/grove
```

Grove's formula installs `grove-core` and the five domain packs. Homebrew links
each one into Ravel's default skill directory, and Ravel 2.0.0 reads that
directory with no settings. `brew install` and `brew upgrade` are therefore the
whole skill setup. The [root README](../README.md#skills) names the other
formulae, and [the directory route](../docs/USAGE.md#usage-directory-route)
states the details. Grove ships no manual installer.

The hook adapters need one registration in each harness, because Homebrew cannot
write the configuration of a harness.
[The hook adapters](../docs/USAGE.md#usage-hook-adapters) give the commands.

<a id="install--codex"></a>
### Codex skills that earlier Grove releases installed

<!-- tc: pending because the conversion has not reached this part of the document -->

Grove releases up to 22.8 installed every bundled Codex-compatible skill in
`~/.agents/skills` at bare startup. Those links point through
`~/.agents/.grove/current` into complete snapshots under
`~/.agents/.grove/snapshots`. In `~/.codex/skills`, those releases also
repointed existing links to Grove plugin skills in the same way.

Later releases leave all of that unchanged. They do not adopt, repair, repoint
or remove it, and no upgrade does either. The
links keep working while their snapshots remain, but they no longer follow a
Grove upgrade. Readers that already resolved an old snapshot keep their files.

When both a provisioned link and a Ravel delivery supply one skill, the agent
can see both copies. Keep the old installation, or remove its links
by hand under the
[optional cleanup procedure](../docs/USAGE.md#usage-codex-cleanup). That
procedure first stops every process that can read the entries. It keeps the
snapshots.
Path shape alone does not show that Grove wrote an entry.

## Skill metadata

Each `SKILL.md` carries a `harnesses:` frontmatter key. The value is `[any]`,
or an allowlist such as `[claude-code]`. A skill can also declare
`assumes-personal-setup: true`. Both keys stay as metadata. No installer reads
them, and Ravel reads no frontmatter. [`CONTEXT.md`](CONTEXT.md) defines both
keys.

## Versioning

<!-- tc: on -->

**No `plugin.json` declares a `version`, and that is deliberate. Do not add
one.** A Homebrew formula carries the version of each source. Grove's version
covers `grove-core` and the five domain packs. A version cut changes only
`CHANGELOG.md`, so the `grove-core` bytes of a release equal those of its
candidate keg. A `version` that followed Grove's version would change those
bytes at each cut and break that equality.

The plugin manifests stay in their source directories as layout metadata. No
route installs core or a pack through them. The hook adapters need theirs.

Grove's pack record `plugins/packs.json` names the commit of each domain pack.
A pack change reaches users only when a Grove release records its new commit,
as [Releasing](../docs/RELEASING.md#a-pack-change) states. The record names the
five domain packs only. The `personal`, `testanyware`, `superpowers` and
`quint-llm-kit` formulae each pin their own commit.

<!-- tc: pending because the conversion has not reached this part of the document -->

One trap: `claude plugin validate --strict` warns on the missing `version` and so
fails on each manifest. That warning is expected — silencing it by adding a
`version` is the change this section exists to prevent.

## Editing a skill

Edit the `SKILL.md` under `<plugin>/skills/<name>/` (or `<plugin>/craft/<name>/`
for a craft skill) and commit. Keep
`description` sharp (a capability clause plus an explicit *Use when …* trigger)
and the body concise — an invoked skill stays in context for the rest of the
session. The house conventions are the `authoring-conventions` skill of
PersonalSkills (`/authoring-conventions`).

Each external pack keeps the provenance of its own skills in
`plugins/<plugin>/PROVENANCE.md`. The table of authoring sources is in the
[provenance file of `personal`](https://github.com/Linkuistics/PersonalSkills/blob/main/plugins/personal/PROVENANCE.md).
