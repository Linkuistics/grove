# pilot-preregistration-k53

**Reviews:** pilot-preregistration-k24

## Goal

Read `docs/evaluations/editorial-pipeline-pilot/preregistration.md` adversarially
and report findings. It is the instrument six later sessions are bound by and no
human sees before they run, so the question is not whether it reads well but
whether a session can satisfy each of its four parts by reinterpretation, and
whether the verdict it produces would survive being disbelieved.

## Context

- The artifact is one document; the producer's commit is the diff to read it in.
  Its own reasoning, with the rejected alternatives, is in
  `pilot-preregistration-k24`'s `## Decisions (running log)`.
- **You sit ahead of the drafting on purpose.** `pilot-preregistration-k24`'s
  notes required it: a review that arrives after `jj-workspace-book-k25` is a
  report on the experiment rather than a guard on it, and by then the instrument
  has already been read. Any `integrate-review-design` you cut is
  `leaf-insert`ed at `jj-workspace-book-k25` for the same reason, never appended.
- The in-house standard for this kind of artifact is
  `docs/evaluations/writing-code-walkthroughs/` — its
  [rubric](../../docs/evaluations/writing-code-walkthroughs/baseline/rubric.md)
  for what a frozen instrument states in advance, and its
  [README](../../docs/evaluations/writing-code-walkthroughs/README.md) *Verdict*
  for what an honest negative result reads like. The preregistration claims to
  follow both; check that it does rather than that it says so.
- The stages it binds are the six leaves under `pilot-measure-k26` plus
  `jj-workspace-book-k25`. Each of their `## Done when` sections defers to "the
  form the preregistration's attribution rule requires", so a stage obligation
  the document leaves unstated is unstated everywhere.

## Done when

Findings are reported against the artifact, with no fixes applied. The specific
doubts this leaf exists to press, none of which the producing session could test
on itself:

- **Is the comparator honest?** The two-stage alternative is never run; it is
  simulated by a blind proof-only probe over the draft. Ask whether that probe
  can bear the weight the decision rule puts on it, whether its four stated
  biases are the real ones and pointed the way the document claims, and whether
  its fallback — the report session producing the probe itself — is a fallback
  or an escape hatch.
- **Can a stage satisfy the attribution rule while recording nothing useful?**
  The claim ledger is self-reported and verified against the diff afterwards.
  Find the gap that verification does not close.
- **Are the taxonomy's seventeen classes decidable by a session that did not
  write them?** A class whose observable test needs the author's judgement to
  apply is a class two stages will apply differently, and the document forbids
  extending the taxonomy mid-campaign.
- **Does the decision rule's threshold of three do the work claimed for it?**
  It is asserted rather than derived. Ask what result it would produce on a
  plausible book, and whether a stage can be pushed across it by splitting or
  merging claims despite the de-duplication rule.
- **Is `Undetermined` genuinely inescapable?** Check that no path through the
  verdict table, the `[N_min, N_max]` range, and the downstream licensing table
  lets a stage with no evidence read as kept.
- **Does anything in the document require a capability a later session may not
  have?** The probe needs an independent context. Say what happens to the
  measurement if none of the six sessions can obtain one.

## Notes

**Inspection only.** Findings, not fixes; the producer's artifact is not yours to
edit, and a finding you fix is a finding the integration cannot weigh.

**A finding that the instrument is too heavy is as welcome as one that it is too
weak.** The decision it feeds is which of six stages becomes a skill across four
remaining books; an instrument that costs more than that decision is worth is a
real defect, and this campaign's own root brief is what warns against machinery
ordered ahead of the measurement justifying it.

**If you find nothing worth acting on, cut nothing and retire.** An empty
integration leaf is the cost the lazy chain exists to avoid.

## Findings

### F1 · High — The non-independent probe fallback still produces decisive verdicts

The preferred comparator requires an independent context, but the fallback lets
the report session produce the probe after it has read this entire instrument,
including the hypothesis and decision rule. The document acknowledges that
exposure and says its bias has an undetermined direction, yet the only consequence
is an annotation on each verdict (`docs/evaluations/editorial-pipeline-pilot/preregistration.md:267-325`).
The first-match verdict table has no `probe not independent` arm, so the resulting
probe still drives `Drop`, `Merge`, and `Keep` exactly like a blind one
(`docs/evaluations/editorial-pipeline-pilot/preregistration.md:474-484`).

That makes the fallback an escape hatch rather than a fallback: failure to obtain
the capability the comparator requires changes the comparator's epistemic status
without changing what it licenses. Treat inability to materialise an independent
context as a missing/invalid probe, or make every comparator-dependent stage
`Undetermined`; an annotation cannot carry the weight of the blind arm it replaces.

### F2 · High — Diff verification proves recorded claims, not that the record is complete

The stage schema lets a session self-report any subset of its changes, and the
report verifies only that each reported claim can be found in the diff. Validity
likewise rejects claims that cannot be located, but never rejects a substantive
diff hunk that has no claim (`docs/evaluations/editorial-pipeline-pilot/preregistration.md:383-436,583-598`).
The limits section admits the resulting blind spot, but leaves its verdicts valid
(`docs/evaluations/editorial-pipeline-pilot/preregistration.md:658-661`).

For example, a developmental edit can make a `D1` fix and three out-of-charter
`C1` fixes while recording only the `D1` claim. The report verifies the true
entry, does not look exhaustively for omitted positive effects, and never sets
`b(S)` for the copy fixes. Copy edit can then read as `Drop` instead of
pre-empted `Undetermined`, and the developmental stage's merge evidence is also
wrong. Require an exhaustive classification of every reader-facing diff hunk
(including `no claim`), or make an unaccounted hunk invalidate the stage record.

### F3 · High — The de-duplication rule can be bypassed to cross the keep threshold

De-duplication merges only claims with the same class *and* anchored subject, and
a defect recurring at three or more subjects merely **may** be collapsed into one
`book-wide` claim (`docs/evaluations/editorial-pipeline-pilot/preregistration.md:221-245`).
The optional wording lets a stage choose the count. One terminology drift fixed
under three headings can be recorded as three `C1` claims; none shares an anchored
subject, so all survive de-duplication and `net(S) = 3` yields `Keep`. Recording
the same homogeneous family as the permitted `book-wide` claim yields
`net(S) = 1` and `Merge` (`docs/evaluations/editorial-pipeline-pilot/preregistration.md:474-492`).

Thus the asserted threshold of three does not count three distinct defects; it
counts whichever subject partition the stage chose. Make homogeneous recurring
families collapse deterministically, with a checkable equivalence rule, before a
claim count is allowed to select a verdict.

### F4 · High — `Undetermined` explicitly permits the extraction it is meant to forbid

The headline range correctly keeps an `Undetermined` stage out of `N_min`, but
the downstream licensing table then permits `pipeline-kinds-k27` to extract that
stage on any other basis it states in its own leaf
(`docs/evaluations/editorial-pipeline-pilot/preregistration.md:527-579`). There is
no evidentiary threshold for that basis and no human between report and
extraction. A stage with no valid record can therefore remain visibly
`Undetermined` while still becoming a skill.

That contradicts the campaign's governing rule that a stage which cannot be
shown to have paid is not extracted (`.grove/BRIEF.md:232-246`) and the pilot
brief's same instruction (`.grove/09-pilot-k12/BRIEF.md:79-83`). Naming the
escape as "not this pilot's evidence" does not make the pipeline derived from the
pilot. `Undetermined` must license no extraction in this campaign; a genuinely
new basis needs a new preregistered measurement rather than an assertion in the
extraction leaf.

### F5 · High — A partial probe leaves the stage-level arithmetic undefined

The partial-probe rule marks claims outside covered pages `Undetermined`, but it
only says the *stage* is `Undetermined` when **all** its claims are outside
(`docs/evaluations/editorial-pipeline-pilot/preregistration.md:628-631`). The
verdict table consumes one scalar `net(S)` and has no row for a stage containing
both covered and undetermined claims (`docs/evaluations/editorial-pipeline-pilot/preregistration.md:462-484`).

A stage with one covered marginal claim and two uncovered claims can therefore
be read as `Merge` by dropping the unknown claims, `Undetermined` by propagating
their status, or some unstated range. This is the partial-sample arithmetic hole
the producer's own completion conditions required the instrument to close.
Specify a stage-level propagation rule; the conservative coherent rule is that
any claim needed to bound `net(S)` but outside probe coverage makes the stage
`Undetermined`.

### F6 · High — The frozen taxonomy weakens obligations it claims to derive

`T1` says a defect exists when an argued claim names **neither** the jj behaviour,
the proving test, **nor** the rejected alternative and cost
(`docs/evaluations/editorial-pipeline-pilot/preregistration.md:185-203`). That is
an all-three-missing test. The frozen structure brief requires every argued claim
to name all three, so one missing element is already a defect
(`docs/specs/jj-workspace-book-structure.md:332-345`). A passage naming only a
test therefore passes `T1` while breaching its source standard. `D3` similarly
checks that a chapter opens on its refusal and states a thesis, but omits the
brief's coequal requirement to name who owns the refused responsibility
(`docs/specs/jj-workspace-book-structure.md:52-70`).

These are not unknown classes the open-questions section can defer: they are
explicit frozen obligations mistranscribed into the closed taxonomy, and the
taxonomy cannot be extended after stages begin. Reconcile every derived class
clause-by-clause with its source standard before the draft, starting with `T1`
as an any-required-element-missing test and restoring the missing ownership test.

### F7 · Medium — The claimed direction of the comparator's biases is arithmetically false

The four declared biases are labelled: one against six, one for six, one against
six, and one direction unknown. The document nevertheless concludes that three
of four run against six and that surviving the comparator is therefore stronger
evidence (`docs/evaluations/editorial-pipeline-pilot/preregistration.md:340-358`).
Only two are stated to run that way. Unknown recall is especially unsafe to count
as conservative: a missed probe defect is treated as marginal stage work and so
can push a middle stage toward `Keep`.

Correct the tally and specify how probe uncertainty limits verdicts. As written,
the document's central defence of a one-context simulated arm relies on a
conservatism it has not established.
