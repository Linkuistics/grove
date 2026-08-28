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

**The in-session reviewer could not be spent, and a `review-design` leaf is not
cut either.** `references/execute.md` grants a picked plain producer one
in-session reviewer; this session's harness instructions forbid materialising a
subagent unless the human asks for one, and the human did not. That is the same
constraint `obligation-placement-k63` and `finish-verdicts-k65` recorded, and
both of them cut a `review-design` leaf in its place. This leaf does not, on a
difference those two did not have: **its deliverable is itself the adversarial
read.** `k63` and `k65` were producers whose conclusions licensed themselves —
`k65`'s four verdicts followed from its own declaration that a criterion was
mis-typed. This leaf's charter is to attack six claims written from inside the
campaign, its success condition is a drop, and every verdict below is decided
against a named file rather than against a recollection. A review leaf here
would buy a second read of the same artifacts under the same charter, which is
`ordinal-root-lifecycle-k14`'s reason for declining and cutting nothing. What
replaces the fresh context is the citation discipline: **every load-bearing
citation below was opened at the source and checked against what the file
asserts**, and the three that did not say what the tree said they said are
recorded as such in *Where the tree misreports itself*.

**Six verdicts, landed in [`docs/candidate-lessons.md`](../../docs/candidate-lessons.md).**
Three of the six moved, so the parent's *at least one is expected to be dropped
or materially weakened* is discharged three times over rather than by the
inherited drop alone.

- **1 · a green suite is not evidence** — **evidenced**, and the candidate
  understates it twice. Fifteen named obligations across three scopes and both
  families, plus three defects in `models/run.sh` itself and four controls that
  fired for the wrong reason or not at all, found by ten sessions and enumerated
  in the artifact. The strengthening is the useful half: a control that *can*
  kill a claim is not enough — `mutant_block_clears`, `mutant_unproven_ownership`
  and `relax_EN_13` were each killing something other than the claim they were
  credited with — so what the corpus establishes is that a control earns a claim
  only when shown to kill it **for the reason claimed**, by the isolating
  measurement `inv_FN_28`'s comment already carries.
- **2 · self-certifying over own history or classifier** — **evidenced in full**,
  the only candidate whose entire sentence including its derived obligation is
  landed verbatim as a decision record
  (`a-shared-safety-claim-names-the-role-not-the-artifact`, closing section).
  The recursive clause rests on one complete cycle over three sessions
  (`k80`'s reviewer → `k80`'s repair → `k84` f1 → `k85`'s isolating kills), and
  is named as one instance.
- **3 · false greens are found by narrowing** — **materially weakened, and half
  of it falsified.** The supporting instance is verified at the source
  (`finish.qnt`'s `Hist` comment). But narrowing made five false greens here
  against one it found, and *not widening* is falsified twice — `TT-20`'s dead
  control was repaired by widening `FOCUS` 4→5, and `EN-08`/`FN-31.c`'s declared
  impossibility died to a deeper run that reached the disk in fourteen states.
  What survives is the conditional form, and the corpus's own compression of it:
  **a mutant module's environment is part of the control.**
- **4 · a module must be run against every claim** — **evidenced on one node**,
  said so in as many words. Two sweeps, four unlooked-for findings, no clean
  sweep. What carries it is structural rather than repetitive: `models/run.sh`
  *defines* the module rule, and `k80` records the consequence exactly —
  crediting a coverage cell from a world in which the claim was false. The cost
  the candidate omits is added: `k86` was abandoned because supplying one missing
  kill needed a `const` in 39 module instantiations.
- **5 · measure, freeze, then repair** — **two claims sharing a sentence.** The
  instrument half has five instances across four sessions and became a published
  provenance rule. The predicate half has exactly **one** instance and it is the
  one that named the rule (`k80`'s `wit_FN_24a_…`, 3410 traces → unreached in
  8000 the moment its subject was fixed). Splitting them is the finding.
- **6 · review yield did not decay** — **dropped**, inheriting `review-yield.md`
  rather than re-measuring it, with a second and stronger reason: the observation
  spans three channels and was never about review chains. The substitute is
  stated visibly and then weakened in its turn — the three readers were a
  **relay**, each aimed by the one before it, and the two findings nobody aimed
  at came from a sweep rather than from a reader, which is candidate 4 arriving
  from the other side.

**Three citations did not say what they were said to say, and each is published**
in the artifact's *Where the tree misreports itself*: the root brief's *four
claims in one node* fuses three counts over three populations (the ADR's *three
of the four claims in one retained set*, `finish.qnt`'s *three times*, `k86`'s
*four times*); `k80` and `k84` both cite a flag `rootTakenWithoutCommitted` that
`k85` renamed to `rootTakenWithoutProof`, so the citation dead-ends in the model;
and this leaf's own charter was wrong about candidate 5 in the **favourable**
direction — the predicate-as-subject instance exists, and the correction is that
the candidate is two claims rather than that it is unsupported.

**The link set was resolved rather than eyeballed.** All 14 relative links in the
new document resolve, fragments stripped before resolution — `handoff-audit-k66`'s
note, where a checker that resolved `path#L1234` as part of the path reported 28
broken links of which 25 were fine. The resolver was shown to report a break
against an invented target before its clean run was believed.

**No control handle was invented.** `finish-verdicts-k65` burnt one by publishing
a sweep's negative control into a durable artifact, and the annotation recording
that in `docs/formalism-findings.md` burnt a second in the same paragraph. Both
are already dead; this document names them as dead and coins none.

**Three clusters, and the count is the finding.** (A) what a self-checking runner
owes — candidates 1, 2, 4 and 3's surviving half, which is one coherent
obligation set and goes almost entirely to the write-up, since Grove runs no
model suites; (B) the provenance discipline, which binds to
`content/references/execute.md`; (C) the review chain's structural defect, which
binds to `content/references/decompose.md` and `integrate-review.md` and is the
most actionable thing the harvest produced. Verified before being written down:
`decompose.md:133` really does say *or the findings verbatim*, `execute.md:48`
really does carry the section the freeze rule joins, **neither the freeze rule
nor `a control that has never been seen to fail is not a control` appears
anywhere in `content/`**, and `content/references` is byte-identical to the
provisioned skill copy, so what was read is what ships.

**Child 4 folds into this leaf, and one leaf is cut rather than three.** The
parent's decomposition allows the fold *if the clusters are obvious by then*, and
they are. What it does **not** license is cutting a lesson leaf per cluster: the
root decomposition's step 2 asks for the lessons "stated as claims with the
evidence that supports them and the conditions under which they would be wrong",
and `docs/candidate-lessons.md` **is** that record for all three clusters. Three
further `design` sessions restating one cluster each would produce what this one
has produced, which is what constraint 4 forbids. So step 2 is discharged here,
and the one genuinely unstarted piece is cut: `methodology-changes-k91`, root
step 3, `impl` because it is product change with an embedded payload, a rebuild
trigger and corpus tests. **The write-up (root step 4) is deliberately not cut
here** — its central content is *which lessons bound and which did not*, which is
`k91`'s output and not this leaf's, so `k91`'s body instructs it to cut the
write-up as its last act whatever it decides binds. That is the chain shape
`references/decompose.md` prefers over a body written by a session that cannot
supply its subject.

**The node close for `harvest-the-loop-record-k87`, checked item by item rather
than inferred from the subtree going terminal.** Five of its six `Done when`
clauses are met by an artifact: the structural record and its reproducible
derivation (`docs/loop-record.md`, `scripts/loop-record.py`); the yield question
answered with a number **and** its method, and answered *null*
(`docs/review-yield.md`); the six candidates adjudicated with at least one moved
— three moved (`docs/candidate-lessons.md`); the cost account in raw form
(`loop-record.md` §*Cost account*); and nothing in `.grove/` edited beyond this
leaf's own body, the leaf it cut, and the promotion below.

**The sixth is discharged differently, and it is recorded rather than glossed.**
*The lesson leaves for step 2 are cut* asked for one leaf per surviving cluster,
each producing a durable record. The clusters are three and they are named — but
**the record those leaves were to produce already exists**, for all three, with
the evidence and the falsifiers, in `docs/candidate-lessons.md`. Three further
`design` sessions restating one cluster each is exactly the make-work
decomposition constraint 4 forbids, and the root brief says of its own shape that
it is *intent, not a tree*. So step 2 is discharged by the artifact and step 3 is
cut as a leaf. This is a scope judgement a human may disagree with; it is
deliberately visible in the diff, which `references/retire.md` says is where a
close is reviewed.

**Promoted upward into the root brief, which is the close's fourth step.** Two
edits, both to live guidance and neither to a retired body — and neither can move
a published count, because both derivations are pinned to a change id. Its
*Pointers* now names the three `docs/` documents **ahead of** `.grove/`, since a
remaining leaf should read the harvest rather than re-walk the tree; and its
closing hazard note records that the embargo's condition is **met** — stating
what discharges it (the derivations re-run after teardown, because finish removes
`.grove/` from the tip and not from history) without lifting anything, since
whether a grove is finished is the driver's call and never a leaf's.

**One residue is named and handed on rather than absorbed.** Nothing outside
`docs/` links `candidate-lessons.md`, and the only file that does is the root
brief, which finish deletes — `handoff-audit-k66`'s own failure condition, *a
link that only works from `.grove/` is a failure*. It is not fixed here because
the two obvious anchors are hand-edit-forbidden derived files whose pointer
belongs in a script template, which is `methodology-changes-k91`'s register and
not this leaf's. Written into `k91`'s `Done when` with the constraint attached.
