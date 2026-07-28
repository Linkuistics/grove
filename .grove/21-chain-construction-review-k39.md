# chain-construction-review-k39

**Kind:** review-design

## Goal

Review the design produced by **chain-construction-k38** — the decision on
whether and how chain construction gets a mechanism. Find, do not fix; the
findings are applied by **chain-construction-integrate-k40**.

## Context

The design under review has one obvious failure mode and one non-obvious one,
and the review should be biased towards both.

**The obvious one: a verb that gates.** grove's constraints 3 and 5 —
*suggested shape, not enforced schema* and *grove guides, it does not gate* —
both push against making chains structural. **chain-group-unit-k36** already
rejected a first-class chain on exactly this axis, and found that "remove a
gate" arguments can smuggle in a larger gate. Test the design for anything that
*validates*, *refuses*, or makes the non-chain path harder rather than making
the chain path easier.

**The non-obvious one: a mechanism that does not change behaviour.**
**compose-task-chains-k29** is the precedent that matters: five documented
surfaces produced zero chains in 26 leaves, because none of them was the surface
a session reads *while cutting*. A verb has the same exposure — a
`leaf-add-chain` nobody reaches for is k29's failure with a compile step. Test
whether the design says how a session *comes to use it*, not just that it
exists.

## Done when

- The design is reviewed against constraints 1–7, naming the ones it strains
  and the ones it is neutral on. Constraints 3 and 5 get an explicit verdict.
- Both failure modes above are checked for, and the check is reported even when
  it finds nothing — a clean bill on a named risk is a finding.
- If the design chose "prose is enough", that answer is reviewed as hard as a
  verb would have been: does the argument turn on evidence that k29's fix
  changed the odds, or on the same optimism k29 falsified?
- The vendor-pair coverage is checked — `research` has no `review-research`
  sibling, so a surface designed around the review chain may not fit it, and a
  design that quietly covers only one shape should say so rather than imply
  both.
- Findings are written down in a form k40 can apply, each with enough context
  to be actionable without re-deriving the design.

## Notes

Cut together with its producer and integrator, per the habit
**chain-group-unit-k36** named: a chain's steps go into the tree in one motion,
because a step decided on after its producer ran needs `leaf-insert` and is the
shape that actually breaks a chain in practice.
