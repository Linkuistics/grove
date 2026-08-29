# The candidate lessons, adjudicated

Six claims were written down at the end of this repository's formal-modelling
campaign, in the root brief of the workstream that harvested it, as *what the
campaign looked like from inside it*. They were written by a session that is not
neutral about the campaign, from recollection, and each was marked as a claim the
harvest must **either evidence or drop**.

This is the adjudication. One verdict per candidate, each naming the file and
section that decides it and **what would have falsified it**.

> **The models this record adjudicates no longer exist.** They were deleted with
> the rest of the formal-methods apparatus once the campaign's lessons had been
> distilled into the `linkuistics` skills (`delete-formal-models-k29`), along
> with the runner `models/run.sh`, its controls, and the claim catalogue
> `docs/specs/semantic-contract.md`. This document was kept precisely because the
> skills cite it for their authority: a rule whose evidence has been deleted is a
> rule nothing can falsify. So every model path named below is a description of
> where a verdict was reached, not a link a reader can follow.

**Three of the six moved.** One is dropped outright, one is materially weakened
with half its sentence falsified, and one turns out to be two claims sharing a
sentence and resting on very different amounts of evidence. The three that stand
are stated more narrowly than they were written, and one of them is stated more
*strongly*.

It is the third of three durable records this harvest produced, and the only one
that is judgement rather than derivation. [`loop-record.md`](loop-record.md)
enumerates the sessions; [`review-yield.md`](review-yield.md) counts the review
findings; this one reads the **model artifacts** the other two deliberately do
not open, and decides what the campaign is entitled to claim.

## What was checked, and how

**Every citation below was opened at the source.** The bar is the one this tree
set for itself and then failed twice: `experiment-baseline-k29`'s findings 3 and
4 both found `docs/preservation-baseline.md` citing a test file for what its
*name* suggested rather than what it *asserts* — `tests/migration_transition.rs`
offered as refusal evidence when it checks legacy-format refusal,
`tests/env_hygiene.rs` offered for spawned-child hygiene when it protects the
test runner's signal channel. So a claim here is not credited to a task body that
describes a model; it is credited when the model says it.

**Three kinds of source, and they disagree in useful ways.**

- **`.grove/` task bodies** — what a session recorded on its way out. Complete
  for producers, and for the four integrations that wrote back into their own
  body. **These citations survive teardown even though `.grove/` does not**:
  every session is cited by its permanent `<slug>-k<key>` handle,
  [`loop-record.md`](loop-record.md) §*Every session* maps each handle to its
  change id, and `jj show <change>` reaches the body and its `DONE` filename in
  history. Finish removes `.grove/` from the tip, not from history.
- **Commit messages** — where five of the nine integrations recorded their
  per-finding dispositions instead. `review-yield.md` §*Five of nine dispositions
  are not in `.grove/` at all* is why this document quotes commits as often as
  task files.
- **The committed models, the runner and the ADR set** — the subject. These
  survive teardown; `.grove/` does not.

**What this document is not permitted to do**, and the rule comes from the leaf
that broke it: `finish-verdicts-k65` ran a citation sweep whose negative control
was an invented handle, and wrote the result into a durable artifact — so the
handle went from 0 sites to 1 by being reported. **A control handle, once named
in a durable artifact, stops being a valid negative control**
([`formalism-findings.md`](formalism-findings.md), the annotation at the
`finish-verdicts` handoff). This document therefore invents no control handle and
states no count of itself. The two already burnt are named there and are not
re-used.

## The verdicts

| # | the candidate, as written | verdict |
|--:|---|---|
| 1 | A green suite is not evidence; only a control that can kill the claim is | **evidenced, and strengthened** — the corpus demands more than the candidate does |
| 2 | A claim stated over the model's own history or classifier is self-certifying, and the repair recreates the hazard one level down | **evidenced in full**, and it generalises to the loop — on one of the two loop-level instances offered, not both |
| 3 | False greens are found by narrowing, not widening | **materially weakened**; *not widening* is **falsified** |
| 4 | A module that changes what the model does must be run against every claim the model has | **evidenced on one node**, and carried past anecdote by a structural mechanism rather than by repetition |
| 5 | Measure, freeze, then repair — a predicate is a subject too | **two claims sharing a sentence.** The instrument half is heavily evidenced; the predicate half rests on exactly one instance |
| 6 | Review yield did not decay | **dropped** — inherited from `review-yield.md`, with the substitute stated and then weakened in its turn |

---

## 1 · A green suite is not evidence; only a control that can kill the claim is

> *One node met four claims that were true by construction and green.*

**Verdict: evidenced, overwhelmingly, and the candidate understates its own
finding twice** — once in the count, and once in what a control has to do.

### The evidence

**Fifteen named obligations, across three scopes and both independently built
model families, plus three defects in the runner itself and four controls that
fired for the wrong reason or did not fire at all — found by ten different
sessions.** Enumerated rather than counted, because a count is not checkable:

| obligation or instrument | how the green was empty | found by |
|---|---|---|
| `quint verify` exit handling | a non-zero exit whose wording was not on a five-string list was recorded as *model-checked, no counterexample*. Measured: `JVM_ARGS=-Xmx6m` killed the JVM before it read its own jar and the runner reported **43 verified properties** | `task-tree-k55`, commit `xwzuynow` |
| `quint run` exit handling | a non-zero exit read directly as *violated* — which in a premise-break control is recorded as the control **passing**: a dead tool reporting that an assumption carried weight | same |
| reverse obligation coverage | checked an obligation's **shape**, never the manifest, so `inv_TT_99_invented` counted under a key nothing reads | same |
| `FN-25.b` | claimed a comparison between two encodings of the diagnosis partition and made none — every block site derived its diagnosis from the classifier it was being compared with | `finish-k57` B1 |
| `FN-24.a`'s `Current(Live)` branch | an undeclared vacuous classification branch: `liveOrdinaryWork` hard-coded `false` | `finish-k57` B4 |
| `FN-14`, `FN-26`, `FN-30` | asserted over fields no transition or mutant could make bad — `unrelatedMutated` and `historyRewritten` initialised false and only read; the commit setting `hooksRan` false regardless of input | `finish-k57` B5 |
| every `SY-` property | `driverStep` forced each launched finish session through teardown, so **decline, early failure, no signal and the reopening exit were all unreachable**: the whole scope was checked over a world in which a finish session cannot decline | `system-k59` F1 |
| `SY-09.a`, `SY-09.b` | `mutant_no_signal_is_done` was documented as a bundle control over three endings and changed one arm, leaving both with **no control at all** | `system-k59` F3 |
| `SY-14.a` | the sweep read *some action returned `Applied`* as *a block was cleared*; `mutant_block_clears` **had been firing through the defect** | `system-k59` F4 |
| `TT-20` | `mutant_scaffold_absence_only` reported no violation over 4000 traces and the invariant was fine: written at `FOCUS = 4`, whose action menu is inserts only, so `root-init` never ran | `task-tree-scope-k70` |
| `SCPartialAmbiguous` | Quint's `match` has no nested constructor patterns and typechecks anyway: `RSPartialScaffold(PSExact)` binds a *variable*, leaving the arm below dead. The suite reported the state unreachable while `classifyTree` was returning it — **a green run describing a state it could not see** | `lifecycle-scope-k72` |
| `SY-06.b` | completion flags drafted over the classification — a restatement of the gate the obligation is about; `mutant_absent_witness` went **green** under them | `lifecycle-scope-k72` |
| `FN-32` | Quint's `inv_FN_32` read the aggregate `hist.mutatedUnproven` and was **character-for-character `inv_FN_10b`**, the incumbent-mechanics claim it was created to escape | `obligation-placement-k67` F2 → `k68` |
| `FN-32`, again | `relax_EN_03` left `OWNERSHIP_PROVEN = true`, and every `noteUnproven*` is guarded on `not(OWNERSHIP_PROVEN)`, so `mutatedUnprovenSlot`/`Marker` — the **whole content** of the invariant — are unsettable in every module that leaves the dial true | `finish-verdicts-k77` R1 → `k78` |
| `FN-25.a` (Alloy) | `FN_25a` read `lone diagnosedRaw or declaredDiagnosisOverlap`, exempting the two overlap classes it had itself declared. Measured: with the precedence relation **fully reversed**, the exempted check stayed green | `finish-scope-k75` → `k76` |
| `FN-25.a` (Quint) | stated as *the two diagnoses are disjoint*, which `diagnose`'s if/else chain made true by construction — **no mutation could have moved it** | same |
| `FN-24.a` | its two *failable* conjuncts guarded on the incumbent's own artifacts, in **both independently built families**; a candidate protocol holding neither makes both vacuous | `honest-classification-k80` |
| `FN-28` | its second operand read `(ATOMIC_DISPOSAL or hist.appliedAfterQuarantine)` — **an enumeration of the two protocols that happened to exist** | `k80`'s in-session reviewer |
| `FN-28`, again | the repair was itself satisfied by construction: two history flags written only by the three root-removal sites the invariant asks about, with no control able to make a site omit either | `honest-classification-k84` finding 1 |
| `FN-22.e` | `renamedWithoutCommitted` is written at exactly one site, reachable only on `d == DCommitted` — so the flag is **unreachable in `base`: 0 of 8000 traces** at `--max-steps=24` with every environment kind admitted | `honest-classification-k85`, externalised as `quarantine-gate-control-k86` |

Two of those entries deserve to be read twice. The runner's `quint verify`
defect meant a **dead tool** produced a clean coverage report; and the
`system-k59` F1 defect meant an entire scope's claims were green over a world
that could not exhibit the behaviour they were about. Neither is a subtle
modelling slip.

### Where it is landed durably

[`a-closed-partition-is-over-outcomes-not-states`](adr/a-closed-partition-is-over-outcomes-not-states.md)
— *a check that exempts its own declared counterexamples tests nothing where the
claim is hardest*, with the reversed-precedence measurement that proves it. And
[`a-shared-safety-claim-names-the-role-not-the-artifact`](adr/a-shared-safety-claim-names-the-role-not-the-artifact.md)
— *vacuity is invisible from inside a green suite, and it reached three of the
four claims in one retained set.*

### The strengthening, and it is the useful part

The candidate says a control that **can** kill the claim. The corpus says that is
not enough, on its own evidence:

- `mutant_block_clears` **was** killing `SY-14.a` — through a defect in the
  sweep rather than through the mechanism it named (`system-k59` F4).
- `mutant_unproven_ownership` **was** killing `FN-32` — always through the
  witness slot, never through the cleanup marker the claim also named, because
  its environment (`ENV_PHASES = Set(0,1,2)`, `ENV_FOREIGN = Set(0)`) never
  reaches a foreign marker (`obligation-placement-k68` F2).
- `relax_EN_13` **was** violating `inv_FN_28` — under the old operand *and* the
  new one, "which is evidence that the new operand was never exercised rather
  than that it was controlled" (`honest-classification-k84` finding 1).

So the rule that survives contact with the evidence is narrower and testable:

> **A control earns a claim only when it is shown to kill that claim for the
> reason claimed.** The measurement that establishes it is the *isolating* one —
> run each operand as its own predicate over the same module and show the others
> stay green.

That is landed, in those words and with its measurements, in
[`finish.qnt`](../crates/grove-finish/models/finish.qnt)'s `inv_FN_28` comment:
`inv_fail_EN_01_FN_28_a_torn_rename_is_not_a_removal` and
`mutant_status_classifier`'s kill, "each measured green on the *other* operand
over its own module, which is what makes them isolating rather than merely red."

### What would have falsified it

Mutation controls added retroactively to claims that had been green for months,
and killing all of them — i.e. the greens turning out to have been evidence after
all. It never happened once. Every session that went looking for a false green
found one.

### The limit, stated because the claim's shape hides it

**This corpus cannot say what fraction of greens were sound**, because it never
ran an arm without controls. What it can say is the hit rate of *looking*, and
the hit rate was 100% across ten sessions. That is an argument for looking; it is
not a measurement of how much rot there was.

---

## 2 · A claim stated over its own history or classifier is self-certifying

> *…and the repair — record the fact at the step that establishes it —
> recreates the hazard one level down, so each recorded fact owes a control that
> makes a step omit it.*

**Verdict: evidenced in full — the only candidate whose entire sentence,
recursion and derived obligation included, is landed verbatim as a decision
record.** And it generalises to the loop, on one of the two loop-level instances
that were offered for it.

### The first clause

Four instances, three of them in different scopes: `FN-24.b`'s enumeration
(`ALL_STEPS.forall(s => … or DECLARED_STEPS.contains(s))` — "whether the
candidate's removal step passes is a modeller's choice about a static table",
`quarantine-necessity-k79`'s brief); `SY-06.b`'s completion flags stated over
`classifyTree`, under which an existing control went green
(`lifecycle-scope-k72`); `FN-25.b` comparing a classifier with itself
(`finish-k57` B1); and `TT-24.c`'s uncontrolled Quint transcription
(`catalogue-disposition-k64`'s brief).

### The recursive clause — one complete cycle, over three sessions

This is the load-bearing evidence and it is a single instance, so it is named as
one:

1. **`honest-classification-k80`'s in-session reviewer** found `inv_FN_28`'s
   second operand was an enumeration of the two protocols that existed.
2. **`k80` repaired it by recording the fact at the step that establishes it** —
   `rootTakenAway` and `rootTakenWithoutCommitted`, written by each root-freeing
   step.
3. **`honest-classification-k84` finding 1 found the repair self-certifying**:
   the flags "are written only by the three root-removal sites and are then read
   directly by `inv_FN_28_one_successful_exit`. There is **no independent control
   that can make one of those sites omit or falsify either flag**."
4. **`honest-classification-k85` repaired that** with two isolating kills, each
   measured green on the other operand.

### The derived obligation, landed as a decision

[`a-shared-safety-claim-names-the-role-not-the-artifact`](adr/a-shared-safety-claim-names-the-role-not-the-artifact.md),
closing section:

> **A green with no available kill is not a pass, and that is the rule's second
> half.** A role-form operand is a fact the model's own steps record, so it is one
> edit away from being satisfied by construction; every one of them therefore
> owes a control that makes a step omit or falsify it.

### Does it generalise to the loop? Yes, on one instance and not the other

Two loop-level instances were offered. **They are not the same shape, and the
corpus already treats them as two rules.**

**The sweep whose report lives inside its own subject — *not* an instance.**
`finish-verdicts-k65`'s citation sweep used an invented handle as its negative
control and published the result into a durable artifact, so the handle went from
0 sites to 1 by being reported. That claim did not become unfalsifiable; it
became **false**. It is self-*invalidation*, the mirror image, and
`references/execute.md` already carries it under its own name — *never document a
claim with a count of itself*. `honest-classification-k80` hit the model-side
version of the same thing (a provenance sentence reading *GAP-line count 5 both
times*, which `grep -c GAP` then made 6). Same family, opposite sign.

**The integration that verifies a charter the reviewer wrote — an instance, and
an exact one.** `review-yield.md` §*A 100% survival rate is what a broken
instrument reads*: in five of nine chains the integrating session's task body was
written by the reviewer, so the charter **is** the finding list and `Done when`
is the finding list restated as obligations. The pass condition of the
verification is produced by the thing being verified. It has the signature too —
**45 findings raised, 45 verified, zero rejected**, and no tree-level review
finding has ever been rejected in this grove.

**And the recursion holds at the loop level, which is what makes it one rule
rather than an analogy.** `review-yield.md`'s proposed repair is that an
`integrate-review-*` body should carry the review's **handle** and read the
findings from the review's own commit. That moves the hazard one level down
rather than removing it: the finding list is still the reviewer's, so what the
integration still cannot do is notice a finding the reviewer never wrote — and
that residue is already published, in the same document's *It cannot see a
finding nobody wrote down*. The control the third clause demands would be a
mechanism by which an integration can reject a finding; the corpus shows one
exists **only in the in-session channel** (4 of 24 dropped, 17%, against 0 of 45
at tree level), plus `finish-verdicts-k78`'s reviewer falsifying a finding
`obligation-placement-k68` had raised itself.

### What would have falsified it

A claim stated over a fact the model's own steps record, with a control that
makes a step omit it, arriving *without* anyone having to be told — i.e. the
hazard being one people avoid unprompted. Instead the same node hit it three
times at three different places in a claim's expression (`FN-32`'s antecedent,
`FN-24.a`'s guard, `FN-28`'s consequent) and a fourth time in the repair.

---

## 3 · False greens are found by narrowing, not widening

> *A shared-safety claim was violated in a strict subset of the widest world's
> traces, which that world's 8000 samples had never drawn. A property checked in
> the widest world is not therefore checked hardest; a wide environment dilutes
> the sampler.*

**Verdict: materially weakened. The supporting instance is real and verified at
the source; the generalisation inverts the corpus's own ratio five to one; and
the *not widening* half is falsified twice.**

### What is true, and it is verified in the model rather than in a task body

[`finish.qnt`](../crates/grove-finish/models/finish.qnt), the `Hist` comment
above `rootTakenAway`:

> MEASURED, not argued: with every `EN-` assumption granted and every model
> mutation off, in a world that narrows `base`'s environment to topology changes
> (a strict SUBSET of `base`'s traces, so the counterexample is `base`'s too),
> `inv_FN_28_one_successful_exit` is VIOLATED … `base`'s 8000 samples never drew
> it.

The mechanism is sound and the subset argument is what carries it.

### What is not true

**In this corpus, narrowing is the dominant way false greens were *made*, not
found.** Five against one:

| the narrowing | what it produced |
|---|---|
| `mutant_correlation_wins_the_overlap` copying its neighbours' `ENV_KINDS = Set(0)` | **green**, because the overlap is reached only by an in-transaction hand edit. The comment recording the measurement is in place at [`finish-controls.qnt`](../crates/grove-finish/models/finish-controls.qnt), above `ENV_KINDS = Set(1)` |
| `mutant_unproven_ownership`'s `ENV_PHASES = Set(0,1,2)`, `ENV_FOREIGN = Set(0)` | never reaches a foreign cleanup marker at all, so its recorded kill was always the witness slot's |
| `relax_EN_03`'s `ENV_BUDGET = 0` with `ENV_PHASES`/`ENV_KINDS` empty | `foreignWriteAt` can never fire, so `FN-32` was asserted over a world with nothing to be about |
| `mutant_scaffold_absence_only` at `FOCUS = 4` | no violation over 4000 traces, from a control that could not reach the situation it mutates |
| `scenario_in_place_march` at `ENTRIES = 1` with no environment action | hid an un-evacuated-entry deletion from the session that wrote it — *"the narrowing I had just argued was safe was also what hid a bug from me"* (`honest-classification-k80`) |

And the same session-pair supplies both signs: `k80` narrowed and hid a bug from
itself; `k85` narrowed and found one.

### *Not widening* is falsified, twice

- **`TT-20`'s dead control was repaired by widening.** `FOCUS = 4` → `FOCUS = 5`,
  and it fires in 1.5 s (`task-tree-scope-k70`).
- **A declared impossibility was killed by running the deeper attempt.**
  `finish-scope-k71` argued Alloy's `EN-08`/`FN-31.c` cell unmeetable from an
  estimated seventeen states against a thirteen-state bound. Run, the disk is
  reached in **fourteen** states in 8.9 s at a 16-step bound — and two commands
  already in the file had reached both boundaries at eleven and twelve states,
  "so the general claim was contradicted twice over inside the file that made
  it" (`finish-scope-k76`). The catalogue's assumption table gained the rule that
  came out of it: *a family's failure to meet an exercise-removal row is
  established by running the deeper attempt, never by costing it in prose.*

### What survives

> **A narrowed world is evidence about the wide one only when the narrowing is a
> strict subset that still reaches the subject.** Both halves are load-bearing
> and this corpus violated each of them. The subset property is what makes a
> *violation* found narrow a violation wide, and it has to be established by hand
> — `honest-classification-k80` did so for `scenario_in_place_march` ("the module
> differs from `base` in exactly one non-search `const`") and
> `finish-verdicts-k78` had to do so for `relax_EN_03` after the fact.
> Reachability is what makes a *green* mean anything, and five modules failed it.
> So narrowing is a sound way to hunt a counterexample the sampler never drew,
> and an unsound way to read a green.

The corpus's own compression of this, reached independently three times, is
already the better sentence: **a mutant module's environment is part of the
control.**

### What would have falsified the surviving form

A narrowed module reporting a violation that does not reproduce in the wide
world. None did — but only because in every case the subset property was checked,
twice by hand and after the fact.

---

## 4 · A module must be run against every claim the model has

> *…not the ones it declares — the module rule hides failing claims by design.*

**Verdict: evidenced. It survives on one node and two sweeps, and it says so
here; what carries it past anecdote is that the mechanism is structural and
written down, not that the instances repeat.**

### The mechanism, verified at source

`models/run.sh`, which was the **only** place the rule was defined, and which
was deleted with the campaign's apparatus (`delete-formal-models-k29`). The
rule, quoted here so that it survives its definition site:

> a `relax_`, `mutant_` or `scenario_` instance carries ONLY the commands written
> inside it. Each exists precisely because some obligation behaves differently
> there, so inheriting the library's would assert the opposite of what the
> instance is for.

The rule is correct and the reasoning is correct. The hazard is its shadow, which
is exactly what *by design* means in the candidate's own wording.

### The consequence, which is the sharpest sentence in the tree

`honest-classification-k80`, on why its green suite could not see a shared-safety
claim it had just falsified:

> a `scenario_` module carries only the commands written inside it, and this one
> declared `wit_FN_28_…` while never checking `inv_FN_28` — **crediting `FN-28`'s
> coverage cell from a world in which `FN-28` was false**.

The runner's two-direction coverage assertion — the instrument built precisely to
stop a cell being credited without evidence — passed while the cell's evidence
came from a world where the claim was untrue.

### The two applications, both in one node

1. **`honest-classification-k80`'s in-session reviewer** swept all 63 library
   `inv_` commands against the candidate's module and found exactly two violated:
   the intended kill, and `FN-28`. The second is how it surfaced at all.
2. **`honest-classification-k85`'s sweep**, run to verify `k84`'s findings,
   "returned two more" the review did not have: `inv_FN_28` violated with no
   model mutation, and `inv_FN_25b` red under the in-place candidate,
   pre-existing.

**Two sweeps, four unlooked-for findings, no clean sweep.** Both in
`quarantine-necessity-k79`, both against the same candidate module.

### What would have falsified it

A sweep of the library's claims against a module returning only that module's
declared kills. The corpus never ran the sweep and got nothing back.

### The cost, which the candidate does not mention and the tree measured

Reading a module against every claim is cheap; **repairing what it finds may not
be.** `quarantine-gate-control-k86` was cut to give `FN-22.e` an available kill,
and abandoned: the `const` that supplies it "must be added to **every** module
instantiation (2 in `finish.qnt`, 37 in `finish-controls.qnt`)". The obligation is
to *run* the sweep and record what it says. It is not an obligation to fix
everything it finds in the same session.

---

## 5 · Measure, freeze, then repair — a predicate is a subject too

> *A witness written by calling the definition it is about stops measuring the
> moment that definition is fixed.*

**Verdict: two claims sharing a sentence, and joining them makes the specific one
look better supported than it is.** Both are evidenced. They are about different
things and they rest on very different amounts of evidence, so this document
splits them and the write-up should carry them apart.

### 5a · The instrument half — heavily evidenced

*A suite is an instrument, and an instrument you adjust mid-reading has not read
anything.* Five instances across four sessions, and it became a published
provenance rule rather than a recollection:

- **`obligation-placement-k63` paid two hours for it.** Three scope runs were
  launched and invalidated: `models/run.sh` edited in place while all three were
  executing it, and `task-tree.als`/`.qnt` while the task-tree run read them.
  "A bash script rewritten under a running shell, and a model file rewritten
  under a running `exec` loop, are both undefined rather than merely untidy."
- **`obligation-placement-k68` destroyed a measurement the other way.** Two
  mutation-64 sweeps ran concurrently against one log file — the first launched
  with a trailing `&` inside a tool invocation and reported complete when the
  invocation returned. The log showed `KILLED FN_31d` and then did not: "a
  reading that was not merely incomplete but self-contradictory." Rule: **one
  measurement, one writer, and never infer that a background job is finished
  from the return of the thing that launched it.**
- **`finish-scope-k76` froze five subjects with digests either side and killed a
  third attempt** because a comment-only edit landed while it was in flight —
  "the freeze is the point of the digest, and a run whose subject moved under it
  is not a measurement of the file it reports on, however harmless the edit
  looks." It also recorded its first Alloy run rather than overwriting it, when
  re-reading its own edit found a deleted clause was checking something true.
- **`honest-classification-k80` digested four run subjects either side**, wrote
  the README row **after** the run, and confirmed a re-run **command by command**
  rather than by matching totals — "since two runs can agree on a total while
  disagreeing on which command did what."
- **`obligation-placement-k68` stopped a whole-repository run two minutes in** to
  land one comment edit, "because a comment could not have changed the result,
  and that is exactly why it was cheaper to restart."

### 5b · The predicate half — one instance, and it is the one that named the rule

`honest-classification-k80`. The witness measuring `FN-24.a`'s defect was written
as `… and classifiesHonestly(w, t)` — a witness **calling the definition it is
about** — and went from reached in 3410 traces to **unreached in 8000 samples**
the moment the repair landed:

> not because the defect went away but because the sentence had come to mean
> something else … a run whose subject moved under it is not a measurement of the
> thing it reports on, and a **predicate** is a subject too.

Repaired by writing the four pre-repair conjuncts out **inline** and renaming to
`wit_FN_24a_the_artifact_guarded_encoding_accepts_it`, which
[`a-shared-safety-claim-names-the-role-not-the-artifact`](adr/a-shared-safety-claim-names-the-role-not-the-artifact.md)
now cites as "the same disk under the pre-repair encoding, **frozen inline**: an
A/B on one world."

**That is the whole of the evidence for 5b, and it is where the sentence came
from.** It is a corollary of 5a rather than a second instance of it: 5a is about
a file moving under a run, 5b is about a *definition* moving under a measurement
of that definition. The corollary is worth keeping — it is the cheaper failure to
miss, because nothing errors and the number simply changes — but it is one
instance, and stating it in the same breath as five borrows their weight.

### What would have falsified either

For 5a: a measurement taken against files edited mid-run that turned out to
reproduce. Two of the three were re-run and one produced a self-contradictory
log, so no. For 5b: a witness that calls its own subject and keeps measuring
across the repair — which is exactly what did not happen, in the one case
observed.

---

## 6 · Review yield did not decay

> *A producer's own reviewer found three substantive defects in a green suite;
> the review leaf beside it found five more in the repairs; the integration found
> two more while applying them.*

**Verdict: dropped.** [`review-yield.md`](review-yield.md) measured it across all
nine chains and the answer is the null one: **nine chains and three channels
cannot separate flat from falling.** That measurement is inherited, not re-opened.

Three things stop the curve being readable, and the first is the least
interesting: the 6, 6, 7, 6, 6, 3, 4, 2, 5 sequence is ≈1.8σ before any confound;
the subjects are not matched and not randomly so; the review **rate** rose while
the yield fell, because review is a function of session kind (`design` 42%,
`prototype` 13%) and the loop reviewed further down its own value curve as it
went. And the unit is elastic — sub-corrections per finding run 1.0 to 3.3, and
consequence is not captured at all.

### The second reason for the drop, and it is stronger than the null result

**The observation was never about review chains.** Three of the ten findings came
from a producer's *in-session* reviewer, five from a tree-level `review-design`
leaf, and two from the `integrate-review-design` step — three channels, and one
node. Read as a claim about review chains it generalises from a single node.

### The substitute, stated visibly rather than folded in

The original is not rewritten into its replacement. What the node actually
describes is:

> *Three independent readers of one artifact each found what the two before them
> had not.*

### And the substitute is weakened in its turn

Two things, and the second changes what the lesson is:

1. **It is one node, and it is the node the loop selected for review because it
   was the hardest.** `honest-classification-k84`'s own body says this is "the
   third time this node has taken" the second-review signal. That is selection,
   not sampling.
2. **The three readers were not independent of each other — they were a relay.**
   `k84` was cut by `k80`, whose body writes the specific doubt into it: *"The
   `FN-28` restatement is specifically attacked for construction-satisfaction."*
   `k84`'s sharpest finding is that doubt, answered. `k85`'s work list is `k84`'s
   five findings. Each reader was **aimed** by the one before it.

   And the findings nobody aimed at came from an **instrument**, not from a
   reader: `k85`'s two extra findings both came out of the sweep it ran to verify
   finding 1 — which is candidate 4, arriving from the other side.

So the transferable claim, on one node and stated as such:

> **What a review chain bought here was the handoff, not the head count.** The
> value came from each session writing a specific doubt into the next session's
> body, and from the instruments those sessions ran — not from the number of
> fresh contexts. A chain of three readers who were not aimed is a different
> experiment, and this tree did not run it.

### What would have falsified the drop

A yield curve with a gap outside the noise across chains of matched subject size.
`review-yield.md` puts that at roughly 25 chains, and adds that more chains is the
**weaker** fix: without a pre-registered severity scale, a longer run measures
more enumeration style rather than more review.

---

## Where the tree misreports itself

Three of the citations offered to this leaf did not say what they were said to
say. All three were caught by opening the file.

**1 · The root brief's "one node met four claims" does not reproduce, because
there are three different counts over three different populations.**

- [`a-shared-safety-claim-names-the-role-not-the-artifact`](adr/a-shared-safety-claim-names-the-role-not-the-artifact.md)
  says **three of the four claims in one retained set** — `FN-32`, `FN-24.a` and
  `FN-28`, of the retained set `FN-20`, `FN-24`, `FN-27`, `FN-32`.
- [`finish.qnt`](../crates/grove-finish/models/finish.qnt)'s `inv_FN_28` comment
  says the shape is one "this node has been burned by **three** times".
- `quarantine-gate-control-k86` says "**four** times", counting `TT-24.c` and
  `SY-06.b` from a different child and treating `FN-22.e` as the fifth.

The candidate's illustration fuses the first and the third. **The class is
evidenced far past four either way** — fifteen named obligations, above — so
nothing turns on it; but the arithmetic in the recollection is not the
arithmetic in any artifact, and that is the failure mode the harvest exists to
catch.

**2 · A citation chased into the model finds a flag that no longer exists.**
`honest-classification-k80` and `k84` both name `rootTakenWithoutCommitted`. The
committed source says `rootTakenWithoutProof` — renamed by `k85`'s repair, along
with the reason for the rename. A reader following the task body into
`finish.qnt` finds nothing under the cited name.

**3 · The framing this leaf was handed about candidate 5 was wrong in the
favourable direction.** Its charter said candidate 5's instances "are about
running an instrument, not about a predicate being a subject", and asked whether
the wording and the evidence were about the same thing. They are: the
predicate-as-subject instance exists, in `honest-classification-k80`, and it is
where the sentence was written. The correction is not that the candidate is
unsupported — it is that it is **two** claims, one of which is supported five
times and one of which is supported once.

## What binds, and what only gets written down

Three clusters, and they are named here because deciding how many there are was
this leaf's job.

**A · What a self-checking runner owes** — candidates 1, 2, 4 and candidate 3's
surviving half. A control per claim; a control shown to fail, and to fail for the
reason claimed, by the isolating measurement; a module's environment counted as
part of its control; every module run against every claim rather than the ones it
declares; and no claim stated over facts its own mechanism records without a
control that makes a step omit them. This is one coherent obligation set for any
suite that produces evidence. **Almost all of it goes to the write-up**, because
Grove's methodology does not run model suites — with one exception, below.

**B · The provenance discipline** — candidate 5a. Finish all edits, then measure;
digest every subject before and after; one measurement, one writer; a run whose
subject moved under it is not a measurement; never infer a background job's
completion from the return of the thing that launched it. **This binds**, as an
addition to `references/execute.md`'s *Verifying a claim about the repo itself* —
it is generic to any session that runs anything and reports a number, which is
most of them.

**C · The review chain's structural defect** — candidate 2 at the loop level, and
candidate 6's residue. An `integrate-review-*` leaf whose body is the review's
finding list has no structural place to reject one. **This binds, and it is the
most actionable thing the whole harvest produced**: the integration's body should
carry the review's *handle*, and its findings should be read from the review's own
commit — the handoff a `review-*` step already uses. It touches
`references/decompose.md`, which until `methodology-changes-k91` explicitly
invited the defect ("…or the findings verbatim"), and
`references/integrate-review.md`.

The one piece of cluster A that binds with them: **a control that has never been
seen to fail is not a control** — already this corpus's own rule, applied by
`obligation-placement-k68` to `models/run-controls.sh` and by
`review-yield.py`'s three deliberate mutations, and not yet stated anywhere in
the methodology that asks sessions to verify their claims.

**Where the three landed, and what did not.** `methodology-changes-k91` wrote all
three into `content/`: A′ into `references/execute.md`'s two-controls block, B as
that file's *The provenance of a measurement* section, and C across
`references/decompose.md` (what a review writes into the body it cuts) and
`references/integrate-review.md` (where an integration reads findings from). Six
rows were added to `docs/specs/corpus-rule-ownership.md` and its sweep in
`tests/rule_ownership.rs`. **5b did not land**, on this document's own reasoning:
it is one instance, and stating it beside 5a's five in a rule a session is asked
to obey is exactly the borrowing §5 split them to stop. It and the rest of
cluster A go to the write-up, which is
[`driving-a-checkable-loop.md`](driving-a-checkable-loop.md): the account of what
the campaign cost, the three rules as they now ship, and the five claims that
only get written down — this document's §5b among them, with the reason for each
decline.

## What this record does not establish

- **It adjudicates six claims, not the campaign.** Lessons the campaign learned
  and nobody wrote into the root brief are outside its scope by construction.
- **It reads the models as evidence for a lesson, not as models.** Where a model
  artifact and a candidate disagreed, the model artifact was treated as the
  subject and the candidate as the claim under test — but no model was re-run and
  no measurement here is new.
- **One grove, one subject matter, one operator.** `review-yield.md`'s closing
  caveat applies to every verdict above: every instance is formal-modelling work
  in a repository whose whole purpose was rigour about evidence. The hit rates
  are hit rates *here*.
- **The verdicts are judgements and the enumerations are hand-built.** Neither is
  derived by a script that asserts coverage in both directions, which is what
  separates this document from the two beside it. What replaces that is the
  citation: every claim above names the file that decides it, and every
  load-bearing one was opened.
