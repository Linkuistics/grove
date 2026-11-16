# A reader checkpoint is a requirements leaf

A document's contract may declare **reader checkpoints**. A checkpoint is a read
by the intended reader or a labelled stand-in, with stated tasks, entry
surfaces and viewing environment. The session that cuts the document's leaf
also cuts one `requirements` leaf per declared checkpoint, placed after the
document's `proof`. That leaf holds the read.

Its procedure is `grove-requirements/reader-test.md`, reached by one condition
in that skill. The procedure has six steps:

1. Frame the read first.
2. Let an early comprehension failure stop it.
3. Record the reader's words, the locations they report, and what stayed
   unknown.
4. Keep proxy, mechanical, acceptance and task evidence apart.
5. Route each correction to its owning stage under the bounded correction rule.
6. Start the next read at the repaired entry.

## The trade-off

**Reader evidence has to be scheduled, because nothing else produces it.** On
APIAnyware's first reading path, the documentation checks and a model proxy
both passed. The human reader still stopped at the first compressed phrase
(RC-1 in [`documentation-evidence.md`](../research/documentation-evidence.md)).
Writers do not judge difficulty well (X-06). No editorial stage can stand in
for the reader, and each stage is a fresh-context model read. So the read is
tree work of its own, placed where it cannot be skipped.

**It needs a human, so it needs a human-in-the-loop kind, and one already
fits.** `requirements` is the kind that exists to take down the human's own
words. Every APIAnyware reading checkpoint was a `requirements` leaf, and so is
this workstream's own reading checkpoint, `sample-reading-k11`. Reusing it mints
no kind. Every user's launch configuration keeps working without a new
template, and a `requirements` session that holds no checkpoint gains only one
condition line.

**The cost is legibility.** A reader checkpoint's kind does not name what it
is, and only its body and the contract say so. That is paid knowingly: the
alternative costs every user a configuration change for a name.

**It sits outside the four stages.** Stages repair their own class of defect,
and a read that found a defect in several classes would have no owner inside
the pipeline. The checkpoint records the defects. Correction runs of the owning
stages repair them, under the existing bound: a second re-run of one stage
against one document is an escalation
([a feedback edge is forward tree growth](a-feedback-edge-is-forward-tree-growth.md)).

## Considered options

- **A `reader-test` kind.** Clearer by name. Rejected because a kind needs a
  launch template in every user's configuration before its first leaf can
  launch ([a kind is an open token](a-kind-is-an-open-token.md)), and nothing
  measured shows the name buys a better read. Reopen if reader checkpoints come
  to need a model, effort or harness different from `requirements`, which would
  make a separate configuration key worth its cost.
- **Run the read inside `proof`.** Rejected. The human required reader testing
  outside the four stages; `proof` is AFK; and a human read cannot run
  unattended. Reopen never.
- **Let `planning` hold the read.** Rejected because `planning` is AFK and grows
  the tree. It may schedule a checkpoint; it cannot hold one.
- **Accept a model proxy in place of the human.** Rejected as evidence of human
  comprehension (PR-15, RC-1). A proxy may still run inside a checkpoint when it
  is labelled and reported apart.

## What would reopen this

- Checkpoints frequent enough that a dedicated human-in-the-loop mark or
  launch configuration pays for itself.
- `requirements` sessions observed treating a checkpoint as ordinary
  requirements elicitation. That would show the shared kind hiding the task.

The design this records is [`documentation-guidance`](../specs/documentation-guidance.md),
section 6.
