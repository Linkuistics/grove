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
