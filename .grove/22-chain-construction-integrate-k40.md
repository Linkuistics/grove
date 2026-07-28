# chain-construction-integrate-k40

**Kind:** integrate-review-design

## Goal

Apply **chain-construction-review-k39**'s findings to the design from
**chain-construction-k38**, and leave the outcome in its durable home — the
spec, the ADR set, the guidance surfaces, or `grove-llm` itself, as the design
concluded.

## Done when

- Every finding from k39 is **dispositioned**: applied, or explicitly declined
  with a reason. A declined finding is a legitimate outcome and is recorded as
  one, not silently dropped.
- The durable record k38 nominated exists and is coherent —
  `docs/specs/task-kind-taxonomy.md` reworked in place if that is the home, or
  the ADR set reworked as a minimum coherent set (never a superseding ADR), with
  every citation reconciled.
- If the design landed a `grove-llm` verb: it is implemented, tested, and the
  property that matters is **falsified by mutation** rather than asserted —
  this repo's standing bar (*jj-first-coverage-k6*, *codex-grant-refused-k35*,
  *guard-loop-signal-k37*). Whatever guidance surfaces name the old hand-cut
  procedure are updated to name the verb.
- If the design landed prose only: the three cutting-time surfaces
  **compose-task-chains-k29** identified (the bootstrap prompt, `SKILL.md`'s
  Decompose step, `TASK-FORMAT.md`) carry the strengthened wording, and the
  change is described in terms of what a session now *reads at the moment it
  cuts*, not what the reference material says.
- Anything reaching real sessions only through a release is called out as such —
  this repo's recurring in-the-tree-not-in-the-binary gap — and the CHANGELOG
  entry is written even if the release is not cut.

## Notes

Cut together with k38 and k39 as one chain, per **chain-group-unit-k36**'s
operational habit.

The chain's own history is evidence for whatever it concludes: if a mechanism
lands, note whether *this* chain would have been cut differently under it.
