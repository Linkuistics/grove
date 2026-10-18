# technical-communication-k2

## Goal

Decompose the building and publishing of the `technical-communication` skill
into leaves, each one focused session, in dependency order, so that the root
brief's done-criteria hold when the last of them retires.

## Context

- The requirements are settled: read `plan-k1`'s running log in full before
  cutting anything. Its decisions are the human's.
- The "how" questions the requirements left to design, each of which needs a
  home in the tree:
  - which hook events each harness offers — Claude Code, Codex, Pi and Gemini —
    and how the always-on instruction reaches each one. Codex's and Pi's global
    `AGENTS.md` files carry skills-priming sections maintained outside this
    repository, and Gemini has no global instruction file;
  - the switch, the reply log's location and contents, and how the checker
    accepts a justified departure;
  - how the checker is vendored and extended: limits per text type, a paragraph
    check, British spelling, whole-file checking;
  - the fallback for documents too large to convert in one session;
  - whether grove's "Human-facing communication" section
    (`plugins/grove/skills/grove/SKILL.md`) is reworded to sit under the
    precedence order;
  - the ADRs the root brief requires.
- Check per-harness hook support against each harness's current documentation
  or source, not from memory. It may earn a research leaf of its own, ahead of
  the design.

## Done when

- The tree holds the leaves that take the skill from nothing to released, each
  with its kind, goal and done-criteria, in dependency order.

## Notes

- The human wants to develop, publish and use the skill, then iterate in later
  groves. Keep the plan lean, with no test or evaluation leaves.


## Decisions (running log)

**Working increments — flat leaves, ordered by what each one needs.** The seams
that leave the product working are: the vendored checker alone; the researched
harness surfaces and the design that uses them; the two checker extensions, one
per consumer (document gate, reply log); the skill's text in three parts, the
first of which creates a loadable skill; one always-on leaf per harness; grove's
own communication section; and publication. The checker is vendored first
because nothing about it depends on the design, and the design can then read
real code. No node directories were made: no group needed a brief of its own, so
constraint 4 says not to write one.

**One survey, not a pair.** Reading each harness's current documentation is not a
contested question, so `harness-hooks-k4` is a lone `research-a`, as the human's
lean-plan note asks.

**Skill text is split by the standard's sections, in three leaves.** Sections 4
to 6 come first together with the cross-cutting rules and the always-on text,
because that is the smallest slice that makes a working skill. Sections 1 to 3
and 7 to 9 follow, twenty-three and twenty-two rules each. Each leaf may
decompose if its reading proves bigger than one session.

**Claude Code goes first among the harnesses.** It is where the experiment runs,
it has the richest hooks, and it builds the shared scripts and the switch that
the other three reuse.

**No review chain cut now.** The tree is lean by the human's choice. The design
leaf's brief says when to weigh a `review-design`, and the skill-text leaves say
when a fresh read for overlap with the standard is worth it.

**No release or integration leaf.** `CLAUDE.md` gives them to the finish cycle,
which the driver appends once no ordinary leaf is live.

**Fog.** What the reply log shows, and how the rules change because of it, is
not a question anyone can state yet. It stays outside this grove: the human
judges the skill in use, logs failures as GitHub issues on `Linkuistics/grove`
and iterates in later groves.
