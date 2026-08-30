# supplemental-evaluation-k93

**Integrates:** supplemental-evaluation-k92

## Goal

Apply the `supplemental-evaluation-k92` findings to the
`paired-acceptance-campaign-k80` decomposition before `acceptance-instrument-k81`
runs, so the campaign that executes cannot deliver an unverifiable treatment,
cannot convert infrastructure failure into an unattainable endpoint, and cannot
be amended or repeated into a favorable result.

## Context

- Findings, verbatim and severity-ordered: the `## Findings` section of
  `supplemental-evaluation-k92`, read from its own commit.
- Producer plan being corrected: `supplemental-evaluation-k77` and the
  `paired-acceptance-campaign-k80` subtree it created.
- Requirements authority and amended acceptance wording:
  `acceptance-replication-authority-k76` and `walkthrough-skill-delivery-k19`.
- Claim boundary: `acceptance-contract-reconciliation-k75`.
- Prior findings the review re-derives rather than trusts: F1-F14 in
  `evaluation-recovery-k73` and their dispositions in `evaluation-recovery-k74`.
- Immutable evidence the findings are grounded in:
  `docs/evaluations/writing-code-walkthroughs/README.md` and
  `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.

## Done when

- Every finding is classified as applied, applied differently, or declined with
  a reason recorded against the parent contract. A finding is not obligatory
  because it was written down; declining one is a legitimate outcome that the
  running log must argue rather than assert.
- The delivery-channel finding is settled first and explicitly, because the
  remaining shape depends on it: name the mechanism by which verified treatment
  bytes reach an enabled context, name the artifact that evidences delivery, and
  state the per-surface tool-access rule that mechanism implies. If no available
  channel makes delivery machine-checkable, say so and externalize the
  requirements question rather than weakening the delivery claim.
- Any finding whose repair changes what the campaign measures — rather than how
  a task file states it — is externalized as a new producer leaf beside this one
  with its own review chain, not absorbed here.
- Fixes land in the `.grove/` task files and briefs of the campaign subtree. No
  historical evaluation record, no frozen rubric byte, and no committed campaign
  result is edited, and no evaluated treatment or control context runs.
- Where a finding restores a disposition that `evaluation-recovery-k74` settled
  and this subtree dropped, the restoration cites that disposition so the two
  cannot diverge again.
- Ownership gaps are closed by naming an owning leaf, never by adding the work
  to whichever leaf is nearest.
- Any leaf the findings show to be more than one focused session is decomposed
  or marked as an expected `leaf-decompose` candidate with its seam named.
- After integration, `acceptance-instrument-k81` remains the first live leaf of
  the campaign and no execution child has run.

## Notes

Read the findings from the review's own commit; they are deliberately not
restated here, so that rejecting one is not rejecting this leaf's charter.

The review states which findings it considers load-bearing and why in its
running log. Treat that ordering as the reviewer's argument, not as a verdict:
the campaign's shape is this session's call against the parent contract.

Several findings interact and should be judged together rather than in file
order. The delivery mechanism decides what a delivery receipt can be and what
the tool-access rule must say. The outcome taxonomy, the resource-exhaustion
rule, and the stop-or-continue rule are three faces of one question — which
failures the campaign is allowed to retry, and who decides — so applying one
without the others leaves the channel open. The freeze-immutability and
amendment rules govern what the freeze candidate's own scheduled review is able
to do, so they must be settled before `campaign-freeze-k84` is rewritten.
