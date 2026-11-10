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

## Decisions (running log)

- The design experiment stays as a cheap failure control, and its task name
  stays. The delivery spec already calls it a reusable failure control. It
  needs no formula, download or tap, so it remains usable after the tap
  formulae change layout.
- The seam is `scripts/shared-directory-seam.rb` under `brew ruby`. A shell
  front, `scripts/shared-directory-seam.sh`, prepares its inputs, and
  `task seam:shared-directory` runs it alone. The instrument substitutes the
  prefix, Cellar, cache, Caskroom, lock and linked-keg constants for each
  case. It calls `Keg#link`, `Keg#unlink`, `Keg#optlink` and `Keg#uninstall`
  as `Install.install_formula`, `FormulaInstaller#link`,
  `Cleanup#cleanup_keg` and `Uninstall` call them at Homebrew `bc63375ac3a4`.
  It runs each install method and then `Cleaner#clean`.
- The seam loads each formula through `Formulary.load_formula_from_path`, so
  a published formula and its revision load side by side in separate
  namespaces.
- `scripts/skill-formulae.json` declares the layout of each of the five
  skill formulae, both for the working copy and for the published tap. The
  seam refuses a keg that differs from its declaration. A published direct
  formula joins the failure control. A published direct formula whose
  working copy is projected is also the predecessor of the migration case.
- The failure control and the migration case always include direct and
  projected variants of a fixture formula. They therefore keep two direct
  kegs after the tap formulae change layout.
- The Ravel keg comes from a Ravel formula file and its archive. The seam
  identifies the archive by its SHA-256 in a lane of
  `scripts/ravel_target_inputs.py` that records a build identity. It checks
  that `bin/ravel` holds that identity, and refuses an unrecorded archive.
  Ravel prints no build identity, so the lane record is the only source.
- Accepted values come from the caller where it has plain checkouts: Grove's
  six sources in `release:formula:test`, and the formula under test in
  `tap:formula:test`. Elsewhere the seam resolves each pristine private
  payload through explicit settings before any link. Each tap formula's
  plain-checkout comparison remains in its own `tap:formula:test`.
- The failure control links every published direct formula first, then the
  two direct fixtures, in both orders. Homebrew expands only the keg that
  linked first, so each order names a different expanded formula.
- The ordinary cases add `seam-fixture` 2 and `seam-duplicate` 1 to the
  projected subjects. Two projected kegs therefore exist even when the
  formula under test is still direct. The duplicate's second version ships
  the first source of the first projected subject, `grove-core` in the
  release test.
- The mixed-layout case resolves only after the last upgrade. Before it,
  the case requires the expansion when a projected keg links after the
  direct keg. A migration pair with equal package versions is refused,
  because Homebrew would not upgrade it.
- `release:formula:test` takes `TAP`, `RAVEL_FORMULA` and `RAVEL_ARCHIVE`.
  `tap:formula:test` takes the two Ravel inputs and reads the published
  version of its formula from its tap's `origin/main`. No release task
  calls either test, so the new required inputs break no automated step.
- The seam compares each formula's sha256 with its archive. That found the
  renderer of `release-formula.test.sh` leaving `@SHA_AARCH64_APPLE_DARWIN@`,
  because `[A-Z_]` excludes digits. The pattern is now `[A-Z0-9_]`.
- `docs/grove-delivery-contraction.md` had two paragraphs over the sentence
  limit. Their split is its own change, before the content edit.
