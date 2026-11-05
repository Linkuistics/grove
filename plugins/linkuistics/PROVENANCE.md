# Linkuistics skill provenance

The skills use independent prose, but several disciplines and examples were
adapted from public prior art. This file keeps the claim-level source keys used
inside `SKILL.md` files without retaining the large point-in-time survey that
originally discovered them.

## Authoring conventions

The following source keys refer to material inspected on 2026-06-25 unless a
revision is stated. Links point to the primary repositories or specifications.

| Key prefix | Primary source | Snapshot and licence | Adopted idea |
|---|---|---|---|
| `superpowers-S*` | [`obra/superpowers`, `writing-skills`](https://github.com/obra/superpowers/tree/main/skills/writing-skills) | installed v6.0.3; MIT | Descriptions must not summarize a workflow; match the document shape to the behavioral failure; test trigger wording; use progressive disclosure. |
| `gstack-S*` | [`garrytan/gstack`](https://github.com/garrytan/gstack) | `main`, inspected 2026-06-25 | Factor shared skill material, keep descriptions terse, track size budgets, and use a hand-enabled safety mode. |
| `addyosmani-S*` | [`addyosmani/agent-skills`](https://github.com/addyosmani/agent-skills) | `main`, inspected 2026-06-25; MIT | Fixed verification anatomy, source-driven claims, and doubt-driven adversarial review. |
| `hermes-S*` | [`NousResearch/hermes-agent`](https://github.com/NousResearch/hermes-agent) | `main`, inspected 2026-06-25; MIT | Concise routing descriptions and keeping mutable state out of frontmatter. |
| `openclaw-S*` | [`openclaw/openclaw`](https://github.com/openclaw/openclaw) | `main`, inspected 2026-06-25; repository licence was not asserted by the survey | Description discipline and content-hash refresh markers; ideas only, no copied files. |
| `mattpocock-S*` | [`mattpocock/skills`](https://github.com/mattpocock/skills) | `d574778` (v1.1); MIT | User-invoked versus model-invoked skills, progressive disclosure, pruning by behavioral value, and manifest/documentation invariants. |
| `anthropics-S*` | [Agent Skills specification](https://agentskills.io/specification) and [`anthropics/skills`](https://github.com/anthropics/skills/tree/main/skills/skill-creator) | live specification plus `main` inspected 2026-06-25; `skill-creator` carries its own licence | “What and when” descriptions, frontmatter constraints, progressive disclosure, and trigger evaluation. |
| `wshobson-S*` | [`wshobson/agents`](https://github.com/wshobson/agents) | `main`, inspected 2026-06-25; MIT | Keep one canonical source, generate consumer indexes, and detect cross-plugin name collisions. |

The house rule deliberately combines two sources: preserve Superpowers' ban on
workflow summaries, while following the Agent Skills specification's
capability-plus-trigger shape.

## Individual skills

The sections of `codebase-design`, `decision-records` and
`model-led-development` moved to `plugins/design/PROVENANCE.md` in
[DesignSkills](https://github.com/Linkuistics/DesignSkills), with those skills.
The section of `doubt-driven-development` moved to
`plugins/engineering/PROVENANCE.md` in
[EngineeringSkills](https://github.com/Linkuistics/EngineeringSkills), with
that skill.

### `writing-code-walkthroughs`

The method was synthesized in this repository from two independent surveys and
then exercised by the complete, technically reviewed, and editorially reviewed
`ordinal-fs-tree` book. Its external foundations are primary or authoritative
sources: Reigeluth and Stein, Pennington, and Letovsky and Soloway for
whole-to-parts concept ordering; [Sweller and Cooper](https://doi.org/10.1207/s1532690xci0201_3)
plus [Chi et al.](https://doi.org/10.1207/s15516709cog1302_1) for worked
examples and self-explanation; Ayres and Sweller plus Letovsky and Soloway for
local integration and delocalized-plan goals; CWEB and Ramsey's noweb paper for
named recursive source expansion; RFC 7322, Google's developer-documentation
guide, and WCAG for direct prose, headings, links, and navigation. The exact
fragment grammar, byte-comparison contract, scoped-progress protocol, and
separate technical/editorial review briefs are Linkuistics' project synthesis;
the skill does not claim controlled reader-outcome evidence for them. The
[behavioral evaluation campaign](../../docs/evaluations/writing-code-walkthroughs/README.md)
did not establish its predeclared acceptance rubric; the report distinguishes
the retained judged outputs from the stronger deterministic checks.
The proposed replacement campaign did not run: the behavioral-acceptance
conjunct is closed as unmet after no inspected hosted provider documented the
required final-boundary attestation and controlled self-hosting was declined.
The current decision and its reopening boundary are recorded in
[`behavioral-acceptance-is-closed-unmet`](../../docs/adr/behavioral-acceptance-is-closed-unmet.md).

### `technical-communication`

The skill is modelled on ASD-STE100, Issue 9. Its
rules are restated in our own words from the standard's Part 1. The standard is
licensed to the maintainer and may not be redistributed, so nothing in this
repository quotes it: no sentence, no rule title, no example and no dictionary.
Every example is invented, about software and model work. The skill names the
standard only to say what it is modelled on, and it does not claim to conform
to it. The decision is
[technical-communication carries no text from the standard](../../docs/adr/technical-communication-carries-no-standard-text.md).

The checker, `skills/technical-communication/scripts/ste-lint.py`, is vendored
code.

| Item | Detail |
|---|---|
| Upstream | [`danyuchn/asd-ste100-skill`](https://github.com/danyuchn/asd-ste100-skill), file `scripts/ste-lint.py` |
| Commit | `3944283c183eea13fe5bd902b572da3b4de8bf44` |
| Licence | MIT, copyright (c) 2026 Dustin Yuchen Teng. The notice is `skills/technical-communication/LICENSES/danyuchn-asd-ste100-skill.LICENSE`, copied unchanged from upstream at that commit |
| Entered | Unchanged in `vendored-checker-k3`, apart from an attribution comment that names the commit |

What we changed, after the checker came in unchanged. Every changed or added
hunk in the script carries a comment that starts with `ours:`, and code without
one is as upstream wrote it. The spec section *The checker's contract* in
`docs/specs/technical-communication.md` states the result.

- **Whole-document checks.** Rules for what counts as prose in Markdown, front
  matter, commit messages and plain text; British spelling by suffix patterns,
  with no dictionary; and the `tc:` markers that record a departure.
- **Reply scoring.** The `--log-start` and `--log-reply` modes, which append
  counts to the reply log and store no reply text.

Only the checker came from upstream. The upstream skill text, its word lists
and its rule wording were not adopted. The hook core (`scripts/tc_hook.py` and
`scripts/tc-hook`), the harness entries, the skill text and its reference files
are original to this repository. The survey of what each harness allows is
`docs/research/harness-hooks-a.md`.

### `guardrail`

Adapted in independent prose from
[`garrytan/gstack`](https://github.com/garrytan/gstack)'s `careful`, `freeze`,
and `guard` class on `main`, inspected 2026-06-25. Linkuistics composes those
ideas into one hand-invoked skill and implements its hook independently.

### `using-jujutsu`

Command facts were verified against Jujutsu 0.43.0. The skill combines
independently written guidance from sources inspected 2026-07-22:

| Source | Licence | Contribution |
|---|---|---|
| [`danverbraganza/jujutsu-skill`](https://github.com/danverbraganza/jujutsu-skill/blob/main/jujutsu/SKILL.md) | MIT | Native workflow and agent hygiene. |
| [`RealAdarsh/jj-skill`](https://github.com/RealAdarsh/jj-skill) | MIT | Git-read-only policy in colocated repositories. |
| [Carbon's Jujutsu skill](https://github.com/carbon-language/carbon-lang/blob/trunk/.agents/skills/jj/SKILL.md) | Apache-2.0 with LLVM exception | Symmetric Jujutsu/Git detection. |
| [`muloka/claude-plugins`](https://github.com/muloka/claude-plugins/blob/main/plugins/project-setup-jj/templates/CLAUDE.md.template) | Apache-2.0 for the plugin directory | Working-copy-as-commit framing. |
| [`kawaz/claude-plugin-jj`](https://github.com/kawaz/claude-plugin-jj) | MIT | Guard-hook architecture, reimplemented rather than copied. |
| [`causes-tracker` failure catalogue](https://github.com/causes-tracker/causes-tracker/blob/master/.claude/skills/jj/references/jj-for-claude.md) | no licence stated | Factual catalogue of common agent failures; no prose copied. |
