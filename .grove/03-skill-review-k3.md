# skill-review-k3

**Kind:** review-impl

## Goal

Try to **disprove** `plugins/linkuistics/skills/using-codebase-memory/SKILL.md`.
Read it as a hostile reproducer: run every command it contains, and for each
sentence that makes a factual claim, ask what output would falsify it and go get
that output. Produce findings; change nothing.

## Context

Read the skill file **first and on its own**, before any grove document, and
form your expectations from it alone. It is going to be trusted by agents in
four harnesses who have no access to this grove — so the only fair test is the
one they will run.

Only afterwards, read `.grove/BRIEF.md` and
`docs/superpowers/specs/2026-07-29-portable-codebase-memory-skill-design.md` to
check coverage, and
`plugins/linkuistics/skills/authoring-conventions/SKILL.md` to check the
frontmatter against the house rule (capability + "Use when …", never a workflow
summary, frontmatter under 1024 chars).

**Fixture:** `Users-antony-Development-herdr`. Binary `codebase-memory-mcp`
0.8.1.

## Done when

Findings are written into this file under a `## Findings` heading, each one
naming the exact claim, the command run, the actual output, and whether it
**confirms** or **refutes**. Then this leaf is committed and retired.

Nothing in `plugins/` is edited by this leaf — corrections belong to
`skill-integrate-k4`. If there are no findings, say so explicitly; an empty
review is a real result.

## Notes

**The specific failure mode this chain exists to catch.** During scoping, the
plan's Global Constraints were found to assert that passing
`relationship`/`direction` alongside `min_degree` "produces byte-identical
results". The commands had been run. The projection the author checked
(`.results[]`) *did* match. The prose was still false — `.total` differs (2460 vs
2458). So: **a passing command is not a verified claim.** Check the claim's
actual scope, not the narrowed view that made it look true.

Highest-yield checks, in rough order:

- Every claim of *equality* or *identity* between two invocations — compare the
  whole payload, not a projection.
- Every exact number, name, or count. The fixture drifts (23,641/97,504 when the
  plan was written; 23,681/97,906 at scoping time).
- The exit-code claims — run them **without** a pipeline, then with one.
- Whether the stated stream split (JSON→stdout, logs→stderr) survives the exact
  pipe form the file documents.
- Coverage: all eight of the spec's "Skill content" items, and the fourteen tool
  names spelled correctly.
- Anything asserted about a harness other than this one, which cannot be tested
  here and therefore must be sourced or marked `UNVERIFIED`.
