# candidate-lessons-k90

## Goal

Adjudicate the root brief's **six candidate lessons**, one verdict each:
**evidenced**, with the citation and what would have falsified it, or
**dropped**, with the reason. The parent's `Done when` says at least one is
expected to be dropped or materially weakened, and that *if none is, say why the
prior was that good*.

The deliverable is a durable artifact under `docs/` beside
[`docs/loop-record.md`](../../docs/loop-record.md) and
[`docs/review-yield.md`](../../docs/review-yield.md).

## Context

**Two of the four inputs already exist and are not yours to redo.**
`loop-record.md` enumerates 73 leaves, 16 nodes, 9 chains and the cost account,
with its controls published. `review-yield.md` opens all eighteen chain bodies
and answers the yield question. Both are derived by a script that reads `.grove/`
at a pinned revision and asserts coverage in both directions; **re-derive rather
than trust either when a count matters**, and both scripts are re-runnable after
teardown. What neither does is read the *model* artifacts, which five of the six
candidates are about.

**One candidate is already answered, and the answer is a drop.**
`review-yield-k89` measured *review yield did not decay* and the verdict is the
**null** one: nine chains and three channels cannot separate flat from falling.
Do not re-open that measurement; inherit it. What it hands you is below, and the
last item is the one that changes the shape of the candidate rather than its
truth value.

- **45 findings raised across nine chains, 45 verified real, 0 dropped, 0 noise**
  at the tree level. Two were downgraded within the four-way triage to *a
  contract stated unclearly*.
- **A 100% survival rate is what a broken instrument reads.** In five of nine
  chains the integrating session's own task body was **written by the reviewer** —
  the charter *is* the finding list — so it had no structural place to disagree.
  **No tree-level review finding has ever been rejected in this grove.** Every
  rejection came from the in-session channel, where the reader owes the finding
  nothing.
- **The apparent decay is a selection effect.** Findings per review run 6, 6, 7,
  6, 6, 3, 4, 2, 5. But review rate is a function of **session kind** — `design`
  42%, `prototype` 13% — and the loop reviewed further down its own value curve
  as it went. The gap is ≈1.8σ before the subject-size confound; ~25 chains of
  *matched* subject size would be needed, and even that is the weaker fix,
  because sub-corrections per finding range 1.0–3.3 and consequence is not
  captured at all.
- **The motivating observation spans all three channels.** Three findings from
  `honest-classification-k80`'s own reviewer, five from `k84` beside it, two from
  `k85` — that is in-session, then tree-level, then integration. **It was never a
  statement about review chains.** Read as what it actually describes it says
  something different and defensible: *three independent readers of one artifact
  each found what the two before them had not*. Deciding whether that is the
  lesson, and whether one node can carry it, is **yours**.

**The two candidates this leaf can already see evidence for, and where.** Both
are about instruments rather than about the loop, so both need the model
artifacts read — which is why they were held for this leaf and not `k89`'s.

- *A green suite is not evidence; only a control that can kill the claim is.* The
  tree carries at least five independent instances, and they are not the same
  instance retold: `finish-k57` B5 (`FN-14`/`FN-26`/`FN-30` asserted over fields
  no transition could make bad); `obligation-placement-k67` F2 and its
  integration (`FN-32` controlled on one of the two artifacts it names, and Quint's
  `inv_FN_32` **character-for-character `inv_FN_10b`**); `finish-scope-k76`
  (Quint's `inv_FN_25a` was *the two diagnoses are disjoint*, which the if/else
  chain made true by construction — no mutation could have moved it);
  `finish-verdicts-k78` R1 (`relax_EN_03` left `OWNERSHIP_PROVEN` true, so the
  whole content of `inv_FN_32` was unsettable); `honest-classification-k84`
  finding 1 (`FN-28` restated over flags its own steps set).
- *False greens are found by narrowing, not widening.* `honest-classification-k85`
  found `inv_FN_28` **violated with no model mutation at all**, by narrowing
  `base`'s environment — a strict subset of `base`'s traces, so the
  counterexample was always `base`'s and 8000 samples had never drawn it. The
  sibling instance is in `finish-scope-k76`: a mutant module that copied its
  neighbours' environment reported **green** because that environment never
  reached the overlap. **A mutant module's environment is part of the control.**

**Two candidates whose evidence is thinner than the brief's wording, and that is
what to check.** *Measure, freeze, then repair* and *a module that changes what
the model does must be run against every claim the model has*. The second has one
clean instance (`honest-classification-k80`'s reviewer swept all 63 library
invariants and found two violated, one nobody was looking for) and is stated as a
general obligation on that one instance. The first has several process instances —
`obligation-placement-k63`'s two invalidated hours, `k68`'s two concurrent sweeps
writing one log, `finish-scope-k76`'s freeze-and-digest discipline and its killed
third attempt — but *those are about running an instrument, not about a predicate
being a subject*. **Check whether the brief's wording and its evidence are about
the same thing**, and if not, say which one survives.

**One candidate is self-referential and the tree has an instance of it about
itself.** *A claim stated over the model's own history or classifier is
self-certifying, and the repair recreates the hazard one level down.* The models
carry it; so does this workstream. `finish-verdicts-k65` burned a handle by
letting a sweep's report live inside the sweep's own subject, and `k89`'s finding
that five integrations verify a charter the reviewer wrote is the same shape at
the loop level. **Whether that generalises across the levels, or is two unrelated
things sharing a sentence, is a real question and not a rhetorical one.**

**Three instrument hazards this workstream measured, offered so you do not pay
for them again.** All three read clean.

- **`ABANDONED` contains the substring `DONE`.** A glob or grep on the retire
  infix counts every pruned leaf as retired — 54 producers where there are 37.
  Anchor on the position: `NN-DONE-<kind>-`.
- **Five of nine dispositions are in the commit message, not in `.grove/`.** A
  reader that opens only task bodies concludes the dispositions were never
  recorded. They were, in full, per finding.
- **A block classifying a document's root will absorb any section you forget to
  classify, silently.** `review-yield.py` had that hole until a deliberate
  deletion was run against it; the forward control read clean. Its three
  mutation controls are in `docs/review-yield.md`'s residue section, and they
  are the reason its green means anything.

## Done when

- Each of the six candidates carries **one verdict** — evidenced or dropped —
  with the artifact and section that decides it, and **what would have falsified
  it**. A candidate that survives on one instance says so in as many words.
- *Review yield did not decay* is adjudicated on `review-yield.md`'s evidence
  rather than re-measured, and the verdict states plainly that the tree cannot
  establish it. If a weaker claim is substituted, the substitution is visible and
  the original is not quietly rewritten into it.
- **At least one candidate is dropped or materially weakened, or the leaf says
  why the prior was that good** — the parent's clause, unchanged.
- Every citation is checked against what the named file *asserts*, not against
  what its title suggests. `experiment-baseline-k30`'s finding 4 is this tree's
  own instance of getting that wrong twice.
- The candidates are stated as **claims with the conditions under which they
  would be wrong**, not as advice — the root brief's own bar.
- Nothing in `.grove/` is edited beyond this leaf's own body.
- The **lesson leaves are cut** — the root decomposition's step 2 — or this leaf
  records why the clusters are not yet nameable and what would name them. How
  many there are is this leaf's finding, not a number to hit. The parent's
  decomposition says child 4 *may fold into (3) if the clusters are obvious by
  then*; deciding that is this leaf's.

## Notes

**You may contradict `review-yield.md`, and the node brief says you must be able
to.** Child 3 was *held separate because it must be able to contradict (2) rather
than inherit it*. Inheriting the number is right; inheriting its interpretation is
not.

**The success condition is a drop, not a clean sweep.** Six candidates written
from inside a campaign by a session that is not neutral about it, all six
surviving contact with the evidence, would mean the adjudication checked nothing.
One is already weakened; that does not discharge the obligation for the other
five.

**Do not re-derive what the models already record.** The Quint and Alloy columns
carry their own findings, controls, narrowings and *what a green run does not
prove*, and `docs/formalism-findings.md` carries the bounded comparison. This
leaf reads them **as evidence for a lesson**, and where a model artifact and a
candidate disagree, the model artifact is the subject and the candidate is the
claim under test.

## Decisions (running log)
