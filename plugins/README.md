# Skill plugins

<!-- tc: pending because the conversion has not reached this part of the document -->

One Claude Code plugin lives here — `grove` —
published through the marketplace declared in
[`../.claude-plugin/marketplace.json`](../.claude-plugin/marketplace.json). The
marketplace also lists `requirements`, `design`, `engineering`, `research`, `writing`, `technical-communication-hooks`, `personal`, `guardrail` and `testanyware`, which live in their own
repositories and which it pins by commit. The plugin here
shares this repo with the Grove CLI because the two change in lockstep
([architecture](../docs/ARCHITECTURE.md#skills-monorepo)). Homebrew installs it
with Grove, and Ravel delivers it to each session. The Grove binary embeds and
installs no skill.

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

The marketplace lists `requirements` with a `git-subdir` source. That source
names `plugins/requirements` and pins one RequirementsSkills commit in `sha`.
Grove's `task check:packs` checks the pinned commit alone and with this tree's
core. The plugin declares a dependency on `grove`, so a Claude Code install
also installs the spine.

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

The marketplace lists `design` with a `git-subdir` source. That source names
`plugins/design` and pins one DesignSkills commit in `sha`. Grove's
`task check:packs` checks the pinned commit alone and with this tree's core.
Conformance reads only `skills/`, and the pack's own `task check` checks
the craft skills. The plugin declares a dependency on `grove`, so a Claude Code
install also installs the spine.

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

The marketplace lists `engineering` with a `git-subdir` source. That source
names `plugins/engineering` and pins one EngineeringSkills commit in `sha`.
Grove's `task check:packs` checks the pinned commit alone and with this tree's
core. Conformance reads only `skills/`. The pack's own `task check` checks the
craft skills, and its doubt check compares `doubt-driven-development` with
Grove's text. The plugin declares a dependency on `grove`, so a Claude Code
install also installs the spine.

<!-- tc: pending because the conversion has not reached this part of the document -->

## `research` — the research kinds, from ResearchSkills

<!-- tc: on -->

The research kinds `research-a`, `research-b` and `combine-research` ship in
their own repository,
[ResearchSkills](https://github.com/Linkuistics/ResearchSkills). Its plugin
`research` also holds the `research-family` skill, the procedure that the two
survey halves share. This repository holds no copy of those skills.

The marketplace lists `research` with a `git-subdir` source. That source names
`plugins/research` and pins one ResearchSkills commit in `sha`. Grove's
`task check:packs` checks the pinned commit alone and with this tree's core. The
plugin declares a dependency on `grove`, so a Claude Code install also installs
the spine. [`docs/specs/external-pack-contract.md`](../docs/specs/external-pack-contract.md)
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

On Claude Code, the hooks ship in the separate plugin
`technical-communication-hooks`, which holds no skill. The plugin `writing`
declares it as a dependency, so a Claude Code install of `writing` also installs
the hooks. A Ravel user who gets the skill from a directory can install the
hooks plugin alone.

Codex needs the hooks plugin that the Codex marketplace of WritingSkills lists.
The human adds and trusts that plugin once. Pi gets an extension by one
documented link command, and Gemini CLI gets the skill only. The
[user document](https://github.com/Linkuistics/WritingSkills/blob/main/docs/TECHNICAL-COMMUNICATION.md)
in WritingSkills gives the commands, the switch and the reply log.

The marketplace lists `writing` with a `git-subdir` source. That source names
`plugins/writing` and pins one WritingSkills commit in `sha`. Grove's
`task check:packs` checks the pinned commit alone and with this tree's core.
Conformance reads only `skills/`. The pack's own `task check` checks the craft
skills and counts the hook declarations of each installation.

The marketplace lists `technical-communication-hooks` from the same commit,
with the source directory `plugins/writing/craft/technical-communication`. The plugin `writing`
declares dependencies on `grove` and `technical-communication-hooks`, so a
Claude Code install also installs the spine and the hooks.

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

The marketplace lists both plugins with `git-subdir` sources at one
PersonalSkills commit. The source directories are `plugins/personal` and
`plugins/guardrail`. The plugin `personal` declares a dependency on
`guardrail`, so a Claude Code install of `personal` also installs the
guardrail. Grove's `task check:packs` reads neither source, because Grove's
pack record names the five domain packs only. The pack's own `task check`
checks the skills and the hook script.

<!-- tc: pending because the conversion has not reached this part of the document -->

## `testanyware` — GUI testing in isolated VMs

<!-- tc: on -->

The skill `using-testanyware` tells an agent to drive a GUI app inside an
isolated macOS, Linux or Windows VM through the `testanyware` command. The app
then does not touch the host. The plugin is not in this repository. It is
`plugins/testanyware` in the
[TestAnyware](https://github.com/Linkuistics/TestAnyware) repository, and the
marketplace pins one TestAnyware commit for it. TestAnyware's README states the
source directory and its checks. The plugin installs independently of the
other plugins.

<!-- tc: pending because the conversion has not reached this part of the document -->

## Install — Claude Code

```
/plugin marketplace add Linkuistics/grove
/plugin install grove@linkuistics
/plugin install requirements@linkuistics
/plugin install design@linkuistics
/plugin install engineering@linkuistics
/plugin install research@linkuistics
/plugin install writing@linkuistics
/plugin install personal@linkuistics
/plugin install testanyware@linkuistics
```

Enable auto-update for the marketplace (`/plugin` → Marketplaces → Enable
auto-update) so every Claude Code startup picks up each plugin's latest published
version — see *Versioning* below for what "published" means here.

The marketplace's identity is the `name` field in `marketplace.json`, not the
repo URL — it was already `linkuistics` while this tree lived in
`Linkuistics/skills`, so every `<plugin>@linkuistics` install command is
unaffected by the move. Only where you add the marketplace *from* changed.

## Install — Codex

Grove names the skill each session must load. It does not install skills.
Homebrew installs the skill sources, and Ravel delivers them to each session.
Grove ships no manual installer. Codex reads the user skill roots
`~/.codex/skills` and, from Codex 0.166.0, `~/.agents/skills`.

Grove releases up to 22.8 installed every bundled Codex-compatible skill in
`~/.agents/skills` at bare startup. Those links point through
`~/.agents/.grove/current` into complete snapshots under
`~/.agents/.grove/snapshots`. In `~/.codex/skills`, those releases also
repointed existing links to Grove plugin skills in the same way.

Later releases leave all of that unchanged. They do not adopt, repair, repoint
or remove it, and no upgrade does either. The
links keep working while their snapshots remain, but they no longer follow a
Grove upgrade. Readers that already resolved an old snapshot keep their files.

When both a provisioned link and a manually installed link supply one skill,
the harness can see both copies. Keep the old installation, or remove its links
by hand under the
[optional cleanup procedure](../docs/USAGE.md#usage-codex-cleanup). That
procedure first stops every process that can read the entries. It keeps the
snapshots.
Path shape alone does not show that Grove wrote an entry.

Ravel skill-directory delivery is the alternative route, and it needs no native
copy. Ravel 2.0.0 reads skill sources from local directories that its settings
name. Make each checkout of this repository with a tool outside Ravel, because
Ravel downloads and updates nothing. Point a source at the `plugins/grove`
directory of that checkout, with the root `skills`. Ravel 1.1.0 refuses those
settings, so use a separate HOME while a 1.1 launch remains. The
[directory route](../docs/USAGE.md#usage-directory-route) gives the settings.

## Skill metadata

Each `SKILL.md` carries a `harnesses:` frontmatter key. The value is `[any]`,
or an allowlist such as `[claude-code]`. A skill can also declare
`assumes-personal-setup: true`. Both keys stay as metadata. No installer reads
them, and Ravel reads no frontmatter. [`CONTEXT.md`](CONTEXT.md) defines both
keys.

## Versioning

**No `plugin.json` declares a `version`, and that is deliberate — do not add
one.** Without it Claude Code versions a plugin by the commit SHA of its source,
and the source is the repo rather than the subdirectory, so the plugin here reports
a version that moves with every commit. Every push therefore delivers: edit a
skill, commit, done. Nothing to bump, nothing to grade, nothing to forget.

The cost is churn — in this repo, where most commits are grove's, each one reads as
an update to the plugin and re-installs content that did not change. That is the
accepted side of the trade, because pinning an explicit semver fails the other way:
the version is the cache key an update is decided on, so an unbumped change reaches
nobody, `/plugin update` reports "already at the latest version", and nothing
reports the omission
([Version management](https://code.claude.com/docs/en/plugins-reference#version-management)).
Churn is noisy and self-correcting; staleness is silent, and this repo has no CI and
no `pre-commit` hook (jj snapshots the working copy) to catch it. The repository
and delivery boundary is described in
[the architecture](../docs/ARCHITECTURE.md#skills-monorepo).

<!-- tc: on -->

The `requirements`, `design`, `engineering`, `research`, `writing`, `technical-communication-hooks`, `personal`, `guardrail` and `testanyware` plugins are the exceptions. The marketplace
entry of each pins a commit of its own repository in `sha`. Claude Code
delivers that commit until a Grove commit moves the pin. The pin gives native
users the commit that Grove's `task check:packs` accepted. That check reads
Grove's pack record `plugins/packs.json`, which names the five domain packs
only. It reads no commit of `testanyware`, `technical-communication-hooks`,
`personal` or `guardrail`.

<!-- tc: pending because the conversion has not reached this part of the document -->

One trap: `claude plugin validate --strict` warns on the missing `version` and so
fails on each manifest. That warning is expected — silencing it by adding a
`version` is the change this section exists to prevent.

None of this touches the symlink install above — those skills update with a
`git pull`, no version involved.

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
