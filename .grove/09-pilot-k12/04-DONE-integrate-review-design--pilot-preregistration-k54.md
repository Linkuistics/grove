# pilot-preregistration-k54

**Integrates:** pilot-preregistration-k53

## Goal

Triage `pilot-preregistration-k53`'s findings against the committed editorial-
pipeline preregistration and apply the valid ones before any page of the
`jj-workspace` book is drafted.

## Context

- The reviewed artifact is
  `docs/evaluations/editorial-pipeline-pilot/preregistration.md`, produced by
  `pilot-preregistration-k24`.
- This leaf is inserted immediately ahead of `jj-workspace-book-k25` because the
  instrument must be settled before the experiment reads it. Do not let a fix
  turn into a post-draft reinterpretation.
- Read the findings from the review task and its commit. They are evidence to
  weigh, not this leaf's charter; reject any that do not survive re-derivation.

## Done when

- Every finding in `pilot-preregistration-k53` is classified, with the valid
  findings integrated and the rejected ones recorded with evidence.
- The four preregistered parts remain precise enough that later stage and report
  sessions cannot choose their interpretation after seeing the book.
- The corrected instrument is committed before `jj-workspace-book-k25` begins.
- `bash scripts/check.sh` passes.

## Notes

**The artifact is still frozen ahead of its subject, not frozen against review.**
This integration is the scheduled pre-draft correction point. After this leaf
retires, any rule that proves unworkable is a protocol breach to report, not a
licence for a later session to substitute a better rule.

## Decisions (running log)

**1 · All seven findings survive triage, and none is a redesign.** Each was
re-derived against the artifact and, where it cited one, against the frozen
standard it claimed was mistranscribed. Nothing was accepted on the reviewer's
say-so and nothing was rejected: every finding names a place where a rule was
stated loosely enough that a later session could choose its own reading, which is
precisely the defect class a preregistration exists to have none of. All are
repairs to stated rules — the four parts, their names and their structure stand —
so no new producer review chain is cut beside `pilot-preregistration-k24`.

**2 · The instrument's frozen inputs are intact, and that was checked rather than
assumed.** All four corpus digests, all five standards digests, the
`ordinal-fs-tree` book-directory aggregate and all seven stage task-file digests
recompute to their recorded values. The `.grove/` paths in the stage-charter table
have moved — this leaf's own `leaf-insert` shifted the draft from position 04 to
05, and `pilot-measure-k26` from 05 to 06 — and are deliberately left as recorded:
the document already states that the `<slug>-k<key>` handle is the identity and a
moved path is not drift. Rewriting them would be churn against a stated design.

**3 · F1 accepted, with the remedy chosen from two the finding offered.** The
finding is right that an annotation changing a verdict's epistemic status without
changing what it licenses is an escape hatch. Of its two proposed remedies —
treat the fallback as a missing probe, or make every comparator-dependent stage
Undetermined — both nullify the pilot whenever the harness cannot furnish an
independent context, including the `N = 2` outcome the campaign explicitly calls a
success. Taken instead: **the fallback licenses no `Keep`**, added as a clause of
the first-match Undetermined row so it fires before `Keep` and cannot be
reinterpreted; `Merge` and `Drop` stand. The asymmetry is principled rather than
convenient, and F7 supplies its derivation — probe recall error pushes toward
`Keep`, and a `Keep` is what costs four skills and a session per book across four
books. The negative result stays reachable; the expensive verdict does not.

**4 · F2 accepted in full.** Verified: *Part 1*'s verification rejects a claim
that cannot be located in the diff, and nothing anywhere rejected a diff hunk
carrying no claim. The stage record now carries `## Unclaimed changes`, and the
claim table plus that section must account for every prose hunk. Two consequences
are stated separately because they land on two different stages: an unaccounted
hunk that closes or opens a taxonomy defect invalidates the *recording* stage
(Undetermined), **and** the report classifies and credits it anyway, so
pre-emption still fires for the stage it silently pre-empted. Only the pair closes
the finding's own worked example — a developmental edit hiding three `C1` fixes
must not be able to delete the copy edit's verdict by omission.

**5 · F3 accepted in full.** The `may` was doing the damage: collapse was
optional, so the partition — and therefore the count that selects the verdict —
was the stage's to choose. Collapse is now mandatory, and the finding's request
for a checkable equivalence is met by a test a report can apply without the
author: instances are one family when a single sentence stating the correction
applies to all of them without naming a subject. The report re-applies the
collapse before counting rather than trusting the record's partition.

**6 · F4 accepted in full, and it was the clearest of the seven.** The licensing
table's `Undetermined` row permitted `pipeline-kinds-k27` to extract on a basis
stated in its own leaf, with no evidentiary threshold and no human between the
two — a direct contradiction of the root brief and the pilot brief, both of which
say a stage that cannot be shown to have paid is not extracted. The row now
licenses nothing at all; a new basis is a new preregistered measurement. Also
added: `N_max` measures unresolvedness and licenses nothing, so a wide range
cannot be read as six stages kept.

**7 · F5 accepted, with the arithmetic made explicit rather than merely
constrained.** The finding's conservative rule — any uncovered claim needed to
bound `net(S)` makes the stage Undetermined — is right, and stating it as an
interval is the precise form of it: `net_lo(S)` treats every uncovered claim as
reached, `net_hi(S)` as marginal, and the stage resolves only where both select
the same row. That resolves the cases where the missing coverage cannot change
the verdict, which a blanket rule would throw away for nothing.

**8 · F6 accepted, and the clause-by-clause reconciliation it asked for was
run over all seventeen classes.** Five needed correcting, not two. Confirmed at
source: the structure brief requires an argued claim to name all three of the jj
behaviour, the test and the rejected alternative with its cost, so `T1`'s
all-three-missing test was strictly weaker than its source; the spine sentence
states the refusal and *who owns it instead* coequally, which `D3` omitted. Found
in the same pass: `D2` enumerated only the brief's list where the book contract's
differs and both bind; `C2` dropped *behaviour* from the contract's list of what a
sentence identifies; and `C4`'s link-label clause overlapped `M201`, a mechanical
property the measure explicitly scores nothing for. `A1` and `A2` derive from no
frozen standard — art has none — and are stated rather than reconciled. The
reconciliation is recorded in the artifact, because the taxonomy cannot be
extended once a stage has run and this was the last point at which the derivation
could be made faithful.

**9 · F7 accepted, and it turned out to be load-bearing rather than cosmetic.**
The tally is plainly wrong on the document's own list: two biases run against the
six-stage hypothesis, one for it, one undetermined. Correcting the count would
have been enough to close the finding, but the finding's second half is the
substantive part — unknown probe recall is not a conservative unknown, because a
missed probe defect reads as marginal stage work and pushes toward `Keep`. That
established the direction the instrument must be cautious in, and it is what
justifies F1's asymmetric remedy rather than the symmetric one. The two findings
were integrated as one argument.

**10 · A provenance paragraph was added to the artifact, not only to this log.**
`.grove/` does not outlive the campaign and the preregistration does, so the
document itself now records that it was reviewed and corrected before any page was
drafted, what the seven defects were, and that the correction point is closed. The
document's own rule — an unworkable part is a protocol breach to report, not a
rule to replace — is restated immediately after it, so the correction cannot be
read as precedent for a later substitution.

**11 · No in-session reviewer spent.** Every finding was decidable against the
artifact, the two frozen standards, or arithmetic on the document's own list.
Nothing turned on a judgement a fresh context could settle better.
