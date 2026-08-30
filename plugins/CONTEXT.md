# skills

The `grove`, `linkuistics` and `testanyware` skill plugins under `plugins/` —
the language of **authoring, packaging, triggering and installing** a skill.

**Scope boundary.** A skill's own *subject matter* — jj's working-copy-as-commit,
testanyware's VM vocabulary — belongs to that skill and its ADRs, not here. This
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
and a `skills/` directory — and a Claude Code concept only; other harnesses receive
the same [[Skill]] directories by [[Symlink install]]. Three ship here: `grove`,
`linkuistics` and `testanyware`.

**Marketplace**:
`.claude-plugin/marketplace.json`, the catalogue a user adds once to reach every
[[Plugin]] in it. **Its identity is the `name` field, not the repo URL** — already
`linkuistics` while the tree lived in `Linkuistics/skills` — so every
`linkuistics:<skill>` reference is repo-independent and a repo move rewrites only
where the marketplace is added from.
_Avoid_: treating the repository URL or owner as the namespace.

### Invocation and triggering

**Namespaced invocation** (`<plugin>:<skill>`):
How Claude Code addresses a [[Skill]] that arrived inside a [[Plugin]] —
`linkuistics:using-jujutsu`, `grove:grove`. A [[Symlink install]] has no
namespace, so the same skill is the bare directory name there. Two consequences.
A plugin skill is **addressed differently** from a personal skill of the same
name, which is why `grove:grove` coexists with the `grove` the binary provisions
into `~/.claude/skills/`. Claude Code's plugins reference states the namespacing;
it does not state a collision rule, so that consequence rests on the namespacing
plus the observed case. And a prompt that names a skill for a session to load names
**one target in two spellings** rather than branching on the harness — the
session uses whichever its own offers.
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
`authoring-conventions` and `guardrail` pull it.
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
activation channel is the `description`. Carried by the `coding-style*` skills and
inert wherever they arrive by [[Symlink install]].
_Avoid_: assuming a `paths:`-triggered skill fires on a spec-only harness.

**Hook-carrying skill**:
A [[Skill]] whose frontmatter declares Claude Code `hooks:`, so activating it
installs tool-call interception for the session. Only `guardrail` does — a
`PreToolUse` gate that asks rather than denies. Claude Code only; the block is a
no-op elsewhere.

### Authoring

**House delta**:
The `authoring-conventions` [[Skill]] records only where this corpus decides
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

**Symlink install** (`plugins/install.sh`):
The delivery path for harnesses that read `SKILL.md` but have no [[Plugin]]
mechanism: symlink each **eligible** skill directory into that harness's personal
skills folder (`~/.codex/skills`, `~/.gemini/skills`, `~/.pi/agent/skills`).
Because the targets are links, a `git pull` updates content in place — re-run only
when skills are added, removed, or change their [[Harness eligibility]]. It scans
**every** bundled plugin: which harnesses a skill reaches is the skill's own
declaration, not a property of the directory it ships in. A real file or directory
already at a target path is **left untouched and refused** — the run reports it,
still installs every other skill, and exits non-zero, because a skill silently not
installed is indistinguishable from one installed successfully.
_Avoid_: running it for Claude Code, which installs by [[Marketplace]]; reading it
as append-only — a re-run also **reconciles** (see [[Link reconciliation]]).

**Harness eligibility** (`harnesses:`):
The frontmatter key by which a [[Skill]] declares which harnesses it can be
installed into — a YAML flow list, either the single claim `[any]` or an explicit
allowlist of harness ids (`claude-code`, `codex`, `gemini`, `pi`). It answers one
question: *can a session on this harness follow these instructions?* `guardrail`
declares `[claude-code]` because its whole mechanism is a Claude Code
[[Hook-carrying skill]] frontmatter block, and the `grove` spine declares it
because the binary still owns `~/.codex/skills/grove` and `~/.pi/agent/skills/grove`
until provisioning is deleted; every other bundled skill declares `[any]`. Frontmatter is a safe carrier because a conforming loader parses the
block into a map and reads only the keys it knows — verified by reading pi's
`core/skills.js`, and already relied on by `paths:` and `hooks:` reaching codex
today. Gemini CLI is the one target not checked directly (it is not installed
here, so nothing is symlinked there either); the standard's own extension keys
are the precedent it would be violating.
_Avoid_: reading `any` as an enumeration of today's harnesses — it is a claim
about the *skill*, that nothing in it depends on a particular harness's
affordances, so a harness added later inherits it correctly.

**Skip-loudly default**:
What [[Symlink install]] does with a [[Skill]] carrying no `harnesses:` key: install
it nowhere, and print a note naming it. Chosen from a measurement rather than by
instinct — 15 of the 16 bundled skills are portable and 1 is Claude-only, so a
silent *install everywhere* would mis-install exactly the skill that cannot work,
while a silent *Claude Code only* would withhold the other 15. Skipping never
mis-installs and the note removes the silence; since every bundled skill declares a
key, it fires only on a newly authored one. The install still exits 0 — one
under-annotated skill must not block the rest — and the hard assertion that every
bundled skill declares a key lives in `install.test.sh`.
_Avoid_: calling it a conservative *default*; it is a conservative default plus the
report that makes the default observable, and the report is the half that was
missing.

**Personal-setup assumption** (`assumes-personal-setup: true`):
The frontmatter key by which a [[Skill]] declares that its content names the
author's own models, subscriptions, or machine configuration — so it is correct
here and misleading for anyone else. Orthogonal to [[Harness eligibility]]: it does
not filter the install (on the author's own machine the assumption holds), it makes
the install **report** the skill. Only `doubt-driven-development` carries it, for
`references/harness-spawns.md`.
_Avoid_: overloading it onto a skill that merely needs a tool installed — a missing
`testanyware` or `codebase-memory-mcp` binary is a dependency, not a private
assumption.

**Link reconciliation**:
[[Symlink install]]'s removal half: on every run it drops each symlink of its own
that the run will not re-create, so a skill deleted, renamed, or newly ineligible
for that harness leaves nothing behind. Ownership is decided by the link's stored
target matching `…/plugins/<plugin>/skills/…` for a plugin this repo ships — the
**plugin** segment, not the skill name, because a deleted skill's name is by
definition no longer shipped and keying on it would disown exactly the links most
in need of reclaiming.
_Avoid_: expecting it to touch a symlink this script did not place; and the older
advice to delete dangling links by hand, which this replaces.

**Workspace guard**:
`plugins/install.sh`'s refusal to run from anywhere but the repo's **main checkout**,
since it links from the tree it lives in and re-links *every* skill
unconditionally — so a linked git worktree or secondary jj workspace would
capture the whole install and dangle it on teardown. The failure is otherwise
unobservable: a symlink to a vanished target reads as "skill not installed", not
as an error. `--force` opts in deliberately. The probe is jj-first, sharing the
[VCS seam](../docs/ARCHITECTURE.md#symmetric-vcs-rule) with the `grove` binary.
_Avoid_: reading it as a jj feature — a linked `git worktree` trips it identically.

## Flagged ambiguities

**"skill"** means two different things in this repo. In this context it is a member
of the corpus under `plugins/*/skills/`, shipped by [[Marketplace]] or
[[Symlink install]]. But grove's methodology is *also* provisioned as a skill
(`~/.claude/skills/grove/`, written by the `grove` binary from `content/`) while
being no part of this corpus. Qualify as "a marketplace skill" versus "grove's
skill" when the distinction matters; see `CONTEXT-MAP.md`.

## Example dialogue

> **Dev:** We moved the repo, so every `linkuistics:coding-style-rust` reference
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
