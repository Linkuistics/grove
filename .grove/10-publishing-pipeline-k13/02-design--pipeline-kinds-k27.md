# pipeline-kinds-k27

## Goal

Decide, against the preregistered decision rule and the pilot's report, which
editorial stages become session kinds; name them; and state each one's
discipline, its deliverable, its HITL/AFK mark and how it hands off to the next.

## Context

- The decision rule is `pilot-preregistration-k24`'s and the evidence is
  `measurement-report-k45`'s, under the `pilot-measure-k26` node. Apply the rule; do not re-derive a better one. A stage
  the report leaves genuinely undetermined is not evidence for keeping it.
- The candidate stages: draft, developmental edit, technical edit, copy edit,
  art, proof (decision 11 of `plan-k1`). The alternative they had to beat: draft
  plus proof.
- `docs/adr/a-kind-is-an-open-token.md`: a kind is any well-formed token and
  exists iff a `grove-<kind>` skill exists. So this leaf is choosing tokens and
  disciplines, and nothing in the binary constrains the choice — but every chosen
  token becomes a configuration key the human must declare before any leaf of
  that kind can run.
- The methodology's existing kinds are five producers each with a `review-` and
  an `integrate-review-` step, plus a research trio and the driver's `finish`.
  A new *family* of kinds has to say whether its members take review steps of
  their own, and if so, what a `review-<stage>` would even read for.
- `docs/adr/corpus-rules-have-one-owner.md` decides where each new rule is filed
  — inline in the kind's own skill, or in a family reference file in the spine —
  by when a session meets it, not by its topic. A family of editorial kinds
  sharing a discipline is the case a family file exists for.
- `/Users/antony/Development/TheGreatExplainer` `docs/requirements.md` §1.6 is
  the prior art for the feedback edges: each stage can send issues back to an
  earlier one, and the pipeline has human gates.

## Done when

- An ADR records the extracted set with its trade-off: which stages became kinds,
  which merged, which were dropped, and what the evidence said about each.
- Every kept kind has its discipline written: goal, deliverable, HITL/AFK mark,
  what it reads, what it hands on, and how a feedback edge back to an earlier
  stage is expressed in a tree whose only ordering is position order.
- The launch templates the human will have to declare are named explicitly, so
  `pipeline-skills-k28` can hand back a concrete list rather than a category.
- `bash scripts/check.sh` passes.

## Notes

**Do not extract a stage that did not pay.** The whole preregistration exists so
that this decision is made against evidence, and a pipeline shipped because it
was already designed is the outcome the root brief set out to prevent.

**A feedback edge is the hard part.** `pick` is a depth-first pre-order walk and
is deliberately not a scheduler: it cannot skip a live leaf, and it cannot return
to one already retired. So "technical edit sends this back to draft" has to be
expressed as tree growth — a new leaf, inserted where the walk reaches it — and
saying exactly how is part of this leaf's deliverable, not an implementation
detail for the skills leaf to improvise.

**This decides how five books get written and no human is downstream of it.**
Consider `review-design` as your last act — and `leaf-insert` it at
`pipeline-skills-k28` rather than appending it, so the walk reaches the review,
and any integration it cuts, before the leaf that implements this design.
