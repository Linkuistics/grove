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

homebrew-tap-trust-docs-k337 found that Homebrew 6.0.0 and later refuse the
short-name route `brew tap Linkuistics/taps` then `brew install grove` for a
new user. Tapping grants no trust, and Grove's dependency `ravel` loads only
from a trusted tap. Grove's README now gives `brew tap linkuistics/taps`,
`brew trust linkuistics/taps`, `brew install grove`, and cites
<https://docs.brew.sh/Tap-Trust#installing-from-a-tap>. The same stale route
is in the READMEs of RequirementsSkills, DesignSkills, EngineeringSkills,
ResearchSkills and WritingSkills, in WritingSkills'
`docs/TECHNICAL-COMMUNICATION.md`, and as `brew install personal` in
PersonalSkills' README.

A short name needs the formula or the tap trusted
first. A fully qualified `brew install linkuistics/taps/personal` needs no
trust step, because `personal` loads no other formula from the tap. Check
the tap's own formula documentation for the same route.

## Decisions (running log)

- The enumeration lives in its own record,
  `docs/cross-repo-docs-reconciliation-record.md`, in the shape of
  `docs/grove-docs-reconciliation-record.md`. It names each repository's
  statements before the edits and the class of each.
- A pack README gives the correct commands and points to Grove's README or
  Ravel's book for the rest. A domain pack points to Grove's README, because
  Grove's formula carries it. PersonalSkills and TestAnyware point to
  chapter 3 of Ravel's book, because their formulae do not depend on Grove.
- "Grove's formula carries the packs" gains the qualifier "from the release
  after 22.8", which Grove's own documents use. The statement is then true
  before and after Grove's release.
- `personal` and `testanyware` keep fully qualified names. Homebrew's Tap
  Trust page states that a fully qualified name trusts only the formula it
  names, and neither formula loads another formula from the tap.
- The tap's formula files are rendered by the release tooling of Grove,
  Ravel and TestAnyware. The caveats of `personal`, `superpowers` and
  `quint-llm-kit` are current. `grove.rb` holds the released 22.8.0 caveats,
  and Grove's release replaces them from `scripts/templates/grove.rb.tmpl`,
  which is current. No tap file changes.
- The manual is append-only, so its stale checkout row receives an appended,
  dated correction. Its native-route statements describe the owner's live
  machine until the switch, so they are current now. Their correction after
  the switch goes into finish-handoff-k324 as an obligation, because only a
  post-switch observation can state it truthfully.
- The manual's maintenance checker defaults `AGENT_TOOLING_GROVE_REPOSITORY`
  to the removed `~/Development/grove`, so it drops the Grove source row.
  The default moves to the current checkout, under the manual's own
  verification procedure. The switch plan pins no vault file.
