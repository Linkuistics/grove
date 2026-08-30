# delivery-channel-authority-k94

## Goal

Decide whether an available execution channel can make guaranteed treatment
delivery machine-checkable for the replacement paired campaign, and either
authorize that exact channel or leave behavioral acceptance open rather than
substituting a filesystem or command-line proxy for delivery.

## Context

- Parent delivery amendment: `acceptance-replication-authority-k76` and the
  amended `walkthrough-skill-delivery-k19` brief chain.
- Finding that exposed the unresolved mechanism: `supplemental-evaluation-k92`
  G1 and its integration at `supplemental-evaluation-k93`.
- Campaign consumer: `paired-acceptance-campaign-k80`, which must not begin
  `acceptance-instrument-k81` until this decision and its review chain settle.
- Existing product decision: `docs/adr/skill-delivers-the-methodology.md`, whose
  rejection of an observable skill-read receipt and harness-specific Grove
  launch policy does not itself authorize a weaker evaluation receipt.

## Done when

- Ask the human to choose between provisioning a direct authenticated request
  channel whose exact effective request body and provider acknowledgement can
  be preserved, identifying another harness channel that emits an authoritative
  loaded-instruction receipt, or leaving the amended behavioral claim open.
  Recommend the direct request channel; a captured CLI argument, installed
  directory, or asserted file read is not sufficient evidence.
- For any authorized channel, pin the provider/runtime identity, request schema,
  exact treatment-placement field, acknowledgement/stream event that proves
  acceptance, freshness rule, error taxonomy, and receipt artifacts. State how
  the channel detects ignored, normalized, truncated, rejected, or partially
  delivered treatment before an outcome can bear acceptance authority.
- Define the treatment transport without rewriting the deployable skill into
  evaluation-specific instructions. Specify exhaustive recursive manifest
  coverage, treatment-file ordering and boundaries, admissible file types and
  encoding, symlink/non-regular handling, maximum payload size, and the exact
  status of any framing bytes visible to the model.
- Fix per-surface model-interface access at the same seam: intake and
  exposition/assurance expose no tools; source/fragment exposes exactly one
  manifest-bound read operation for the vendored fixture, with no caller-
  supplied path, traversal, symlink escape, mutation, or network retrieval.
- Make freshness checkable: each arm is one new request containing no prior
  message or resumable session identifier, and its preserved body enumerates
  the complete system, user, tool, fixture, and treatment inputs.
- If no available channel meets those conditions, amend no delivery claim and
  run no campaign. Record that the behavioral conjunct remains open until a
  checkable channel is provisioned; do not let `paired-acceptance-campaign-k80`
  reinterpret command construction or filesystem state as receipt evidence.
- Update the parent acceptance wording if and only if the chosen answer changes
  what guaranteed delivery means. Give `paired-acceptance-campaign-k80` an
  exact stable-handle handoff it can pin without making a new requirements
  choice.
- Launch no evaluated treatment or control context. A bounded non-evaluated
  transport probe may establish interface mechanics only if the human
  authorizes the required credential or channel.
- As the final planning action, commission a lazy `review-requirements` sibling
  with bare stem `delivery-channel-authority`, inserted immediately before
  `paired-acceptance-campaign-k80`, chartered to try to disprove receipt
  authority, treatment identity, tool confinement, and freshness. Any findings
  must settle through an earned `integrate-review-requirements` step before the
  campaign runs.

## Notes

The current local CLI exposes a system-prompt append flag, but preserving argv
does not prove the effective request received by the model and does not detect
CLI normalization, truncation, or an ignored flag. No direct provider API
credential is present in this session's environment. Those observations justify
the requirements gate; they do not decide which credential or provider the
human should authorize.
