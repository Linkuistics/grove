# chain-group-unit-k36

**Kind:** design

## Goal

grove has **one** decomposition mechanism (a node directory with ordered child
leaves) and it is currently reused for two things that are not the same thing:

1. **Genuine decomposition** — the current item proved bigger, so it becomes a
   node whose children are vertical slices, each standing demoable on its own.
2. **A step chain over one artifact** — `X` → `review-X` → `integrate-review-X`,
   and the research vendor pair `research-a` / `research-b` →
   `combine-research`. These are *not* separate work items. They exist for one
   reason only: **a step boundary is the only place grove can switch harness or
   model** (task-kind-taxonomy, model-per-task-kind). One piece of work, cut into
   sessions so a different vendor reviews what another produced.

Decide whether a chain is a **first-class construct** distinct from ordinary
decomposition, and what it costs.

## Context

Raised by the human 2026-07-28 during **codex-grant-refused-k35**; independent of
the herdr work and of that leaf.

**compose-task-chains-k29** already made chains the habitual shape a session cuts
in — bootstrap prompt, `SKILL.md` Decompose, `TASK-FORMAT.md` — and gave them a
naming structure (`<stem>-review` / `<stem>-integrate`; `<stem>-a` / `<stem>-b` /
`<stem>-combine`). It deliberately stopped at *encouragement*: "grove validates no
ordering between leaves and this must not start (*task-kind-taxonomy*)." That
constraint is the thing this leaf has to re-examine, because the human is asking
for exactly two behaviours a pure-encouragement scheme cannot give:

- **The group does not jump around.** Once a chain's first step is done, `pick`
  must land on the *next step of that chain*, not wander to whatever the
  depth-first walk would otherwise reach. Today a chain is only ordered because it
  happens to sit in adjacent positions; `leaf-insert` ahead of it, or a chain cut
  as siblings of unrelated leaves, breaks that with nothing to notice.
- **Closing the group needs no confirmation.** The Retire step asks the human
  before treating a node as done, so they can add a follow-up leaf. That question
  is *right* for a decomposition node (is this area actually finished?) and
  **noise** for a chain — an integrate step finishing means the chain is finished,
  by construction. Constraint 5 says grove guides and does not gate; a confirmation
  prompt that has exactly one sensible answer is a gate with extra steps.

The human's stated shape: **reuse the directory/file structure**, treat it as a
single unit. So the question is not "invent a new storage form" — it is what
*marks* a node as a chain, and what reads that mark.

## Done when

- A decision exists on whether chains are first-class, recorded as an ADR
  (**reworked in place** — this very likely edits *task-kind-taxonomy* and/or
  *task-tree-scheme* rather than adding a record), plus whatever
  `docs/specs/task-kind-taxonomy.md` needs.
- If first-class: the **mark** is decided (a node-brief field? a slug/kind
  convention the verbs can read? a `**Chain:**` line?), and so is which of
  `pick`, `leaf-insert`, `leaf-add` and the Retire cascade must read it — with the
  blast radius on `SKILL.md` and `TASK-FORMAT.md` stated.
- The **tension with the spine is confronted head-on, not waved past**: constraint
  3 (suggested shape, not enforced schema) and constraint 5 (guides, does not
  gate) both push against a mechanism that *constrains* the walk. Say why a chain
  earns the exception, or say it does not and close the question. A well-argued
  **no** — chains stay convention, and the confirmation prompt is the only real
  cost — is an acceptable outcome and retires the question for good.
- If the answer is yes, the *implementation* is **not** done here: cut it as its
  own chain (`chain-group-<step>`), which is itself the pattern under discussion.

## Notes

Prior art to read before deciding, in this order:

- `docs/specs/task-kind-taxonomy.md` — the chain naming structure and the
  escalation call (chain a load-bearing artifact, subagent a one-file change),
  and **why** node-per-chain was rejected as a *naming* answer by
  **compose-task-chains-k29**: it lost on what a node *already means* in a real
  tree. That rejection was about naming, not about semantics — this leaf may
  reach the opposite conclusion for a different reason, and if it does, it must
  say so explicitly rather than quietly reversing it.
- ADR *task-tree-scheme* — positions, permanent keys, and the walk `pick`
  implements.
- `SKILL.md`'s Retire step — the cascade and its per-node confirmation; the
  prose is deliberately un-verbed because it is judgement. A chain is the case
  where it *isn't* judgement.

The two candidate marks worth costing, at minimum: **(a)** a node whose brief
declares it a chain, read by `pick` and the cascade; **(b)** inference from the
children's *kinds* — a node whose children are `X` / `review-X` /
`integrate-review-X` **is** a chain, with no new field at all. (b) costs no
schema and cannot drift out of sync with reality; (a) is explicit but is one more
thing to hand-maintain (constraint 3). Cost both.
