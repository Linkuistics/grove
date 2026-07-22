# skill-design-k3

**Kind:** planning

## Goal

Digest `docs/research/jj-agent-prior-art.md` with the user and settle the
design of both skills, then grow the work leaves that build them.

## Context

Read the research report first; its **Synthesis for skill-design-k3** section
pre-answers much of the grilling. Decisions this leaf must land:

- **Adopt / adapt / write** — if a quality prior skill exists, adapt it
  (licence permitting) instead of writing from scratch.
- **Names** — canonical skill names for the workflow skill and the mapping
  skill (glossary working names; rename the CONTEXT.md entries once settled).
- **Triggers** — how each loads: the workflow skill by description on VCS
  work; the mapping skill on demand (user-invoked vs description-triggered).
- **Harness scope** — Claude Code only (cf. `guardrail`) or harness-neutral
  like the coding-style suite.
- **Native-workflow coverage** — which jj concepts the workflow skill must
  teach (working-copy-as-commit, `jj new`/`describe`, bookmarks, op-log undo,
  first-class conflicts, colocation etiquette), informed by the researched
  failure modes.
- **Reconciliation scope** — exact edits for `guardrail`, `decision-records`,
  `cli-tool-design`.

Then grow work leaves (expected: one per skill, one for reconciliation +
README/CHANGELOG) shaped as vertical slices.

## Done when

- Decisions logged inline; CONTEXT.md updated as terms resolve.
- Work leaves exist with briefs sharp enough to run AFK.
- Committed; this leaf retired.

## Notes

HITL leaf — needs the user present for the grilling. Settled root-level
semantics (jj-enabled → primary; else offer colocation once per session;
silent without jj) are in the root brief and CONTEXT.md — do not re-litigate
them unless research contradicts.
