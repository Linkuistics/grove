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
