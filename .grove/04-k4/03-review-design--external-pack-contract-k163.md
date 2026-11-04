# external-pack-contract-k163

**Reviews:** external-pack-contract-k155

## Goal

Read the external pack contract adversarially before the runner, the pack and
the removal build on it. Every later pack move inherits its decisions.

## Context

- `docs/specs/external-pack-contract.md`, the artifact under review. Its views
  are under `docs/design/external-pack-contract/`.
- The three reworked records: `docs/adr/corpus-rules-have-one-owner.md`,
  `docs/adr/a-skill-states-what-binds-without-its-dependencies.md` and
  `docs/adr/behavioural-coverage-asserts-delivery.md`. A preparatory commit
  converted each one to controlled English first, so compare the rework with
  that commit, not with the older text.
- The reconciled sections: `docs/specs/owner-skill-delivery.md`, "Ravel
  sources" and "Pack checkouts", and `docs/specs/codex-skill-provisioning.md`,
  "Resources and conformance".
- The producer's decision log in its task file. The commit of
  external-pack-contract-k155 names its handle.
- The current runner `plugins/grove/conformance.sh`, its manifest and the four
  moving files under `plugins/grove/skills/`.
- Ravel's `docs/specs/skill-discovery.md` in
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`, for
  capture, roots and the loading rules.
- The Claude Code plugin documentation, for the marketplace sources, the
  manifest `skills` paths, `dependencies` and namespaces. Read it, not memory.

## Done when

- Test the edge syntax. Can the runner read "Load the `<name>` skill" without a
  false edge or a missed edge in core's text and in the moved text? Does rule 3
  of "References between skills" keep the meaning of core's existing paths?
- Test the family skill. Look for a case where a skill fails and a resource or
  another arrangement works. Check the claims about the `grove-` prefix and
  `disable-model-invocation`.
- Test the conformance design. Does each control fail for the stated reason,
  and can a broken runner pass the clean readings? Does the change from a
  pending row to a failure break a check that should pass?
- Test the records. Is the marketplace `sha` a sound record for Grove's check?
  Is a network fetch inside `scripts/check.sh` acceptable? Does the pack's
  `GROVE_COMMIT` create a cycle of pins?
- Test the missing-skill sentence against the too-late test and against the
  launch prompt's actual wording.
- Check each reworked record and each reconciled section for a rule with two
  owners. Look also for a statement that the contract contradicts.
- Produce findings only. If a finding is actionable, cut an
  integrate-review-design step immediately before pack-conformance-runner-k156.

## Notes

The producer accepted one gap on purpose: no mechanical check covers the
external rows of `TASK-FORMAT.md`'s kind table. Challenge that trade-off if a
cheap check exists that keeps kinds open.

The Grove line of this workstream is not on the remote. The contract states the
consequence for a pack check before finish. Check that statement against the
later leaves.
