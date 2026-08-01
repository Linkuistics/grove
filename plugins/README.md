# Skill plugins

Two Claude Code plugins live here — `linkuistics` and `testanyware` — published
through the marketplace declared in
[`../.claude-plugin/marketplace.json`](../.claude-plugin/marketplace.json). They
share this repo with the Grove CLI because the two change in lockstep
([architecture](../docs/ARCHITECTURE.md#skills-monorepo)); they ship by their own
path and are installed separately from Grove.

The vocabulary of authoring, packaging, triggering and installing a skill is in
[`CONTEXT.md`](CONTEXT.md).

## `linkuistics` — coding-standards skills

A suite of agent **skills** that load lazily — only when relevant to the file or
task at hand — across Claude Code, Codex, and other agents supporting the
[`SKILL.md`](https://agentskills.io) open standard.

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
| `codebase-design` | by description | deep-module design vocabulary — Ousterhout depth + Feathers seams, language-neutral |
| `decision-records` | by description | ADRs as a minimum coherent set describing the design's current state — current-state over changelog, identity by slug not number |
| `doubt-driven-development` | by description | in-flight adversarial verify — spawn a fresh-context reviewer to disprove a non-trivial decision before it stands |
| `git-to-jj-mapping` | by description | on-demand git→jj reference — command and concept translation, loaded only when a specific translation is needed |
| `using-jujutsu` | by description | drive version control through Jujutsu natively when the repo is jj-enabled (`.jj/` present); git, silently, everywhere else |
| `authoring-conventions` | by hand (`/authoring-conventions`, user-invoked) | house `SKILL.md` conventions — a thin delta over superpowers' `writing-skills` |
| `guardrail` | by hand (`/guardrail`, user-invoked) | session-scoped `PreToolUse` gate — pauses for confirmation before destructive shell commands or edits outside the project ("freeze") |

Each skill's one-line `description` is the only standing context cost; the body
loads on demand. In Claude Code the `paths:` frontmatter makes the language
skills auto-load deterministically by file type. Other harnesses ignore `paths:`
and fall back to the `description`.

Two skills are **user-invoked** (`disable-model-invocation: true`): they never
auto-fire and cost no standing context, reachable only by typing `/<name>`.

## `testanyware` — GUI testing in isolated VMs

One skill, `using-testanyware`, that makes driving GUI apps inside isolated
macOS/Linux/Windows VMs (via the `testanyware` CLI) standard practice — run,
test, screenshot, or record a GUI app without it touching your host, or reach a
Windows/Linux environment from macOS. It installs independently of `linkuistics`.

## Install — Claude Code

```
/plugin marketplace add Linkuistics/grove
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

## Install — Codex, Gemini CLI, other SKILL.md harnesses

```
git clone https://github.com/Linkuistics/grove.git
cd grove
./plugins/install.sh
```

[`install.sh`](install.sh) symlinks each `linkuistics` skill directory into
`~/.codex/skills/`, `~/.gemini/skills/`, and `~/.pi/agent/skills/` (only for
harnesses that are installed). Because the targets are symlinks, `git pull`
refreshes the content in place — re-run the script only when skills are added
or removed.

**Run it from the repo's main checkout.** The script links from whichever tree it
lives in and re-links every skill unconditionally, so running it from a linked
`git worktree` or a secondary `jj workspace` would re-point *all* your installed
skills at a tree that is usually temporary — and nothing would report it, since a
symlink whose target later disappears reads as "skill not installed" rather than
as an error. It detects that and refuses. Pass `--force` when linking from a side
tree is what you actually want (testing an unmerged skill against a live
harness), and re-run from the main checkout afterwards to repair the links.

`testanyware` is **not** covered by `install.sh`; it ships through the
marketplace only, so it is Claude Code only today.

This is a separate install path from grove's own. The `grove` binary provisions
*grove's* methodology to `~/.claude/skills/grove/` (and the codex and pi
equivalents) and nothing else — it never provisions these plugins. See
[`../README.md`](../README.md).

## Versioning

**Neither `plugin.json` declares a `version`, and that is deliberate — do not add
one.** Without it Claude Code versions a plugin by the commit SHA of its source,
and the source is the repo rather than the subdirectory, so both plugins report one
shared version that moves with every commit. Every push therefore delivers: edit a
skill, commit, done. Nothing to bump, nothing to grade, nothing to forget.

The cost is churn — in this repo, where most commits are grove's, each one reads as
an update to both plugins and re-installs content that did not change. That is the
accepted side of the trade, because pinning an explicit semver fails the other way:
the version is the cache key an update is decided on, so an unbumped change reaches
nobody, `/plugin update` reports "already at the latest version", and nothing
reports the omission
([Version management](https://code.claude.com/docs/en/plugins-reference#version-management)).
Churn is noisy and self-correcting; staleness is silent, and this repo has no CI and
no `pre-commit` hook (jj snapshots the working copy) to catch it. The repository
and delivery boundary is described in
[the architecture](../docs/ARCHITECTURE.md#skills-monorepo).

One trap: `claude plugin validate --strict` warns on the missing `version` and so
fails on both manifests. That warning is expected — silencing it by adding a
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
