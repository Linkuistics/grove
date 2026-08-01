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

### `decision-records`

The minimal template, examples, and three-part when-to-write test were adapted
from [`mattpocock/skills`](https://github.com/mattpocock/skills) at `b8be62ff`
(MIT). The minimum-coherent-set framing, in-place rework, and slug identity are
original to Linkuistics.

### `doubt-driven-development`

Adapted in independent prose from
[`addyosmani/agent-skills`](https://github.com/addyosmani/agent-skills)'
`doubt-driven-development` on `main`, inspected 2026-06-25 (MIT). The
fresh-context and do-not-bias-the-reviewer rules were cross-checked against
Superpowers, gstack, and wshobson.

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
