# skill-k2

**Kind:** impl

## Goal

Write `plugins/linkuistics/skills/using-codebase-memory/SKILL.md` complete in one
session — frontmatter, the CLI contract, the `min_degree` gotcha, and the
composition patterns — and execute every command it documents against the live
graph before committing.

This merges the plan's Task 1 and Task 2, which both write this one file. See
`.grove/BRIEF.md` § Decomposition for why.

## Context

Read in this order:

1. `.grove/BRIEF.md` — the verified CLI contract, **including the `min_degree`
   correction below**, which supersedes the plan's wording.
2. `plugins/linkuistics/skills/authoring-conventions/SKILL.md` — **the house
   delta, and the governing authority for frontmatter**. The plan cites
   `superpowers:writing-skills` and does not mention this file; read both, and
   where they disagree the house delta wins.
3. `docs/superpowers/plans/2026-07-29-using-codebase-memory-skill.md` — Tasks 1
   and 2 carry the proposed file content near-verbatim and the runnable
   verification steps. Take the content as a strong draft, not as dictation.
4. `plugins/linkuistics/skills/using-jujutsu/SKILL.md` — nearest prior art for
   house voice and shape (264 lines, single file, no `references/`).

**The description must keep its leading capability clause.** The house rule is
capability + "Use when …"; upstream's when-only rule is injected every session
and will tempt you to strip it. Frontmatter stays under 1024 chars.

## Done when

- The file exists at
  `plugins/linkuistics/skills/using-codebase-memory/SKILL.md`, covering all
  eight items of the spec's "Skill content" section.
- **Every command in the file has been run and its output compared to what the
  prose claims.** Where they disagree, the file is wrong — fix the file, not the
  expectation.
- Committed with `jj describe` / `jj new` (load `linkuistics:using-jujutsu`
  first; this repo is jj-colocated and git is read-only here).

## Notes

**The `min_degree` claim the plan gets wrong.** Verified this session against
0.8.1 on `Users-antony-Development-herdr`:

```
min_degree:10, label:Function, limit:5             → total=2460
  + relationship:CALLS, direction:inbound          → total=2458
results[] identical in both; both include  in=0 out=11 above_pane_sets_autoscroll_up
```

So the *substantive* gotcha holds — `min_degree` gates on `in + out`, and adding
`relationship`/`direction` does **not** make it directional, which is why "find
high fan-in functions" is not a single call. But the plan's phrasing ("produces
byte-identical results", "the results are byte-identical with or without them")
is **false**: `.total` differs. Write the precise claim — the *filter semantics*
are unchanged, the responses are not — or the skill ships a fact that does not
survive its own reproduction.

**Verify without a pipeline.** The plan's own caveat bit during scoping: `$?`
after a pipe reports the last stage. Capturing output through `$(…)` and
re-echoing it into `jq` can also fail in ways the direct pipe does not — the
direct pipe *is* clean (checked: one line, zero control characters), so test the
form the skill actually documents.

**Fixture drifts.** `herdr` was 23,641/97,504 when the plan was written and is
23,681/97,906 now. Exact counts do not belong in the skill. The one exact value
the plan asserts — the fan-in table topped by `123 app_for_mouse_test` — still
held this session, but treat any exact figure as a re-check, not a given.
