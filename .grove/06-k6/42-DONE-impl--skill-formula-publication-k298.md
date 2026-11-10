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

## Decisions (running log)

- The subjects equal those of k294 to k297. The Grove archive
  `5f6093a6e6bb…` is current, because no change since k293 touches
  `plugins`, `assets`, `crates` or the formula template. The lane 2.0 Ravel
  archive `289b4ac57ffd…` holds the accepted staging build: its `bin/ravel`
  has the pinned SHA-256 `3a9a9129…`. Homebrew stays at `bc63375ac3a4`. The
  tap's remote `main` stays at `9ca2b5e37510`.
- The seam's "as accepted" for a tap formula compares with that formula's
  own pristine payload, not with the staging-accepted digest. Each
  `tap:formula:test` therefore ran again, and its digest of the plain
  checkout was compared with "The accepted sources". All four are equal,
  so no source needs staging trials.
- Only `personal` installs a native-only plugin and a marketplace file. The
  hook readings of k297 that apply are the `guardrail` payload comparison
  and capability trial. The Grove hook-adapter readings name Grove's
  writing payload, so no hook seam applies to `superpowers`,
  `quint-llm-kit` or `testanyware`.
- The acceptance records go in a new section of
  `docs/owner-skill-delivery-evidence.md`, after "Staging acceptance".
  owner-switch-plan-k254 and owner-pack-checkouts-k93 read accepted values
  from that document.
- The four records exist in "Packaging-only acceptance" of
  `docs/owner-skill-delivery-evidence.md`. The tap push (`jj bookmark set
  main -r 12b53f8c449d`, then `jj git push --bookmark main`) was refused by
  the harness's permission classifier. The tap stayed unchanged. The push,
  the fresh read and the `published` declarations wait for the human.
- The human approved the push on retry. The tap's `main` moved from
  `9ca2b5e37510` to `12b53f8c449d`. A new clone read each published full
  package version with Homebrew's loader.
- After the push, the four `published` declarations became `projected`.
  `task seam:shared-directory` passed with the tap subjects, and its
  failure control failed on the two direct fixtures alone.
- The 14 older checker findings of `docs/owner-skill-delivery-evidence.md`
  predate this leaf. This leaf adds none and converts none.
