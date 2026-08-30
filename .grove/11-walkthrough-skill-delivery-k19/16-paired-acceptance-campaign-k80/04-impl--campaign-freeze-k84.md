# campaign-freeze-k84

## Goal

Reconcile the independently authored instrument, minimal apparatus, and
verified treatment into the sole immutable pre-execution campaign manifest.

## Context

- Inputs: `acceptance-instrument-k81`, `campaign-apparatus-k82`, and
  `treatment-verification-k83`.
- Requirements and claim boundary: `acceptance-replication-authority-k76` and
  `acceptance-contract-reconciliation-k75`.
- Historical comparison source only:
  `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.

## Done when

- One signed, content-addressed manifest pins all verified treatment bytes, the
  reviewed delivery-authority decision, effective-request and receipt schema,
  arm request bodies, per-surface tool declarations, runtime identity, prompts,
  fixtures, surface rows, samples,
  replacement and resource rules, scoring prompts, comparative thresholds,
  absolute floors, access rules, schedules, and the three-surface AND endpoint.
- `acceptance` and predeclared `supplemental` namespaces are explicit and frozen.
  The manifest reserves a third post-verdict `exploratory` namespace whose
  contents are necessarily post-hoc and can never enter an acceptance formula.
  No supplemental or exploratory row, discovery measure, arm guess, alternate
  endpoint, or transfer result can enter an acceptance formula.
- A post-authoring treatment-to-row coverage map proves coverage without adding,
  deleting, or weakening any independently authored row. A retained/reworded/
  dropped map against every historical rubric row gives a reason for each
  relation as non-weakening audit evidence, never as a criterion source.
- A disposition table confirms F1-F14: generation precedes adjudication;
  replacement is deterministic and replayed under the carrier / behavioral /
  apparatus taxonomy; carrier retrying and automatic replenishment are globally
  bounded, pair-atomic, and non-selective; transfer remains supplemental; criterion authoring
  is role-separated; regression coverage is exhaustive with a mixed-row rule;
  unavailable data fails closed; scorer/resolver ownership is named; arm guesses
  are supplemental; every bundle is dual-scored; pair arms are adjacent; and
  pre-freeze tests use synthesized evidence only.
- Stubbed end-to-end rehearsal proves manifest validation, authoritative treatment delivery,
  prompt identity, adjacent pairing, complete record capture, replacement
  replay, access audit, blind dual scoring, and resolver invocation without a
  live evaluated model. It does not automate surface or AND verdict arithmetic;
  the published per-row counts and frozen formulas make both independently
  re-derivable.
- The manifest states that the historical records are the sole pre-skill
  baseline, current no-skill runs are contemporaneous comparators, and the new
  verdict cannot combine with, repair, erase, or rescore the failed campaign.
- The campaign artifact root is
  `docs/evaluations/writing-code-walkthroughs-paired-acceptance/`; the manifest
  maps instrument, apparatus, vendored fixture, treatment, raw records, score
  records, and report to distinct paths there. A recursive path-and-digest
  manifest of the historical
  `docs/evaluations/writing-code-walkthroughs/` tree is captured at freeze for
  `three-surface-verdict-k91` to reverify.
- No evaluated output predates the *final reviewed manifest commit*. Before any
  evaluated context runs, a finding integration may amend the candidate only by
  emitting and committing a complete replacement manifest with a new digest;
  the review chain then binds execution to its final manifest. This scheduled
  temporal separation deliberately reverses `evaluation-recovery-k74`'s F14
  trade-off because this is the single acceptance-authorized campaign per
  treatment digest and the new delivery seam is acceptance-critical. After the
  review chain settles, any frozen-byte change invalidates the cycle and needs a
  new requirements-authorized manifest rather than an amendment in place.
- To settle F14, the manifest receives a scheduled fresh-context adversarial
  read. As this leaf's final act, commission a lazy `review-impl` sibling with
  bare stem `campaign-freeze`, inserted immediately before
  `intake-generation-k85`, chartered to try to disprove manifest boundaries,
  treatment delivery, scoring authority, and fail-closed endpoint. Execution
  waits for that review and any earned integration.

## Notes

The review is delayed until the immutable candidate exists because that is the
first point at which the complete freeze can be disproved as one artifact.
