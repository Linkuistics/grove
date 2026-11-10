# skill-formula-publication-k298

## Goal

Accept and publish the four revised non-Grove formulae. Run the
shared-directory seam with all five revised formula outputs and Ravel's
keg. Write a packaging-only acceptance record for each package. Then push
the tap.

## Context

- "Skill formulae" of `docs/specs/owner-skill-delivery.md` owns
  packaging-only acceptance and the full package version.
- "Shared default directory acceptance" of
  `docs/specs/codex-skill-provisioning.md` owns the seam. Its instrument is
  that of shared-directory-seam-k294.
- tap-skill-keg-projection-k295 and testanyware-keg-projection-k296 left
  the four revisions as local tap commits. hook-adapter-keg-seam-k297
  recorded the hook and guardrail readings.
- The section "Staging acceptance" of
  `docs/owner-skill-delivery-evidence.md` records the accepted commits,
  roots and digests and the source trials that read them.

## Done when

- The seam passes every case with Grove's projected output, the four
  revised tap formulae and Ravel's formula output from the accepted 2.0
  build. The failure control fails on the current published formulae. The
  evidence records the subjects and each case.
- Each of `personal`, `superpowers`, `quint-llm-kit` and `testanyware` has
  an acceptance record. It names the full package version with its
  revision and the formula content digest. It names the equal commit,
  roots and digest, and the earlier source trials that it retains. It
  names the seam run and the applicable hook readings.
- The record of `personal` names guardrail's payload comparison and its
  capability trial. No record claims acceptance from equal source digests
  alone.
- The tap's `main` holds the four revisions, and the push succeeded. A
  fresh read of the published tap shows each full package version.
- No real keg, link or HOME changed. owner-pack-checkouts-k93 installs the
  revisions.

## Notes

Grove's revised template publishes through finish's release, not here.
Its seam participation here uses the working-copy output.

If any source digest differs from its accepted value, that source needs
its staging trials. Stop and report it. Do not publish that formula.

Push the tap only after every record exists. The tap publishes only
accepted versions.

shared-directory-seam-k294 delivered the seam. Run it through
`task release:formula:test` with `TAP` set to the tap working copy. The
failure control reads the published formulae from the tap's `origin/main`.
After the push, change each `published` declaration in
`scripts/skill-formulae.json` to `projected`. The fixture formulae then keep
the failure control.
