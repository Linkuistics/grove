# Skill plugins

<!-- tc: pending because the conversion has not reached this part of the document -->

Two Claude Code plugins live here — `grove` and `linkuistics` —
published through the marketplace declared in
[`../.claude-plugin/marketplace.json`](../.claude-plugin/marketplace.json). The
marketplace also lists `research` and `testanyware`, which live in their own
repositories and which it pins by commit. The two plugins here
share this repo with the Grove CLI because the two change in lockstep
([architecture](../docs/ARCHITECTURE.md#skills-monorepo)); they ship by their own
path for Claude Code and through [`install.sh`](install.sh) for Codex, Gemini CLI
and Pi. The Grove binary embeds and installs none of them.

The vocabulary of authoring, packaging, triggering and installing a skill is in
[`CONTEXT.md`](CONTEXT.md).

## `grove` — the methodology, as skills

Grove's own methodology: one shared `grove` spine skill, with one
`grove-<kind>` skill per session kind beside it. The plugin tree is the
authoritative copy: Claude Code loads it through the marketplace, and the manual
installer links it for the other harnesses. Its own
[`grove/README.md`](grove/README.md) carries the fatness rule, the migration
ledger, and the dependency-free conformance runner that asserts delivery over the
shipped skill set.

Grove skills declare `harnesses: [any]`. The manual installer selects skills
through this metadata.

The `configure-grove` operator skill installs, inspects and edits the owner's
Ravel policy, which selects the harness, model and effort for each
session kind, with its owner settings and per-checkout choice file. It is
available without a task tree and is not a lifecycle kind.

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

<!-- tc: pending because the conversion has not reached this part of the document -->

## `linkuistics` — engineering-practice skills

A suite of agent **skills** that load lazily — only when relevant to the file or
task at hand — across Claude Code, Codex, and other agents supporting the
[`SKILL.md`](https://agentskills.io) open standard.

| Skill | Loads when | Notes |
|-------|-----------|-------|
| `machine-environment` | project setup, development commands, toolchains, or persistent configuration on Antony's Mac | mise selects project tools, Taskfiles retain workflows, mise bootstrap owns machine setup and tracked dotfiles, and the Obsidian manual supplies current details; declares `assumes-personal-setup: true` |
| `coding-style-rust` | `*.rs` | |
| `coding-style-python` | `*.py` | |
| `coding-style-elixir` | `*.ex`, `*.exs` | |
| `coding-style-bash` | `*.sh`, `*.bash` | |
| `coding-style-swift` | `*.swift` | |
| `coding-style-typescript` | `*.ts`, `*.tsx` | |
| `cli-tool-design` | by description | checklist in `SKILL.md`, audit detail in `references/` |
| `codebase-design` | by description | deep modules, structural simplicity and testable seams — Ousterhout, Hickey and Feathers; worked examples in `references/`, language-neutral |
| `decision-records` | by description | ADRs as a minimum coherent set describing the design's current state — current-state over changelog, identity by slug not number |
| `doubt-driven-development` | by description | in-flight adversarial verify — spawn a fresh-context reviewer to disprove a non-trivial decision before it stands |
| `model-led-development` | by description | route a specification question to the instrument that answers it — Alloy, Quint, a type system, or no model at all — then let the checked model lead the implementation; the routing table, the non-formal instruments and the log format are in `references/` |
| `simplify-project` | by description | contract-preserving repository simplification — consolidate current-state docs, remove proven-dead artifacts, reconcile every consumer |
| `technical-communication` | always on, and by description | the model writes replies, documents and prompts in controlled English modelled on ASD-STE100; hooks put it in front of every session on Claude Code, Codex and Pi, and one off file turns them off — see [the user document](../docs/TECHNICAL-COMMUNICATION.md) |
| `using-jujutsu` | by description | drive version control through Jujutsu natively when the repo is jj-enabled (`.jj/` present); git, silently, everywhere else — command surface and the git→jj mapping in `references/` |
| `using-codebase-memory` | by description | query a codebase knowledge graph from any shell via the `codebase-memory-mcp` CLI; the silent failure modes are in `references/` |
| `authoring-conventions` | by hand (`/authoring-conventions`, user-invoked) | house `SKILL.md` conventions — a thin delta over superpowers' `writing-skills` |
| `guardrail` | by hand (`/guardrail`, user-invoked) | session-scoped `PreToolUse` gate — pauses for confirmation before destructive shell commands or edits outside the project ("freeze"). **Claude Code only** (`harnesses: [claude-code]`) |

The six language skills are the whole of the coding-standard coverage: there is
no language-neutral coding skill, so a language without one here — Go, Java, C —
triggers no linkuistics standard at all, test-first included. Adding a language
means adding its skill; the cross-language rules that survive on their own are the
design ones in `codebase-design`.

Each skill's one-line `description` is the only standing context cost; the body
loads on demand, and a `references/` file only when the body sends you there. In Claude Code the `paths:` frontmatter makes the language
skills auto-load deterministically by file type. Other harnesses ignore `paths:`
and fall back to the `description`.

Two skills are **user-invoked** (`disable-model-invocation: true`): they never
auto-fire and cost no standing context, reachable only by typing `/<name>`.

One skill is **always on**: `technical-communication`. Its instruction reaches
every session through a harness entry, which costs context on every turn, so a
single off file turns all of its layers off. Claude Code loads
[`linkuistics/hooks/hooks.json`](linkuistics/hooks/hooks.json) with the plugin.
Codex needs the hooks plugin that [`.agents/plugins/marketplace.json`](../.agents/plugins/marketplace.json)
lists, which the human adds and trusts once. Pi gets an extension through
`install.sh`, and Gemini CLI gets the skill only. The commands, the switch and
the reply log are in [the user document](../docs/TECHNICAL-COMMUNICATION.md).

## `testanyware` — GUI testing in isolated VMs

<!-- tc: on -->

The skill `using-testanyware` tells an agent to drive a GUI app inside an
isolated macOS, Linux or Windows VM through the `testanyware` command. The app
then does not touch the host. The plugin is not in this repository. It is
`plugins/testanyware` in the
[TestAnyware](https://github.com/Linkuistics/TestAnyware) repository, and the
marketplace pins one TestAnyware commit for it. TestAnyware's README states the
source directory and its checks. The plugin installs independently of
`linkuistics`.

<!-- tc: pending because the conversion has not reached this part of the document -->

## Install — Claude Code

```
/plugin marketplace add Linkuistics/grove
/plugin install grove@linkuistics
/plugin install research@linkuistics
/plugin install linkuistics@linkuistics
/plugin install testanyware@linkuistics
```

Enable auto-update for the marketplace (`/plugin` → Marketplaces → Enable
auto-update) so every Claude Code startup picks up each plugin's latest published
version — see *Versioning* below for what "published" means here.

The marketplace's identity is the `name` field in `marketplace.json`, not the
repo URL — it was already `linkuistics` while this tree lived in
`Linkuistics/skills`, so every `linkuistics:<skill>` reference is unaffected by
the move. Only where you add the marketplace *from* changed.

## Install — Codex

Grove names the skill each session must load. It does not install skills. Run
the manual installer below for Codex. It links each eligible skill into
`~/.codex/skills`, one of the user skill roots that Codex reads. Codex 0.166.0
also reads `~/.agents/skills`.

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

## Manual install — Codex, Gemini CLI and Pi

```
git clone https://github.com/Linkuistics/grove.git
cd grove
./plugins/install.sh
```

[`install.sh`](install.sh) symlinks each **eligible** skill directory — from
both plugins here — into `~/.codex/skills/`, `~/.gemini/skills/`, and `~/.pi/agent/skills/`
(only for harnesses that are installed). A skill that ships `harnesses/pi/*.ts`
also has that Pi extension linked into `~/.pi/agent/extensions/`, under the same
ownership and reconciliation rules; `technical-communication` is the one that does. Because the targets are symlinks,
`git pull` refreshes the content in place — re-run the script only when skills are
added, removed, or change which harnesses they declare.

**Which harnesses a skill reaches is the skill's own declaration.** Every
`SKILL.md` carries a `harnesses:` frontmatter key — either `[any]`, meaning
nothing in it depends on a particular harness's affordances, or an explicit
allowlist such as `[claude-code]`. `guardrail` declares `[claude-code]`, because
its whole mechanism is a Claude Code `PreToolUse` hook and a codex or pi session
loading it would read instructions it cannot act on. Every skip is printed with
the declaration that caused it:

```
skip   codex  guardrail  (harnesses: claude-code)
```

A skill with **no** `harnesses:` key is installed nowhere and named in a note.
The allowlist keeps harness-specific instructions out of incompatible sessions
without withholding the portable skills.

A skill may also declare `assumes-personal-setup: true` — its content names the
author's own models, subscriptions, or machine configuration, so it is correct
here and misleading elsewhere. It still installs; the script reports it so you can
review it before relying on it. Only `doubt-driven-development` carries it.

**A re-run reconciles, it does not only add.** Every symlink pointing into a
checkout of this repo is the script's to manage, so one whose skill was deleted,
renamed, or made ineligible for that harness is removed and reported:

```
unlink codex  old-skill  (no longer eligible here)
```

Symlinks the script did not place are never touched. (This replaces the old
advice to hunt dangling links by hand.)

**Run it from the repo's main checkout.** The script links from whichever tree it
lives in and re-links every skill unconditionally, so running it from a linked
`git worktree` or a secondary `jj workspace` would re-point *all* your installed
skills at a tree that is usually temporary — and nothing would report it, since a
symlink whose target later disappears reads as "skill not installed" rather than
as an error. It detects that and refuses. Pass `--force` when linking from a side
tree is what you actually want (testing an unmerged skill against a live
harness), and re-run from the main checkout afterwards to repair the links.

`install.sh` covers both plugins here: `grove` and `linkuistics`.
It selects skills eligible for each detected harness using their `harnesses:`
frontmatter.

<!-- tc: on -->

**An external pack needs `--pack`.** The research kinds are in ResearchSkills,
so the plain run above does not install them. Make a checkout of ResearchSkills
and name its plugin directory:

```
git clone https://github.com/Linkuistics/ResearchSkills.git
./plugins/install.sh --pack ../ResearchSkills/plugins/research
```

The TestAnyware skill is also outside this repository. Name the plugin
directory of a TestAnyware checkout in the same way:

```
./plugins/install.sh --pack ../TestAnyware/plugins/testanyware
```

The script scans each `--pack` directory as it scans a bundled plugin, with the
same eligibility and the same workspace guard. Repeat `--pack` for each pack.
The script refuses a pack skill whose name a scanned plugin already uses. A run
reconciles the links of the plugins that it scans. A run without
`--pack <dir>` therefore leaves the links of that pack unchanged.

<!-- tc: pending because the conversion has not reached this part of the document -->

A real file or directory at a target path is an **error**: the installer leaves
it untouched, installs the other skills, and exits non-zero naming what was not
installed. If an older installation left a real directory there, preserve any
files you need, then move it aside or remove it and rerun the installer.

Grove itself never adopts, repairs or removes these links. They remain managed
by this script.

## Versioning

**No `plugin.json` declares a `version`, and that is deliberate — do not add
one.** Without it Claude Code versions a plugin by the commit SHA of its source,
and the source is the repo rather than the subdirectory, so both plugins here report
one shared version that moves with every commit. Every push therefore delivers: edit a
skill, commit, done. Nothing to bump, nothing to grade, nothing to forget.

The cost is churn — in this repo, where most commits are grove's, each one reads as
an update to every plugin and re-installs content that did not change. That is the
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

The `research` and `testanyware` plugins are the exceptions. The marketplace
entry of each pins a commit of its own repository in `sha`. Claude Code
delivers that commit until a Grove commit moves the pin. The pin gives native
users the commit that Grove's `task check:packs` accepted. For `testanyware`,
that check is the directory-only check, because the plugin has no kind skill.

<!-- tc: pending because the conversion has not reached this part of the document -->

One trap: `claude plugin validate --strict` warns on the missing `version` and so
fails on each manifest. That warning is expected — silencing it by adding a
`version` is the change this section exists to prevent.

None of this touches the symlink install above — those skills update with a
`git pull`, no version involved.

## Editing a skill

Edit the `SKILL.md` under `<plugin>/skills/<name>/` and commit. Keep
`description` sharp (a capability clause plus an explicit *Use when …* trigger)
and the body concise — an invoked skill stays in context for the rest of the
session. The house conventions are the `authoring-conventions` skill itself
(`/authoring-conventions`).

Source snapshots, licences, and the ideas adapted by individual skills are in
[`linkuistics/PROVENANCE.md`](linkuistics/PROVENANCE.md).
