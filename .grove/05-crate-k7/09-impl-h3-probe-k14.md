# h3-probe-k14

## Goal

Test H3 — *an LLM given a checked formal model produces a more faithful
implementation than one given prose of equivalent length* — deliberately, rather
than reporting an impression gathered in passing. The deliverable is an entry in
`docs/formalism-findings.md` and a verdict on H3; the code this leaf writes is
throwaway and must not ship.

**Everything below the `Done when` was written by `library-k6` before any
implementation leaf ran.** That is the point: a measure chosen after seeing the
results is not a measure. Do not revise the pre-registration to fit what you
find — record the mismatch instead, which is a finding in its own right.

## Pre-registration

**Probe.** `insert`, implemented by `06-impl-insert-k11.md`. Chosen because its
ordering rule's payoff is an *intermediate-state* property: the reconciled prose
now states it correctly, so the prose arm is not being set up to fail, but acting
on it requires the reader to notice that the order of two renames is a design
decision rather than a loop's direction. **Fallback probe:** `promote`, if the
first result is degenerate — both arms identical, or both arms failing so
broadly that the comparison says nothing.

**Arm A — model-led.** The `insert` implementation as `06-impl-insert-k11.md`
produced it. Already in the tree; nothing to run.

**Arm B — prose-only.** A fresh context, given:
- the sections of `ARCHITECTURE.md` covering `insert` (the *Operations →
  Mutating* row, *How an operation runs*, *The plan is checked against itself, in
  order*, *Why the shift runs highest-first*, the `insert` refusals, and the
  *Invariants* list),
- the crate's public types and the `insert` signature,
- nothing else.

It must **not** receive `operations.qnt`, `structure.als`, the model-derived
tests, arm A's implementation, or `docs/formalism-findings.md`. It writes into a
scratch path outside the crate.

**Measure.** The model-claim-citing test suite from `06-impl-insert-k11.md`,
unchanged, run against both arms. Report, per arm: the number of model claims
violated, which claims, and which stated refusals are missing or wrong. Record
the input sizes actually used, in lines — "prose of equivalent length" is the
hypothesis's own wording and the two inputs will not be exactly equivalent, so
the real ratio is data rather than a detail to smooth over.

**Prediction.** The prose arm produces a working happy path and fails on at
least two of: (1) the shift order, run lowest-first or unspecified; (2) the
sequential destination check, done against the snapshot as found; (3) the gap
refusal, which the prose gives a rationale that does not fit.

**Falsification.** If the prose arm violates no model claim, H3 is not supported
by this probe, and the entry says so plainly. A hypothesis that cannot come out
negative was not being tested.

## Context

Beyond the brief chain:

- `docs/formalism-findings.md` — the whole log, and its *hypotheses under test*
  section, which this leaf is discharging one third of.
- The findings entries every implementation leaf in this subtree has appended by
  now. They are the *other* evidence about H3 — the incidental kind — and the
  entry should say whether the deliberate probe agrees with them.

## Done when

- Arm B exists, produced under the conditions above, and the conditions actually
  held. If a leak occurred — arm B saw something it should not have — say so and
  treat the result as contaminated rather than salvaging it.
- Both arms are scored by the same suite, and the numbers are in the entry.
- `docs/formalism-findings.md` carries the entry, with the six fields and a
  verdict on H3 that can be negative.
- The throwaway arm-B code is deleted, or parked somewhere that cannot be
  mistaken for the crate.
- The routing table at the end of `docs/formalism-findings.md` gains or amends
  the row this result bears on.

## Notes

**The limitation to record honestly.** Arm A's session was not told it was an
arm — `06-impl-insert-k11.md` carries no mention of this leaf — but the node
brief does say a model-versus-prose experiment exists somewhere in the subtree
without naming the probe. So arm A ran under a diffuse awareness that something
was being measured, and arm B ran under none. That asymmetry favours arm A and
belongs in the entry's *missed* field. It is not a reason to skip the probe; a
flawed measurement recorded with its flaw beats an impression recorded as a
finding, which is the failure the whole log exists to prevent.

**The suite was not tuned to either arm**, and that is load-bearing. Every test
in it cites a claim in `operations.qnt` — that rule was set by `library-k6`
before any of them were written, precisely so the measure would predate both
arms. If some tests turn out to cite nothing, exclude them from the score and
say how many.

**This leaf is where H3 stops being an article of faith or stays one.** Either
outcome is a result. The one outcome that is not a result is a paragraph saying
the model felt helpful.
