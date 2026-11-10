# shared-directory-seam-k294

## Goal

Implement the durable shared-directory seam of the delivery spec as a
reusable instrument. It links actual formula output with Homebrew's own
linker into one private prefix, then resolves the shared default directory
with Ravel. Wire it into `release:formula:test` and `tap:formula:test`.

## Context

- "Shared default directory acceptance" of
  `docs/specs/codex-skill-provisioning.md` owns the cases, the comparisons,
  the subject record and the repeat conditions.
- `docs/design/skill-keg-link-layout/reproduce.rb` and
  `task design:keg-link-layout:test` are the design experiment. They use
  fixture kegs and the real `Keg#link` and `Keg#unlink`. The evidence
  section "Directory-link layout experiment" records its subjects and
  limits.
- `scripts/release-formula.test.rb` and `scripts/tap-formula.test.rb` run
  actual install methods into private prefixes. Their kegs are the actual
  formula output that the seam must link.
- Findings L3, L4 and L5 of the evidence section "Directory-link layout
  review" explain why the seam must include Ravel's shared entries and
  whole-keg rollback.

## Done when

- One task runs the seam over a set of formula kegs and a Ravel keg. Each
  keg comes from an actual install method and Homebrew's cleaner. Each case
  owns a new private prefix and HOME, and changes no real link or setting.
- The cases are those of the spec: both skill-formula installation orders,
  absent shared ancestors and Ravel-first dependency installation. They
  also cover Ravel promotion beside installed sources, mixed-layout
  migration, upgrade, old-keg cleanup and uninstall. A duplicate ID covers
  fresh installation and upgrade, with rollback of the whole later keg.
- After each successful link, the instrument resolves the shared directory
  with no source settings. It compares the exact IDs, roots and digests with
  the accepted values. It compares each canonical `originPath` with the
  owning private payload, and checks regular payload files stay regular.
- After a refused duplicate, it checks the earlier owner's sources. It
  checks that the prefix holds no source or executable entry of the refused
  keg.
- Each run records the formula content, the full package versions, the
  Ravel build identity and the Homebrew revision. It digests the instrument
  and Homebrew's linker, installer and cleaner before and after, and a
  changed subject fails the run.
- Ravel's participating keg comes from Ravel's formula install method. It
  holds `examples`, `notices` and `installation.json` under `share/ravel`.
  Record which Ravel archive or build supplied it. After the cut, the same
  task accepts the published archives.
- The task declares the expected layout of each participating formula. An
  old-layout formula takes part only in the failure control and the
  mixed-layout migration case.
- `release:formula:test` runs the seam with Grove's projected keg. It also
  takes the tap formula files from a tap working copy. `tap:formula:test`
  runs it for its formula. Neither task passes without the seam.
- The failure control runs the current published tap formulae, whose
  sources are real directories. The control must show expansion and
  `skill_resource_invalid`, as the experiment did. Watch it fail before you
  credit a clean projected run.
- The projected cases pass with Grove's revised output and a minimal
  fixture formula for the duplicate ID. The evidence document records the
  run, its subjects and each case result. The unreleased changelog names
  the seam.

## Notes

Before the tap formulae are revised, the all-five cases cannot pass. This
leaf delivers the instrument, its failure control and Grove's cases.
skill-formula-publication-k298 runs it with all five revised formulae.

Decide whether the design experiment stays as a cheap failure control or
the new instrument replaces it. Keep its task name working, or update every
reference to it.

`scripts/check.sh` does not run this seam, because it needs Homebrew and a
Ravel 2.0 build. Say so in the task's description and in
`docs/RELEASING.md`.
