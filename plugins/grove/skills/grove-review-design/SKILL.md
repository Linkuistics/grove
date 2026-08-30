---
name: grove-review-design
description: The `review-design` session kind — a fresh-context, inspection-only adversarial read of one design artifact — a spec, an ADR set, or both, producing findings and no fixes. Use when a grove mandate names this skill, or when running a `review-design` session in a grove working tree.
harnesses: [claude-code]
---

# review-design

**Load the `grove` skill now** — on Claude Code, where plugin skills are
namespaced, that is `grove:grove` — and read `references/review.md` in it.

The five `review-*` kinds are one family with one procedure, and that file is
it: the goal, the deliverable, what a review may not do, and what each of the
five reads looks for. This skill states none of it a second time. Everything
else — the constraints, the bootstrap, and the execute, decompose, retire and
commit procedures — is the spine's.

Your artifact is the design artifact — a spec, an ADR set, or both produced by the `design` leaf this review
sits beside.
