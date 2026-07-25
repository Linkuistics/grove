# adr-set-k8

**Kind:** work

## Goal

Bring the merged ADR set back to a **minimum coherent set describing the current
design**, and settle where a two-context repo's ADRs live.

## Context

The merge brings `docs/adr/symmetric-vcs-rule.md` alongside this repo's existing
slugs (`cli-binary-split`, `codex-gitdir-grant`, `do-is-sole-lifecycle-verb`,
`fresh-grove-start-contract`, `in-session-finish-cycle`, `model-per-task-kind`,
`pruning`, `self-driving-loop`, `self-extension-core-and-methodology`,
`task-kind-taxonomy`, `task-tree-scheme`, `user-owned-worktrees`) plus
`skills-monorepo` from planning. Use `linkuistics:decision-records`: rework in
place — merge, split or delete — never append a superseding record.

`self-extension-core-and-methodology` is the one most likely to need editing: it
describes the distribution model, and while this merge does not *change* that
model, it changes what else lives in the repo alongside it.

**The open decision.** `content/ADR-FORMAT.md` says that in a multi-context repo —
one carrying a root `CONTEXT-MAP.md`, which this repo now does — system-wide
decisions stay in the root `docs/adr/` while **context-specific** ones go under
that context's own `docs/adr/`. By that rule `symmetric-vcs-rule` belongs at
`plugins/docs/adr/symmetric-vcs-rule.md`, and `skills-monorepo` is arguably
system-wide. Decide it explicitly: split per the rule, or keep one flat set and
amend `content/ADR-FORMAT.md` to say when a flat set is right.

Weigh it on the evidence rather than the rule's letter — the set is 14 records for
one maintainer, `CONTEXT-MAP.md` already asserts a slug is unique repo-wide, and
`content/ADR-FORMAT.md` is itself grove methodology this repo is free to amend.

## Done when

- The placement decision is made and enacted — either the split, or a flat set
  with `content/ADR-FORMAT.md` amended to state when flat is right. Either way
  `CONTEXT-MAP.md`'s third relationship ("both sit in the root `docs/adr/` today")
  reads as settled rather than provisional.
- The set passes the minimum-coherent-set test: no two ADRs describing the same
  decision, none describing a design that no longer exists.
- No dangling citation. Grep every ADR slug across `docs/`, `content/`,
  `README.md`, `plugins/`, `src/` and `.grove/`, and fix what the rework breaks.

## Notes

- If the decision goes to a split, the `grove` binary embeds `content/` — check
  whether any provisioned methodology file names a moved path.
- Citation counts before the pass: `task-tree-scheme` 61, `pruning` 45,
  `model-per-task-kind` 40, `task-kind-taxonomy` 34, `self-driving-loop` 26,
  `user-owned-worktrees` 24. Those are raw grep hits including prose uses of the
  word, not citations — but they size the sweep.
- Out of scope: `README.md` and `CHANGELOG.md` prose (`01-prose-reconciliation-k7`),
  except citations this leaf's rework breaks.
