# experiment-baseline-k29

**Reviews:** experiment-baseline-k4

## Goal

Challenge the two artifacts `experiment-baseline-k4` landed, while both are still
revisable: the before-change ledger at `docs/preservation-baseline.md`, and the
*Experiment 2 — pre-registration* section appended to
`docs/formalism-findings.md`.

## Context

This step is inserted **ahead of** `design-model-contract-k5` rather than
appended at the node's end, because its subject stops being revisable the moment
modelling starts. A pre-registration's entire value is that it was fixed before
the results; reviewed after `alloy-models-k6` and `quint-models-k10` have run, a
correction to its measures is indistinguishable from tuning the measures to the
findings. Read `k4`'s commit against the current tree, per the standard review
handoff.

Three specific doubts the producing session could not resolve about its own work.

**1. Are the measures actually falsifiable, and are they countable by a session
that was not in the room?** M1–M8 fix counting rules, and the load-bearing one is
the definition of a *material finding* — a defect a reader would act on, as
against a modelling artifact. That boundary decides every number in the
experiment. Take three plausible borderline cases (a bound that turned out too
small; a predicate that was ambiguous until a model forced a choice; a shipped
behaviour the model says is fine but the catalogue never described) and ask
whether the definition sorts them the same way for two different sessions. If it
does not, the comparison is uninterpretable and the fix belongs here, not later.

**2. Does the ledger overclaim, and is §10 right this time?** §10 maps preserved
claims to the tests that check them. Two rows were wrong on first pass —
`tests/lifecycle_invariants.rs` was cited for tree format when it is about
methodology-corpus delivery, and `tests/tree_access.rs` for witness refusal when
it is about the advisory lock — and both were plausible from the filename alone.
Assume the same class of error survives. The remaining rows are the ones a later
phase will trust; check each against what the named file actually asserts, not
what it is called. The five **nothing**-rows matter more than the covered ones:
if any of them is in fact covered somewhere, the ledger is telling `k2` and `k3`
to build a guard that already exists.

**3. Is anything material missing from the ledger's scope?** The root brief's
*Preservation ledger* is the checklist; §11 records four deliberate omissions
(configuration diagnostics, `finish-commit`, Linux behaviour, performance).
Judge whether each omission is genuinely out of reach here or merely
inconvenient — §11 exists so absence is not read as a measured negative, and it
is only worth that if the list is complete. The captured transcripts in §8 are
the other half: is there a refusal class a later phase will change that has no
captured *before* state?

## Done when

- Each of the three doubts is answered against the artifacts, with the evidence
  read rather than inferred from names.
- Every finding is stated as a concrete correction to a named section, not as a
  concern.
- The verdict says plainly whether the pre-registration is fit to bind `k1`'s
  evidence as written, or names exactly what must change before
  `design-model-contract-k5` runs.
- If nothing is found, this leaf retires and creates nothing.

## Notes

Do not write models, change product code, or rewrite architecture/user
documentation — the producer's scope binds this step too.

`docs/preservation-baseline.md` is framed as a **contract** rather than an
experiment snapshot; that framing was the human's call and is not open here.
What is open is whether its contents are accurate and complete enough to serve
that framing.

Findings worth acting on are cut as `integrate-review-requirements` beside this
leaf, per the chain rule. Findings that are purely corrections to a factual
table may be cheaper to fix in place and say so — this leaf may edit the two
artifacts directly when the fix is mechanical and the evidence is in hand.
