# harness-hooks-k4

## Goal

Establish, from current documentation and source rather than memory, what each
harness offers for making the `technical-communication` skill always on: Claude
Code, Codex, Pi and Gemini. The survey is the evidence base for `skill-design-k5`
and for the four always-on leaves (`claude-code-always-on-k11`,
`codex-always-on-k12`, `pi-always-on-k13`, `gemini-always-on-k14`).

This is one survey, so it is a `research-a` leaf. It writes
`docs/research/harness-hooks-a.md`.

## Context

- Ground truth is the installed version of each harness. Run `claude --version`,
  `codex --version`, `pi --version` and `gemini --version`, read the installed
  package's own documentation and source where it is on disk, and then read the
  current upstream documentation. Say which version each claim is about.
  A project memory records that Pi's `~/.pi/rules/` is not loaded, verified in
  its `resource-loader.js`; check claims of that kind in the same way.
- How skills reach each harness today: Claude Code through the marketplace
  (`.claude-plugin/marketplace.json`, `plugins/linkuistics/.claude-plugin/plugin.json`);
  Codex through Grove's startup provisioning, which embeds a snapshot of
  `plugins/` in the binary (`docs/specs/codex-skill-provisioning.md`); and Codex,
  Gemini and Pi also through `plugins/install.sh`. The existing `guardrail` skill
  carries a skill-scoped frontmatter hook that works on Claude Code only.
- Codex's and Pi's global `AGENTS.md` files carry skills-priming sections that
  nothing in this repository writes. Gemini has no global instruction file. See
  `plan-k1`, "Always-on surfaces".

## Done when

The survey answers these questions for each of the four harnesses, with a
citation per claim, a walk-away check per harness (what still works with the
mechanism removed), and an explicit note wherever the search found silence.

For `skill-design-k5`:

1. Which hook or extension events exist: session start, before each user prompt,
   after a tool call or file write, end of a turn. What does each receive (tool
   name, file path, the assistant's last message, a transcript path), and what
   can it return (add context, give feedback to the model, block)?
2. Can a hook or extension be always on and shipped with an installed skill or
   plugin, or does it need the user to edit a configuration file? Where does
   that configuration live, and at what scope (user, project, plugin)?
3. Where does a harness read global instructions (`CLAUDE.md`, `AGENTS.md`,
   `GEMINI.md` or others), and can a plugin or skill contribute always-on
   context without editing the user's own file?
4. Do hooks also run in subagents and delegated sessions, so that text written
   for other models is covered?
5. How can one switch reach every layer: environment variable, flag file, a
   setting? What does each harness pass through to a hook?

For the four always-on leaves: the exact configuration schema, file paths,
install steps and known pitfalls for each harness, so an implementing session
does not rediscover them.

Bias the search toward what went wrong after real use: hooks that slow every
turn, hooks that loop or retrigger, payload or schema changes between versions,
silent failures, and sandboxes that block a hook's writes (a project memory
records that Codex's sandbox blocks gitdir writes). The fallback to beat is
"the skill tells the model to run the checker itself".

## Notes

- Read `references/research.md` in the `grove` skill for what a survey owes.
- Do not run a second survey; one corpus is enough for documentation reading.

## Decisions (running log)

**Versions of record.** Claude Code 2.1.302 (native binary), codex-cli 0.163.0
(Homebrew cask), Pi 0.84.2 (`@earendil-works/pi-coding-agent`, first on
`PATH`; an older `@mariozechner` copy in `/usr/local` is shadowed). Gemini CLI
is not installed: `gemini` is not on `PATH` and `~/.gemini` does not exist, so
`plugins/install.sh` skips it. Its version of record is the latest stable npm
release, read from the published package and upstream source, never run.

**Evidence gathering split by harness.** Four delegated readers, one per
harness, each wrote a cited evidence file in the session scratchpad; the survey
is written from those files, and every claim a later leaf's design rests on is
re-checked against its source before it goes into the survey. No model session
of any harness is started here: the live runs belong to the always-on leaves.

**A cross-harness always-on guide is externalised, not grown here.** The human
asked whether a skill-writing skill covers always-on mechanisms for several
harnesses. None does: `superpowers:writing-skills` assumes Claude Code,
`authoring-conventions` covers only the `harnesses:` key, and `plugin-dev` is
Claude Code only. At the human's request it is filed as
Linkuistics/grove#21, for a future grove that distils this survey and the
always-on leaves' live evidence once they are on `main`. The survey's handoff
names the issue; no leaf in this tree owns it.

**Gemini's scope is handed on, not asked here.** Gemini CLI's README says that,
for unpaid-tier and Google One users, it was replaced by Antigravity CLI on
2026-06-18, and it is not installed here. Whether the grove still covers it is
the human's call. `skill-design-k5` already asks the human about the switch,
the log and the harness mapping, so the survey's handoff puts the question
there with the evidence, beside the Codex trust route, which also touches the
human's own files. Antigravity CLI is not surveyed.
