# skills

The `linkuistics` and `testanyware` skill plugins under `plugins/` — the
language of **authoring, packaging, triggering and installing** a skill.

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
the same [[Skill]] directories by [[Symlink install]]. Two ship here: `linkuistics`
and `testanyware`.

**Marketplace**:
`.claude-plugin/marketplace.json`, the catalogue a user adds once to reach every
[[Plugin]] in it. **Its identity is the `name` field, not the repo URL** — already
`linkuistics` while the tree lived in `Linkuistics/skills` — so every
`linkuistics:<skill>` reference is repo-independent and a repo move rewrites only
where the marketplace is added from.
_Avoid_: treating the repository URL or owner as the namespace.

### Invocation and triggering

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
the **repo** and not the subdirectory — so both plugins here report one shared version
string. It is the **cache key** an update is decided on, and it moves with every commit,
so every push delivers. Both plugins use it deliberately
(`docs/adr/skills-monorepo.md`); the alternative, an explicit semver, *pins* — a skill
change shipped without a bump reaches no consumer and raises no error.
_Avoid_: adding a `version` to silence `claude plugin validate --strict`, which pins
delivery on a bump nothing in this repo can check; reading the SHA as identifying a
plugin's own content, when it identifies the repo's.

**Symlink install** (`install.sh`):
The delivery path for harnesses that read `SKILL.md` but have no [[Plugin]]
mechanism: symlink each skill directory into that harness's personal skills folder
(`~/.codex/skills`, `~/.gemini/skills`). Because the targets are links, a `git pull`
updates content in place — re-run only when skills are added or removed.
_Avoid_: running it for Claude Code, which installs by [[Marketplace]].

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
