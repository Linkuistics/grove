# loop-construct-k223

**Integrates:** loop-construct-k220

## Goal

Triage the findings from the adversarial read of the pass-series design and
integrate every finding that survives verification into the current design set.

## Context

Read `loop-construct-k220` from its review commit. The review's findings stay in
that artifact; this body deliberately does not transcribe them.

## Done when

Every review finding has a recorded disposition, every accepted finding has
been reconciled across the spec, ADR, glossary and implementation handoff it
affects, and the resulting design is one coherent current-state set.

## Notes

This is an integration, not a work list: verify each finding independently and
reject, narrow or accept it on the evidence before changing an artifact.

## Decisions (running log)

**All five findings verified independently before any edit.** Each was graded
against the artifacts and the source, not against the review's prose. Four are
accepted and one is accepted narrowed; none is noise, and none required a new
producer review chain.

**F1 — accepted; the argument is replaced, the decision is not.** The
marker-token rejection is unsound as written, on five of its six objections:

- Objections 1 and 3 are **one objection counted twice**. "It cannot go anywhere
  else" resolves by saying *before the slug is the outcome slot again* — which is
  objection 1. Two of the six are one.
- Objection 2 is **false**. `refuse_token` already refuses uppercase for both of
  a name's words (lowercase ASCII, digits and dashes only,
  `crates/grove-loop/src/task_name.rs`), so **no existing name in any grove can
  spell an uppercase marker** and adding one to the reserved set renames nothing.
  The cited cutover cost is zero, not "paid once per tree".
- Objection 4 prices `Option<Marker>` when the proposal's real cost is a second
  `Parts` variant. Nodes genuinely have no outcome field
  (`Parts::Node { slug }`), so objection 1 is an argument from shape, not a
  collision.
- Objection 5 survives but defeats only the slug placement, which the record
  already lists as a separately considered and rejected option.
- Objection 6 — the one the record calls decisive — contradicts the same
  record's claim two sections earlier that `find .grove` **is** the whole reader.
  A human with `find` is a reader; constraint 6 is why.

**The decision nevertheless holds, on an argument the record did not make.** A
marker is *redundant*, not unread. Every pass node under a series carries the
series' bare stem as its whole slug, so a series with two or more passes is
already **sibling node directories sharing one slug** — a name-level
discriminator that exists today and that nothing else in a grove produces.
Enumerated across three live groves: 0 sibling-slug collisions among all node
directories in this tree, Writegood's and `grove.gh-issue-12`'s, while the same
test over leaf files fires 13 times in this tree alone on review chains and
vendor pairs — so the instrument was watched to report dirty before its clean
read was credited. And a one-pass series is honestly *not* distinguishable,
because it is not yet a repetition: its series-ness is a claim about the future,
and the three facts that constitute it — sequence, exit condition, cap — cannot
be carried by a name at all. A reader must open `BRIEF.md` either way, which is
what dissolves the buy without denying the reader.

**F2 — accepted.** The spec's *A one-step pass is a leaf and gets no directory*
instantiates the ADR's own stated reopen condition, so the design set holds two
answers to whether a pass is a node. The ADR's categorical claim is scoped, its
falsified reopen condition is replaced, and the record is **renamed** —
`iteration-is-a-node-of-nodes` asserts as its identity the thing that is false of
a one-step series, and `ADR-FORMAT.md` makes the slug the identity. New slug:
`iteration-reuses-the-existing-species`.

**F3 — accepted as a trade-off to state visibly, not as a mechanism change.** A
satisfied exit and a reached cap both cut nothing, so the two outcomes are
byte-identical in the tree; adding a checkable cap would mean state beside the
tree, which constraint 1 forbids. What was wrong is the *claim*: the spec called
four freeform, mutable brief declarations "the filesystem representation". The
spec now says which facts the tree carries and which only the brief does, and
names the brief as where a cap escalation is recorded.

**F4 — accepted as a contract stated unclearly.** `SPEC-FORMAT.md`'s `Decisions`
section is where "the modules built or modified, their interfaces" are settled,
and the delivery surface was left to `pass-series-discipline-k221`, which says so
explicitly. The analysis existed — in that leaf's body — but not in the
agreement-point artifact. The spec now settles it: **a condition in the spine's
register pointing at `references/decompose.md`'s composition-shape section, plus
`BRIEF-FORMAT.md` for the four declarations**, with the new kind and the family
reference file recorded as rejected and why. `k221` keeps the shipping work and
its body is reconciled; nothing about it is reshaped.

**F5 — accepted, narrowed to a scoping repair.** Verified in the prior art the
review cited: `Writegood`'s `18-impl--machine-supply-k22.md` is **still live**
under `13-measure-k18/` and carries **six** commits whose own subjects number the
readings first through sixth. So one leaf ran six sessions. That does not
falsify the construct, but it does falsify the unqualified justification the spec
used to keep *one task is one session* — "a pass's step is one leaf and one
session" — and it shows the pass/correction-run pair is not a partition of Grove
repetition. The spec bounds its subject to completed work deliberately run again
and names the third form rather than implying it does not exist.

**Not done here, deliberately.** No marker token is added, no verb is added, no
Rust moves — the design's own out-of-scope claims still hold, and F1's repair is
to the record's reasoning rather than to its decision.
