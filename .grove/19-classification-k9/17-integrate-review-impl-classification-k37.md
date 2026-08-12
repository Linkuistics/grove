# classification-k37

**Integrates:** `classification-k35`

## Goal

Repair the semantic classification defects found by `classification-k35`, then
re-run the classification and whole-embed evidence required by the node brief.

## Context

The producer batches already proved byte partition, marker coverage, and graph
validity. This leaf is about the semantic properties those checks cannot prove:
honest triggers, correct scope, correct addressing, and slices that stand alone.

The review findings are copied verbatim below:

1. **[high] Row 23 sends “citation discipline” to the wrong body.**
   `skill-adrs-and-specs` promises field-guide habits for grilling,
   research-leaf commissioning, and the review chain. Its fourth field-guide
   address is currently `driving-cite-framework-decisions-to-the-source`, whose
   condition and procedure are specifically about code depending on a framework
   version. The promised research-leaf habit is instead stated by
   `driving-how-to-write-a-research-leaf-brief`: demand a citation for every
   failure-mode claim, prefer primary sources, and record missing sources. Point
   the edge at the body the source actually promises; existing reachability is
   not evidence that the current edge is semantically valid.
2. **[high] Narrative units have been made artificial triggering roots.**
   `skill-loop-diagram` contains an overview diagram, not a condition a session
   can recognize. `driving-field-guide` contains file-level framing and an
   anchor index, not a condition either; its generic `kinds=*` root also hides
   the concrete prohibitions in procedural `driving-anti-patterns`. Rework or
   remove this file-reader prose and give each rule that must ship an honest
   triggering statement. Do not preserve narrative reachability by calling the
   narrative a trigger, and do not leave the anti-pattern conditions reachable
   only through a generic table of contents.
3. **[medium] `driving-when-to-invoke-grilling` has a scope/prose mismatch.**
   Its marker says `kinds=*`, but its complete trigger says only “a
   `requirements` leaf's brief lists three or more questions whose answers
   interdepend.” Either narrow the marker to the condition actually written or
   rewrite the condition to state the intended all-kind behaviour, including
   what a non-requirements session does when it encounters those questions.
   The batch rationale cannot silently broaden semantics that are absent from
   the delivered bytes.
4. **[medium] Several slices still fail the prose half of “reads correctly
   standing alone.”** Rewrite `driving-when-asserting-a-repo-wide-claim` so it
   names its subject instead of opening with “The counterpart to the section
   above”; remove or localize `grilling.md`'s cross-unit
   `<supporting-info>` wrapper; and attach the relevant licence attribution
   comment to each separated adapted unit. In the same focused prose cleanup,
   remove file-reader-only residue and near-verbatim hub restatements that have
   no mandate-delivery job. Preserve genuinely useful rules by restating their
   conditions, not by retaining a document-navigation shell.

## Done when

- All four findings above are addressed without weakening the byte-partition,
  scope, DAG, and reachability invariants in the classification node brief.
- The corrected edges are justified semantically, not merely accepted by the
  reachability check.
- Every triggering unit states an honest condition or prohibition delivered to
  exactly the kinds that need it; narrative survives only where it has an
  explicit mandate-delivery job.
- Every touched unit reads correctly when fetched alone, including local
  attribution where adapted prose was split from its original comment.
- Focused and whole-embed checks pass against the rebuilt, installed binary,
  with representative per-kind retrieval spot-checks for the repaired paths.

## Notes

If the prose cleanup proves larger than one focused session, decompose this leaf
and execute only its first child; do not partially absorb a corpus-wide rewrite.
