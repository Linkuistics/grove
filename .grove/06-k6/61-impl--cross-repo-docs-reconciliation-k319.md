# cross-repo-docs-reconciliation-k319

## Goal

Make the documentation and configuration of the other affected repositories
describe the final common path.

## Context

- The parent brief, Done when, first item.
- `docs/repository-moves-record.md`, section "The handoff to the owner
  cutover": the ten sources, their formulae and their repositories.
- `docs/specs/owner-skill-delivery.md`, section "Cutover contract": the
  maintained manual still names the earlier `~/Development/grove` checkout.

## Done when

- The enumeration covers RequirementsSkills, DesignSkills, EngineeringSkills,
  ResearchSkills, WritingSkills, PersonalSkills, TestAnyware's skill
  documentation, the tap's formula documentation and the maintained manual.
- Each statement about installation, native routes or skill delivery is
  classified as current, historical or stale. Each stale one is corrected.
- TestAnyware's release template keeps the keg layout of
  testanyware-keg-projection-k296.
- Each changed repository passes its own checks, and each change is
  published through that repository's procedure.

## Notes

Do not change a file inside an accepted source root, which is a pack's
`skills` or `craft` directory. Such a change moves its `resourceDigest`.
Do not advance a pin in Grove's pack record or in a formula. A change outside
the roots needs no new pin. Carry a necessary change inside a root as live
work with its restaging consequence.

promotion-upgrade-rehearsal-k318 observed Homebrew's caveat for `ravel@2`
beside a linked `ravel` 1.1.0: `Run brew link ravel@2 to switch the active
version to this keg.` Check that no Grove or Ravel page repeats that
suggestion for states B and C. "Observations for later states" in
`docs/promotion-upgrade-rehearsal-record.md` gives the reason.
