# retire-confirmation-k12

**Kind:** design

## Goal

Revisit whether the Retire cascade should ask the human for confirmation at all,
and if so when.

Raised by the human, 2026-07-29, during `chain-as-node-k7`: *"I also think that we
need to revisit the idea that all retirements need confirmation."*

## Context

Two confirmations exist in the loop's Retire step, and they are different things
that the phrase "all retirements need confirmation" spans:

1. **Marking a leaf done** — `grove-llm leaf-retire` after committing. The skill
   already calls this *"mechanical bookkeeping, no need to ask"*, so it is not
   confirmed today. Worth checking that the shipped `content/SKILL.md` prose
   actually reads that way in practice, and that sessions behave accordingly.
2. **Treating a node as done** — the cascade walk. When a node has no live leaf
   left, the skill says *"Ask the user before treating it as done"*, so they can
   add a follow-up leaf and so anything durable in the brief gets promoted upward.
   This one recurses up the parent chain, asking again at each level.

`chain-as-node-k7` already narrowed (2): the confirmation is asked of
**brief-carrying** nodes only, because a chain node is brief-less by rule and has
no charter to promote. That is a *narrowing*, not an answer to the broader
question — which is whether a brief-carrying node's close needs a human at all.

Prior art to re-derive rather than inherit:

- `CONTEXT.md` §§ *Node directory*, *Pruning*, *Complete finish cycle*.
- `content/SKILL.md` § Retire — the cascade walk and brief-promotion are
  deliberately prose, not a verb, *"both are judgement steps with no stable
  input/output shape that would justify a verb"*.
- `docs/adr/pruning.md` — abandonment is HITL by rule; that is a different
  confirmation again and is probably not in scope, but the boundary should be
  stated rather than assumed.

## Done when

The decision is recorded as an in-place edit to the ADR / spec / glossary set, and
covers:

- Which of the two confirmations survive, and for which node species.
- What replaces a dropped confirmation, if anything — the confirmation currently
  carries two jobs (add a follow-up leaf; promote the brief upward), and dropping
  the question does not drop the jobs.
- How the answer interacts with the **self-driving loop running unattended**. A
  confirmation is a stall by design; the HITL/AFK rule says a stall is correct
  behaviour, not a fault. If confirmations go, say what an unattended cascade does
  instead and whether anything is lost.
- Whether `content/SKILL.md`'s Retire prose changes, which is `impl` work to be
  externalized rather than done here.

## Notes

**Sequencing.** This sits behind the `chain-node-k9` chain, which writes cascade
prose under the current rule. If this leaf changes that rule, the prose needs one
more pass — cheap, and noted in `chain-node-k9`.

**Watch the framing.** "Does this need confirmation?" invites the answer *no* on
friction grounds alone. The prior decisions in this area were made on a different
axis — what the question *buys* — so the case for removing one should be made in
the same terms: name the job the confirmation does, and say what else does it.

**Scope check.** Like `chain-as-node-k7`, this is grove-methodology work in a
grove whose `BRIEF.md` ships the `using-codebase-memory` skill. It is here because
it was raised here and the human chose to keep it here; see the root brief's
*Scope* note.
