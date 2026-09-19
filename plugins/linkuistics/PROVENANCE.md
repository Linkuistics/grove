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

### `codebase-design`

The skill distinguishes Ousterhout's reduction of design complexity through
interfaces and information hiding, Hickey's separation of independent concerns,
and Feathers' mechanism for substituting behaviour. Sources below were inspected
on 2026-09-19. The book's public extracts and the author's teaching material
support the Ousterhout guidance; this is not a claim to reproduce the full book.

| Key | Primary source | Contribution |
|---|---|---|
| `O1` | John Ousterhout, [Complexity and Dependencies](https://web.stanford.edu/~ouster/cgi-bin/cs190-spring16/lecture.php?topic=complexity) | Change amplification, cognitive load, unknown dependencies, and the cost of tactical design. |
| `O2` | Ousterhout, [Modular Design](https://web.stanford.edu/~ouster/cgi-bin/cs190-spring16/lecture.php?topic=modularDesign) | Interfaces include informal obligations; module depth, information hiding, useful generality, downward complexity, and alternative designs. |
| `O3` | Ousterhout, [A Philosophy of Software Design, second-edition extract](https://web.stanford.edu/~ouster/cgi-bin/aposd2ndEdExtract.pdf), chapters 6 and 21; [book page](https://web.stanford.edu/~ouster/cgi-bin/aposd.php) | General-purpose interfaces serving present needs, separating special-purpose policy, and identifying what matters. |
| `O4` | Ousterhout, [CS190 wrap-up slides](https://web.stanford.edu/~ouster/cs190-winter24/slides/wrapup.pdf) | Design warnings, documentation, and reducing avoidable error cases rather than multiplying caller handling. |
| `O5` | Ousterhout, [A Philosophy of Software Design vs. Clean Code](https://github.com/johnousterhout/aposd-vs-clean-code/blob/main/README.md) | Method length is not a design objective; comments carry information that code alone cannot express. |
| `H1` | Rich Hickey, [Simple Made Easy](https://www.infoq.com/presentations/Simple-Made-Easy/), Strange Loop 2011 | Simplicity versus ease, entanglement, state and time dependencies, composition, and abstraction. Publisher notes are summaries, not a verbatim transcript. |
| `H2` | Hickey, [Values and Change](https://clojure.org/about/state) | Values, identities, state over time, and explicit coordination. This supporting essay is distinct from the talk. |
| `F1` | Michael Feathers, [Working Effectively with Legacy Code: Seams](https://www.informit.com/articles/article.aspx?p=359417&seqNum=2) | A seam permits behaviour substitution without editing at that point. |
| `F2` | Feathers, [Seam Types](https://www.informit.com/articles/article.aspx?p=359417&seqNum=3) | Enabling points and concrete substitution mechanisms. |

The workflow, evidence table, deletion test, dependency categories, review
checklist, tradeoff rules, adapter vocabulary, and worked examples are Linkuistics
synthesis. They are not a joint method proposed by the three authors. In
particular, neither interface depth nor an immutable representation establishes
independence by itself, and testability is not a proxy for either.

The earlier skill also credited [`mattpocock/skills`](https://github.com/mattpocock/skills)
for its design-it-twice exercise. That inherited attribution is retained here;
the exact upstream snapshot is **UNVERIFIED**. The revised skill cites Ousterhout
for comparing designs and does not retain a required number of design agents.

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

### `decision-records`

The minimal template, examples, and three-part when-to-write test were adapted
from [`mattpocock/skills`](https://github.com/mattpocock/skills) at `b8be62ff`
(MIT). The minimum-coherent-set framing, in-place rework, and slug identity are
original to Linkuistics.

### `model-led-development`

Original to Linkuistics, and unusual in this file for having **no external prior
art at all**. Every rule in it is distilled from `docs/formalism-findings.md` in
this repository, across **two** modelling campaigns and in two passes.

- **Entries 001 – 025**, distilled by `formalism-skill-k38`: one workstream
  extracting a tree-on-disk library under two formalisms (Alloy 6.2 on its
  structure, Quint 0.32.0 on its operations), with the model written before each
  operation was implemented.
- **Entries 026 – 048 and the log's closing synthesis**, distilled by
  `model-led-development-k94`: Alloy 6 and Quint on the *same* behavioural
  questions about this repository's own protocols, 129 obligations across 258
  `(family, obligation)` cells, against an implementation already shipped and
  green. Its rules are cited to the adjudication in `docs/candidate-lessons.md`
  as well as to the log, because that document re-opened the model files and
  falsified or weakened three of the six claims the producing sessions believed —
  including one the skill had shipped as a caution.

The citation keys inside the skill are `[003]` for a log entry, `[synthesis]` for
the log's closing sections and `[c1]` – `[c5]` for the adjudicated candidates;
the log's two distillation notes record where each entry landed and what did not
survive. The named tools are third-party: [Alloy](https://alloytools.org/) and
[Quint](https://quint-lang.org/) are cited as instruments, not adapted from.

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
