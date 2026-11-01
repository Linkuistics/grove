# ravel-skill-directories-k15

## Goal

Reconcile Ravel's skill-delivery design with the human's corrected directory-
injection boundary before Grove adoption review and implementation planning.

## Context

Work in `/Users/antony/Development/AgenticOrchestration/Ravel` under its own
instructions. Read the human corrections recorded in grove-skill-delivery-k8
and the current root/parent briefs. The prior Ravel design and integration are
ravel-skill-discovery-k7/k14; their resulting Ravel commit is
`66aaec06f5c020215b9b0e4139c2ba2e0b969644`.

The human stated that Grove may name a skill, but the user supplies it within
each harness or through Ravel. Ravel does not resolve a requested skill: it
injects skill-directory paths into the prompt, and the agent finds and reads
applicable guidance. The prior generated skill catalog is not a settled contract
for implementation. Do not repeat the requirements interview.

Grove's corrected design is in `docs/specs/codex-skill-provisioning.md` and
`docs/specs/harness-selection-and-execution.md`; its evidence document records
the source audit and explicit upstream dependency. Grove has no skill registry,
location reader or delivery-route requirement. Native-only setup remains valid.

## Done when

- Ravel's current spec, ADR, glossary, evidence and visual set consistently
  describes configured directory injection with agent-owned skill discovery.
  Remove or rework the generated-catalog/name-resolution contract in place;
  leave no competing normative account.
- Preserve accepted local and exact pinned Git source acquisition, offline
  reuse without revision substitution, full resource reachability, reader
  lifetime, confinement, compaction and delegation requirements. Distinguish
  directory acquisition from resolving a requested skill.
- Settle how the injected instructions expose directory locations and support
  mandatory guidance and relevant-skill discovery without a skill lookup API.
  Define the disabled/no-source path, original task/ending preservation and
  owner policy interface, compatibility and actionable failure behavior.
- Reconcile any public setup/resolution operation with that boundary; do not
  carry forward catalog fields or duplicate-name admission merely because the
  prior proposal had them. Ravel remains generic and knows no Grove kind set.
- Specify source-directory/launch acceptance and actual harness observations,
  with native-only Grove compatibility independent of Ravel discovery.
- Record owning-repository commits and update the Grove adoption handoff if its
  stated session-interface assumptions need correction. Do not silently assume
  that Grove's existing caller accepts a new Ravel major or policy schema.

## Notes

Design only: no owner settings, installations, releases or product code changes.
Preserve this session's original ending command. Decide any earned Ravel design
review lazily and insert it before grove-skill-delivery-k16; that review consumes
the corrected upstream contract. Do not add an in-session reviewer beside a
scheduled review of this producer.
