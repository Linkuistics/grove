# model-contract-k31

**Reviews:** model-contract-k5

## Goal

Challenge `docs/specs/semantic-contract.md` while it is still the only thing
either model family has been written from.

## Context

This step is inserted **ahead of** `alloy-models-k6` and `quint-models-k10`
rather than appended at the node's end, for the same reason
`experiment-baseline-k29` sat ahead of `model-contract-k5`: the artifact stops
being cheaply revisable the moment two independent model families have been
written from it, and after that a correction to a claim cannot be told apart
from a correction to the models. Read `k5`'s commit against the current tree,
per the standard review handoff.

The catalogue is the **single common ancestor of both model families**, which
the Experiment 2 pre-registration names as a live false-confidence hazard: two
models built from one catalogue agree wherever the catalogue is wrong, and
agreement then reads as proof. That makes an adversarial read of the catalogue
worth more than an adversarial read of either model.

Three specific doubts the producing session could not resolve about its own
work.

**1. Is the enumerated-assumption control load-bearing, or decorative?**
`EN-01`–`EN-16` exist to give a smuggled-in assumption somewhere to fail, and
each names a family and an instance that relaxes it. Two things could be wrong
and neither is visible from the list. First, the list may be **incomplete** —
take three claims that a model could satisfy for a reason the catalogue never
states (`FN-24`'s exhaustive interruption sweep, `SY-13`'s progress claim, and
`TT-21`'s one-snapshot rule are the likely carriers) and ask what each is
quietly granted that no `EN-` row names. Second, a relaxation may **break
nothing**: three rows are marked *exercised rather than relaxed*, which is an
argument, not a check, and the remaining thirteen each assert that relaxing them
breaks a claim without saying which. Name the claim each relaxation should break.
A relaxation that breaks nothing means the assumption was carrying no weight,
which is a finding about the catalogue and not about the model.

**2. Do the claims sort every reachable state, or only the ones the producer
thought of?** The outcome set is asserted to be closed and the blocked diagnoses
to be a partition (`FN-25`), and both were derived from prose rather than from
the code — `k5`'s brief forbids reading the implementation. Take the refusal
reason list and the state table and look for a reachable state that lands in
**two** rows or in **none**. The suspicious neighbourhoods, named so they are
not the ones skipped: a `Reserved(Preparing)` tree whose task entries are all
still in place (`TT-19` says refuse, the tree looks walkable); a foreign entry
at a reserved name during disposal (`TT-24` and `FN-21` both claim it, and
`OwnershipConflict` may or may not be the diagnosis); and the interval `FN-22`
covers, where the disposition changes *after* a handoff — the catalogue names
three sub-cases and may be missing a fourth.

**3. Are the four `TODO.finish_process.md` questions actually decidable from the
claims mapped to them?** The mapping table promises that `formal-synthesis-k16`
can read a keep / delete-replace / defer verdict off the models rather than
argue one. Test that promise on Q3, which is the weakest: the marker-replacement
protocol's byte layout is a *deliberate omission* while its replace **transition**
is required to exist, and the catalogue says abstracting the transition away
would answer Q3 by construction. Judge whether `FN-21` and `FN-22` as written
force a model to represent that transition, or merely permit it. Then check the
other three for the same failure — a question whose deciding claim can be
satisfied without ever reaching the state the question is about.

## Done when

- Each of the three doubts is answered against the spec, with the evidence read
  rather than inferred from a claim's title.
- Every finding is stated as a concrete correction to a named claim, assumption
  or table row — a new claim, a changed SHALL, a changed witness obligation, or a
  changed relaxation owner — not as a concern.
- The verdict says plainly whether the catalogue is fit to be the sole input to
  two independently built model families as written, or names exactly what must
  change before `alloy-models-k6` runs.
- If nothing is found, this leaf retires and creates nothing.

## Notes

Do not write models, change product code, or rewrite architecture/user
documentation — the producer's scope binds this step too.

Two things are settled and not open here. Finish keeps today's single successful
exit, and the root brief's merge-and-remove exit is out of scope: that was put to
the human and confirmed, and the reasoning is in the spec's `## Out of scope`.
The single-runner test seam was put to the human in the same exchange. What is
open is whether the claims and assumptions are correct and complete enough to
serve those decisions.

Findings worth acting on are cut as `integrate-review-design` beside this leaf,
per the chain rule. Findings that are purely local corrections to a claim's
wording may be cheaper to fix in place and say so — this leaf may edit the spec
directly when the fix is mechanical and the evidence is in hand.
