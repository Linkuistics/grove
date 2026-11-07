# skills

<!-- tc: pending because the conversion has not reached this part of the document -->

The `grove` skill plugin under `plugins/`, and the external packs that the
marketplace lists — the language of **authoring, packaging, triggering and installing** a skill.

**Scope boundary.** A skill's own *subject matter* — jj's working-copy-as-commit,
for example — belongs to that skill and its ADRs, not here. This
glossary is about the corpus, not its contents.

## Language

### Units and packaging

**Skill**:
A directory containing `SKILL.md` — YAML frontmatter (`name`, `description`) over
a markdown body — as defined by the open Agent Skills standard, so it loads on
any conforming harness.
_Avoid_: "command" (a Claude Code slash command is a different artifact), "prompt",
"agent".

**Plugin**:
Claude Code's packaging unit — `plugins/<name>/` holding `.claude-plugin/plugin.json`
and a `skills/` directory — and a Claude Code concept only; Ravel delivers
the same [[Skill]] directories to every harness. One ships here: `grove`.
Their own repositories ship `requirements`, `design`, `engineering`, `research`,
`writing`, `technical-communication-hooks`, `personal`, `guardrail` and
`testanyware`.

<!-- tc: on -->

**Pack**:
A repository whose plugin directories ship methodology skills or craft skills.
Core is the pack in Grove's repository, and its plugin `grove` holds the spine
and the core kind skills. An external pack is a pack in another repository,
such as RequirementsSkills. `docs/specs/external-pack-contract.md` states how an
external pack joins Grove's methodology.
_Avoid_: "plugin" for a whole pack. One pack can ship more than one [[Plugin]].

**Source directory**:
The plugin directory `plugins/<plugin>/` inside a checkout of a [[Pack]]. Ravel
captures it as a source, and a skill formula copies it into its keg. It has one or both of two skill roots:

- `skills/`: the kind skills and any family skill.
- `craft/`: the craft skills.

_Avoid_: the repository root as the source directory. Ravel would then capture
the README, the Taskfile and any cache in each capture.

**Keg source**:
The copy of one [[Source directory]] in a Homebrew keg, at
`share/ravel/skills/<source-id>/`. Homebrew links it into
[[Default skill directory]]. Its `resourceDigest` equals that of a clean
checkout of the same commit. `docs/specs/codex-skill-provisioning.md` states
the keg layout.
_Avoid_: the versioned path under `Cellar/`. Homebrew's cleanup erases it after
an upgrade.

**Default skill directory**:
`$(brew --prefix)/share/ravel/skills/`, where Homebrew links each installed
[[Keg source]] by its source ID. Ravel 2.0.0 reads it when the settings name no
sources, so `brew install` and `brew upgrade` are the whole skill setup.
Ravel's specification owns the contract.
_Avoid_: `~/.config/ravel/skills/`. Ravel refuses a real source inside its
configuration directory, and Homebrew cannot write in HOME.

**Skill formula**:
A Homebrew formula that installs one or more [[Keg source]]s. Grove's formula
carries core and the five domain packs. The formulae `personal`,
`superpowers`, `quint-llm-kit` and `testanyware` carry the others.
_Avoid_: "plugin installation" for it. A formula installs files, and Ravel
delivers them.

**Pack record**:
Grove's `plugins/packs.json`. For each domain [[Pack]] that Grove's formula
carries, it holds the pack's `url`, its `path` and the accepted `sha`.
`task check:packs` reads it, and the release copies each pack from it.
_Avoid_: the marketplace as the record of a pack's revision. That role ended
with native distribution.

**Hook adapter**:
A native plugin or extension that runs a skill's hooks, such as
`technical-communication-hooks`. Ravel installs no hook. A keg therefore ships
the marketplace file of each adapter, and each harness registers it from the
keg's stable path.
_Avoid_: "hooks skill". An adapter holds no skill.

<!-- tc: pending because the conversion has not reached this part of the document -->

**Marketplace**:
`.claude-plugin/marketplace.json`, the catalogue a user adds once to reach every
[[Plugin]] in it. **Its identity is the `name` field, not the repo URL** — already
`linkuistics` while the tree lived in `Linkuistics/skills` — so every
`<plugin>@linkuistics` install command is repo-independent and a repo move
rewrites only where the marketplace is added from.
_Avoid_: treating the repository URL or owner as the namespace.

### Invocation and triggering

<!-- tc: on -->

**Directed load**:
The sentence form "Load the `<name>` skill", which names a [[Skill]] that the
reading session must load. Pack text names another skill only in this form,
by its bare name. The conformance runner reads each directed load as an edge of
a kind's loaded path, across the supplied packs. An HTML comment, a fenced code
block or a block quote holds no directed load.
_Avoid_: a plugin-qualified name such as `grove:grove` in pack text. A pack move
changes the namespace, and the qualified name then addresses nothing.

**Bare skill name**:
The name of a [[Skill]] without a plugin namespace, such as `grove` or
`codebase-design`. Pack text writes "Load the `<name>` skill" for a directed
load and "the `<name>` skill" for a citation of a craft skill. Where a harness
needs a namespace, the session uses the namespace under which the skill is
actually installed.
_Avoid_: a plugin-qualified name in a load or a citation, because a later pack
move changes its plugin. Until their own moves, the kinds that leave core keep
`grove:grove` in their openings.

<!-- tc: pending because the conversion has not reached this part of the document -->

**Namespaced invocation** (`<plugin>:<skill>`):
How Claude Code addresses a [[Skill]] that arrived inside a [[Plugin]] —
`personal:using-jujutsu`, `grove:grove`. A skill directory that Ravel delivers has no
namespace, so the same skill is the bare directory name there. Two consequences.
A plugin skill is **addressed differently** from a personal skill of the same
name, which is why `grove:grove` coexisted with the `grove` the binary used to
provision into `~/.claude/skills/`. Claude Code's plugins reference states the
namespacing; it does not state a collision rule, so that consequence rested on
the namespacing plus the observed case — and the case is gone since
`delete-provisioning-k19`, which stopped any binary writing that path. And a
prompt that names a skill for a session to load names **one bare target** rather
than branching on the harness — the session adds the namespace its own harness
needs. The qualified names in this entry are examples of the native spelling;
pack text never writes one ([[Bare skill name]]).
_Avoid_: treating the two spellings as two skills, or making the caller choose
between them. A selection is the thing the naming exists to remove.

**Model-invoked** (the default):
A [[Skill]] whose `description` sits in every session's context, so the agent can
auto-fire it on a match. It spends standing **context load** — the description is
in the window every turn, needed or not.

**User-invoked** (`disable-model-invocation: true`):
A [[Skill]] reachable only by a human typing `/<name>` — never auto-fired, zero
standing context cost, spending **cognitive load** instead, since the human is the
index. The lever is for operator guidance rather than capability, which is why only
`authoring-conventions` and `guardrail`, both in PersonalSkills, pull it.
_Avoid_: "disabled skill" — it is fully functional, just hand-only.

**Description shape** (the house rule):
A one-sentence **capability** plus an explicit **"Use when …"** trigger clause, and
never a step-by-step **workflow** summary — a description that lists steps becomes a
shortcut the agent takes *instead of* reading the body. Deliberately overrides
`superpowers:writing-skills`' when-only rule, because a stated capability is
load-bearing routing signal for a reference corpus.
_Avoid_: "when-to-use only" (that is upstream's rule, not this one).

**`paths:` trigger**:
A frontmatter glob list scoping a [[Skill]] to matching files — a **Claude Code
plugin auto-activation extension beyond the Agent Skills spec**, whose only
activation channel is the `description`. Carried by the `coding-style*` skills of
EngineeringSkills, and inert wherever they arrive as a Ravel skill directory.
_Avoid_: assuming a `paths:`-triggered skill fires on a spec-only harness.

**Hook-carrying skill**:
A [[Skill]] whose frontmatter declares Claude Code `hooks:`, so activating it
installs tool-call interception for the session. Only `guardrail`, in the
`guardrail` plugin of PersonalSkills, does — a
`PreToolUse` gate that asks rather than denies. Claude Code only; the block is a
no-op elsewhere.
_Avoid_: calling an [[Always-on skill]] hook-carrying. Its hooks are not in its
frontmatter, and they run in every session.

**Always-on skill**:
A [[Skill]] whose instruction reaches every session through a [[Harness entry]],
not only when its `description` matches. It costs context on every turn, so one
switch turns all of its always-on layers off. `technical-communication`, in the
`writing` plugin of WritingSkills, is the instance: its off file is the switch.
_Avoid_: "hook-carrying skill", which names a skill whose frontmatter declares
hooks that last only while the skill is active.

**Harness entry**:
The thin file, one for each harness, that maps that harness's events onto an
[[Always-on skill]]'s shared hook script. It is a plugin `hooks/hooks.json` for
Claude Code, a hooks plugin for Codex and an extension file for Pi. It never changes
shape, because Codex trusts a hash of each entry. Changes of behaviour go into
the script. `technical-communication` has three: `hooks/hooks.json` of the Claude
Code plugin `technical-communication-hooks`, `harnesses/codex/hooks.json` and
`harnesses/pi/technical-communication.ts`. All three are in the skill directory in
WritingSkills. Gemini CLI has none, so it gets the skill only.
_Avoid_: "adapter", which names the code for one harness's payload inside the
shared script.

### Authoring

**House delta**:
The `authoring-conventions` [[Skill]] of PersonalSkills records only where this corpus decides
*differently* from `superpowers:writing-skills`, plus the few conventions it adopts
by pointer. It never restates upstream craft — the upstream skill is a dependency,
not a thing to copy.

**Progressive disclosure**:
Keeping a [[Skill]] cheap to load: body under ~500 lines, a `references/` file over
~300 lines gets a table of contents, references stay one level deep, and
cross-references name another skill rather than linking a path. **Never `@path`** —
it force-loads the target before it is needed.

**Source citation** / **`UNVERIFIED`**:
The discipline for an embedded external fact (an API, a flag, a version-specific
behaviour): prefer the authority hierarchy (official docs > official blog >
standards > third-party), link the deep anchor, and where no source is found write
the literal marker `UNVERIFIED` rather than let the prose imply confidence.

### Distribution

**Commit-SHA version** (no `version` in `plugin.json`):
The version Claude Code derives for a [[Plugin]] when neither its manifest nor its
[[Marketplace]] entry declares one: the commit SHA of the plugin's *source*, which is
the **repo** and not the subdirectory — so every plugin here reports one shared version
string. It is the **cache key** an update is decided on, and it moves with every commit,
so every push delivers. Every plugin here uses it deliberately
([architecture](../docs/ARCHITECTURE.md#skills-monorepo)); the alternative, an explicit semver, *pins* — a skill
change shipped without a bump reaches no consumer and raises no error.
_Avoid_: adding a `version` to silence `claude plugin validate --strict`, which pins
delivery on a bump nothing in this repo can check; reading the SHA as identifying a
plugin's own content, when it identifies the repo's.

**Manual installer** (retired):
`plugins/install.sh`, which linked eligible skill directories into the
personal skill folders of Codex, Gemini CLI and Pi, and Pi extensions into
`~/.pi/agent/extensions/`. Its `--pack <dir>` option added an external
[[Pack]]. `manual-installer-removal-k236` removed it, with its link
reconciliation, its workspace guard and its tests. Homebrew installs the skill
sources, and Ravel delivers them.
_Avoid_: naming the installer as a route. Links that it made earlier stay until
their owner removes them.

**Harness eligibility** (`harnesses:`):
The frontmatter key by which a [[Skill]] declares which harnesses can follow it.
No installer reads it now, and Ravel reads no frontmatter, so it stays as
metadata. It is a YAML flow list, either the single claim `[any]` or an explicit
allowlist of harness ids (`claude-code`, `codex`, `gemini`, `pi`). It answers one
question: *can a session on this harness follow these instructions?* `guardrail`,
now in PersonalSkills, declares `[claude-code]` because its whole mechanism is a Claude Code
[[Hook-carrying skill]] frontmatter block, and it is now the only one that does:
the `grove` plugin's twenty skills declared it while the binary owned
`~/.claude/skills/grove` and two siblings, and flipped to `[any]` at
`delete-provisioning-k19` when it stopped writing them. Frontmatter is a safe carrier because a conforming loader parses the
block into a map and reads only the keys it knows — verified by reading pi's
`core/skills.js`, and already relied on by `paths:` and `hooks:` reaching codex
today. Gemini CLI is the one target not checked directly (it is not installed
here, so nothing is symlinked there either); the standard's own extension keys
are the precedent it would be violating.
_Avoid_: reading `any` as an enumeration of today's harnesses — it is a claim
about the *skill*, that nothing in it depends on a particular harness's
affordances, so a harness added later inherits it correctly.

**Personal-setup assumption** (`assumes-personal-setup: true`):
The frontmatter key by which a [[Skill]] declares that its content names the
author's own models, subscriptions, or machine configuration — so it is correct
here and misleading for anyone else. Orthogonal to [[Harness eligibility]]. No
installer reads it now, so it stays as metadata for the reader. No skill here carries it. In PersonalSkills,
`machine-environment` carries it. In EngineeringSkills, `doubt-driven-development` carries it for
`references/harness-spawns.md`.
_Avoid_: overloading it onto a skill that merely needs a tool installed — a missing
`codebase-memory-mcp` binary is a dependency, not a private
assumption.

## Flagged ambiguities

**"skill"** used to mean two different things in this repo, and now means one.
In this context it is a member of the corpus under `plugins/*/skills/`, shipped
by Homebrew and delivered by Ravel. Grove's methodology used to *also* be
provisioned as a skill (`~/.claude/skills/grove/`, written by the `grove` binary
from an embedded `content/`) while being no part of this corpus;
`delete-provisioning-k19` deleted that path and moved the methodology into
`plugins/grove/`, so grove's skills are members like any other. The distinction
worth keeping is the one `CONTEXT-MAP.md` draws: a `grove` skill's *contents* are
the grove context's, and its packaging is this one's.

## Example dialogue

> **Dev:** We moved the repo, so every `/plugin install grove@linkuistics`
> breaks, right?
> **Expert:** No — the *marketplace*'s identity is its `name` field, not the repo
> URL. It was `linkuistics` in the old repo and it is `linkuistics` here. Only
> where you add the marketplace from changed.
> **Dev:** Then why didn't `guardrail` fire before that `rm -rf`?
> **Expert:** It's *user-invoked*. `disable-model-invocation: true` keeps its
> description out of context entirely, so nothing but you typing `/guardrail`
> can reach it — no *context load*, at the price of *cognitive load*: you have to
> remember it exists.
> **Dev:** And `coding-style-rust` is model-invoked *and* carries `paths:`?
> **Expert:** On Claude Code, yes — belt and braces. But `paths:` is a plugin
> auto-activation extension the Agent Skills spec doesn't define, so on codex,
> where it arrives by *symlink install*, the description is the only trigger.
> That's why the *description shape* rule keeps the capability clause.
> **Dev:** So `guardrail` was symlinked into codex all along, doing nothing?
> **Expert:** Worse than nothing — a codex session that loaded it read
> instructions about `${CLAUDE_PLUGIN_ROOT}` and `/guardrail` it had no way to
> act on. It now declares *harness eligibility* `[claude-code]` and the symlink
> install skips it, out loud.
> **Dev:** And a new skill I write without the key just goes everywhere?
> **Expert:** Nowhere, and you get told — the *skip-loudly default*. Silent
> either way would have been wrong: 16 of our 17 are portable, so
> install-everywhere would have mis-installed the one that matters and
> Claude-only would have withheld the sixteen that don't.
