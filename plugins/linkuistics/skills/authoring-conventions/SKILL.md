---
name: authoring-conventions
description: House authoring conventions for this marketplace's skills — a thin delta over superpowers' writing-skills covering our description-shape rule (capability + "Use when", overriding upstream's when-only), the user/model-invoked lever, source-citation, and progressive-disclosure thresholds. Use when authoring, auditing, or reshaping a SKILL.md in this repo; user-invoked, run it by hand.
disable-model-invocation: true
---

# Authoring conventions — house delta

This marketplace **depends on superpowers' `writing-skills`**: the superpowers
`SessionStart` hook injects `using-superpowers` into every session, and that routes to
`writing-skills` on demand. **Read `writing-skills` (and its bundled
`anthropic-best-practices.md`) for the craft** — TDD-for-skills, bulletproofing,
flowcharts, naming, file organization. This note records only where *our house* decides
differently, plus the few conventions we adopt; it does **not** duplicate the upstream
skill.

> Citation keys such as `superpowers-S3` identify the primary sources and
> adopted claims listed in [`../../PROVENANCE.md`](../../PROVENANCE.md).

## The one real override: description shape

`writing-skills` teaches **"description = when-to-use, NEVER what it does"** — the
*when-only* shape (`writing-skills/SKILL.md:99,150-158`). That rule is injected every
session, so it will actively tempt you to strip the leading capability clause from our
skills. **Don't.** Our corpus is reference/style guides where the capability *is*
load-bearing routing signal, so the house rule overrides upstream:

> **House rule.** A `description:` is a one-sentence **capability** + an explicit
> **"Use when …"** trigger clause — pushy enough to beat undertriggering — and
> **never** a step-by-step **workflow** summary.

Both halves come from different sources and both survive:

- **Keep upstream's core: never summarize the *workflow*.** A description that lists the
  skill's steps becomes a shortcut the agent takes *instead of* reading the body —
  superpowers measured exactly this: a description saying "code review between tasks"
  made an agent do *one* review where the skill's flowchart specified *two*
  (`superpowers-S2`). This is the survey's highest-convergence finding — seven of nine
  deep-dives reached it independently (`C1`: `gstack-S2`, `addyosmani-S4`, `hermes-S2`,
  `openclaw-S1`, `mattpocock-S4`, `anthropics-S2`).
- **Override the over-correction: a *capability* is not a workflow.** Upstream's
  "*never* what-it-does" is too strict for our niche. anthropics' own spec ships
  "what + when" descriptions (`anthropics-S2`); a capability stated as a **noun phrase**
  ("Rust coding standards", "drives GUI apps in isolated VMs") routes faster than
  triggers alone and carries no step sequence for an agent to shortcut.

**Capability vs workflow — the line that decides it:**

```yaml
# ✅ capability + Use when (house rule) — a content list is a capability, not a workflow
description: Rust coding standards — rustfmt, thiserror/anyhow, Tokio, workspace deps. Use when writing or refactoring Rust code.

# ❌ workflow summary — the agent follows these steps instead of reading the body
description: Use when reviewing code — first check the spec, then run the linter, then file findings.
```

A **content list** ("rustfmt, thiserror, Tokio") elaborates the capability and is fine.
A **step sequence** ("first … then … then …") is the forbidden shape.

**Byte budget.** Keep the whole frontmatter under the **1024-char** spec limit
(`anthropics-S1`); our descriptions run ≤~470. Ignore hermes' ≤60-char cap
(`hermes-S2`) — far too tight for a capability + trigger sentence.

## Conventions adopted from upstream (pointers, not copies)

Each is fully specified in `writing-skills` / `anthropic-best-practices.md`; here is only
our house position.

- **Match the form to the failure** (`superpowers-S3`, `addyosmani-S4`, `anthropics-S4`).
  Classify the baseline failure *first*, then pick the form: a **discipline** failure
  (knows the rule, skips it under pressure) → prohibition + rationalization table +
  red-flags; a **wrong-shaped-output** failure → a positive *recipe/contract* that states
  what the output IS, **not** a prohibition. The measured caveat: a "don't X" prohibition
  *backfires* on shaping problems — it trended worse than even the no-guidance control.
  Most of our skills are reference/style guides (neither failure mode), so they need
  neither apparatus.
- **Negation** (`mattpocock/skills` `writing-great-skills/GLOSSARY.md` § Negation).
  Generalises the bullet above: a bare prohibition drags the forbidden behaviour into
  context and makes it *more* available, not less — "don't write verbose comments"
  primes verbosity, the thing just read. Cure: state the positive target instead.
  Reserve `never`/`don't` for guardrails you truly cannot phrase positively, and even
  then pair it with the positive so attention lands on what to do.
- **Test the wording, cheaply** (`superpowers-S4`, `anthropics-S6`). For any
  behaviour-shaping wording, micro-test it against a **no-skill control** (5+ reps; if the
  control doesn't exhibit the failure, there is nothing to fix — stop). Reserve full
  subagent pressure-testing for genuine discipline skills; a reference/style guide does
  not need it.
- **Progressive disclosure** (`superpowers-S5`, `anthropics-S5`). Thresholds: a SKILL.md
  **body under ~500 lines**; a reference file over **~300 lines gets a table of
  contents**; references stay **one level deep**; and **never `@path` links** — `@`
  force-loads the file immediately, burning context before it is needed. Cross-reference
  other skills by **name**, not path. Today every one of our skills is correctly
  self-contained (only `cli-tool-design` splits a `references/` file, one level deep);
  this is the playbook for when one outgrows a single file.
- **Hunt no-ops sentence by sentence, not just line by line** (`mattpocock/skills`
  `aa7ed40`). Run the no-op test — does this change behaviour versus the
  no-skill default? — on each *sentence* in isolation; when one fails, delete the
  whole sentence rather than trim words from it. Be aggressive: most prose that
  fails the test should go, not be rewritten.

## House lever: user-invoked vs model-invoked

A skill is **model-invoked** by default — its `description` sits in context every session
so the agent can auto-fire it on a match. Eleven of our thirteen shipped skills are
model-invoked, which is **correct**: they are reference/style guides that *should*
auto-fire on a language or task match (`mattpocock-S3`).

A **hand-only** skill — an orchestrator, or house guidance like *this* note — should
instead set `disable-model-invocation: true`. It is then invokable only by a human typing
`/<name>`, is never auto-fired, and pays **zero** standing auto-load cost. This skill
dogfoods that (see its own frontmatter). Reach for the lever whenever a skill is
authoring/operator guidance a human runs deliberately, not a capability the agent should
reach for on its own.

**Name the two costs the lever trades** (`mattpocock/skills` `writing-great-skills/
{SKILL,GLOSSARY}.md`). A model-invoked skill spends **context load** — its description
sits in the window every turn, whether or not this session needs it. A hand-only skill
spends **cognitive load** instead — the human is the index, holding in their head which
hand-only skills exist and when to reach for one. At two hand-only skills (this note and
`guardrail`) that's still trivial; the cure once hand-only skills multiply past what's
easy to remember is a **router skill** — one user-invoked skill that names the others
and when to reach for each, never firing them itself (a user-invoked skill has no
description, so nothing but the human can reach it).

## House convention: cite sources, flag the unverified

When a skill embeds an external or framework fact (an API, a flag, a version-specific
behaviour), carry the discipline from `source-driven-development` (`addyosmani-S2`):
prefer the **authority hierarchy** (official docs > official blog/changelog > standards >
third-party), link the **deep anchor** (it survives doc restructuring better than a
top-level page), and when you cannot find a source, write **`UNVERIFIED`** explicitly
rather than implying confidence. This repo's `claude-api` skill plus the Context7 MCP are
the existing instances; the convention generalises them. Lifted as a convention — *not* a
standalone skill.

## Future work (out of scope for this note)

As the corpus grows, add **mechanical CI lints** rather than trusting reviewer vigilance:
a per-skill size-budget regression (`gstack-S3`), `skills-ref validate` spec-conformance
(`anthropics-S1`), a skill/command **name-collision** check across both plugins
(`wshobson-S5` — the one with a present-day hazard), and a manifest↔README invariant
(`mattpocock-S5`). Tracked as `[LINT]` in the survey synthesis; **not** part of this note.
