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

## Decisions (running log)

**The set is read off the report's licensing table and nothing else.**
`measurement-report-k45` filled in the table the preregistration fixed before the
pilot ran: developmental edit and technical edit are `Merge` into **draft**, copy
edit and art are `Keep`, no stage is `Undetermined` and none is `Drop`. So four
kinds — `draft`, `copy-edit`, `art`, `proof` — and this leaf is not entitled to a
fifth. Proof's extraction rests on its being the preregistered fallback, which
the report states in those terms; the ADR repeats it rather than dressing it as
evidence.

**`draft` is its own kind and not `impl`.** Reusing `impl` costs no new token and
no launch template, and it is wrong on two counts. The merge target for both
merged stages is the draft, so folding the developmental and technical charters
into `impl` would bind every `impl` session in every grove to a structural and
technical-accuracy discipline over a document it is not writing. And `draft`
carries a precondition `impl` must not gain: a structure brief must exist, because
the report names the fold's safety as conditional on that brief continuing to be
written.

**No `review-<stage>` or `integrate-review-<stage>` kinds.** Nothing measured
them, and extracting eight kinds on no evidence is the root brief's own rule
broken one level up. The pipeline is already a sequence of adversarial reads: each
later stage reads the whole book against its own charter, so every stage but
`proof` has its review *scheduled* in `references/execute.md`'s sense and spends
none of the in-session allowance on that account. `book-check --final --check all`
decides the mechanical half. `proof` is last and keeps the ordinary
one-in-session-reviewer allowance.

**The chain is lazy in construction and not optional in membership.** A book leaf
`leaf-decompose`s with `--kind draft`; each stage's last act cuts the next with
`leaf-add`; `proof` cuts nothing. Lazy, because the cutting session can write the
specific case into the next leaf's body — the payoff `references/decompose.md`
names. Not optional, because the four stages are what the pilot graded: a stage
skipped on the previous session's judgement is that session re-grading a stage the
measurement already graded. This is the one place the editorial family differs
from a review chain, where the producer genuinely decides whether review is
warranted.

**A feedback edge is forward tree growth, never a return.** `pick` is a
depth-first pre-order walk that cannot re-enter a retired leaf, and
`references/retire.md` forbids a fourth leaf state. So "send this back to draft"
is a **re-run leaf of that stage, cut ahead of the next stage** — ordinary
`leaf-add` in walk order while the chain is lazy and nothing is queued behind the
running stage, and `leaf-insert` only where a later sibling entry already holds
live work. The re-run leaf's body carries the specific defect, and names which
later stages must re-read the changed material.

**A second re-run of the same stage on the same book is an escalation.** Not a
third leaf. `crate-books-k14` has no human in it, and an unbounded re-run rule is
an AFK oscillation that spends sessions without terminating.

**Hand-forwards have one surface: the book node's `BRIEF.md`.** A stage that meets
a defect a *later* stage owns records it there under `## Handed forward`, naming
the owning stage and the location; a stage clears the entries it closes, because a
brief is current-state context rather than a log. Every stage reads it — the brief
chain is root→leaf — so it reaches a stage two hops away, which writing the next
leaf's body cannot. An entry surviving `proof` is promoted at node close.

**Filing follows `corpus-rules-have-one-owner.md` rule 2 with no exception.** Rules
bound to all four kinds go in a fourth family reference file,
`plugins/grove/skills/grove/references/editorial.md`, which each member's skill
directs a load of by name; rules bound to one stage go inline in that stage's
`SKILL.md`. The spine's `SKILL.md` gains nothing: `restatement-declares-its-class.md`
makes class `none` mandatory for a rule binding one kind or one family.

**The disciplines are a work order, not a spec.** `SPEC-FORMAT.md`'s membership
test asks whether a session on an unrelated future grove would read it, and such a
session reads the *skills*; `corpus-rules-have-one-owner.md` rejected `docs/` as a
normative home outright, because it is installed nowhere. So the disciplines are
written into `pipeline-skills-k28`'s task file, where the one session that
consumes them will read them, and the skills k28 authors are the durable record.
Two ADRs carry what outlives `.grove/`.

**The tokens are unprefixed.** `draft`, `copy-edit`, `art`, `proof` — not
`book-draft` and its siblings. A kind is a discipline rather than a finding, and
each discipline is stated over *a document and its contract*, with the classes
supplied by whatever contract the repository declares. Prefixing bakes this
repository's subject into a token installed on every machine, which makes the kind
unusable for the next document pipeline while still shipping it everywhere.
