# skill-router-k21

**Integrates:** skill-router-k20

## Goal

Restore the durable-artifact rule that the router rewrite deleted, and make the
condition-register budget fail when a trigger identity or an `own` row's
structure is lost. Preserve the 900-word ceiling, the 26-trigger grammar, and
licensed rewording.

## Context

- `content/SKILL.md`
- `content/references/grove.md`
- `docs/specs/corpus-rule-ownership.md`
- `tests/methodology.rs`
- `.grove/13-DONE-review-impl-skill-router-k20.md`

The four rule moves called out by the producer landed whole, and their procedure
registers do not repeat their triggering conditions. The five repointed test
files preserve the claims they moved. The findings below are the actionable
residue of the 26-trigger and budget audit.

## Findings

### P1 — the durable-artifact trigger leads to an owner that no longer states the rule

`content/SKILL.md:109` now reduces `durable-artifact-set` to a trigger for
`references/grove.md`, but that owner discusses glossary rationale and spec grain
without giving the inventory's four-member mapping — glossary, ADR set, spec set,
task tree — or saying which members outlive the grove
(`content/references/grove.md:31`). The deleted artifact table in the producer's
parent was the only complete statement. The spec's claim that only four rules
needed moving is therefore false (`docs/specs/corpus-rule-ownership.md:462`).

A session deciding whether material belongs in a brief or a durable record now
opens the named owner and never learns that the task tree is ephemeral while the
other three sets outlive it. It can leave durable output under `.grove/`, where
the finish cycle deletes it. Restore the complete rule in its owner without
copying the trigger's condition into the procedure register, and reconcile the
spec's move audit.

### P1 — counting trigger-shaped bullets does not protect the trigger set

The budget test asserts only that `trigger_sentences(body).len() == 26`
(`tests/methodology.rs:302`) and that each collected bullet is at most 25 words
(`tests/methodology.rs:317`). It never checks uniqueness, the 26 situations, or
their owner paths. Replacing sentence 18 with a second copy of sentence 19 still
has 26 short `When` bullets and remains under 900 words, so this test stays green;
no other test names `durable-artifact-set`. A session then never asks where a
durable artifact belongs and can make the loss above permanent at teardown.

Make the assertion identify every canonical trigger row (while retaining the
design's permission to reword within the trigger grammar), and add a control
where one trigger is replaced by another rather than merely removed.

### P1 — the `own` assertion ignores the structure that makes several rows rules

`missing_phrases` flattens the whole body and checks unordered substring
occurrence (`tests/methodology.rs:452`). In particular, `bootstrap-order` is only
six phrases with no order assertion (`tests/methodology.rs:391`), and the routing
row checks neither “before you act” nor “select nothing here”
(`tests/methodology.rs:375`). A rewrite can put the task file before the glossary
and cited ADRs, or let a session begin work before reading its kind reference,
while every phrase remains somewhere in the body and the budget stays green.
That session bootstraps against unqualified task prose or applies no
kind-specific discipline — exactly the failure these two `own` rows exist to
prevent.

Assert each `own` row's meaningful structure, not body-wide word presence:
ordering for Bootstrap and the before-work/no-selection obligation for routing.
Keep semantic alternatives or scoped predicates so a legitimate rewording does
not become the only way to make the check fail.

## Confirmed verdicts

- The 26 current trigger sentences preserve the inventory's situations and
  point to the prescribed eventual owners. Rules deliberately awaiting
  `loop-step-references-k11`, `corpus-split-k6`, or `plugin-fallback-k9` still
  have a statement somewhere in `content/`; the durable-artifact rule above is
  the exception.
- `one-focused-commit`, its Retire-first reason, `node-close-is-implicit`, the
  silent cascade, and the agent-side half of `pruning-is-hitl` landed whole in
  `references/commit.md` and `references/retire.md`. The pruning owner states the
  obligation without restating the condition.
- The changes in `commit_guidance`, `composition_guidance`,
  `lifecycle_invariants`, `prompt`, and `retire_guidance` relocate their claims
  without weakening them. No further finding is required there.

## Done when

- `references/grove.md` states the complete durable-artifact mapping, and the
  spec no longer claims an exhaustive move audit that omitted it.
- Replacing one canonical trigger with a duplicate or unrelated short `When`
  bullet makes the methodology suite fail by the missing rule's name.
- Reordering Bootstrap's reads, dropping the routing-before-work clause, or
  dropping the no-selection clause makes the `own` assertion fail, while
  semantically equivalent wording remains supportable.
- `content/SIGNAL.md`, `content/SIGNAL-FINISH.md`, and `src/prompt.rs` remain
  unchanged.

## Notes

- This review ran no test, build, lint, or format command and edited no reviewed
  artifact. Its only writes are this integration leaf and the review leaf's
  retirement.
