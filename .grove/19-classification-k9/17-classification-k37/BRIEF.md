# classification-k37 — brief

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

## Decomposition

**This was a single leaf until it was bootstrapped.** Its own *Notes* licensed
the split, and scoping the four findings with the corpus open showed the cleanup
is two sessions, not one. The measurement, not the impression:

- Findings 1 and 3 are **one marker attribute each** — a `defers=` member and a
  `kinds=` value.
- Finding 4 is **two different jobs in one paragraph**. Its three *named*
  repairs (`driving-when-asserting-a-repo-wide-claim`'s opening, `grilling.md`'s
  `<supporting-info>` wrapper, licence attribution on separated adapted units)
  are edits *inside* existing units. Its closing clause — *remove file-reader-only
  residue and near-verbatim hub restatements* — is a **deletion sweep over nine
  or more units across three files**, each deletion cascading into `defers=`
  edges, the 143-id pinned set, and the byte-partition claim.
- Finding 2 is the same species as that sweep and **collides with it on the same
  lines**: `driving-field-guide` is simultaneously finding 2's artificial root
  and finding 4's `## In this guide` residue.

### The seam, and why it is checkable rather than aesthetic

**Child 1 leaves the pinned unit set untouched; child 2 moves it.** Every repair
in `addressing-k38` changes a marker attribute or prose *within* a unit — no id
is minted, retired, or reclassified, so `EMBEDDED_UNITS` in
`tests/methodology.rs` needs no edit and the partition arithmetic is unchanged by
construction. Every repair in `narrative-k39` removes or re-roots units, so the
pinned set moves and each removal must be reconciled edge by edge.

That is a boundary a session can *check* it stayed inside, which is why the split
falls there and not on the finding numbering.

### Why finding 2 cannot be done cheaply, and why it leads child 2

Verified with the corpus open, before the split was chosen:
`driving-anti-patterns` and `driving-the-shortest-version` each have **exactly
one inbound edge, and it is `driving-field-guide`** — the table of contents the
review calls an artificial root. So removing that root without first giving the
four anti-pattern prohibitions an honest triggering statement makes them
unreachable and the build says so. Finding 2's *"do not leave the anti-pattern
conditions reachable only through a generic table of contents"* is therefore
structural work — new triggering prose — not a tidy-up, and it belongs with the
sweep that deletes the same index.

### The children

1. `addressing-k38` (integrate-review-impl) — findings **1**, **3**, and finding
   **4's three named repairs**. The pinned unit set is unchanged; that invariant
   is the leaf's own boundary check.
2. `narrative-k39` (integrate-review-impl) — finding **2** and finding **4's
   residue-and-restatement sweep**, together because they are one species and one
   set of lines. The review itself grouped them: *"They are one species:
   narrative addressed to a reader of a **file**, in a corpus that no longer
   delivers files."*

Ordering is a real dependency in one direction only: child 1 settles what
`skill-adrs-and-specs` actually addresses before child 2 re-roots units around
it. Child 2 may decompose again if the sweep proves bigger than one session; the
seam to look for there is *additive* (writing honest triggers) against
*subtractive* (deleting prose with no mandate-delivery job).

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

- Retiring the **last** live child here closes this node **and** its parent
  `classification-k9`, whose own last live leaf this was. Both cascades fire in
  one session; check each brief's `Done when`, promote upward, and name every
  closed handle in the commit message.
- The original leaf's instruction — *do not partially absorb a corpus-wide
  rewrite* — survives the decomposition and now applies per child.
